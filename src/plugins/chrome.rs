use crate::store;
use diesel::prelude::*;
use keyring::Keyring;
use openssl::{
    hash,
    pkcs5,
    symm
};
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
pub fn cookies() -> HashMap<String, String> {
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

    let mut m: HashMap<String, String> = HashMap::new();
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
fn decode_cookies(v: Vec<u8>) -> String {
    let ring = Keyring::new("Chrome Safe Storage", "Chrome");
    let pass = ring.get_password().expect("Get Password failed");

    let mut key = [0_u8; 16];
    pkcs5::pbkdf2_hmac(
        &pass.as_bytes(),
        b"saltysalt",
        1003,
        hash::MessageDigest::sha1(),
        &mut key
    ).expect("pbkdf2 hmac went error.");
    
    chrome_decrypt(v, key)
}


/// Decrypt chrome cookie value with aes-128-cbc
fn chrome_decrypt(v: Vec<u8>, key: [u8;16]) -> String {
    // <space>: \u16
    let iv = vec![32_u8; 16];
    let mut decrypter = symm::Crypter::new(
        symm::Cipher::aes_128_cbc(),
        symm::Mode::Decrypt,
        &key,
        Some(&iv)
    ).unwrap();

    
    let data_len = v.len() - 3;
    let block_size = symm::Cipher::aes_128_cbc().block_size();
    let mut plaintext = vec![0; data_len + block_size];
    
    decrypter.pad(false);

    let mut count = decrypter.update(&v[3..], &mut plaintext).unwrap();
    count += decrypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.truncate(count - block_size);
    
    String::from_utf8_lossy(&plaintext.to_vec()).to_string()
}
