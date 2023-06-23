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
//!     -c, --category <category>    Filter problems by category name
//!                                  [algorithms, database, shell, concurrency]
//!     -q, --query <query>          Filter questions by conditions:
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
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
/// Abstract `list` command
///
/// ## handler
/// + try to request cache
///   + prints the list
/// + if cache doesn't exist, download problems list
/// + ...
pub struct ListCommand;

static CATEGORY_HELP: &str = r#"Filter problems by category name
[algorithms, database, shell, concurrency]
"#;

static QUERY_HELP: &str = r#"Filter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

static LIST_AFTER_HELP: &str = r#"EXAMPLES:
    leetcode list                   List all questions
    leetcode list array             List questions that has "array" in name, and this is letter non-sensitive
    leetcode list -c database       List questions that in database category
    leetcode list -q eD             List questions that with easy level and not done
    leetcode list -t linked-list    List questions that under tag "linked-list"
    leetcode list -r 50 100         List questions that has id in between 50 and 100
"#;

/// implement Command trait for `list`
#[async_trait]
impl Command for ListCommand {
    /// `list` command usage
    fn usage() -> ClapCommand {
        ClapCommand::new("list")
            .about("List problems")
            .visible_alias("l")
            .arg(
                Arg::new("category")
                    .short('c')
                    .long("category")
                    .num_args(1)
                    .help(CATEGORY_HELP),
            )
            .arg(
                Arg::new("plan")
                    .short('p')
                    .long("plan")
                    .num_args(1)
                    .help("Invoking python scripts to filter questions"),
            )
            .arg(
                Arg::new("query")
                    .short('q')
                    .long("query")
                    .num_args(1)
                    .help(QUERY_HELP),
            )
            .arg(
                Arg::new("range")
                    .short('r')
                    .long("range")
                    .num_args(2..)
                    .value_parser(clap::value_parser!(i32))
                    .help("Filter questions by id range"),
            )
            .after_help(LIST_AFTER_HELP)
            .arg(
                Arg::new("stat")
                    .short('s')
                    .long("stat")
                    .help("Show statistics of listed problems")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("tag")
                    .short('t')
                    .long("tag")
                    .num_args(1)
                    .help("Filter questions by tag"),
            )
            .arg(
                Arg::new("keyword")
                    .num_args(1)
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
            if m.contains_id("plan") {
                let ids = crate::pym::exec(m.get_one::<String>("plan").unwrap_or(&"".to_string()))?;
                crate::helper::squash(&mut ps, ids)?;
            }
        }

        // filter tag
        if m.contains_id("tag") {
            let ids = cache
                .get_tagged_questions(m.get_one::<String>("tag").unwrap_or(&"".to_string()))
                .await?;
            crate::helper::squash(&mut ps, ids)?;
        }

        // filter category
        if m.contains_id("category") {
            ps.retain(|x| {
                x.category
                    == *m
                        .get_one::<String>("category")
                        .unwrap_or(&"algorithms".to_string())
            });
        }

        // filter query
        if m.contains_id("query") {
            let query = m.get_one::<String>("query").ok_or(Error::NoneError)?;
            crate::helper::filter(&mut ps, query.to_string());
        }

        // filter range
        if m.contains_id("range") {
            let num_range: Vec<i32> = m
                .get_many::<i32>("range")
                .ok_or(Error::NoneError)?
                .copied()
                .into_iter()
                .collect();
            ps.retain(|x| num_range[0] <= x.fid && x.fid <= num_range[1]);
        }

        // retain if keyword exists
        if let Some(keyword) = m.get_one::<String>("keyword") {
            let lowercase_kw = keyword.to_lowercase();
            ps.retain(|x| x.name.to_lowercase().contains(&lowercase_kw));
        }

        // output problem lines sorted by [problem number] like
        // [ 1 ] Two Sum
        // [ 2 ] Add Two Numbers
        ps.sort_unstable_by_key(|p| p.fid);

        let out: Vec<String> = ps.iter().map(ToString::to_string).collect();
        println!("{}", out.join("\n"));

        // one more thing, filter stat
        if m.contains_id("stat") {
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
