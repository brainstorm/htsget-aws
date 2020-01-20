use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    #[snafu(display("Error retrieving reads: {}", cause))]
    ReadsQueryError { cause: String }
}

//pub type Result<T, E = Error> = std::result::Result<T, E>;
