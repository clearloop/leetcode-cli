//! Clap commander
use clap::{App, AppSettings};
use crate::{
    cmds::{
        Command,
        ListCommand,
        PickCommand,
        StatCommand,
        CacheCommand,
    },
    flag::{
        Flag,
        Debug,
    },
    err::Error,
};

/// get maches
pub fn main() -> Result<(), Error>{
    let m = App::new("leetcode")
        .author("clearloop <udtrokia@163.com>")
        .version("0.1.6")
        .about("Leet your code in command-line.")
        .subcommands(vec![
            CacheCommand::usage().display_order(1),
            ListCommand::usage().display_order(2),
            PickCommand::usage().display_order(3),
            StatCommand::usage().display_order(4),
        ])
        .arg(Debug::usage())
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if m.is_present("debug") {
        Debug::handler()?;
    } else {
        env_logger::from_env(env_logger::Env::new().default_filter_or("info"))
        .format_timestamp(None)
        .init();
    }

    match m.subcommand() {
        ("list", Some(sub_m)) => Ok(ListCommand::handler(sub_m)?),
        ("pick", Some(sub_m)) => Ok(PickCommand::handler(sub_m)?),
        ("stat", Some(sub_m)) => Ok(StatCommand::handler(sub_m)?),
        ("cache", Some(sub_m)) => Ok(CacheCommand::handler(sub_m)?),
        _ => Err(Error::MatchError)
    }
}
