//! Leetcode data models
use colored::Colorize;
use serde::Serialize;
use super::schemas::problems;

/// Problem model
#[derive(AsChangeset, Clone, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "problems"]
pub struct Problem {
    pub category: String,
    pub fid: i32,    
    pub id: i32,
    pub level: i32,
    pub locked: bool,
    pub name: String,
    pub percent: f32,
    pub slug: String,
    pub starred: bool,
    pub state: String,
}

static DONE: &'static str = " ✔";
static ETC: &'static str = "...";
static LOCK: &'static str = "🔒";
static NDONE: &'static str = "✘";
static SPACE: &'static str = " ";
impl std::fmt::Display for Problem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let space_2 = SPACE.repeat(2);
        let mut lock = space_2.as_str();
        let mut done = space_2.normal();
        let mut id = "".to_string();
        let mut name = "".to_string();
        let mut level = "".normal();

        if self.locked { lock = LOCK };
        if self.state == "ac".to_string() {
            done = DONE.green().bold();
        } else if self.state == "notac" {
            done = NDONE.green().bold();
        }

        match self.fid.to_string().len() {
            1 => {
                id.push_str(&SPACE.repeat(2));
                id.push_str(&self.fid.to_string());
                id.push_str(&SPACE.repeat(1));
            },
            2 => {
                id.push_str(&SPACE.repeat(1));
                id.push_str(&self.fid.to_string());
                id.push_str(&SPACE.repeat(1));
            },
            3 => {
                id.push_str(&SPACE.repeat(1));
                id.push_str(&self.fid.to_string());
            },
            4 => {
                id.push_str(&self.fid.to_string());
            },
            _ => {
                id.push_str(&space_2);
                id.push_str(&space_2);
            }
        }

        if &self.name.len() < &60_usize {
            name.push_str(&self.name);
            name.push_str(&SPACE.repeat(60 - &self.name.len()));
        } else {
            name.push_str(&self.name[..49]);
            name = name.trim_end().to_string();
            name.push_str(ETC);
            name.push_str(&SPACE.repeat(60 - name.len()));
        }

        level = match self.level {
            1 => "Easy  ".bright_green(),
            2 => "Medium".bright_yellow(),
            3 => "Hard  ".bright_red(),
            _ => level
        };
        
        write!(
            f,
            "  {} {} [{}] {} {} ({} %)",
            lock, done, id, name, level,
            &self.percent.to_string()[0..5]
        )
    }
}


/// Description Model
pub struct DescData {
    pub question: Question
}

/// desc.question
pub struct Question {
    pub content: String,
    pub stat: QuestionStat,
    pub code_defintion: Vec<CodeDefintion>,
    pub sample_text_case: String,
    pub enable_run_code: bool,
    pub meta_data: MetaData,
    pub translated_cotent: String
}

pub struct QuestionStat {
    pub total_accepted: String,
    pub total_submission: String,
    pub total_accepted_aw: i64,
    pub total_submission_raw: i64,
    pub ac_rate: String
}

pub struct CodeDefintion {
    pub value: String,
    pub text: String,
    pub default_code: String,
}

pub struct MetaData {
    pub name: String,
    pub params: Vec<Param>,
    pub r#return: Return,
}

pub struct Param {
    pub name: String,
    pub r#type: String
}

pub struct Return {
    pub r#type: String
}
