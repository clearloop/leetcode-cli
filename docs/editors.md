# Editors & LSP

leetcode-cli generates flat solution files (e.g. `code/1.two-sum.rs`). That keeps submissions simple, but a language server like rust-analyzer expects a project it recognizes. Getting full autocomplete, diagnostics, and go-to-definition is an **editor-side** setup, not something leetcode-cli does for you — and there are two clean ways to do it.

This page uses Rust as the running example; the same ideas apply to other languages and their servers.

## Option A — single-file mode (recommended)

rust-analyzer supports [detached / single files](https://github.com/rust-lang/rust-analyzer/pull/8955) that don't belong to a Cargo workspace. This needs no extra files and no change to how leetcode-cli generates code — just tell your editor to treat the directory as detached.

- **VS Code** — open the `code/` directory; rust-analyzer picks up detached files automatically. You can pin them with `rust-analyzer.linkedProjects` if needed.
- **Neovim / other LSP clients** — point the rust-analyzer root at the `code/` directory (e.g. set the workspace root to the folder rather than searching for a `Cargo.toml`).
- **Helix / Zed** — make sure rust-analyzer is launched with the `code/` directory as its root; both honor rust-analyzer's detached-file handling.

If your standard library completions don't show up, confirm `rustc` and `rust-analyzer` are recent — detached-file support has improved over time.

## Option B — a Cargo workspace

If you'd rather have a real crate (so `cargo check`/`cargo test` work too), put a workspace around the generated files and `include!` them. leetcode-cli's existing config does all the wiring — no patches required.

1. Create a workspace and point leetcode-cli's storage at it:

   ```sh
   cargo new --lib leetcode-solutions
   ```

   ```toml
   # ~/.leetcode/leetcode.toml
   [storage]
   root = '/path/to/leetcode-solutions'
   code = 'solutions'
   ```

   Now `leetcode edit 1` writes into `leetcode-solutions/solutions/`, inside a crate rust-analyzer understands.

2. Pull the solution files into the crate from `src/lib.rs`:

   ```rust
   #![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

   mod solutions {
       include!("../solutions/1.two-sum.rs");
       include!("../solutions/2.add-two-numbers.rs");
   }
   ```

You get full rust-analyzer features across every file, and `leetcode test`/`leetcode exec` still extract and submit the solution snippet unchanged.

### Tip: one directory per problem

The [`[code] pick`](./configuration.md#filename-template) template can contain a `/`, so you can give each problem its own folder if your editor prefers that layout:

```toml
[code]
pick = '${fid}.${slug}/solution'
```

This writes `code/1.two-sum/solution.rs`, etc.

## Why leetcode-cli doesn't generate per-problem crates

Scaffolding a Cargo crate (or any per-language project structure) for every problem would bake an editor concern into the tool, with a config flag and a directory layout to maintain forever. The two approaches above solve the same problem entirely from the editor and existing config — so that's where it belongs. See the discussion in [#204](https://github.com/clearloop/leetcode-cli/issues/204).
