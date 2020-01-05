# leetcode-cli
[![doc](https://img.shields.io/badge/0.1.8-docs-green.svg)](https://docs.rs/leetcode-cli/)
[![Crates.io](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![Crates.io](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)

## Note - Not Available for Now
> (only support OSX temporarily) [#1][#1]

Please make sure you have logined in `leetcode.com` with `chrome`.


## Features

1. the edit flow —— solution files will generate automatically!
2. doc support, `lc-rs` can compile the annotation of your solutions to markdown!
   1. btw, generate a site is easy for `lc-rs`!
3. support local signal to keep coding as longer as you want.


## Building

```
cargo install leetcode-cli
```


## Usage
```sh
leetcode 0.1.9
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
[storage]
code = "code"

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

## PR

PR is welcome, [here][pr] it is.

## LICENSE
MIT


[pr]: https://github.com/clearloop/leetcode-cli/pulls
[#1]: https://github.com/clearloop/leetcode-cli/issues/1
