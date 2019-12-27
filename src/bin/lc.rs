use lc::{info, error, warn};
use lc::api::API;
    
fn main() {
    let mut a = API::new();
    a.login();
}
