//! A set of helper traits
pub use self::digit::Digit;

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
    use crate::Error;
    
    pub enum Token {
        Plain(&'static str),
        Tag(&'static str),
    }

    /// html render plugin
    ///
    /// ## tag maps
    /// + <b> -> bold
    /// + <em> -> italic
    /// + <u> -> underline
    /// + <>
    pub trait HTML<T> {
        fn ser() -> Vec<Token>;
        fn render() -> Result<(), Error>;
    }
}
