# leetcode-cli
![Rust](https://github.com/clearloop/leetcode-cli/workflows/Rust/badge.svg)
[![crate](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/leetcode-cli/)
[![downloads](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![gitter](https://img.shields.io/gitter/room/odditypark/leetcode-cli)](https://gitter.im/Odditypark/leetcode-cli)
[![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)

## Contributors
+ [@hulufi](https://github.com/hulufei)
+ [@ldm0](https://github.com/ldm0)
+ [@Raees678](https://github.com/Raees678)
+ [@clearloop](https://github.com/clearloop)

## Features

+ [x] the edit flow —— solution files will generate automatically!
+ [x] support Python script to filter questions
+ [ ] doc support, `lc-rs` can compile the annotation of your solutions to Markdown!
+ [ ] support local signal to keep coding as longer as you want

## Installing
```sh
cargo install leetcode-cli
```

### `error[E0554]`
If this happens when compiling the program, it means that the package cannot be compiled with stable Rust. To fix this, install Rust Nightly and try the following:
```sh
rustup install nightly
cargo +nightly install leetcode-cli
```

## Usage
**Make sure you have logged in to `leetcode.com` with `Chrome`**. See [Cookies](#cookies) for why you need to do this first.

```sh
leetcode 0.3.0
May the Code be with You 👻

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

For example, given this config (can be found in `~/.leetcode/leetcode.toml`):

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
# struct Solution;
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

```sh

  Success

  Runtime: 0 ms, faster than 100% of Rustonline submissions for Two Sum.

  Memory Usage: 2.4 MB, less than 100% of Rustonline submissions for Two Sum.


```

## Cookies

The cookie plugin of leetcode-cli can work on OSX and [Linux][#1]. **If you are on a different platform, there are problems with caching the cookies**, you can manually input your LeetCode Cookies to the configuration file.

```toml
[cookies]
csrf = "..."
session = "..."
```

For Example, using Chrome (after logging in to LeetCode):


#### Step 1

Open Chrome and navigate to the link below:

```sh
chrome://settings/cookies/detail?site=leetcode.com
```

#### Step 2

Copy `Content` from `LEETCODE_SESSION` and `csrftoken` to `session` and `csrf` in your configuration file, respectively:
```toml
[cookies]
csrf = "${csrftoken}"
session = "${LEETCODE_SESSION}"
```

## Programmable

If you want to filter LeetCode questions using custom Python scripts, add the following to your the configuration file:

```toml
[storage]
scripts = "scripts"
```

Then write the script:

```python
# ~/.leetcode/scripts/plan1.py
import json;

def plan(sps, stags):
    ##
    # `print` in python is supported, 
    # if you want to know the data structures of these two args, 
    # just print them
    ##
    problems = json.loads(sps)
    tags = json.loads(stags)
	
    ret = []
    tm = {}
    for tag in tags:
        tm[tag["tag"]] = tag["refs"];

    for i in problems:
        if i["level"] == 1 and str(i["id"]) in tm["linked-list"]:
            ret.append(str(i["id"]))

    # return is `List[string]`
    return ret
```

Then run `list` with the filter that you just wrote:

```sh
leetcode list -p plan1
```

And that's it! Enjoy!


## PR
[PRs][pr] are more than welcome!

## LICENSE
MIT


[pr]: https://github.com/clearloop/leetcode-cli/pulls
[#1]: https://github.com/clearloop/leetcode-cli/issues/1
