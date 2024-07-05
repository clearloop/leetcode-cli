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
