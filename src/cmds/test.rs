//! Test command
use super::Command;
use crate::{Error, Result};
use async_trait::async_trait;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command as ClapCommand};

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
            .about("Test a question")
            .visible_alias("t")
            .arg(
                Arg::new("id")
                    .num_args(1)
                    .value_parser(clap::value_parser!(i32))
                    .help("question id"),
            )
            .arg(
                Arg::new("testcase")
                    .num_args(1)
                    .required(false)
                    .help("custom testcase"),
            )
            .arg(
                Arg::new("daily")
                    .short('d')
                    .long("daily")
                    .help("Test today's daily challenge")
                    .action(ArgAction::SetTrue),
            )
            .group(
                ArgGroup::new("question-id")
                    .args(["id", "daily"])
                    .multiple(false)
                    .required(true),
            )
    }

    /// `test` handler
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

        let testcase = m.get_one::<String>("testcase");
        let case_str: Option<String> = match testcase {
            Some(case) => Option::from(case.replace("\\n", "\n")),
            _ => None,
        };
        let res = cache.exec_problem(id, Run::Test, case_str).await?;

        println!("{}", res);
        Ok(())
    }
}
