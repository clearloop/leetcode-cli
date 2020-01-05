use crate::cache;
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
struct Cookies {
    pub encrypted_value: Vec<u8>,
    pub host_key: String,
    pub name: String,
}

/// Spawn cookies to cookie format
#[derive(Debug)]
pub struct Ident {
    pub csrf: String,
    session: String,
}

impl std::string::ToString for Ident {
    fn to_string(&self) -> String {
        format!(
            "LEETCODE_SESSION={};csrftoken={};",
            self.session,
            self.csrf
        ).to_string()
    }
}

/// Get cookies from chrome storage
pub fn cookies() -> Result<Ident, crate::Error> {
    use self::schema::cookies::dsl::*;
    debug!("Derive cookies from google chrome...");
    
    let home = dirs::home_dir()?;
    let p = match std::env::consts::OS {
        "macos" => home.join("Library/Application Support/Google/Chrome/Default/Cookies"),
        "linux" => home.join(".config/google-chrome/Default/Cookies"),
        _ => panic!("Opps...only works on OSX or Linux now...")
    };
    
    let conn = cache::conn(p.to_string_lossy().to_string());
    let res = cookies
        .filter(host_key.like("%leetcode.com"))
        .load::<Cookies>(&conn)
        .expect("Loading cookies from google chrome failed.");

    if &res.len() == &(0 as usize) {
        return Err(crate::Error::CookieError);
    }
    
    // Get system password
    let ring = Keyring::new("Chrome Safe Storage", "Chrome");
    let pass = ring.get_password().expect("Get Password failed");

    // Decode cookies
    let mut m: HashMap<String, String> = HashMap::new();
    for c in res.to_owned() {
        if (
            c.name == "csrftoken".to_string()
        ) || (
            c.name == "LEETCODE_SESSION".to_string()
        ) {
            m.insert(c.name, decode_cookies(&pass, c.encrypted_value));
        }
    }
    
    Ok(Ident {
        csrf: m.get("csrftoken")?.to_string(),
        session: m.get("LEETCODE_SESSION")?.to_string(),
    })
}


/// Decode cookies from chrome
fn decode_cookies(pass: &str, v: Vec<u8>) -> String {
    let mut key = [0_u8; 16];
    match std::env::consts::OS {
        "macos" => {
            pkcs5::pbkdf2_hmac(
                pass.as_bytes(),
                b"saltysalt",
                1003,
                hash::MessageDigest::sha1(),
                &mut key
            ).expect("pbkdf2 hmac went error.");
        },
        "linux" => {
            pkcs5::pbkdf2_hmac(
                b"peanuts",
                b"saltysalt",
                1,
                hash::MessageDigest::sha1(),
                &mut key
            ).expect("pbkdf2 hmac went error.");
        },
        _ => panic!("Opps...only works on OSX or Linux now...")
    }
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

    let count = decrypter.update(&v[3..], &mut plaintext).unwrap();
    decrypter.finalize(&mut plaintext[count..]).unwrap();
    plaintext.retain(|x| x >= &20_u8);

    String::from_utf8_lossy(&plaintext.to_vec()).to_string()
}
