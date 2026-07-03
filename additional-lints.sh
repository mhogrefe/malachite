#!/bin/bash
# Runs the house Dylint lints (see malachite-lints/) over every crate, covering all targets. The
# lints are deny-by-default, so any hit fails the run. Lint configuration, including the
# long-lines exception list, lives in dylint.toml.
set -e
cd "$(dirname "$0")"
# A stale `#[expect(long_lines)]` exemption surfaces as an unfulfilled-expectation warning; make
# it fail the run like any other lint hit.
export DYLINT_RUSTFLAGS="-D unfulfilled_lint_expectations"
# Cargo's fingerprints include neither the lint library's code nor dylint.toml (both reach the
# driver via environment variables), so changing them would not re-lint otherwise-unchanged
# crates. Hash them and force a full re-lint when they change.
LINT_HASH=$( (find malachite-lints/src malachite-lints/Cargo.toml dylint.toml -type f | sort | xargs shasum) | shasum | cut -d' ' -f1)
STAMP=target/dylint/lint-sources.hash
if [ ! -f "$STAMP" ] || [ "$(cat "$STAMP")" != "$LINT_HASH" ]; then
    echo "The lints changed; forcing a full re-lint"
    rm -rf target/dylint/target
    mkdir -p target/dylint
    echo "$LINT_HASH" > "$STAMP"
fi
for crate in malachite-base malachite-nz malachite-q malachite-float; do
    echo "Linting $crate"
    (cd $crate && cargo dylint --all -- --all-targets --features bin_build)
done
for crate in malachite malachite-bigint; do
    echo "Linting $crate"
    (cd $crate && cargo dylint --all -- --all-targets)
done
# The criterion-bench bench targets require features that `build.sh` never enables, so, like
# `build.sh`, only check the default target.
(cd malachite-criterion-bench && echo "Linting malachite-criterion-bench" && cargo dylint --all)
