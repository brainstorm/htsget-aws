# Serverless htsget

Proof of concept for a htsget serverless implementation, using AWS lambdas and Rust as a runtime. This is a work in progress still, [rust-htslib](https://github.com/rust-bio/rust-htslib/pull/193), htslib and [bio-index-formats](https://github.com/brainstorm/bio-index-formats) need more work.

NOTE: The previous implementation with AWS Athena [was shelved](https://github.com/brainstorm/htsget-aws/tree/athena) due to latency issues.

# Quickstart

To deploy the readId/variantId endpoints in the API Gateway, just run the **serverless.com** deployment script(s).

```bash
npx serverless deploy
```

# Development

Some handy dev shortcuts:

```bash
$ sls plugin install -n serverless-offline
$ sls deploy && sls invoke -f reads --path tests/rest/apigw_proxy_request.json
```
