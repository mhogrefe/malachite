#!/bin/bash
cd malachite-base &&
cargo update &&
cargo +nightly fmt &&
#cargo +nightly clippy
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-nz &&
cargo update &&
cargo +nightly fmt &&
#cargo +nightly clippy
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-test &&
cargo update &&
cargo +nightly fmt &&
#cargo +nightly clippy
cargo test --release &&
#cargo run --release -- exhaustive 100000 all &&
cargo rustc --release --lib -- --emit asm
