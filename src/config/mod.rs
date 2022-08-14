//! Soft-link with `config.toml`
//!
//! leetcode-cli will generate a `leetcode.toml` by default,
//! if you wanna change to it, you can:
//!
//! + Edit leetcode.toml at `~/.leetcode/leetcode.toml` directly
//! + Use `leetcode config` to update it
use crate::{config::sys::Sys, Error};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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
    /// Sync new config to config.toml
    pub fn sync(&self) -> Result<(), Error> {
        let home = dirs::home_dir().ok_or(Error::NoneError)?;
        let conf = home.join(".leetcode/leetcode.toml");
        fs::write(conf, toml::ser::to_string_pretty(&self)?)?;

        Ok(())
    }
}

/// Cookie settings
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
}

/// Code config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Code {
    pub editor: String,
    #[serde(rename(serialize = "editor-args", deserialize = "editor-args"))]
    pub editor_args: Option<Vec<String>>,
    pub edit_code_marker: bool,
    pub start_marker: String,
    pub end_marker: String,
    pub comment_problem_desc: bool,
    pub comment_leading: String,
    pub test: bool,
    pub lang: String,
    pub pick: String,
    pub submission: String,
}

/// Locate code files
///
/// + cache -> the path to cache
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Storage {
    cache: String,
    code: String,
    root: String,
    scripts: Option<String>,
}

impl Storage {
    /// convert root path
    pub fn root(&self) -> Result<String, Error> {
        let home = dirs::home_dir()
            .ok_or(Error::NoneError)?
            .to_string_lossy()
            .to_string();
        let path = self.root.replace('~', &home);
        Ok(path)
    }

    /// get cache path
    pub fn cache(&self) -> Result<String, crate::Error> {
        let home = dirs::home_dir()
            .ok_or(Error::NoneError)?
            .to_string_lossy()
            .to_string();
        let path = PathBuf::from(self.cache.replace('~', &home));
        if !path.is_dir() {
            info!("Generate cache dir at {:?}.", &path);
            fs::DirBuilder::new().recursive(true).create(&path)?;
        }

        Ok(path.join("Problems").to_string_lossy().to_string())
    }

    /// get code path
    pub fn code(&self) -> Result<String, crate::Error> {
        let root = &self.root()?;
        let p = PathBuf::from(root).join(&self.code);
        if !PathBuf::from(&p).exists() {
            fs::create_dir(&p)?
        }

        Ok(p.to_string_lossy().to_string())
    }

    /// get scripts path
    pub fn scripts(mut self) -> Result<String, crate::Error> {
        let root = &self.root()?;
        if self.scripts.is_none() {
            let tmp = toml::from_str::<Config>(DEFAULT_CONFIG)?;
            self.scripts = Some(tmp.storage.scripts.ok_or(Error::NoneError)?);
        }

        let p = PathBuf::from(root).join(&self.scripts.ok_or(Error::NoneError)?);
        if !PathBuf::from(&p).exists() {
            std::fs::create_dir(&p)?
        }

        Ok(p.to_string_lossy().to_string())
    }
}

/// Locate lc's config file
pub fn locate() -> Result<Config, crate::Error> {
    let conf = root()?.join("leetcode.toml");
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
