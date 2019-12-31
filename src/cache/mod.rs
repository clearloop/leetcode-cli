//! Cache part - Save bad networks' ass.
mod models;
mod parser;
mod schemas;
mod sql;
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


/// req if data not download
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
    pub fn download_problems(self) -> Result<(), Error> {
        info!("Downloading leetcode categories...");
        let mut ps: Vec<Problem> = vec![];

        for i in self.leetcode.conf.sys.categories.clone().into_iter() {
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
        let j = serde_json::to_string(&ps[..3]);
        if j.is_err() {
            error!("{:?}", Error::ParseError("data from cache"));
            return Err(Error::ParseError("data from cache"));
        }

        println!("{:?}", &j);
        Ok(())
    }

    /// Get problems from cache
    ///
    /// if cache doesn't exist, request a new copy
    ///
    /// [TODO]:
    ///  1. make downloading async
    pub fn get_problems(&self) -> Vec<Problem> {
        let mut res = problems.load::<Problem>(&self.conn);
        if res.is_err() {
            error!("Select problems from cache failed -> {:?}", res.err().unwrap());
            // &self.download_problems();

            res = problems.load::<Problem>(&self.conn);
            if res.is_err() {
                // error!("Select problems from cache failed");
            }
        }

        res.unwrap()
    }

    /// New cache
    pub fn new() -> Result<Self, Error> {
        let p = cfg::root().join("lc.db");
        let c = conn(p.to_string_lossy().to_string());

        let r = diesel::sql_query(CREATE_PROBLEMS_IF_NOT_EXISTS).execute(&c);
        if r.is_err() {
            let err = r.err().unwrap();
            error!("{:?}", Error::CacheError(format!("create local cache failed -> {}", &err)));
            return Err(Error::CacheError(format!("create local cache failed -> {}", &err)));
        }
        
        Ok(Cache{
            conn: c,
            leetcode: LeetCode::new(),
        })
    }
}
