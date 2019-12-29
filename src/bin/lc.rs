use lc::plugins::chrome;

fn main() {
    let cookies = chrome::cookies();
    println!("{:#?}", cookies);
}
