//! All subcommands in leetcode-cli
//! 
//! ```sh
//! SUBCOMMANDS:
//!     help    Prints this message or the help of the given subcommand(s)
//!     list    List problems
//! ```
use clap::{App, ArgMatches};
use crate::err::Error;
/// Abstract commands' traits.
pub trait Command {
    /// Usage of the spefic command
    fn usage<'a, 'c>() -> App<'a, 'c>;

    /// The handler will deal [args, options,...] from the command-line
    fn handler(m: &ArgMatches) -> Result<(), Error>;
}

mod data;
mod edit;
mod list;
mod pick;
mod stat;
mod test;
pub use data::DataCommand;
pub use edit::EditCommand;
pub use list::ListCommand;
pub use pick::PickCommand;
pub use stat::StatCommand;
pub use test::TestCommand;
