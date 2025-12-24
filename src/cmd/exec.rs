//! Exec command
use crate::{Error, Result};
use clap::Args;

/// Exec command arguments
#[derive(Args)]
#[command(group = clap::ArgGroup::new("question-id").args(&["id", "daily"]).required(true))]
pub struct ExecArgs {
    /// Question id
    #[arg(value_parser = clap::value_parser!(i32))]
    pub id: Option<i32>,

    /// Submit today's daily challenge
    #[arg(short = 'd', long)]
    pub daily: bool,
}

impl ExecArgs {
    /// `exec` handler
    pub async fn run(&self) -> Result<()> {
        use crate::cache::{Cache, Run};

        let cache = Cache::new()?;

        let daily_id = if self.daily {
            Some(cache.get_daily_problem_id().await?)
        } else {
            None
        };

        let id = self.id.or(daily_id).ok_or(Error::NoneError)?;

        let res = cache.exec_problem(id, Run::Submit, None).await?;

        println!("{}", res);
        Ok(())
    }
}
