//! Soft-link with `config.toml`
//!
//! leetcode-cli will generate a `leetcode.toml` by default,
//! if you wanna change to it, you can:
//!
//! + Inspect code and problems at `cache_dir` directory:
//! |Platform | Value                               | Example                      |
//! | ------- | ----------------------------------- | ---------------------------- |
//! | Unix    | `$XDG_CACHE_HOME` or `$HOME`/.cache | /home/alice/.cache           |
//! | Windows | `{FOLDERID_LocalAppData}`           | C:\Users\Alice\AppData\Local |
//!
//! + Edit leetcode.toml at `config_dir` directly:
//! |Platform | Value                                 | Example                                  |
//! | ------- | ------------------------------------- | ---------------------------------------- |
//! | Unix    | `$XDG_CONFIG_HOME` or `$HOME`/.config | /home/alice/.config                      |
//! | Windows | `{FOLDERID_RoamingAppData}`           | C:\Users\Alice\AppData\Roaming           |
//!
//! + Use `leetcode config` to update it
use crate::Error;
use etcetera::base_strategy::{choose_base_strategy, BaseStrategy};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Sync with `~/.leetcode/config.toml`
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub sys: Sys,
    pub code: Code,
    pub cookies: Cookies,
}

impl Config {
    /// Sync new config to config.toml
    pub fn sync(&self) -> Result<(), Error> {
        let conf = Self::config_dir().join("leetcode.toml");
        fs::write(conf, toml::ser::to_string_pretty(&self)?)?;

        Ok(())
    }

    pub fn home_dir() -> PathBuf {
        etcetera::home_dir().expect("Unable to find the home directory!")
    }

    /// config_dir for `leetcode.toml`
    pub fn config_dir() -> PathBuf {
        let strategy = choose_base_strategy().expect("Unable to find the config directory!");
        let mut path = strategy.config_dir();
        path.push("leetcode");
        path
    }

    /// create `config_dir` if not exists, and serialize default `Config` to it.
    pub fn config_content() -> Result<Config, Error> {
        let config_filepath = Self::config_dir();
        if !config_filepath.exists() {
            fs::create_dir_all(&config_filepath)?;
        }
        let config_filepath = config_filepath.join("leetcode.toml");
        match fs::read_to_string(&config_filepath) {
            Ok(s) => Ok(toml::from_str(&s)?),
            Err(_) => {
                // serialize default config
                let def_config = Config::default();
                fs::write(config_filepath, toml::ser::to_string_pretty(&def_config)?)?;
                Ok(def_config)
            }
        }
    }

    pub fn script_dir_or_create() -> Result<String, Error> {
        let script_dir = Self::config_dir().join("scripts");
        if !script_dir.exists() {
            fs::create_dir_all(&script_dir)?;
        }
        Ok(script_dir.to_string_lossy().to_string())
    }

    /// cache_dir for `code` and `problems`
    pub fn cache_dir() -> PathBuf {
        let strategy = choose_base_strategy().expect("Unable to find the cache directory!");
        let mut path = strategy.cache_dir();
        path.push("leetcode");
        path
    }

    /// problems filepath
    pub fn problems_filepath() -> Result<String, Error> {
        let cache_dir = Self::cache_dir();
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)?;
        }
        Ok(cache_dir.join("problems").to_string_lossy().to_string())
    }

    /// cache `code` dir
    pub fn code_dir_or_create() -> Result<String, Error> {
        let code_dir = Self::cache_dir().join("code");
        if !code_dir.exists() {
            fs::create_dir_all(&code_dir)?;
        }
        Ok(code_dir.to_string_lossy().to_string())
    }
}

/// Cookie settings
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct Cookies {
    pub csrf: String,
    pub session: String,
}

/// System settings, for leetcode api mainly
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Sys {
    pub categories: Vec<String>,
    pub langs: Vec<String>,
    pub urls: Urls,
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            categories: vec!["algorithms", "concurrency", "database", "shell"]
                .into_iter()
                .map(|s| s.into())
                .collect(),
            langs: vec![
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
                "swift",
            ]
            .into_iter()
            .map(|s| s.into())
            .collect(),
            urls: Urls::default(),
        }
    }
}

/// Leetcode API
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Urls {
    pub base: String,
    pub graphql: String,
    pub login: String,
    pub problems: String,
    pub problem: String,
    pub tag: String,
    pub test: String,
    pub session: String,
    pub submit: String,
    pub submissions: String,
    pub submission: String,
    pub verify: String,
    pub favorites: String,
    pub favorite_delete: String,
}

impl Default for Urls {
    fn default() -> Self {
        Self {
            base: "https://leetcode.com".into(),
            graphql: "https://leetcode.com/graphql".into(),
            login: "https://leetcode.com/accounts/login/".into(),
            problems: "https://leetcode.com/api/problems/$category/".into(),
            problem: "https://leetcode.com/problems/$slug/description/".into(),
            tag: "https://leetcode.com/tag/$slug/".into(),
            test: "https://leetcode.com/problems/$slug/interpret_solution/".into(),
            session: "https://leetcode.com/session/".into(),
            submit: "https://leetcode.com/problems/$slug/submit/".into(),
            submissions: "https://leetcode.com/submissions/detail/$id/".into(),
            submission: "https://leetcode.com/submissions/detail/$id/".into(),
            verify: "https://leetcode.com/submissions/detail/$id/check/".into(),
            favorites: "https://leetcode.com/list/api/questions".into(),
            favorite_delete: "https://leetcode.com/list/api/questions/$hash/$id".into(),
        }
    }
}

/// default editor and langs
///
/// + support editor: [emacs, vim]
/// + support langs: all in config
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

impl Default for Code {
    fn default() -> Self {
        Self {
            editor: "vim".into(),
            editor_args: None,
            edit_code_marker: false,
            start_marker: "".into(),
            end_marker: "".into(),
            comment_problem_desc: false,
            comment_leading: "///".into(),
            test: true,
            lang: "rust".into(),
            pick: "${fid}.${slug}".into(),
            submission: "${fid}.${slug}.${sid}.${ac}".into(),
        }
    }
}
