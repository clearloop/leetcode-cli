//! Test command
use super::Command;
use crate::{Error, Result};
use async_trait::async_trait;
use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command as ClapCommand};

/// Abstract Test Command
///
/// ```sh
/// leetcode-test
/// Edit question by id
///
/// USAGE:
///     leetcode test <id>
///
/// FLAGS:
///     -h, --help       Prints help information
///     -V, --version    Prints version information
///
/// ARGS:
///     <id>    question id
/// ```
pub struct TestCommand;

#[async_trait]
impl Command for TestCommand {
    /// `test` usage
    fn usage() -> ClapCommand {
        ClapCommand::new("test")
            .about("Test a question")
            .visible_alias("t")
            .arg(
                Arg::new("id")
                    .num_args(1)
                    .value_parser(clap::value_parser!(i32))
                    .help("question id"),
            )
            .arg(
                Arg::new("testcase")
                    .num_args(1)
                    .required(false)
                    .help("custom testcase"),
            )
            .arg(
                Arg::new("daily")
                    .short('d')
                    .long("daily")
                    .help("Test today's daily challenge")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("watch")
                    .short('w')
                    .long("watch")
                    .help("Watch for file changes and test automatically")
                    .action(ArgAction::SetTrue),
            )
            .group(
                ArgGroup::new("question-id")
                    .args(["id", "daily"])
                    .multiple(false)
                    .required(true),
            )
    }

    /// `test` handler
    async fn handler(m: &ArgMatches) -> Result<()> {
        use crate::cache::{Cache, Run};
        use crate::helper::code_path;
        use notify::{Config as NotifyConfig, Event, RecommendedWatcher, RecursiveMode, Watcher};
        use std::path::Path;
        use std::sync::mpsc::channel;
        use std::time::{Duration, Instant};

        let cache = Cache::new()?;

        let daily = m.get_one::<bool>("daily").unwrap_or(&false);
        let daily_id = if *daily {
            Some(cache.get_daily_problem_id().await?)
        } else {
            None
        };

        let id = m
            .get_one::<i32>("id")
            .copied()
            .or(daily_id)
            .ok_or(Error::NoneError)?;

        let testcase = m.get_one::<String>("testcase");
        let case_str: Option<String> = match testcase {
            Some(case) => Option::from(case.replace("\\n", "\n")),
            _ => None,
        };

        if *m.get_one::<bool>("watch").unwrap_or(&false) {
            let problem = cache.get_problem(id)?;
            let path_str = code_path(&problem, None)?;
            let path = Path::new(&path_str);
            let parent = path
                .parent()
                .ok_or_else(|| anyhow::anyhow!("Could not get parent directory"))?;

            let (tx, rx) = channel();
            let mut watcher = RecommendedWatcher::new(tx, NotifyConfig::default())
                .map_err(|e| anyhow::anyhow!("Failed to create watcher: {}", e))?;

            // Watch parent directory to handle file creation and atomic saves (which may delete/rename)
            watcher
                .watch(parent, RecursiveMode::NonRecursive)
                .map_err(|e| anyhow::anyhow!("Failed to watch parent directory: {}", e))?;

            if path.exists() {
                println!("Watching for changes in {}...", path_str);
                let res = cache.exec_problem(id, Run::Test, case_str.clone()).await?;
                println!("{}", res);
            } else {
                println!("File {} does not exist. Waiting for creation...", path_str);
            }

            let mut last_run = Instant::now() - Duration::from_secs(1);
            while let Ok(res_event) = rx.recv() {
                match res_event {
                    Ok(event) if event.paths.iter().any(|p| p == path) => {
                        if event.kind.is_create() {
                            println!("File created, watching for changes...");
                        } else if event.kind.is_modify() {
                            // Debounce: ignore events within 500ms of the last run
                            if last_run.elapsed() < Duration::from_millis(500) {
                                continue;
                            }

                            // Wait for atomic save to complete (e.g. file content flush)
                            std::thread::sleep(Duration::from_millis(200));
                            // Drain any subsequent events generated during the sleep
                            while rx.try_recv().is_ok() {}

                            if !path.exists() {
                                continue;
                            }
                            println!("File changed, testing again...");
                            match cache.exec_problem(id, Run::Test, case_str.clone()).await {
                                Ok(res) => println!("{}", res),
                                Err(e) => println!("Error: {}", e),
                            }
                            last_run = Instant::now();
                        }
                    }
                    Err(e) => println!("watch error: {:?}", e),
                    _ => {}
                }
            }
        } else {
            let res = cache.exec_problem(id, Run::Test, case_str).await?;
            println!("{}", res);
        }
        Ok(())
    }
}
