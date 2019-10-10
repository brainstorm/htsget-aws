use snafu::Snafu;
use rusoto_core;

#[derive(Snafu, Debug, PartialEq)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Error retrieving reads"))]
    ReadsQueryError { cause: String }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
