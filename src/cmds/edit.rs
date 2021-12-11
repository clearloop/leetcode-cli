//! Edit command
use crate::Error;
use super::Command;
use async_trait::async_trait;
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

#[async_trait]
impl Command for EditCommand {
    /// `edit` usage
    fn usage<'a, 'edit>() -> App<'a, 'edit> {
        use clap::{Arg, SubCommand};
        SubCommand::with_name("edit")
            .about("Edit question by id")
            .visible_alias("e")
            .arg(
                Arg::with_name("lang")
                    .short("l")
                    .long("lang")
                    .takes_value(true)
                    .help("Edit with specific language"),
            )
            .arg(
                Arg::with_name("id")
                    .takes_value(true)
                    .required(true)
                    .help("question id"),
            )
    }

    /// `edit` handler
    async fn handler(m: &ArgMatches<'_>) -> Result<(), crate::Error> {
        use crate::{cache::models::Question, Cache};
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let id: i32 = m.value_of("id").ok_or(Error::NoneError)?.parse()?;
        let cache = Cache::new()?;
        let target = cache.get_problem(id)?;
        let mut conf = cache.to_owned().0.conf;

        // condition language
        if m.is_present("lang") {
            conf.code.lang = m.value_of("lang").ok_or(Error::NoneError)?.to_string();
            conf.sync()?;
        }

        let lang = conf.code.lang;
        let path = crate::helper::code_path(&target, Some(lang.to_owned()))?;
        let tests_path = crate::helper::test_cases_path(&target)?;

        if !Path::new(&path).exists() {
            let mut qr = serde_json::from_str(&target.desc);
            if qr.is_err() {
                qr = Ok(cache.get_question(id).await?);
            }

            let question: Question = qr?;

            let mut file_code = File::create(&path)?;
            let mut file_tests = File::create(&tests_path)?;

            let mut flag = false;
            for d in question.defs.0 {
                if d.value == lang {
                    flag = true;
                    file_code.write_all(d.code.to_string().as_bytes())?;
                    file_tests.write_all(question.all_cases.as_bytes())?;
                }
            }

            // if language is not found in the list of supported languges clean up files
            if !flag {
                std::fs::remove_file(&path)?;
                std::fs::remove_file(&tests_path)?;

                return Err(crate::Error::FeatureError(format!(
                    "This question doesn't support {}, please try another",
                    &lang
                )));
            }
        }

        std::process::Command::new(conf.code.editor)
            .arg(path)
            .status()?;
        Ok(())
    }
}
