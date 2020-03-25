use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use serde::{ Serialize };

use bio_index_formats::parser_bai::{coffset, Ref};
use bio_index_formats::csi::{ reg2bin };

use rust_htslib::bam::{ Reader, Read as BamRead };
use rusoto_s3::{GetObjectRequest, S3Client, S3};

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

/// Gets the header (first few bytes) from a BAM to translate BAI indexes into names
pub fn reference_ids(client: S3Client, bucket: String, obj: &Path) -> Vec<String> {
    let bam_bytes = s3_getobj(client, bucket, obj);
//    let lambda_bai = Path::new("/tmp/");

//    // From bytes to local lambda file for now since async-await/streaming is painful
//    let mut file = File::create(obj.file_name().unwrap()).expect("create failed");
//    file.write_all(&bam_bytes).expect("failed to write BAM to lambda storage");

    let reader = Reader::from_path(lambda_bai).expect("Cannot read BAM file");

    reader.header().target_names().into_iter()
        .map(|refname| String::from_utf8_lossy(refname).to_string())
        .collect()
}

pub fn s3_getobj(client: S3Client, bucket: String, obj: &Path) -> Vec<u8> {
    let mut buf:Vec<u8> = Vec::new();

    let get_req = GetObjectRequest {
        bucket,
        key: obj.to_string_lossy().to_string(),
        ..Default::default()
    };

    let result = client
        .get_object(get_req).sync().expect("Couldn't GET object");

    // Convert ByteStream to slice of bytes (&[u8]'s) to buf
    result.body.map(|astream| astream.into_blocking_read().read_to_end(&mut buf));

    buf
}