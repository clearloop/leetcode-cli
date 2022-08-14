//! System section
//!
//! This section is a set of constants after #88

const CATEGORIES: [&str; 4] = ["algorithms", "concurrency", "database", "shell"];

/// Leetcode API
#[derive(Clone, Debug)]
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
            base: "https://leetcode.com".into(),
            graphql: "https://leetcode.com/graphql".into(),
            login: "https://leetcode.com/accounts/login/".into(),
            problems: "https://leetcode.com/api/problems/$category/".into(),
            problem: "https://leetcode.com/problems/$slug/description/".into(),
            tag: "https://leetcode.com/tag/$slug/".into(),
            test: "https://leetcode.com/problems/$slug/interpret_solution/".into(),
            session: "https://leetcode.com/session/".into(),
            submit: "https://leetcode.com/problems/$slug/submit/".into(),
            submissions: "https://leetcode.com/submissions/detail/$id/".into(),
            submission: "https://leetcode.com/submissions/detail/$id/".into(),
            verify: "https://leetcode.com/submissions/detail/$id/check/".into(),
            favorites: "https://leetcode.com/list/api/questions".into(),
            favorite_delete: "https://leetcode.com/list/api/questions/$hash/$id".into(),
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
        self.problem.replace("$category", category)
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
#[derive(Clone, Debug)]
pub struct Sys {
    pub categories: Vec<String>,
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
