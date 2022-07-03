//! Pick command
use super::Command;
use crate::err::Error;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};
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
///     -q, --query <query>    Fliter questions by conditions:
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

static QUERY_HELP: &str = r#"Fliter questions by conditions:
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
    fn usage<'a>() -> ClapCommand<'a> {
        ClapCommand::new("pick")
            .about("Pick a problem")
            .visible_alias("p")
            .arg(Arg::with_name("id").help("Problem id").takes_value(true))
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
                Arg::with_name("tag")
                    .short('t')
                    .long("tag")
                    .takes_value(true)
                    .help("Filter questions by tag"),
            )
            .arg(
                Arg::with_name("daily")
                    .short('d')
                    .long("daily")
                    .takes_value(false)
                    .help("Pick today's daily challenge"),
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
            if m.is_present("plan") {
                let ids = crate::pym::exec(m.value_of("plan").unwrap_or(""))?;
                crate::helper::squash(&mut problems, ids)?;
            }
        }

        // tag filter
        if m.is_present("tag") {
            let ids = cache
                .clone()
                .get_tagged_questions(m.value_of("tag").unwrap_or(""))
                .await?;
            crate::helper::squash(&mut problems, ids)?;
        }

        // query filter
        if m.is_present("query") {
            let query = m.value_of("query").ok_or(Error::NoneError)?;
            crate::helper::filter(&mut problems, query.to_string());
        }

        let daily_id = if m.is_present("daily") {
            Some(cache.get_daily_problem_id().await?)
        } else {
            None
        };

        let fid = m
            .value_of("id")
            .and_then(|id| id.parse::<i32>().ok())
            .or(daily_id)
            .unwrap_or_else(|| {
                // Pick random without specify id
                let problem = &problems[rand::thread_rng().gen_range(0..problems.len())];
                problem.fid
            });

        let r = cache.get_question(fid).await;
        if r.is_err() {
            let e = r.err().ok_or(Error::NoneError)?;
            eprintln!("{:?}", &e);
            if let Error::FeatureError(_) | Error::NetworkError(_) = e {
                Self::handler(m).await?;
            }
        } else {
            println!("{}", r?);
        }

        Ok(())
    }
}
