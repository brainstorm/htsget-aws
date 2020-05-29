from aws_cdk import (
    aws_lambda as lambda_,
    aws_s3 as s3,
    core,
)

# Creates reference to already existing s3 bucket and lambda code
class htsgetLambda(core.Stack):
    def __init__(self, app: core.App, id: str) -> None:
        super().__init__(app, id)

        lambda_code_bucket = s3.Bucket.from_bucket_attributes(
            self, 'LambdaCodeBucket',
            bucket_name='umccr-research-dev'
        )

        lambdaFn = lambda_.Function(
            self, 'Singleton',
            handler='main',
            code=lambda_.S3Code(
                bucket=lambda_code_bucket,
                key='htsget/app/reads.zip'
            ),
            runtime=lambda_.Runtime.PROVIDED,
            timeout=core.Duration.seconds(10)
        )

        lambda_code_bucket.grant_read(lambdaFn, "htsget/*")


app = core.App()
htsgetLambda(app, "htsgetLambda")
app.synth()
