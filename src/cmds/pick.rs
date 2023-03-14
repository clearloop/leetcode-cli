//! Pick command
use super::Command;
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

        let fid = m
            .get_one::<i32>("id")
            .copied()
            .or(daily_id)
            .unwrap_or_else(|| {
                // Pick random without specify id
                let problem = &problems[rand::thread_rng().gen_range(0..problems.len())];
                problem.fid
            });

        let r = cache.get_question(fid).await;

        match r {
            Ok(q) => println!("{}", q.desc()),
            Err(e) => {
                eprintln!("{:?}", e);
                if let Error::NetworkError(_) = e {
                    Self::handler(m).await?;
                }
            }
        }

        Ok(())
    }
}
