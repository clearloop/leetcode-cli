//! leetcode-cil plugins
//! + chrome cookie parser
//! + leetcode API
//!
//! ## login to `leetcode.com`
//! leetcode-cli use chrome cookie directly, do not need to login, please make sure you have loggined in `leetcode.com` before usnig `leetcode-cli`
//! 
mod chrome;
mod leetcode;

pub use leetcode::LeetCode;
