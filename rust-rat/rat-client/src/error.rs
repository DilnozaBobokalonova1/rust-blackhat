use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

impl std::convert::From<uuid::Error> for Error {
    fn from(uuid_error: uuid::Error) -> Self {
        Error::Internal(uuid_error.to_string())
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(req_error: reqwest::Error) -> Self {
        Error::Internal(req_error.to_string())
    }
}