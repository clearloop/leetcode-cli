#[macro_use]
extern crate diesel;
use diesel::prelude::*;

mod schema {
    table! {
        cookies (host_key) {
            host_key -> Text,
            name -> Text,
            encrypted_value -> Binary,
        }
    }
}

#[derive(Queryable, Debug)]
pub struct Cookies {
    pub host_key: String,
    pub name: String,
    pub encrypted_value: Vec<u8>,
}

pub fn establish_connection() -> SqliteConnection {
    let home = dirs::home_dir().unwrap();
    let p = home
        .join("Library/Application Support/Google/Chrome/Default/Cookies")
        .to_string_lossy().to_string();
    
    SqliteConnection::establish(&p)
        .unwrap_or_else(|_| panic!("Error connecting to {}", p))
}


fn main() {
    use schema::cookies::dsl::*;
    
    let connection = establish_connection();
    let results = cookies
        .filter(host_key.like("%leetcode.com"))
        .load::<Cookies>(&connection)
        .expect("Error loading cookies");

    println!("Displaying {} cookies", results.len());
    for post in results {
        println!("{:?}", post.encrypted_valueh);
    }
}
