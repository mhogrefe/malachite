# oracle-test: differential testing against FLINT and Azurite, without FFI

This harness checks Malachite functions against other libraries by comparing text, not by
linking: a Malachite demo prints one `input = output` line per case, and an oracle re-reads the
file, recomputes every line, and exits nonzero on the first disagreement. There is no FFI, no
`unsafe`, and no foreign build coupling in the Malachite crates themselves. Two oracles exist: a
small C program (the sources in `oracle/`) built against FLINT, and the `oracle` executable of
[Azurite](https://github.com/mhogrefe/azurite), a Lean library whose arithmetic is formally
verified, so that a disagreement there is a bug in Malachite, in a demo's output format, or in
the oracle's reading of Malachite's conventions, never in the reference.

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

- For the Azurite oracle, an **Azurite checkout** that has fetched its Mathlib cache
  (`lake exe cache get`) and been built once (`lake build`), with the Lean toolchain (`elan`)
  installed. The default location is `../../../azurite` relative to this directory (that is,
  `~/azurite` when the Malachite repository is at `~/rust/malachite`); override it with the
  `MALACHITE_AZURITE_DIR` environment variable. The driver runs `lake build oracle` there before
  testing, which is a no-op when the executable is up to date.

## Running

```
cargo run --release -- flint
cargo run --release -- azurite
```

from this directory, or `cargo run --release` for both. `build.sh` runs them as two steps. For
FLINT, the driver compiles the oracle sources in `oracle/` on demand (into `target/`); for Azurite,
it builds `lake exe oracle` in the Azurite checkout. It then runs each registered Malachite demo in
all three generator modes (`exhaustive`, `random`, `special_random`) at 10000 lines each, checking
every run against the oracle, after the unit-row files described below. Any disagreement fails the
run with the offending line reported. The demo output lands in `test-out.txt`, which is shared
state: don't run the driver and a manual demo regeneration concurrently.

## Unit rows

Besides the generated cases, each backend checks hand-picked edge cases: the files under
`unit/flint/<mode>/` and `unit/azurite/<mode>/` hold lines in the format the demos print, and the
driver hands every `<mode>/<name>.txt` to the oracle's mode `<mode>` as it is, before the demos.
Most files are the rows of one `test_*` function in Malachite's own tests, reformatted, and are
named after it (`unit/azurite/az_nat_div_round/test_div_round.txt`); the FLINT files that were
once constants in `src/main.rs` keep those constants' names. `cargo run --release -- units` checks
the unit files alone, for both backends, which is the quick loop while adding rows. An input
string in a unit file is printed bare, exactly as the demo would print it.

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
| `fmpz_mod_poly_evaluate_fmpz` | `fmpz_mod_poly_evaluate_fmpz`, with the modulus 2^pow or m | `demo_natural_polynomial_mod_power_of_2_evaluate`, `demo_natural_polynomial_mod_evaluate`, and their `_ref` versions (malachite-nz); `demo_unsigned_polynomial_mod_power_of_2_evaluate`, `demo_unsigned_polynomial_mod_evaluate`, and their `_ref` versions (malachite-base); plus unit rows |
| `fmpz_poly_evaluate_mod` | `fmpz_poly_evaluate_mod` | `demo_integer_polynomial_mod_evaluate_u64` (malachite-nz), plus unit rows |
| `fmpq_poly_add` | `fmpq_poly_add` | `demo_rational_polynomial_add` and its `_val_ref`, `_ref_val`, `_ref_ref`, `_assign`, and `_assign_ref` versions (malachite-q), plus unit rows; identically written operands take FLINT's aliased path |
| `fmpq_poly_sub` | `fmpq_poly_sub` | the same six `demo_rational_polynomial_sub` demos (malachite-q), plus unit rows |
| `fmpz_poly_add_series` | `fmpz_poly_add_series` | `demo_integer_polynomial_add_truncated`, `demo_natural_polynomial_add_truncated`, and their `_val_ref`, `_ref_val`, `_ref_ref`, `_assign`, and `_assign_ref` versions (malachite-nz), plus unit rows |
| `_fmpz_poly_mul`, `_fmpz_poly_mullow`, `_fmpz_poly_mulhigh`, `_fmpz_poly_mulmid`, `_fmpz_poly_sqr`, `_fmpz_poly_sqrlow`, and their `_classical`, `_karatsuba`, `_KS`, and `_SS` versions, and `_fmpz_poly_mul_mid_default_mpn_ctx` | the FLINT function of the same name (`_sqr_SS` and `_sqrlow_SS` call `_fmpz_poly_mul_SS` and `_fmpz_poly_mullow_SS` with the same operand twice) | the slice-level `IntegerPolynomial` multiplication demos (malachite-nz), plus unit rows; see `oracle/fmpz_poly_mul_slices.c`. Where FLINT's Schönhage–Strassen result differs from its classical product (FLINT 3.6.0's `_fmpz_vec_set_fft` can get a sign wrong), the classical product is compared |
| `mpn_addmod_2expp1_1`, `flint_mpn_sumdiff_n`, `mpn_normmod_2expp1`, `mpn_negmod_2expp1`, `mpn_mul_2expmod_2expp1`, `mpn_div_2expmod_2expp1`, `fft_adjust`, `fft_adjust_sqrt2`, `butterfly_lshB`, `butterfly_rshB`, the `fft_`/`ifft_` butterflies and transforms, `flint_mpn_mulmod_2expp1_basecase`, `fft_naive_convolution_1`, `_fft_mulmod_2expp1`, `fft_mulmod_2expp1`, `fft_adjust_limbs`, `fft_split_limbs`, `fft_split_bits`, `fft_combine_limbs`, `fft_combine_bits`, `fft_convolution`, `_fmpz_vec_get_fft`, and `_fmpz_vec_set_fft` | the FLINT function of the same name | the demos of the port of FLINT's `fft` module, `natural::arithmetic::mul::schonhage_strassen` (malachite-nz), some with fewer lines because theirs are long, plus unit rows; see `oracle/fft.c`. Residues are compared limb for limb; `_` in a result matches anything. The `_fmpz_vec_set_fft` mode applies the fix FLINT made to its sign test after 3.6.0 (commit 7ad753d51c) |
| `fmpz_poly_sub_series` | `fmpz_poly_sub_series` | the same six `demo_integer_polynomial_sub_truncated` demos (malachite-nz), plus unit rows |
| `fmpq_poly_add_series` | `fmpq_poly_add_series` | `demo_rational_polynomial_add_truncated` and its `_val_ref`, `_ref_val`, `_ref_ref`, `_assign`, and `_assign_ref` versions (malachite-q), plus unit rows; identically written operands take FLINT's aliased path |
| `fmpq_poly_sub_series` | `fmpq_poly_sub_series` | the same six `demo_rational_polynomial_sub_truncated` demos (malachite-q), plus unit rows |
| `fmpz_poly_bit_pack` | `fmpz_poly_bit_pack`, or `fmpz_poly_evaluate_fmpz` at $2^b$ when $b = 0$ or a coefficient is wider than its field | `demo_integer_polynomial_bit_pack`, `_ref`, and `_small_bits`, and the same three for `natural_polynomial` (malachite-nz), plus unit rows |
| `fmpz_poly_bit_unpack` | `fmpz_poly_bit_unpack` | `demo_integer_polynomial_bit_unpack` and `_ref` (malachite-nz), plus unit rows |
| `fmpz_poly_bit_unpack_unsigned` | `fmpz_poly_bit_unpack_unsigned` | `demo_natural_polynomial_bit_unpack` and `_ref` (malachite-nz), plus unit rows |
| `sqrtmod_stress` | `fmpz_sqrtmod` | none — a memory-stress diagnostic, run manually |

