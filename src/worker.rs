use std::sync::Arc;

use anyhow::Result;
use aws_sdk_s3::model::Object;
use sha2::{Digest, Sha256};
use tokio::sync::{mpsc::Sender, Mutex};
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;

use crate::dtos::ProcessedFile;

pub fn spawn(
    tx: Sender<ProcessedFile>,
    s3_client: aws_sdk_s3::Client,
    bucket: String,
    files: Arc<Vec<Object>>,
    index: Arc<Mutex<usize>>,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move { worker_loop(tx, s3_client, bucket, files, index).await })
}

async fn worker_loop(
    tx: Sender<ProcessedFile>,
    s3_client: aws_sdk_s3::Client,
    bucket: String,
    files: Arc<Vec<Object>>,
    index: Arc<Mutex<usize>>,
) -> Result<()> {
    loop {
        let mut guard = index.lock().await;
        if *guard >= files.len() {
            return Ok(());
        }
        let file = &files[*guard];
        *guard += 1;
        drop(guard);

        if let Err(err) = calculate_checksum(&tx, &s3_client, &bucket, file).await {
            println!(
                "ERROR: Unable to process {}: {}",
                file.key.as_ref().unwrap(),
                err
            );
        }
    }
}

async fn calculate_checksum(
    tx: &Sender<ProcessedFile>,
    s3_client: &aws_sdk_s3::Client,
    bucket: &str,
    file: &Object,
) -> Result<()> {
    let key = file.key.as_ref().unwrap();
    let mut resp = s3_client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    let mut hasher = Sha256::new();
    while let Some(bytes) = resp.body.try_next().await? {
        hasher.update(&bytes);
    }
    let checksum = hex::encode(hasher.finalize());

    tx.send(ProcessedFile {
        checksum,
        key: key.to_string(),
        size: file.size,
        last_modified: file.last_modified,
    })
    .await?;

    Ok(())
}
