use leetcode_cli::Config;
use std::{
    env,
    sync::{Mutex, OnceLock},
};

const CONFIG: &str = r#"
[code]
editor = 'vim'
lang = 'rust'

[cookies]
csrf = ''
session = ''
site = 'leetcode.com'

[storage]
code = 'code'
root = '~/.leetcode'
scripts = 'scripts'
"#;

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

/// Parse `toml` the way `Config::locate` does, then apply the env override.
fn editor_of(toml: &str, editor: Option<&str>, visual: Option<&str>) -> String {
    let _guard = env_lock().lock().unwrap();

    let old_editor = env::var_os("EDITOR");
    let old_visual = env::var_os("VISUAL");

    unsafe {
        set("EDITOR", editor);
        set("VISUAL", visual);
    }

    let config: Config = toml::from_str(toml).unwrap();
    let editor = config.code.with_env_override().editor;

    unsafe {
        set("EDITOR", old_editor.as_deref().and_then(|v| v.to_str()));
        set("VISUAL", old_visual.as_deref().and_then(|v| v.to_str()));
    }

    editor
}

unsafe fn set(key: &str, value: Option<&str>) {
    unsafe {
        match value {
            Some(value) => env::set_var(key, value),
            None => env::remove_var(key),
        }
    }
}

#[test]
fn visual_overrides_the_configured_editor() {
    assert_eq!(editor_of(CONFIG, None, Some("nvim")), "nvim");
}

#[test]
fn editor_overrides_the_configured_editor() {
    assert_eq!(editor_of(CONFIG, Some("nvim"), None), "nvim");
}

#[test]
fn visual_takes_precedence_over_editor() {
    assert_eq!(editor_of(CONFIG, Some("nano"), Some("nvim")), "nvim");
}

#[test]
fn the_configured_editor_is_kept_without_env() {
    assert_eq!(editor_of(CONFIG, None, None), "vim");
}

#[test]
fn empty_env_vars_are_ignored() {
    assert_eq!(editor_of(CONFIG, Some(""), Some("")), "vim");
}

#[test]
fn vim_is_the_fallback_when_nothing_is_set() {
    let bare = CONFIG.replace("editor = 'vim'\n", "");
    assert_eq!(editor_of(&bare, None, None), "vim");
}
