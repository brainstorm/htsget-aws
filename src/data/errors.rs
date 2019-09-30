use snafu::Snafu;
use rusoto_core::error::RusotoError;
use rusoto_athena;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Snafu, Debug, PartialEq)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("No results found"))]
    NoResults,
    #[snafu(display("Malformed query error"))]
    MalformedQuery,
    #[snafu(display("Rusoto StartQueryExecutionError"))]
    RusotoStartQueryExecError {
        source: rusoto_core::RusotoError<rusoto_athena::StartQueryExecutionError>
    }
}