//! Sub-Module for parsing resp data
use super::models::*;
use serde_json::{Value,from_value};


/// contest parser
pub fn contest(v: Value) -> Option<Contest> {
    let o = v.as_object()?;
    let mut contest: Contest = from_value(
        o.get("contest")?.clone()
    ).ok()?;
    contest.questions = from_value(
        o.get("questions")?.clone()
    ).ok()?;
    contest.contains_premium = o.get("containsPremium")?.as_bool()?;
    contest.registered = o.get("registered")?.as_bool()?;
    Some(contest)
}

/// problem parser
pub fn problem(problems: &mut Vec<Problem>, v: Value) -> Option<()> {
    let pairs = v.get("stat_status_pairs")?.as_array()?;
    for p in pairs {
        let stat = p.get("stat")?.as_object()?;
        let total_acs = stat.get("total_acs")?.as_f64()? as f32;
        let total_submitted = stat.get("total_submitted")?.as_f64()? as f32;

        problems.push(Problem {
            category: v.get("category_slug")?.as_str()?.to_string(),
            fid: stat.get("frontend_question_id")?.as_i64()? as i32,
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

// TODO: implement test for this
/// graphql problem && question parser
pub fn graphql_problem_and_question(v: Value) -> Option<(Problem,Question)> {
    // parse top-level data from API
    let mut qn = Question::default();
    assert_eq!(Some(true), desc(&mut qn, v.clone()));
    let percent = &qn.stats.rate;
    let percent = percent[..percent.len()-1].parse::<f32>().ok()?;

    // parse v.question specifically
    let v = v.as_object()?.get("data")?
        .as_object()?.get("question")?;
    let mut p: Problem = from_value(v.clone()).unwrap();
    p.percent = percent;
    p.level = match v.as_object()?.get("difficulty")?.as_str()?.chars().next()? {
        'E' => 1,
        'M' => 2,
        'H' => 3,
        _ => 0,
    };
    p.status = v.get("status")?.as_str().unwrap_or("Null").to_owned();
    p.desc = serde_json::to_string(&qn).ok()?;
    /* The graphql API doesn't return the category slug, only the printed category title. */
    p.category = p.category.to_ascii_lowercase(); // Currently working (June 2022)
    /* But lowercasing is stupid. This will break if a category with whitespaces appears. */
    Some((p,qn))
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
        all_cases: o.get("exampleTestcases")
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
        .get("data")?.as_object()?
        .get("activeDailyCodingChallengeQuestion")?.as_object()?
        .get("question")?.as_object()?
        .get("questionFrontendId")?.as_str()?
        .parse().ok()
}

/// user parser
pub fn user(v: Value) -> Option<Option<(String,bool)>> {
    // None => error while parsing
    // Some(None) => User not found
    // Some("...") => username
    let user = v.as_object()?.get("data")?
        .as_object()?.get("user")?;
    if *user == Value::Null { return Some(None) }
    let user = user.as_object()?;
    Some(Some((
        user.get("username")?.as_str()?.to_owned(),
        user.get("isCurrentUserPremium")?.as_bool()?
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
