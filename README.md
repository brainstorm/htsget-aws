# Serverless htsget

Proof of concept for a htsget serverless implementation, using AWS lambdas and Rust as a runtime. This is a work in progress still, rust-htslib, htslib and [bio-index-formats](https://github.com/brainstorm/bio-index-formats) need more work.

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
