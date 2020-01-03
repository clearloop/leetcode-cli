//! Errors in leetcode-cli
use std::fmt;
use std::error::Error as StdError;

/// Error enum
#[derive(Clone)]
pub enum Error {
    MatchError,
    DownloadError(String),
    NetworkError(String),
    ParseError(String),
    CacheError(String),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CacheError(s) => {
                error!("{}, please try again", s);
                write!(f, "{}, please try again", s)
            },
            Error::DownloadError(s) => {
                write!(f, "Download {} failed, please try again", s)
            },
            Error::NetworkError(s) => {
                error!("Network request {}, please try again", s);
                write!(f, "Network request {}, please try again", s)
            },
            Error::ParseError(s) => {
                error!("{}, please try again", s);
                write!(f, "{}, please try again", s)
            },
            Error::MatchError => {
                error!("Nothing matches");
                write!(f, "Nothing matches")
            }
        }
    }
}

// network
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NetworkError(err.description().to_string())
    }
}

// sql
impl std::convert::From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::CacheError(err.description().to_string())
    }
}

impl std::convert::From<serde_json::error::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseError(err.description().to_string())
    }
}

// io
impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::CacheError(err.description().to_string())
    }
}

// options
impl std::convert::From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Error::ParseError("json from response".to_string())
    }
}
