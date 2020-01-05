//! Edit command
use super::Command;
use clap::{App, ArgMatches};

/// Abstract `edit` command
///
/// ```sh
/// leetcode-edit 
/// Edit question by id
/// 
/// USAGE:
///     leetcode edit <id>
/// 
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
/// 
/// ARGS:
///     <id>    question id
/// ```
pub struct EditCommand;

impl Command for EditCommand {
    /// `edit` usage
    fn usage<'a, 'edit>() -> App<'a, 'edit> {
        use clap::{SubCommand, Arg};
        SubCommand::with_name("edit")
            .about("Edit question by id")
            .visible_alias("e")
            .arg(Arg::with_name("id")
                 .takes_value(true)
                 .required(true)
                 .help("question id")
            )
    }

    /// `edit` handler
    fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::{Cache, cache::models::Question};
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let id: i32 = m.value_of("id")?.parse()?;
        let cache = Cache::new()?;
        let target = cache.get_problem(id)?;
        let path = crate::helper::code_path(&target)?;

        if !Path::new(&path).exists() {
            let mut f = File::create(&path)?;
            let question: Question = serde_json::from_str(&target.desc)?;
            let mut flag = false;
            for d in question.defs.0 {
                if d.value == cache.0.conf.code.lang {
                    flag = true;
                    f.write_all(d.code.to_string().as_bytes())?;
                }
            }

            if !flag {
                return Err(crate::Error::FeatureError(
                    format!(
                        "This question doesn't support {}, please try another",
                        &cache.0.conf.code.lang
                    )
                ));
            }
        }

        std::process::Command::new(cache.0.conf.code.editor).arg(path).status()?;
        Ok(())
    }
}
