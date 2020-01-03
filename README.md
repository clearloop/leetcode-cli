# leetcode-cli
[![doc](https://img.shields.io/badge/0.1.5-docs-green.svg)](https://docs.rs/leetcode-cli/)
[![Crates.io](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![Crates.io](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)

## Not Available for Now

If you need to, keep time on me...expect to launch at v0.3.0.

and,

the DEADLINE is `Sub Jan 5 23:59:59 CST 2020`.


## Note
> (only support OSX temporarily)

Please make sure you have logined in `leetcode.com` with `chrome`.

## Features

What's the difference between `lc-rs` and skygragon's [leetcode-cli][sky]?

**Here are three features in `lc-rs`**:

1. the edit flow —— solution file will generate automatically!
2. doc support, `lc-rs` can compile the annotation of your solutions to markdown!
   1. btw, generate a site is easy for `lc-rs`!
3. support local signal to keep coding as longer as you want.

## Example

For example, if your config is:

```toml
[storage]
code = "code"

[code]
lang = "rust"
editor = "emacs"
```

### 1. <kbd>pick</kbd> a question:

```
leetcode pick 1
```

`lc-rs` will generate `1.two-sum.alogrithms` at `~/.leetcode/code/1.two-sum.algorithms.rs`

### 2. <kbd>edit</kbd> it

```
leetcode edit 1
```

#### 3. The `emacs` will be with you

```
coding...
```

#### 4. <kbd>test</kbd> it.

```
leetcode test 1
```

#### 5. <kbd>submit</kbd> it

```
leetcode submit 1
```

Enjoy Coding!


## Building

```
cargo install leetcode-cli
```


## Usage
```sh
leetcode 0.1.6
clearloop <udtrokia@163.com>
Leet your code in command-line.

USAGE:
    leetcode [FLAGS] [SUBCOMMAND]

FLAGS:
    -d, --debug      debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    cache    Manage Cache [aliases: cc]
    list     List problems [aliases: ls]
    stat     Show simple chart about submissions [aliases: st]
    help     Prints this message or the help of the given subcommand(s)
```

## PR

PR is welcome to the repo, [here][pr] it is.

## LICENSE
MIT


[pr]: https://github.com/clearloop/leetcode-cli/pulls
[sky]: https://github.com/skygragon/leetcode-cli
