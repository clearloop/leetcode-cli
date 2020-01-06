#[macro_use]
extern crate log;

#[test]
fn test_tag_graphql() {
    use leetcode_cli::plugins::LeetCode;
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("leetcode"));
    let l = LeetCode::new().unwrap();

    let r = l.get_question_ids_by_tag("linked-list")?.text()?;
    error!("{:#?}", &r);
    assert!(r.is_ok());
}
