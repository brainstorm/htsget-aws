from zipfile import ZipFile
import shutil
import boto3
from aws_cdk import (
    aws_lambda as lambda_,
    aws_s3 as s3,
    core,
)

s3c = boto3.client('s3')

CA_BUNDLE="ca.pem"
BUCKET="umccr-research-dev"
KEY="htsget/app/reads.zip"
ASSET="reads.zip"

# Creates reference to already existing s3 bucket and lambda code
class htsgetLambda(core.Stack):
    def __init__(self, app: core.App, id: str) -> None:
        super().__init__(app, id)

        lambda_bucket = s3.Bucket.from_bucket_attributes(
            self, 'LambdaCodeBucket',
            bucket_name=BUCKET
        )

        lambdaFn = lambda_.Function(
            self, 'htsget',
            handler='main',
            code=lambda_.Code.asset(ASSET),
            runtime=lambda_.Runtime.PROVIDED,
            timeout=core.Duration.seconds(10)
        )

        lambda_bucket.grant_read(lambdaFn, "htsget/*")
        lambdaFn.add_environment("CURL_CA_BUNDLE", CA_BUNDLE)

app = core.App()

# Pack for lambda PROVIDED runtime (must be a .zip)...
with ZipFile("reads.zip", 'w') as fzip:
    fzip.write("../target/x86_64-unknown-linux-musl/release/bootstrap", "bootstrap")
    fzip.write(CA_BUNDLE)       

# ... and ship it!
htsgetLambda(app, "htsgetLambda")
app.synth()