The `sqrtmod` modes skip documented divergence windows, noted in comments in
`oracle/sqrtmod.c`;
all involve only composite moduli, where Malachite computes the mathematically expected value
and FLINT's behavior rests on undefined or wrapping operations.

## Azurite oracles

The Azurite modes are named after the `AzNat` operation they check, and each reads the output of
the `Natural` demos listed in `AZURITE_NATURAL_STAGES` in `src/main.rs`: the operators (`+`, `-`,
`*`, `/`, `%`, `^ 2`), `saturating_sub`, `pow`, `div_mod`/`div_rem`, `div_round`, `shr_round`
(unsigned and signed shift amounts), `gcd`, `coprime_with`, `mod_inverse`, `jacobi_symbol`,
`multi_crt`, `mod_power_of_2` and its `add`/`sub`/`mul`/`square` family, `floor_sqrt`, `sqrt_rem`,
`floor_root`, `checked_root`, `power_of_2`, `is_power_of_2`, `even`/`odd`,
`divisible_by_power_of_2`, `significant_bits`, `trailing_zeros`, `low_mask`, `limbs`,
`from_limbs_asc`, `to_digits_asc`, `from_string_base`, `from_str`, `to_string_base`, `cmp`,
`cmp_normalized`, `Natural::from`, `saturating_from`, and `wrapping_from`. The modes live in
`Azurite/Oracle/AzNat.lean` in the Azurite repository, with the line parsers in
`Azurite/Oracle/Parse.lean` and the mode table in `Azurite/Oracle/Main.lean`.

