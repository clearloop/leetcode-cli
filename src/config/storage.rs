//! Storage in config.
use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Name of the local sqlite cache file under `root`.
const CACHE: &str = "Problems";

/// Locate code files
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Storage {
    code: String,
    root: String,
    scripts: Option<String>,
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            code: "code".into(),
            scripts: Some("scripts".into()),
            root: "~/.leetcode".into(),
        }
    }
}

impl Storage {
    /// convert root path
    pub fn root(&self) -> Result<String> {
        let home = dirs::home_dir()
            .ok_or(Error::NoneError)?
            .to_string_lossy()
            .to_string();
        let path = self.root.replace('~', &home);
        Ok(path)
    }

    /// get cache path
    pub fn cache(&self) -> Result<String> {
        let root = PathBuf::from(self.root()?);
        if !root.exists() {
            info!("Generate cache dir at {:?}.", &root);
            fs::DirBuilder::new().recursive(true).create(&root)?;
        }

        Ok(root.join(CACHE).to_string_lossy().to_string())
    }

    /// get code path
    pub fn code(&self) -> Result<String> {
        let root = &self.root()?;
        let p = PathBuf::from(root).join(&self.code);
        if !PathBuf::from(&p).exists() {
            fs::create_dir(&p)?
        }

        Ok(p.to_string_lossy().to_string())
    }

    /// get scripts path
    pub fn scripts(mut self) -> Result<String> {
        let root = &self.root()?;
        if self.scripts.is_none() {
            self.scripts = Some("scripts".into());
        }

        let p = PathBuf::from(root).join(self.scripts.ok_or(Error::NoneError)?);
        if !PathBuf::from(&p).exists() {
            std::fs::create_dir(&p)?
        }

        Ok(p.to_string_lossy().to_string())
    }
}
