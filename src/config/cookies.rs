//! Cookies in config
use std::str::FromStr;

use serde::{Deserialize, Serialize};

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

impl ToString for LeetcodeSite {
    fn to_string(&self) -> String {
        match self {
            LeetcodeSite::LeetcodeCom => "leetcode.com".to_string(),
            LeetcodeSite::LeetcodeCn => "leetcode.cn".to_string(),
        }
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

impl std::string::ToString for Cookies {
    fn to_string(&self) -> String {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf)
    }
}
