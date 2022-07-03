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
    fn usage<'a>() -> ClapCommand<'a> {
        ClapCommand::new("test")
            .about("Test question by id")
            .visible_alias("t")
            .arg(
                Arg::with_name("id")
                    .takes_value(true)
                    .required(true)
                    .help("question id"),
            )
            .arg(
                Arg::with_name("testcase")
                    .takes_value(true)
                    .required(false)
                    .help("custom testcase"),
            )
    }

    /// `test` handler
    async fn handler(m: &ArgMatches) -> Result<(), Error> {
        use crate::cache::{Cache, Run};
        let id: i32 = m.value_of("id").ok_or(Error::NoneError)?.parse()?;
        let testcase = m.value_of("testcase");
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
