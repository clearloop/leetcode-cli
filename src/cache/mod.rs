//! Save bad network\'s ass.
pub mod models;
pub mod parser;
pub mod schemas;
mod sql;
use self::models::*;
use self::schemas::{problems::dsl::*, tags::dsl::*};
use self::sql::*;
use crate::{cfg, err::Error, plugins::LeetCode};
use colored::Colorize;
use diesel::prelude::*;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::collections::HashMap;
use reqwest::Response;

/// sqlite connection
pub fn conn(p: String) -> SqliteConnection {
    SqliteConnection::establish(&p).unwrap_or_else(|_| panic!("Error connecting to {:?}", p))
}

/// Condition submit or test
#[derive(Clone, Debug)]
pub enum Run {
    Test,
    Submit,
}

impl std::default::Default for Run {
    fn default() -> Self {
        Run::Submit
    }
}

/// Requests if data not download
#[derive(Clone)]
pub struct Cache(pub LeetCode);

impl Cache {
    /// Ref to sqliteconnection
    fn conn(&self) -> Result<SqliteConnection, Error> {
        Ok(conn(self.0.conf.storage.cache()?))
    }

    /// Clean cache
    pub fn clean(&self) -> Result<(), Error> {
        Ok(std::fs::remove_file(&self.0.conf.storage.cache()?)?)
    }

    /// ref to download probems
    pub async fn update(self) -> Result<(), Error> {
        self.download_problems().await?;
        Ok(())
    }

    pub fn update_after_ac(self, rid: i32) -> Result<(), Error> {
        let c = conn((&self.0.conf.storage.cache()?).to_owned());
        let target = problems.filter(id.eq(rid));
        diesel::update(target).set(status.eq("ac")).execute(&c)?;
        Ok(())
    }

    async fn get_user_info(&self) -> Result<(String,bool), Error> {
        let user = parser::user(
            self.0
            .get_user_info().await?
            .json().await?
        );
        match user {
            None => Err(Error::NoneError),
            Some(None) => Err(Error::CookieError),
            Some(Some((s,b))) => Ok((s,b))
        }
    }

    async fn is_session_bad(&self) -> bool {
        matches!(self.get_user_info().await, Err(Error::CookieError))
    }

    async fn resp_to_json<T: DeserializeOwned>(&self, resp: Response) -> Result<T, Error> {
        let maybe_json: Result<T,_> = resp.json().await;
        if maybe_json.is_err() && self.is_session_bad().await {
            Err(Error::CookieError)
        } else { Ok(maybe_json?) }
    }

    /// Download leetcode problems to db
    pub async fn download_problems(self) -> Result<Vec<Problem>, Error> {
        info!("Fetching leetcode problems...");
        let mut ps: Vec<Problem> = vec![];

        for i in &self.0.conf.sys.categories.to_owned() {
            let json = self
                .0
                .get_category_problems(i)
                .await?
                .json() // does not require LEETCODE_SESSION
                .await?;
            parser::problem(&mut ps, json).ok_or(Error::NoneError)?;
        }

        diesel::replace_into(problems)
            .values(&ps)
            .execute(&self.conn()?)?;

        Ok(ps)
    }

    // TODO: get rid of this
    pub fn push_problem(&self, p: Problem) -> Result<(), Error> {
        diesel::replace_into(problems)
            .values(&vec![p])
            .execute(&self.conn()?)?;
        Ok(())
    }

    /// TODO: implement caching
    /// Get contest 
    pub async fn get_contest(&self, contest: &str) -> Result<Contest, Error> {
        let ctest = self.0
            .get_contest_info(contest)
            .await?
            .json()
            .await?;
        let ctest = parser::contest(ctest).ok_or(Error::NoneError)?;
        Ok(ctest)
    }

    /// Get problem
    pub fn get_problem(&self, rfid: i32) -> Result<Problem, Error> {
        let p: Problem = problems.filter(fid.eq(rfid)).first(&self.conn()?)?;
        if p.category != "algorithms" && p.category != "contest" {
            return Err(Error::FeatureError(
                "Not support database and shell questions for now".to_string(),
            ));
        }

        Ok(p)
    }

