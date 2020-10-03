//! A set of helper traits
pub use self::digit::Digit;
pub use self::file::{code_path, load_script};
pub use self::filter::{filter, squash};
pub use self::html::HTML;

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
    ///     -q, --query <query>          Fliter questions by conditions:
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
    pub fn squash(ps: &mut Vec<Problem>, ids: Vec<String>) -> Result<(), crate::Error> {
        use std::collections::HashMap;

        let mut map: HashMap<String, bool> = HashMap::new();
        ids.iter().for_each(|x| {
            map.insert(x.to_string(), true).unwrap_or_default();
        });
        ps.retain(|x| map.get(&x.id.to_string()).is_some());
        Ok(())
    }
}

/// Render html to command-line
mod html {
    // use crate::Error;
    use colored::{Color, Colorize};
    use escaper::decode_html;
    use regex::Regex;
    pub enum Token {
        Plain(String),
        Bold(String),
        Sup(String),
        Sub(String),
        Font((String, Color)),
        Eof(String),
    }

    /// Html render plugin
    pub trait HTML {
        fn ser(&self) -> Vec<Token>;
        fn render(&self) -> String;
    }

    pub fn superscript(n: u8) -> String {
        match n {
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
            x if x > 10 => (superscript(n / 10).parse().unwrap_or(0)
                + superscript(n % 10).parse().unwrap_or(0))
            .to_string(),
            _ => n.to_string(),
        }
    }

    pub fn subscript(n: u8) -> String {
        match n {
            x if x >= 10 => (subscript(n / 10).parse().unwrap_or(0)
                + subscript(n % 10).parse().unwrap_or(0))
            .to_string(),
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
    impl HTML for String {
        fn ser(&self) -> Vec<Token> {
            // empty tags
            let tks = self.to_string();
            let res: Vec<Token>;
            // styled
            {
                let mut ptr = 0;
                let mut output = vec![];
                let mut bold = false;
                let mut sup = false;
                let mut sub = false;
                let mut color: Option<Color> = None;

                // TODO: check how to make this `unwrap` more flexible..
                //
                // or looks better.
                //
                // or do some handwrite matching.
                let re_color = Regex::new(r#"color=['"]([^'"]+)"#).unwrap();
                for (i, e) in tks.chars().enumerate() {
                    match e {
                        '<' => {
                            if bold {
                                output.push(Token::Bold(tks[ptr..i].to_string()));
                                bold = false;
                            } else if sup {
                                output.push(Token::Sup(tks[ptr..i].to_string()));
                                sup = false;
                            } else if sub {
                                output.push(Token::Sub(tks[ptr..i].to_string()));
                                sub = false;
                            } else if color.is_some() {
                                output.push(Token::Font((tks[ptr..i].to_string(), color.unwrap())));
                                color = None;
                            } else {
                                output.push(Token::Plain(tks[ptr..i].to_string()));
                            }
                            ptr = i;
                        }
                        '>' => {
                            match &tks[i - 1..i] {
                                "-" => continue,
                                _ => match &tks[(ptr + 1)..i] {
                                    "b" | "strong" => bold = true,
                                    "sup" => sup = true,
                                    "sub" => sub = true,
                                    s if s.starts_with("font") => {
                                        color = re_color
                                            .captures(s)
                                            .and_then(|caps| caps.get(1))
                                            .and_then(|cap| cap.as_str().parse().ok());
                                    }
                                    _ => {}
                                },
                            }
                            ptr = i + 1;
                        }
                        _ => {}
                    }
                }
                output.push(Token::Eof(tks[ptr..tks.len()].to_string()));
                res = output;
            }

            res
        }

        fn render(&self) -> String {
            let ts = self.ser();
            let mut tks: Vec<String> = vec![];

            for i in ts {
                match i {
                    Token::Plain(s) => tks.push(s.normal().to_string()),
                    Token::Bold(s) => {
                        if s.contains("Example") {
                            let mut br = "-".repeat(50).dimmed().to_string();
                            br.push_str("\n\n");
                            tks.push(br);
                        } else if s.contains("Note") {
                            let mut br = "* ".repeat(25).dimmed().to_string();
                            br.push_str("\n\n");
                            tks.push(br);
                        }

                        tks.push(s.bold().to_string());
                    }
                    Token::Sup(s) => tks.push(match s.parse::<u8>() {
                        Ok(n) => superscript(n),
                        _ => s,
                    }),
                    Token::Sub(s) => tks.push(match s.parse::<u8>() {
                        Ok(n) => subscript(n),
                        _ => s,
                    }),
                    Token::Font((s, color)) => tks.push(s.color(color).to_string()),
                    Token::Eof(s) => tks.push(s.normal().to_string()),
                }
            }

            // post replace
            let tks = tks.join("");

            decode_html(&tks).unwrap_or(tks)
        }
    }
}

mod file {
    /// Convert file suffix from language type
    pub fn suffix(l: &str) -> Result<&'static str, crate::Error> {
        match l {
            "bash" => Ok("sh"),
            "c" => Ok("c"),
            "cpp" => Ok("cpp"),
            "csharp" => Ok("cs"),
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
            _ => Ok("c"),
        }
    }

    use crate::cache::models::Problem;
    /// Generate code path by fid
    pub fn code_path(target: &Problem, l: Option<String>) -> Result<String, crate::Error> {
        let conf = crate::cfg::locate()?;
        let mut lang = conf.code.lang;
        if l.is_some() {
            lang = l?;
        }

        let mut path = format!(
            "{}/{}.{}",
            conf.storage.code()?,
            conf.code.pick,
            suffix(&lang)?,
        );

        path = path.replace("${fid}", &target.fid.to_string());
        path = path.replace("${slug}", &target.slug.to_string());

        Ok(path)
    }

    /// Load python scripts
    pub fn load_script(module: &str) -> Result<String, crate::Error> {
        use std::fs::File;
        use std::io::Read;
        let conf = crate::cfg::locate()?;
        let mut script = "".to_string();
        File::open(format!("{}/{}.py", conf.storage.scripts()?, module))?
            .read_to_string(&mut script)?;

        Ok(script)
    }
}
