use lc::api::API;
    
fn main() {
    let mut a = API::new();
    println!("{:#?}", a.login().unwrap());
}
