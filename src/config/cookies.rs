//! Cookies in config
use serde::{Deserialize, Serialize};

/// Cookies settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
}
