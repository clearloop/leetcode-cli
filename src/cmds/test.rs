//! test command
use super::Command;
use clap::{App, ArgMatches};

pub struct TestCommand;

impl Command for TestCommand {
    /// `edit` usage
    fn usage<'a, 'edit>() -> App<'a, 'edit> {
        use clap::{SubCommand, Arg};
        SubCommand::with_name("test")
            .about("Edit question by id")
            .visible_alias("t")
            .arg(Arg::with_name("id")
                 .takes_value(true)
                 .required(true)
                 .help("question id")
            )
    }

    /// `edit` handler
    fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::Cache;

        let id: i32 = m.value_of("id")?.parse()?;
        let cache = Cache::new()?;
        let res = cache.test_problem(id);
        if res.is_err() {
            return Err(res.err()?);
        }

        Ok(())
    }
}
