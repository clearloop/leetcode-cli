//! Leetcode-cli plugins
//!
//! + chrome cookie parser
//! + leetcode API
//!
//! ## login to `leetcode.com`
//! Leetcode-cli uses chrome cookie directly, do not need to login, please make sure you have logged in `leetcode.com` before using `leetcode-cli`
//!

// FIXME: Read cookies from local storage. (issue #122)
// Diesel's QueryableByName derive emits explicit field names.
#[allow(clippy::redundant_field_names)]
mod chrome;
mod leetcode;
pub use leetcode::LeetCode;
