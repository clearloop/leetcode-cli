//! Soft-link with `config.toml`
//!
//! leetcode-cli will generate a `leetcode.toml` by default,
//! if you wanna change to it, you can:
//!
//! + Edit leetcode.toml at `~/.leetcode/leetcode.toml` directly
//! + Use `leetcode config` to update it
use crate::{
    config::{code::Code, cookies::Cookies, storage::Storage, sys::Sys},
    Error, Result,
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

    /// Locate lc's config file
    pub fn locate() -> Result<Config> {
        let conf = Self::root()?.join("leetcode.toml");

        if !conf.is_file() {
            Self::write_default(&conf)?;
        }

        let s = fs::read_to_string(&conf)?;
        match toml::from_str::<Config>(&s) {
            Ok(mut config) => {
                // Override config.cookies with environment variables
                config.cookies = config.cookies.with_env_override();

                match config.cookies.site {
                    cookies::LeetcodeSite::LeetcodeCom => Ok(config),
                    cookies::LeetcodeSite::LeetcodeCn => {
                        let mut config = config;
                        config.sys.urls = sys::Urls::new_with_leetcode_cn();
                        Ok(config)
                    }
                }
            }
            Err(e) => {
                let tmp = Self::root()?.join("leetcode.tmp.toml");
                Self::write_default(tmp)?;
                Err(e.into())
            }
        }
    }

    /// Get root path of leetcode-cli
    pub fn root() -> Result<std::path::PathBuf> {
        let dir = dirs::home_dir().ok_or(Error::NoneError)?.join(".leetcode");
        if !dir.is_dir() {
            info!("Generate root dir at {:?}.", &dir);
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
