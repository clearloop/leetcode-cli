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
    FeatureError(String),
    CookieError,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use colored::Colorize;
        let e = "error:".bold().red();
        match self {
            Error::CacheError(s) => {
                write!(f, "{} {}, please try again", e, s)
            },
            Error::CookieError => {
                write!(
                    f,
                    "{} {}{}",
                    e,
                    "cannot get leetcode cookies from chrome, ",
                    "please make sure you have logined in leetcode.com with chrome.".bold()
                )
            },
            Error::DownloadError(s) => {
                write!(f, "{} Download {} failed, please try again", e, s)
            },
            Error::NetworkError(s) => {
                write!(f, "{} {}, please try again", e, s)
            },
            Error::ParseError(s) => {
                write!(f, "{} {}, please try again", e, s)
            },
            Error::FeatureError(s) => {
                write!(f, "{} {}", e, s)
            }
            Error::MatchError => {
                write!(f, "{} Nothing matches", e)
            },
        }
    }
}

// network
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NetworkError(err.description().to_string())
    }
}

impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseError(err.description().to_string())
    }
}

// sql
impl std::convert::From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::CacheError(err.description().to_string())
    }
}

// serde
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
