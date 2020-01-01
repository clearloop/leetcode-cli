//! Clap commander
use clap::{App, AppSettings};
use crate::{
    cmds::{
        Command,
        ListCommand,
        CacheCommand,
    },
    flag::{
        Flag,
        Debug,
    }
};

/// get maches
pub fn main() {
    let m = App::new("leetcode")
        .author("clearloop <udtrokia@163.com>")
        .version("0.1.5")
        .about("Leet your code in command-line.")
        .subcommands(vec![
            CacheCommand::usage().display_order(1),
            ListCommand::usage().display_order(2),
        ])
        .arg(Debug::usage())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if m.is_present("debug") {
        Debug::handler();
    } else {
        env_logger::from_env(env_logger::Env::new().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    }

    match m.subcommand() {
        ("list", Some(sub_m)) => ListCommand::handler(sub_m),
        ("cache", Some(sub_m)) => CacheCommand::handler(sub_m),
        _ => {}
    }
}
