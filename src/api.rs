use std::io::Result;
use std::collections::HashMap;
use sled::Db;
use reqwest::{
    Client,
    RequestBuilder,
    Response,
    header::{
        HeaderMap,
        HeaderValue,
    }
};
use colored::Colorize;
use crate::conf::Conf;
use crate::err::LcError;
use crate::{info, warn, error};

/// LeetCode API sets
pub struct API {
    conf: Conf,
    client: Client,
    db: Db
}

impl API {
    /// new API with http client and database.
    pub fn new() -> API {
        let home_dir = dirs::home_dir()
            .expect(&LcError::DirError.to_string());
        
        API {
            conf: Conf::default(),
            client: Client::builder()
                .gzip(true)
                .cookie_store(true)
                .build()
                .expect(&LcError::ClientBuildError.to_string()),
            db: sled::open(home_dir)
                .expect(&LcError::DirError.to_string())
        }
    }

    /// Login in Leetcode
    pub fn login(&mut self) -> Result<Response> {
        let conf_exp = LcError::ConfigError.to_string();
        let parse_exp = LcError::ParseCookieError.to_string();
        let req_exp = LcError::RequestError.to_string();
        
        info!("Get cookies...");
        let res = self.client.get(
            *self.conf.urls.get("login").expect(&conf_exp)
        ).send().expect(&req_exp);

        // get csrf
        let csrf = helper::parse_cookie(
            res.headers(), "csrftoken"
        ).expect(&parse_exp);

        // std::thread::sleep(std::time::Duration::from_millis(1000));
        
        // send a new post to login
        info!("Logining in leetcode.com...");
        let mut headers = HeaderMap::new();
        headers.insert(
            "content-type",
            HeaderValue::from_str(
                "multipart/form-data; boundary=----WebKitFormBoundaryKp0MpivQB8hk9lQA"
            ).expect(&parse_exp),
        );
        
        headers.insert(
            "Origin",
            HeaderValue::from_str(
                *self.conf.urls.get("base").expect(&req_exp)
            ).expect(&parse_exp),
        );

        headers.insert(
            "Referer",
            HeaderValue::from_str(
                *self.conf.urls.get("login").expect(&req_exp)
            ).expect(&parse_exp),
        );

        headers.insert(
            "Cookie",
            HeaderValue::from_str(
                &format!("csrftoken={};", csrf)
            ).expect(&parse_exp),
        );

        headers.insert(
            "x-csrftoken",
            HeaderValue::from_str(&csrf).expect(&parse_exp),
        );

        let mut params = HashMap::new();
        params.insert("csrfmiddlewaretoken", csrf);
        params.insert("login", "".to_string());
        params.insert("password", "".to_string());
        
        let req = self.client.post(
            *self.conf.urls.get("login").expect(&req_exp)
        ).headers(headers).form(&params);

        println!("{:#?}", &req);
        
        Ok(
            req
           .send()
           .expect(&req_exp)
        )
    }
}

mod helper {
    use std::io::Result;
    use std::collections::HashMap;
    use crate::err::LcError;
    use reqwest::header::HeaderMap;
    pub(super) fn parse_cookie(map: &HeaderMap, key: &'static str) -> Result<String> {
        let mut cookies = "".to_string();

        for (k, v) in map.iter() {
            if k == "set-cookie" {
                cookies.push_str(v.to_str().expect(&LcError::ParseCookieError.to_string()));
                cookies.push(';');
            }
        }

        let mut map: HashMap<String, String> = HashMap::new();
        let cks: Vec<&str> = cookies.split(';').collect();
        for i in cks {
            if i.find('=').is_none() {
                continue;
            }
            
            let kv: Vec<&str> = i.trim().split('=').collect();
            map.insert(kv[0].to_string(), kv[1].to_string());
        }

        Ok(map.get(key).expect(&LcError::ParseCookieError.to_string()).to_string())
    }
}
