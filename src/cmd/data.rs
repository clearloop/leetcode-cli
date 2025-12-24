//! Cache manager
use crate::{Error, cache::Cache, helper::Digit};
use clap::Args;
use colored::Colorize;

/// Data command arguments
#[derive(Args)]
pub struct DataArgs {
    /// Delete cache
    #[arg(short, long)]
    pub delete: bool,

    /// Update cache
    #[arg(short, long)]
    pub update: bool,
}

impl DataArgs {
    /// `data` handler
    pub async fn run(&self) -> Result<(), Error> {
        use std::fs::File;
        use std::path::Path;

        let cache = Cache::new()?;
        let path = cache.0.conf.storage.cache()?;
        let f = File::open(&path)?;
        let len = format!("{}K", f.metadata()?.len() / 1000);

        let out = format!(
            "  {}{}",
            Path::new(&path)
                .file_name()
                .ok_or(Error::NoneError)?
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
        if self.delete {
            flags += 1;
            cache.clean()?;
            println!("{}", "ok!".bright_green());
        }

        if self.update {
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
