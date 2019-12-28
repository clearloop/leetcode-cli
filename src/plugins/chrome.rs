mod schema {
    table! {
        cookies (host_key) {
            name -> Text,
            encrypted_data -> Text,
            host_key -> Text,
        }
    }
}

mod models {
    #[derive(Queryable, Debug)]
    pub struct Cookies {
        name: String,
        encrypted_data: String,
        host_key: String
    }
}


use crate::store;
use diesel::prelude::*;
use models::*;

pub fn get_cookies() {
    use self::schema::cookies::dsl::*;
    
    let home = dirs::home_dir().unwrap();
    let p = match std::env::consts::OS {
        "macos" => home.join("Library/Application Support/Google/Chrome/Default/Cookies"),
        "windows" => {
            let mut appd = std::path::PathBuf::new();
            let dir = app_dirs::get_data_root(app_dirs::AppDataType::SharedData);
            if dir.is_ok() {
                appd = dir.unwrap();
            }
            
            appd.join("../Local/Google/Chrome/User Data/Default/Cookies")
        },
        _ => home.join(".config/google-chrome/Default/Cookies"),
    };
    
    let conn = store::conn(p.to_string_lossy().to_string());
    let res = cookies
        .select(host_key)
        .filter(host_key.like("%leetcode.com"))
        .limit(5)
        .load::<Cookies>(&conn)
        .expect("Error loading posts");

    println!("{:?}", res);
}
