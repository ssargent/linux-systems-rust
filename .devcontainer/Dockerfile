FROM mcr.microsoft.com/devcontainers/base:ubuntu

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    build-essential clang gdb lldb strace lsof procps \
    iproute2 net-tools iputils-ping dnsutils man-db \
    pkg-config git curl wget ca-certificates && \
    rm -rf /var/lib/apt/lists/*

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:${PATH}

RUN mkdir -p ${RUSTUP_HOME} ${CARGO_HOME} && \
    chown -R vscode:vscode ${RUSTUP_HOME} ${CARGO_HOME}

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain stable && \
    rustup component add rustfmt clippy && \
    cargo install just --locked && \
    chown -R vscode:vscode ${RUSTUP_HOME} ${CARGO_HOME}
