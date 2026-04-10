# Linux Systems Rust Workspace

This repository is a Rust workspace for a Linux systems programming course.

## Layout

- `crates/syslib`: shared low-level helpers used across exercises.
- `crates/syslab`: integrated CLI for course exercise commands.
- `crates/chapter*`: chapter-focused binaries from file I/O through networking.
- `crates/capstone`: capstone project crate.
- `xtask`: project task runner for CI and environment checks.
- `docs/`: course plans, platform notes, and syscall study notes.
- `examples/`: focused runnable examples for common systems patterns.
- `testdata/`: fixtures for file trees and portability notes.
- `.devcontainer/`: Linux development container configuration.
- `.github/workflows/`: CI pipelines.

## Quick Start

```bash
cargo check
cargo run -p syslab -- --help
cargo run -p xtask -- --help
```
