//! Test command
use crate::{Error, Result};
use clap::Args;

/// Test command arguments
#[derive(Args)]
#[command(group = clap::ArgGroup::new("question-id").args(&["id", "daily"]).required(true))]
pub struct TestArgs {
    /// Question id
    #[arg(value_parser = clap::value_parser!(i32))]
    pub id: Option<i32>,

    /// Custom testcase
    pub testcase: Option<String>,

    /// Test today's daily challenge
    #[arg(short = 'd', long)]
    pub daily: bool,

    /// Watch for file changes and test automatically
    #[arg(short, long)]
    pub watch: bool,
}

impl TestArgs {
    /// `test` handler
    pub async fn run(&self) -> Result<()> {
        use crate::cache::{Cache, Run};
        use crate::helper::code_path;
        use notify::{Config as NotifyConfig, RecommendedWatcher, RecursiveMode, Watcher};
        use std::path::Path;
        use std::sync::mpsc::channel;
        use std::time::{Duration, Instant};

        let cache = Cache::new()?;
        let daily_id = if self.daily {
            Some(cache.get_daily_problem_id().await?)
        } else {
            None
        };

        let id = self.id.or(daily_id).ok_or(Error::NoneError)?;

        let case_str: Option<String> = self.testcase.as_ref().map(|case| case.replace("\\n", "\n"));

        if self.watch {
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
