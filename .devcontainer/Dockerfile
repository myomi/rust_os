FROM rustlang/rust:nightly

ENV IMAGE_NAME=rust_os

# setting for use Qemu from X Window server
ENV DISPLAY host.docker.internal:0.0

RUN rustup update
RUN rustup component add rls rust-analysis rust-src llvm-tools-preview

# Install tools
RUN apt-get update \
    && apt-get install -q -y --no-install-recommends \
    git \
    procps \
    nasm \
    binutils \
    grub-common \
    xorriso \
    grub-pc-bin \
    qemu \
    qemu-kvm

# Install other dependencies
RUN apt-get install -y lldb-3.9

# Install bin crates
RUN cargo install \
    xargo \
    cargo-xbuild \
    cargo-edit

RUN cargo install bootimage --version "^0.7.3"

# Clean up
RUN apt-get autoremove -q -y \
    && apt-get clean -q -y \
    && rm -rf /var/lib/apt/lists/*

# Set the default shell to bash instead of sh
ENV SHELL /bin/bash
