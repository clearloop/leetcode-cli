//! Soft-link with `config.toml`
//!
//! leetcode-cli will generate a `leetcode.toml` by default,
//! if you wanna change to it, you can:
//!
//! + Edit leetcode.toml at `~/.leetcode/leetcode.toml` directly
//! + Use `leetcode config` to update it
use crate::{
    Error, Result,
    config::{code::Code, cookies::Cookies, storage::Storage, sys::Sys},
};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

mod code;
mod cookies;
mod storage;
mod sys;

pub use cookies::LeetcodeSite;

/// Sync with `~/.leetcode/leetcode.toml`
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default, skip_serializing)]
    pub sys: Sys,
    pub code: Code,
    pub cookies: Cookies,
    pub storage: Storage,
}

impl Config {
    fn write_default(p: impl AsRef<Path>) -> Result<()> {
        fs::write(p.as_ref(), toml::ser::to_string_pretty(&Self::default())?)?;

        Ok(())
    }

    /// Parse a config, applying the environment overrides on top of it
    pub fn parse(s: &str) -> Result<Config> {
        let mut config: Config = toml::from_str(s)?;

        config.code = config.code.with_env_override();
        config.cookies = config.cookies.with_env_override();

        if let cookies::LeetcodeSite::LeetcodeCn = config.cookies.site {
            config.sys.urls = sys::Urls::new_with_leetcode_cn();
        }

        Ok(config)
    }

    /// Locate lc's config file
    pub fn locate() -> Result<Config> {
        let conf = Self::root()?.join("leetcode.toml");

        if !conf.is_file() {
            Self::write_default(&conf)?;
        }

        match Self::parse(&fs::read_to_string(&conf)?) {
            Ok(config) => Ok(config),
            Err(e) => {
                Self::write_default(Self::root()?.join("leetcode.tmp.toml"))?;
                Err(e)
            }
        }
    }

    /// Get root path of leetcode-cli
    pub fn root() -> Result<std::path::PathBuf> {
        let dir = dirs::home_dir().ok_or(Error::NoneError)?.join(".leetcode");
        if !dir.is_dir() {
            info!("Generate root dir at {:?}.", dir);
            fs::DirBuilder::new().recursive(true).create(&dir)?;
        }

        Ok(dir)
    }

    /// Sync new config to config.toml
    pub fn sync(&self) -> Result<()> {
        let home = dirs::home_dir().ok_or(Error::NoneError)?;
        let conf = home.join(".leetcode/leetcode.toml");
        fs::write(conf, toml::ser::to_string_pretty(&self)?)?;

        Ok(())
    }
}
