use std::future::Future;

struct AthenaStore {
    // TODO aws connection
    // TODO: AthenaClient
}

impl AthenaStore {
    fn new(/* ... */) -> AthenaStore {
        unimplemented!();
    }
}

impl ReadsIndex for AthenaStore {
    fn find_by_id(id: String) -> Future<Output=ReadsRef> {
        unimplemented!();
    }
}
