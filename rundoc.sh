#!/bin/bash
# Runs, in order: the doctests of the crate in the current directory (via doc_runner), Clippy
# over all of that crate's targets with warnings denied, and the house Dylint lints over the
# whole workspace (additional-lints.sh). Any failure fails the run. Extra arguments are passed
# through to doc_runner.
set -e
trap 'code=$?; if [ "$code" -ne 0 ]; then echo "RUNDOC FAILED (exit $code)" >&2; fi' EXIT
# Cap build parallelism unless the caller says otherwise: the child cargo invocations (including
# doc_runner's internal `cargo test`) do not inherit any -j flag, and uncapped runs have
# OOM-crashed this machine.
export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-4}
P=$PWD
ROOT=$(cd "$(dirname "$0")" && pwd)
(cd "$ROOT/../doc_runner" && cargo run --release -- "$P" keep "$@")
# doc_runner leaves the extracted-doctests crate behind (the `keep` argument) so that the house
# lints can run over the doc examples, which rustc-based lints cannot see in comment form. The
# MALACHITE_LINT_DOCTESTS variable turns off the lints' test-code exemptions: doc examples are
# documentation, not tests. The persistent target directory keeps the lint driver and dependency
# builds cached across runs, since the crate itself is regenerated every time.
echo "Running the house lints on the extracted doctests"
DOCTESTS=$ROOT/../doctest_workspace/doctests
# Some house lints are inapplicable to teaching examples: docs construct values from literals,
# demonstrate the prec_round forms under Nearest, show the assign and by-value variants on
# purpose, compare bignums where a comparison is the subject matter, and never want const-block
# advice.
(cd "$DOCTESTS" && MALACHITE_LINT_DOCTESTS=1 \
    DYLINT_RUSTFLAGS="-A runtime_literal_conversion -A redundant_nearest \
        -A assign_then_consumed_once -A assign_then_returned -A clone_with_ref_variant \
        -A use_const_block -A compare_with_primitive -A mul_div_by_power_of_2 \
        -A mul_div_by_power_of_2_literal -A use_fused_mul" \
    CARGO_TARGET_DIR="$ROOT/../doctest_workspace/target" \
    cargo dylint --all -- --all-targets)
rm -rf "$DOCTESTS"
echo "Running Clippy"
# The feature and target flags per crate match build.sh and additional-lints.sh.
case "$(basename "$P")" in
    malachite-base | malachite-nz | malachite-q | malachite-float)
        (cd "$P" && cargo clippy --all-targets --features bin_build -- -D warnings) ;;
    malachite-criterion-bench)
        (cd "$P" && cargo clippy -- -D warnings) ;;
    *)
        (cd "$P" && cargo clippy --all-targets -- -D warnings) ;;
esac
echo "Running the house lints"
bash "$ROOT/additional-lints.sh"
