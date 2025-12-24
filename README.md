# leetcode-cli

![Rust](https://github.com/clearloop/leetcode-cli/workflows/leetcode-cli/badge.svg)
[![crate](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/leetcode-cli/)
[![downloads](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![telegram](https://img.shields.io/badge/telegram-blue?logo=telegram)](https://t.me/+U_5si6PhWykxZTI1)
[![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)

## Installing

```sh
# Required dependencies:
#
#  gcc
#  libssl-dev
#  libdbus-1-dev
#  libsqlite3-dev

cargo install leetcode-cli
```

<details>
<summary>Shell completions</summary>

For Bash and Zsh (by default picks up `$SHELL` from environment)

```sh
eval "$(leetcode completions)"
```

Copy the line above to `.bash_profile` or `.zshrc`

You may also obtain specific shell configuration using.

```sh
leetcode completions fish
```

If no argument is provided, the shell is inferred from the `SHELL` environment variable.

</details>

## Usage

**Make sure you have logged in to `leetcode.com` with `Firefox`**. See [Cookies](#cookies) for why you need to do this first.

```sh
leetcode 0.4.0
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
    test    Test question by id [aliases: t]
    help    Prints this message or the help of the given subcommand(s)
```

## Example

To configure leetcode-cli, create a file at `~/.leetcode/leetcode.toml`):

```toml
[code]
editor = 'emacs'
# Optional parameter
editor_args = ['-nw']
# Optional environment variables (ex. [ "XDG_DATA_HOME=...", "XDG_CONFIG_HOME=...", "XDG_STATE_HOME=..." ])
editor_envs = []
lang = 'rust'
edit_code_marker = false
start_marker = ""
end_marker = ""
# if include problem description
comment_problem_desc = false
# comment syntax
comment_leading = ""
test = true

[cookies]
csrf = '<your-leetcode-csrf-token>'
session = '<your-leetcode-session-key>'
# leetcode.com or leetcode.cn
site = "leetcode.com"

[storage]
cache = 'Problems'
code = 'code'
root = '~/.leetcode'
scripts = 'scripts'
```

<details>
  <summary>Configuration Explanation</summary>

```toml
[code]
editor = 'emacs'
# Optional parameter
editor_args = ['-nw']
# Optional environment variables (ex. [ "XDG_DATA_HOME=...", "XDG_CONFIG_HOME=...", "XDG_STATE_HOME=..." ])
editor_envs = []
lang = 'rust'
edit_code_marker = true
start_marker = "start_marker"
end_marker = "end_marker"
# if include problem description
comment_problem_desc = true
# comment syntax
comment_leading = "//"
test = true

[cookies]
csrf = '<your-leetcode-csrf-token>'
session = '<your-leetcode-session-key>'

[storage]
cache = 'Problems'
code = 'code'
root = '~/.leetcode'
scripts = 'scripts'
```

If we change the configuration as shown previously, we will get the following code after `leetcode edit 15`.

```rust
// Category: algorithms
// Level: Medium
// Percent: 32.90331%

// Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
//
// Notice that the solution set must not contain duplicate triplets.
//
//  
// Example 1:
//
// Input: nums = [-1,0,1,2,-1,-4]
// Output: [[-1,-1,2],[-1,0,1]]
// Explanation:
// nums[0] + nums[1] + nums[2] = (-1) + 0 + 1 = 0.
// nums[1] + nums[2] + nums[4] = 0 + 1 + (-1) = 0.
// nums[0] + nums[3] + nums[4] = (-1) + 2 + (-1) = 0.
// The distinct triplets are [-1,0,1] and [-1,-1,2].
// Notice that the order of the output and the order of the triplets does not matter.
//
//
// Example 2:
//
// Input: nums = [0,1,1]
// Output: []
// Explanation: The only possible triplet does not sum up to 0.
//
//
// Example 3:
//
// Input: nums = [0,0,0]
// Output: [[0,0,0]]
// Explanation: The only possible triplet sums up to 0.
//
//
//  
// Constraints:
//
//
// 3 <= nums.length <= 3000
// -10⁵ <= nums[i] <= 10⁵
//

// start_marker
impl Solution {
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

    }

}
// end_marker

```

</details>

## Language-Specific Configuration

### Rust
For `lang = 'rust'`, leetcode-cli generates per-problem crate structures to enable full LSP support (e.g., rust-analyzer in editors like Helix or VS Code).

- **Structure**: `code/{fid}-{slug}/src/lib.rs` (code), `tests.dat` (test cases), and `Cargo.toml` (basic crate config with commented dependencies for common crates like `itertools` or `regex`).
- **Example**: For problem 1 ("Two Sum"), creates `code/1-two_sum/` with `prob-1-two_sum` as package name.
- **Config Option**: Set `enable_rust_crates = false` in `[code]` to fall back to flat files (e.g., `1.two-sum.rs`).
- **Usage**: Run `leetcode edit 1`, then open the dir: `hx code/1-two_sum/` for LSP features (autocomplete, diagnostics, etc.).
- **Migration**: If flat files exist, the tool notes them—manually move content to `lib.rs` if needed.
- **Local Testing**: Edit `Cargo.toml` to add deps, then `cargo check` or `cargo test` (tests.dat can be adapted for unit tests).

This keeps submissions unchanged (sends code snippet to LeetCode API) while improving local editing.

For other languages, files remain flat. Please contribute if needed!

<br>

Some linting tools/lsps will throw errors unless the necessary libraries are imported. leetcode-cli can generate this boilerplate automatically if the `inject_before` key is set. Similarly, if you want to test out your code locally, you can automate that with `inject_after`. For c++ this might look something like:

```toml
[code]
inject_before = ["#include<bits/stdc++.h>", "using namespace std;"]
inject_after = ["int main() {\n    Solution solution;\n\n}"]
```

#### 1. <kbd>pick</kbd>

```sh
leetcode pick 1
```

```sh
leetcode pick --name "Two Sum"
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

#### 4. <kbd>exec</kbd>

```sh
leetcode exec 1
```

```sh

  Success

  Runtime: 0 ms, faster than 100% of Rustonline submissions for Two Sum.

  Memory Usage: 2.4 MB, less than 100% of Rustonline submissions for Two Sum.


```

## Cookies

The cookie plugin of leetcode-cli can work on OSX and [Linux][#1]. **If you are on a different platform, there are problems with caching the cookies**,
you can manually input your LeetCode Cookies to the configuration file.

```toml
[cookies]
csrf = "..."
session = "..."
```

For Example, using Firefox (after logging in to LeetCode):

#### Step 1

Open Firefox, press F12, and click `Storage` tab.

#### Step 2

Expand `Cookies` tab on the left and select https://leetcode.com.

#### Step 2

Copy `Value` from `LEETCODE_SESSION` and `csrftoken` to `session` and `csrf` in your configuration file, respectively:

```toml
[cookies]
csrf = '<your-leetcode-csrf-token>'
session = '<your-leetcode-session-key>'
```

#### Environment variables

The cookies can also be overridden by environment variables, which might be useful to exclude the sensitive information from the configuration file `leetcode.toml`. To do this, you can leave the `csrf` and `session` fields empty in the configuration file and override cookies settings via the environment variables `LEETCODE_CSRF`, `LEETCODE_SESSION`, and `LEETCODE_SITE`:

```toml
[cookies]
csrf = ''
session = ''
site = 'leetcode.com'
```

Then set the environment variables:

```bash
export LEETCODE_CSRF='<your-leetcode-csrf-token>'
export LEETCODE_SESSION='<your-leetcode-session-key>'
export LEETCODE_SITE='leetcode.cn' # or 'leetcode.com'
```

Note that `cookies.site` in still required in the `leetcode.toml` to avoid exception during configuration file parsing, but can be overridden using environment variables.

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

That's it! Enjoy!

## Contributions

Feel free to add your names and emails in the `authors` field of `Cargo.toml` !

## LICENSE

MIT

[pr]: https://github.com/clearloop/leetcode-cli/pulls
[#1]: https://github.com/clearloop/leetcode-cli/issues/1
