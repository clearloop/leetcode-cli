//! Completions command

use super::Command;
use crate::err::Error;
use async_trait::async_trait;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
use clap_complete::{Generator, Shell, generate};

/// Abstract shell completions command
///
/// ```sh
/// Generate shell Completions
///
/// USAGE:
///     leetcode completions <shell>
///
/// ARGUMENTS:
///     <shell>  [possible values: bash, elvish, fish, powershell, zsh]
/// ```
pub struct CompletionCommand;

#[async_trait]
impl Command for CompletionCommand {
    /// `pick` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("completions")
            .about("Generate shell Completions")
            .visible_alias("c")
            .arg(
                Arg::new("shell")
                    .action(ArgAction::Set)
                    .value_parser(clap::value_parser!(Shell)),
            )
    }

    async fn handler(_m: &ArgMatches) -> Result<(), Error> {
        // defining custom handler to print the completions. Handler method signature limits taking
        // other params. We need &ArgMatches and &mut ClapCommand to generate completions.
        println!(
            "Don't use this handler. Does not implement the functionality to print completions. Use completions_handler() below."
        );
        Ok(())
    }
}

fn get_completions_string<G: Generator>(
    generator: G,
    cmd: &mut ClapCommand,
) -> Result<String, Error> {
    let mut v: Vec<u8> = Vec::new();
    let name = cmd.get_name().to_string();
    generate(generator, cmd, name, &mut v);
    Ok(String::from_utf8(v)?)
}

pub fn completion_handler(m: &ArgMatches, cmd: &mut ClapCommand) -> Result<(), Error> {
    let shell = *m.get_one::<Shell>("shell").unwrap_or(
        // if shell value is not provided try to get from the environment
        &Shell::from_env().ok_or(Error::MatchError)?,
    );
    let completions = get_completions_string(shell, cmd)?;
    println!("{}", completions);
    Ok(())
}
