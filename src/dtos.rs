use aws_smithy_types::DateTime;
use clap::Parser;
use strum::Display;

const DEFAULT_WORKER_THREAD_COUNT: u8 = 4;

#[derive(Clone, Copy, Display, clap::ValueEnum)]
#[strum(serialize_all = "snake_case")]
pub enum Algorithm {
    Sha1,
    Sha256,
    Sha512,
}

impl Algorithm {
    pub fn get_hex_hash_len(self) -> usize {
        match self {
            Algorithm::Sha1 => 40,
            Algorithm::Sha256 => 64,
            Algorithm::Sha512 => 128,
        }
    }
}

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

    /// Local checksum file to load and verify [default: calculates and creates checksum file instead]
    #[clap(short, long, value_parser)]
    pub check: Option<String>,

    /// Checksum hashing algorithm to use
    #[clap(short, long, value_enum, default_value_t = Algorithm::Sha256)]
    pub algorithm: Algorithm,

    /// Number of threads to start for parallel downloading
    #[clap(short, long, value_parser, default_value_t = DEFAULT_WORKER_THREAD_COUNT)]
    pub threads: u8,

    /// Custom AWS endpoint [default: real AWS]
    #[clap(short, long, value_parser)]
    pub url: Option<String>,
}

pub struct FileToProcess {
    pub key: String,
    pub checksum: Option<String>,
}

#[derive(Debug)]
pub struct ProcessedFile {
    pub key: String,
    pub size: i64, // bytes,
    pub actual_checksum: String,
    pub expected_checksum: Option<String>,
    pub last_modified: Option<DateTime>,
}
