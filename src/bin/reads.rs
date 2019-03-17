use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use serde_json::json;

use htsget::bam::read_header;

fn main() {
    lambda!(handler)
}

fn handler(
    _: Request,
    _: Context,
) -> Result<impl IntoResponse, HandlerError> {
    // `serde_json::Values` impl `IntoResponse` by default
    // creating an application/json response
    Ok(json!({
        "message": "Reads: Your function executed successfully!",
        "bam_header": read_header()
    }))
}