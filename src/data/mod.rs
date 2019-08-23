use std::ops::Range;
use futures::Future;
use std::error::Error;

pub mod athena;

// XXX: ReadsRef name not cool, change
#[derive(Debug)]
pub struct ReadsRef {
    url: String,
    range: Range<usize>,
}

pub trait ReadsIndex {
    fn find_by_id(&self, id: String) -> Result<Vec<ReadsRef>, Box<dyn Error>>;
}

// trait VariantsIndex {
//     fn search_by_id(id: String) -> Range;
// }