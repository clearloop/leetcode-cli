use diesel::{
    Connection,
    SqliteConnection,
    RunQueryDsl,
    query_dsl::filter_dsl::FindDsl
};

/// sqlite connection
pub fn conn(p: String) -> SqliteConnection {
    SqliteConnection::establish(&p)
        .unwrap_or_else(|_| panic!("Error connecting to {:?}", p))
}

/// Leetcode data schemas
mod schemas {
    table! {
        problems(id) {
            state -> Text,
            id -> Integer,
            fid -> Integer,
            name -> Text,
            slug -> Text,
            locked -> Bool,
            percent -> Float,
            level -> Integer,
            starred -> Bool,
            category -> Text,
        }
    }
}

/// Leetcode data models
mod models {
    #[derive(Queryable, Debug, Clone)]
    pub struct Problem {
        state: String,
        id: i32,
        fid: i32,
        name: String,
        slug: String,
        locked: bool,
        percent: f32,
        level: i32,
        starred: bool,
        category: String
    }
}

use self::models::*;
use self::schemas::*;
use crate::{
    cfg,
    err::Error,
    plugins::LeetCode,
};
use reqwest::Error as ReqwestError;
use serde_json::{
    map::Map,
    Value,
};
/// Save bad networks' ass.
pub struct Cache {
    conn: SqliteConnection,
    leetcode: LeetCode
}

impl Cache {
    pub fn new() -> Self {
        let p = cfg::root().join("lc.db");
        Cache{
            conn: conn(p.to_string_lossy().to_string()),
            leetcode: LeetCode::new(),
        }
    }

    /// Download leetcode problems to db
    pub fn download_problems(self) -> Result<(), Error> {
        info!("Downloading leetcode categories...");
        let problems: Vec<Problem> = vec![];

        for i in self.leetcode.conf.sys.categories.clone().into_iter() {
            let res = self.leetcode
                .clone()
                .get_category_problems(&i);

            // Download error
            if res.is_err() {
                return Err(res.err().unwrap());
            }

            // Parse error
            let json: Result<Value, ReqwestError> = res.unwrap().json();
            if json.is_err() {
                error!("Downloading category {} failed, please try again.", &i);
                return Err(Error::DownloadError);
            }

            // Get "stat_status_pairs" from respnonse
            let obj = json.unwrap();
            if let Some(Value::Array(pairs)) = obj.get("stat_status_pairs") {
                for p in pairs {
                    let state: String = match p.get("status") {
                        Some(Value::Null) => "Null".to_string(),
                        Some(Value::String(s)) => s.to_string(),
                        _ => return Err(Error::ParseError),
                    };

                    let paid_only: bool = match p.get("paid_only") {
                        Some(Value::Bool(b)) => *b,
                        _ => return Err(Error::ParseError),
                    };

                    let stat = p.get("stat");
                    println!("stat: {:#?}", stat);
                    
                    let is_favor: bool = match p.get("is_favor") {
                        Some(Value::Bool(b)) => *b,
                        _ => return Err(Error::ParseError),
                    };

                    let difficult: i32 = match p.get("difficulty") {
                        Some(Value::Object(o)) => {
                            match o.get("level") {
                                Some(Value::Number(n)) => n.as_i64().unwrap() as i32,
                                _ => return Err(Error::ParseError),
                            }
                        },
                        _ => return Err(Error::ParseError),
                    };

                    let category_slug: String = match obj.get("category_slug") {
                        Some(Value::String(s)) => s.to_string(),
                        _ => return Err(Error::ParseError),
                    };
                    // problems.push(Problem{
                    //     state: p["status"],
                    //     id: p["stat"]["question_id"],
                    //     fid: p["stat"]["frontend_question_id"],
                    //     name: p["stat"]["question_title"],
                    //     slug: p["stat"]["question_title_slug"],
                    //     locked: p["paid_only"],
                    //     percent: p["stat"]["total_acs"] * 100 / p["stat"]["total_submitted"],
                    //     level: p["difficulty"]["level"],
                    //     starred: p["is_favor"],
                    //     category: p["category_slug"]
                    // })
                }                    
            }
        }
        Ok(())
    }
    
    /// Get problems from cache
    ///
    /// if cache doesn't exist, request a new copy
    ///
    /// [TODO]:
    ///  1. make downloading async
    ///  2. download from config
    pub fn get_problems(&self) -> Vec<Problem> {
        use self::schemas::problems::dsl::*;
        let mut res = problems.load::<Problem>(&self.conn);
        if res.is_err() {
            error!("Select problems from cache failed");
            // &self.download_problems();

            res = problems.load::<Problem>(&self.conn);
            if res.is_err() {
                panic!("Download error.");
            }
        }

        res.unwrap()
    }
}
