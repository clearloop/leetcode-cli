//! Save bad network\'s ass.
mod parser;
mod sql;
pub mod models;
pub mod schemas;
use self::models::*;
use self::schemas::problems::dsl::*;
use self::sql::*;
use crate::{cfg, err::Error, plugins::LeetCode};
use diesel::prelude::*;
use serde_json::Value;
use colored::Colorize;

/// sqlite connection
pub fn conn(p: String) -> SqliteConnection {
    SqliteConnection::establish(&p)
        .unwrap_or_else(|_| panic!("Error connecting to {:?}", p))
}

/// req if data not download
#[derive(Clone)]
pub struct Cache(pub LeetCode);

impl Cache {
    /// ref to sqliteconnection
    fn conn(&self) -> SqliteConnection {
        conn(self.0.conf.storage.cache())
    }
    
    /// Clean cache
    pub fn clean(&self) -> Result<(), Error> {
        Ok(std::fs::remove_file(&self.0.conf.storage.cache())?)
    }

    /// ref to download probems
    pub fn update(self) -> Result<(), Error> {
        let c = conn((&self.0.conf.storage.cache()).to_owned());
        let ps = self.download_problems()?;
        for i in ps.into_iter() {
            let target = problems.filter(id.eq(i.id));
            diesel::update(target).set(i.to_owned()).execute(&c)?;
        }
        
        Ok(())
    }
    
    /// Download leetcode problems to db
    pub fn download_problems(self) -> Result<Vec<Problem>, Error> {
        info!("Fetching leetcode problems...");
        let mut ps: Vec<Problem> = vec![];

        for i in &self.0.conf.sys.categories.to_owned() {
            let json = self.0.clone().get_category_problems(&i)?.json()?;
            parser::problem(&mut ps, json)?;
        }

        let count = self.get_problems()?.len();
        if count == 0 {
            ps.sort_by(|a, b| b.id.partial_cmp(&a.id).unwrap_or(std::cmp::Ordering::Equal));
            diesel::insert_into(problems).values(&ps).execute(&self.conn())?;
        }

        Ok(ps)
    }

    /// Get problem description
    pub fn get_desc(&self, rfid: i32) -> Result<Question, Error> {
        let target: Problem = problems
            .filter(fid.eq(rfid))
            .first(&self.conn())?;

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
        if target.category != "algorithms".to_string() {
            return Err(Error::FeatureError(
                "Not support database and shell questions for now".to_string()
            ));
        }

        if target.locked  {
            return Err(Error::FeatureError(
                "Not support premium question for now".to_string()
            ));
        }

        let mut rdesc = Question::default();
        if target.desc.len() > 0 {
            rdesc = serde_json::from_str(&target.desc)?;
        } else {
            let json: Value = self.0.clone().get_question_detail(&target.slug)?.json()?;
            parser::desc(&mut rdesc, json)?;

            // update the question
            let sdesc = serde_json::to_string(&rdesc)?;
            diesel::update(&target).set(desc.eq(sdesc)).execute(&self.conn())?;
        }

        Ok(rdesc)
    }
    
    /// Get problems from cache
    ///
    /// if cache doesn't exist, request a new copy
    pub fn get_problems(&self) -> Result<Vec<Problem>, Error> {
        Ok(problems.load::<Problem>(&self.conn())?)
    }

    /// New cache
    pub fn new() -> Result<Self, Error> {
        let conf = cfg::locate();
        let c = conn(conf.storage.cache());
        diesel::sql_query(CREATE_PROBLEMS_IF_NOT_EXISTS).execute(&c)?;
        
        Ok(Cache(LeetCode::new()))
    }
}
