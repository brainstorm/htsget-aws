use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;

use reads::{Format, Class, htsget_response, htsget_request, reference_ids, bucket_obj_bytes};
use bio_index_formats::parser_bai::parse_bai;

use std::io::Read;
use rusoto_core::Region;
use rusoto_s3::S3Client;
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

    let region = Region::default();
    let s3 = S3Client::new(region);

    let bucket = "umccr-misc-temp".to_string();
    let obj_bam_path = "htsget_sample.bam".to_string();
    let obj_bai_path = "htsget/sample.bam.bai".to_string();
    let chrom = "11".to_string();
    let chrom_start = 4999976 as u32;
    let chrom_end = 5002147 as u32;
    let auth = "Bearer: foo".to_string();

    // Get BAI from AWS
    let bai_stream = bucket_obj_bytes(s3, bucket.clone(), obj_bai_path);

    // Convert ByteStream to slice of bytes (&[u8]'s)
    let mut bai_vec = Vec::new();
    bai_stream.map(|astream| astream.into_blocking_read().read_to_end(&mut bai_vec));

    // Parse BAI
    let bai = parse_bai(&bai_vec);
    let refs = bai.map(|r| r.1.refs).unwrap();

    // Get "symbols" such as chr1 from BAM header
    let ref_ids = reference_ids(obj_bam_path);
    let ref_id = ref_ids.iter().position(|name| name == &chrom).unwrap();
    let reference = &refs[ref_id];

    // Send request and fetch response
    let range = htsget_request(reference, chrom_start, chrom_end);
    let res = htsget_response(auth, range, bucket.clone(), Format::BAM, Class::Body);

    Ok(json!(res))
}
