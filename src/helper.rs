//! A set of helper traits
pub use self::{
    digit::Digit,
    file::{code_path, load_script, test_cases_path},
    filter::{filter, squash},
    html::HTML,
};

/// Convert i32 to specific digits string.
mod digit {
    /// Abstract Digit trait, fill the empty space to specific length.
    pub trait Digit<T> {
        fn digit(self, d: T) -> String;
    }

    impl Digit<i32> for i32 {
        fn digit(self, d: i32) -> String {
            let mut s = self.to_string();
            let space = " ".repeat((d as usize) - s.len());
            s.push_str(&space);

            s
        }
    }

    impl Digit<i32> for String {
        fn digit(self, d: i32) -> String {
            let mut s = self.clone();
            let space = " ".repeat((d as usize) - self.len());
            s.push_str(&space);

            s
        }
    }

    impl Digit<i32> for &'static str {
        fn digit(self, d: i32) -> String {
            let mut s = self.to_string();
            let space = " ".repeat((d as usize) - self.len());
            s.push_str(&space);

            s
        }
    }
}

/// Question filter tool
mod filter {
    use crate::cache::models::Problem;
    /// Abstract query filter
    ///
    /// ```sh
    ///     -q, --query <query>          Filter questions by conditions:
    ///                                  Uppercase means negative
    ///                                  e = easy     E = m+h
    ///                                  m = medium   M = e+h
    ///                                  h = hard     H = e+m
    ///                                  d = done     D = not done
    ///                                  l = locked   L = not locked
    ///                                  s = starred  S = not starred
    /// ```
    pub fn filter(ps: &mut Vec<Problem>, query: String) {
        for p in query.chars() {
            match p {
                'l' => ps.retain(|x| x.locked),
                'L' => ps.retain(|x| !x.locked),
                's' => ps.retain(|x| x.starred),
                'S' => ps.retain(|x| !x.starred),
                'e' => ps.retain(|x| x.level == 1),
                'E' => ps.retain(|x| x.level != 1),
                'm' => ps.retain(|x| x.level == 2),
                'M' => ps.retain(|x| x.level != 2),
                'h' => ps.retain(|x| x.level == 3),
                'H' => ps.retain(|x| x.level != 3),
                'd' => ps.retain(|x| x.status == "ac"),
                'D' => ps.retain(|x| x.status != "ac"),
                _ => {}
            }
        }
    }

    /// Squash questions and ids
    pub fn squash(ps: &mut Vec<Problem>, ids: Vec<String>) -> crate::Result<()> {
        use std::collections::HashMap;

        let mut map: HashMap<String, bool> = HashMap::new();
        ids.iter().for_each(|x| {
            map.insert(x.to_string(), true).unwrap_or_default();
        });
        ps.retain(|x| map.get(&x.id.to_string()).is_some());
        Ok(())
    }
}

pub fn superscript(n: u8) -> String {
    match n {
        x if x >= 10 => format!("{}{}", superscript(n / 10), superscript(n % 10)),
        0 => "⁰".to_string(),
        1 => "¹".to_string(),
        2 => "²".to_string(),
        3 => "³".to_string(),
        4 => "⁴".to_string(),
        5 => "⁵".to_string(),
        6 => "⁶".to_string(),
        7 => "⁷".to_string(),
        8 => "⁸".to_string(),
        9 => "⁹".to_string(),
        _ => n.to_string(),
    }
}

pub fn subscript(n: u8) -> String {
    match n {
        x if x >= 10 => format!("{}{}", subscript(n / 10), subscript(n % 10)),
        0 => "₀".to_string(),
        1 => "₁".to_string(),
        2 => "₂".to_string(),
        3 => "₃".to_string(),
        4 => "₄".to_string(),
        5 => "₅".to_string(),
        6 => "₆".to_string(),
        7 => "₇".to_string(),
        8 => "₈".to_string(),
        9 => "₉".to_string(),
        _ => n.to_string(),
    }
}

/// Render html to command-line
mod html {
    use crate::helper::{subscript, superscript};
    use regex::Captures;
    use scraper::Html;

    /// Html render plugin
    pub trait HTML {
        fn render(&self) -> String;
    }

    impl HTML for String {
        fn render(&self) -> String {
            let sup_re = regex::Regex::new(r"<sup>(?P<num>[0-9]*)</sup>").unwrap();
            let sub_re = regex::Regex::new(r"<sub>(?P<num>[0-9]*)</sub>").unwrap();

            let res = sup_re.replace_all(self, |cap: &Captures| {
                let num: u8 = cap["num"].to_string().parse().unwrap();
                superscript(num)
            });

            let res = sub_re.replace_all(&res, |cap: &Captures| {
                let num: u8 = cap["num"].to_string().parse().unwrap();
                subscript(num)
            });

            let frag = Html::parse_fragment(&res);

            let res = frag
                .root_element()
                .text()
                .fold(String::new(), |acc, e| acc + e);

            res
        }
    }
}

mod file {
    /// Convert file suffix from language type
    pub fn suffix(l: &str) -> crate::Result<&'static str> {
        match l {
            "bash" => Ok("sh"),
            "c" => Ok("c"),
            "cpp" => Ok("cpp"),
            "csharp" => Ok("cs"),
            "elixir" => Ok("ex"),
            "golang" => Ok("go"),
            "java" => Ok("java"),
            "javascript" => Ok("js"),
            "kotlin" => Ok("kt"),
            "mysql" => Ok("sql"),
            "php" => Ok("php"),
            "python" => Ok("py"),
            "python3" => Ok("py"),
            "ruby" => Ok("rb"),
            "rust" => Ok("rs"),
            "scala" => Ok("scala"),
            "swift" => Ok("swift"),
            "typescript" => Ok("ts"),
            _ => Ok("c"),
        }
    }

    use crate::{cache::models::Problem, Error};

    /// Generate test cases path by fid
    pub fn test_cases_path(problem: &Problem) -> crate::Result<String> {
        let conf = crate::config::Config::locate()?;
        let mut path = format!("{}/{}.tests.dat", conf.storage.code()?, conf.code.pick);

        path = path.replace("${fid}", &problem.fid.to_string());
        path = path.replace("${slug}", &problem.slug.to_string());
        Ok(path)
    }

    /// Generate code path by fid
    pub fn code_path(problem: &Problem, l: Option<String>) -> crate::Result<String> {
        let conf = crate::config::Config::locate()?;
        let mut lang = conf.code.lang;
        if l.is_some() {
            lang = l.ok_or(Error::NoneError)?;
        }

        let mut path = format!(
            "{}/{}.{}",
            conf.storage.code()?,
            conf.code.pick,
            suffix(&lang)?,
        );

        path = path.replace("${fid}", &problem.fid.to_string());
        path = path.replace("${slug}", &problem.slug.to_string());

        Ok(path)
    }

    /// Load python scripts
    pub fn load_script(module: &str) -> crate::Result<String> {
        use std::fs::File;
        use std::io::Read;
        let conf = crate::config::Config::locate()?;
        let mut script = "".to_string();
        File::open(format!("{}/{}.py", conf.storage.scripts()?, module))?
            .read_to_string(&mut script)?;

        Ok(script)
    }
}
