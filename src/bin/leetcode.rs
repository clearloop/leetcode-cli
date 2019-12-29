use leetcode_cli::plugins::leetcode;

fn main() {
    env_logger::init();
    
    let lc = leetcode::LeetCode::new();
    let mut res = lc.get_category_problems("database");
    println!("{:#?}", res.text());
}
