//! Errors in leetcode-cli
use anyhow::anyhow;
use colored::Colorize;

#[cfg(debug_assertions)]
const CONFIG: &str = "~/.leetcode/leetcode.tmp.toml";
#[cfg(not(debug_assertions))]
const CONFIG: &str = "~/.leetcode/leetcode_tmp.toml";

/// Leetcode result.
pub type Result<T> = std::result::Result<T, Error>;

/// Leetcode cli errors
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Nothing matched")]
    MatchError,
    #[error("Download {0} failed, please try again")]
    DownloadError(String),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error(transparent)]
    HeaderName(#[from] reqwest::header::InvalidHeaderName),
    #[error(transparent)]
    HeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error(
        "Your leetcode cookies seems expired, \
         {} \
         Either you can handwrite your `LEETCODE_SESSION` and `csrf` into `leetcode.toml`, \
         more info please checkout this: \
         https://github.com/clearloop/leetcode-cli/blob/master/README.md#cookies",
        "please make sure you have logined in leetcode.com with chrome. ".yellow().bold()
    )]
    CookieError,
    #[error(
        "Your leetcode account lacks a premium subscription, which the given problem requires.\n \
         If this looks like a mistake, please open a new issue at: {}",
        "https://github.com/clearloop/leetcode-cli/".underline()
    )]
    PremiumError,
    #[error(transparent)]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error(
        "json from response parse failed, please open a new issue at: {}.",
        "https://github.com/clearloop/leetcode-cli/".underline()
    )]
    NoneError,
    #[error(
        "Parse config file failed, \
         leetcode-cli has just generated a new leetcode.toml at {}, \
         the current one at {} seems missing some keys, Please compare \
         the new file and add the missing keys.\n",
        CONFIG,
        "~/.leetcode/leetcode.toml".yellow().bold().underline(),
    )]
    Config(#[from] toml::de::Error),
    #[error("Maybe you not login on the Chrome, you can login and retry")]
    ChromeNotLogin,
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Toml(#[from] toml::ser::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    #[error(transparent)]
    OpenSSL(#[from] openssl::error::ErrorStack),
    #[cfg(feature = "pym")]
    #[error(transparent)]
    Pyo3(#[from] pyo3::PyErr),
}

impl std::convert::From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Error::Anyhow(anyhow!(
                "NotFound, you may update cache with `leetcode data -u`, and try it again\r\n"
            )),
            _ => Error::Anyhow(anyhow!("{err}")),
        }
    }
}
