#!/bin/bash
echo "Step 1. Checking for cargo-dylint and dylint-link (cargo install cargo-dylint dylint-link)" &&
command -v cargo-dylint > /dev/null &&
command -v dylint-link > /dev/null &&
echo "Step 2. Checking for updates" &&
rustup update &&
echo "Step 3. Updating headers" &&
cd ../fix-headers &&
cargo test --release &&
cd ../malachite/malachite-base &&
echo "Step 4. Formatting malachite-base" &&
bash ../superfmt.sh &&
echo "Step 5. Checking malachite-base lib" &&
cargo check --lib &&
echo "Step 6. Checking malachite-base lib with random" &&
cargo check --lib --features random &&
echo "Step 7. Checking all malachite-base targets with bin_build" &&
cargo check --all-targets --features bin_build &&
echo "Step 8. Checking malachite-base lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 9. Checking malachite-base lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 10. Checking all malachite-base targets with bin_build and no std" &&
cargo check --all-targets --features bin_build --no-default-features &&
cd ../malachite-nz &&
echo "Step 11. Formatting malachite-nz" &&
bash ../superfmt.sh &&
echo "Step 12. Checking malachite-nz lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 13. Checking malachite-nz lib" &&
cargo check --lib &&
echo "Step 14. Checking malachite-nz lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 15. Checking malachite-nz lib with random" &&
cargo check --lib --features random &&
echo "Step 16. Checking malachite-nz lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 17. Checking malachite-nz lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 18. Checking malachite-nz lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 19. Checking malachite-nz lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 20. Checking all malachite-nz targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 21. Checking all malachite-nz targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 22. Checking all malachite-nz targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 23. Checking all malachite-nz targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 24. Checking malachite-nz lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 25. Checking malachite-nz lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 26. Checking malachite-nz lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 27. Checking malachite-nz lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 28. Checking malachite-nz lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 29. Checking malachite-nz lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 30. Checking malachite-nz lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 31. Checking malachite-nz lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 32. Checking all malachite-nz targets with bin_build, 32_bit_limbs, and serde and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 33. Checking all malachite-nz targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 34. Checking all malachite-nz targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 35. Checking all malachite-nz targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../malachite-q &&
echo "Step 36. Formatting malachite-q" &&
bash ../superfmt.sh &&
echo "Step 37. Checking malachite-q lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 38. Checking malachite-q lib" &&
cargo check --lib &&
echo "Step 39. Checking malachite-q lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 40. Checking malachite-q lib with random" &&
cargo check --lib --features random &&
echo "Step 41. Checking malachite-q lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 42. Checking malachite-q lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 43. Checking malachite-q lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 44. Checking malachite-q lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 45. Checking all malachite-q targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 46. Checking all malachite-q targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 47. Checking all malachite-q targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 48. Checking all malachite-q targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 49. Checking malachite-q lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 50. Checking malachite-q lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 51. Checking malachite-q lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 52. Checking malachite-q lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 53. Checking malachite-q lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 54. Checking malachite-q lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 55. Checking malachite-q lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 56. Checking malachite-q lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 57. Checking all malachite-q targets with bin_build, 32_bit_limbs, and serde and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 58. Checking all malachite-q targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 59. Checking all malachite-q targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 60. Checking all malachite-q targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../malachite-float &&
echo "Step 61. Formatting malachite-float" &&
bash ../superfmt.sh &&
echo "Step 62. Checking malachite-float lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 63. Checking malachite-float lib" &&
cargo check --lib &&
echo "Step 64. Checking malachite-float lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 65. Checking malachite-float lib with random" &&
cargo check --lib --features random &&
echo "Step 66. Checking malachite-float lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 67. Checking malachite-float lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 68. Checking malachite-float lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 69. Checking malachite-float lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 70. Checking all malachite-float targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 71. Checking all malachite-float targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 72. Checking all malachite-float targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 73. Checking all malachite-float targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 74. Checking malachite-float lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 75. Checking malachite-float lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 76. Checking malachite-float lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 77. Checking malachite-float lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 78. Checking malachite-float lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 79. Checking malachite-float lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 80. Checking malachite-float lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 81. Checking malachite-float lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 82. Checking all malachite-float targets with bin_build, 32_bit_limbs, and serde no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 83. Checking all malachite-float targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 84. Checking all malachite-float targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 85. Checking all malachite-float targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../malachite-criterion-bench &&
echo "Step 86. Formatting malachite-criterion-bench" &&
bash ../superfmt.sh &&
echo "Step 87. Checking malachite-criterion-bench" &&
cargo check &&
cd ../malachite-bigint &&
echo "Step 88. Formatting malachite-bigint" &&
bash ../superfmt.sh &&
echo "Step 89. Checking all malachite-bigint targets" &&
cargo check --all-targets &&
echo "Step 90. Checking all malachite-bigint targets with no std" &&
cargo check --all-targets --no-default-features &&
echo "Step 91. Checking the malachite meta-crate" &&
cd ../malachite &&
cargo check --all-targets &&
echo "Step 92. Checking the malachite meta-crate with serde" &&
cargo check --all-targets --features enable_serde &&
cd .. &&
echo "Step 93. Checking and testing malachite-lints" &&
cd malachite-lints &&
cargo fmt --check &&
cargo test -- --test-threads 1 &&
cd .. &&
echo "Step 94. Running additional-lints" &&
bash additional-lints.sh &&
cd malachite-base &&
echo "Step 95. Updating malachite-base" &&
cargo update &&
echo "Step 96. Formatting malachite-base" &&
bash ../superfmt.sh &&
echo "Step 97. Running clippy on malachite-base" &&
cargo clippy --all-targets --features bin_build &&
echo "Step 98. Running clippy on malachite-base with no std" &&
cargo clippy --all-targets --features bin_build --no-default-features &&
echo "Step 99. Testing malachite-base" &&
cargo test --release --tests --features bin_build &&
echo "Step 100. Testing malachite-base with no std" &&
cargo test --release --tests --features bin_build --no-default-features &&
echo "Step 101. Testing malachite-base doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 102. Testing malachite-base doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 103. Documenting malachite-base" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 104. Building malachite-base lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-nz &&
echo "Step 105. Updating malachite-nz" &&
cargo update &&
echo "Step 106. Formatting malachite-nz" &&
bash ../superfmt.sh &&
echo "Step 107. Running clippy on malachite-nz" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 108. Running clippy on malachite-nz with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 109. Running clippy on malachite-nz with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 110. Running clippy on malachite-nz with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 111. Testing malachite-nz" &&
cargo test --release --tests --features test_build --features enable_serde &&
echo "Step 112. Testing malachite-nz with no std" &&
cargo test --release --tests --features test_build --features enable_serde --no-default-features &&
echo "Step 113. Testing malachite-nz doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 114. Testing malachite-nz with 32_bit_limbs" &&
cargo test --release --tests --features test_build --features 32_bit_limbs --features enable_serde &&
echo "Step 115. Testing malachite-nz with 32_bit_limbs and no std" &&
cargo test --release --tests --features test_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 116. Testing malachite-nz doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs&&
echo "Step 117. Testing malachite-nz doctests with 32_bit_limbs and random" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs --features random &&
echo "Step 118. Running extra tests for malachite-nz" &&
python3 extra-tests.py &&
echo "Step 119. Documenting malachite-nz" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features doc-images --features random &&
echo "Step 120. Building malachite-nz lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
echo "Step 121. Building malachite-nz lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-q &&
echo "Step 122. Updating malachite-q" &&
cargo update &&
echo "Step 123. Formatting malachite-q" &&
bash ../superfmt.sh &&
echo "Step 124. Running clippy on malachite-q" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 125. Running clippy on malachite-q with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 126. Running clippy on malachite-q with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 127. Running clippy on malachite-q with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 128. Testing malachite-q" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 129. Testing malachite-q with no std" &&
cargo test --release --tests --features bin_build --features enable_serde --no-default-features &&
echo "Step 130. Testing malachite-q doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 131. Testing malachite-q with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 132. Testing malachite-q with 32_bit_limbs and no std" &&
cargo test --release --tests --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 133. Testing malachite-q doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 134. Testing malachite-q doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 135. Testing malachite-q doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 136. Documenting malachite-q" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 137. Building malachite-q lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 138. Building malachite-q lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../malachite-float &&
echo "Step 139. Updating malachite-float" &&
cargo update &&
echo "Step 140. Formatting malachite-float" &&
bash ../superfmt.sh &&
echo "Step 141. Running clippy on malachite-float" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 142. Running clippy on malachite-float with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 143. Running clippy on malachite-float with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 144. Running clippy on malachite-float with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 145. Testing malachite-float" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 146. Testing malachite-float with no std" &&
cargo test --release --tests --features bin_build --features enable_serde --no-default-features &&
echo "Step 147. Testing malachite-float doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 148. Testing malachite-float with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features 32_bit_limbs &&
echo "Step 149. Testing malachite-float with 32_bit_limbs and no std" &&
cargo test --release --tests --features bin_build --features 32_bit_limbs --no-default-features &&
echo "Step 150. Testing malachite-float doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 151. Testing malachite-float doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 152. Testing malachite-float doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 153. Documenting malachite-float" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 154. Building malachite-float lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 155. Building malachite-float lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../malachite-bigint &&
echo "Step 156. Updating malachite-bigint" &&
cargo update &&
echo "Step 157. Formatting malachite-bigint" &&
bash ../superfmt.sh &&
echo "Step 158. Running clippy on malachite-bigint" &&
cargo clippy --all-targets &&
echo "Step 159. Running clippy on malachite-bigint with no std" &&
cargo clippy --all-targets --no-default-features &&
echo "Step 160. Testing malachite-bigint" &&
cargo test --release &&
echo "Step 161. Testing malachite-bigint with no std" &&
cargo test --release --no-default-features &&
echo "Step 162. Documenting malachite-bigint" &&
cargo doc --lib --no-deps &&
echo "Step 163. Building malachite-bigint lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite &&
echo "Step 164. Documenting malachite" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
cd ../malachite-criterion-bench &&
echo "Step 165. Updating malachite-criterion-bench" &&
cargo update &&
echo "Step 166. Formatting malachite-criterion-bench" &&
bash ../superfmt.sh &&
cd .. &&
echo "Step 167. Running additional-lints" &&
bash additional-lints.sh &&
echo "Step 168. Testing against FLINT" &&
cd ../malachite-cpp-test/malachite-test-cpp &&
cargo run --release &&
echo "Step 169. Checking links" &&
cd ../../check-malachite-links &&
cargo run --release
