use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

use htsget::query::read_header;

fn main() {
    lambda!(handler)
}

fn handler(
    req: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response

    // 1. Get full url "segments", parse them accordingly, can be free-form such as /data/platinum/NA12878
    //    or even have "Inline data block URIs".
    Ok(json!({
        "message": "Reads: Your function executed successfully!",
        "bam_header": read_header(),
        "request_body": req.body(),
        "req": req.uri().path()
    }))
}