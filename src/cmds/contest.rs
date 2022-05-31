// TODO: clean this entire file
//! Pick command
use super::Command;
use crate::err::Error;
use async_trait::async_trait;
use clap::{App, Arg, ArgMatches, SubCommand};

/// TODO: put the actual correct docstring here
/// ```
pub struct ContestCommand;

// TODO: and here
static _QUERY_HELP: &str = r#"Fliter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

/*
 *
*/

fn time_diff_from_now(time_since_epoch: i64) -> i64 {
    use chrono::{Utc,TimeZone};
    let now = Utc::now();
    let time = Utc.timestamp(time_since_epoch, 0);
    let diff = now.signed_duration_since(time);
    -diff.num_seconds()
}

#[async_trait]
impl Command for ContestCommand {
    /// `contest` usage
    fn usage<'a, 'contest>() -> App<'a, 'contest> {
        SubCommand::with_name("contest")
            .about("Run a contest")
            .visible_alias("c")
            .arg(
                Arg::with_name("title")
                .help("Contest title (e.g. 'weekly-contest-999')")
                .takes_value(true)
                .required(true)
            ).arg(
                Arg::with_name("update")
                .help("push contest problems into db")
                .short("u")
            ).arg(
                Arg::with_name("register")
                .help("register for contest")
                .short("r")
            )
    }

    /// `contest` handler
    async fn handler(m: &ArgMatches<'_>) -> Result<(), Error> {
        use crate::cache::Cache;
        use std::io::{stdout, Write};
        use std::thread::sleep;
        use std::time::Duration;

        let cache = Cache::new()?;
        let contest_slug = m.value_of("title").unwrap();
        let mut contest = cache.get_contest(contest_slug).await?;
        debug!("{:#?}", contest);
        if m.is_present("register") {
            if contest.registered {
                println!("You are already registered for this contest.");
            } else {
                println!("Registering for contest...");
                cache.0.register_contest(contest_slug).await?;
                println!("Registered!");
                contest = cache.get_contest(contest_slug).await?;
            }
        }

        let tdiff = time_diff_from_now(contest.start_time);
        if tdiff > 0 {
            loop {
                let tdiff = time_diff_from_now(contest.start_time);
                if tdiff < 0 { break; }
                print!("starts in {} seconds      \r", tdiff);
                stdout().flush().unwrap();
                sleep(Duration::from_secs(1));
            }
            println!("");
            contest = cache.get_contest(contest_slug).await?;
        } else {
            println!("started {} seconds ago", -tdiff);
        };

        println!("{}", contest);
        println!("fID    Points Difficulty Title");
        println!("------|------|----------|--------------------");

        for question_stub in contest.questions {
            let slug = &question_stub.title_slug;
            let (problem,_question) = cache.get_contest_qnp(slug).await?;
            println!("{:5} |{:5} |{:9} |{}",
                problem.fid,
                question_stub.credit,
                problem.level,
                problem.name
            );
            debug!("{:#?}", problem);
            //println!("{:#?}", cache.get_problem(problem.fid)?);
            debug!("----------------------------------");
            if m.is_present("update") {
                cache.push_problem(problem)?;
            }
        }

        Ok(())
    }
}
