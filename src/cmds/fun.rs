// TODO: get rid of this debug command? clean it && make it permanent?

//! Pick command
use super::Command;
use crate::err::Error;
use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
/// Abstract contest command
///
/// ```sh
/// leetcode-contest
/// Pick a problem
///
/// USAGE:
///     leetcode contest [OPTIONS] [id]
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
pub struct FunCommand;

static _QUERY_HELP: &str = r#"Fliter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

#[async_trait]
impl Command for FunCommand {
    /// `contest` usage
    fn usage<'a, 'contest>() -> App<'a, 'contest> {
        SubCommand::with_name("fun")
            .about("fun")
            .visible_alias("f")
            .arg(
                Arg::with_name("query")
                .help("GraphQL query - MUST be of the format `query a { ... }`")
                .takes_value(true)
                .short("q")
                .conflicts_with("type")
                .required(true)
            ).arg(
                Arg::with_name("variables")
                .help("Variables to pass to the GraphQL query, e.g. `{'slug': 'two-sum'}`")
                .takes_value(true)
                .short("v")
                .requires("query")
            ).arg(
                Arg::with_name("type")
                .help("type to get the definition of, e.g. `ContestNode`")
                .takes_value(true)
                .short("t")
                .required(true)
                .conflicts_with("query")
            )
    }

    /// `contest` handler
    async fn handler(m: &ArgMatches<'_>) -> Result<(), Error> {
        use crate::cache::Cache;

        let cache = Cache::new()?;
        let query = if let Some(q) = m.value_of("query") { q.to_string() }
        else if let Some(t) = m.value_of("type"){
            "query a {
              __type(name: \"$type\") {
                name 
                fields {
                  name 
                  args {
                    name 
                    description 
                    defaultValue 
                    type {
                      name 
                      kind 
                      ofType {
                        name 
                        kind 
                      }
                    }
                  }
                  type {
                    name 
                    kind 
                    ofType {
                      name 
                      kind 
                    }
                  }
                }
              }
            }".replace("$type", t)
        } else { unreachable!() };
        let vars = m.value_of("variables")
            .map(|v| v.to_string());
        println!("{}", cache.0.get_graphql(query, vars).await?.text().await?);

        Ok(())
    }
}
