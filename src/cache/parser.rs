//! mod for parse resp data
use serde_json::Value;
use crate::err::Error;
use super::models::Problem;

pub fn parse_problem(problems: &mut Vec<Problem>, v: Value) -> Result<(), Error> {
    let pairs = v.get("stat_status_pairs")?.as_array()?;
    for p in pairs {
        let category = v.get("category_slug")?.as_str()?.to_string();
        let level = p.get("difficulty")?.as_object()?.get("level")?.as_i64()? as i32;
        let starred = p.get("is_favor")?.as_bool()?;
        let locked = p.get("paid_only")?.as_bool()?;
        let state = p.get("status")?.as_str()?.to_string();
        let stat = p.get("stat")?.as_object()?;
        let id = stat.get("question_id")?.as_i64()? as i32;
        let fid = stat.get("frontend_question_id")?.as_i64()? as i32;
        let name = stat.get("question__title")?.as_str()?.to_string();
        let slug = stat.get("question__title_slug")?.as_str()?.to_string();
        let total_acs = stat.get("total_acs")?.as_f64()? as f32;
        let total_submitted = stat.get("total_submitted")?.as_f64()? as f32;

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
