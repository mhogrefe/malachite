#!/bin/bash
echo "Step 1. Checking for updates" &&
rustup update &&
cd malachite-base &&
echo "Step 2. Formatting malachite-base" &&
bash ../superfmt.sh &&
echo "Step 3. Checking malachite-base lib" &&
cargo check --lib &&
echo "Step 4. Checking malachite-base lib with random" &&
cargo check --lib --features random &&
echo "Step 5. Checking all malachite-base targets with bin_build" &&
cargo check --all-targets --features bin_build &&
cd ../malachite-nz &&
echo "Step 6. Formatting malachite-nz" &&
bash ../superfmt.sh &&
echo "Step 7. Checking malachite-nz lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 8. Checking malachite-nz lib" &&
cargo check --lib &&
echo "Step 9. Checking malachite-nz lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 10. Checking malachite-nz lib with random" &&
cargo check --lib --features random &&
echo "Step 11. Checking malachite-nz lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 12. Checking malachite-nz lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 13. Checking malachite-nz lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 14. Checking malachite-nz lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 15. Checking all malachite-nz targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 16. Checking all malachite-nz targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 17. Checking all malachite-nz targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 18. Checking all malachite-nz targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
cd ../malachite-q &&
echo "Step 19. Formatting malachite-q" &&
bash ../superfmt.sh &&
echo "Step 20. Checking malachite-q lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 21. Checking malachite-q lib" &&
cargo check --lib &&
echo "Step 22. Checking malachite-q lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 23. Checking malachite-q lib with random" &&
cargo check --lib --features random &&
echo "Step 24. Checking malachite-q lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 25. Checking malachite-q lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 26. Checking malachite-q lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 27. Checking malachite-q lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 28. Checking all malachite-q targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 29. Checking all malachite-q targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 30. Checking all malachite-q targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 31. Checking all malachite-q targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
cd ../malachite-float &&
echo "Step 32. Formatting malachite-float" &&
bash ../superfmt.sh &&
echo "Step 33. Checking malachite-float lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 34. Checking malachite-float lib" &&
cargo check --lib &&
echo "Step 35. Checking malachite-float lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 36. Checking malachite-float lib with random" &&
cargo check --lib --features random &&
echo "Step 37. Checking malachite-float lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 38. Checking malachite-float lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 39. Checking malachite-float lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 40. Checking malachite-float lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 41. Checking all malachite-float targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 42. Checking all malachite-float targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 43. Checking all malachite-float targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 44. Checking all malachite-float targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
cd ../malachite-criterion-bench &&
echo "Step 45. Formatting malachite-criterion-bench" &&
bash ../superfmt.sh &&
echo "Step 46. Checking malachite-criterion-bench" &&
cargo check &&
echo "Step 47. Checking the malachite meta-crate" &&
cd ../malachite &&
cargo check --all-targets &&
echo "Step 48. Checking the malachite meta-crate with serde" &&
cargo check --all-targets --features enable_serde &&
cd .. &&
echo "Step 49. Running additional-lints" &&
python3 additional-lints.py &&
cd malachite-base &&
echo "Step 50. Updating malachite-base" &&
cargo update &&
echo "Step 51. Formatting malachite-base" &&
bash ../superfmt.sh &&
echo "Step 52. Running clippy on malachite-base" &&
cargo clippy --all-targets --features bin_build &&
echo "Step 53. Testing malachite-base" &&
cargo test --release --tests --features bin_build &&
echo "Step 54. Testing malachite-base doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 55. Testing malachite-base doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 56. Documenting malachite-base" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 57. Building malachite-base lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-nz &&
echo "Step 58. Updating malachite-nz" &&
cargo update &&
echo "Step 59. Formatting malachite-nz" &&
bash ../superfmt.sh &&
echo "Step 60. Running clippy on malachite-nz" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 61. Running clippy on malachite-nz with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 62. Testing malachite-nz" &&
cargo test --release --tests --features test_build --features enable_serde &&
echo "Step 63. Testing malachite-nz doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 64. Testing malachite-nz with 32_bit_limbs" &&
cargo test --release --tests --features test_build --features 32_bit_limbs --features enable_serde &&
echo "Step 65. Testing malachite-nz doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs&&
echo "Step 66. Testing malachite-nz doctests with 32_bit_limbs and random" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs --features random &&
echo "Step 67. Running extra tests for malachite-nz" &&
python3 extra-tests.py &&
echo "Step 68. Documenting malachite-nz" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features doc-images --features random &&
echo "Step 69. Building malachite-nz lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
echo "Step 70. Building malachite-nz lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-q &&
echo "Step 71. Updating malachite-q" &&
cargo update &&
echo "Step 72. Formatting malachite-q" &&
bash ../superfmt.sh &&
echo "Step 73. Running clippy on malachite-q" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 74. Running clippy on malachite-q with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 75. Testing malachite-q" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 76. Testing malachite-q doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 77. Testing malachite-q with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 78. Testing malachite-q doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 79. Testing malachite-q doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 80. Testing malachite-q doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 81. Documenting malachite-q" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 82. Building malachite-q lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 83. Building malachite-q lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../malachite-float &&
echo "Step 84. Updating malachite-float" &&
cargo update &&
echo "Step 85. Formatting malachite-float" &&
bash ../superfmt.sh &&
echo "Step 86. Running clippy on malachite-float" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 87. Running clippy on malachite-float with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 88. Testing malachite-float" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 89. Testing malachite-float doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 90. Testing malachite-float with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features 32_bit_limbs &&
echo "Step 91. Testing malachite-float doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 92. Testing malachite-float doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 93. Testing malachite-float doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 94. Documenting malachite-float" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 95. Building malachite-float lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 96. Building malachite-float lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../malachite &&
echo "Step 97. Documenting malachite" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
cd ../malachite-criterion-bench &&
echo "Step 98. Updating malachite-criterion-bench" &&
cargo update &&
echo "Step 99. Formatting malachite-criterion-bench" &&
bash ../superfmt.sh &&
cd .. &&
echo "Step 100. Running additional-lints" &&
python3 additional-lints.py
