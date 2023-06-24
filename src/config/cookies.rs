//! Cookies in config
use serde::{Deserialize, Serialize};

/// Cookies settings
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
}

impl std::string::ToString for Cookies {
    fn to_string(&self) -> String {
        format!("LEETCODE_SESSION={};csrftoken={};", self.session, self.csrf)
    }
}
