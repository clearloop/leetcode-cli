//! # leetcode-cli
//! [![doc](https://img.shields.io/badge/current-docs-green.svg)](https://docs.rs/leetcode-cli/)
//! [![Crates.io](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
//! [![Crates.io](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
//! [![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)
//!
//! ## Contributors
//! + [@hulufi](https://github.com/hulufei)
//! + [@ldm0](https://github.com/ldm0)
//! + [@Raees678](https://github.com/Raees678)
//! + [@clearloop](https://github.com/clearloop)
//!
//! ## Features
//!
//! + [x] the edit flow —— solution files will generate automatically!
//! + [x] support python script to filter questions
//! + [ ] doc support, `lc-rs` can compile the annotation of your solutions to markdown!
//! + [ ]  support local signal to keep coding as longer as you want.
//!
//! ## Building
//!
//! ```sh
//! cargo install leetcode-cli
//! ```
//!
//! ## Usage
//!
//! **Please make sure you have logined in `leetcode.com` with `chrome`**, more info plz checkout [this](#cookies)
//!
//! ```sh
//! leetcode 0.3.3
//! May the Code be with You 👻
//!
//! USAGE:
//!     leetcode [FLAGS] [SUBCOMMAND]
//!
//! FLAGS:
//!     -d, --debug      debug mode
//!     -h, --help       Prints help information
//!     -V, --version    Prints version information
//!
//! SUBCOMMANDS:
//!     data    Manage Cache [aliases: d]
//!     edit    Edit question by id [aliases: e]
//!     exec    Submit solution [aliases: x]
//!     list    List problems [aliases: l]
//!     pick    Pick a problem [aliases: p]
//!     stat    Show simple chart about submissions [aliases: s]
//!     test    Edit question by id [aliases: t]
//!     help    Prints this message or the help of the given subcommand(s)
//! ```
//!
//! ## Example
//!
//! For example, if your config is:
//!
//! ```toml
//! [code]
//! lang = "rust"
//! editor = "emacs"
//! ```
//!
//! #### 1. <kbd>pick</kbd>
//!
//! ```sh
//! leetcode pick 1
//! ```
//!
//! ```sh
//! [1] Two Sum is on the run...
//!
//!
//! Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//!
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! --------------------------------------------------
//!
//! Example:
//!
//!
//! Given nums = [2, 7, 11, 15], target = 9,
//!
//! Because nums[0] + nums[1] = 2 + 7 = 9,
//! return [0, 1].
//! ```
//!
//! #### 2. <kbd>edit</kbd>
//!
//! ```sh
//! leetcode edit 1
//! ```
//!
//! ```rust
//! # struct Solution;
//! impl Solution {
//!     pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//!         use std::collections::HashMap;
//!         let mut m: HashMap<i32, i32> = HashMap::new();
//!
//!         for (i, e) in nums.iter().enumerate() {
//!             if let Some(v) = m.get(&(target - e)) {
//!                 return vec![*v, i as i32];
//!             }
//!
//!             m.insert(*e, i as i32).unwrap_or_default();
//!         }
//!
//!         return vec![];
//!     }
//! }
//! ```
//!
//! #### 3. <kbd>test</kbd>
//!
//! ```sh
//! leetcode test 1
//! ```
//!
//! ```sh
//!
//!   Accepted       Runtime: 0 ms
//!
//!   Your input:    [2,7,11,15], 9
//!   Output:        [0,1]
//!   Expected:      [0,1]
//!
//! ```
//!
//! #### 4. <kbd>submit</kbd>
//!
//! ```sh
//! leetcode submit 1
//! ```
//!
//! ```sh
//!
//!   Success
//!
//!   Runtime: 0 ms, faster than 100% of Rustonline submissions for Two Sum.
//!
//!   Memory Usage: 2.4 MB, less than 100% of Rustonline submissions for Two Sum.
//!
//!
//! ```
//!
//! ## Cookies
//!
//! The cookie plugin of leetcode-cil can work on OSX and [Linux][#1], **If you are on other platforms or your cookies just don't want to be catched**, you can handwrite your LeetCode Cookies to `~/.leetcode/leetcode.toml`
//!
//! ```toml
//! # Make sure `leetcode.toml` file is placed at `~/.leetcode/leetcode.toml`
//! [cookies]
//! csrf = "..."
//! session = "..."
//! ```
//!
//! For Example, if you're using chrome to login to leetcode.com.
//!
//!
//! #### Step 1
//!
//! Open chrome and paste the link below to the `chrome linkbar`.
//!
//! ```sh
//! chrome://settings/cookies/detail?site=leetcode.com
//! ```
//!
//! #### Step 2
//!
//! Copy the contents of `LEETCODE_SESSION` and `csrftoken`.
//!
//! #### Step 3
//!
//! Paste them to `session` and `csrf`.
//!
//! ```toml
//! # Make sure `leetcode.toml` file is placed at `~/.leetcode/leetcode.toml`
//! [cookies]
//! csrf = "${csrftoken}"
//! session = "${LEETCODE_SESSION}"
//! ```
//!
//!
//! ## Programmable
//!
//! If we want to filter leetcode questions using our own python scripts, what should we do?
//!
//! For example, our config is:
//!
//! ```toml
//! # Make sure `leetcode.toml` file is placed at `~/.leetcode/leetcode.toml`
//! [storage]
//! scripts = "scripts"
//! ```
//!
//! We write our python scripts:
//!
//! ```python
//! # ~/.leetcode/scripts/plan1.py
//! import json;
//!
//! def plan(sps, stags):
//!     ##
//!     # `print` in python is supported,
//!     # if you want to know the data structures of these two args,
//!     # just print them
//!     ##
//!     problems = json.loads(sps)
//!     tags = json.loads(stags)
//!
//!     ret = []
//!     tm = {}
//!     for tag in tags:
//!         tm[tag["tag"]] = tag["refs"];
//!
//!     for i in problems:
//!         if i["level"] == 1 and str(i["id"]) in tm["linked-list"]:
//!             ret.append(str(i["id"]))
//!
//!     # return is `List[string]`
//!     return ret
//! ```
//!
//! Then we can run filter as what we write now:
//!
//! ```sh
//! leetcode list -p plan1
//! ```
//!
//! Well done, enjoy it!
//!
//!
//! ## PR
//!
//! PR is welcome, [here][pr] it is.
//!
//! ## LICENSE
//! MIT
//!
//!
//! [pr]: https://github.com/clearloop/leetcode-cli/pulls
//! [#1]: https://github.com/clearloop/leetcode-cli/issues/1
#![feature(try_trait)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;

// show docs
pub mod cache;
pub mod cfg;
pub mod cli;
pub mod cmds;
pub mod err;
pub mod flag;
pub mod helper;
pub mod plugins;
#[cfg(feature = "pym")]
pub mod pym;

// re-exports
pub use cache::Cache;
pub use cfg::Config;
pub use err::Error;
