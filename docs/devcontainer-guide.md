# Devcontainer Guide

This repository includes a VS Code devcontainer for running the course workspace in Linux even when your host machine is macOS.

## What the devcontainer is for

Use the devcontainer when you want Linux behavior for systems programming exercises, debugging, networking, or process inspection. The container is the safest default place to run anything that depends on Linux-specific runtime details.

The container setup in this repo includes:

- Ubuntu as the base environment
- Rust stable
- `just`
- `rustfmt` and `clippy`
- `gdb`, `lldb`, `strace`, `lsof`, and common networking tools
- extra container permissions for debugger-friendly workflows such as `ptrace`

## Open the repo in the devcontainer

### VS Code

1. Install Docker.
2. Install the VS Code extension `Dev Containers`.
3. Open this repository in VS Code.
4. Run `Dev Containers: Reopen in Container` from the command palette.

VS Code will build the image from [.devcontainer/Dockerfile](/Users/scott/source/github/ssargent/linux-systems-rust/.devcontainer/Dockerfile) and apply the settings in [.devcontainer/devcontainer.json](/Users/scott/source/github/ssargent/linux-systems-rust/.devcontainer/devcontainer.json).

### GitHub Codespaces

If you open the repo in a Codespace, the same devcontainer configuration should be used automatically.

## What happens after startup

The workspace is mounted into the container, so editing files in VS Code updates the repo directly. Your terminal inside VS Code is also running inside the container, which means commands like `cargo`, `just`, `strace`, and `ip` execute against the Linux environment instead of your host OS.

The container uses the `vscode` user, and a post-create step installs the Rust formatting and linting components.

## First commands to run

From a terminal inside the container:

```bash
uname -a
rustc --version
just
just check
just test
```

If you want the full local verification pass:

```bash
just ci
```

## Daily workflow

- Edit code normally in the repo.
- Run Rust commands from the integrated terminal inside the container.
- Use `just` recipes as the main entry point for common tasks.
- Use `cargo run -p ...` or `just chapter07` style recipes when working on a specific crate.

Examples:

```bash
just syslab --help
just chapter03
just run capstone
```

## When to rebuild the container

Rebuild the container if:

- [.devcontainer/Dockerfile](/Users/scott/source/github/ssargent/linux-systems-rust/.devcontainer/Dockerfile) changes
- [.devcontainer/devcontainer.json](/Users/scott/source/github/ssargent/linux-systems-rust/.devcontainer/devcontainer.json) changes
- tool installation inside the container looks broken or out of sync

In VS Code, run `Dev Containers: Rebuild Container`.

## Troubleshooting

### `just` is not found

Rebuild the container. The image now installs `just`, so this usually means the current container was created before that change or the build did not complete successfully.

### Cargo reports permission denied under `/usr/local/cargo`

Rebuild the container. Older images could leave Cargo's cache owned by `root`, while the workspace runs as the `vscode` user.

### Debuggers or `strace` behave differently on macOS

That is expected. Run those workflows inside the devcontainer so they execute on Linux.

### A crate works on macOS but not in the devcontainer

Prefer the devcontainer result for Linux-targeted exercises. See [macos-vs-linux.md](/Users/scott/source/github/ssargent/linux-systems-rust/docs/macos-vs-linux.md) for the platform note.
