//! cache managger
use super::Command;
use crate::{cache::Cache, helper::Digit};
use clap::{SubCommand, App, Arg, ArgMatches};

/// Abstract `cache` command in `leetcode-cli`.
///
/// ## handler
/// + update cache
/// + delete cache
/// + show the location of cache.
pub struct CacheCommand;

impl Command for CacheCommand {
    /// `cache` command usage
    fn usage<'a, 'cache>() -> App<'a, 'list> {
        SubCommand::with_name("cache")
            .about("Manage cache")
            .visible_alias("cc")
            .arg(Arg::with_name("delete")
                 .short("d")
                 .long("delete")
                 .help("Delete cache")
            )
            .arg(Arg::with_name("update")
                 .short("u")
                 .long("update")
                 .help("Update cache")
            )
            .arg(Arg::with_name("id")
                 .takes_value(true)
                 .help("Cache name or question id")
            )
    }

    fn handler(m: &ArgMatches) {
        let cache = Cache::new();
    }
}
