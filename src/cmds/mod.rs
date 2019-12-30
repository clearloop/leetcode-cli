//! cmds mod contains all subcommands in leetcode-cli
//! 
//! + list -> List problems
use clap::{App, ArgMatches};

/// Abstract commands' traits.
pub trait Command {
    /// Usage of the spefic command
    fn usage<'a, 'c>() -> App<'a, 'c>;

    /// The handler will deal [args, options,...] from the command-line
    fn handler(m: &ArgMatches);
}

mod list;
pub use list::ListCommand;
