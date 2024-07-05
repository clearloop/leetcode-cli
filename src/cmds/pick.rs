//! Pick command
use super::Command;
use crate::cache::models::Problem;
use crate::err::Error;
use async_trait::async_trait;
use clap::{Arg, ArgAction, ArgMatches, Command as ClapCommand};
/// Abstract pick command
///
/// ```sh
/// leetcode-pick
/// Pick a problem
///
/// USAGE:
///     leetcode pick [OPTIONS] [id]
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// OPTIONS:
///     -q, --query <query>    Filter questions by conditions:
///                            Uppercase means negative
///                            e = easy     E = m+h
///                            m = medium   M = e+h
///                            h = hard     H = e+m
///                            d = done     D = not done
///                            l = locked   L = not locked
///                            s = starred  S = not starred
///
/// ARGS:
///     <id>    Problem id
/// ```
pub struct PickCommand;

static QUERY_HELP: &str = r#"Filter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

#[async_trait]
impl Command for PickCommand {
    /// `pick` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("pick")
            .about("Pick a problem")
            .visible_alias("p")
            .arg(
                Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_parser(clap::value_parser!(String))
                    .help("Problem name")
                    .num_args(1),
            )
            .arg(
                Arg::new("id")
                    .value_parser(clap::value_parser!(i32))
                    .help("Problem id")
                    .num_args(1),
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
                Arg::new("tag")
                    .short('t')
                    .long("tag")
                    .num_args(1)
                    .help("Filter questions by tag"),
            )
            .arg(
                Arg::new("daily")
                    .short('d')
                    .long("daily")
                    .help("Pick today's daily challenge")
                    .action(ArgAction::SetTrue),
            )
    }

    /// `pick` handler
    async fn handler(m: &ArgMatches) -> Result<(), Error> {
        use crate::cache::Cache;
        use rand::Rng;

        let cache = Cache::new()?;
        let mut problems = cache.get_problems()?;
        if problems.is_empty() {
            cache.download_problems().await?;
            Self::handler(m).await?;
            return Ok(());
        }

        // filtering...
        // pym scripts
        #[cfg(feature = "pym")]
        {
            if m.contains_id("plan") {
                let ids = crate::pym::exec(m.get_one::<String>("plan").unwrap_or(&"".to_string()))?;
                crate::helper::squash(&mut problems, ids)?;
            }
        }

        // tag filter
        if m.contains_id("tag") {
            let ids = cache
                .clone()
                .get_tagged_questions(m.get_one::<String>("tag").unwrap_or(&"".to_string()))
                .await?;
            crate::helper::squash(&mut problems, ids)?;
        }

        // query filter
        if m.contains_id("query") {
            let query = m.get_one::<String>("query").ok_or(Error::NoneError)?;
            crate::helper::filter(&mut problems, query.to_string());
        }

        let daily_id = if m.contains_id("daily") {
            Some(cache.get_daily_problem_id().await?)
        } else {
            None
        };

        let fid = match m.contains_id("name") {
            // check for name specified, or closest name
            true => {
                match m.get_one::<String>("name") {
                    Some(quesname) => closest_named_problem(&problems, quesname).unwrap_or(1),
                    None => {
                        // Pick random without specify id
                        let problem = &problems[rand::thread_rng().gen_range(0..problems.len())];
                        problem.fid
                    }
                }
            }
            false => {
                m.get_one::<i32>("id")
                    .copied()
                    .or(daily_id)
                    .unwrap_or_else(|| {
                        // Pick random without specify id
                        let problem = &problems[rand::thread_rng().gen_range(0..problems.len())];
                        problem.fid
                    })
            }
        };

        let r = cache.get_question(fid).await;

        match r {
            Ok(q) => println!("{}", q.desc()),
            Err(e) => {
                eprintln!("{:?}", e);
                if let Error::Reqwest(_) = e {
                    Self::handler(m).await?;
                }
            }
        }

        Ok(())
    }
}

// Returns the closest problem according to a scoring algorithm
// taking into account both the longest common subsequence and the size
// problem string (to compensate for smaller strings having smaller lcs).
// Returns None if there are no problems in the problem list
fn closest_named_problem(problems: &Vec<Problem>, lookup_name: &str) -> Option<i32> {
    let max_name_size: usize = problems.iter().map(|p| p.name.len()).max()?;
    // Init table to the max name length of all the problems to share
    // the same table allocation
    let mut table: Vec<usize> = vec![0; (max_name_size + 1) * (lookup_name.len() + 1)];

    // this is guaranteed because of the earlier max None propegation
    assert!(!problems.is_empty());
    let mut max_score = 0;
    let mut current_problem = &problems[0];
    for problem in problems {
        // In case bug becomes bugged, always return the matching string
        if problem.name == lookup_name {
            return Some(problem.fid);
        }

        let this_lcs = longest_common_subsequence(&mut table, &problem.name, lookup_name);
        let this_score = this_lcs * (max_name_size - problem.name.len());

        if this_score > max_score {
            max_score = this_score;
            current_problem = problem;
        }
    }

    Some(current_problem.fid)
}

// Longest commong subsequence DP approach O(nm) space and time. Table must be at least
// (text1.len() + 1) * (text2.len() + 1) length or greater and is mutated every call
fn longest_common_subsequence(table: &mut Vec<usize>, text1: &str, text2: &str) -> usize {
    assert!(table.len() >= (text1.len() + 1) * (text2.len() + 1));
    let height: usize = text1.len() + 1;
    let width: usize = text2.len() + 1;

    // initialize base cases to 0
    for i in 0..height {
        table[i * width + (width - 1)] = 0;
    }
    for j in 0..width {
        table[((height - 1) * width) + j] = 0;
    }

    let mut i: usize = height - 1;
    let mut j: usize;
    for c0 in text1.chars().rev() {
        i -= 1;
        j = width - 1;
        for c1 in text2.chars().rev() {
            j -= 1;
            if c0.to_lowercase().next() == c1.to_lowercase().next() {
                table[i * width + j] = 1 + table[(i + 1) * width + j + 1];
            } else {
                let a = table[(i + 1) * width + j];
                let b = table[i * width + j + 1];
                table[i * width + j] = std::cmp::max(a, b);
            }
        }
    }
    table[0]
}
