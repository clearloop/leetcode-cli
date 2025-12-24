use crate::{Error, Result, cache};
use anyhow::anyhow;
use diesel::prelude::*;
use keyring::Entry;
use openssl::{hash, pkcs5, symm};
use std::{collections::HashMap, fmt::Display};

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
    #[allow(dead_code)]
    pub host_key: String,
    pub name: String,
}

/// Spawn cookies to cookie format
#[derive(Debug)]
pub struct Ident {
    pub csrf: String,
    session: String,
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LEETCODE_SESSION={};csrftoken={};",
            self.session, self.csrf
        )
    }
}

/// Get cookies from chrome storage
pub fn cookies() -> Result<Ident> {
    let ccfg = crate::config::Config::locate()?.cookies;
    if !ccfg.csrf.is_empty() && !ccfg.session.is_empty() {
        return Ok(Ident {
            csrf: ccfg.csrf,
            session: ccfg.session,
        });
    }

    // If doesn't config SESSION and csrftoken
    use self::schema::cookies::dsl::*;
    trace!("Derive cookies from google chrome...");

    let home = dirs::home_dir().ok_or(Error::NoneError)?;
    let p = match std::env::consts::OS {
        "macos" => home.join("Library/Application Support/Google/Chrome/Default/Cookies"),
        "linux" => home.join(".config/google-chrome/Default/Cookies"),
        _ => panic!("Opps...only works on OSX or Linux now..."),
    };

    debug!("Chrome Cookies path is {:?}", &p);
    let mut conn = cache::conn(p.to_string_lossy().to_string());
    let res = cookies
        .filter(host_key.like(format!("#{}", ccfg.site)))
        .load::<Cookies>(&mut conn)
        .expect("Loading cookies from google chrome failed.");

    debug!("res {:?}", &res);
    if res.is_empty() {
        return Err(Error::CookieError);
    }

    // Get system password
    let ring = Entry::new("Chrome Safe Storage", "Chrome")?;
    let pass = ring.get_password().expect("Get Password failed");

    // Decode cookies
    let mut m: HashMap<String, String> = HashMap::new();
    for c in res {
        if (c.name == "csrftoken") || (c.name == "LEETCODE_SESSION") {
            m.insert(c.name, decode_cookies(&pass, c.encrypted_value)?);
        }
    }

    Ok(Ident {
        csrf: m.get("csrftoken").ok_or(Error::ChromeNotLogin)?.to_string(),
        session: m
            .get("LEETCODE_SESSION")
            .ok_or(Error::ChromeNotLogin)?
            .to_string(),
    })
}

/// Decode cookies from chrome
fn decode_cookies(pass: &str, v: Vec<u8>) -> Result<String> {
    let mut key = [0_u8; 16];
    match std::env::consts::OS {
        "macos" => {
            pkcs5::pbkdf2_hmac(
                pass.as_bytes(),
                b"saltysalt",
                1003,
                hash::MessageDigest::sha1(),
                &mut key,
            )
            .expect("pbkdf2 hmac went error.");
        }
        "linux" => {
            pkcs5::pbkdf2_hmac(
                b"peanuts",
                b"saltysalt",
                1,
                hash::MessageDigest::sha1(),
                &mut key,
            )
            .expect("pbkdf2 hmac went error.");
        }
        _ => return Err(anyhow!("only supports OSX or Linux for now").into()),
    }

    chrome_decrypt(v, key)
}

/// Decrypt chrome cookie value with aes-128-cbc
fn chrome_decrypt(v: Vec<u8>, key: [u8; 16]) -> Result<String> {
    // <space>: \u16
    let iv = vec![32_u8; 16];
    let mut decrypter = symm::Crypter::new(
        symm::Cipher::aes_128_cbc(),
        symm::Mode::Decrypt,
        &key,
        Some(&iv),
    )?;

    let data_len = v.len() - 3;
    let block_size = symm::Cipher::aes_128_cbc().block_size();
    let mut plaintext = vec![0; data_len + block_size];

    decrypter.pad(false);

    let count = decrypter.update(&v[3..], &mut plaintext)?;
    decrypter.finalize(&mut plaintext[count..])?;
    plaintext.retain(|x| x >= &20_u8);

    Ok(String::from_utf8_lossy(&plaintext.to_vec()).to_string())
}
