use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};
//use bio_index_formats::parser_bai::parse_bai;

use serde_json::json;
use reads::{Format, htsget_response, Class};

fn main() {
    // Init env logger for debugging: https://www.rusoto.org/debugging.html
    let _ = env_logger::try_init();
    lambda!(handler);
}

fn handler(
    _req: Request,
    _ctx: Context,
) -> Result<impl IntoResponse, HandlerError> {

    let url = "https://some_presigned_url".to_string();
    let auth = "Bearer: foo".to_string();
    let range = "bytes = 1-100".to_string();

    let htsget = htsget_response(auth, range,
                                                        url, Format::BAM, Class::Body);

    //let htsget = serde_json::to_string(&container)?;

    Ok(json!(htsget))
}
