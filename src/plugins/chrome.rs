use crate::store;
use crypto::{
    aes,
    pbkdf2,
    sha1::Sha1,
    hmac::Hmac,
    blockmodes::NoPadding,
    buffer::{
        RefReadBuffer,
        RefWriteBuffer,
    },
};
use diesel::prelude::*;
use keyring::Keyring;
use std::collections::HashMap;

/// LeetCode Cookies Schema
mod schema {
    table! {
        cookies (host_key) {
            encrypted_value -> Binary,
            host_key -> Text,
            name -> Text,
        }
    }
}

/// Fix LeetCode Cookie
type CookieValue = [u8; 1024];

/// Fix Chrome PBKDF2 key
type ChromeKey = [u8; 16];

/// Please make sure the order
/// 
/// The order between table and struct must be same.
#[derive(Queryable, Debug, Clone)]
pub struct Cookies {
    pub encrypted_value: Vec<u8>,
    pub host_key: String,
    pub name: String,
}

/// Get cookies from chrome storage
pub fn cookies() -> HashMap<String, Vec<u8>> {
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
        .filter(host_key.like("%leetcode.com"))
        .load::<Cookies>(&conn)
        .expect("Error loading cookies");

    let mut m: HashMap<String, Vec<u8>> = HashMap::new();
    for c in res.to_owned() {
        if (
            c.name == "csrftoken".to_string()
        ) || (
            c.name == "LEETCODE_SESSION".to_string()
        ) {
            m.insert(c.name, decode_cookies(c.encrypted_value));
        }
    }
    
    m
}


/// Decode cookies from chrome
fn decode_cookies(v: Vec<u8>) -> Vec<u8> {
    let ring = Keyring::new("Chrome Safe Storage", "Chrome");
    let pass = ring.get_password().expect("Get Password failed");
    let mut mac = Hmac::new(Sha1::new(), pass.as_bytes());
    let mut key: ChromeKey = [0_u8; 16];

    pbkdf2::pbkdf2(&mut mac, b"saltysalt", 1003, &mut key);
    chrome_decrypt(v, key)
}


/// Decrypt chrome cookie value with aes-128-cbc
fn chrome_decrypt(v: Vec<u8>, key: [u8;16]) -> Vec<u8> {
    let iv: ChromeKey = [0_u8; 16];
    let mut res: CookieValue = [0_u8; 1024];
    
    let mut decryptor = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        &key,
        &iv,
        NoPadding
    );
    
    let mut v_buf = RefReadBuffer::new(&v[3..]);
    let mut r_buf = RefWriteBuffer::new(&mut res);

    decryptor.decrypt(
        &mut v_buf, &mut r_buf, true
    ).expect("Decrypt Cookies with aes-128-cbc failed");

    std::str::from_utf8(&res)
        .expect("Failed to trime cookie bytes")
        .trim_matches(char::from(0))
        .as_bytes()
        .to_vec()
}
