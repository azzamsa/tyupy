use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

/// all possible errors returned by the app.
#[derive(Debug, Error, Diagnostic)]
pub enum Error {
    #[error("{0}")]
    Internal(String),

    #[error("Configuration file is not found in `{path}`.")]
    #[diagnostic(
        code(tyupy::no_config),
        url(docsrs),
        help("Try creating a config of your choosen formatter.")
    )]
    ConfigNotFound { path: PathBuf },

    #[error("Invalid configuration: {message}")]
    #[diagnostic(
        code(tyupy::invalid_config),
        url(docsrs),
        help("See the configuration example of your choosen formatter.")
    )]
    InvalidConfig { message: String },
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Internal(err.to_string())
    }
}