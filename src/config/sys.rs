//! System section
//!
//! This section is a set of constants after #88

use serde::{Deserialize, Serialize};

const CATEGORIES: [&str; 4] = ["algorithms", "concurrency", "database", "shell"];

// TODO: find a better solution.
fn categories() -> Vec<String> {
    CATEGORIES.into_iter().map(|s| s.into()).collect()
}

/// Leetcode API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Urls {
    pub base: String,
    pub graphql: String,
    pub login: String,
    pub problems: String,
    pub problem: String,
    pub tag: String,
    pub test: String,
    pub session: String,
    pub submit: String,
    pub submissions: String,
    pub submission: String,
    pub verify: String,
    pub favorites: String,
    pub favorite_delete: String,
}

impl Default for Urls {
    fn default() -> Self {
        Self {
            base: "https://leetcode.cn".into(),
            graphql: "https://leetcode.cn/graphql".into(),
            login: "https://leetcode.cn/accounts/login/".into(),
            problems: "https://leetcode.cn/api/problems/$category/".into(),
            problem: "https://leetcode.cn/problems/$slug/description/".into(),
            tag: "https://leetcode.cn/tag/$slug/".into(),
            test: "https://leetcode.cn/problems/$slug/interpret_solution/".into(),
            session: "https://leetcode.cn/session/".into(),
            submit: "https://leetcode.cn/problems/$slug/submit/".into(),
            submissions: "https://leetcode.cn/submissions/detail/$id/".into(),
            submission: "https://leetcode.cn/submissions/detail/$id/".into(),
            verify: "https://leetcode.cn/submissions/detail/$id/check/".into(),
            favorites: "https://leetcode.cn/list/api/questions".into(),
            favorite_delete: "https://leetcode.cn/list/api/questions/$hash/$id".into(),
        }
    }
}

impl Urls {
    /// problem url with specific `$slug`
    pub fn problem(&self, slug: &str) -> String {
        self.problem.replace("$slug", slug)
    }

    /// problems url with specific `$category`
    pub fn problems(&self, category: &str) -> String {
        self.problems.replace("$category", category)
    }

    /// submit url with specific `$slug`
    pub fn submit(&self, slug: &str) -> String {
        self.submit.replace("$slug", slug)
    }

    /// tag url with specific `$slug`
    pub fn tag(&self, slug: &str) -> String {
        self.tag.replace("$slug", slug)
    }

    /// test url with specific `$slug`
    pub fn test(&self, slug: &str) -> String {
        self.test.replace("$slug", slug)
    }

    /// verify url with specific `$id`
    pub fn verify(&self, id: &str) -> String {
        self.verify.replace("$id", id)
    }
}

/// System settings, for leetcode api mainly
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sys {
    #[serde(default = "categories")]
    pub categories: Vec<String>,
    #[serde(default)]
    pub urls: Urls,
}

impl Default for Sys {
    fn default() -> Self {
        Self {
            categories: CATEGORIES.into_iter().map(|s| s.into()).collect(),
            urls: Default::default(),
        }
    }
}
