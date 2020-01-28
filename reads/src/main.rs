use lambda_http::{lambda, IntoResponse, Request};
use lambda_runtime::{error::HandlerError, Context};

use reads::{Format, Class, htsget_response, htsget_request, reference_ids, bucket_obj_bytes};
use bio_index_formats::parser_bai::parse_bai;

use futures::stream::StreamExt;
use rusoto_core::Region;
use rusoto_s3::S3Client;
use serde_json::json;

// https://github.com/awslabs/aws-lambda-rust-runtime/issues/14#issuecomment-569046122
//#[tokio::main]
//async fn main() {
fn main() {
    // Init env logger for debugging: https://www.rusoto.org/debugging.html
    let _ = env_logger::try_init();
    lambda!(handler);
}

async fn handler(
//fn handler(
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
    let bai_stream = bucket_obj_bytes(s3, bucket, obj_bai_path).await.unwrap();


    // https://stackoverflow.com/questions/57810173/streamed-upload-to-s3-with-rusoto/59884256#comment102487432_57812269
    // let byte_stream = FramedRead::new(bai_bytes, BytesCodec::new()).map(|r| r.freeze())?;

    /*
    Sadly it's become a bit painful to concatenate a stream of Bytes at the moment
    (see: https://github.com/tokio-rs/bytes/issues/324)
    In the meantime doing a while let Some(bytes) = stream.next().await { ... }
    and extend some Vec<u8> inside the loop then pass that to parse_bai
    */

    let mut bai_iter = bai_stream;
    let mut bai_vec = Vec::<u8>::new();
    while let Some(b) = bai_iter.next().await {
        bai_vec.push(b.unwrap()[0]);
    }

    // Parse BAI
    let bai = parse_bai(&bai_vec);
    let refs = bai.map(|r| r.1.refs).unwrap();

    // Get "symbols" such as chr1 from BAM header
    let ref_ids = reference_ids(obj_bam_path);
    let ref_id = ref_ids.iter().position(|name| name == &chrom).unwrap();
    let reference = &refs[ref_id];

    // Send request and fetch response
    let range = htsget_request(reference, chrom_start, chrom_end);
    let res = htsget_response(auth, range, bucket, Format::BAM, Class::Body);

    Ok(json!(res))
}
