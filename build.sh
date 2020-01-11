#!/bin/bash
cd malachite-base &&
cargo update &&
cargo fmt &&
cargo clippy
cargo test --release &&
cargo doc &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-nz &&
cargo update &&
cargo fmt &&
cargo clippy --features 32_bit_limbs &&
cargo test --release --features 32_bit_limbs &&
cargo clippy &&
cargo doc &&
cargo build --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-bench &&
cargo update &&
cargo fmt &&
#cargo clippy &&
cd ../malachite-test &&
cargo update &&
cargo fmt &&
#cargo clippy
python extra-tests.py
cargo test --release --features 32_bit_limbs &&
cargo test --release &&
cd .. &&
python additional-lints.py
