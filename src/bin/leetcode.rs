use clap::{App, AppSettings};
use leetcode_cli::{
    cmds::{
        Command,
        ListCommand,
    },
    flag::{
        Flag,
        Debug,
    }
};

fn main() {
    let m = App::new("leetcode")
        .author("clearloop <udtrokia@163.com>")
        .version("0.1.0")
        .about("Leet your code in command-line.")
        .subcommand(ListCommand::usage())
        .arg(Debug::usage())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if m.is_present("debug") {
        Debug::handler();
    } else {
        env_logger::from_env(env_logger::Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    }

    match m.subcommand() {
        ("list", Some(sub_m)) => ListCommand::handler(sub_m),
        _ => {}
    }
}
