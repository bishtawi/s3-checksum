use crate::dtos::{Algorithm, Args, FileToProcess};

use std::sync::Arc;

use anyhow::{anyhow, Result};
use tokio::sync::mpsc::Receiver;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub fn spawn(
    rx: Receiver<oneshot::Sender<FileToProcess>>,
    s3_client: aws_sdk_s3::Client,
    args: Arc<Args>,
) -> JoinHandle<Result<()>> {
    tokio::spawn(async move { get_list_of_files(rx, s3_client, args).await })
}

async fn get_list_of_files(
    rx: Receiver<oneshot::Sender<FileToProcess>>,
    s3_client: aws_sdk_s3::Client,
    args: Arc<Args>,
) -> Result<()> {
    if let Some(file) = &args.check {
        parse_checksum_file(rx, file, args.algorithm, &args.path).await
    } else {
        crawl_bucket(rx, s3_client, &args.bucket, &args.path).await
    }
}

async fn parse_checksum_file(
    mut rx: Receiver<oneshot::Sender<FileToProcess>>,
    file: &str,
    algorithm: Algorithm,
    path: &Option<String>,
) -> Result<()> {
    let mut lines = BufReader::new(File::open(file).await?).lines();
    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(index) = line.chars().position(char::is_whitespace) {
            let hash = line[..index].trim();
            let file = line[index + 1..].trim();

            if path
                .as_ref()
                .map(|p| !file.starts_with(p))
                .unwrap_or_default()
            {
                continue;
            }

            assert_eq!(
                hash.len(),
                algorithm.get_hex_hash_len(),
                "Hash length in file does not match algorithm {}",
                algorithm
            );

            distribute_work(
                &mut rx,
                FileToProcess {
                    key: file.into(),
                    checksum: Some(hash.to_lowercase()),
                },
            )
            .await?;
        } else {
            println!("WARNING: Skipping line '{}'", line);
        }
    }

    Ok(())
}

async fn crawl_bucket(
    mut rx: Receiver<oneshot::Sender<FileToProcess>>,
    s3_client: aws_sdk_s3::Client,
    bucket: &String,
    path: &Option<String>,
) -> Result<()> {
    let mut continuation_token: Option<String> = None;
    loop {
        let resp = s3_client
            .list_objects_v2()
            .bucket(bucket)
            .set_prefix(path.clone())
            .set_continuation_token(continuation_token)
            .send()
            .await?;

        if let Some(contents) = resp.contents {
            for object in contents {
                distribute_work(
                    &mut rx,
                    FileToProcess {
                        key: object.key.unwrap(),
                        checksum: None,
                    },
                )
                .await?;
            }
        }

        if resp.next_continuation_token.is_none() {
            break;
        }

        continuation_token = resp.next_continuation_token;
    }

    Ok(())
}

async fn distribute_work(
    rx: &mut Receiver<oneshot::Sender<FileToProcess>>,
    file: FileToProcess,
) -> Result<()> {
    let tx = rx
        .recv()
        .await
        .ok_or_else(|| anyhow!("need_work channel is closed"))?;
    tx.send(file)
        .map_err(|_| anyhow!("oneshot channel is closed"))
}
