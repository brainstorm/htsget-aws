use url::Url;
use std::{path::Path, fs::File, io};
use serde::{ Serialize };

use noodles_bam as bam;
use noodles_sam as sam;

use rusoto_core;
use rusoto_s3::{ GetObjectRequest, S3, S3Client };

use bio_index_formats::parser_bai::{coffset, Ref};
use bio_index_formats::csi::{ reg2bin };

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

// pub fn bam_header_from_local(fname: Path) -> io::Result<()> {

//     let mut reader = File::open(fname).map(bam::Reader::new)?;
//     let header = reader.read_header()?;

//     if header.is_empty() {
//         let reference_sequences = reader.read_reference_sequences()?;
//         let mut builder = sam::Header::builder();

//         for reference_sequence in reference_sequences {
//             builder = builder.add_reference_sequence(reference_sequence);
//         }

//         print!("{}", builder.build());
//     } else {
//         print!("{}", header);
//     }

//     Ok(())
// }

pub async fn bam_header_from_s3(bucket: String, key: String) -> () {
    let obj = S3Client::new(rusoto_core::Region::default())
        .get_object(GetObjectRequest { bucket, key, ..GetObjectRequest::default() }
    );

    // let mut reader = File::open(src).map(bam::Reader::new)?;
    // let reference_sequences = reader.read_reference_sequences()?;
    // let header = reader.read_header()?;

    //return header;
}