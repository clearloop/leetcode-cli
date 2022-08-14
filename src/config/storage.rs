//! Storage in config.
use crate::Error;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

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

impl Default for Storage {
    fn default() -> Self {
        Self {
            cache: "Problems".into(),
            code: "code".into(),
            scripts: Some("scripts".into()),
            root: "~/.leetcode".into(),
        }
    }
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
            self.scripts = Some("scripts".into());
        }

        let p = PathBuf::from(root).join(&self.scripts.ok_or(Error::NoneError)?);
        if !PathBuf::from(&p).exists() {
            std::fs::create_dir(&p)?
        }

        Ok(p.to_string_lossy().to_string())
    }
}
