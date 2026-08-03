# cpp-test: differential testing against FLINT, without FFI

This harness checks Malachite functions against FLINT by comparing text, not by linking: a
Malachite demo prints one `input = output` line per case, and a small C oracle
(`flint-oracle.c`) re-reads the file, recomputes every line with FLINT, and exits nonzero on the
first disagreement. There is no FFI, no `unsafe`, and no C build coupling in the Malachite
crates themselves.

Use it when no good Rust oracle exists — in particular for FLINT-specific functions that `rug`
(which wraps GMP and MPFR only) cannot reach, and for functions whose behavior on invalid input
(such as composite moduli for `mod_sqrt`) is easiest to pin down as "exactly what FLINT does."

## Prerequisites

- A **built FLINT source tree**. The FLINT source is deliberately not part of this repository.
  The default location is `../../flint-3.6.0` relative to this directory (that is,
  `~/rust/flint-3.6.0` when the Malachite repository is at `~/rust/malachite`); override it with
  the `MALACHITE_FLINT_DIR` environment variable. To build one:

  ```
  cd ~/rust/flint-3.6.0
  ./bootstrap.sh
  ./configure --with-gmp=/opt/homebrew --with-mpfr=/opt/homebrew
  make -j 4
  ```

  Do not use a development snapshot: a 3.3.0-dev snapshot was found to have a memory-corruption
  bug in its `fmpz` mpz allocator (flaky segfaults after a few thousand promotions), which the
  `sqrtmod_stress` oracle mode reproduces.

## Running

```
cargo run --release
```

from this directory. The driver compiles `flint-oracle.c` on demand (into `target/`), then runs
each registered Malachite demo in all three generator modes (`exhaustive`, `random`,
`special_random`) at 10000 lines each, diffing every run against FLINT. Any disagreement fails
the run with the offending line reported. The demo output lands in `test-out.txt`, which is
shared state: don't run the driver and a manual demo regeneration concurrently.

## Current oracles

| mode | FLINT function | Malachite demo |
|---|---|---|
| `fmpz_sqrtmod` | `fmpz_sqrtmod` | `demo_natural_mod_sqrt` (malachite-nz) |
| `n_sqrtmod` | `n_sqrtmod` | `demo_mod_sqrt_u64` (malachite-base) |
| `n_primitive_root_prime` | `n_primitive_root_prime` | `demo_*_primitive_root_prime` (malachite-base) |
| `sqrtmod_stress` | `fmpz_sqrtmod` | none — a memory-stress diagnostic, run manually |

The `sqrtmod` modes skip documented divergence windows, noted in comments in `flint-oracle.c`;
all involve only composite moduli, where Malachite computes the mathematically expected value
and FLINT's behavior rests on undefined or wrapping operations.

## Adding an oracle

1. Write a Malachite demo that prints one line per case in a stable `input = output` format.
2. Add a mode to `flint-oracle.c` that parses that format (the input file arrives as `argv[2]`),
   recomputes with FLINT, and returns 1 with a diagnostic on the first mismatch.
3. Register the demo in `src/main.rs` with `check_demo_against_flint`.
4. Prove the harness can fail: corrupt one line of `test-out.txt` by hand and check that the
   oracle catches it.
