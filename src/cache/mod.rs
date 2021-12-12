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
use serde_json::Value;
use std::collections::HashMap;

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

    /// Download leetcode problems to db
    pub async fn download_problems(self) -> Result<Vec<Problem>, Error> {
        info!("Fetching leetcode problems...");
        let mut ps: Vec<Problem> = vec![];

        for i in &self.0.conf.sys.categories.to_owned() {
            let json = self
                .0
                .clone()
                .get_category_problems(i)
                .await?
                .json()
                .await?;
            parser::problem(&mut ps, json).ok_or(Error::NoneError)?;
        }

        diesel::replace_into(problems)
            .values(&ps)
            .execute(&self.conn()?)?;

        Ok(ps)
    }

    /// Get problem
    pub fn get_problem(&self, rfid: i32) -> Result<Problem, Error> {
        let p: Problem = problems.filter(fid.eq(rfid)).first(&self.conn()?)?;
        if p.category != "algorithms" {
            return Err(Error::FeatureError(
                "Not support database and shell questions for now".to_string(),
            ));
        }

        Ok(p)
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
                .clone()
                .get_question_detail(&target.slug)
                .await?
                .json()
                .await?;
            debug!("{:#?}", &json);
            parser::desc(&mut rdesc, json).ok_or(Error::NoneError)?;

            // update the question
            let sdesc = serde_json::to_string(&rdesc)?;
            diesel::update(&target)
                .set(desc.eq(sdesc))
                .execute(&self.conn()?)?;
        }

        Ok(rdesc)
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
                self.clone()
                    .0
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

        let url = match run {
            Run::Test => conf.sys.urls.get("test").ok_or(Error::NoneError)?.replace("$slug", &p.slug),
            Run::Submit => {
                json.insert("judge_type", "large".to_string());
                conf.sys.urls.get("submit").ok_or(Error::NoneError)?.replace("$slug", &p.slug)
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
        use serde_json::{from_str, Error as SJError};
        use std::time::Duration;

        trace!("Run veriy recursion...");
        std::thread::sleep(Duration::from_micros(3000));

        // debug resp raw text
        let debug_raw = self
            .clone()
            .0
            .verify_result(rid.clone())
            .await?
            .text()
            .await?;
        debug!("debug resp raw text: \n{:#?}", &debug_raw);
        if debug_raw.is_empty() {
            return Err(Error::CookieError);
        }

        // debug json deserializing
        let debug_json: Result<VerifyResult, SJError> = from_str(&debug_raw);
        debug!("debug json deserializing: \n{:#?}", &debug_json);

        Ok(debug_json?)
    }

    /// Exec problem filter —— Test or Submit
    pub async fn exec_problem(
        &self,
        rfid: i32,
        run: Run,
        testcase: Option<String>,
    ) -> Result<VerifyResult, Error> {
        trace!("Exec problem filter —— Test or Submit");
        let (json, [url, refer]) = self.pre_run_code(run.clone(), rfid, testcase).await?;
        trace!("Pre run code result {:?}, {:?}, {:?}", json, url, refer);

        let run_res: RunCode = self
            .0
            .clone()
            .run_code(json.clone(), url.clone(), refer.clone())
            .await?
            .json()
            .await?;
        trace!("Run code result {:#?}", run_res);

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
