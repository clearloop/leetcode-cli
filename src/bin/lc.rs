use lc::plugins::leetcode;

fn main() {
    env_logger::init();
    
    let lc = leetcode::LeetCode::new();
    if let Some(mut res) = lc.get_user_info() {
        println!("{:#?}", res.text());
    }
}
