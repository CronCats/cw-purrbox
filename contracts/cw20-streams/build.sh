#!/bin/bash
set -e

export RUSTFLAGS='-C link-arg=-s'

cargo fmt --all
cargo clippy -- -D warnings
cargo build --target wasm32-unknown-unknown --release
cargo schema

rm -rf res && mkdir res && cp target/wasm32-unknown-unknown/release/cw20_streams.wasm res/cw20_streams.wasm