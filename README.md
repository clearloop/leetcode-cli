# leetcode-cli
[![doc](https://img.shields.io/badge/0.1.3-docs-green.svg)](https://docs.rs/leetcode-cli/)
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

**What's the difference between `lc-rs`(this repo) and skygragon's [leetcode-cli](https://github.com/skygragon/leetcode-cli)?**

Here two features in this `lc-rs`:

1. the edit flow —— solution file will generate automatically!
2. doc support, `lc-rs` can compile the annotation of your solutions to markdown!
   1. btw, generate a site is easy for `lc-rs`!

For example, if your config is:

```toml
[storage]
code = "code"

[code]
lang = "rust"
editor = "emacs"
```

After pick a question:

```
leetcode pick 1
```

`lc-rs` will generate `1.two-sum.alogrithms` at `~/.leetcode/code/1.two-sum.algorithms.rs`

And you want to edit it, so:

```
leetcode edit 1
```

Emacs will be with you, and then, test and submit is just:


```
leetcode test 1
leetcode submit 1
```

Enjoy Coding!

## RoadMap

[ROADMAP](/ROADMAP.md)

## Building

```
cargo install leetcode-cli
```

## RoadMap

<kbd>list</kbd> before the first weekend of 2020.

## Usage
```sh
leetcode 0.1.4
clearloop <udtrokia@163.com>
Leet your code in command-line.

USAGE:
    leetcode [FLAGS] [SUBCOMMAND]

FLAGS:
    -d, --debug      debug mode
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    list    List problems [aliases: ls]
```

### leetcode-list
```
leetcode-list 
List problems

USAGE:
    leetcode list [FLAGS] [OPTIONS] [keyword]

FLAGS:
    -h, --help       Prints help information
    -s, --stat       Show statistics of listed problems
    -V, --version    Prints version information

OPTIONS:
    -c, --category <category>    Fliter problems by category name
                                 [alogrithms, database, shell]
    -q, --query <query>          Fliter questions by conditions:
                                 Uppercase means negative
                                 e = easy     E = m+h
                                 m = medium   M = e+h
                                 h = hard     H = e+m
                                 d = done     D = not done
                                 l = locked   L = not locked
                                 s = starred  S = not starred

ARGS:
    <keyword>    Keyword in select query

EXAMPLES:
    leetcode list               List all questions
    leetcode list array         List questions that has "array" in name
    leetcode list -c database   List questions that in database category
    leetcode list -q eD         List questions that with easy level and not done
```

## LICENSE
MIT
