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

/// This should be called before calling any cli method or printing any output.
pub fn reset_signal_pipe_handler() {
    #[cfg(target_family = "unix")]
    {
        use nix::sys::signal;

        unsafe {
            let _ = signal::signal(signal::Signal::SIGPIPE, signal::SigHandler::SigDfl)
                .map_err(|e| println!("{:?}", e));
        }
    }
}

/// Get maches
pub async fn main() -> Result<(), Error> {
    let _ = reset_signal_pipe_handler();
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

    if m.contains_id("debug") {
        Debug::handler()?;
    } else {
        env_logger::Builder::from_env(env_logger::Env::new().default_filter_or("info"))
            .format_timestamp(None)
            .init();
    }

    match m.subcommand() {
        Some(("data", sub_m)) => Ok(DataCommand::handler(sub_m).await?),
        Some(("edit", sub_m)) => Ok(EditCommand::handler(sub_m).await?),
        Some(("exec", sub_m)) => Ok(ExecCommand::handler(sub_m).await?),
        Some(("list", sub_m)) => Ok(ListCommand::handler(sub_m).await?),
        Some(("pick", sub_m)) => Ok(PickCommand::handler(sub_m).await?),
        Some(("stat", sub_m)) => Ok(StatCommand::handler(sub_m).await?),
        Some(("test", sub_m)) => Ok(TestCommand::handler(sub_m).await?),
        _ => Err(Error::MatchError),
    }
}
