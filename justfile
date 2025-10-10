################################################################################################
# File: justfile
################################################################################################

# Use PowerShell on Windows, sh elsewhere (keeps things truly cross-platform)
# The conditional form (inline `if ... { ... } else { ... }`) is not supported by
# some `just` versions and causes a parse error like:
#   error: Expected '[', but found identifier
# To remain broadly compatible, pick one of the explicit shells below and
# uncomment it for your platform.
#
# Windows PowerShell Core:
# set shell := ["pwsh", "-NoProfile", "-Command"]
#
# If you only have classic PowerShell (powershell.exe):
set shell := ["powershell", "-NoProfile", "-Command"]
#
# POSIX shell (uncomment on Linux/macOS):
# set shell := ["sh", "-cu"]

# Default recipe
default: dev

# ---------- Dev loop ----------

# Full dev loop: format → lint → type-check → run
dev: fmt clippy check
    cargo run

# ---------- Single-crate workflow ----------

fmt:
    cargo fmt --all

clippy:
    cargo clippy --all-targets --all-features -- -D warnings

check:
    cargo check

run:
    cargo run

release:
    cargo build -r

clean:
    cargo clean

doc:
    cargo doc --open

# ---------- Workspace workflow (if/when you add yelcore + ylc) ----------

build-all:
    cargo build

release-all:
    cargo build -r

ylc-build:
    cargo build -p ylc

ylc-run:
    cargo run -p ylc

ylc-release:
    cargo build -p ylc -r

# ---------- Handy flags ----------

build-verbose:
    cargo build -vv

build-package:
    cargo build --package yelra

run-bin:
    cargo run --bin yelra

run-experimental:
    cargo run --features=experimental

build-nodefaults:
    cargo build --no-default-features

# Release with debug symbols (profiling-friendly on Windows)
release-with-debug:
    RUSTFLAGS="-C debuginfo=1" cargo build -r

# ---------- rustup / toolchain -----------

toolchain-update:
    rustup update

toolchain-list:
    rustup toolchain list

rustc-version:
    rustc --version

cargo-version:
    cargo --version

rust-docs:
    rustup doc
