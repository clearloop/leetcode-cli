//! Exec command
use super::Command;
use crate::Error;
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
    fn usage<'a>() -> ClapCommand<'a> {
        ClapCommand::new("exec")
            .about("Submit solution")
            .visible_alias("x")
            .arg(
                Arg::with_name("id")
                    .takes_value(true)
                    .required(true)
                    .help("question id"),
            )
    }

    /// `exec` handler
    async fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let id: i32 = m.value_of("id").ok_or(Error::NoneError)?.parse()?;
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Submit, None).await?;

        println!("{}", res);
        Ok(())
    }
}
