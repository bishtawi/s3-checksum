use anyhow::Result;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;

use crate::dtos::{Algorithm, FileToProcess, ProcessedFile};
use crate::hasher::Hasher;

pub fn spawn(
    need_work_tx: Sender<oneshot::Sender<FileToProcess>>,
    processed_file_tx: Sender<ProcessedFile>,
    algorithm: Algorithm,
    s3_client: aws_sdk_s3::Client,
    bucket: String,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move {
        worker_loop(
            need_work_tx,
            processed_file_tx,
            algorithm,
            s3_client,
            bucket,
        )
        .await
    })
}

async fn worker_loop(
    need_work_tx: Sender<oneshot::Sender<FileToProcess>>,
    processed_file_tx: Sender<ProcessedFile>,
    algorithm: Algorithm,
    s3_client: aws_sdk_s3::Client,
    bucket: String,
) -> Result<()> {
    loop {
        let (tx, rx) = oneshot::channel();
        if need_work_tx.send(tx).await.is_err() {
            break;
        }
        match rx.await {
            Err(_) => break,
            Ok(file) => match calculate_checksum(algorithm, &s3_client, &bucket, &file).await {
                Ok(file) => {
                    processed_file_tx.send(file).await?;
                }
                Err(err) => {
                    processed_file_tx
                        .send(ProcessedFile {
                            key: file.key.to_string(),
                            size: 0,
                            actual_checksum: Err(err),
                            expected_checksum: file.checksum.clone(),
                            last_modified: None,
                        })
                        .await?;
                }
            },
        }
    }

    Ok(())
}

async fn calculate_checksum(
    algorithm: Algorithm,
    s3_client: &aws_sdk_s3::Client,
    bucket: &str,
    file: &FileToProcess,
) -> Result<ProcessedFile> {
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

    Ok(ProcessedFile {
        key: file.key.to_string(),
        size: resp.content_length,
        actual_checksum: Ok(checksum),
        expected_checksum: file.checksum.clone(),
        last_modified: resp.last_modified,
    })
}
