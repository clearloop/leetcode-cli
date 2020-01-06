//! Pick command
use super::Command;
use crate::err::Error;
use clap::{SubCommand, App, Arg, ArgMatches};

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

static QUERY_HELP: &'static str = r#"Fliter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

impl Command for PickCommand {
    /// `pick` usage
    fn usage<'a, 'pick>() -> App<'a, 'pick> {
        SubCommand::with_name("pick")
            .about("Pick a problem")
            .visible_alias("p")
            .arg(Arg::with_name("id")
                 .help("Problem id")
                 .takes_value(true)
            ).arg(Arg::with_name("query")
                  .short("q")
                  .long("query")
                  .takes_value(true)
                  .help(QUERY_HELP)
            ).arg(Arg::with_name("tag")
                 .short("t")
                 .long("tag")
                 .takes_value(true)
                 .help("Filter questions by tag")
            )
    }

    /// `pick` handler
    fn handler(m: &ArgMatches) -> Result<(), Error>{
        use rand::Rng;
        use crate::cache::Cache;
        use std::collections::HashMap;
        
        let cache = Cache::new()?;
        let mut problems = cache.get_problems()?;
        if problems.len() == 0 {
            cache.clone().download_problems()?;
            Self::handler(m)?
        }

        // tag filter
        if m.is_present("tag") {
            let mut map: HashMap<String, bool> = HashMap::new();
            let ids = cache.clone().get_tagged_questions(m.value_of("tag").unwrap_or(""))?;
            ids.iter().for_each(|x| {
                map.insert(x.to_string(), true).unwrap_or_default();
            });

            problems.retain(|x| map.get(&x.id.to_string()).is_some());
        }
        
        // query filter
        if m.is_present("query") {
            let query = m.value_of("query")?;
            crate::helper::filter(&mut problems, query.to_string());
        }

        if let Some(id) =  m.value_of("id") {
            problems.retain(|x| x.fid.to_string() == id);
        }

        let problem = &problems[rand::thread_rng().gen_range(0, problems.len())];

        let r = cache.get_question(problem.fid);
        if r.is_err() {
            let e = r.err()?;
            eprintln!("{:?}", &e);
            if let Error::FeatureError(_) | Error::NetworkError(_) = e {
                Self::handler(m)?;
            }
        } else {
            println!("{}", r?);
        }

        Ok(())
    }
}
