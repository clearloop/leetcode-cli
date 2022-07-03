//! All subcommands in leetcode-cli
//!
//! ```sh
//! SUBCOMMANDS:
//!     data    Manage Cache [aliases: d]
//!     edit    Edit question by id [aliases: e]
//!     list    List problems [aliases: l]
//!     pick    Pick a problem [aliases: p]
//!     stat    Show simple chart about submissions [aliases: s]
//!     test    Edit question by id [aliases: t]
//!     help    Prints this message or the help of the given subcommand(s)
//! ```
use crate::err::Error;
use async_trait::async_trait;
use clap::{ArgMatches, Command as ClapCommand};

/// Abstract commands' trait.
#[async_trait]
pub trait Command {
    /// Usage of the specific command
    fn usage<'a>() -> ClapCommand<'a>;

    /// The handler will deal [args, options,...] from the command-line
    async fn handler(m: &ArgMatches) -> Result<(), Error>;
}

mod data;
mod edit;
mod exec;
mod list;
mod pick;
mod stat;
mod test;
pub use data::DataCommand;
pub use edit::EditCommand;
pub use exec::ExecCommand;
pub use list::ListCommand;
pub use pick::PickCommand;
pub use stat::StatCommand;
pub use test::TestCommand;

pub use edit::{CODE_END, CODE_START};
