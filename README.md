# Serverless htsget

Proof of concept for a htsget serverless implementation, using AWS lambdas and Rust as a runtime.

# Quickstart

To deploy the readId/variantId endpoints in the API Gateway, just run the **serverless.com** deployment script(s).

```bash
npx serverless deploy --profile default
```

# Development

Some handy dev shortcuts:

```bash
$ sls plugin install -n serverless-offline
$ sls deploy --profile default && sls invoke -f reads --path tests/rest/apigw_proxy_request.json
```
