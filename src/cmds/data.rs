//! Cache managger
use super::Command;
use crate::{cache::Cache, helper::Digit, Error};
use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};
use colored::Colorize;

/// Abstract `data` command
///
/// ```sh
/// leetcode-data
/// Manage Cache
///
/// USAGE:
///     leetcode data [FLAGS]
///
/// FLAGS:
///     -d, --delete     Delete cache
///     -u, --update     Update cache
///     -h, --help       Prints help information
///     -V, --version    Prints version information
/// ```
pub struct DataCommand;

#[async_trait]
impl Command for DataCommand {
    /// `data` command usage
    fn usage<'a, 'cache>() -> App<'a, 'cache> {
        SubCommand::with_name("data")
            .about("Manage Cache")
            .visible_alias("d")
            .arg(
                Arg::with_name("delete")
                    .display_order(1)
                    .short("d")
                    .long("delete")
                    .help("Delete cache"),
            )
            .arg(
                Arg::with_name("update")
                    .display_order(2)
                    .short("u")
                    .long("update")
                    .help("Update cache"),
            )
    }

    /// `data` handler
    async fn handler(m: &ArgMatches<'_>) -> Result<(), Error> {
        use std::fs::File;
        use std::path::Path;

        let cache = Cache::new()?;
        let path = cache.0.conf.storage.cache()?;
        let f = File::open(&path)?;
        let len = format!("{}K", f.metadata()?.len() / 1000);

        let out = format!(
            "  {}{}",
            Path::new(&path)
                .file_name().ok_or(Error::NoneError)?
                .to_string_lossy()
                .to_string()
                .digit(65 - (len.len() as i32))
                .bright_green(),
            len
        );

        let mut title = "\n  Cache".digit(63);
        title.push_str("Size");
        title.push_str("\n  ");
        title.push_str(&"-".repeat(65));

        let mut flags = 0;
        if m.is_present("delete") {
            flags += 1;
            cache.clean()?;
            println!("{}", "ok!".bright_green());
        }

        if m.is_present("update") {
            flags += 1;
            cache.update().await?;
            println!("{}", "ok!".bright_green());
        }

        if flags == 0 {
            println!("{}", title.bright_black());
            println!("{}\n", out);
        }

        Ok(())
    }
}
