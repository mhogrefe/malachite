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
echo "Step 6. Checking malachite-base lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 7. Checking malachite-base lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 8. Checking all malachite-base targets with bin_build and no std" &&
cargo check --all-targets --features bin_build --no-default-features &&
cd ../malachite-nz &&
echo "Step 9. Formatting malachite-nz" &&
bash ../superfmt.sh &&
echo "Step 10. Checking malachite-nz lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 11. Checking malachite-nz lib" &&
cargo check --lib &&
echo "Step 12. Checking malachite-nz lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 13. Checking malachite-nz lib with random" &&
cargo check --lib --features random &&
echo "Step 14. Checking malachite-nz lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 15. Checking malachite-nz lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 16. Checking malachite-nz lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 17. Checking malachite-nz lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 18. Checking all malachite-nz targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 19. Checking all malachite-nz targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 20. Checking all malachite-nz targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 21. Checking all malachite-nz targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 22. Checking malachite-nz lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 23. Checking malachite-nz lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 24. Checking malachite-nz lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 25. Checking malachite-nz lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 26. Checking malachite-nz lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 27. Checking malachite-nz lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 28. Checking malachite-nz lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 29. Checking malachite-nz lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 30. Checking all malachite-nz targets with bin_build, 32_bit_limbs, and serde and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 31. Checking all malachite-nz targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 32. Checking all malachite-nz targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 33. Checking all malachite-nz targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../malachite-q &&
echo "Step 34. Formatting malachite-q" &&
bash ../superfmt.sh &&
echo "Step 35. Checking malachite-q lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 36. Checking malachite-q lib" &&
cargo check --lib &&
echo "Step 37. Checking malachite-q lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 38. Checking malachite-q lib with random" &&
cargo check --lib --features random &&
echo "Step 39. Checking malachite-q lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 40. Checking malachite-q lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 41. Checking malachite-q lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 42. Checking malachite-q lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 43. Checking all malachite-q targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 44. Checking all malachite-q targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 45. Checking all malachite-q targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 46. Checking all malachite-q targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 47. Checking malachite-q lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 48. Checking malachite-q lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 49. Checking malachite-q lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 50. Checking malachite-q lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 51. Checking malachite-q lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 52. Checking malachite-q lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 53. Checking malachite-q lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 54. Checking malachite-q lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 55. Checking all malachite-q targets with bin_build, 32_bit_limbs, and serde and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 56. Checking all malachite-q targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 57. Checking all malachite-q targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 58. Checking all malachite-q targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../malachite-float &&
echo "Step 59. Formatting malachite-float" &&
bash ../superfmt.sh &&
echo "Step 60. Checking malachite-float lib with 32_bit_limbs" &&
cargo check --lib --features 32_bit_limbs &&
echo "Step 61. Checking malachite-float lib" &&
cargo check --lib &&
echo "Step 62. Checking malachite-float lib with 32_bit_limbs and random" &&
cargo check --lib --features 32_bit_limbs --features random &&
echo "Step 63. Checking malachite-float lib with random" &&
cargo check --lib --features random &&
echo "Step 64. Checking malachite-float lib with 32_bit_limbs and serde" &&
cargo check --lib --features 32_bit_limbs --features enable_serde &&
echo "Step 65. Checking malachite-float lib with serde" &&
cargo check --lib --features enable_serde &&
echo "Step 66. Checking malachite-float lib with 32_bit_limbs, serde, and random" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 67. Checking malachite-float lib with serde and random" &&
cargo check --lib --features enable_serde --features random &&
echo "Step 68. Checking all malachite-float targets with bin_build, 32_bit_limbs, and serde" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 69. Checking all malachite-float targets with bin_build and serde" &&
cargo check --all-targets --features bin_build --features enable_serde &&
echo "Step 70. Checking all malachite-float targets with bin_build, 32_bit_limbs, serde, and random" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random &&
echo "Step 71. Checking all malachite-float targets with bin_build, serde, and random" &&
cargo check --all-targets --features bin_build --features enable_serde --features random &&
echo "Step 72. Checking malachite-float lib with 32_bit_limbs and no std" &&
cargo check --lib --features 32_bit_limbs --no-default-features &&
echo "Step 73. Checking malachite-float lib with no std" &&
cargo check --lib --no-default-features &&
echo "Step 74. Checking malachite-float lib with 32_bit_limbs and random and no std" &&
cargo check --lib --features 32_bit_limbs --features random --no-default-features &&
echo "Step 75. Checking malachite-float lib with random and no std" &&
cargo check --lib --features random --no-default-features &&
echo "Step 76. Checking malachite-float lib with 32_bit_limbs and serde and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 77. Checking malachite-float lib with serde and no std" &&
cargo check --lib --features enable_serde --no-default-features &&
echo "Step 78. Checking malachite-float lib with 32_bit_limbs, serde, and random and no std" &&
cargo check --lib --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 79. Checking malachite-float lib with serde and random and no std" &&
cargo check --lib --features enable_serde --features random --no-default-features &&
echo "Step 80. Checking all malachite-float targets with bin_build, 32_bit_limbs, and serde no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 81. Checking all malachite-float targets with bin_build and serde and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 82. Checking all malachite-float targets with bin_build, 32_bit_limbs, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --features random --no-default-features &&
echo "Step 83. Checking all malachite-float targets with bin_build, serde, and random and no std" &&
cargo check --all-targets --features bin_build --features enable_serde --features random --no-default-features &&
cd ../malachite-criterion-bench &&
echo "Step 84. Formatting malachite-criterion-bench" &&
bash ../superfmt.sh &&
echo "Step 85. Checking malachite-criterion-bench" &&
cargo check &&
cd ../malachite-bigint &&
echo "Step 86. Formatting malachite-bigint" &&
bash ../superfmt.sh &&
echo "Step 87. Checking all malachite-bigint targets" &&
cargo check --all-targets &&
echo "Step 88. Checking all malachite-bigint targets with no std" &&
cargo check --all-targets --no-default-features &&
echo "Step 89. Checking the malachite meta-crate" &&
cd ../malachite &&
cargo check --all-targets &&
echo "Step 90. Checking the malachite meta-crate with serde" &&
cargo check --all-targets --features enable_serde &&
cd .. &&
echo "Step 91. Running additional-lints" &&
python3 additional-lints.py &&
cd malachite-base &&
echo "Step 92. Updating malachite-base" &&
cargo update &&
echo "Step 93. Formatting malachite-base" &&
bash ../superfmt.sh &&
echo "Step 94. Running clippy on malachite-base" &&
cargo clippy --all-targets --features bin_build &&
echo "Step 95. Running clippy on malachite-base with no std" &&
cargo clippy --all-targets --features bin_build --no-default-features &&
echo "Step 96. Testing malachite-base" &&
cargo test --release --tests --features bin_build &&
echo "Step 97. Testing malachite-base with no std" &&
cargo test --release --tests --features bin_build --no-default-features &&
echo "Step 98. Testing malachite-base doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 99. Testing malachite-base doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 100. Documenting malachite-base" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 101. Building malachite-base lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-nz &&
echo "Step 102. Updating malachite-nz" &&
cargo update &&
echo "Step 103. Formatting malachite-nz" &&
bash ../superfmt.sh &&
echo "Step 104. Running clippy on malachite-nz" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 105. Running clippy on malachite-nz with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features 32_bit_limbs --features enable_serde &&
echo "Step 106. Running clippy on malachite-nz with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 107. Running clippy on malachite-nz with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 108. Testing malachite-nz" &&
cargo test --release --tests --features test_build --features enable_serde &&
echo "Step 109. Testing malachite-nz with no std" &&
cargo test --release --tests --features test_build --features enable_serde --no-default-features &&
echo "Step 110. Testing malachite-nz doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 111. Testing malachite-nz with 32_bit_limbs" &&
cargo test --release --tests --features test_build --features 32_bit_limbs --features enable_serde &&
echo "Step 112. Testing malachite-nz with 32_bit_limbs and no std" &&
cargo test --release --tests --features test_build --features 32_bit_limbs --features enable_serde --no-default-features &&
echo "Step 113. Testing malachite-nz doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs&&
echo "Step 114. Testing malachite-nz doctests with 32_bit_limbs and random" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs --features random &&
echo "Step 115. Running extra tests for malachite-nz" &&
python3 extra-tests.py &&
echo "Step 116. Documenting malachite-nz" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features doc-images --features random &&
echo "Step 117. Building malachite-nz lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
echo "Step 118. Building malachite-nz lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite-q &&
echo "Step 119. Updating malachite-q" &&
cargo update &&
echo "Step 120. Formatting malachite-q" &&
bash ../superfmt.sh &&
echo "Step 121. Running clippy on malachite-q" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 122. Running clippy on malachite-q with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 123. Running clippy on malachite-q with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 124. Running clippy on malachite-q with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 125. Testing malachite-q" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 126. Testing malachite-q with no std" &&
cargo test --release --tests --features bin_build --features enable_serde --no-default-features &&
echo "Step 127. Testing malachite-q doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 128. Testing malachite-q with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 129. Testing malachite-q with 32_bit_limbs and no std" &&
cargo test --release --tests --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 130. Testing malachite-q doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 131. Testing malachite-q doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 132. Testing malachite-q doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 133. Documenting malachite-q" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 134. Building malachite-q lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 135. Building malachite-q lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../malachite-float &&
echo "Step 136. Updating malachite-float" &&
cargo update &&
echo "Step 137. Formatting malachite-float" &&
bash ../superfmt.sh &&
echo "Step 138. Running clippy on malachite-float" &&
cargo clippy --all-targets --features bin_build --features enable_serde &&
echo "Step 139. Running clippy on malachite-float with 32_bit_limbs" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs &&
echo "Step 140. Running clippy on malachite-float with no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --no-default-features &&
echo "Step 141. Running clippy on malachite-float with 32_bit_limbs and no std" &&
cargo clippy --all-targets --features bin_build --features enable_serde --features 32_bit_limbs --no-default-features &&
echo "Step 142. Testing malachite-float" &&
cargo test --release --tests --features bin_build --features enable_serde &&
echo "Step 143. Testing malachite-float with no std" &&
cargo test --release --tests --features bin_build --features enable_serde --no-default-features &&
echo "Step 144. Testing malachite-float doctests" &&
bash ../rundoc.sh --features test_build &&
echo "Step 145. Testing malachite-float with 32_bit_limbs" &&
cargo test --release --tests --features bin_build --features 32_bit_limbs &&
echo "Step 146. Testing malachite-float with 32_bit_limbs and no std" &&
cargo test --release --tests --features bin_build --features 32_bit_limbs --no-default-features &&
echo "Step 147. Testing malachite-float doctests with 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features 32_bit_limbs &&
echo "Step 148. Testing malachite-float doctests with random" &&
bash ../rundoc.sh --features test_build --features random &&
echo "Step 149. Testing malachite-float doctests with random and 32_bit_limbs" &&
bash ../rundoc.sh --features test_build --features random --features 32_bit_limbs &&
echo "Step 150. Documenting malachite-float" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
echo "Step 151. Building malachite-float lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
echo "Step 152. Building malachite-float lib for wasm with 32_bit_limbs" &&
cargo build --lib --release --features 32_bit_limbs --target wasm32-unknown-unknown &&
cd ../malachite-bigint &&
echo "Step 153. Updating malachite-bigint" &&
cargo update &&
echo "Step 154. Formatting malachite-bigint" &&
bash ../superfmt.sh &&
echo "Step 155. Running clippy on malachite-bigint" &&
cargo clippy --all-targets &&
echo "Step 156. Running clippy on malachite-bigint with no std" &&
cargo clippy --all-targets --no-default-features &&
echo "Step 157. Testing malachite-bigint" &&
cargo test --release &&
echo "Step 158. Testing malachite-bigint with no std" &&
cargo test --release --no-default-features &&
echo "Step 159. Documenting malachite-bigint" &&
cargo doc --lib --no-deps &&
echo "Step 160. Building malachite-bigint lib for wasm" &&
cargo build --lib --release --target wasm32-unknown-unknown &&
cd ../malachite &&
echo "Step 161. Documenting malachite" &&
RUSTDOCFLAGS="--html-in-header katex-header.html" cargo doc --lib --no-deps --features random &&
cd ../malachite-criterion-bench &&
echo "Step 162. Updating malachite-criterion-bench" &&
cargo update &&
echo "Step 163. Formatting malachite-criterion-bench" &&
bash ../superfmt.sh &&
cd .. &&
echo "Step 164. Running additional-lints" &&
python3 additional-lints.py &&
echo "Step 165. Testing against FLINT" &&
cd ../malachite-cpp-test/malachite-test-cpp &&
cargo run --release &&
echo "Step 166. Checking links" &&
cd ../../check-malachite-links &&
cargo run --release
