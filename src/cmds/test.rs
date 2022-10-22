//! Test command
use super::Command;
use crate::Error;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

/// Abstract Test Command
///
/// ```sh
/// leetcode-test
/// Edit question by id
///
/// USAGE:
///     leetcode test <id>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <id>    question id
/// ```
pub struct TestCommand;

#[async_trait]
impl Command for TestCommand {
    /// `test` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("test")
            .about("Test question by id")
            .visible_alias("t")
            .arg(
                Arg::new("id")
                    .num_args(1)
                    .required(true)
                    .help("question id"),
            )
            .arg(
                Arg::new("testcase")
                    .num_args(1)
                    .required(false)
                    .help("custom testcase"),
            )
    }

    /// `test` handler
    async fn handler(m: &ArgMatches) -> Result<(), Error> {
        use crate::cache::{Cache, Run};
        let id: i32 = m.get_one::<&str>("id").ok_or(Error::NoneError)?.parse()?;
        let testcase = m.get_one::<&str>("testcase");
        let case_str: Option<String> = match testcase {
            Some(case) => Option::from(case.replace("\\n", "\n")),
            _ => None,
        };
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Test, case_str).await?;

        println!("{}", res);
        Ok(())
    }
}
