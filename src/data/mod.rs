use std::ops::Range;

pub mod athena;
pub mod errors;

use crate::data::errors::{Error, Result};

// htsget response as described in: https://samtools.github.io/hts-specs/htsget.html

struct HtsGetResponse {
    Format: Format,
    Urls: ReadsRef
}

#[derive(Debug)]
enum Format {
    BAM,
    CRAM,
    VCF
}

// XXX
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
    bytes: Range<usize>,
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
    fn find_by_id(&self, id: String) -> Result<Vec<ReadsRef>, Error>;
}

// trait VariantsIndex {
//     fn search_by_id(id: String) -> Range;
// }