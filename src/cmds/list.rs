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
//!                                  [algorithms, database, shell, concurrency]
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
use crate::{cache::Cache, err::Error, helper::Digit};
use async_trait::async_trait;
use clap::{Command as ClapCommand, Arg, ArgMatches};
/// Abstract `list` command
///
/// ## handler
/// + try to request cache
///   + prints the list
/// + if chache doesn't exist, download problems list
/// + ...
pub struct ListCommand;

static CATEGORY_HELP: &str = r#"Fliter problems by category name
[algorithms, database, shell, concurrency]
"#;

static QUERY_HELP: &str = r#"Fliter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

static LIST_AFTER_HELP: &str = r#"EXAMPLES:
    leetcode list                   List all questions
    leetcode list array             List questions that has "array" in name
    leetcode list -c database       List questions that in database category
    leetcode list -q eD             List questions that with easy level and not done
    leetcode list -t linked-list    List questions that under tag "linked-list"
    leetcode list -r 50 100         List questions that has id in between 50 and 100
"#;

/// implement Command trait for `list`
#[async_trait]
impl Command for ListCommand {
    /// `list` command usage
    fn usage<'a>() -> ClapCommand<'a> {
        ClapCommand::new("list")
            .about("List problems")
            .visible_alias("l")
            .arg(
                Arg::with_name("category")
                    .short('c')
                    .long("category")
                    .takes_value(true)
                    .help(CATEGORY_HELP),
            )
            .arg(
                Arg::with_name("plan")
                    .short('p')
                    .long("plan")
                    .takes_value(true)
                    .help("Invoking python scripts to filter questions"),
            )
            .arg(
                Arg::with_name("query")
                    .short('q')
                    .long("query")
                    .takes_value(true)
                    .help(QUERY_HELP),
            )
            .arg(
                Arg::with_name("range")
                    .short('r')
                    .long("range")
                    .takes_value(true)
                    .min_values(2)
                    .help("Filter questions by id range"),
            )
            .after_help(LIST_AFTER_HELP)
            .arg(
                Arg::with_name("stat")
                    .short('s')
                    .long("stat")
                    .help("Show statistics of listed problems"),
            )
            .arg(
                Arg::with_name("tag")
                    .short('t')
                    .long("tag")
                    .takes_value(true)
                    .help("Filter questions by tag"),
            )
            .arg(
                Arg::with_name("keyword")
                    .takes_value(true)
                    .help("Keyword in select query"),
            )
    }

    /// `list` command handler
    ///
    /// List commands contains "-c", "-q", "-s" flags.
    /// + matches with `-c` will override the default <all> keyword.
    /// + `-qs`
    async fn handler(m: &ArgMatches) -> Result<(), Error> {
        trace!("Input list command...");

        let cache = Cache::new()?;
        let mut ps = cache.get_problems()?;

        // if cache doesn't exist, request a new copy
        if ps.is_empty() {
            cache.download_problems().await?;
            return Self::handler(m).await;
        }

        // filtering...
        // pym scripts
        #[cfg(feature = "pym")]
        {
            if m.is_present("plan") {
                let ids = crate::pym::exec(m.value_of("plan").unwrap_or(""))?;
                crate::helper::squash(&mut ps, ids)?;
            }
        }

        // filter tag
        if m.is_present("tag") {
            let ids = cache
                .get_tagged_questions(m.value_of("tag").unwrap_or(""))
                .await?;
            crate::helper::squash(&mut ps, ids)?;
        }

        // filter category
        if m.is_present("category") {
            ps.retain(|x| x.category == m.value_of("category").unwrap_or("algorithms"));
        }

        // filter query
        if m.is_present("query") {
            let query = m.value_of("query").ok_or(Error::NoneError)?;
            crate::helper::filter(&mut ps, query.to_string());
        }

        // filter range
        if m.is_present("range") {
            let num_range: Vec<i32> = m.values_of("range").ok_or(Error::NoneError)?
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap_or(0))
                .collect();
            ps.retain(|x| num_range[0] <= x.fid && x.fid <= num_range[1]);
        }

        // retain if keyword exists
        if let Some(keyword) = m.value_of("keyword") {
            let lowercase_kw = keyword.to_lowercase();
            ps.retain(|x| x.name.to_lowercase().contains(&lowercase_kw));
        }

        let out: Vec<String> = ps.iter().map(ToString::to_string).collect();
        println!("{}", out.join("\n"));

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
                if p.starred {
                    starred += 1;
                }
                if p.locked {
                    locked += 1;
                }

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
            println!(
                "
        Listed: {}     Locked: {}     Starred: {}
        Accept: {}     Not-Ac: {}     Remain:  {}
        Easy  : {}     Medium: {}     Hard:    {}",
                listed.digit(4),
                locked.digit(4),
                starred.digit(4),
                ac.digit(4),
                notac.digit(4),
                remain.digit(4),
                easy.digit(4),
                medium.digit(4),
                hard.digit(4),
            );
        }
        Ok(())
    }
}
