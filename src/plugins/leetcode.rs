use self::req::{Json, Mode, Req};
use crate::{
    cfg::{self, Config},
    err::Error,
    plugins::chrome,
};
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, ClientBuilder, Response,
};
use std::{collections::HashMap, str::FromStr, time::Duration};
use ::function_name::named;

/// LeetCode API set
#[derive(Clone)]
pub struct LeetCode {
    pub conf: Config,
    client: Client,
    default_headers: HeaderMap,
}

impl LeetCode {
    /// Parse reqwest headers
    fn headers(mut headers: HeaderMap, ts: Vec<(&str, &str)>) -> Result<HeaderMap, Error> {
        for (k, v) in ts.into_iter() {
            let name = HeaderName::from_str(k);
            let value = HeaderValue::from_str(v);
            if name.is_err() || value.is_err() {
                return Err(Error::ParseError("http header parse failed".to_string()));
            }

            headers.insert(name.unwrap(), value.unwrap());
        }

        Ok(headers)
    }

    /// New LeetCode client
    pub fn new() -> Result<LeetCode, crate::Error> {
        let conf = cfg::locate()?;
        let cookies = chrome::cookies()?;
        let default_headers = LeetCode::headers(
            HeaderMap::new(),
            vec![
                ("Cookie", cookies.to_string().as_str()),
                ("x-csrftoken", &cookies.csrf),
                ("x-requested-with", "XMLHttpRequest"),
                ("Origin", &conf.sys.urls["base"]),
            ],
        )?;

        let client = ClientBuilder::new()
            .gzip(true)
            .connect_timeout(Duration::from_secs(30))
            .build()?;

        // Sync conf
        if conf.cookies.csrf != cookies.csrf {
            conf.sync()?;
        }

        Ok(LeetCode {
            conf,
            client,
            default_headers,
        })
    }

    /// Get category problems
    #[named]
    pub async fn get_category_problems(self, category: &str) -> Result<Response, Error> {
        trace!("Requesting {} problems...", &category);
        let url = &self
            .conf
            .sys
            .urls
            .get("problems").ok_or(Error::NoneError)?
            .replace("$category", category);

        Req {
            default_headers: self.default_headers,
            refer: None,
            json: None,
            mode: Mode::Get,
            name: function_name!(),
            url: url.to_string(),
        }
        .send(&self.client)
        .await
    }

    #[named]
    pub async fn get_question_ids_by_tag(self, slug: &str) -> Result<Response, Error> {
        trace!("Requesting {} ref problems...", &slug);
        let url = &self.conf.sys.urls.get("graphql").ok_or(Error::NoneError)?;
        let mut json: Json = HashMap::new();
        json.insert("operationName", "getTopicTag".to_string());
        json.insert("variables", r#"{"slug": "$slug"}"#.replace("$slug", slug));
        json.insert(
            "query",
            "query getTopicTag($slug: String!) {
               topicTag(slug: $slug) {
                 questions {
                  questionId
                 }
               }
             }".to_owned()
        );

        Req {
            default_headers: self.default_headers,
            refer: Some((self.conf.sys.urls.get("tag").ok_or(Error::NoneError)?).replace("$slug", slug)),
            json: Some(json),
            mode: Mode::Post,
            name: function_name!(),
            url: (*url).to_string(),
        }
        .send(&self.client)
        .await
    }

    #[named]
    pub async fn get_user_info(self) -> Result<Response, Error> {
        trace!("Requesting user info...");
        let url = &self.conf.sys.urls.get("graphql").ok_or(Error::NoneError)?;
        let mut json: Json = HashMap::new();
        json.insert("operationName", "a".to_string());
        json.insert(
            "query",
            "query a {
                 user {
                     username
                     isCurrentUserPremium
                 }
             }".to_owned()
        );

