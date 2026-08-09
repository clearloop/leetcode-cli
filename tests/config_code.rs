use leetcode_cli::Config;
use std::{
    env,
    sync::{Mutex, OnceLock},
};

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn with_env(editor: Option<&str>, visual: Option<&str>, f: impl FnOnce()) {
    let _guard = env_lock().lock().unwrap();

    let old_editor = env::var_os("EDITOR");
    let old_visual = env::var_os("VISUAL");

    unsafe {
        match editor {
            Some(value) => env::set_var("EDITOR", value),
            None => env::remove_var("EDITOR"),
        }
        match visual {
            Some(value) => env::set_var("VISUAL", value),
            None => env::remove_var("VISUAL"),
        }
    }

    f();

    unsafe {
        match old_editor {
            Some(value) => env::set_var("EDITOR", value),
            None => env::remove_var("EDITOR"),
        }
        match old_visual {
            Some(value) => env::set_var("VISUAL", value),
            None => env::remove_var("VISUAL"),
        }
    }
}

#[test]
fn config_uses_editor_from_visual_env_var_when_set() {
    with_env(None, Some("nvim"), || {
        assert_eq!(Config::default().code.editor, "nvim");
    });
}

#[test]
fn config_uses_editor_from_editor_env_var_when_set() {
    with_env(Some("nvim"), None, || {
        assert_eq!(Config::default().code.editor, "nvim");
    });
}

#[test]
fn config_falls_back_to_vim_on_empty_env_vars() {
    with_env(None, None, || {
        assert_eq!(Config::default().code.editor, "vim");
    });
}

#[test]
fn config_prefers_visual_over_editor() {
    with_env(Some("nvim"), Some("vim"), || {
        assert_eq!(Config::default().code.editor, "vim");
    });
}
