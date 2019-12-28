use toml;
use std::{fs, collections::HashMap};
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

# but you will want change these
[autologin]
enable = false
retry = 2

[code]
editor = "vim"
lang = "rust"

[file]
show = "${fid}.${slug}"
submission = "${fid}.${slug}.${sid}.${ac}"

[color]
enable = true
theme = "default"

[network]
concurrency = 10
delay = 1
"#;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub sys: Sys,
    pub autologin: AutoLogin,
    pub code: Code,
    pub file: File,
    pub color: Color,
    pub network: Network
}

impl Config {
    /// Sync new config to config.toml
    pub fn sync(&self) {
        let home = dirs::home_dir().unwrap();
        let conf = home.join(".leetcode/conf.toml");
        fs::write(conf, toml::ser::to_string_pretty(&self).unwrap()).unwrap();
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sys {
    pub categories: [String; 3],
    pub langs: [String; 16],
    pub urls: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
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

#[derive(Debug, Deserialize, Serialize)]
pub struct AutoLogin {
    pub enable: bool,
    pub retry: i32
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Code {
    pub editor: String,
    pub lang: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct File {
    pub show: String,
    pub submission: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Color {
    pub enable: bool,
    pub theme: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Network {
    pub concurrency: i32,
    pub delay: i32
}

pub fn locate() -> Config {
    let home = dirs::home_dir().unwrap();
    let root = home.join(".leetcode");
    let conf = root.join("conf.toml");
    
    if !root.is_dir() {
        info!("Generate leetcode root dir at {:?}.", &root);
        fs::DirBuilder::new()
            .recursive(true)
            .create(&root)
            .unwrap();
    }

    if !conf.is_file() {
        fs::write(&conf, &DEFAULT_CONFIG[1..]).unwrap();
    }

    let s = fs::read_to_string(&conf).unwrap();
    toml::from_str(&s).unwrap()
}

pub fn root() -> std::path::PathBuf {
    dirs::home_dir().unwrap().join(".leetcode")
}
