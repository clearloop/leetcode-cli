# leetcode-cli
[![doc](https://img.shields.io/badge/0.2.0-docs-green.svg)](https://docs.rs/leetcode-cli/)
[![Crates.io](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![Crates.io](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)

## Cookies

The cookie plugin of leetcode-cil can work on OSX and [Linux][#1], If you are on other platforms or your cookies just don't want to be catched, you can **handwrite your LeetCode Cookies to `~/.leetcode/leetcode.toml`**

```toml
# Make sure `leetcode.toml` file is placed at `~/.leetcode/leetcode.toml`
[cookies]
csrf = "..."
session = "..."
```

### How to find LeetCode Cookies?

For Example, if you're using chrome to login to leetcode.com.


#### Step 1

Open chrome and paste the link below to the `chrome linkbar`.

```sh
chrome://settings/cookies/detail?site=leetcode.com
```

#### Step 2

Copy the contents of `LEETCODE_SESSION` and `csrftoken`.

#### Step 3

Paste them to `session` and `csrf`.

```toml
# Make sure `leetcode.toml` file is placed at `~/.leetcode/leetcode.toml`
[cookies]
csrf = "${LEETCODE_SESSION}"
session = "${csrf}"
```



## Building

```sh
cargo install leetcode-cli
```

## Usage

Please make sure you have logined in `leetcode.com` with `chrome`, more info plz checkout [this](#cookies)

```sh
leetcode 0.2.1
clearloop <udtrokia@163.com>
Here's to the crazy ones 👻

USAGE:
    leetcode [FLAGS] [SUBCOMMAND]

FLAGS:
    -d, --debug      debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    data    Manage Cache [aliases: d]
    edit    Edit question by id [aliases: e]
    exec    Submit solution [aliases: x]
    list    List problems [aliases: l]
    pick    Pick a problem [aliases: p]
    stat    Show simple chart about submissions [aliases: s]
    test    Edit question by id [aliases: t]
    help    Prints this message or the help of the given subcommand(s)
```

## Example

For example, if your config is:

```toml
[code]
lang = "rust"
editor = "emacs"
```

#### 1. <kbd>pick</kbd>

```sh
leetcode pick 1
```

```sh
[1] Two Sum is on the run...


Given an array of integers, return indices of the two numbers such that they add up to a specific target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

--------------------------------------------------

Example:


Given nums = [2, 7, 11, 15], target = 9,

Because nums[0] + nums[1] = 2 + 7 = 9,
return [0, 1].
```

#### 2. <kbd>edit</kbd>

```sh
leetcode edit 1
```

```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut m: HashMap<i32, i32> = HashMap::new();

        for (i, e) in nums.iter().enumerate() {
            if let Some(v) = m.get(&(target - e)) {
                return vec![*v, i as i32];
            }

            m.insert(*e, i as i32).unwrap_or_default();
        }

        return vec![];
    }
}
```

#### 3. <kbd>test</kbd>

```sh
leetcode test 1
```

```sh

  Accepted       Runtime: 0 ms

  Your input:    [2,7,11,15], 9
  Output:        [0,1]
  Expected:      [0,1]

```

#### 4. <kbd>submit</kbd>

```sh
leetcode submit 1
```

```

  Success

  Runtime: 0 ms, faster than 100% of Rustonline submissions for Two Sum.

  Memory Usage: 2.4 MB, less than 100% of Rustonline submissions for Two Sum.


```

## Features

+ [x] the edit flow —— solution files will generate automatically!
+ [ ] doc support, `lc-rs` can compile the annotation of your solutions to markdown!
   + [ ] btw, generate a site is easy for `lc-rs`!
+ [ ]  support local signal to keep coding as longer as you want.

## PR

PR is welcome, [here][pr] it is.

## LICENSE
MIT


[pr]: https://github.com/clearloop/leetcode-cli/pulls
[#1]: https://github.com/clearloop/leetcode-cli/issues/1
