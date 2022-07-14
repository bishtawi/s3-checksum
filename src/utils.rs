use crate::{
    aws,
    dtos::{Args, FileToProcess},
};

use anyhow::Result;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn get_list_of_files(
    args: &Args,
    s3_client: &aws_sdk_s3::Client,
) -> Result<Vec<FileToProcess>> {
    if let Some(file) = &args.check {
        return parse_checksum_file(args, file).await;
    }

    let s3_objects = aws::crawl_bucket(s3_client, &args.bucket, &args.path).await?;
    Ok(s3_objects
        .into_iter()
        .filter(|o| o.key.is_some())
        .map(|o| FileToProcess {
            key: o.key.unwrap(),
            checksum: None,
        })
        .collect())
}

async fn parse_checksum_file(args: &Args, file: &str) -> Result<Vec<FileToProcess>> {
    let mut lines = BufReader::new(File::open(file).await?).lines();
    let mut files: Vec<FileToProcess> = Vec::new();
    while let Some(line) = lines.next_line().await? {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(index) = line.chars().position(char::is_whitespace) {
            let hash = line[..index].trim();
            let file = line[index + 1..].trim();

            if args
                .path
                .as_ref()
                .map(|p| !file.starts_with(p))
                .unwrap_or_default()
            {
                continue;
            }

            assert_eq!(
                hash.len(),
                args.algorithm.get_hex_hash_len(),
                "Hash length in file does not match algorithm {}",
                args.algorithm
            );

            files.push(FileToProcess {
                key: file.into(),
                checksum: Some(hash.to_lowercase()),
            });
        } else {
            println!("WARNING: Skipping line '{}'", line);
        }
    }

    Ok(files)
}
