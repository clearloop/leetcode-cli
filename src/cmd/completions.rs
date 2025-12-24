//! Completions command
use crate::err::Error;
use clap::{Args, Command as ClapCommand};
use clap_complete::{Generator, Shell, generate};

/// Completions command arguments
#[derive(Args)]
pub struct CompletionsArgs {
    /// Shell type [possible values: bash, elvish, fish, powershell, zsh]
    #[arg(value_parser = clap::value_parser!(Shell))]
    pub shell: Option<Shell>,
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

impl CompletionsArgs {
    /// Generate and print shell completions
    pub fn run(&self, cmd: &mut ClapCommand) -> Result<(), Error> {
        let shell = self
            .shell
            .or_else(Shell::from_env)
            .ok_or(Error::MatchError)?;
        let completions = get_completions_string(shell, cmd)?;
        println!("{}", completions);
        Ok(())
    }
}
