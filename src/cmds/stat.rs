//! status command
use super::Command;
use colored::Colorize;
use clap::{SubCommand, App, ArgMatches};

/// Abstract statues command
pub struct StatCommand;

impl Command for StatCommand {
    /// `stat` usage
    fn usage<'a, 'stat>() -> App<'a, 'stat> {
        SubCommand::with_name("stat")
            .about("Show simple chart about submissions")
            .visible_alias("s")
    }

    /// `stat` handler
    fn handler(_m: &ArgMatches) {
        use crate::{
            cache::Cache,
            helper::Digit,
        };
        let cache = Cache::new().unwrap();
        let res = cache.get_problems().unwrap();
        
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
                    if i.state == "ac".to_string() {
                        easy_ac += 1.00;
                    }
                },
                2 => {
                    medium += 1.00;
                    if i.state == "ac".to_string() {
                        medium_ac += 1.00;
                    }
                },
                3 => {
                    hard += 1.00;
                    if i.state == "ac".to_string() {
                        hard_ac += 1.00;
                    }
                },
                _ => {}
            }
        }


        // level: len = 8
        // count: len = 8
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
        for (i, l) in vec![
            (easy, easy_ac),
            (medium, medium_ac),
            (hard, hard_ac)
        ].iter().enumerate() {
            match i {
                0 => {
                    print!("  {}", "Easy".bright_green());
                    print!("{}", " ".digit(4));
                },
                1 => {
                    print!("  {}", "Medium".bright_yellow());
                    print!("{}", " ".digit(2));
                },
                2 => {
                    print!("  {}", "Hard".bright_red());
                    print!("{}", " ".digit(4));
                },
                _ => continue
            }
            
            let count = format!("{}/{}", l.1, l.0);
            let pct = format!("( {:.2} %)", ((100.0 * l.1) / l.0));
            let mut line = "".to_string();
            line.push_str(&" ".digit(8 - (count.len() as i32)));
            line.push_str(&count);
            line.push_str(&" ".digit(12 - (pct.len() as i32)));
            line.push_str(&pct);
            print!("{}", line);
            print!("     ");
            
            let done = "░".repeat(
                ((32.00 * l.1) / l.0) as usize
            ).bright_green();
            let udone = "░".repeat(
                32 - ((32.00 * l.1) / l.0) as usize
            ).red();
            print!("{}", done);
            println!("{}", udone);
        }
        println!("");
    }
}
