//! Test command
use super::Command;
use async_trait::async_trait;
use clap::{App, ArgMatches};

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
    fn usage<'a, 'edit>() -> App<'a, 'edit> {
        use clap::{Arg, SubCommand};
        SubCommand::with_name("test")
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
    async fn handler(m: &ArgMatches<'_>) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};
        let id: i32 = m.value_of("id")?.parse()?;
        let testcase = m.value_of("testcase");
        let case_str: Option<String>;
        match testcase {
            Some(case) => case_str = Option::from(case.replace("\\n", "\n")),
            _ => case_str = None,
        }
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Test, case_str).await?;

        println!("{}", res);
        Ok(())
    }
}
