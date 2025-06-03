#! /bin/bash

cargo test
cargo fmt && cargo clippy
cargo build --release
cd contracts && npm install && cd ..