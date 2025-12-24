//! All subcommands in leetcode-cli
//!
//! ```sh
//! SUBCOMMANDS:
//!     data    Manage Cache [aliases: d]
//!     edit    Edit question by id [aliases: e]
//!     exec    Submit solution [aliases: x]
//!     list    List problems [aliases: l]
//!     pick    Pick a problem [aliases: p]
//!     stat    Show simple chart about submissions [aliases: s]
//!     test    Test a question [aliases: t]
//!     completions    Generate shell completions [aliases: c]
//!     help    Prints this message or the help of the given subcommand(s)
//! ```

mod completions;
mod data;
mod edit;
mod exec;
mod list;
mod pick;
mod stat;
mod test;

pub use completions::CompletionsArgs;
pub use data::DataArgs;
pub use edit::EditArgs;
pub use exec::ExecArgs;
pub use list::ListArgs;
pub use pick::PickArgs;
pub use stat::StatArgs;
pub use test::TestArgs;
