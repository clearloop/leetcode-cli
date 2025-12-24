//! Exec command
use super::Command;
use crate::{Error, Result};
use async_trait::async_trait;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command as ClapCommand};

/// Abstract Exec Command
///
/// ```sh
/// leetcode-exec
/// Submit solution
///
/// USAGE:
///     leetcode exec <id>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <id>    question id
/// ```
pub struct ExecCommand;

#[async_trait]
impl Command for ExecCommand {
    /// `exec` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("exec")
            .about("Submit solution")
            .visible_alias("x")
            .arg(
                Arg::new("id")
                    .num_args(1)
                    .required(true)
                    .value_parser(clap::value_parser!(i32))
                    .help("question id"),
            )
            .arg(
                Arg::new("daily")
                    .short('d')
                    .long("daily")
                    .help("Exec today's daily challenge")
                    .action(ArgAction::SetTrue),
            )
            .group(
                ArgGroup::new("question-id")
                    .args(["id", "daily"])
                    .multiple(false)
                    .required(true),
            )
    }

    /// `exec` handler
    async fn handler(m: &ArgMatches) -> Result<()> {
        use crate::cache::{Cache, Run};

        let cache = Cache::new()?;

        let daily = m.get_one::<bool>("daily").unwrap_or(&false);
        let daily_id = if *daily {
            Some(cache.get_daily_problem_id().await?)
        } else {
            None
        };

        let id = m
            .get_one::<i32>("id")
            .copied()
            .or(daily_id)
            .ok_or(Error::NoneError)?;

        let res = cache.exec_problem(id, Run::Submit, None).await?;

        println!("{}", res);
        Ok(())
    }
}
