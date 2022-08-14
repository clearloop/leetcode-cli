//! Soft-link with `config.toml`
//!
//! leetcode-cli will generate a `leetcode.toml` by default,
//! if you wanna change to it, you can:
//!
//! + Edit leetcode.toml at `~/.leetcode/leetcode.toml` directly
//! + Use `leetcode config` to update it
use crate::{
    config::{code::Code, cookies::Cookies, storage::Storage, sys::Sys},
    Error,
};
use serde::{Deserialize, Serialize};
use std::fs;

mod code;
mod cookies;
mod storage;
mod sys;

pub const DEFAULT_CONFIG: &str = r#"
[code]
editor = "vim"
lang = "rust"
edit_code_marker = false
comment_problem_desc = false
comment_leading = "///"
start_marker = "@lc code=start"
end_marker = "@lc code=start"
test = true
pick = "${fid}.${slug}"
submission = "${fid}.${slug}.${sid}.${ac}"

[cookies]
csrf = ""
session = ""

[storage]
root = "~/.leetcode"
scripts = "scripts"
code = "code"
cache = "Problems"
"#;

/// Sync with `~/.leetcode/config.toml`
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(skip)]
    pub sys: Sys,
    pub code: Code,
    pub cookies: Cookies,
    pub storage: Storage,
}

impl Config {
    /// Locate lc's config file
    pub fn locate() -> Result<Config, crate::Error> {
        let conf = Self::root()?.join("leetcode.toml");
        if !conf.is_file() {
            fs::write(&conf, &DEFAULT_CONFIG.trim())?;
        }

        let s = fs::read_to_string(&conf)?;
        Ok(toml::from_str::<Config>(&s)?)
    }

    /// Get root path of leetcode-cli
    pub fn root() -> Result<std::path::PathBuf, Error> {
        let dir = dirs::home_dir().ok_or(Error::NoneError)?.join(".leetcode");
        if !dir.is_dir() {
            info!("Generate root dir at {:?}.", &dir);
            fs::DirBuilder::new().recursive(true).create(&dir)?;
        }

        Ok(dir)
    }

    /// Sync new config to config.toml
    pub fn sync(&self) -> Result<(), Error> {
        let home = dirs::home_dir().ok_or(Error::NoneError)?;
        let conf = home.join(".leetcode/leetcode.toml");
        fs::write(conf, toml::ser::to_string_pretty(&self)?)?;

        Ok(())
    }
}
