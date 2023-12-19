//! Exec command
use super::Command;
use crate::{Error, Result};
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

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
    }

    /// `exec` handler
    async fn handler(m: &ArgMatches) -> Result<()> {
        use crate::cache::{Cache, Run};

        let id: i32 = *m.get_one::<i32>("id").ok_or(Error::NoneError)?;
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Submit, None).await?;

        println!("{}", res);
        Ok(())
    }
}
