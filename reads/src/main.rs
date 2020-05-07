use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::error::HandlerError;
use lambda_runtime::Context;

use bio_index_formats::parser_bai::parse_bai;
use reads::{
    bam_header, htsget_request, htsget_response, s3_getobj_to_bytes, Class, Format,
};

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

async fn handler(_req: Request, _ctx: Context) -> Result<impl IntoResponse, HandlerError> {
    let region = Region::default();
    let s3 = S3Client::new(region);

    let bucket = "umccr-research-temp".to_string();
    let obj_bam = "htsget/htsnexus_test_NA12878.bam".to_string();
    let obj_bai = "htsget/htsnexus_test_NA12878.bam.bai".to_string();
    let query = "11".to_string();
    let chrom_start = 4999976 as u32;
    let chrom_end = 5002147 as u32;
    let auth = "Bearer: foo".to_string();

    // Get BAI from S3 
    let bai = s3_getobj_to_bytes(s3, bucket.clone(), obj_bai);

    // Parse BAI
    let bai_bytes = parse_bai(&bai.await);
    let bai_refs = bai_bytes.map(|r| r.1.refs).unwrap();

    // Fetch BAM header
    let bam_hdr = bam_header(
        bucket.clone(),
        obj_bam,
    );

    // Given BAM header and BAI bins, find references

    //reference = bam_bai_to_ref();
    
    // Send request and fetch response
    let range = htsget_request(reference, chrom_start, chrom_end);
    let res = htsget_response(auth, range, bucket.clone(), Format::BAM, Class::Body);

    //Ok( CustomOutput{ body: json!(res).to_string() })
    Ok(json!(res))
}
