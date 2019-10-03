use std::ops::Range;

pub mod athena;
pub mod errors;

use crate::data::errors::{Error, Result};

// XXX: ReadsRef name not cool, change
#[derive(Debug)]
pub struct ReadsRef {
    url: String,
    range: Range<usize>,
}

//XXX: Change name
pub trait ReadsIndex {
    fn find_by_id(&self, id: String) -> Result<Vec<ReadsRef>, Error>;
}

// trait VariantsIndex {
//     fn search_by_id(id: String) -> Range;
// }