# Clean-room build of leetcode-cli, proving SQLite is bundled (no system libsqlite3).
#
# This mirrors a distro/AUR makepkg build — `cargo build --locked --release
# --all-features` — in an environment that has NO `libsqlite3-dev` installed. It
# succeeds because `libsqlite3-sys` compiles SQLite from its bundled amalgamation,
# so the binary needs no system SQLite at build time or run time.
#
#   docker build -f clean-build.Dockerfile -t leetcode-clean .
#
# If this succeeds but a packaging build fails with `undefined symbol: sqlite3_*`,
# the difference is in that build environment (e.g. LIBSQLITE3_SYS_USE_PKG_CONFIG
# set, or a `[target.*.sqlite3]` cargo build-script override), not the crate.
FROM rust:slim-bookworm

# Build-time deps only. Deliberately NO libsqlite3-dev — SQLite is bundled.
#   build-essential -> C toolchain + binutils (compiles the bundled sqlite3.c)
#   cmake           -> aws-lc-sys (rustls TLS backend)
#   python3-dev     -> pyo3 (the `pym` feature, enabled by --all-features)
#   git             -> rookie's build script reads the commit hash
RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential cmake python3-dev git \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /src
COPY . .

# rookie's build script runs `git rev-parse HEAD`; ensure that works in the image.
RUN git rev-parse HEAD >/dev/null 2>&1 \
    || (git init -q && git add -A \
        && git -c user.email=ci@local -c user.name=ci commit -qm clean-build)

RUN cargo build --locked --release --all-features

# Prove it: sqlite3 symbols are baked into the binary, and there is no dynamic
# dependency on a system libsqlite3.
RUN nm target/release/leetcode | grep -q ' T sqlite3_step' \
    && ! ldd target/release/leetcode | grep -qi sqlite \
    && echo "OK: SQLite is statically bundled; no system libsqlite3 required"
