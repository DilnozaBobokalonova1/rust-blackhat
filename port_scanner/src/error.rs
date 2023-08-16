use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Usage: tricoder <kerkour.com")]
    CliUsage,
    #[error("Reqwest: {0}")]
    Reqwest(String),
}

//convert reqwest::Error into a custom error defined above
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}
