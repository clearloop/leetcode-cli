//! [![doc](https://img.shields.io/badge/0.1.0-docs-green.svg)](https://docs.rs/leetcode-cli/)
//! [![Crates.io](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
//! [![Crates.io](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
//! [![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)
//!
//! ref to [leetcode-cli](https://github.com/skygragon/leetcode-cli), rust version.
//! 
//! ## Note
//! 
//! > (OS X support for now)
//! 
//! Please make sure you have logined in `leetcode.com` with `chrome`.
//!
//!
//! ## Installaion
//!
//! ```sh
//! cargo install leetcode-cli
//! ```
//!
//! ## Usage
//! 
//! ```sh
//! leetcode 0.1.0
//! clearloop <udtrokia@163.com>
//! Leet your code in command-line.
//! 
//! USAGE:
//!     leetcode [FLAGS] [SUBCOMMAND]
//! 
//! FLAGS:
//!     -d, --debug      debug mode
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//! 
//! SUBCOMMANDS:
//!     help    Prints this message or the help of the given subcommand(s)
//!     list    List problems
//! ```
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

mod cache;
mod cfg;
mod err;
mod plugins;
pub mod cmds;
pub mod flag;
