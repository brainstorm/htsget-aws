use std:ops:Range;
use std::future::Future;

// XXX: ReadsRef not cool, change
##[derive(Debug)]
struct ReadsRef {
    url: String,
    range: Range,
}

trait ReadsIndex {
    fn find_by_id(id: String) -> Future<Output=Vec<ReadsRef>>;
}

// trait VariantsIndex {
//     fn search_by_id(id: String) -> Range;
// }