//! Errors in leetcode-cli
use crate::cmds::{Command, DataCommand};
use colored::Colorize;
use std::fmt;

/// Error enum
#[derive(Clone)]
pub enum Error {
    MatchError,
    DownloadError(String),
    NetworkError(String),
    ParseError(String),
    CacheError(String),
    FeatureError(String),
    ScriptError(String),
    CookieError,
    DecryptError,
    SilentError,
    NoneError,
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let e = "error:".bold().red();
        match self {
            Error::CacheError(s) => write!(f, "{} {}, please try again", e, s),
            Error::CookieError => write!(
                f,
                "{} \
                 Your leetcode cookies seems expired, \
                 {} \
                 Either you can handwrite your `LEETCODE_SESSION` and `csrf` into `leetcode.toml`, \
                 more info please checkout this: \
                 https://github.com/clearloop/leetcode-cli/blob/master/README.md#cookies",
                e,
                "please make sure you have logined in leetcode.com with chrome. "
                    .yellow()
                    .bold(),
            ),
            Error::DownloadError(s) => write!(f, "{} Download {} failed, please try again", e, s),
            Error::NetworkError(s) => write!(f, "{} {}, please try again", e, s),
            Error::ParseError(s) => write!(f, "{} {}", e, s),
            Error::FeatureError(s) => write!(f, "{} {}", e, s),
            Error::MatchError => write!(f, "{} Nothing matches", e),
            Error::DecryptError => write!(f, "{} openssl decrypt failed", e),
            Error::ScriptError(s) => write!(f, "{} {}", e, s),
            Error::SilentError => write!(f, ""),
            Error::NoneError => write!(f,
                "json from response parse failed, please open a new issue at: {}.",
                "https://github.com/clearloop/leetcode-cli/".underline(),
            )
        }
    }
}

// network
impl std::convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::NetworkError(err.to_string())
    }
}

// nums
impl std::convert::From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseError(err.to_string())
    }
}

// sql
impl std::convert::From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => {
                println!("NotFound, you may update cache, and try it again\r\n");
                DataCommand::usage().print_help().unwrap_or(());
                Error::SilentError
            }
            _ => Error::CacheError(err.to_string()),
        }
    }
}

// serde
impl std::convert::From<serde_json::error::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}

// toml
impl std::convert::From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::ParseError(format!(
            "{}, {}{}{}{}{}{}{}{}",
            err,
            "Parse config file failed, ",
            "leetcode-cli has just generated a new leetcode.toml at ",
            "~/.leetcode/leetcode_tmp.toml,".green().bold().underline(),
            " the current one at ",
            "~/.leetcode/leetcode.toml".yellow().bold().underline(),
            "seems missing some keys, please compare to ",
            "the tmp one and add them up : )\n",
            ".",
        ))
    }
}

impl std::convert::From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::ParseError(err.to_string())
    }
}

// io
impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::CacheError(err.to_string())
    }
}

// openssl
impl std::convert::From<openssl::error::ErrorStack> for Error {
    fn from(_: openssl::error::ErrorStack) -> Self {
        Error::DecryptError
    }
}

// pyo3
#[cfg(feature = "pym")]
impl std::convert::From<pyo3::PyErr> for Error {
    fn from(_: pyo3::PyErr) -> Self {
        Error::ScriptError("Python script went Error".to_string())
    }
}
