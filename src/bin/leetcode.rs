extern crate clap;
use clap::{
    App, Arg, AppSettings, SubCommand,
};

fn main() {
    let m = App::new("leetcode")
        .author("clearloop <udtrokia@163.com>")
        .version("0.1.0")
        .about("Leet your code in command-line.")
        .subcommand(
            SubCommand::with_name("list")
                .about("List problems")
                .arg(Arg::with_name("all")
                     .short("w")
                     .long("all")
                     .help("List all problems")
                     .display_order(1)
                )
                .arg(Arg::with_name("algorithm")
                     .short("a")
                     .long("algorithm")
                     .help("List algorithm problems")
                     .display_order(2)
                )
                .arg(Arg::with_name("database")
                     .short("d")
                     .long("database")
                     .help("List database problems")
                     .display_order(3)
                )
                .arg(Arg::with_name("shell")
                     .short("s")
                     .long("shell")
                     .help("List shell problems")
                     .display_order(4)
                )
        )
        .setting(AppSettings::ArgRequiredElseHelp)
        .get_matches();

    if let Some(m) = m.subcommand_matches("list") {
        println!("{:?}", m);
    }
}
