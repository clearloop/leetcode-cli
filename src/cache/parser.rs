//! mod for parse resp data
use serde_json::Value;
use crate::err::Error;
use super::models::Problem;

pub fn parse_problem(problems: &mut Vec<Problem>, v: Value) -> Result<(), Error> {
    if let Some(Value::Array(pairs)) = v.get("stat_status_pairs") {
        for p in pairs {
            let category: String = match v.get("category_slug") {
                Some(Value::String(s)) => s.to_string(),
                _ => {
                    error!("{:?}", Error::ParseError("String category_slug"));
                    return Err(Error::ParseError("String category_slug"));
                }
            };

            let level: i32 = match p.get("difficulty") {
                Some(Value::Object(o)) => {
                    match o.get("level") {
                        Some(Value::Number(n)) => n.as_i64().unwrap() as i32,
                        _ => {
                            error!("{:?}", Error::ParseError("Integer level"));
                            return Err(Error::ParseError("Integer level"));
                        }
                    }
                },
                _ => {
                    error!("{:?}", Error::ParseError("Integer level"));
                    return Err(Error::ParseError("Integer level"));
                }
            };

            let starred: bool = match p.get("is_favor") {
                Some(Value::Bool(b)) => *b,
                _ => {
                    error!("{:?}", Error::ParseError("bool is_favor"));
                    return Err(Error::ParseError("bool is_favor"));
                }
            };

            let locked: bool = match p.get("paid_only") {
                Some(Value::Bool(b)) => *b,
                _ => {
                    error!("{:?}", Error::ParseError("Integer paid_only"));
                    return Err(Error::ParseError("Integer paid_only"));
                }
            };
            
            let state: String = match p.get("status") {
                Some(Value::Null) => "Null".to_string(),
                Some(Value::String(s)) => s.to_string(),
                _ => {
                    error!("{:?}", Error::ParseError("String status"));
                    return Err(Error::ParseError("String status"));
                }
            };

            // first cond with stat, and then no more.
            let id: i32 = match p.get("stat") {
                Some(Value::Object(o)) => {
                    match o.get("question_id") {
                        Some(Value::Number(n)) => n.as_i64().unwrap() as i32,
                        _ => {
                            error!("{:?}", Error::ParseError("Integer question_id"));
                            return Err(Error::ParseError("Integer question_id"));
                        }
                    }
                },
                _ => {
                    error!("{:?}", Error::ParseError("Integer question_id"));
                    return Err(Error::ParseError("Integer question_id"));
                }
            };

            let fid: i32 = match p.get("stat").unwrap().get("frontend_question_id") {
                Some(Value::Number(n)) => n.as_i64().unwrap() as i32,
                _ => {
                    error!("{:?}", Error::ParseError("Integer frontend_question_id"));
                    return Err(Error::ParseError("Integer frontend_question_id"));
                }
            };

            let name: String = match p.get("stat").unwrap().get("question__title") {
                Some(Value::String(s)) => s.to_string(),
                _ => {
                    error!("{:?}", Error::ParseError("String question__title"));
                    return Err(Error::ParseError("String question__title"));
                }
            };

            let slug: String = match p.get("stat").unwrap().get("question__title_slug") {
                Some(Value::String(s)) => s.to_string(),
                _ => {
                    error!("{:?}", Error::ParseError("String question__title_slug"));
                    return Err(Error::ParseError("String question__title_slug"));
                }
            };

            let total_acs: f32 = match p.get("stat").unwrap().get("total_acs") {
                Some(Value::Number(n)) => n.as_i64().unwrap() as f32,
                _ => {
                    error!("{:?}", Error::ParseError("Float tatal_acs"));
                    return Err(Error::ParseError("Float tatal_acs"));
                }
            };

            let total_submitted: f32 = match p.get("stat").unwrap().get("total_submitted") {
                Some(Value::Number(n)) => n.as_i64().unwrap() as f32,
                _ => {
                    error!("{:?}", Error::ParseError("Float tatal_submitted"));
                    return Err(Error::ParseError("Float tatal_submitted"));
                }
            };

            // push problems
            problems.push(Problem{
                category,
                fid,
                id,
                level,
                locked,
                name,
                percent: total_acs / total_submitted * 100.0,
                slug,
                starred,
                state,
            });
        }
        
        return Ok(());
    }

    error!("Response from https://leetcode.com doesn't content problem details");
    Err(Error::ParseError("the resp of `https://leetcode.com`"))
}
