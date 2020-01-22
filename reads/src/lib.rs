use serde::{ Serialize };
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
    pub byte_range: String,
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

pub fn htsget_response(auth: String, byte_range: String,
                       url: String, format: Format, class: Class)
    -> HtsGetResponseContainer {
    let headers = ReadsRequestHeaders {
        auth,
        byte_range,
    };

    let reads_ranges = ReadsResponse {
        url,
        class,
        headers,
    };

    let htsget = HtsGetResponsePayload {
        format,
        urls: vec!(reads_ranges),
    };

    let res = HtsGetResponseContainer {
        htsget,
    };

    return res;
}