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
use super::Command;
use crate::cache::Cache;

/// Abstract `list` command in `leetcode-cli`.
///
/// ## handler
/// + try to request cache
///   + prints the list
/// + if chache doesn't exist, download problems list
/// + ...
pub struct ListCommand;

static CATEGORY_HELP: &'static str = r#"Fliter problems by category name
[alogrithms, database, shell]
"#;

static QUERY_HELP: &'static str = r#"Fliter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

static LIST_AFTER_HELP: &'static str = r#"EXAMPLES:
    leetcode list               List all questions
    leetcode list array         List questions that has "array" in name
    leetcode list -c database   List questions that in database category
    leetcode list -q eD         List questions that with easy level and not done    
"#;

/// implement Command trait for `list`
impl Command for ListCommand {
    /// `list` command usage
    fn usage<'a, 'list>() -> App<'a, 'list> {
        SubCommand::with_name("list")
            .about("List problems")
            .arg(Arg::with_name("category")
                 .short("c")
                 .long("category")
                 .help(CATEGORY_HELP)
            )
            .arg(Arg::with_name("query")
                 .short("q")
                 .long("query")
                 .help(QUERY_HELP)
            )
            .after_help(LIST_AFTER_HELP)
            .arg(Arg::with_name("stat")
                 .short("s")
                 .long("stat")
                 .help("Show statistics of listed problems")
            )
            .arg(Arg::with_name("tag")
                 .short("t")
                 .long("tag")
                 .help("Fliter problems by tag")
            )
            .arg(Arg::with_name("key")
                 .takes_value(true)
                 .help("Keywords in select query")
            )
    }

    /// `list` command handler
    /// List commands contains "algorithm", "database", and "shell" methods.
    ///
    /// because of...leetcode content these three categories.
    fn handler(m: &ArgMatches) {
        let cache = Cache::new();
        let r = cache.download_problems();
        println!("res: {:?}", r);
        // let cli = leetcode::LeetCode::new();
    }
}
