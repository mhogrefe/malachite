#!/bin/bash
cd malachite-gmp &&
cargo update &&
cargo fmt &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cd ../malachite-native &&
cargo update &&
cargo fmt &&
cargo test --release &&
cargo doc &&
cargo rustc --release -- --emit asm &&
cd ../malachite-test &&
cargo update &&
cargo fmt &&
cargo test --release &&
cargo bench &&
cargo rustc --release -- --emit asm
