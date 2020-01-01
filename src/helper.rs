//! Convert i32 to specific digits string.
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
