#!/bin/bash
cd malachite-base &&
cargo update &&
#rustup run nightly cargo-fmt &&
#rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cd ../malachite-nz &&
cargo update &&
#rustup run nightly cargo-fmt &&
#rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cd ../malachite-test &&
cargo update &&
#rustup run nightly cargo-fmt &&
#rustup run nightly cargo clippy &&
cargo test --release &&
#cargo run --release -- exhaustive 100000 all &&
cargo rustc --release --lib -- --emit asm