        Req {
            default_headers: self.default_headers,
            refer: None,
            json: Some(json),
            mode: Mode::Post,
            name: function_name!(),
            url: (*url).to_string(),
        }
        .send(&self.client)
        .await
    }

    /// Get daily problem
    #[named]
    pub async fn get_question_daily(self) -> Result<Response, Error> {
        trace!("Requesting daily problem...");
        let url = &self.conf.sys.urls.get("graphql").ok_or(Error::NoneError)?;
        let mut json: Json = HashMap::new();
        json.insert("operationName", "daily".to_string());
        json.insert(
            "query",
            "query daily {
               activeDailyCodingChallengeQuestion {
                 question {
                   questionFrontendId
                 }
               }
             }".to_owned()
        );

        Req {
            default_headers: self.default_headers,
            refer: None,
            json: Some(json),
            mode: Mode::Post,
            name: function_name!(),
            url: (*url).to_string(),
        }
        .send(&self.client)
        .await
    }

    /// Get specific problem detail
    #[named]
    pub async fn get_question_detail(self, slug: &str) -> Result<Response, Error> {
        trace!("Requesting {} detail...", &slug);
        let refer = self.conf.sys.urls.get("problems").ok_or(Error::NoneError)?.replace("$slug", slug);
        let mut json: Json = HashMap::new();
        json.insert(
            "query",
            "query getQuestionDetail($titleSlug: String!) {
               question(titleSlug: $titleSlug) {
                 content
                 stats
                 codeDefinition
                 sampleTestCase
                 exampleTestcases
                 enableRunCode
                 metaData
                 translatedContent
               }
             }".to_owned()
        );

        json.insert(
            "variables",
            r#"{"titleSlug": "$titleSlug"}"#.replace("$titleSlug", slug),
        );

        json.insert("operationName", "getQuestionDetail".to_string());

        Req {
            default_headers: self.default_headers,
            refer: Some(refer),
            json: Some(json),
            mode: Mode::Post,
            name: function_name!(),
            url: (&self.conf.sys.urls["graphql"]).to_string(),
        }
        .send(&self.client)
        .await
    }

    /// Send code to judge
    #[named]
    pub async fn run_code(self, j: Json, url: String, refer: String) -> Result<Response, Error> {
        info!("Sending code to judge...");
        Req {
            default_headers: self.default_headers,
            refer: Some(refer),
            json: Some(j),
            mode: Mode::Post,
            name: function_name!(),
            url,
        }
        .send(&self.client)
        .await
    }

    /// Get the result of submission / testing
    #[named]
    pub async fn verify_result(self, id: String) -> Result<Response, Error> {
        trace!("Verifying result...");
        let url = self.conf.sys.urls.get("verify").ok_or(Error::NoneError)?.replace("$id", &id);
        Req {
            default_headers: self.default_headers,
            refer: None,
            json: None,
            mode: Mode::Get,
            name: function_name!(),
            url,
        }
        .send(&self.client)
        .await
    }
}

/// Sub-module for leetcode, simplify requests
mod req {
    use super::LeetCode;
    use crate::err::Error;
    use reqwest::{header::HeaderMap, Client, Response};
    use std::collections::HashMap;

    /// Standardize json format
    pub type Json = HashMap<&'static str, String>;

    /// Standardize request mode
    pub enum Mode {
        Get,
        Post,
    }

    /// LeetCode request prototype
    pub struct Req {
        pub default_headers: HeaderMap,
        pub refer: Option<String>,
        pub json: Option<Json>,
        pub mode: Mode,
        pub name: &'static str,
        pub url: String,
    }

    impl Req {
        pub async fn send(self, client: &Client) -> Result<Response, Error> {
            trace!("Running leetcode::{}...", &self.name);
            let url = self.url.to_owned();
            let headers = LeetCode::headers(
                self.default_headers,
                vec![("Referer", &self.refer.unwrap_or(url))],
            )?;

            let req = match self.mode {
                Mode::Get => client.get(&self.url),
                Mode::Post => client.post(&self.url).json(&self.json),
            };

            Ok(req.headers(headers).send().await?)
        }
    }
}
