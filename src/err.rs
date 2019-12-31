/// leetcode-cli Errors
use std::fmt;

/// Error enum
pub enum Error {
    DownloadError(String),
    NetworkError(&'static str),
    ParseError(&'static str),
    CacheError(String),
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::CacheError(s) => {
                write!(f, "{}, please try again", s)
            },
            Error::DownloadError(s) => {
                write!(f, "Download {} failed, please try again", s)
            },
            Error::NetworkError(s) => {
                write!(f, "Network request leetcode::{} failed, please try again", s)
            },
            Error::ParseError(s) => {
                write!(f, "Parse {} failed, please try again", s)
            },
        }
    }
}
