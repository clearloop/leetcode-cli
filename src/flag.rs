//! Flags in leetcode-cli
//!
//! ```sh
//! FLAGS:
//!     -d, --debug      debug mode
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//! ```
use crate::err::Error;
use clap::{Arg, ArgAction};
use env_logger::Env;

/// Abstract flag trait
pub trait Flag {
    fn usage() -> Arg;
    fn handler() -> Result<(), Error>;
}

/// Debug logger
pub struct Debug;

impl Flag for Debug {
    fn usage() -> Arg {
        Arg::new("debug")
            .short('d')
            .long("debug")
            .help("debug mode")
            .action(ArgAction::SetTrue)
    }

    fn handler() -> Result<(), Error> {
        env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

        Ok(())
    }
}
