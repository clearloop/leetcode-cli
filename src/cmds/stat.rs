//! status command
use super::Command;
use async_trait::async_trait;
use clap::{ArgMatches, Command as ClapCommand};
use colored::Colorize;

/// Abstract statues command
///
/// ```sh
/// leetcode-stat
/// Show simple chart about submissions
///
/// USAGE:
///     leetcode stat
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
/// ```
pub struct StatCommand;

#[async_trait]
impl Command for StatCommand {
    /// `stat` usage
    fn usage<'a>() -> ClapCommand<'a> {
        ClapCommand::new("stat")
            .about("Show simple chart about submissions")
            .visible_alias("s")
    }

    /// `stat` handler
    async fn handler(_m: &ArgMatches) -> Result<(), crate::err::Error> {
        use crate::{helper::Digit, Cache};

        let cache = Cache::new()?;
        let res = cache.get_problems()?;

        let mut easy: f64 = 0.00;
        let mut easy_ac: f64 = 0.00;
        let mut medium: f64 = 0.00;
        let mut medium_ac: f64 = 0.00;
        let mut hard: f64 = 0.00;
        let mut hard_ac: f64 = 0.00;

        for i in res.into_iter() {
            match i.level {
                1 => {
                    easy += 1.00;
                    if i.status == "ac" {
                        easy_ac += 1.00;
                    }
                }
                2 => {
                    medium += 1.00;
                    if i.status == "ac" {
                        medium_ac += 1.00;
                    }
                }
                3 => {
                    hard += 1.00;
                    if i.status == "ac" {
                        hard_ac += 1.00;
                    }
                }
                _ => {}
            }
        }

        // level: len = 8
        // count: len = 10
        // percent: len = 16
        // chart: len = 32
        // title
        println!(
            "\n{}",
            "  Level      Count     Percent                                Chart".bright_black()
        );
        println!(
            "{}",
            "  -----------------------------------------------------------------".bright_black()
        );

        // lines
        for (i, l) in vec![(easy, easy_ac), (medium, medium_ac), (hard, hard_ac)]
            .iter()
            .enumerate()
        {
            match i {
                0 => {
                    print!("  {}", "Easy".bright_green());
                    print!("{}", " ".digit(4));
                }
                1 => {
                    print!("  {}", "Medium".bright_yellow());
                    print!("{}", " ".digit(2));
                }
                2 => {
                    print!("  {}", "Hard".bright_red());
                    print!("{}", " ".digit(4));
                }
                _ => continue,
            }

            let checked_div = |lhs: f64, rhs: f64| if rhs == 0. { 0. } else { lhs / rhs };
            let count = format!("{}/{}", l.1, l.0);
            let pct = format!("( {:.2} %)", checked_div(100.0 * l.1, l.0));
            let mut line = "".to_string();
            line.push_str(&" ".digit(10 - (count.len() as i32)));
            line.push_str(&count);
            line.push_str(&" ".digit(12 - (pct.len() as i32)));
            line.push_str(&pct);
            print!("{}", line);
            print!("     ");

            let done = "░"
                .repeat(checked_div(32.00 * l.1, l.0) as usize)
                .bright_green();
            let udone = "░"
                .repeat(32 - checked_div(32.00 * l.1, l.0) as usize)
                .red();
            print!("{}", done);
            println!("{}", udone);
        }
        println!();
        Ok(())
    }
}
