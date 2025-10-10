# rustup / toolchain (global)

```powershell
# Maintain your toolchains
rustup update
rustup toolchain list
rustc --version
cargo --version

# Local docs in browser (std, book, etc.)
rustup doc
```

# single-crate workflow (from yelra repo root)

```powershell
# Fast UX
cargo check                  # type-check only (fast)
cargo run                    # build + run (debug)
cargo build -r               # release build (optimized)

# Code hygiene
cargo fmt --all              # format
cargo clippy --all-targets --all-features   # lints

# Clean up build artifacts
cargo clean
```

# workspace workflow (if/when you split into yelcore + ylc)

```powershell
# Build everything in the workspace
cargo build
cargo build -r

# Only the CLI / REPL (ylc)
cargo build -p ylc
cargo run   -p ylc
cargo build -p ylc -r
```

# handy flags you’ll actually use

```powershell
# More output while linking/debugging
cargo build -vv

# Build only current package even in a workspace
cargo build --package yelra

# Run a specific binary if a crate has multiple [[bin]] targets
cargo run --bin yelra

# Feature gates (when you add them)
cargo run --features=experimental
cargo build --no-default-features

# Target release but keep debug symbols (good for profiling on Windows)
RUSTFLAGS="-C debuginfo=1" cargo build -r

# Open docs for a crate dependency (uses local cache)
cargo doc --open

# Time where your compile is going (rustc internal timings)
RUSTC_BOOTSTRAP=1 RUSTFLAGS="-Z time-passes" cargo build   # nightly only
```

# quick sanity loop (what I actually type)

```powershell
# During active edits
cargo fmt --all && cargo clippy --all-targets --all-features && cargo check

# When testing behavior
cargo run

# Before committing or tagging
cargo clean && cargo build -r && cargo test
```

# optional project niceties

```toml
# rust-toolchain.toml (pin compiler + tools for reproducibility)
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

```gitignore
# .gitignore
/target
**/*.rs.bk
```

---

# Git Diff

git --no-pager diff HEAD > gitdiff