    /// Get daily problem 
    pub async fn get_daily_problem_id(&self) -> Result<i32, Error> {
        parser::daily(
            self.0
                .get_question_daily()
                .await?
                .json() // does not require LEETCODE_SESSION
                .await?
        ).ok_or(Error::NoneError)
    }

    /// Get problems from cache
    pub fn get_problems(&self) -> Result<Vec<Problem>, Error> {
        Ok(problems.load::<Problem>(&self.conn()?)?)
    }

    /// Get question
    #[allow(clippy::useless_let_if_seq)]
    pub async fn get_question(&self, rfid: i32) -> Result<Question, Error> {
        let target: Problem = problems.filter(fid.eq(rfid)).first(&self.conn()?)?;

        let ids = match target.level {
            1 => target.fid.to_string().green(),
            2 => target.fid.to_string().yellow(),
            3 => target.fid.to_string().red(),
            _ => target.fid.to_string().dimmed(),
        };

        println!(
            "\n[{}] {} {}\n\n",
            &ids,
            &target.name.bold().underline(),
            "is on the run...".dimmed()
        );

        if target.category != "algorithms" {
            return Err(Error::FeatureError(
                "Not support database and shell questions for now".to_string(),
            ));
        }

        let mut rdesc = Question::default();
        if !target.desc.is_empty() {
            rdesc = serde_json::from_str(&target.desc)?;
        } else {
            let json: Value = self
                .0
                .get_question_detail(&target.slug)
                .await?
                .json()
                .await?;
            debug!("{:#?}", &json);
            match parser::desc(&mut rdesc, json) {
                None => return Err(Error::NoneError),
                Some(false) => return
                    if self.is_session_bad().await {
                        Err(Error::CookieError)
                    } else {
                        Err(Error::PremiumError)
                    },
                Some(true) => ()
            }

            // update the question
            let sdesc = serde_json::to_string(&rdesc)?;
            diesel::update(&target)
                .set(desc.eq(sdesc))
                .execute(&self.conn()?)?;
        }

        Ok(rdesc)
    }

    // TODO: we can probably use this for all questions in general, actually
    /// Get contest question 
    pub async fn get_contest_qnp(&self, problem: &str) -> Result<(Problem,Question), Error> {
        let graphql_res = self.0
            .get_contest_question_detail(problem)
            .await?
            .json()
            .await?;
        parser::graphql_problem_and_question(graphql_res).ok_or(Error::NoneError)
    }

    pub async fn get_tagged_questions(self, rslug: &str) -> Result<Vec<String>, Error> {
        trace!("Geting {} questions...", &rslug);
        let ids: Vec<String>;
        let rtag = tags
            .filter(tag.eq(rslug.to_string()))
            .first::<Tag>(&self.conn()?);
        if let Ok(t) = rtag {
            trace!("Got {} questions from local cache...", &rslug);
            ids = serde_json::from_str(&t.refs)?;
        } else {
            ids = parser::tags(
                self.0
                    .get_question_ids_by_tag(rslug)
                    .await?
                    .json()
                    .await?,
            ).ok_or(Error::NoneError)?;
            let t = Tag {
                r#tag: rslug.to_string(),
                r#refs: serde_json::to_string(&ids)?,
            };

            diesel::insert_into(tags)
                .values(&t)
                .execute(&self.conn()?)?;
        }

        Ok(ids)
    }

    pub fn get_tags(&self) -> Result<Vec<Tag>, Error> {
        Ok(tags.load::<Tag>(&self.conn()?)?)
    }

