//! A set of helper traits
pub use self::digit::Digit;
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

/// Render html to command-line
mod html {
    // use crate::Error;
    use colored::Colorize;
    pub enum Token {
        Plain(String),
        Bold(String),
        Eof(String)
    }

    /// html render plugin
    pub trait HTML {
        fn ser(&self) -> Vec<Token>;
        fn render(&self) -> String;
    }

    impl HTML for String {
        fn ser(&self) -> Vec<Token> {
            // empty tags
            let mut tks = self.to_string();
            
            // converting symbols
            tks = tks.replace(r#"&lt;"#, "<");
            tks = tks.replace(r#"&gt;"#, ">");
            tks = tks.replace(r#"&amp;"#, "&");
            tks = tks.replace(r#"&quot;"#, "\"");
            tks = tks.replace(r#"&nbsp;"#, " ");

            let res: Vec<Token>;
            // styled
            {
                let mut ptr = 0;
                let mut output = vec![];
                let mut bold = false;
                for (i, e) in tks.chars().enumerate() {
                    match e {
                        '<' => {
                            match bold {
                                true => {
                                    output.push(Token::Bold(tks[ptr..i].to_string()));
                                    bold = false;
                                }
                                false => output.push(
                                    Token::Plain(tks[ptr..i].to_string())
                                ),
                            }
                            ptr = i;
                        },
                        '>' => {
                            match &tks[i-1..i] {
                                "-" => continue,
                                _ => match &tks[(ptr + 1)..i] {
                                    "b" | "strong" => bold = true,
                                    _ => {},
                                },
                            }
                            ptr = i + 1;
                        },
                        _ => {},
                    }
                };
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
                        }
                        tks.push(s.bold().to_string());
                    }
                    Token::Eof(s) => tks.push(s.normal().to_string()),
                }
            }

            tks.join("")
        }
    }
}
