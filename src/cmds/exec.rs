//! exec command
use super::Command;
use clap::{App, ArgMatches};

/// Exec Command
pub struct ExecCommand;

impl Command for ExecCommand {
    /// `exec` usage
    fn usage<'a, 'edit>() -> App<'a, 'edit> {
        use clap::{SubCommand, Arg};
        SubCommand::with_name("exec")
            .about("Submit solution")
            .visible_alias("x")
            .arg(Arg::with_name("id")
                 .takes_value(true)
                 .required(true)
                 .help("question id")
            )
    }

    /// `exec` handler
    fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::cache::{Cache, Run};

        let id: i32 = m.value_of("id")?.parse()?;
        let cache = Cache::new()?;
        let res = cache.exec_problem(id, Run::Submit)?;

        println!("{}", res);
        Ok(())
    }
}
