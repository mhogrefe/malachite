#!/bin/bash
rustup update &&
cd malachite-base &&
cargo +nightly fmt --all &&
cargo check --all-targets &&
cd ../malachite-base-test-util &&
cargo +nightly fmt --all &&
cargo check --all-targets &&
cd ../malachite-nz &&
cargo +nightly fmt --all &&
cargo check --all-targets --features 32_bit_limbs --features serde &&
cargo check --all-targets --features serde &&
cd ../malachite-nz-test-util &&
cargo +nightly fmt --all &&
cargo check --all-targets &&
cd ../malachite-q &&
cargo +nightly fmt --all &&
cargo check --all-targets --features 32_bit_limbs --features serde &&
cargo check --all-targets --features serde &&
cd ../malachite-q-test-util &&
cargo +nightly fmt --all &&
cargo check --all-targets &&
cd ../malachite-bench &&
cargo +nightly fmt --all &&
cd .. &&
python3 additional-lints.py &&
cd malachite-base &&
cargo update &&
cargo +nightly fmt --all &&
# cargo clippy --tests &&
cargo test --release &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-base-test-util &&
cargo update &&
cargo +nightly fmt --all &&
# cargo clippy &&
cd ../malachite-nz &&
cargo update &&
cargo +nightly fmt --all &&
# cargo clippy --tests --features 32_bit_limbs --features serde &&
cargo test --release --features 32_bit_limbs --features fail_on_untested_path --features serde &&
cargo test --release --features fail_on_untested_path --features serde &&
python3 extra-tests.py &&
# cargo clippy --tests --features serde &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps &&
cargo build --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-nz-test-util &&
cargo update &&
cargo +nightly fmt --all &&
# cargo clippy &&
cd ../malachite-q &&
cargo update &&
cargo +nightly fmt --all &&
# cargo clippy --tests --features 32_bit_limbs --features serde &&
cargo test --release --features 32_bit_limbs --features fail_on_untested_path --features serde &&
cargo test --release --features fail_on_untested_path --features serde &&
# cargo clippy --tests --features serde &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --no-deps &&
cargo build --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cargo build --release --target wasm32-unknown-unknown &&
cd ../malachite-q-test-util &&
cargo update &&
cargo +nightly fmt --all &&
# cargo clippy &&
cd ../malachite-bench &&
cargo update &&
cargo +nightly fmt --all &&
cd .. &&
python3 additional-lints.py
