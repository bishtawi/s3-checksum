# s3-checksum

CLI tool written in Rust that crawls an AWS S3 bucket and calculates the checksum (SHA1, SHA256 or SHA512) of every file in said bucket.

Default behavior will calculate SHA256 checksums and will write all checksums to a `<bucket_name>.sha256.checksum` file.

Leverages parallelism to speed up the process.

## Usage

```bash
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

# Example

First you need to set some AWS environment variables (your credentials need to grant you read access to the bucket):
```bash
$ export AWS_ACCESS_KEY_ID=xxx
$ export AWS_SECRET_ACCESS_KEY=yyy
$ export AWS_REGION=us-east-1
```

Here is a command to find all the files in the `elasticmapreduce` bucket under the `bootstrap-actions/log4j/` folder, calculate the SHA256 hash of each file, and write them to `elasticmapreduce.sha256.checksum`:

```bash
$ s3-checksum --bucket elasticmapreduce --path bootstrap-actions/log4j/
6359fc22944cd4f6e4a0afb610983769c190b94322ec225205800bd9028442ef bootstrap-actions/log4j/patch-log4j-emr-5.11.4-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
e396488e99476526f4e1c1b31b5c932fe7e7aeda3371cb646837a18f5c76cacf bootstrap-actions/log4j/patch-log4j-emr-5.10.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
fcf12974182674cfcb45876173658af452c22c1c384c56c24bd74b76282b3aaa bootstrap-actions/log4j/patch-log4j-emr-5.12.3-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
93595a43cb6c04c6fd5589c721f5ac9180304c616a77ae41f2c8909cdf3f0d7e bootstrap-actions/log4j/patch-log4j-emr-5.14.2-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
1bb01dbe608c70f94231cd6d837c1d2786eab5ad8523d8b6b465577b4406251f bootstrap-actions/log4j/patch-log4j-emr-5.13.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
2c7314ffa16a463db3094f8028386414262618310f1021781ed0c74b42a824a6 bootstrap-actions/log4j/patch-log4j-emr-5.15.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
bf6d579cb72f774019dc8bc394c3a7561cb4bedc67d461baaf32d5129a2a25ff bootstrap-actions/log4j/patch-log4j-emr-5.16.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
96c56a5fe0e3d8a4afee87209916f5dadda82d5a00f3761a3cfea1448a93a2b9 bootstrap-actions/log4j/patch-log4j-emr-5.18.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
a4349d6b67d71bf3f444df8b5ca69bd91fd8ca4b5f689ae166193700028dfbcf bootstrap-actions/log4j/patch-log4j-emr-5.17.2-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
c70bc9530bc4a717ce7014a2a27e5b75d21b75c5fc70791001be5be83c7a3aca bootstrap-actions/log4j/patch-log4j-emr-5.19.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
7068fde2bee3aaff0f19793a163396aa9f755fcbe3f6a9a46b8ae8a0ede4a9ad bootstrap-actions/log4j/patch-log4j-emr-5.20.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
a54577cae3b75592259185c522e2d497bc84031efd0822be9dcf3d08a395c2a5 bootstrap-actions/log4j/patch-log4j-emr-5.22.0-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
20b8c7f9092afb7a131f2b51e1da7b784e6977ce1eae6248ed05ac7a39a02a45 bootstrap-actions/log4j/patch-log4j-emr-5.21.2-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
26b59b4cbdea32d74c53062037753d603e2d1b0db5be1ab0a47214124265e06f bootstrap-actions/log4j/patch-log4j-emr-5.23.1-v1.sh (3.83 kilobytes / 2021-12-16T02:25:31Z)
5d15139560244d02ae487dc6dba63f97a376e2f50df582887172be7d04a5a9dd bootstrap-actions/log4j/patch-log4j-emr-5.24.1-v1.sh (3.90 kilobytes / 2021-12-16T02:25:31Z)
45b915eac789337c06268f15278bb4157c8e4b134922d74c5aa3d684307099f9 bootstrap-actions/log4j/patch-log4j-emr-5.26.0-v1.sh (3.90 kilobytes / 2021-12-16T02:25:31Z)
9c5dffa3cfc8e117f6f5d742d85ec5e440e868fdec144f0e3122c9834bacc542 bootstrap-actions/log4j/patch-log4j-emr-5.25.0-v1.sh (3.89 kilobytes / 2021-12-16T02:25:31Z)
df2e7a6de9fd256378fecf01ab6173622bb98e2179cd583a5e6927d4637eb0c3 bootstrap-actions/log4j/patch-log4j-emr-5.27.1-v1.sh (3.90 kilobytes / 2021-12-16T02:25:31Z)
a44c9865e54c4c60f686ed12303347a8885cc2d44ae16abce44924952b9d1ada bootstrap-actions/log4j/patch-log4j-emr-5.29.0-v1.sh (4.22 kilobytes / 2021-12-16T02:25:31Z)
46dffcbf33662ad3491e65b344e3539e980b881d4f55de42fcf7ec60779ceebc bootstrap-actions/log4j/patch-log4j-emr-5.30.2-v1.sh (4.20 kilobytes / 2021-12-16T02:25:31Z)
097010cf1a5405722eeaaa27e4f80a2999c5446827bee4bc8d9312d08e1d8d24 bootstrap-actions/log4j/patch-log4j-emr-5.28.1-v1.sh (4.22 kilobytes / 2021-12-16T02:25:31Z)
cedd6ca36123bff97cb5dd58d2fb7eacf534c8dafde1b10dc4f27ef90a483847 bootstrap-actions/log4j/patch-log4j-emr-5.31.1-v1.sh (4.46 kilobytes / 2021-12-16T02:25:31Z)
ecff25db23d746ddfdb6c54ce51ad11433364f395ea0c9b0164be66e12a4fc73 bootstrap-actions/log4j/patch-log4j-emr-5.32.1-v1.sh (4.46 kilobytes / 2021-12-16T02:25:31Z)
d11b49cb4f9276059a4f3d944194218c20c8733f947476fd77e16ac383f4b449 bootstrap-actions/log4j/patch-log4j-emr-5.34.0-v1.sh (4.69 kilobytes / 2021-12-16T02:25:31Z)
f3d8d47cc24f4e10dfa8b4ed7f1a8a7f06b55b936aed391da70bff833903c8ad bootstrap-actions/log4j/patch-log4j-emr-5.33.1-v1.sh (4.69 kilobytes / 2021-12-16T02:25:31Z)
d9fbd4c37bd27a9f78fbd82d72630ea596e41fc9c909f74409afb814239a6a8d bootstrap-actions/log4j/patch-log4j-emr-5.7.1-v1.sh (3.76 kilobytes / 2021-12-16T02:25:31Z)
721bb2251ab0d81ecd5c87820004af1a12d1e201d23d543d7706982fc55d981a bootstrap-actions/log4j/patch-log4j-emr-5.9.1-v1.sh (3.82 kilobytes / 2021-12-16T02:25:31Z)
eeecb79e62630a2930ccee8c2ce6303dd41874d0cd36512b554480fd8bafd0a6 bootstrap-actions/log4j/patch-log4j-emr-6.0.1-v1.sh (4.14 kilobytes / 2021-12-16T02:25:31Z)
53ad08a2064dd854a61996d95241c38f0e3638cf16aa033c2831df2d6f434fc1 bootstrap-actions/log4j/patch-log4j-emr-5.8.3-v1.sh (3.82 kilobytes / 2021-12-16T02:25:31Z)
755ba53f5458bdad342e239fd906dc25fe19603d8d3eaeeca38b5a41e6b4b336 bootstrap-actions/log4j/patch-log4j-emr-6.1.1-v1.sh (4.46 kilobytes / 2021-12-16T02:25:31Z)
4d0e7a4e1f1f60f8a54464440c8578105e46a2da24f23292d6d89608c5b63796 bootstrap-actions/log4j/patch-log4j-emr-6.2.1-v1.sh (4.95 kilobytes / 2022-03-24T18:42:57Z)
5e3f733ef8d9e738d55a8ef56b84958b21b673f4317b23e38d4f9046ea5f00de bootstrap-actions/log4j/patch-log4j-emr-6.3.1-v1.sh (5.17 kilobytes / 2022-03-24T18:43:02Z)
d580180255553d84ced56f30b6d16105310dbfc75106089a01604dc652873e89 bootstrap-actions/log4j/patch-log4j-emr-6.4.0-v1.sh (5.41 kilobytes / 2022-03-24T18:43:08Z)
dc3483370b0ebf5d403e9e673f76f4b46955412abb55dd74aec4f438dab034fc bootstrap-actions/log4j/patch-log4j-emr-6.5.0-v1.sh (5.23 kilobytes / 2022-03-24T18:43:14Z)

File Counts:
sh: 34 files (141.00 kilobytes)
Total: 34 files (141.00 kilobytes)
```

In addition to calculating the checksums, statistics (file size, file count, last modified date) are also generated.

The above command will write just the checksums (no file statistics) to a file named `elasticmapreduce.sha256.checksum`.

And omit the `--path` argument and crawl the full bucket in its entirety.

Given a checksum file, you can validate all the hashes in it:
```bash
$ s3-checksum --bucket elasticmapreduce --check ./elasticmapreduce.sha256.checksum
```

While the tool defaults to SHA256, you can pass in the argument `--algorithm` to use SHA1 or SHA512 instead.

Note: While SHA1 support is implemented, you should avoid using SHA1 as it is considered broken.