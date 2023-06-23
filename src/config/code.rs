//! Code in config
use serde::{Deserialize, Serialize};

/// Code config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Code {
    #[serde(default)]
    pub editor: String,
    #[serde(rename(serialize = "editor-args"), alias = "editor-args", default)]
    pub editor_args: Option<Vec<String>>,
    #[serde(default, skip_serializing)]
    pub edit_code_marker: bool,
    #[serde(default, skip_serializing)]
    pub start_marker: String,
    #[serde(default, skip_serializing)]
    pub end_marker: String,
    #[serde(default, skip_serializing)]
    pub comment_problem_desc: bool,
    #[serde(default, skip_serializing)]
    pub comment_leading: String,
    #[serde(default, skip_serializing)]
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
            comment_leading: "".into(),
            test: true,
            lang: "rust".into(),
            pick: "${fid}.${slug}".into(),
            submission: "${fid}.${slug}.${sid}.${ac}".into(),
        }
    }
}
