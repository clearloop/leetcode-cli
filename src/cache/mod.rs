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
        info!("Downloading leetcode problems...");
        let mut ps: Vec<Problem> = vec![];

        for i in &self.0.conf.sys.categories.to_owned() {
            let json = self.0.clone().get_category_problems(&i)?.json()?;
            parser::problem(&mut ps, json)?
        }

        let count = self.get_problems()?.len();
        if count == 0 {
            ps.sort_by(|a, b| b.id.partial_cmp(&a.id).unwrap_or(std::cmp::Ordering::Equal));
            diesel::insert_into(problems).values(&ps).execute(&self.conn())?;
        }

        Ok(ps)
    }

    /// Get problem description
    pub fn get_desc(&self, rfid: i32) -> Result<bool, Error> {
        let rslug: String = problems
            .select(slug)
            .filter(fid.eq(rfid))
            .first(&self.conn())
            .unwrap();

        let mut res = self.0.clone().get_question_detail(&rslug)?;
        let json: Value = res.json()?;
        println!("{:?}", json);
        // let problem = diesel::select(
        //     problems.filter(fid.eq(fid))
        // ).execute(&self.conn());
        Ok(true)
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
