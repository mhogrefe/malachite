#!/bin/bash
cd malachite-gmp &&
cargo update &&
cargo fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cd ../malachite-native &&
cargo update &&
cargo fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cd ../malachite-test &&
cargo update &&
cargo fmt &&
cargo clippy &&
cargo test --release &&
cargo bench &&
cargo rustc --release -- --emit asm
