# leetcode-cli

![Rust](https://github.com/clearloop/leetcode-cli/workflows/leetcode-cli/badge.svg)
[![crate](https://img.shields.io/crates/v/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![doc](https://img.shields.io/badge/current-docs-brightgreen.svg)](https://docs.rs/leetcode-cli/)
[![downloads](https://img.shields.io/crates/d/leetcode-cli.svg)](https://crates.io/crates/leetcode-cli)
[![telegram](https://img.shields.io/badge/telegram-blue?logo=telegram)](https://t.me/+U_5si6PhWykxZTI1)
[![LICENSE](https://img.shields.io/crates/l/leetcode-cli.svg)](https://choosealicense.com/licenses/mit/)

Leet your code in the command-line.

## Install

```sh
# Required dependencies: gcc, libssl-dev, libdbus-1-dev, libsqlite3-dev
cargo install leetcode-cli
```

Python filtering scripts (the `--plan` flag) require the optional `pym` feature:

```sh
cargo install leetcode-cli --features pym
```

Nix users can `nix build` / `nix develop` against the bundled [`flake.nix`](./flake.nix).

## Quickstart

Sign in to LeetCode in **Chrome** first — leetcode-cli reads its cookies automatically (see [Cookies](./docs/cookies.md) for other browsers, manual setup, and environment variables).

```sh
leetcode pick 1        # pick a problem and print its description
leetcode edit 1        # open the solution file in your editor
leetcode test 1        # run the sample test cases
leetcode exec 1        # submit the solution
```

Run `leetcode --help` (or `leetcode <command> --help`) for the full, always-current list of commands and flags. The headline ones:

| Command | Alias | What it does |
| --- | --- | --- |
| `pick` | `p` | Pick a problem by id, `--name`, `--tag`, `--query`, `--plan`, or `--daily` |
| `edit` | `e` | Open a problem's code file; `--lang` overrides the configured language, `--daily` opens today's challenge |
| `test` | `t` | Run test cases; `--watch` re-runs on save, `--daily` targets today's challenge |
| `exec` | `x` | Submit the solution |
| `list` | `l` | List/filter problems by category, tag, id range, or `--query` |
| `stat` | `s` | Show a chart of your submissions |
| `data` | `d` | Manage the local cache (`--update`, `--delete`) |
| `completions` | `c` | Generate shell completions (`bash`, `elvish`, `fish`, `powershell`, `zsh`) |

<details>
<summary>Shell completions</summary>

By default the shell is inferred from `$SHELL`:

```sh
eval "$(leetcode completions)"
```

Copy that line into `.bash_profile` or `.zshrc`. Pass a shell explicitly to target another:

```sh
leetcode completions fish
```

</details>

## Documentation

- [Configuration](./docs/configuration.md) — the full `leetcode.toml` reference: editor, code generation, filename templates, and storage paths.
- [Cookies](./docs/cookies.md) — automatic Chrome cookies, manual setup from any browser, `leetcode.cn` support, and environment-variable overrides.
- [Editors & LSP](./docs/editors.md) — getting rust-analyzer (and other language servers) working with generated solution files.
- [Scripting](./docs/scripting.md) — filtering problems with custom Python plans.

## Contributing

Feel free to add your name and email to the `authors` field of `Cargo.toml`, and open a [pull request][pr].

## License

MIT

[pr]: https://github.com/clearloop/leetcode-cli/pulls
