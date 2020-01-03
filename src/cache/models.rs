//! Leetcode data modelsA
use colored::Colorize;
use serde::{Serialize, Deserialize};
pub use self::question::*;
use super::schemas::problems;

/// Problem model
#[derive(AsChangeset, Clone, Identifiable, Insertable, Queryable, Serialize, Debug)]
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
    pub status: String,
    pub desc: String,
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
        if self.status == "ac".to_string() {
            done = DONE.green().bold();
        } else if self.status == "notac" {
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

/// desc model
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Question {
    pub content: String,
    pub stats: Stats,
    pub defs: CodeDefintion,
    pub case: String,
    pub metadata: MetaData,
    pub test: bool,
    pub t_content: String,
}

/// deps of Question
mod question {
    use serde::{Serialize, Deserialize};

    /// Code samples
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct CodeDefintion(pub Vec<CodeDefintionInner>);

    /// CodeDefinition Inner struct
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct CodeDefintionInner {
        pub value: String,
        pub text: String,
        #[serde(alias = "defaultCode")]
        pub code: String,
    }

    /// Question status
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Stats {
        #[serde(alias = "totalAccepted")]
        tac: String,
        #[serde(alias = "totalSubmission")]
        tsm: String,
        #[serde(alias = "totalAcceptedRaw")]
        tacr: i32,
        #[serde(alias = "totalSubmissionRaw")]
        tsmr: i32,
        #[serde(alias = "acRate")]
        rate: String
    }
    
    /// Algorithm metadata
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct MetaData {
        pub name: String,
        pub params: Vec<Param>,
        pub r#return: Return,
    }

    /// MetaData nested fields
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Param {
        pub name: String,
        pub r#type: String
    }

    /// MetaData nested fields
    #[derive(Debug, Default, Serialize, Deserialize)]
    pub struct Return {
        pub r#type: String
    }
}