    /// run_code data
    async fn pre_run_code(
        &self,
        run: Run,
        rfid: i32,
        testcase: Option<String>,
        contest: Option<&str>
    ) -> Result<(HashMap<&'static str, String>, [String; 2]), Error> {
        trace!("pre run code...");
        use crate::helper::{code_path, test_cases_path};
        use std::fs::File;
        use std::io::Read;

        let mut p = self.get_problem(rfid)?;
        if p.desc.is_empty() {
            trace!("Problem description does not exist, pull desc and exec again...");
            self.get_question(rfid).await?;
            p = self.get_problem(rfid)?;
        }

        let d: Question = serde_json::from_str(&p.desc)?;
        let conf = &self.0.conf;
        let mut json: HashMap<&'static str, String> = HashMap::new();
        let mut code: String = "".to_string();

        let maybe_file_testcases: Option<String> = test_cases_path(&p)
                .map(|filename| {
                    let mut tests = "".to_string();
                    File::open(filename)
                        .and_then(|mut file_descriptor| file_descriptor.read_to_string(&mut tests))
                        .map(|_| Some(tests))
                        .unwrap_or(None)
                })
                .unwrap_or(None);

        // Takes test cases using following priority
        // 1. cli parameter
        // 2. test cases from the file
        // 3. sample test case from the task
        let testcase = testcase
            .or(maybe_file_testcases)
            .unwrap_or(d.case);

        File::open(code_path(&p, None)?)?.read_to_string(&mut code)?;

        json.insert("lang", conf.code.lang.to_string());
        json.insert("question_id", p.id.to_string());
        json.insert("typed_code", code);

        // pass manually data
        json.insert("name", p.name.to_string());
        json.insert("data_input", testcase);

        // TODO: make this less ugly
        let make_url = |s: &str| {
            if let Some(c) = contest {
                let s = format!("{}_contest", s);
                conf.sys.urls.get(&s).map(|u| u.replace("$contest", c))
            } else {
                conf.sys.urls.get(s).map(|u| u.to_owned())
            }.ok_or(Error::NoneError)
        };

        let url = match run {
            Run::Test => make_url("test")?.replace("$slug", &p.slug),
            Run::Submit => {
                json.insert("judge_type", "large".to_string());
                make_url("submit")?.replace("$slug", &p.slug)
            }
        };

        Ok((
            json,
            [
                url,
                conf.sys.urls.get("problems").ok_or(Error::NoneError)?.replace("$slug", &p.slug),
            ],
        ))
    }

    /// TODO: The real delay
    async fn recur_verify(&self, rid: String) -> Result<VerifyResult, Error> {
        use std::time::Duration;

        trace!("Run veriy recursion...");
        std::thread::sleep(Duration::from_micros(3000));

        let json: VerifyResult = self.resp_to_json(
            self
            .0
            .verify_result(rid.clone())
            .await?
        ).await?;

        Ok(json)
    }

    /// Exec problem filter —— Test or Submit
    pub async fn exec_problem(
        &self,
        rfid: i32,
        run: Run,
        testcase: Option<String>,
        contest: Option<&str>
    ) -> Result<VerifyResult, Error> {
        trace!("Exec problem filter —— Test or Submit");
        let (json, [url, refer]) = self.pre_run_code(run.clone(), rfid, testcase, contest).await?;
        trace!("Pre run code result {:?}, {:?}, {:?}", json, url, refer);

        let run_res: RunCode = self
            .0
            .run_code(json.clone(), url.clone(), refer.clone())
            .await?
            .json() // does not require LEETCODE_SESSION (very oddly)
            .await?;
        trace!("Run code result {:#?}", run_res);

        // Check if leetcode accepted the Run request
        if match run {
            Run::Test => run_res.interpret_id.is_empty(),
            Run::Submit => run_res.submission_id == 0
        } { return Err(Error::CookieError) }

        let mut res: VerifyResult = VerifyResult::default();
        while res.state != "SUCCESS" {
            res = match run {
                Run::Test => self.recur_verify(run_res.interpret_id.clone()).await?,
                Run::Submit => self.recur_verify(run_res.submission_id.to_string()).await?,
            };
        }
        trace!("Recur verify result {:#?}", res);

        res.name = json.get("name").ok_or(Error::NoneError)?.to_string();
        res.data_input = json.get("data_input").ok_or(Error::NoneError)?.to_string();
        res.result_type = run;
        Ok(res)
    }

    /// New cache
    pub fn new() -> Result<Self, Error> {
        let conf = cfg::locate()?;
        let c = conn(conf.storage.cache()?);
        diesel::sql_query(CREATE_PROBLEMS_IF_NOT_EXISTS).execute(&c)?;
        diesel::sql_query(CREATE_TAGS_IF_NOT_EXISTS).execute(&c)?;

        Ok(Cache(LeetCode::new()?))
    }
}