The `az_int_*` modes do the same for the `Integer` demos listed in `AZURITE_INTEGER_STAGES`,
following the "Malachite for Azurite Users: Integers" page: `+`, `-`, `*`, negation, the
Euclidean family (`div_euclidean`, `mod_euclidean`, `div_mod_euclidean`, Azurite's `/` and `%`),
the floor family (`div_mod`, `mod_op`), `div_exact`, `div_round`, `<<` and `>>` with unsigned and
signed amounts, `shr_round`, `pow`, `gcd`, `Natural::extended_gcd` (whose Bézout coefficients are
`Integer`s), `power_of_2`, `low_mask`, `is_power_of_2`, `even`/`odd`, `sign`, `significant_bits`,
`trailing_zeros`, `from_string_base`, `from_str`, `to_string`, `cmp` and `eq` against `Integer`,
`Natural`, and the machine integers, `Integer::from` from each of those, `from_sign_and_abs`,
`unsigned_abs`, and `wrapping_from`. They live in `Azurite/Oracle/AzInt.lean`. Malachite's
truncating `/` and `%` have no Azurite counterpart and are not checked.

The `az_zmod_pow2_*` modes check the `mod_power_of_2_*` demos listed in
`AZURITE_MOD_POWER_OF_2_STAGES` against Azurite's `AzZModPow2 k` residue type, following the
"Malachite for Azurite Users: Integers Modulo a Power of 2" page: `mod_power_of_2_add`, `_sub`,
`_mul`, `_square`, `_neg`, `_pow`, `_inverse` (including the "not invertible" lines), `_shl` and
`_shr` (spelled as shifts of the representative followed by reduction, since the type has no
shifts), `_is_reduced`, `eq_mod_power_of_2`, and `Integer::mod_power_of_2`. Each check first
requires the printed inputs to be reduced, the precondition Malachite asserts and the type's
invariant. They live in `Azurite/Oracle/AzZModPow2.lean`.

The `az_zmod_*` modes do the same for the general-modulus `mod_*` demos listed in
`AZURITE_MOD_STAGES` against `AzZMod m`, following the "Malachite for Azurite Users: Integers
Modulo a Natural" page: `mod_add`, `_sub`, `_mul`, `_square`, `_neg`, `_pow`, their
`_precomputed` variants (checked against the plain operations, since the precomputed data changes
nothing but speed), `_inverse`, `_shl`, `_shr`, `_is_reduced`, `eq_mod`, and `mod_div`, whose
quotient is not unique when the divisor is not a unit: the check is Malachite's documented
existence condition (`gcd(y, m)` divides `x`) and `q·y ≡ x` for the printed `q`. `mod_sqrt` has
no Azurite counterpart and is not checked. They live in `Azurite/Oracle/AzZMod.lean`.

The `az_rat_*` modes check the `Rational` demos of `malachite-q` listed in
`AZURITE_RATIONAL_STAGES` against `AzRat`, following the "Malachite for Azurite Users: Rationals"
page: the four field operations, negation, `abs`, `reciprocal`, `pow` with `u64` and `i64`
exponents, `<<` and `>>`, `floor` and `ceiling`, `Integer::rounding_from`, the base-2 and `u64`-base
logarithms (`ceiling_log_base` and `checked_log_base` derived from Azurite's floor and a power
comparison), `cmp` and `eq` against `Rational`, `Natural`, `Integer`, and the machine integers,
`sign`, the `from_naturals`/`from_integers`/`from_sign_and_naturals` constructors, `Rational::from`,
`to_string`, `from_str` (Malachite's grammar, with its `+` signs and nonzero denominator),
`from_sci_string` in any base, `to_sci`, `to_sci_with_options` (the `ToSciOptions` `Debug` text is
parsed into Azurite's `SciOptions`; `Exact` is Azurite's `toSciExact` predicate), `fmt_sci_valid`,
and `length_after_point_in_small_base`. They live in `Azurite/Oracle/AzRat.lean`, and they run
with the other Azurite stages.

The `az_float_*` modes check the `Float` demos of `malachite-float` listed in
`AZURITE_FLOAT_STAGES` against `AzFloat`, following the "Malachite for Azurite Users: Floats"
page. Only the `_debug` demos are used, since they print every value in the exact hexadecimal
format with its precision (`0x1.8#2`), which both libraries read and write; the `_extreme` ones
drive the exponent-range emulation. `AzFloat` has an unbounded exponent and no overflow or
underflow, so Azurite's `Azurite/Oracle/MalachiteFloat.lean` implements Malachite's documented
exponent range from the "Overflow and underflow" rules of its operations (`clamp`), using the
`Ordering` of Azurite's rounded result to settle the `Nearest` tie at half the smallest positive
value. Azurite also has no `-0.0`, so a zero result is compared regardless of its sign, and the
sign of a zero input is read from the line where a check needs it (`sign`, `ComparableFloat`'s
equality and order). Covered: `+ - * /` with their `_prec`, `_round`, and `_prec_round` forms,
`square`, `sqrt`, `reciprocal_sqrt`, negation, `abs`, `<<`/`>>`, `power_of_2`, `set_prec(_round)`, the
classification predicates, `sign`, `is_power_of_2`, `get_exponent`, `get_prec`, `to_significand`,
`ulp`, the precision constants, `Float` and `ComparableFloat` comparison and equality, comparison
with `Natural` and `Integer`, `Float::try_from` and `from_*_prec(_round)` from `Natural`,
`Integer`, and `Rational`, and `Float::from` for unsigned machine integers. They live in
`Azurite/Oracle/AzFloat.lean`.

Where the two libraries' conventions differ, the oracle applies the adjustment the mapping page
"Malachite for Azurite Users: Naturals" documents rather than skipping the line: Malachite's
`Exact` rounding mode (which Azurite lacks) is checked as an exact division or shift, a negative
`shr_round` amount as a left shift, `multi_crt`'s `None` as a violated precondition of Garner's
algorithm, and Malachite's base-62 digit rule for `from_string_base` and `to_string_base` is
evaluated with Azurite's arithmetic since `AzNat`'s own string functions stop at base 36. For
`extended_gcd`, whose Bézout pair Azurite normalizes differently, the oracle checks the GCD, the
identity `s·x + t·y = g` in Azurite arithmetic, and the pair Malachite's documentation specifies.

## Adding an oracle

1. Write a Malachite demo that prints one line per case in a stable `input = output` format,
   and put the rows of the corresponding `test_*` function, in that format, in a file under
   `unit/<backend>/<mode>/`.
2. Add a mode file under `oracle/` that parses that format (one `run_*` entry point; declare it
   in `oracle/oracle.h` and add a row to the table in `oracle/main.c`),
   recomputes with FLINT, and returns 1 with a diagnostic on the first mismatch.
   Make the mode strict when its input holds one kind of line: treat any unrecognized nonempty
   line as an error, and fail an input in which nothing was checked (`require_some_lines`).
   Otherwise a change to the demo's output format makes every run pass without checking anything.
   The polynomial modes do this, and share `split_polynomial_scalar_line` and
   `fmpz_poly_set_str_malachite` from `oracle/util.c`.
3. Register the demo in `src/main.rs` with `check_demo_against_flint`; the unit file needs no
   registration.
4. Prove the harness can fail: corrupt one line of `test-out.txt` by hand and check that the
   oracle catches it.

For an Azurite oracle, step 2 is a `check*` function in `Azurite/Oracle/AzNat.lean` (or a new
file for another type) and a row in the `modes` table of `Azurite/Oracle/Main.lean`, written
against the parsers in `Azurite/Oracle/Parse.lean`; the strictness rule is built into the runner,
which errors on any unrecognized nonempty line and on an input in which nothing was checked. Step 3
is a row in one of the `AZURITE_*_STAGES` tables, and the unit rows a file under
`unit/azurite/<mode>/`. Azurite
is Apache-licensed and must not derive code from
Malachite or the LGPL libraries Malachite ports, so an oracle implements Malachite's *documented*
behavior, never a translation of its code.
