//! list subcommand - List leetcode problems
use crate::{cache::Cache, err::Error, helper::Digit};
use clap::Args;

static CATEGORY_HELP: &str = r#"Filter problems by category name
[algorithms, database, shell, concurrency]
"#;

static QUERY_HELP: &str = r#"Filter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

static LIST_AFTER_HELP: &str = r#"EXAMPLES:
    leetcode list                   List all questions
    leetcode list array             List questions that has "array" in name, and this is letter non-sensitive
    leetcode list -c database       List questions that in database category
    leetcode list -q eD             List questions that with easy level and not done
    leetcode list -t linked-list    List questions that under tag "linked-list"
    leetcode list -r 50 100         List questions that has id in between 50 and 100
"#;

/// List command arguments
#[derive(Args)]
#[command(after_help = LIST_AFTER_HELP)]
pub struct ListArgs {
    /// Keyword in select query
    pub keyword: Option<String>,

    /// Filter problems by category name
    #[arg(short, long, help = CATEGORY_HELP)]
    pub category: Option<String>,

    /// Invoking python scripts to filter questions
    #[arg(short, long)]
    pub plan: Option<String>,

    /// Filter questions by conditions
    #[arg(short, long, help = QUERY_HELP)]
    pub query: Option<String>,

    /// Filter questions by id range
    #[arg(short, long, num_args = 2.., value_parser = clap::value_parser!(i32))]
    pub range: Vec<i32>,

    /// Show statistics of listed problems
    #[arg(short, long)]
    pub stat: bool,

    /// Filter questions by tag
    #[arg(short, long)]
    pub tag: Option<String>,
}

impl ListArgs {
    /// `list` command handler
    pub async fn run(&self) -> Result<(), Error> {
        trace!("Input list command...");

        let cache = Cache::new()?;
        let mut ps = cache.get_problems()?;

        // if cache doesn't exist, request a new copy
        if ps.is_empty() {
            cache.download_problems().await?;
            return Box::pin(self.run()).await;
        }

        // filtering...
        // pym scripts
        #[cfg(feature = "pym")]
        {
            if let Some(ref plan) = self.plan {
                let ids = crate::pym::exec(plan)?;
                crate::helper::squash(&mut ps, ids)?;
            }
        }

        // filter tag
        if let Some(ref tag) = self.tag {
            let ids = cache.get_tagged_questions(tag).await?;
            crate::helper::squash(&mut ps, ids)?;
        }

        // filter category
        if let Some(ref category) = self.category {
            ps.retain(|x| x.category == *category);
        }

        // filter query
        if let Some(ref query) = self.query {
            crate::helper::filter(&mut ps, query.to_string());
        }

        // filter range
        if self.range.len() >= 2 {
            ps.retain(|x| self.range[0] <= x.fid && x.fid <= self.range[1]);
        }

        // retain if keyword exists
        if let Some(ref keyword) = self.keyword {
            let lowercase_kw = keyword.to_lowercase();
            ps.retain(|x| x.name.to_lowercase().contains(&lowercase_kw));
        }

        // output problem lines sorted by [problem number] like
        // [ 1 ] Two Sum
        // [ 2 ] Add Two Numbers
        ps.sort_unstable_by_key(|p| p.fid);

        let out: Vec<String> = ps.iter().map(ToString::to_string).collect();
        println!("{}", out.join("\n"));

        // one more thing, filter stat
        if self.stat {
            let mut listed = 0;
            let mut locked = 0;
            let mut starred = 0;
            let mut ac = 0;
            let mut notac = 0;
            let mut easy = 0;
            let mut medium = 0;
            let mut hard = 0;

            for p in ps {
                listed += 1;
                if p.starred {
                    starred += 1;
                }
                if p.locked {
                    locked += 1;
                }

                match p.status.as_str() {
                    "ac" => ac += 1,
                    "notac" => notac += 1,
                    _ => {}
                }

                match p.level {
                    1 => easy += 1,
                    2 => medium += 1,
                    3 => hard += 1,
                    _ => {}
                }
            }

            let remain = listed - ac - notac;
            println!(
                "
        Listed: {}     Locked: {}     Starred: {}
        Accept: {}     Not-Ac: {}     Remain:  {}
        Easy  : {}     Medium: {}     Hard:    {}",
                listed.digit(4),
                locked.digit(4),
                starred.digit(4),
                ac.digit(4),
                notac.digit(4),
                remain.digit(4),
                easy.digit(4),
                medium.digit(4),
                hard.digit(4),
            );
        }
        Ok(())
    }
}
