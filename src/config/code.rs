//! Code in config
use serde::{Deserialize, Serialize};

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
