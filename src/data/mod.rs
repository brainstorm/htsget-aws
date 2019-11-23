//use std::ops::Range;

pub mod local;
pub mod athena;
pub mod errors;

use crate::data::errors::{Error, Result};


// XXX: Move to client module/class
// Minimum required parameters as seen on igv.js testsuite:
// https://github.com/igvteam/igv.js/blob/0c8f6982bc4cc8756bfa5cf3c962ff81faf08284/test/testHtsgetReader.js#L43
#[derive(Debug)]
pub struct ReadsRequest {
    pub url: String,
    pub id: String,
    pub chromosome: String,
    pub start: usize,
    pub end: usize
}

// htsget response as described in: https://samtools.github.io/hts-specs/htsget.html
struct HtsGetResponse {
    format: Format,
    urls: ReadsRef
}

#[allow(dead_code)]
#[derive(Debug)]
enum Format {
    BAM,
    CRAM,
    VCF
}

// XXX: Make better use of this enum
#[allow(dead_code)]
#[derive(Debug)]
enum Class {
    Body,
    Header
}

#[derive(Debug)]
pub struct ReadsRef {
    url: String,
    class: String,
    headers: ReadsRefHeaders,
}

#[derive(Debug)]
struct ReadsRefHeaders {
    authorization: String,
    range: String,
}

impl ReadsRef {
    fn new(url: String, class: String, headers: ReadsRefHeaders) -> ReadsRef {
        ReadsRef {
            url,
            class,
            headers
        }
    }
}

//XXX: Change name
pub trait ReadsIndex {
    fn find_by_id(&self, id: ReadsRequest) -> Result<Vec<ReadsRef>, Error>;
}

// trait VariantsIndex {
//     fn search_by_id(id: String) -> Range;
// }