//! Sub-Module for parsing resp data
use super::models::*;
use serde_json::Value;

/// problem parser
pub fn problem(problems: &mut Vec<Problem>, v: Value) -> Option<()> {
    let pairs = v.get("stat_status_pairs")?.as_array()?;
    for p in pairs {
        let stat = p.get("stat")?.as_object()?;
        let total_acs = stat.get("total_acs")?.as_f64()? as f32;
        let total_submitted = stat.get("total_submitted")?.as_f64()? as f32;

        problems.push(Problem {
            category: v.get("category_slug")?.as_str()?.to_string(),
            fid: stat
                .get("frontend_question_id")?
                .as_str()?
                .split(" ")
                .last()?
                .parse::<i32>()
                .ok()?,
            id: stat.get("question_id")?.as_i64()? as i32,
            level: p.get("difficulty")?.as_object()?.get("level")?.as_i64()? as i32,
            locked: p.get("paid_only")?.as_bool()?,
            name: stat.get("question__title")?.as_str()?.to_string(),
            percent: total_acs / total_submitted * 100.0,
            slug: stat.get("question__title_slug")?.as_str()?.to_string(),
            starred: p.get("is_favor")?.as_bool()?,
            status: p.get("status")?.as_str().unwrap_or("Null").to_string(),
            desc: String::new(),
        });
    }

    Some(())
}

/// desc parser
pub fn desc(q: &mut Question, v: Value) -> Option<bool> {
    /* None - parsing failed
     * Some(false) - content was null (premium?)
     * Some(true) - content was parsed
     */
    let o = &v
        .as_object()?
        .get("data")?
        .as_object()?
        .get("question")?
        .as_object()?;

    if *o.get("content")? == Value::Null {
        return Some(false);
    }

    *q = Question {
        content: o.get("content")?.as_str().unwrap_or("").to_string(),
        stats: serde_json::from_str(o.get("stats")?.as_str()?).ok()?,
        defs: serde_json::from_str(o.get("codeDefinition")?.as_str()?).ok()?,
        case: o.get("sampleTestCase")?.as_str()?.to_string(),
        all_cases: o
            .get("exampleTestcases")
            .unwrap_or(o.get("sampleTestCase")?) // soft fail to the sampleTestCase
            .as_str()?
            .to_string(),
        metadata: serde_json::from_str(o.get("metaData")?.as_str()?).ok()?,
        test: o.get("enableRunCode")?.as_bool()?,
        t_content: o
            .get("translatedContent")?
            .as_str()
            .unwrap_or("")
            .to_string(),
    };

    Some(true)
}

/// tag parser
pub fn tags(v: Value) -> Option<Vec<String>> {
    trace!("Parse tags...");
    let tag = v.as_object()?.get("data")?.as_object()?.get("topicTag")?;

    if tag.is_null() {
        return Some(vec![]);
    }

    let arr = tag.as_object()?.get("questions")?.as_array()?;

    let mut res: Vec<String> = vec![];
    for q in arr.iter() {
        res.push(q.as_object()?.get("questionId")?.as_str()?.to_string())
    }

    Some(res)
}

/// daily parser
pub fn daily(v: Value) -> Option<i32> {
    trace!("Parse daily...");
    v.as_object()?
        .get("data")?
        .as_object()?
        .get("todayRecord")?
        .as_array()?[0]
        .as_object()?
        .get("question")?
        .as_object()?
        .get("questionFrontendId")?
        .as_str()?
        .parse()
        .ok()
}

/// user parser
pub fn user(v: Value) -> Option<Option<(String, bool)>> {
    // None => error while parsing
    // Some(None) => User not found
    // Some("...") => username
    let user = v.as_object()?.get("data")?.as_object()?.get("user")?;
    if *user == Value::Null {
        return Some(None);
    }
    let user = user.as_object()?;
    Some(Some((
        user.get("username")?.as_str()?.to_owned(),
        user.get("isCurrentUserPremium")?.as_bool()?,
    )))
}

pub use ss::ssr;
/// string or squence
mod ss {
    use serde::{de, Deserialize, Deserializer};
    use std::fmt;
    use std::marker::PhantomData;

    /// de Vec<String> from string or sequence
    pub fn ssr<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct StringOrVec(PhantomData<Vec<String>>);

        impl<'de> de::Visitor<'de> for StringOrVec {
            type Value = Vec<String>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("string or list of strings")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(vec![value.to_owned()])
            }

            fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
            where
                S: de::SeqAccess<'de>,
            {
                Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
            }
        }

        deserializer.deserialize_any(StringOrVec(PhantomData))
    }
}
