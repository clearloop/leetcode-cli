//! Exec command
use super::Command;
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
    }

    /// `exec` handler
    fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let id: i32 = m.value_of("id")?.parse()?;
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Submit, None)?;

        println!("{}", res);
        Ok(())
    }
}
