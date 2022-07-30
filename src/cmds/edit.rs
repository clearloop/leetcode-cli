//! Edit command
use super::Command;
use crate::Error;
use async_trait::async_trait;
use clap::{Arg, ArgMatches, Command as ClapCommand};

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

pub const CODE_START: &str = r#"// @lc code=start"#;
pub const CODE_END: &str = r#"// @lc code=end"#;

#[async_trait]
impl Command for EditCommand {
    /// `edit` usage
    fn usage<'a>() -> ClapCommand<'a> {
        ClapCommand::new("edit")
            .about("Edit question by id")
            .visible_alias("e")
            .arg(
                Arg::with_name("lang")
                    .short('l')
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
            .arg(
                Arg::with_name("test")
                    .long("test")
                    .short('t')
                    .required(false)
                    .help("write test file"),
            )
    }

    /// `edit` handler
    async fn handler(m: &ArgMatches) -> Result<(), crate::Error> {
        use crate::{cache::models::Question, Cache};
        use std::fs::File;
        use std::io::Write;
        use std::path::Path;

        let id: i32 = m.value_of("id").ok_or(Error::NoneError)?.parse()?;
        let cache = Cache::new()?;
        let problem = cache.get_problem(id)?;
        let mut conf = cache.to_owned().0.conf;

        let test_flag = m.contains_id("test");

        let p_desc_comment = problem.desc_comment();
        // condition language
        if m.contains_id("lang") {
            conf.code.lang = m.value_of("lang").ok_or(Error::NoneError)?.to_string();
            conf.sync()?;
        }

        let lang = conf.code.lang;
        let path = crate::helper::code_path(&problem, Some(lang.to_owned()))?;

        if !Path::new(&path).exists() {
            let mut qr = serde_json::from_str(&problem.desc);
            if qr.is_err() {
                qr = Ok(cache.get_question(id).await?);
            }

            let question: Question = qr?;

            let mut file_code = File::create(&path)?;
            let question_desc = question.desc_comment() + "\n";

            let test_path = crate::helper::test_cases_path(&problem)?;
            let mut file_tests = File::create(&test_path)?;

            let mut flag = false;
            for d in question.defs.0 {
                if d.value == lang {
                    flag = true;
                    file_code.write_all(p_desc_comment.as_bytes())?;
                    file_code.write_all(question_desc.as_bytes())?;
                    file_code.write_all((CODE_START.to_string() + "\n").as_bytes())?;
                    file_code.write_all((d.code.to_string() + "\n").as_bytes())?;
                    file_code.write_all((CODE_END.to_string() + "\n").as_bytes())?;
                }

                if test_flag {
                    file_tests.write_all(question.all_cases.as_bytes())?;
                }
            }

            // if language is not found in the list of supported languges clean up files
            if !flag {
                std::fs::remove_file(&path)?;
                if test_flag {
                    std::fs::remove_file(&test_path)?;
                }
                return Err(crate::Error::FeatureError(format!(
                    "This question doesn't support {}, please try another",
                    &lang
                )));
            }
        }

        // Get arguments of the editor
        //
        // for example:
        //
        // ```toml
        // [code]
        // editor = "emacsclient"
        // editor-args = [ "-n", "-s", "doom" ]
        // ```
        //
        // ```rust
        // Command::new("emacsclient").args(&[ "-n", "-s", "doom", "<problem>" ])
        // ```
        let mut args: Vec<String> = Default::default();
        if let Some(editor_args) = conf.code.editor_args {
            args.extend_from_slice(&editor_args);
        }

        args.push(path);
        std::process::Command::new(conf.code.editor)
            .args(args)
            .status()?;
        Ok(())
    }
}
