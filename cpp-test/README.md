# cpp-test: differential testing against FLINT, without FFI

This harness checks Malachite functions against FLINT by comparing text, not by linking: a
Malachite demo prints one `input = output` line per case, and a small C oracle
(the sources in `oracle/`) re-reads the file, recomputes every line with FLINT, and exits
nonzero on the
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

from this directory. The driver compiles the oracle sources in `oracle/` on demand (into
`target/`), then runs
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
| `fmpz_poly_scalar_smod_fmpz` | `fmpz_poly_scalar_smod_fmpz` | `demo_integer_polynomial_balanced_mod`, `_ref`, `_assign`, and `_small_moduli` (malachite-nz), plus unit rows |
| `fmpz_poly_scalar_mod_fmpz` | `fmpz_poly_scalar_mod_fmpz` | `demo_integer_polynomial_mod_op`, `_ref`, `_power_of_2_moduli`, and `_unsigned_u128` (malachite-nz), plus unit rows |
| `fmpz_poly_get_nmod_poly` | `fmpz_poly_get_nmod_poly` | `demo_integer_polynomial_mod_op_unsigned_*` for every word-sized type (malachite-nz), plus unit rows |
| `fmpz_mod_poly_set_fmpz_poly` | `fmpz_mod_poly_set_fmpz_poly` | `demo_natural_polynomial_rem`, `_ref`, `_assign`, `_special_moduli`, `_unsigned_*`, and `_unsigned_ref_*`, and `demo_natural_polynomial_mod_op` and `_mod_assign` (malachite-nz), plus unit rows |
| `fmpz_poly_evaluate_fmpz` | `fmpz_poly_evaluate_fmpz` | `demo_integer_polynomial_evaluate`, `_ref`, and `_long`, and the same three for `natural_polynomial` (malachite-nz), plus unit rows |
| `fmpz_poly_evaluate_horner_fmpz` | `fmpz_poly_evaluate_horner_fmpz` | `demo_integer_polynomial_evaluate_horner` and `demo_natural_polynomial_evaluate_horner` (malachite-nz), which checks the translation of the non-public Horner algorithm, plus unit rows |
| `fmpz_poly_evaluate_divconquer_fmpz` | `fmpz_poly_evaluate_divconquer_fmpz` | `demo_integer_polynomial_evaluate_divide_and_conquer` and `_long`, and the same two for `natural_polynomial` (malachite-nz), which check the translation of the non-public divide-and-conquer algorithm, plus unit rows |
| `fmpz_poly_evaluate_fmpq` | `fmpz_poly_evaluate_fmpq` | `demo_integer_polynomial_evaluate_rational`, `_ref`, and `_long` (malachite-q), plus unit rows |
| `fmpz_poly_evaluate_horner_fmpq` | `fmpz_poly_evaluate_horner_fmpq` | `demo_integer_polynomial_evaluate_rational_horner` (malachite-q), which checks the translation of the non-public Horner algorithm, plus unit rows |
| `fmpz_poly_evaluate_divconquer_fmpq` | `fmpz_poly_evaluate_divconquer_fmpq` | `demo_integer_polynomial_evaluate_rational_divide_and_conquer` and `_long` (malachite-q), which check the translation of the non-public divide-and-conquer algorithm, plus unit rows |
| `fmpq_poly_evaluate_fmpq` | `fmpq_poly_evaluate_fmpq` | `demo_rational_polynomial_evaluate` and `_ref` (malachite-q), plus unit rows; its inputs are read by `fmpq_poly_set_str_malachite` in `oracle/util.c` |
| `fmpq_poly_evaluate_fmpz` | `fmpq_poly_evaluate_fmpz` | `demo_rational_polynomial_evaluate_integer` and `_ref` (malachite-q), plus unit rows |
| `fmpz_mod_poly_evaluate_fmpz` | `fmpz_mod_poly_evaluate_fmpz`, with the modulus 2^pow | `demo_natural_polynomial_evaluate_mod_power_of_2` and `_ref` (malachite-nz), plus unit rows |
| `sqrtmod_stress` | `fmpz_sqrtmod` | none — a memory-stress diagnostic, run manually |

The `sqrtmod` modes skip documented divergence windows, noted in comments in
`oracle/sqrtmod.c`;
all involve only composite moduli, where Malachite computes the mathematically expected value
and FLINT's behavior rests on undefined or wrapping operations.

## Adding an oracle

1. Write a Malachite demo that prints one line per case in a stable `input = output` format.
2. Add a mode file under `oracle/` that parses that format (one `run_*` entry point; declare it
   in `oracle/oracle.h` and add a row to the table in `oracle/main.c`),
   recomputes with FLINT, and returns 1 with a diagnostic on the first mismatch.
   Make the mode strict when its input holds one kind of line: treat any unrecognized nonempty
   line as an error, and fail an input in which nothing was checked (`require_some_lines`).
   Otherwise a change to the demo's output format makes every run pass without checking anything.
   The polynomial modes do this, and share `split_polynomial_scalar_line` and
   `fmpz_poly_set_str_malachite` from `oracle/util.c`.
3. Register the demo in `src/main.rs` with `check_demo_against_flint`.
4. Prove the harness can fail: corrupt one line of `test-out.txt` by hand and check that the
   oracle catches it.
