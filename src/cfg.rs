//! Soft-link with `config.tom`
//!
//! leetcode-cli will generate a `config.toml` by default,
//! if you wanna change to it, you can:
//! 
//! + Edit config.toml at `~/.leetcode/config.toml` directly
//! + Use `leetcode config` to update it
use toml;
use std::{fs, collections::HashMap, path::PathBuf};
use serde::{Deserialize, Serialize};

const DEFAULT_CONFIG: &'static str = r#"
# usually you don't wanna change those
[sys]
categories = [
  "algorithms",
  "database",
  "shell"
]

langs = [
  "bash",
  "c",
  "cpp",
  "csharp",
  "golang",
  "java",
  "javascript",
  "kotlin",
  "mysql",
  "php",
  "python",
  "python3",
  "ruby",
  "rust",
  "scala",
  "swift"
]

[sys.urls]
base = "https://leetcode.com"
graphql = "https://leetcode.com/graphql"
login = "https://leetcode.com/accounts/login/"
problems = "https://leetcode.com/api/problems/$category/"
problem = "https://leetcode.com/problems/$slug/description/"
test = "https://leetcode.com/problems/$slug/interpret_solution/"
session = "https://leetcode.com/session/"
submit = "https://leetcode.com/problems/$slug/submit/"
submissions = "https://leetcode.com/api/submissions/$slug"
submission = "https://leetcode.com/submissions/detail/$id/"
verify = "https://leetcode.com/submissions/detail/$id/check/"
favorites = "https://leetcode.com/list/api/questions"
favorite_delete = "https://leetcode.com/list/api/questions/$hash/$id"

[code]
editor = "vim"
lang = "rust"
pick = "${fid}.${slug}"
submission = "${fid}.${slug}.${sid}.${ac}"

[storage]
cache = "Problems"
code = "code"
root = "~/.leetcode"
"#;

/// sync with `~/.leetcode/config.toml`
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub sys: Sys,
    pub code: Code,
    pub storage: Storage
}

impl Config {
    /// Sync new config to config.toml
    pub fn sync(&self) {
        let home = dirs::home_dir().unwrap();
        let conf = home.join(".leetcode/conf.toml");
        fs::write(conf, toml::ser::to_string_pretty(&self).unwrap()).unwrap();
    }
}

/// System settings, for leetcode api mainly
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sys {
    pub categories: [String; 3],
    pub langs: [String; 16],
    pub urls: HashMap<String, String>,
}

/// Leetcode API
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Urls {
    pub base: String,
    pub graphql: String,
    pub login: String,
    pub problems: String,
    pub problem: String,
    pub test: String,
    pub session: String,
    pub submit: String,
    pub submissions: String,
    pub submission: String,
    pub verify: String,
    pub favorites: String,
    pub favorite_delete: String,
}

/// default editor and langs
///
/// + support editor: [emacs, vim]
/// + support langs: all in config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Code {
    pub editor: String,
    pub lang: String,
    pub pick: String,
    pub submission: String
}

/// storage
///
/// + cache -> the path to cache
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Storage {
    cache: String,
    code: String,
    root: String
}

impl Storage {
    /// convert root path
    pub fn root(&self) -> String {
        let home = dirs::home_dir().unwrap().to_string_lossy().to_string();
        let path = self.root.replace("~", &home);
        path
    }

    /// get cache path
    pub fn cache(&self) -> String {
        let root = &self.root();
        PathBuf::from(root)
            .join(&self.cache)
            .to_string_lossy()
            .to_string()
    }

    /// get cache path
    pub fn code(&self) -> String {
        let root = &self.root();
        PathBuf::from(root)
            .join(&self.code)
            .to_string_lossy()
            .to_string()
    }
}


/// Locate lc's config file
pub fn locate() -> Config {
    let conf = root().join("leetcode.toml");
    if !conf.is_file() {
        fs::write(&conf, &DEFAULT_CONFIG[1..]).unwrap();
    }

    let s = fs::read_to_string(&conf).unwrap();
    toml::from_str(&s).unwrap()
}

/// Get root path of leetcode-cli
pub fn root() -> std::path::PathBuf {
    let dir = dirs::home_dir().unwrap().join(".leetcode");
    if !dir.is_dir() {
        info!("Generate root dir at {:?}.", &dir);
        fs::DirBuilder::new()
            .recursive(true)
            .create(&dir)
            .unwrap();
    }

    dir
}
