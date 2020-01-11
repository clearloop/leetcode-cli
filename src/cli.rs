//! Clap Commanders
use crate::{
    cmds::{
        Command, DataCommand, EditCommand, ExecCommand, ListCommand, PickCommand, StatCommand,
        TestCommand,
    },
    err::Error,
    flag::{Debug, Flag},
};
use clap::{App, AppSettings};

/// Get maches
pub fn main() -> Result<(), Error> {
    let m = App::new("leetcode")
        .author("clearloop <udtrokia@163.com>")
        .version("0.2.8")
        .about("Here's to the crazy ones 👻")
        .subcommands(vec![
            DataCommand::usage().display_order(1),
            EditCommand::usage().display_order(2),
            ExecCommand::usage().display_order(3),
            ListCommand::usage().display_order(4),
            PickCommand::usage().display_order(5),
            StatCommand::usage().display_order(6),
            TestCommand::usage().display_order(7),
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
        ("data", Some(sub_m)) => Ok(DataCommand::handler(sub_m)?),
        ("edit", Some(sub_m)) => Ok(EditCommand::handler(sub_m)?),
        ("exec", Some(sub_m)) => Ok(ExecCommand::handler(sub_m)?),
        ("list", Some(sub_m)) => Ok(ListCommand::handler(sub_m)?),
        ("pick", Some(sub_m)) => Ok(PickCommand::handler(sub_m)?),
        ("stat", Some(sub_m)) => Ok(StatCommand::handler(sub_m)?),
        ("test", Some(sub_m)) => Ok(TestCommand::handler(sub_m)?),
        _ => Err(Error::MatchError),
    }
}
