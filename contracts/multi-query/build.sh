#!/bin/bash
set -e

export RUSTFLAGS='-C link-arg=-s'

cargo fmt --all
cargo clippy -- -D warnings
cargo build --target wasm32-unknown-unknown --release