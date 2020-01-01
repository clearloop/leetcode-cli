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

// impl TryFrom<Response> for Value {
//     type Error = Error;
// 
//     fn try_from(r: Response) -> Result<Self, Self::Error> {
//         if 
//     }
// }
