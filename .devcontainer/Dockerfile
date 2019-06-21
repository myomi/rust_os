FROM rustlang/rust:nightly

ENV IMAGE_NAME=rust_os

RUN rustup update
RUN rustup component add rls rust-analysis rust-src

# Install tools
RUN apt-get update \
    && apt-get install -q -y --no-install-recommends \
    git \
    procps \
    nasm \
    binutils \
    grub-common \
    xorriso \
    grub-pc-bin

# Install other dependencies
RUN apt-get install -y lldb-3.9

# Install bin crates
RUN cargo install \
    xargo \
    cargo-edit

# Clean up
RUN apt-get autoremove -q -y \
    && apt-get clean -q -y \
    && rm -rf /var/lib/apt/lists/*

# Set the default shell to bash instead of sh
ENV SHELL /bin/bash
