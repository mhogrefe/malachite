#!/bin/bash
cd malachite-base &&
cargo update &&
rustup run nightly cargo-fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit mir &&
cd ../malachite-gmp &&
cargo update &&
rustup run nightly cargo-fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit mir &&
cd ../malachite-native &&
cargo update &&
rustup run nightly cargo-fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit mir &&
cd ../malachite &&
cargo update &&
rustup run nightly cargo-fmt &&
rustup run nightly cargo clippy --features native &&
cargo test --release --features gmp &&
cargo test --release --features native &&
cargo doc --features native &&
cargo rustc --release --features native -- --emit mir &&
cd ../malachite-test &&
cargo update &&
rustup run nightly cargo-fmt &&
rustup run nightly cargo clippy &&
cargo test --release &&
cargo run --release -- exhaustive all &&
cargo rustc --release --lib -- --emit mir
