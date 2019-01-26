#!/bin/bash
cd malachite-base &&
cargo update &&
cargo fmt &&
cargo clippy
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-nz &&
cargo update &&
cargo fmt &&
#cargo +nightly clippy
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cargo build --release --target wasm32-unknown-unknown &&
cargo build --release --features 64_bit_limbs --no-default-features --target wasm32-unknown-unknown &&
cd ../malachite-test &&
cargo update &&
cargo fmt &&
#cargo +nightly clippy
cargo test --release &&
cargo test --release --features 64_bit_limbs --no-default-features &&
#cargo run --release -- exhaustive 100000 all &&
cargo rustc --release --lib -- --emit asm &&
cd .. &&
python additional-lints.py
