use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
use bio_index_formats::parser_bai::parse_bai;
use serde_json::json;

fn main() {
    // Init env logger for debugging: https://www.rusoto.org/debugging.html
    let _ = env_logger::try_init();
    lambda!(handler);
}

fn handler(
    _req: Request,
    _ctx: Context,
) -> Result<impl IntoResponse, HandlerError> {
    Ok(json!({
        "message": "Reads: Your function executed successfully!"
    }))
}
