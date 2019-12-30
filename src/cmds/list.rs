//! list subcomAmand - List leeAtcode problems
//!
//! ```
//! leetcode-list 
//! List problems
//! 
//! USAGE:
//!     leetcode list [FLAGS]
//! 
//! FLAGS:
//!     -w, --all          List all problems
//!     -a, --algorithms   List algorithm problems
//!     -d, --database     List database problems
//!     -s, --shell        List shell problems
//!     -h, --help         Prints help information
//!     -V, --version      Prints version information
//! ```
use clap::{SubCommand, App, Arg, ArgMatches};
use crate::plugins::leetcode;
use super::Command;

/// abstract `list` command in `leetcode-cli`.
pub struct ListCommand;

/// implement Command trait for `list`
impl Command for ListCommand {
    /// `list` command usage
    fn usage<'a, 'list>() -> App<'a, 'list> {
        SubCommand::with_name("list")
            .about("List problems")
            .arg(Arg::with_name("all")
                 .short("w")
                 .long("all")
                 .help("List all problems")
                 .display_order(1)
            )
            .arg(Arg::with_name("algorithms")
                 .short("a")
                 .long("algorithm")
                 .help("List algorithm problems")
                 .display_order(2)
            )
            .arg(Arg::with_name("database")
                 .short("d")
                 .long("database")
                 .help("List database problems")
                 .display_order(3)
            )
            .arg(Arg::with_name("shell")
                 .short("s")
                 .long("shell")
                 .help("List shell problems")
                 .display_order(4)
            )
    }

    /// `list` command handler
    /// List commands contains "algorithm", "database", and "shell" methods.
    ///
    /// because of...leetcode content these three categories.
    fn handler(m: &ArgMatches) {
        let cli = leetcode::LeetCode::new();
        if m.is_present("algorithms") {
            let mut res = cli.get_user_info();
            info!("{:?}", res.text());
        } else if m.is_present("database") {
            let mut res = cli.get_user_info();
            info!("{:?}", res.text());
        } else if m.is_present("shell") {
            let mut res = cli.get_user_info();
            info!("{:?}", res.text());
        } else {
            let mut res = cli.get_user_info();
            info!("{:?}", res.text());
        }
    }
}
