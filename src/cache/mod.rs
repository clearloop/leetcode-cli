//! Cache part - Save bad networks' ass.
mod models;
mod parser;
mod sql;
mod schemas;

use self::models::*;
use self::schemas::problems::dsl::*;
use self::sql::*;
use crate::{cfg, err::Error, plugins::LeetCode};
use diesel::{
    Connection,
    SqliteConnection,
    RunQueryDsl,
};
use reqwest::Error as ReqwestError;
use serde_json::Value;

/// sqlite connection
pub fn conn(p: String) -> SqliteConnection {
    SqliteConnection::establish(&p)
        .unwrap_or_else(|_| panic!("Error connecting to {:?}", p))
}


/// req if data not download3
pub struct Cache {
    conn: SqliteConnection,
    leetcode: LeetCode
}

impl Cache {
    /// Clean cache
    pub fn clean(&self) -> Result<(), Error> {
        let res = diesel::sql_query(DROP_PROBLEMS).execute(&self.conn);
        if res.is_err() {
            let err = res.err().unwrap();
            error!("{:?}", Error::CacheError(format!("clean local cache failed -> {}", &err)));
            return Err(Error::CacheError(format!("clean local cache failed -> {}", &err)));
        }
        
        Ok(())
    }
    
    /// Download leetcode problems to db
    pub fn download_problems(self) -> Result<Vec<Problem>, Error> {
        info!("Downloading leetcode categories...");
        let mut ps: Vec<Problem> = vec![];

        for i in &self.leetcode.conf.sys.categories.to_owned() {
            let res = self.leetcode
                .clone()
                .get_category_problems(&i);

            if res.is_err() {
                return Err(res.err().unwrap());
            }

            let json: Result<Value, ReqwestError> = res.unwrap().json();
            if json.is_err() {
                error!("{:?}", Error::DownloadError(format!("category {}", &i)));
                return Err(Error::DownloadError(format!("category {}", &i)));
            }

            // get "stat_status_pairs" from respnonse
            let res = parser::parse_problem(&mut ps, json.unwrap());
            if res.is_err() {
                error!("{:?}", Error::DownloadError(format!("category {}", &i)));
                return Err(Error::DownloadError(format!("category {}", &i)));
            }
        }

        // store problems into database
        let j = serde_json::to_string(&ps);
        if j.is_err() {
            error!("{:?}", Error::ParseError("data from cache"));
            return Err(Error::ParseError("data from cache"));
        }

        ps.sort_by(|a, b| b.id.partial_cmp(&a.id).unwrap());
        let res = diesel::insert_into(problems).values(&ps).execute(&self.conn);
        if res.is_err() {
            let err = res.err().unwrap();
            error!("{:?}", Error::CacheError(format!("Save to cache failed -> {}", &err)));
            return Err(Error::CacheError(format!("Save to cache failed -> {}", &err)));
        }

        Ok(ps)
    }

    /// Get problems from cache
    ///
    /// if cache doesn't exist, request a new copy
    ///
    /// [TODO]:
    ///  1. make downloading async
    pub fn get_problems(&self) -> Result<Vec<Problem>, Error> {
        let res = problems.load::<Problem>(&self.conn);
        if res.is_err() {
            let err = res.err().unwrap();
            warn!("Select problems from cache failed -> {:?} -> try downloading", &err);
            return Err(Error::CacheError(
                format!("Select problems from cache failed -> {:?} -> try downloading", &err)
            ));
        }
        
        Ok(res.unwrap())
    }

    /// New cache
    pub fn new() -> Result<Self, Error> {
        let p = cfg::root().join("lc.db");
        let c = conn(p.to_string_lossy().to_string());
        let r = diesel::sql_query(CREATE_PROBLEMS_IF_NOT_EXISTS).execute(&c);
        if r.is_err() {
            let err = r.err().unwrap();
            error!("{:?}", Error::CacheError(format!("Create local cache failed -> {}", &err)));
            return Err(Error::CacheError(format!("Create local cache failed -> {}", &err)));
        }
        
        Ok(Cache{
            conn: c,
            leetcode: LeetCode::new(),
        })
    }
}
