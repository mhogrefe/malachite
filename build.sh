#!/bin/bash
echo "Step 1. Checking for updates" &&
rustup update &&
cd malachite-base &&
echo "Step 2. Formatting malachite-base" &&
cargo +nightly fmt --all &&
echo "Step 3. Checking malachite-base lib" &&
cargo check --lib &&
echo "Step 4. Checking all malachite-base targets with bin_build" &&
cargo check --all-targets --features bin_build &&
cd ../malachite-nz &&
echo "Step 5. Formatting malachite-nz" &&
cargo +nightly fmt --all &&
echo "Step 6. Checking malachite-nz lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 7. Checking malachite-nz lib" &&
cargo check --lib &&
echo "Step 8. Checking malachite-nz lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features serde &&
echo "Step 9. Checking malachite-nz lib with serde" &&
cargo check --lib --features serde &&
echo "Step 10. Checking all malachite-nz targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features serde &&
echo "Step 11. Checking all malachite-nz targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features serde &&
cd ../malachite-q &&
echo "Step 12. Formatting malachite-q" &&
cargo +nightly fmt --all &&
echo "Step 13. Checking malachite-q lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 14. Checking malachite-q lib" &&
cargo check --lib &&
echo "Step 15. Checking malachite-q lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features serde &&
echo "Step 16. Checking malachite-q lib with serde" &&
cargo check --lib --features serde &&
echo "Step 17. Checking all malachite-q targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features serde &&
echo "Step 18. Checking all malachite-q targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features serde &&
cd ../malachite-criterion-bench &&
echo "Step 19. Formatting malachite-criterion-bench" &&
cargo +nightly fmt --all &&
echo "Step 20. Checking malachite-criterion-bench" &&
cargo check &&
cd .. &&
echo "Step 21. Running additional-lints" &&
python3 additional-lints.py &&
cd malachite-base &&
echo "Step 22. Updating malachite-base" &&
cargo update &&
echo "Step 23. Formatting malachite-base" &&
cargo +nightly fmt --all &&
echo "Step 24. Running clippy on malachite-base" &&
cargo clippy --tests --features bin_build &&
echo "Step 25. Testing malachite-base with bin_build" &&
cargo test --release --features bin_build &&
echo "Step 26. Testing malachite-base doctests without bin_build" &&
cargo test --release --doc &&
echo "Step 27. Documenting malachite-base" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps &&
echo "Step 28. Building malachite-base lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-nz &&
echo "Step 29. Updating malachite-nz" &&
cargo update &&
echo "Step 30. Formatting malachite-nz" &&
cargo +nightly fmt --all &&
echo "Step 31. Running clippy on malachite-nz with 32_bit_limbs" &&
cargo clippy --tests --features bin_build --features 32_bit_limbs --features serde &&
echo "Step 32. Testing malachite-nz with bin_build and 32_bit_limbs" &&
cargo test --release --features bin_build --features 32_bit_limbs --features serde &&
echo "Step 33. Testing malachite-nz with bin_build" &&
cargo test --release --features bin_build --features serde &&
echo "Step 34. Testing malachite-nz doctests without bin_build and with 32_bit_limbs" &&
cargo test --release --doc --features 32_bit_limbs &&
echo "Step 35. Testing malachite-nz doctests without bin_build" &&
cargo test --release --doc &&
echo "Step 36. Running extra tests for malachite-nz" &&
python3 extra-tests.py &&
echo "Step 37. Running clippy on malachite-nz" &&
cargo clippy --tests --features bin_build --features serde &&
echo "Step 38. Documenting malachite-nz" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features doc-images &&
echo "Step 39. Building malachite-nz lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
echo "Step 40. Building malachite-nz lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-q &&
echo "Step 41. Updating malachite-q" &&
cargo update &&
echo "Step 42. Formatting malachite-q" &&
cargo +nightly fmt --all &&
echo "Step 43. Running clippy on malachite-q" &&
cargo clippy --tests --features bin_build --features serde &&
echo "Step 44. Testing malachite-q with bin_build" &&
cargo test --release --features bin_build --features serde &&
echo "Step 45. Testing malachite-q doctests without bin_build" &&
cargo test --release --doc &&
echo "Step 46. Documenting malachite-q" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps &&
echo "Step 47. Building malachite-q lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-criterion-bench &&
echo "Step 48. Updating malachite-criterion-bench" &&
cargo update &&
echo "Step 49. Formatting malachite-criterion-bench" &&
cargo +nightly fmt --all &&
cd .. &&
echo "Step 50. Running additional-lints" &&
python3 additional-lints.py
