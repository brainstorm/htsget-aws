use std::str;
use url::Url;
use serde::{ Serialize };

use bio_index_formats::parser_bai::{coffset, Ref};
use bio_index_formats::csi::{ reg2bin };

use rust_htslib::bam::{IndexedReader, Read };
use rusoto_s3::{GetObjectRequest, S3Client, S3};

use tokio::io::AsyncReadExt;

// Htsget request/response bodies as described in spec
// https://samtools.github.io/hts-specs/htsget.html

// Request
#[derive(Debug)]
pub struct ReadsRequest {
    pub url: String,
    pub id: String,
    pub chromosome: String,
    pub start: usize,
    pub end: usize,
}

#[derive(Serialize, Clone, Debug)]
pub struct ReadsRequestHeaders {
    pub auth: String,
    pub byte_range: (u32, u32),
}

// Response
#[derive(Serialize, Clone, Debug)]
pub struct HtsGetResponseContainer {
    pub htsget: HtsGetResponsePayload,
}

#[derive(Serialize, Clone, Debug)]
pub struct HtsGetResponsePayload {
    pub format: Format,
    pub urls: Vec<ReadsResponse>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ReadsResponse {
    pub url: String,
    pub class: Class,
    pub headers: ReadsRequestHeaders,
}

#[derive(Serialize, Clone, Debug)]
pub enum Format {
    BAM,
    CRAM,
    VCF,
}

#[derive(Serialize, Clone, Debug)]
pub enum Class {
    #[serde(rename = "body")]
    Body,
    #[serde(rename = "header")]
    Header,
}

// Helper functions
pub fn htsget_request(reference: &Ref, start: u32, end: u32) -> (u32, u32) {
    let mut range_beg = u32::max_value(); // Must be Option instead of Integer... if it does not find anything, then None.
    let mut range_end: u32 = 0;
    let bin_id: u32;

    bin_id = reg2bin(start, end);

    for bin in reference.bins.iter() {
        if bin_id == bin.bin_id {
            for chunk in bin.chunks.iter() {
                let chunk_beg = coffset(chunk.chunk_beg);
                let chunk_end = coffset(chunk.chunk_end);

                range_beg = range_beg.min(chunk_beg);
                range_end = range_end.max(chunk_end);
            }
        }
    }

    // Only interested in compressed offset for the final htsget range (request to BAM)
    (range_beg, range_end)
}

pub fn htsget_response(auth: String, byte_range: (u32, u32),
                        url: String, format: Format, class: Class) -> HtsGetResponseContainer {

    let headers = ReadsRequestHeaders { auth, byte_range };
    let reads_ranges = ReadsResponse { url, class, headers };
    let htsget = HtsGetResponsePayload { format, urls: vec!(reads_ranges) };
    HtsGetResponseContainer { htsget }
}

pub fn bam_header(bucket: String, key: String) -> String {
    // XXX: factor this out well, take as reference:
    // https://github.com/holtgrewe/tell-tool/blob/master/lib-scan/src/lib.rs
    // i.e: bam_reader.header().target_names()[rid as usize]).unwrap();
    // let ref_ids = reference_ids(client, bucket, obj_bam);
    // let ref_id = ref_ids.iter().position(|name| name == &query).unwrap();
    let s3_url = Url::parse(&("s3://".to_string() + &bucket + &key)).unwrap();
    let bam_reader = IndexedReader::from_url(&s3_url).unwrap();

    let targets = bam_reader.header().target_names();
    dbg!(&targets);
    //let ref_id = targets.iter().position(|name| name == &query).unwrap();
    str::from_utf8(&targets[1]).unwrap().to_string()
}

/// Gets the header (first few bytes) from a BAM to translate BAI indexes into names
// pub fn reference_ids(client: S3Client, bucket: String, key: &Path) -> Vec<String> {
//     // let bam_bytes = s3_getobj_to_bytes(client, bucket, key);

//     let reader = Reader::from_path(key).expect("Cannot read BAM file");

//     reader.header().target_names().into_iter()
//         .map(|refname| String::from_utf8_lossy(refname).to_string())
//         .collect()
// }

// XXX: Needs to be substituted by s3_open from rust-htslib (for BAMs), should be kept for small index files like .BAI
pub async fn s3_getobj_to_bytes(s3: S3Client, bucket: String, obj: String) -> Vec<u8> {
    let mut content:Vec<u8> = Vec::new();

    let get_req = GetObjectRequest {
        bucket,
        key: obj,
        ..Default::default()
    };

    s3.get_object(get_req).await
                          .unwrap().body.unwrap()
                          .into_async_read()
                          .read_to_end(&mut content).await.unwrap();

    content
}