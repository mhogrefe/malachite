#!/bin/bash
cd malachite-gmp &&
cargo update &&
cargo fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit llvm-ir &&
cd ../malachite-native &&
cargo update &&
cargo fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit llvm-ir &&
cd ../malachite &&
cargo update &&
cargo fmt &&
rustup run nightly cargo clippy --features native &&
cargo test --release --features gmp &&
cargo test --release --features native &&
cargo doc --features native &&
cargo rustc --release --features native -- --emit llvm-ir &&
cd ../malachite-test &&
cargo update &&
cargo fmt &&
cargo clippy &&
cargo test --release &&
cargo rustc --release -- --emit llvm-ir
