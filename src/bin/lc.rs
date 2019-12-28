use lc::plugins::chrome;

fn main() {
    let cs = chrome::cookies();
    println!("{:?}", cs);
}
