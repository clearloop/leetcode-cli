# Configuration

leetcode-cli reads `~/.leetcode/leetcode.toml`. The file is created with defaults the first time you run any command, so the easiest way to start is to run leetcode-cli once and then edit the file.

It has three sections: `[code]`, `[cookies]`, and `[storage]`. Cookies have their own page — see [Cookies](./cookies.md).

## `[code]`

Controls the editor that opens and the code that gets generated.

```toml
[code]
editor = 'vim'
# Extra arguments passed to the editor
editor_args = ['-nw']
# Environment variables for the editor process, as "NAME=VALUE" strings
editor_envs = ['XDG_CONFIG_HOME=/home/me/.config']
# Language of the generated solution file
lang = 'rust'
# Run the sample test cases automatically and write them to a .tests.dat file
test = true
```

| Key | Default | Description |
| --- | --- | --- |
| `editor` | `'vim'` | Command used to open the solution file. |
| `editor_args` | — | Extra arguments passed before the file path. |
| `editor_envs` | — | Environment variables for the editor process, each as `"NAME=VALUE"`. |
| `lang` | `'rust'` | Language of the generated file. `leetcode edit --lang <lang>` overrides this per-call and persists it. |
| `test` | `true` | Generate a `.tests.dat` file with the problem's sample cases. |

### Problem description &amp; code markers

These let you embed the problem statement as a comment and wrap the editable region in markers so tools (or `inject_*`) can find it.

```toml
[code]
# Prepend the problem description as a comment
comment_problem_desc = true
# Comment syntax used for the description and the markers
comment_leading = '//'
# Wrap the generated boilerplate in start/end markers
edit_code_marker = true
start_marker = 'start_marker'
end_marker = 'end_marker'
```

With the above, `leetcode edit 15` produces:

```rust
// Category: algorithms
// Level: Medium
//
// Given an integer array nums, return all the triplets ...

// start_marker
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {

    }
}
// end_marker
```

| Key | Default | Description |
| --- | --- | --- |
| `comment_problem_desc` | `false` | Prepend the problem statement as a comment. |
| `comment_leading` | `''` | Comment prefix used for the description and markers (e.g. `//`, `#`). |
| `edit_code_marker` | `false` | Wrap the generated code in `start_marker`/`end_marker`. |
| `start_marker` / `end_marker` | `''` | Marker text, emitted as `comment_leading + ' ' + marker`. |

### Injecting code before &amp; after

Some language servers and linters complain unless the necessary imports are present, and you may want a `main` to run the solution locally. `inject_before` and `inject_after` add lines around the generated boilerplate without affecting what gets submitted to LeetCode.

```toml
[code]
inject_before = ['#include<bits/stdc++.h>', 'using namespace std;']
inject_after = ['int main() {\n    Solution solution;\n}']
```

For getting a language server working with generated files, see [Editors & LSP](./editors.md).

### Filename template

`pick` controls the file name of generated solution and test files. It is interpolated against the picked problem:

| Variable | Meaning |
| --- | --- |
| `${fid}` | Frontend problem id (e.g. `1`) |
| `${slug}` | Problem slug (e.g. `two-sum`) |

```toml
[code]
# Default
pick = '${fid}.${slug}'
```

With `lang = 'rust'`, `leetcode edit 1` writes `code/1.two-sum.rs` (and `code/1.two-sum.tests.dat` when `test = true`). Because the template can include `/`, it doubles as a way to lay out one directory per problem — see [Editors & LSP](./editors.md).

## `[storage]`

Where leetcode-cli keeps its files. Paths under `root` are created on demand. `~` in `root` expands to your home directory.

```toml
[storage]
root = '~/.leetcode'
code = 'code'
scripts = 'scripts'
```

| Key | Default | Description |
| --- | --- | --- |
| `root` | `'~/.leetcode'` | Base directory for everything below. |
| `code` | `'code'` | Sub-directory (under `root`) for generated solution files. |
| `scripts` | `'scripts'` | Sub-directory for Python filtering scripts — see [Scripting](./scripting.md). |

## `[cookies]`

See [Cookies](./cookies.md) for the full story (automatic Chrome reading, manual setup, `leetcode.cn`, and environment overrides).

```toml
[cookies]
csrf = ''
session = ''
# Either 'leetcode.com' or 'leetcode.cn'
site = 'leetcode.com'
```
