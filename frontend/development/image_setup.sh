#!/bin/zsh

# Donwload with default options
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Adding cargo to path to allow installation 
export PATH="/root/.cargo/bin:/$PATH"

# Base
cargo install cargo-update cargo-edit 
cargo install cargo-generate cargo-feature cargo-info
#cargo install cargo-pgo cargo-profiler cargo-script 
# CLI 
cargo install bat clap du-dust eza fd-find hyperfine ripgrep zoxide

