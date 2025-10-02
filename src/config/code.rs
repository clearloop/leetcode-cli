//! Code in config
use serde::{Deserialize, Serialize};

const PICK_DEFAULT: &str = "${fid}.${slug}";
fn default_pick() -> String {
    PICK_DEFAULT.into()
}

const SUBMISSION_DEFAULT: &str = "${fid}.${slug}.${sid}.${ac}";
fn default_submission() -> String {
    SUBMISSION_DEFAULT.into()
}

fn is_default_pick(t: &String) -> bool {
    t == PICK_DEFAULT
}

fn is_default_submission(t: &String) -> bool {
    t == SUBMISSION_DEFAULT
}

fn is_default_string(t: &String) -> bool {
    t.is_empty()
}
fn is_default_bool(t: &bool) -> bool {
    !t
}

fn default_enable_rust_crates() -> bool {
    true
}

/// Code config
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Code {
    #[serde(default)]
    pub editor: String,
    #[serde(rename(serialize = "editor-args"), alias = "editor-args", default)]
    pub editor_args: Option<Vec<String>>,
    #[serde(rename(serialize = "editor-envs"), alias = "editor-envs", default)]
    pub editor_envs: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "is_default_bool")]
    pub edit_code_marker: bool,
    #[serde(default, skip_serializing_if = "is_default_string")]
    pub start_marker: String,
    #[serde(default, skip_serializing_if = "is_default_string")]
    pub end_marker: String,
    #[serde(rename(serialize = "inject_before"), alias = "inject_before", default)]
    pub inject_before: Option<Vec<String>>,
    #[serde(rename(serialize = "inject_after"), alias = "inject_after", default)]
    pub inject_after: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "is_default_bool")]
    pub comment_problem_desc: bool,
    #[serde(default, skip_serializing_if = "is_default_string")]
    pub comment_leading: String,
    #[serde(default, skip_serializing_if = "is_default_bool")]
    pub test: bool,
    #[serde(default = "default_enable_rust_crates")]
pub enable_rust_crates: bool,
    pub lang: String,
    #[serde(default = "default_pick", skip_serializing_if = "is_default_pick")]
    pub pick: String,
    #[serde(
        default = "default_submission",
        skip_serializing_if = "is_default_submission"
    )]
    pub submission: String,
}

impl Default for Code {
    fn default() -> Self {
        Self {
            editor: "vim".into(),
            editor_args: None,
            editor_envs: None,
            edit_code_marker: false,
            start_marker: "".into(),
            end_marker: "".into(),
            inject_before: None,
            inject_after: None,
            comment_problem_desc: false,
            comment_leading: "".into(),
            test: true,
            enable_rust_crates: default_enable_rust_crates(),
            lang: "rust".into(),
            pick: "${fid}.${slug}".into(),
            submission: "${fid}.${slug}.${sid}.${ac}".into(),
        }
    }
}
