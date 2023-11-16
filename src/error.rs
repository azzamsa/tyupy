use miette::Diagnostic;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("Invalid URL: {message}")]
    #[diagnostic(
        code(tyupy::invalid_url),
        url(docsrs),
        help(
            "The provided URL is invalid. Double-check the URL for any typos or formatting errors."
        )
    )]
    InvalidUrl { message: String },
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Internal(err.to_string())
    }
}
