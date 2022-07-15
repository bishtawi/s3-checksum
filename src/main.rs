#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, ffi::OsStr, path::Path, sync::Arc};

use anyhow::{anyhow, Result};
use clap::Parser;
use futures_util::future::join_all;
use itertools::Itertools;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;

use crate::dtos::{Args, FileToProcess, ProcessedFile};

mod aws;
mod crawler;
mod dtos;
mod hasher;
mod utils;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arc::new(dtos::Args::parse());
    let aws_config = aws::get_aws_config(&args.url).await?;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    let (need_work_tx, need_work_rx): (
        Sender<oneshot::Sender<FileToProcess>>,
        Receiver<oneshot::Sender<FileToProcess>>,
    ) = mpsc::channel(args.threads.into());
    let (processed_file_tx, processed_file_rx): (Sender<ProcessedFile>, Receiver<ProcessedFile>) =
        mpsc::channel(32);

    let crawler_handle = crawler::spawn(need_work_rx, s3_client.clone(), args.clone());
    let worker_handles: Vec<JoinHandle<Result<()>>> = (0..args.threads)
        .map(|_| {
            worker::spawn(
                need_work_tx.clone(),
                processed_file_tx.clone(),
                args.algorithm,
                s3_client.clone(),
                args.bucket.clone(),
            )
        })
        .collect();

    // main thread doesnt need the sender channel
    drop(need_work_tx);
    drop(processed_file_tx);

    let res = process_checksums(args, processed_file_rx).await;

    crawler_handle.await??;
    for res in join_all(worker_handles).await {
        res??;
    }

    res
}

async fn process_checksums(
    args: Arc<Args>,
    mut processed_file_rx: Receiver<ProcessedFile>,
) -> Result<()> {
    let mut writer = if args.check.is_none() {
        Some(BufWriter::new(
            File::create(format!("{}.{}.checksum", args.bucket, args.algorithm)).await?,
        ))
    } else {
        None
    };

    let mut total_count = 0;
    let mut total_size = 0;
    let mut stats: HashMap<String, (u64, i64)> = HashMap::new(); // (count, size)
    let mut errors: HashMap<String, String> = HashMap::new();
    while let Some(message) = processed_file_rx.recv().await {
        match message.actual_checksum {
            Ok(checksum) => {
                let path = Path::new(&message.key);
                let extension = path
                    .extension()
                    .or_else(|| path.file_name())
                    .and_then(OsStr::to_str)
                    .unwrap_or(&message.key)
                    .to_lowercase();
                let (count, size) = stats.entry(extension).or_insert((0, 0));
                *count += 1;
                *size += message.size;
                total_size += message.size;
                total_count += 1;

                println!(
                    "{} {}\t({} / {})",
                    checksum,
                    message.key,
                    utils::display_bytes(message.size),
                    message
                        .last_modified
                        .and_then(|d| d.fmt(aws_smithy_types::date_time::Format::DateTime).ok())
                        .unwrap_or_else(|| "[Unknown last modified date]".into())
                );

                if let Some(writer) = &mut writer {
                    writer
                        .write_all(format!("{} {}\n", checksum, message.key).as_bytes())
                        .await?;
                }

                if message
                    .expected_checksum
                    .as_ref()
                    .map(|c| !c.eq(&checksum))
                    .unwrap_or_default()
                {
                    println!("ERROR: {}:\tChecksum mismatch", message.key);
                    errors.insert(
                        message.key,
                        format!(
                            "Actual {} does not match expected {}",
                            checksum,
                            message.expected_checksum.unwrap(),
                        ),
                    );
                }
            }
            Err(err) => {
                println!("ERROR: {}:\tUnable to calculate checksum", message.key);
                errors.insert(message.key, format!("{}", err));
            }
        };
    }

    println!("\nFile Counts:");
    for (name, (count, size)) in stats.iter().sorted() {
        println!(
            "{}:\t{} files ({})",
            name,
            count,
            utils::display_bytes(*size)
        );
    }
    println!(
        "Total: {} files ({})",
        total_count,
        utils::display_bytes(total_size)
    );

    if args.check.is_some() && errors.is_empty() {
        println!("\nAll checksums match!");
    } else if !errors.is_empty() {
        println!("\nError count: {}", errors.len());
        for (key, err) in errors.iter().sorted() {
            println!("{}:\t{}", key, err);
        }
    }

    if let Some(mut writer) = writer {
        writer.flush().await?;
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("Checksum failure"))
    }
}
