#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::{collections::HashMap, ffi::OsStr, path::Path, sync::Arc};

use anyhow::{anyhow, Result};
use clap::Parser;
use futures_util::future::join_all;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, BufWriter};
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

mod aws;
mod dtos;
mod hasher;
mod utils;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    let args = dtos::Args::parse();
    let aws_config = aws::get_aws_config(&args.url).await?;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    let files = Arc::new(utils::get_list_of_files(&args, &s3_client).await?);
    let index = Arc::new(Mutex::new(0_usize));
    let (tx, mut rx) = mpsc::channel(32);

    let mut writer = if args.check.is_none() {
        Some(BufWriter::new(
            File::create(format!("{}.{}.checksum", args.bucket, args.algorithm)).await?,
        ))
    } else {
        None
    };

    let handles: Vec<JoinHandle<Result<()>>> = (0..args.threads)
        .map(|_| {
            worker::spawn(
                tx.clone(),
                args.algorithm,
                s3_client.clone(),
                args.bucket.clone(),
                files.clone(),
                index.clone(),
            )
        })
        .collect();
    drop(tx); // main thread doesnt need the sender channel

    let mut total_size = 0;
    let mut stats: HashMap<String, (u64, i64)> = HashMap::new(); // (count, size)
    let mut errors: HashMap<String, (String, String)> = HashMap::new(); // (actual, expected)
    while let Some(message) = rx.recv().await {
        println!(
            "{} {}\t({} bytes / {})",
            message.actual_checksum,
            message.key,
            message.size,
            message
                .last_modified
                .and_then(|d| d.fmt(aws_smithy_types::date_time::Format::DateTime).ok())
                .unwrap_or_else(|| "[Unknown last modified date]".into())
        );

        if let Some(writer) = &mut writer {
            writer
                .write_all(format!("{} {}\n", message.actual_checksum, message.key).as_bytes())
                .await?;
        }

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

        if message
            .expected_checksum
            .as_ref()
            .map(|c| !c.eq(&message.actual_checksum))
            .unwrap_or_default()
        {
            errors.insert(
                message.key,
                (message.actual_checksum, message.expected_checksum.unwrap()),
            );
        }
    }

    if let Some(mut writer) = writer {
        writer.flush().await?;
    }

    println!("\nFile Counts:");
    for (name, (count, size)) in &stats {
        println!("{}: {} files ({} bytes)", name, count, size);
    }
    println!("Total: {} files ({} bytes)", files.len(), total_size);

    if args.check.is_some() && errors.is_empty() {
        println!("\nAll checksums match!");
    } else if !errors.is_empty() {
        println!("\nError count: {}", errors.len());
        for (key, (actual, expected)) in &errors {
            println!(
                "{}:\tActual {} does not match expected {}",
                key, actual, expected,
            );
        }
    }

    for res in join_all(handles).await {
        res??;
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(anyhow!("Checksum mismatch"))
    }
}
