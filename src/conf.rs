use std::collections::HashMap;

#[derive(Debug)]
pub struct Config {
    pub categories: [&'static str; 3],
    pub langs: [&'static str; 16],
    pub urls: HashMap<&'static str, &'static str>,
    pub auto_login: HashMap<&'static str, i32>,
    pub code: HashMap<&'static str, &'static str>,
    pub file: HashMap<&'static str, &'static str>,
    pub color: HashMap<&'static str, &'static str>,
    pub icon: HashMap<&'static str, &'static str>,
    pub network: HashMap<&'static str, i32>,
    pub plugins: HashMap<&'static str, &'static str>,
}

impl std::default::Default for Config {
    fn default() -> Config {
        let mut urls: HashMap<&'static str, &'static str> = HashMap::new();
        urls.insert("base", "https://leetcode.com");
        urls.insert("graphql", "https://leetcode.com/graphql");
        urls.insert("login", "https://leetcode.com/accounts/login");
        urls.insert("problems", "https://leetcode.com/api/problems/$category/");
        urls.insert("problem", "https://leetcode.com/problems/$slug/description/");
        urls.insert("test", "https://leetcode.com/problems/$slug/interpret_solution/");
        urls.insert("session", "https://leetcode.com/session/");
        urls.insert("submit", "https://leetcode.com/problems/$slug/submit/");
        urls.insert("submissions", "https://leetcode.com/api/submissions/$slug");
        urls.insert("submission", "https://leetcode.com/submissions/detail/$id/");
        urls.insert("verify", "https://leetcode.com/submissions/detail/$id/check/");
        urls.insert("favorites", "https://leetcode.com/list/api/questions");
        urls.insert("favorite_delete", "https://leetcode.com/list/api/questions/$hash/$id");
        urls.insert("plugin", "https://github.com/skygragon/leetcode-cli-plugins/raw/master/plugins/$name.js");
        
        let mut auto_login: HashMap<&'static str, i32> = HashMap::new();
        auto_login.insert("enable", 0);
        auto_login.insert("retry", 2);
        
        let mut code: HashMap<&'static str, &'static str> = HashMap::new();
        code.insert("editor", "emacs");
        code.insert("lang", "rust");
        
        let mut file: HashMap<&'static str, &'static str> = HashMap::new();
        file.insert("show", "${fid}.${slug}");
        file.insert("submission", "${fid}.${slug}.${sid}.${ac}");
        
        let mut color: HashMap<&'static str, &'static str> = HashMap::new();
        color.insert("enable", "true");
        color.insert("theme", "default");
        
        let mut icon: HashMap<&'static str, &'static str> = HashMap::new();
        icon.insert("theme", "");
        
        let mut network: HashMap<&'static str, i32> = HashMap::new();
        network.insert("concurrency", 10);
        network.insert("delay", 1);
        
        let plugins: HashMap<&'static str, &'static str> = HashMap::new();

        Config {
            categories: [
                "algorithms",
                "database",
                "shell"
            ],
            langs: [
                "bash",
                "c",
                "cpp",
                "csharp",
                "golang",
                "java",
                "javascript",
                "kotlin",
                "mysql",
                "php",
                "python",
                "python3",
                "ruby",
                "rust",
                "scala",
                "swift"
            ],
            urls,
            auto_login,
            code,
            file,
            color,
            icon,
            network,
            plugins,
        }
    }
}
