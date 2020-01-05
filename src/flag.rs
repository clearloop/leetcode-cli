//! Flags in leetcode-cli
//!
//! ```sh
//! FLAGS:
//!     -d, --debug      debug mode
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//! ```
use clap::Arg;
use env_logger::Env;
use crate::err::Error;

/// Abstract flag trait
pub trait Flag {
    fn usage<'a, 'f>() -> Arg<'a, 'f>;
    fn handler() -> Result<(), Error>;
}

/// Debug logger
pub struct Debug;

impl Flag for Debug {
    fn usage<'a, 'f>() -> Arg<'a, 'f> {
        Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("debug mode")
    }

    fn handler() -> Result<(), Error>{
        env_logger::from_env(
            Env::default().default_filter_or("leetcode")
        ).init();

        Ok(())
    }
}
