# s3-checksum

CLI tool written in Rust that crawls an AWS S3 bucket and calculates the checksum (SHA1, SHA256 or SHA512) of every file in said bucket.

Default behavior will calculate SHA256 checksums and will write all checksums to a `<bucket_name>.sha256.checksum` file.

Or you can instead run in `--check path/to/checksum-file` mode which will verify the list of checksums against the files found in the bucket.

Leverages parallelism to speed up the process.

## Usage

```
$ s3-checksum --help
s3-checksum 0.1.0
Crawl an S3 bucket to calculate checksums and other statistics

USAGE:
    s3-checksum [OPTIONS] --bucket <BUCKET>

OPTIONS:
    -a, --algorithm <ALGORITHM>    Checksum hashing algorithm to use [default: sha256] [possible
                                   values: sha1, sha256, sha512]
    -b, --bucket <BUCKET>          Name of S3 bucket to crawl [required]
    -c, --check <CHECK>            Local checksum file to load and verify [default: calculates and
                                   creates checksum file instead]
    -h, --help                     Print help information
    -p, --path <PATH>              Folder path inside bucket to crawl [default: root]
    -t, --threads <THREADS>        Number of threads to start for parallel downloading [default: 4]
    -u, --url <URL>                Custom AWS endpoint [default: real AWS]
    -V, --version                  Print version information
```
