use std::io::Result;
use sled::Db;
use reqwest::Client;
use colored::Colorize;
use crate::conf::Conf;
use crate::err::LcError;
use crate::{info, warn, error};

pub struct API {
    conf: Conf,
    client: Client,
    db: Db
}

impl API {
    pub fn new() -> API {
        let home_dir = dirs::home_dir()
            .expect(&LcError::DirError.to_string());
        
        
        API {
            conf: Conf::default(),
            client: Client::builder()
                .cookie_store(true)
                .build()
                .expect(&LcError::ClientBuildError.to_string()),
            db: sled::open(home_dir)
                .expect(&LcError::DirError.to_string())
        }
    }

    pub fn login(&mut self) -> Result<()> {
        info!("logining in leetcode.com...");
        let res = self.client.get(
            *self.conf.urls.get("login").expect(&LcError::ConfigError.to_string())
        ).send().expect(&LcError::RequestError.to_string());
        
        let csrf = helper::parse_cookie(&self.db, res.headers());
        
        println!("{:#?}", res.headers());
        Ok(())
    }
}

mod helper {
    use sled::Db;
    use std::io::Result;
    use reqwest::header::HeaderMap;
    pub(super) fn parse_cookie(db: &Db, map: &HeaderMap) -> Result<()> {
        unimplemented!();
    }
}
