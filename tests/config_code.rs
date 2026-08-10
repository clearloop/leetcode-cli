use leetcode_cli::Config;

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

/// One test: `set_var` is process-wide, so splitting these would race.
#[test]
fn env_overrides_the_configured_editor() {
    let editor = || Config::parse(CONFIG).unwrap().code.editor;

    unsafe {
        std::env::remove_var("EDITOR");
        std::env::remove_var("VISUAL");
    }
    assert_eq!(editor(), "vim");

    unsafe { std::env::set_var("EDITOR", "nano") };
    assert_eq!(editor(), "nano");

    unsafe { std::env::set_var("VISUAL", "nvim") };
    assert_eq!(editor(), "nvim");

    unsafe { std::env::set_var("VISUAL", "") };
    assert_eq!(editor(), "nano");
}
