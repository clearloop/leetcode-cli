//! Pick command
use crate::cache::models::Problem;
use crate::err::Error;
use clap::Args;

static QUERY_HELP: &str = r#"Filter questions by conditions:
Uppercase means negative
e = easy     E = m+h
m = medium   M = e+h
h = hard     H = e+m
d = done     D = not done
l = locked   L = not locked
s = starred  S = not starred"#;

/// Pick command arguments
#[derive(Args)]
pub struct PickArgs {
    /// Problem id
    #[arg(value_parser = clap::value_parser!(i32))]
    pub id: Option<i32>,

    /// Problem name
    #[arg(short = 'n', long)]
    pub name: Option<String>,

    /// Invoking python scripts to filter questions
    #[arg(short = 'p', long)]
    pub plan: Option<String>,

    /// Filter questions by conditions
    #[arg(short, long, help = QUERY_HELP)]
    pub query: Option<String>,

    /// Filter questions by tag
    #[arg(short, long)]
    pub tag: Option<String>,

    /// Pick today's daily challenge
    #[arg(short = 'd', long)]
    pub daily: bool,
}

impl PickArgs {
    /// `pick` handler
    pub async fn run(&self) -> Result<(), Error> {
        use crate::cache::Cache;
        use rand::Rng;

        let cache = Cache::new()?;
        let mut problems = cache.get_problems()?;
        if problems.is_empty() {
            cache.download_problems().await?;
            return Box::pin(self.run()).await;
        }

        // filtering...
        // pym scripts
        #[cfg(feature = "pym")]
        {
            if let Some(ref plan) = self.plan {
                let ids = crate::pym::exec(plan)?;
                crate::helper::squash(&mut problems, ids)?;
            }
        }

        // tag filter
        if let Some(ref tag) = self.tag {
            let ids = cache.clone().get_tagged_questions(tag).await?;
            crate::helper::squash(&mut problems, ids)?;
        }

        // query filter
        if let Some(ref query) = self.query {
            crate::helper::filter(&mut problems, query.to_string());
        }

        let daily_id = if self.daily {
            Some(cache.get_daily_problem_id().await?)
        } else {
            None
        };

        let fid = if let Some(ref quesname) = self.name {
            // check for name specified, or closest name
            closest_named_problem(&problems, quesname).unwrap_or(1)
        } else {
            self.id.or(daily_id).unwrap_or_else(|| {
                // Pick random without specify id
                let problem = &problems[rand::rng().random_range(0..problems.len())];
                problem.fid
            })
        };

        let r = cache.get_question(fid).await;

        match r {
            Ok(q) => println!("{}", q.desc()),
            Err(e) => {
                eprintln!("{:?}", e);
                if let Error::Reqwest(_) = e {
                    Box::pin(self.run()).await?;
                }
            }
        }

        Ok(())
    }
}

// Returns the closest problem according to a scoring algorithm
// taking into account both the longest common subsequence and the size
// problem string (to compensate for smaller strings having smaller lcs).
// Returns None if there are no problems in the problem list
fn closest_named_problem(problems: &Vec<Problem>, lookup_name: &str) -> Option<i32> {
    let max_name_size: usize = problems.iter().map(|p| p.name.len()).max()?;
    // Init table to the max name length of all the problems to share
    // the same table allocation
    let mut table: Vec<usize> = vec![0; (max_name_size + 1) * (lookup_name.len() + 1)];

    // this is guaranteed because of the earlier max None propegation
    assert!(!problems.is_empty());
    let mut max_score = 0;
    let mut current_problem = &problems[0];
    for problem in problems {
        // In case bug becomes bugged, always return the matching string
        if problem.name == lookup_name {
            return Some(problem.fid);
        }

        let this_lcs = longest_common_subsequence(&mut table, &problem.name, lookup_name);
        let this_score = this_lcs * (max_name_size - problem.name.len());

        if this_score > max_score {
            max_score = this_score;
            current_problem = problem;
        }
    }

    Some(current_problem.fid)
}

// Longest commong subsequence DP approach O(nm) space and time. Table must be at least
// (text1.len() + 1) * (text2.len() + 1) length or greater and is mutated every call
fn longest_common_subsequence(table: &mut [usize], text1: &str, text2: &str) -> usize {
    assert!(table.len() >= (text1.len() + 1) * (text2.len() + 1));
    let height: usize = text1.len() + 1;
    let width: usize = text2.len() + 1;

    // initialize base cases to 0
    for i in 0..height {
        table[i * width + (width - 1)] = 0;
    }
    for j in 0..width {
        table[((height - 1) * width) + j] = 0;
    }

    let mut i: usize = height - 1;
    let mut j: usize;
    for c0 in text1.chars().rev() {
        i -= 1;
        j = width - 1;
        for c1 in text2.chars().rev() {
            j -= 1;
            if c0.to_lowercase().next() == c1.to_lowercase().next() {
                table[i * width + j] = 1 + table[(i + 1) * width + j + 1];
            } else {
                let a = table[(i + 1) * width + j];
                let b = table[i * width + j + 1];
                table[i * width + j] = std::cmp::max(a, b);
            }
        }
    }
    table[0]
}
