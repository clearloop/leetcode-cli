//! Cookies in config
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    str::FromStr,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum LeetcodeSite {
    #[serde(rename = "leetcode.com")]
    LeetcodeCom,
    #[serde(rename = "leetcode.cn")]
    LeetcodeCn,
}

impl FromStr for LeetcodeSite {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "leetcode.com" => Ok(LeetcodeSite::LeetcodeCom),
            "leetcode.cn" => Ok(LeetcodeSite::LeetcodeCn),
            _ => Err("Invalid site key".to_string()),
        }
    }
}

impl Display for LeetcodeSite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LeetcodeSite::LeetcodeCom => "leetcode.com",
            LeetcodeSite::LeetcodeCn => "leetcode.cn",
        };

        write!(f, "{s}")
    }
}

/// Cookies settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
    pub site: LeetcodeSite,
}

impl Default for Cookies {
    fn default() -> Self {
        Self {
            csrf: "".to_string(),
            session: "".to_string(),
            site: LeetcodeSite::LeetcodeCom,
        }
    }
}

impl Display for Cookies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "LEETCODE_SESSION={};csrftoken={};",
            self.session, self.csrf
        )
    }
}

/// Override cookies from environment variables
pub const LEETCODE_CSRF_ENV: &str = "LEETCODE_CSRF";
pub const LEETCODE_SESSION_ENV: &str = "LEETCODE_SESSION";
pub const LEETCODE_SITE_ENV: &str = "LEETCODE_SITE";

impl Cookies {
    /// Load cookies from environment variables, overriding any existing values
    /// if the environment variables are set.
    pub fn with_env_override(mut self) -> Self {
        if let Ok(csrf) = std::env::var(LEETCODE_CSRF_ENV) {
            self.csrf = csrf;
        }
        if let Ok(session) = std::env::var(LEETCODE_SESSION_ENV) {
            self.session = session;
        }
        if let Ok(site) = std::env::var(LEETCODE_SITE_ENV) {
            if let Ok(leetcode_site) = LeetcodeSite::from_str(&site) {
                self.site = leetcode_site;
            }
        }
        self
    }
}
