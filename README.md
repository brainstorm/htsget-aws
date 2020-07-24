# Serverless htsget

This repository is an ongoing htsget serverless implementation, using AWS lambdas and Rust as a runtime. 

See [s3-rust-htslib-bam](https://github.com/brainstorm/s3-rust-htslib-bam) repository for a more bare-bones proof of concept on how to run generic file-format/htslib oriented Bioinformatics on Lambda!

A previous implementation leveraging AWS Athena (and ADAM schema) [was shelved due to latency issues, feel free to have a look at the code on that branch](https://github.com/brainstorm/htsget-aws/tree/athena).

# Quickstart

AWS lambdas require MUSL static binaries. To build this project you must use `cross` instead of `cargo` (unless you know what you are doing). The deployment is achieved via [AWS CDK Python, please install the appropriate python environment to make it work beforehand](https://aws.amazon.com/blogs/developer/getting-started-with-the-aws-cloud-development-kit-and-python/).

```
$ export CFLAGS="-I/usr/local/musl/include"
$ cross build --target x86_64-unknown-linux-musl
$ cd deploy && cdk deploy
```