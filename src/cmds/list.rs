//! list subcomAmand - List leetcode problems
//!
//! ```sh
//! leetcode-list 
//! List problems
//! 
//! USAGE:
//!     leetcode list [FLAGS] [OPTIONS] [keyword]
//! 
//! FLAGS:
//!     -h, --help       Prints help information
//!     -s, --stat       Show statistics of listed problems
//!     -V, --version    Prints version information
//! 
//! OPTIONS:
//!     -c, --category <category>    Fliter problems by category name
//!                                  [alogrithms, database, shell]
//!     -q, --query <query>          Fliter questions by conditions:
//!                                  Uppercase means negative
//!                                  e = easy     E = m+h
//!                                  m = medium   M = e+h
//!                                  h = hard     H = e+m
//!                                  d = done     D = not done
//!                                  l = locked   L = not locked
//!                                  s = starred  S = not starred
//! 
//! ARGS:
//!     <keyword>    Keyword in select query
//! 
//! EXAMPLES:
//!     leetcode list               List all questions
//!     leetcode list array         List questions that has "array" in name
//!     leetcode list -c database   List questions that in database category
//!     leetcode list -q eD         List questions that with easy level and not done
//! ```
use super::Command;
use crate::{cache::Cache, helper::Digit, err::Error};
use clap::{SubCommand, App, Arg, ArgMatches};
/// Abstract `list` command
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
            .visible_alias("l")
            .arg(Arg::with_name("category")
                 .short("c")
                 .long("category")
                 .takes_value(true)
                 .help(CATEGORY_HELP)
            )
            .arg(Arg::with_name("query")
                 .short("q")
                 .long("query")
                 .takes_value(true)
                 .help(QUERY_HELP)
            )
            .after_help(LIST_AFTER_HELP)
            .arg(Arg::with_name("stat")
                 .short("s")
                 .long("stat")
                 .help("Show statistics of listed problems")
            )
            .arg(Arg::with_name("keyword")
                 .takes_value(true)
                 .help("Keyword in select query")
            )
    }

    /// `list` command handler
    ///
    /// List commands contains "-c", "-q", "-s" flags.
    /// + matches with `-c` will override the default <all> keyword.
    /// + `-qs` 
    fn handler(m: &ArgMatches) -> Result<(), Error> {
        let cache = Cache::new().unwrap();
        let mut ps = cache.get_problems()?;

        if ps.len() == 0 {
            cache.download_problems()?;
            Self::handler(m)?
        }

        // filtering...
        // filter category
        if m.is_present("category") {
            ps.retain(|x| x.category == m.value_of("category").unwrap());
        }

        // filter query
        if m.is_present("query") {
            let query = m.value_of("query").unwrap();
            crate::helper::filter(&mut ps, query.to_string());
        }
        
        
        // retain if keyword exists
        if let Some(keyword) =  m.value_of("keyword") {
            ps.retain(|x| x.name.contains(&keyword));
        }
        
        for p in &ps { println!("{}", p); }
        
        // one more thing, filter stat
        if m.is_present("stat") {
            let mut listed = 0;
            let mut locked = 0;
            let mut starred = 0;
            let mut ac = 0;
            let mut notac = 0;
            let mut easy = 0;
            let mut medium = 0;
            let mut hard = 0;

            for p in ps {
                listed += 1;
                if p.starred {starred += 1;}
                if p.locked {locked += 1;}

                match p.status.as_str() {
                    "ac" => ac += 1,
                    "notac" => notac += 1,
                    _ => {}
                }

                match p.level {
                    1 => easy += 1,
                    2 => medium += 1,
                    3 => hard += 1,
                    _ => {}
                }
            }

            let remain = listed - ac - notac;
            println!("
        Listed: {}     Locked: {}     Starred: {}
        Accept: {}     Not-Ac: {}     Remain:  {}
        Easy  : {}     Medium: {}     Hard:    {}",
                     listed.digit(4), locked.digit(4), starred.digit(4),
                     ac.digit(4), notac.digit(4), remain.digit(4),
                     easy.digit(4), medium.digit(4), hard.digit(4),
            );
        }
        Ok(())
    }
}
