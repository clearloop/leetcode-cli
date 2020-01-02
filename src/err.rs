//! Errors in leetcode-cli
//!
//! TODO:
//! 
//! + `Tryfrom`
use std::fmt;
// use core::convert::TryFrom;
// use reqwest::Response;
// use serde_json::Value;

/// Error enum
#[derive(Clone)]
pub enum Error {
    DownloadError(String),
    NetworkError(String),
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
                error!("Network request to {} failed, please try again", s);
                write!(f, "Network request to {} failed, please try again", s)
            },
            Error::ParseError(s) => {
                error!("Parse {} failed, please try again", s);
                write!(f, "Parse {} failed, please try again", s)
            },
        }
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        let url = err.url();
        if url.is_none() {
            return Error::NetworkError("https://leetcode.com".to_string());
        }
        
        Error::NetworkError(url.unwrap().to_string())
    }
}

impl std::convert::From<std::option::NoneError> for Error {
    fn from(_: std::option::NoneError) -> Self {
        Error::ParseError("json from response")
    }
}
