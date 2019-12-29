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

    /// **Deprecated** API: 400 Error
    #[allow(dead_code)]
    fn get_user_info(self) -> Option<Response> {
        debug!("running leetcode.get_user_info...");
        let mut m: HashMap<&str, &str> = HashMap::new();
        m.insert(
            "body",
            r#"{
  "query": "{
     user {
       username,
       isCurrentUserPremium
     }
   }",
  "variables": {}
}"#
        );

        let headers = LeetCode::headers(
            self.default_headers,
            vec![("Referer", &self.conf.sys.urls["graphql"])],
        );
        
        let req = self.client
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
