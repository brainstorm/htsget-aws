use lambda_http::{
    handler,
    lambda::{self, Context},
    IntoResponse, Request//, RequestExt, Response,
};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

use bio_index_formats::parser_bai::parse_bai;
use reads::{
     bam_header, bam_bai_to_ref, htsget_request, htsget_response, s3_getobj_to_bytes, Class, Format,
};

use rusoto_core::Region;
use rusoto_s3::S3Client;

use serde_json::json;


#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda::run(handler(func)).await?;
    Ok(())
}

async fn func(_event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let region = Region::default();
    let s3 = S3Client::new(region);

    let bucket = "umccr-research-dev".to_string();
    let obj_bam = "htsget/htsnexus_test_NA12878.bam".to_string();
    let obj_bai = "htsget/htsnexus_test_NA12878.bam.bai".to_string();
    let _query = "11".to_string();
    let chrom_start = 4999976 as u32;
    let chrom_end = 5002147 as u32;
    let auth = "Bearer: foo".to_string();

    // Get BAI from S3 
    let bai = s3_getobj_to_bytes(s3, bucket.clone(), obj_bai).await;

    // Parse BAI
    let bai_bytes = parse_bai(&bai);
    let bai_refs = bai_bytes.map(|r| r.1.refs).unwrap();

    // Fetch BAM header
    let bam_hdr = bam_header(
        bucket.clone(),
        obj_bam,
    );

    // Given BAM header and BAI bins, find references
    let reference = bam_bai_to_ref(bam_hdr, bai_refs.clone());
    
    // Send request and fetch response
    let range = htsget_request(&reference, chrom_start, chrom_end);
    let res = htsget_response(auth, range, bucket.clone(), Format::BAM, Class::Body);

    //Ok( CustomOutput{ body: json!(res).to_string() })
    Ok(json!(res))
}