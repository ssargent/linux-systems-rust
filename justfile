set positional-arguments

default:
  @just --list

# Show available recipes.
list:
  @just --list

# Check the entire workspace.
check:
  cargo check --workspace

# Run tests for the entire workspace.
test:
  cargo test --workspace

# Format all Rust code.
fmt:
  cargo fmt --all

# Verify formatting without changing files.
fmt-check:
  cargo fmt --all -- --check

# Lint all targets in the workspace with warnings denied.
clippy:
  cargo clippy --workspace --all-targets -- -D warnings

# Local verification pass suitable for CI-style checks.
ci: fmt-check clippy test

# Run the syslab CLI. Extra args are forwarded after `--`.
syslab *args:
  cargo run -p syslab -- {{args}}

# Run the xtask helper. Extra args are forwarded after `--`.
xtask *args:
  cargo run -p xtask -- {{args}}

# Run a single package by name.
run package *args:
  cargo run -p {{package}} -- {{args}}

# Check a single package by name.
check-package package:
  cargo check -p {{package}}

# Test a single package by name.
test-package package:
  cargo test -p {{package}}

# Run an example from the workspace.
example name *args:
  cargo run --example {{name}} -- {{args}}

# Chapter convenience runners.
chapter01 *args:
  cargo run -p chapter01-file-io -- {{args}}

chapter02 *args:
  cargo run -p chapter02-processes -- {{args}}

chapter03 *args:
  cargo run -p chapter03-signals -- {{args}}

chapter04 *args:
  cargo run -p chapter04-ipc -- {{args}}

chapter05 *args:
  cargo run -p chapter05-threads -- {{args}}

chapter06 *args:
  cargo run -p chapter06-filesystems -- {{args}}

chapter07 *args:
  cargo run -p chapter07-networking -- {{args}}

capstone *args:
  cargo run -p capstone -- {{args}}
