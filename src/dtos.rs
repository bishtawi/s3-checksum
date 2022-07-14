use aws_smithy_types::DateTime;
use clap::Parser;

const DEFAULT_WORKER_THREAD_COUNT: u8 = 4;

/// Crawl an S3 bucket to calculate checksums and other statistics
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of S3 bucket to crawl [required]
    #[clap(short, long, value_parser)]
    pub bucket: String,

    /// Folder path inside bucket to crawl [default: root]
    #[clap(short, long, value_parser)]
    pub path: Option<String>,

    /// Number of threads to start for parallel downloading
    #[clap(short, long, value_parser, default_value_t = DEFAULT_WORKER_THREAD_COUNT)]
    pub threads: u8,

    /// Custom AWS endpoint [default: real AWS]
    #[clap(short, long, value_parser)]
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct ProcessedFile {
    pub key: String,
    pub size: i64, // bytes,
    pub checksum: String,
    pub last_modified: Option<DateTime>,
}
