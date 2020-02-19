use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;

use reads::{Format, Class, htsget_response, htsget_request, reference_ids, s3_getobj};
use bio_index_formats::parser_bai::parse_bai;

use rusoto_core::Region;
use rusoto_s3::S3Client;

//use serde::{ Serialize };
use serde_json::json;
use std::path::Path;

// Otherwise the lambda returns "internal" json AWS fields that
// are not part of the raw htsget payload we want
//#[derive(Serialize)]
//struct CustomOutput {
//    body: String,
//}

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
    let obj_bam = Path::new("htsget/htsnexus_test_NA12878.bam");
    let obj_bai = Path::new("htsget/htsnexus_test_NA12878.bam.bai");
    let chrom = "11".to_string();
    let chrom_start = 4999976 as u32;
    let chrom_end = 5002147 as u32;
    let auth = "Bearer: foo".to_string();

    // Get BAI from AWS
    let bai = s3_getobj(s3.clone(), bucket.clone(), obj_bai);

    // Parse BAI
    let bai = parse_bai(&bai);
    let refs = bai.map(|r| r.1.refs).unwrap();

    // Parse BAM (header only)
    let ref_ids = reference_ids(s3.clone(), bucket.clone(), obj_bam);
    let ref_id = ref_ids.iter().position(|name| name == &chrom).unwrap();
    let reference = &refs[ref_id];

    // Send request and fetch response
    let range = htsget_request(reference, chrom_start, chrom_end);
    let res = htsget_response(auth, range, bucket.clone(), Format::BAM, Class::Body);

    //Ok( CustomOutput{ body: json!(res).to_string() })
    Ok(json!(res))
}
