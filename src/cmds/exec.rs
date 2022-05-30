//! Exec command
use crate::Error;
use super::Command;
use async_trait::async_trait;
use clap::{App, ArgMatches};

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
    fn usage<'a, 'edit>() -> App<'a, 'edit> {
        use clap::{Arg, SubCommand};
        SubCommand::with_name("exec")
            .about("Submit solution")
            .visible_alias("x")
            .arg(
                Arg::with_name("id")
                    .takes_value(true)
                    .required(true)
                    .help("question id"),
            )
            .arg(
                Arg::with_name("contest")
                .help("submit to contest")
                .short("c")
                .takes_value(true)
            )
    }

    /// `exec` handler
    async fn handler(m: &ArgMatches<'_>) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let id: i32 = m.value_of("id").ok_or(Error::NoneError)?.parse()?;
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Submit, None, m.value_of("contest")).await?;

        println!("{}", res);
        Ok(())
    }
}
