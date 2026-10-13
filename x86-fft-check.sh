#!/bin/bash
# Runs the FFT multiplication's kernels natively and, when Docker is available, its AVX2 instantiation
# under x86_64 emulation, and compares the results. See x86-fft-check/src/main.rs.
#
# The AVX2 kernels (malachite-nz/src/natural/arithmetic/mul/fft/avx2.rs and avx2_body.rs) cannot run
# on Apple Silicon: Rosetta has no AVX. QEMU's user-mode emulation does, so the check is cross-compiled
# as a static Linux binary and run in an amd64 container. Setup on a Mac, once:
#
#     brew install colima && colima start
#     # lima's QEMU is too old to emulate AVX2; register newer handlers from tonistiigi/binfmt:
#     docker run --privileged --rm tonistiigi/binfmt --uninstall qemu-x86_64
#     docker run --privileged --rm tonistiigi/binfmt --install amd64
#
# colima mounts only the home directory into its VM, so the repository must be under $HOME.
set -e
cd "$(dirname "$0")/x86-fft-check"
echo "Building and running the FFT check natively"
cargo build --release
./target/release/malachite-x86-fft-check > target/native.txt
tail -1 target/native.txt
if ! command -v docker > /dev/null || ! docker info > /dev/null 2>&1; then
    echo "Docker is not running, so the AVX2 kernels were NOT exercised; see x86-fft-check.sh"
    exit 0
fi
echo "Building the FFT check for x86_64-unknown-linux-musl"
rustup target add x86_64-unknown-linux-musl > /dev/null 2>&1
cargo build --release --target x86_64-unknown-linux-musl
echo "Running it under x86_64 emulation"
docker run --rm --platform linux/amd64 -e QEMU_CPU=max \
    -v "$PWD/target/x86_64-unknown-linux-musl/release:/w:ro" alpine /w/malachite-x86-fft-check \
    2>&1 | grep -v "^WARNING: The requested image's platform" > target/emulated.txt
head -1 target/emulated.txt
if ! grep -q "avx2=true fma=true" target/emulated.txt; then
    echo "The emulator does not expose AVX2 and FMA, so the AVX2 kernels did not run; see x86-fft-check.sh"
    exit 1
fi
tail -1 target/emulated.txt
strip_timings() { tail -n +2 "$1" | sed 's/ ([0-9.]*s fft)//'; }
diff <(strip_timings target/native.txt) <(strip_timings target/emulated.txt) &&
echo "The emulated AVX2 results are identical to the native ones"
