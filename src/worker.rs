use std::sync::Arc;

use anyhow::Result;
use tokio::sync::{mpsc::Sender, Mutex};
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;

use crate::dtos::{Algorithm, FileToProcess, ProcessedFile};
use crate::hasher::Hasher;

pub fn spawn(
    tx: Sender<ProcessedFile>,
    algorithm: Algorithm,
    s3_client: aws_sdk_s3::Client,
    bucket: String,
    files: Arc<Vec<FileToProcess>>,
    index: Arc<Mutex<usize>>,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move { worker_loop(tx, algorithm, s3_client, bucket, files, index).await })
}

async fn worker_loop(
    tx: Sender<ProcessedFile>,
    algorithm: Algorithm,
    s3_client: aws_sdk_s3::Client,
    bucket: String,
    files: Arc<Vec<FileToProcess>>,
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

        if let Err(err) = calculate_checksum(&tx, algorithm, &s3_client, &bucket, file).await {
            println!("ERROR: Unable to process {}: {}", file.key, err);
        }
    }
}

async fn calculate_checksum(
    tx: &Sender<ProcessedFile>,
    algorithm: Algorithm,
    s3_client: &aws_sdk_s3::Client,
    bucket: &str,
    file: &FileToProcess,
) -> Result<()> {
    let mut resp = s3_client
        .get_object()
        .bucket(bucket)
        .key(&file.key)
        .send()
        .await?;

    let mut hasher = Hasher::new(algorithm);
    while let Some(bytes) = resp.body.try_next().await? {
        hasher.write(&bytes);
    }
    let checksum = hasher.finish();

    tx.send(ProcessedFile {
        key: file.key.to_string(),
        size: resp.content_length,
        actual_checksum: checksum,
        expected_checksum: file.checksum.clone(),
        last_modified: resp.last_modified,
    })
    .await?;

    Ok(())
}
