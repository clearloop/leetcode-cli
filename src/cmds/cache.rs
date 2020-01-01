//! cache managger
use super::Command;
use crate::{cache::Cache, helper::Digit};
use colored::Colorize;
use clap::{SubCommand, App, Arg, ArgMatches};

/// Abstract `cache` command in `leetcode-cli`.
///
/// ## Handler
/// + update cache
/// + delete cache
/// + show the location of cache.
///
/// ## Storage Config
/// + cache -> Problems db
/// + code -> code storage dir
/// + root -> root path of `leetcode-cli`
pub struct CacheCommand;

impl Command for CacheCommand {
    /// `cache` command usage
    fn usage<'a, 'cache>() -> App<'a, 'cache> {
        SubCommand::with_name("cache")
            .about("Manage cache")
            .visible_alias("cc")
            .arg(Arg::with_name("delete")
                 .display_order(1)
                 .short("d")
                 .long("delete")
                 .help("Delete cache")
            )
            .arg(Arg::with_name("update")
                 .display_order(2)
                 .short("u")
                 .long("update")
                 .help("Update cache")
            )
    }

    /// `cache` handler
    fn handler(m: &ArgMatches) {
        use std::fs::File;
        use std::path::Path;
        
        let cache = Cache::new().unwrap();
        let path = cache.0.conf.storage.cache();
        let f = File::open(&path).unwrap();
        let len = format!("{}K", f.metadata().unwrap().len() / 1000);

        let out = format!(
            "{}{}",
            Path::new(&path)
                .file_name().unwrap()
                .to_string_lossy()
                .to_string().digit(70 - (len.len() as i32))
                .bright_green(),
            len
        );

        let mut title = "Cache".digit(66);
        title.push_str("Size");
        title.push_str("\n");
        title.push_str(&"-".repeat(70).to_string());

        let mut flags = 0;
        if m.is_present("delete") {
            flags += 1;
            cache.clean().unwrap();
            println!("{}", "ok!".bright_green());
        }

        if m.is_present("update") {
            flags += 1;
            cache.update().unwrap();
            println!("{}", "ok!".bright_green());
        }

        if flags == 0 {
            println!("{}", title.bright_black());
            println!("{}", out);
        }
    }
}
