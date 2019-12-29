use std::{
    collections::HashMap,
    str::FromStr,
    time::Duration,
};

use reqwest::{
    Client,
    ClientBuilder,
    Response,
    header::{
        HeaderMap,
        HeaderName,
        HeaderValue,
    }
};

use crate::{
    plugins::chrome,
    conf::{self, Config},
};

/// Standardize leetcode api response
pub type LeetCodeResp = Option<Response>;

/// Leet API set
pub struct LeetCode {
    conf: Config,
    client: Client,
    default_headers: HeaderMap,
}

impl LeetCode {
    fn headers(mut headers: HeaderMap, ts: Vec<(&str, &str)>) -> HeaderMap {
        for (k, v) in ts.into_iter() {
            headers.insert(
                HeaderName::from_str(k).unwrap(),
                HeaderValue::from_str(v).unwrap(),
            );
        }

        headers
    }
    
    pub fn new() -> LeetCode {
        debug!("building reqwest client...");
        
        let conf = conf::locate();
        let cookies = chrome::cookies();
        let default_headers = LeetCode::headers(
            HeaderMap::new(),
            vec![
                ("Cookie", cookies.to_string().as_str()),
                ("x-csrftoken", &cookies.csrf),
                ("x-requested-with", "XMLHttpRequest"),
                ("Origin", &conf.sys.urls["base"])
            ],
        );
        
        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .cookie_store(true)
            .build()
            .expect("Reqwest client build failed");

        LeetCode {
            conf,
            client,
            default_headers,
        }
    }

    pub fn get_favorites(self) -> LeetCodeResp {
        debug!("running leetcode.get_category_problems...");
        let url = &self.conf.sys.urls["favorites"];
        let headers = LeetCode::headers(
            self.default_headers,
            vec![("Referer", url)],
        );

        let req = self.client
            .get(url)
            .headers(headers);

        println!("{:#?}", &req);
        let res = req
            .send();

        match res.is_err() {
            true => {
                error!("get_favorites request failed.");
                None
            }
            _ => Some(res.unwrap())
        }
    }
    
    pub fn get_user_info(self) -> Option<Response> {
        debug!("running leetcode.get_user_info...");
        let headers = LeetCode::headers(
            self.default_headers,
            vec![("Referer", &self.conf.sys.urls["graphql"])],
        );
        
        let mut m: HashMap<&str, &str> = HashMap::new();
        m.insert(
            "query",
            r#"{
              user {
                username,
                isCurrentUserPremium
              },
            }"#,
        );

        let res = self.client
            .post(&self.conf.sys.urls["graphql"])
            .headers(headers)
            .json(&m)
            .send();

        match res.is_err() {
            true => {
                error!("get_user_info request failed.");
                None
            }
            _ => Some(res.unwrap())
        }
    }
}
