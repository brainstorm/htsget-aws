use crate::data::ReadsIndex;
use crate::data::IgvParametersRequest;
use crate::data::{ReadsRef};
use crate::data::errors::{Error, Result};

pub struct LocalStore {
    path: String,
    object: String,
}

impl LocalStore {
    pub fn new(path: String, object: String) -> LocalStore {
        return LocalStore {
            path,
            object,
        }
    }
}

impl ReadsIndex for LocalStore {
    fn find_by_id (&self, id: IgvParametersRequest) -> Result<Vec<ReadsRef>, Error> {
        let refs = Vec::new();

        Ok((refs))
    }
}