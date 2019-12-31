//! Leetcode data models
use serde::{Deserialize, Serialize};

/// Problem model
#[derive(Clone, Debug, Queryable, Serialize)]
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
