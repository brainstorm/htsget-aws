#[derive(Debug)]
pub struct ReadsRequest {
    pub url: String,
    pub id: String,
    pub chromosome: String,
    pub start: usize,
    pub end: usize
}

// htsget response as described in: https://samtools.github.io/hts-specs/htsget.html
//struct HtsGetResponse {
//    format: Format,
//    urls: ReadsRef
//}

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
pub struct ReadsResponse {
    url: String,
    class: String,
    headers: ReadsRequestHeaders,
}

#[derive(Debug)]
struct ReadsRequestHeaders {
    authorization: String,
    range: String,
}