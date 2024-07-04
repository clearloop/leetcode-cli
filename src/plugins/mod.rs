//! Leetcode-cil plugins
//!
//! + chrome cookie parser
//! + leetcode API
//!
//! ## login to `leetcode.cn`
//! Leetcode-cli use chrome cookie directly, do not need to login, please make sure you have loggined in `leetcode.cn` before usnig `leetcode-cli`
//!

// FIXME: Read cookies from local storage. (issue #122)
mod chrome;
mod leetcode;
pub use leetcode::LeetCode;
