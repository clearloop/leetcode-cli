/// convert prints
pub trait Digit<T> {
    fn digit(self, e: T) -> String;
}

impl Digit<i32> for i32 {
    fn digit(self, e: i32) -> String {
        let mut s = self.to_string();
        let space = " ".repeat((e as usize) - s.len());
        s.push_str(&space);

        s
    }
}
