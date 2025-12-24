//! Clap Commanders
use crate::{
    cmd::{
        completion_handler, Command, CompletionCommand, DataCommand, EditCommand, ExecCommand,
        ListCommand, PickCommand, StatCommand, TestCommand,
    },
    err::Error,
    flag::{Debug, Flag},
};
use clap::crate_version;
use log::LevelFilter;

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

/// Get matches
pub async fn main() -> Result<(), Error> {
    reset_signal_pipe_handler();

    let mut cmd = clap::Command::new("leetcode")
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
            CompletionCommand::usage().display_order(8),
        ])
        .arg(Debug::usage())
        .arg_required_else_help(true);

    let m = cmd.clone().get_matches();

    if m.get_flag("debug") {
        Debug::handler()?;
    } else {
        env_logger::Builder::new()
            .filter_level(LevelFilter::Info)
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
        Some(("completions", sub_m)) => Ok(completion_handler(sub_m, &mut cmd)?),
        _ => Err(Error::MatchError),
    }
}
