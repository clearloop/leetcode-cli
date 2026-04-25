//! Leetcode-cli plugins
//!
//! + chrome cookie parser
//! + leetcode API
//!
//! ## login to `leetcode.com`
//! Leetcode-cli use chrome cookie directly, do not need to login, please make sure you have logged into `leetcode.com` before using `leetcode-cli`
//!

// FIXME: Read cookies from local storage. (issue #122)
mod chrome;
mod leetcode;
pub use leetcode::LeetCode;
