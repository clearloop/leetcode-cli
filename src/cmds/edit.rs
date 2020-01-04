//! edit command
use super::Command;
use clap::{App, ArgMatches};

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
        let lang = cache.0.conf.code.lang;
        let mut path = format!(
            "{}/{}.{}",
            cache.0.conf.storage.code()?,
            cache.0.conf.code.pick,
            &crate::helper::suffix(&lang)?,
        );

        path = path.replace("${fid}", &target.fid.to_string());
        path = path.replace("${slug}", &target.slug.to_string());

        if !Path::new(&path).exists() {
            let mut f = File::create(&path)?;
            let question: Question = serde_json::from_str(&target.desc)?;
            let mut flag = false;
            for d in question.defs.0 {
                if d.value == lang.to_string() {
                    flag = true;
                    f.write_all(d.code.to_string().as_bytes())?;
                }
            }

            if !flag {
                return Err(crate::Error::FeatureError(
                    format!("This question doesn't support {}, please try another", &lang)
                ));
            }
        }

        std::process::Command::new(cache.0.conf.code.editor).arg(path).status()?;
        Ok(())
    }
}
