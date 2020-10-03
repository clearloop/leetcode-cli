//! Clap Commanders
use crate::{
    cmds::{
        Command, DataCommand, EditCommand, ExecCommand, ListCommand, PickCommand, StatCommand,
        TestCommand,
    },
    err::Error,
    flag::{Debug, Flag},
};
use clap::{crate_name, crate_version, App, AppSettings};

/// Get maches
pub async fn main() -> Result<(), Error> {
    let m = App::new(crate_name!())
        .version(crate_version!())
        .about("May the Code be with You 👻")
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
        ("data", Some(sub_m)) => Ok(DataCommand::handler(sub_m).await?),
        ("edit", Some(sub_m)) => Ok(EditCommand::handler(sub_m).await?),
        ("exec", Some(sub_m)) => Ok(ExecCommand::handler(sub_m).await?),
        ("list", Some(sub_m)) => Ok(ListCommand::handler(sub_m).await?),
        ("pick", Some(sub_m)) => Ok(PickCommand::handler(sub_m).await?),
        ("stat", Some(sub_m)) => Ok(StatCommand::handler(sub_m).await?),
        ("test", Some(sub_m)) => Ok(TestCommand::handler(sub_m).await?),
        _ => Err(Error::MatchError),
    }
}
