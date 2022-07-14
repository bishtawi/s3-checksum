#![forbid(unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use futures_util::future::join_all;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

mod aws;
mod dtos;
mod worker;

#[tokio::main]
async fn main() -> Result<()> {
    let args = dtos::Args::parse();
    let aws_config = aws::get_aws_config(&args.url).await?;
    let s3_client = aws_sdk_s3::Client::new(&aws_config);

    let files = Arc::new(aws::crawl_bucket(&s3_client, &args.bucket, &args.path).await?);
    let index = Arc::new(Mutex::new(0_usize));
    let (tx, mut rx) = mpsc::channel(32);

    let handles: Vec<JoinHandle<Result<()>>> = (0..args.threads)
        .map(|_| {
            worker::spawn(
                tx.clone(),
                s3_client.clone(),
                args.bucket.clone(),
                files.clone(),
                index.clone(),
            )
        })
        .collect();
    drop(tx); // main thread doesnt need the sender channel

    while let Some(message) = rx.recv().await {
        println!(
            "{} {}\t({} bytes / {})",
            message.checksum,
            message.key,
            message.size,
            message
                .last_modified
                .and_then(|d| d.fmt(aws_smithy_types::date_time::Format::DateTime).ok())
                .unwrap_or_else(|| "[Unknown last modified date]".into())
        );
    }

    for res in join_all(handles).await {
        res??;
    }

    Ok(())
}
