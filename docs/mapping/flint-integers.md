---
layout: default
title: "Malachite for FLINT Users: Integers"
permalink: /mapping/flint-integers/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Integers

This page maps the functions of FLINT's integer type, `fmpz_t`, onto their Malachite
counterparts:
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), from
the `malachite-nz` crate. It follows the organization of the
[fmpz.h chapter](https://flintlib.org/doc/fmpz.html) of the FLINT manual, as of FLINT 3.6.0, and
is a companion to [Malachite for GMP Users: Integers](/mapping/gmp-integers/); the
[mapping index](/mapping/) lists the whole family. The
[Conventions](/mapping/gmp-integers/#conventions) of the GMP page, including
[Allocation](/mapping/gmp-integers/#allocation), apply here unchanged: where FLINT writes into an
output argument, Malachite returns the result. Functions whose names begin with an underscore are
FLINT-internal and are omitted.

## Conventions {#conventions}

### The `fmpz` representation

An `fmpz` stores values of absolute value at most $$2^{62}-1$$ inline and promotes larger ones to
a GMP integer. A
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) likewise
stores values below $$2^{64}$$ inline, and an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is a
sign and a `Natural`. (With 32-bit limbs, the thresholds are $$2^{30}-1$$ and $$2^{32}$$.) The
macros and constants of the tagged representation (`COEFF_MAX`, `COEFF_MIN`, `COEFF_IS_MPZ`,
`PTR_TO_COEFF`, `COEFF_TO_PTR`) have no counterparts, since Malachite's representation is not part
of its public API. Arrays of `fmpz` become `Vec<Integer>` or slices.

### Word types

FLINT's `ulong` and `slong` are `u64` and `i64` on every 64-bit platform, Windows included. As on
the GMP page, most `_ui` and `_si` variants collapse into their base function's row.

### Aliasing

FLINT functions generally permit outputs to alias inputs. Rust does not; use the assign and
by-value forms described under [Allocation](/mapping/gmp-integers/#allocation). The case
`fmpz_mul(x, x, x)` is
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html),
with `x.square_assign()` as the in-place form.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Memory management](https://flintlib.org/doc/fmpz.html#memory-management) {#memory-management}

The [discussion on the GMP page](/mapping/gmp-integers/#initializing-integers) of why Rust has no
separate initialization step and nothing to clear applies here.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_init (fmpz_t f)` | [`Natural::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`Integer::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void fmpz_init2 (fmpz_t f, ulong limbs)` | |
| — | `void fmpz_clear (fmpz_t f)` | |
| ✓ | `void fmpz_init_set (fmpz_t f, const fmpz_t g)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `void fmpz_init_set_ui (fmpz_t f, ulong g)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ✓ | `void fmpz_init_set_si (fmpz_t f, slong g)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |

**`fmpz_init`.** `let x = Integer::ZERO;`, which allocates nothing.
[`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) also returns zero.

**`fmpz_init2`.** A capacity hint; see the note under
[`mpz_init2`](/mapping/gmp-integers/#initializing-integers) on the GMP page.

**`fmpz_clear`.** Memory is released when the value goes out of scope.

**`fmpz_init_set`, `fmpz_init_set_ui`, `fmpz_init_set_si`.** `g.clone()`, `Natural::from(g)`,
and `Integer::from(g)`, as in the
[corresponding GMP section](/mapping/gmp-integers/#combined-initialization-and-assignment);
[`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) turns a signed value
into a `Natural`.

## [Random generation](https://flintlib.org/doc/fmpz.html#random-generation) {#random-generation}

FLINT's `flint_rand_t` state has the same shape as GMP's `gmp_randstate_t`, and the
[same note](/mapping/gmp-integers/#random-number-functions) applies: Malachite's stream
generators take a [`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html)
and return an infinite iterator, and its single-value `get_*` forms borrow a source of random
words mutably (a
[`StripedBitSource`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/struct.StripedBitSource.html)
for the striped forms). Random generation lives behind the `random` feature, in the
[`natural::random`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html)
and
[`integer::random`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html)
modules.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_randbits_unsigned (fmpz_t f, flint_rand_t state, flint_bitcnt_t bits)` | [`get_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_bits.html) |
| ✓ | `void fmpz_randbits (fmpz_t f, flint_rand_t state, flint_bitcnt_t bits)` | [`get_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_bits.html), [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ≈ | `void fmpz_randtest_unsigned (fmpz_t f, flint_rand_t state, flint_bitcnt_t bits)` | [`random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals.html), [`striped_random_naturals`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.striped_random_naturals.html) |
| ≈ | `void fmpz_randtest (fmpz_t f, flint_rand_t state, flint_bitcnt_t bits)` | [`random_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.random_integers.html), [`striped_random_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.striped_random_integers.html) |
| ≈ | `void fmpz_randtest_not_zero (fmpz_t f, flint_rand_t state, flint_bitcnt_t bits)` | [`random_nonzero_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.random_nonzero_integers.html) |
| ✓ | `void fmpz_randm (fmpz_t f, flint_rand_t state, const fmpz_t m)` | [`get_random_natural_less_than`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_less_than.html), [`random_naturals_less_than`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.random_naturals_less_than.html) |
| ≈ | `void fmpz_randtest_mod (fmpz_t f, flint_rand_t state, const fmpz_t m)` | [`striped_random_natural_range`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.striped_random_natural_range.html) |
| ≈ | `void fmpz_randtest_mod_signed (fmpz_t f, flint_rand_t state, const fmpz_t m)` | [`striped_random_integer_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.striped_random_integer_range.html) |
| ✗ | `void fmpz_randprime (fmpz_t f, flint_rand_t state, flint_bitcnt_t bits, int proved)` | |

**`fmpz_randbits`, `fmpz_randbits_unsigned`.** `get_random_natural_with_bits` chooses uniformly
among the `Natural`s of exactly `bits` bits. FLINT's signed version also picks a random sign;
compose it with `Integer::from_sign_and_abs` and a random `bool`.

**The `randtest` family.** The distributions differ, hence ≈. FLINT draws the bit length
uniformly from `0` to `bits` inclusive, with a hard cap. Malachite's streams take a *mean* bit
length and draw sizes from a geometric distribution, with no cap; `random_nonzero_integers`
covers the `not_zero` variant. To reproduce FLINT's shape, draw a length and then a value of that
length;
[`get_random_natural_with_up_to_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_up_to_bits.html)
caps the length but is uniform over values, not over lengths.

**`fmpz_randm`, `fmpz_randtest_mod`, `fmpz_randtest_mod_signed`.** `fmpz_randm` is
`get_random_natural_less_than`, or the `random_naturals_less_than` stream. The two `randtest_mod`
forms bias toward the endpoints of the range (and toward zero, for the signed one); Malachite's
range generators are uniform or striped instead, with no endpoint bias. The signed range
`(-m/2, m/2]` is an ordinary range for
[`uniform_random_integer_inclusive_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.uniform_random_integer_inclusive_range.html)
or its striped counterpart.

**`fmpz_randprime`.** No counterpart: Malachite does not generate bignum primes.

FLINT's limb-level random functions, in
[mpn_extras](https://flintlib.org/doc/mpn_extras.html), are not mapped: Malachite's limb-level
functions are not public.

## [Conversion](https://flintlib.org/doc/fmpz.html#conversion) {#conversion}

Conversions to and from primitives follow the policy-trait scheme
[on the GMP page](/mapping/gmp-integers/#conversion-functions). The multi-word forms become Rust's
128-bit primitives and Malachite's limb-slice conversions.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong fmpz_get_si (const fmpz_t f)` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`WrappingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`SaturatingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) |
| ≈ | `ulong fmpz_get_ui (const fmpz_t f)` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`WrappingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`SaturatingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) |
| ≈ | `void fmpz_get_uiui (ulong * hi, ulong * low, const fmpz_t f)` | [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html), [`WrappingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) |
| ✓ | `ulong fmpz_get_nmod (const fmpz_t f, nmod_t mod)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ≈ | `double fmpz_get_d (const fmpz_t f)` | [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_float_from_integer/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_float_from_integer/index.html) |
| — | `void fmpz_set_mpf (fmpz_t f, const mpf_t x)` | |
| — | `void fmpz_get_mpf (mpf_t x, const fmpz_t f)` | |
| ✓ | `void fmpz_get_mpfr (mpfr_t x, const fmpz_t f, mpfr_rnd_t rnd)` | [`from_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_integer_prec_round) |
| ≈ | `double fmpz_get_d_2exp (slong * exp, const fmpz_t f)` | [`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html) |
| — | `void fmpz_get_mpz (mpz_t x, const fmpz_t f)` | |
| ✓ | `int fmpz_get_mpn (nn_ptr * n, fmpz_t n_in)` | [`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc) |
| ✓ | `char * fmpz_get_str (char * str, int b, const fmpz_t f)` | [`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html), [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✓ | `void fmpz_set_si (fmpz_t f, slong val)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ✓ | `void fmpz_set_ui (fmpz_t f, ulong val)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ≈ | `void fmpz_set_d (fmpz_t f, double c)` | [`RoundingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_float/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_float/index.html) |
| ✓ | `void fmpz_set_d_2exp (fmpz_t f, double d, slong exp)` | [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html), [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html) |
| ✓ | `void fmpz_neg_ui (fmpz_t f, ulong val)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html), [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html) |
| ✓ | `void fmpz_set_uiui (fmpz_t f, ulong hi, ulong lo)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ✓ | `void fmpz_neg_uiui (fmpz_t f, ulong hi, ulong lo)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html), [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html) |
| ✓ | `void fmpz_set_signed_uiui (fmpz_t f, ulong hi, ulong lo)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html) |
| ✓ | `void fmpz_set_signed_uiuiui (fmpz_t f, ulong hi, ulong mid, ulong lo)` | [`from_twos_complement_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_twos_complement_limbs_asc) |
| ✓ | `void fmpz_set_ui_array (fmpz_t out, const ulong * in, slong n)` | [`from_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_limbs_asc) |
| ✓ | `void fmpz_set_signed_ui_array (fmpz_t out, const ulong * in, slong n)` | [`from_twos_complement_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_twos_complement_limbs_asc) |
| ✓ | `void fmpz_get_ui_array (ulong * out, slong n, const fmpz_t in)` | [`to_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.to_limbs_asc) |
| ✓ | `void fmpz_get_signed_ui_array (ulong * out, slong n, const fmpz_t in)` | [`to_twos_complement_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.to_twos_complement_limbs_asc) |
| ✓ | `void fmpz_set_mpn_large (fmpz_t z, nn_srcptr src, slong n, int negative)` | [`from_limbs_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.from_limbs_asc), [`from_sign_and_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.from_sign_and_abs) |
| ✓ | `void fmpz_get_signed_uiui (ulong * hi, ulong * lo, const fmpz_t in)` | [`WrappingFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) |
| — | `void fmpz_set_mpz (fmpz_t f, const mpz_t x)` | |
| ✓ | `int fmpz_set_str (fmpz_t f, const char * str, int b)` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html), [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ✓ | `void fmpz_set_ui_smod (fmpz_t f, ulong x, ulong m)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html) |
| — | `void flint_mpz_init_set_readonly (mpz_t z, const fmpz_t f)` | |
| — | `void flint_mpz_clear_readonly (mpz_t z)` | |
| — | `void fmpz_init_set_readonly (fmpz_t f, const mpz_t z)` | |
| — | `void fmpz_clear_readonly (fmpz_t f)` | |

**`fmpz_get_si`, `fmpz_get_ui`.** The
[policy-trait family](/mapping/gmp-integers/#conversion-functions) from the GMP page: `TryFrom`
fails when the value does not fit, `WrappingFrom` keeps the low bits, `SaturatingFrom` clamps,
and
[`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html)
is the fits-test. FLINT leaves `fmpz_get_ui` of a negative number undefined (GMP's `mpz_get_ui`
returns the low bits of the absolute value); each Malachite trait defines its result.

**Words in pairs and arrays.** The two-word forms are Rust's 128-bit primitives:
`fmpz_set_uiui(f, hi, lo)` is `Natural::from` of a `u128`, `fmpz_set_signed_uiui` is
`Integer::from` of an `i128`, and the two `neg` forms compose with
[`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html). `fmpz_get_uiui` is a `u128`
conversion under whichever policy you choose, and `fmpz_get_signed_uiui` (the value modulo
$$2^{128}$$ in two's complement) is `i128::wrapping_from`. To go between a `u128` and a word pair,
use
[`JoinHalves`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.JoinHalves.html)
(`u128::join_halves(hi, lo)`) and
[`SplitInHalf`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SplitInHalf.html)
(`split_in_half()`, returning `(hi, lo)`). The array forms are the limb-slice conversions:
`fmpz_set_ui_array` is `Natural::from_limbs_asc`, the `signed` versions (including
`fmpz_set_signed_uiuiui`) are `Integer::from_twos_complement_limbs_asc`, and the `get` versions
are `to_limbs_asc` and `to_twos_complement_limbs_asc`. These return a minimal-length
[`Vec`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html) where FLINT fills exactly `n`
words; pad with zero words (or sign words, for two's complement) if you need a fixed width.
`fmpz_set_mpn_large` is `from_limbs_asc` plus `Integer::from_sign_and_abs`, without its
preconditions: `from_limbs_asc` accepts any slice, unnormalized or empty. `fmpz_get_mpn` is
`to_limbs_asc`.

**`fmpz_get_d`, `fmpz_set_d`, and the `2exp` pair.** `fmpz_get_d` truncates, so it is
`f64::rounding_from(&f, Down)`; where FLINT leaves out-of-range values undefined, the
[defined overflow behavior](/mapping/gmp-integers/#conversion-functions) on the GMP page applies.
`fmpz_set_d` is `Integer::rounding_from(c, Down)`; an infinity or NaN fails `try_from` and makes
`rounding_from` panic, and a subnormal (undefined in FLINT) truncates to zero. `fmpz_get_d_2exp`
is
[`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html),
with the mantissa-normalization difference noted
[on the GMP page](/mapping/gmp-integers/#conversion-functions) for `mpz_get_d_2exp`.
`fmpz_set_d_2exp` rounds $$d \cdot 2^{exp}$$ to the nearest integer; the exact spelling is
`Rational::try_from(d)` shifted by `exp`, then `Integer::rounding_from(&q, Nearest)`.

**The GMP and MPFR boundary.** The conversions to and from `mpz_t` and `mpf_t`, including the
four `readonly` functions, have no counterpart because Malachite is not built on GMP. To pass a
value to C, or to GMP through bindings such as [rug](https://docs.rs/rug/latest/rug/), use the
limb-slice conversions above, strings, or [serde](https://serde.rs/). For `mpf_t`, see the
[note on the GMP page](/mapping/gmp-integers/#assigning-integers). `fmpz_get_mpfr` is
`Float::from_integer_prec_round(f, prec, rnd)` on
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), which
takes the target precision explicitly and also returns an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) giving the rounding
direction.

**`fmpz_get_nmod`.** Reduce with `Natural::from(m)` and
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html),
then convert the result to a word. Malachite has no public counterpart of the precomputed
`nmod_t` context; see [the mod-n page](/mapping/flint-integers-mod-n/#conventions).

**`fmpz_get_str`, `fmpz_set_str`.** `to_string_base` and `from_string_base`, covering bases 2
through 62 with GMP's digit alphabet, as described in
[the notes on the GMP page](/mapping/gmp-integers/#assigning-integers):
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) replaces the `-1`
return and [`String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html) replaces
the buffer. FLINT has no uppercase mode, so `to_string_base` covers all of `fmpz_get_str`. When
parsing, FLINT skips surrounding whitespace and tolerates interior whitespace; Malachite accepts
no whitespace, accepts a single leading `+` that FLINT rejects, and, like FLINT, rejects a doubled
minus sign.

**`fmpz_set_ui_smod`.** The representative of `x` modulo `m` in `(-m/2, m/2]`, which is
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html),
returning an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html). FLINT
requires (without checking) `0 <= x < m`; `balanced_mod` accepts any
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and
agrees wherever FLINT's precondition holds.

## [Input and output](https://flintlib.org/doc/fmpz.html#input-and-output) {#input-and-output}

As on [the GMP page](/mapping/gmp-integers/#input-and-output-functions), Malachite has no
`FILE *` functions: formatting and parsing work with any Rust reader or writer.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_read (fmpz_t f)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ✓ | `int fmpz_fread (FILE * file, fmpz_t f)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ≈ | `size_t fmpz_inp_raw (fmpz_t x, FILE * fin)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ✓ | `int fmpz_print (const fmpz_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✓ | `int fmpz_fprint (FILE * fs, const fmpz_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `size_t fmpz_out_raw (FILE * fout, const fmpz_t x)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |

**`fmpz_read`, `fmpz_fread`.** Read a token and call `parse::<Integer>()` (decimal), or
`from_string_base` for other bases. Failure is an `Err` rather than a non-positive return.
FLINT rejects leading zeros; Malachite parses `"00123"`.

**`fmpz_print`, `fmpz_fprint`.** `print!("{x}")`, or `write!(f, "{x}")` for any
[`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html). FLINT returns the number
of characters written; `x.to_string().len()` gives it if needed.

**`fmpz_out_raw`, `fmpz_inp_raw`.** These use GMP's raw format, so the
[discussion on the GMP page](/mapping/gmp-integers/#input-and-output-functions) of `mpz_out_raw`
and `mpz_inp_raw` versus Malachite's [serde](https://serde.rs/) support applies, including the
encoding differences behind the ≈.

## [Basic properties and manipulation](https://flintlib.org/doc/fmpz.html#basic-properties-and-manipulation) {#basic-properties-and-manipulation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `size_t fmpz_sizeinbase (const fmpz_t f, int b)` | [`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html), [`FloorLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorLogBase.html) |
| ✓ | `flint_bitcnt_t fmpz_bits (const fmpz_t f)` | [`SignificantBits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html) |
| ✓ | `slong fmpz_size (const fmpz_t f)` | [`limb_count`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.limb_count) |
| ✓ | `int fmpz_sgn (const fmpz_t f)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |
| ≈ | `flint_bitcnt_t fmpz_val2 (const fmpz_t f)` | [`trailing_zeros`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.trailing_zeros) |
| — | `void fmpz_swap (fmpz_t f, fmpz_t g)` | |
| ✓ | `void fmpz_set (fmpz_t f, const fmpz_t g)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `void fmpz_zero (fmpz_t f)` | [`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void fmpz_one (fmpz_t f)` | [`ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| ✓ | `int fmpz_abs_fits_ui (const fmpz_t f)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html), [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |
| ✓ | `int fmpz_fits_si (const fmpz_t f)` | [`ConvertibleFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ConvertibleFrom.html) |
| ✓ | `void fmpz_setbit (fmpz_t f, ulong i)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `int fmpz_tstbit (const fmpz_t f, ulong i)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `ulong fmpz_abs_lbound_ui_2exp (slong * exp, const fmpz_t x, int bits)` | [`sci_mantissa_and_exponent_round`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.sci_mantissa_and_exponent_round), [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `ulong fmpz_abs_ubound_ui_2exp (slong * exp, const fmpz_t x, int bits)` | [`sci_mantissa_and_exponent_round`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.sci_mantissa_and_exponent_round), [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |

**`fmpz_sizeinbase`, `fmpz_bits`, `fmpz_size`, `fmpz_sgn`.** `fmpz_bits` is
`significant_bits()`, and `fmpz_size` is `limb_count()`, on a `Natural` or on an `Integer`'s
[`unsigned_abs_ref`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.unsigned_abs_ref);
all return 0 for zero. `fmpz_sgn` is
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html),
returning an [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html). For
`fmpz_sizeinbase`, the digit count is `f.floor_log_base(&base) + 1`, but `floor_log_base` panics
on zero, so handle zero separately.

**`fmpz_val2`.** `trailing_zeros()`, on a `Natural` or on an `Integer`'s absolute value. For zero,
FLINT returns 0 and Malachite returns
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), hence ≈.

**`fmpz_swap`, `fmpz_set`, `fmpz_zero`, `fmpz_one`.**
[`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html); `x = y.clone()` or
`x.clone_from(&y)`; `x = Integer::ZERO` and `x = Integer::ONE`.

**`fmpz_abs_fits_ui`, `fmpz_fits_si`.** `u64::convertible_from(f.unsigned_abs_ref())` and
`i64::convertible_from(&f)`.

**`fmpz_setbit`, `fmpz_tstbit`.** `f.set_bit(i)` and `f.get_bit(i)`, from
[`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html),
with negative values acting as their two's complement.

**`fmpz_abs_lbound_ui_2exp`, `fmpz_abs_ubound_ui_2exp`.**
[`sci_mantissa_and_exponent_round`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.sci_mantissa_and_exponent_round)
with `Floor` or `Ceiling` gives a mantissa and exponent bounding the value from below or above,
but its mantissa is an `f32` or `f64` in `[1, 2)` (24 or 53 bits), where FLINT's is an integer of
a caller-chosen width. For FLINT's shape at width `bits`, let
`e = x.significant_bits() as i64 - bits`; the mantissa is
`x.unsigned_abs_ref().shr_round(e, Floor).0`, or `Ceiling` for the upper bound. As in FLINT, the
ceiling can carry to a power of two and gain a bit; shift once more if the width must be exact.

## [Comparison](https://flintlib.org/doc/fmpz.html#comparison) {#comparison}

As on [the GMP page](/mapping/gmp-integers/#comparison-functions), comparisons return an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) instead of a signed
`int`, and the `_ui` and `_si` variants use the same traits as their base functions.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_cmp (const fmpz_t f, const fmpz_t g)` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| ✓ | `int fmpz_cmp_ui (const fmpz_t f, ulong g)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int fmpz_cmp_si (const fmpz_t f, slong g)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int fmpz_cmpabs (const fmpz_t f, const fmpz_t g)` | [`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html) |
| ✓ | `int fmpz_cmp2abs (const fmpz_t f, const fmpz_t g)` | [`OrdAbsDouble`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbsDouble.html) |
| ✓ | `int fmpz_equal (const fmpz_t f, const fmpz_t g)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_equal_ui (const fmpz_t f, ulong g)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_equal_si (const fmpz_t f, slong g)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_is_zero (const fmpz_t f)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_is_one (const fmpz_t f)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_is_pm1 (const fmpz_t f)` | [`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html) |
| ✓ | `int fmpz_is_even (const fmpz_t f)` | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |
| ✓ | `int fmpz_is_odd (const fmpz_t f)` | [`Parity`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Parity.html) |

**`fmpz_cmp2abs`.** `f.cmp_abs_double(&g)`, from
[`OrdAbsDouble`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbsDouble.html),
compares `|f|` against `|2g|` without forming `2g`. On
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), the
same comparison is
[`OrdDouble`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdDouble.html)'s
`cmp_double`.

**The predicates.** `fmpz_is_zero` and `fmpz_is_one` are `f == 0` and `f == 1`; `fmpz_is_pm1` is
`f.eq_abs(&1)`, from
[`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html);
and the parity pair is `f.even()` and `f.odd()`.

## [Basic arithmetic](https://flintlib.org/doc/fmpz.html#basic-arithmetic) {#basic-arithmetic}

This section is divided into subsections in the manual's order. As
[on the GMP page](/mapping/gmp-integers/#arithmetic-functions), the `*Assign` traits cover the
in-place case, every operator has borrowing forms, and the `_ui` and `_si` variants collapse into
their base function's row.

### Addition and multiplication

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_neg (fmpz_t f1, const fmpz_t f2)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `void fmpz_abs (fmpz_t f1, const fmpz_t f2)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |
| ✓ | `void fmpz_add (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `void fmpz_add_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `void fmpz_add_si (fmpz_t f, const fmpz_t g, slong h)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ≈ | `void fmpz_sub (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html), [`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html) |
| ≈ | `void fmpz_sub_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html) |
| ≈ | `void fmpz_sub_si (fmpz_t f, const fmpz_t g, slong h)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html) |
| ✓ | `void fmpz_mul (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void fmpz_mul_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void fmpz_mul_si (fmpz_t f, const fmpz_t g, slong h)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void fmpz_mul2_uiui (fmpz_t f, const fmpz_t g, ulong x, ulong y)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `void fmpz_mul_2exp (fmpz_t f, const fmpz_t g, ulong e)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `void fmpz_one_2exp (fmpz_t f, ulong e)` | [`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html) |
| ✓ | `void fmpz_addmul (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void fmpz_addmul_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void fmpz_addmul_si (fmpz_t f, const fmpz_t g, slong h)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ≈ | `void fmpz_submul (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ≈ | `void fmpz_submul_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ≈ | `void fmpz_submul_si (fmpz_t f, const fmpz_t g, slong h)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void fmpz_fmma (fmpz_t f, const fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_t d)` | [`MulAddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulAddMul.html), [`MulAddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulAddMulAssign.html) |
| ≈ | `void fmpz_fmms (fmpz_t f, const fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_t d)` | [`MulSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulSubMul.html), [`MulSubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulSubMulAssign.html) |

**The operators.** `-g`, `g.abs()`, `g + h`, `g - h`, `g * h`, and `g << e`, with
[`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html)
producing the absolute value as a `Natural`. The subtraction and `submul` rows are ≈ because
on `Natural` a negative difference panics;
[`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html)
and
[`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html)
are the alternatives. On `Integer` they match exactly.

**`fmpz_mul2_uiui`.** `g * Natural::from(u128::from(x) * u128::from(y))`; the word product
cannot overflow a `u128`.

**`fmpz_mul_2exp`, `fmpz_one_2exp`.** `g << e`, and `Integer::power_of_2(e)` from
[`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html).
FLINT's restriction that `e + FLINT_BITS` must not overflow does not apply.

**`fmpz_addmul`, `fmpz_submul`.** `f.add_mul_assign(&g, &h)` and `f.sub_mul_assign(&g, &h)`;
`f.add_mul(&g, &h)` and `f.sub_mul(&g, &h)` return the result instead.

**`fmpz_fmma`, `fmpz_fmms`.** $$ab + cd$$ and $$ab - cd$$ are `a.mul_add_mul(b, c, d)` and
`a.mul_sub_mul(b, c, d)`. The `fmms` row is ≈ for the same reason as `submul`: on `Natural` a
negative result panics, and
[`CheckedMulSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedMulSubMul.html)
and
[`SaturatingMulSubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingMulSubMul.html)
return `None` and 0 instead.

### Division with rounding

FLINT uses GMP's naming: `cdiv` rounds the quotient toward positive infinity, `fdiv` toward
negative infinity, and `tdiv` toward zero. As
[on the GMP page](/mapping/gmp-integers/#division-functions), `cdiv` is
[`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html)
and
[`CeilingMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingMod.html),
`fdiv` is
[`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html)
and [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html),
`tdiv` is
[`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html)
and [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html) (the `/` and `%`
operators), and a quotient alone under any mode is
[`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html)
with `Ceiling`, `Floor`, or `Down`. A zero divisor raises an exception in FLINT and panics in
Malachite.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_cdiv_qr (fmpz_t f, fmpz_t s, const fmpz_t g, const fmpz_t h)` | [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) |
| ✓ | `void fmpz_fdiv_qr (fmpz_t f, fmpz_t s, const fmpz_t g, const fmpz_t h)` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `void fmpz_tdiv_qr (fmpz_t f, fmpz_t s, const fmpz_t g, const fmpz_t h)` | [`DivRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html) |
| ≈ | `void fmpz_ndiv_qr (fmpz_t f, fmpz_t s, const fmpz_t g, const fmpz_t h)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_cdiv_q (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`CeilingDivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingDivMod.html) |
| ✓ | `void fmpz_fdiv_q (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html), [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `void fmpz_tdiv_q (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_cdiv_q_si (fmpz_t f, const fmpz_t g, slong h)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_fdiv_q_si (fmpz_t f, const fmpz_t g, slong h)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_tdiv_q_si (fmpz_t f, const fmpz_t g, slong h)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_cdiv_q_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_fdiv_q_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_tdiv_q_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRound.html) |
| ✓ | `void fmpz_cdiv_q_2exp (fmpz_t f, const fmpz_t g, ulong exp)` | [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `void fmpz_fdiv_q_2exp (fmpz_t f, const fmpz_t g, ulong exp)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `void fmpz_tdiv_q_2exp (fmpz_t f, const fmpz_t g, ulong exp)` | [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `void fmpz_fdiv_r (fmpz_t s, const fmpz_t g, const fmpz_t h)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_cdiv_r_2exp (fmpz_t s, const fmpz_t g, ulong exp)` | [`CeilingModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingModPowerOf2.html) |
| ✓ | `void fmpz_fdiv_r_2exp (fmpz_t s, const fmpz_t g, ulong exp)` | [`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html) |
| ✓ | `void fmpz_tdiv_r_2exp (fmpz_t s, const fmpz_t g, ulong exp)` | [`RemPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RemPowerOf2.html) |
| ✓ | `ulong fmpz_cdiv_ui (const fmpz_t g, ulong h)` | [`CeilingMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingMod.html), [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |
| ✓ | `ulong fmpz_fdiv_ui (const fmpz_t g, ulong h)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `ulong fmpz_tdiv_ui (const fmpz_t g, ulong h)` | [`Rem`](https://doc.rust-lang.org/nightly/std/ops/trait.Rem.html), [`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html) |

**`fmpz_ndiv_qr`.** `DivRound` with `Nearest` rounds the quotient to the nearest integer, but
ties differ: FLINT rounds a tie toward zero, Malachite to even. For 3 / 2, `fmpz_ndiv_qr` gives
1 and `(3).div_round(2, Nearest)` gives 2. To reproduce FLINT's rule, compare twice the truncated
remainder against the divisor (`cmp_abs_double`). `DivRound` returns only the quotient; the
remainder is `g - &q * h`, or one
[`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html).

**The `2exp` columns.**
[`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html)
with `Ceiling`, `Floor`, or `Down` gives `cdiv`, `fdiv`, or `tdiv`; a plain `g >> exp` floors.
The remainders are `ModPowerOf2` (floor, nonnegative), `CeilingModPowerOf2` (ceiling,
nonpositive), and `RemPowerOf2` (truncating, sign of `g`).

**`fmpz_cdiv_ui`, `fmpz_fdiv_ui`, `fmpz_tdiv_ui`.** These return the remainder's absolute value
as a word. `fmpz_fdiv_ui` is `Mod`; the other two compose with
[`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html),
since the ceiling remainder is nonpositive and the truncated one takes `g`'s sign.

### Exact division, divisibility, and modular reduction

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_divexact (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html), [`DivExactAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExactAssign.html) |
| ✓ | `void fmpz_divexact_si (fmpz_t f, const fmpz_t g, slong h)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✓ | `void fmpz_divexact_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✓ | `void fmpz_divexact2_uiui (fmpz_t f, const fmpz_t g, ulong x, ulong y)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✓ | `int fmpz_divisible (const fmpz_t f, const fmpz_t g)` | [`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html) |
| ✓ | `int fmpz_divisible_si (const fmpz_t f, slong g)` | [`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html) |
| ✓ | `int fmpz_divides (fmpz_t q, const fmpz_t f, const fmpz_t g)` | [`DivMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivMod.html) |
| ✓ | `void fmpz_mod (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |
| ✓ | `ulong fmpz_mod_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html) |
| ✓ | `void fmpz_smod (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html) |
| ✓ | `void fmpz_preinvn_init (fmpz_preinvn_t inv, const fmpz_t f)` | [`DivModPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivModPrecomputed.html) |
| — | `void fmpz_preinvn_clear (fmpz_preinvn_t inv)` | |
| ✓ | `void fmpz_fdiv_qr_preinvn (fmpz_t f, fmpz_t s, const fmpz_t g, const fmpz_t h, const fmpz_preinvn_t hinv)` | [`DivModPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivModPrecomputed.html), [`DivAssignModPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivAssignModPrecomputed.html) |

**The `divexact` family.** `g.div_exact(&h)`. As in FLINT, exactness is not checked: if `h`
does not divide `g`, `div_exact` may panic or return a meaningless result. For
`fmpz_divexact2_uiui`, form `x * y` as a `u128` and divide once.

**`fmpz_divisible`, `fmpz_divides`.** `f.divisible_by(&g)`. For `fmpz_divides`, use
`let (q, r) = f.div_mod(&g)` and test `r == 0`. FLINT defines `fmpz_divides(q, f, 0)` to report
whether `f` is zero, while `div_mod` by zero panics, so test the divisor first if it can be zero.

**`fmpz_mod`, `fmpz_mod_ui`.** `fmpz_mod` returns a nonnegative remainder whatever the sign of
`h`, which is
[`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html)
(`mod_euclidean`, returning a `Natural`). For a positive divisor, as in `fmpz_mod_ui`,
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html)
agrees.

**`fmpz_smod`.** The remainder in `(-|h|/2, |h|/2]`, which is
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html).
As in FLINT, only the magnitude of `h` matters, and a remainder of exactly `|h|/2` is positive.

**The `preinvn` trio.**
[`DivModPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivModPrecomputed.html):
`Integer::precompute_div_mod_data(&h)` corresponds to `fmpz_preinvn_init`, and
`g.div_mod_precomputed(&h, &data)` to `fmpz_fdiv_qr_preinvn` (floor division).
[`DivAssignModPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivAssignModPrecomputed.html)
is the in-place form. As in FLINT, the data depends only on `|h|`. `fmpz_preinvn_clear` is
replaced by [`Drop`](https://doc.rust-lang.org/nightly/std/ops/trait.Drop.html).

### Powers and logarithms

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_pow_ui (fmpz_t f, const fmpz_t g, ulong x)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ✓ | `void fmpz_ui_pow_ui (fmpz_t f, ulong g, ulong x)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html) |
| ✓ | `int fmpz_pow_fmpz (fmpz_t f, const fmpz_t g, const fmpz_t x)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) |
| ≈ | `void fmpz_powm_ui (fmpz_t f, const fmpz_t g, ulong e, const fmpz_t m)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) |
| ≈ | `void fmpz_powm (fmpz_t f, const fmpz_t g, const fmpz_t e, const fmpz_t m)` | [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html) |
| ✓ | `slong fmpz_clog (const fmpz_t x, const fmpz_t b)` | [`CeilingLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingLogBase.html) |
| ✓ | `slong fmpz_clog_ui (const fmpz_t x, ulong b)` | [`CeilingLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingLogBase.html) |
| ✓ | `slong fmpz_flog (const fmpz_t x, const fmpz_t b)` | [`FloorLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorLogBase.html) |
| ✓ | `slong fmpz_flog_ui (const fmpz_t x, ulong b)` | [`FloorLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorLogBase.html) |
| ✓ | `double fmpz_dlog (const fmpz_t x)` | [`approx_log`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.approx_log) |

**`fmpz_pow_ui`, `fmpz_ui_pow_ui`, `fmpz_pow_fmpz`.** `g.pow(x)`, with $$0^0 = 1$$ in both
libraries. For `fmpz_pow_fmpz`, convert the exponent with `u64::try_from(&x)`; its `Err` takes
the place of FLINT's failure return (and of FLINT's throw on a negative exponent). FLINT succeeds
for bases -1, 0, and 1 at any exponent, so handle those before converting.

**`fmpz_powm`, `fmpz_powm_ui`.** [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html),
with the [differences explained on the GMP page](/mapping/gmp-integers/#exponentiation-functions)
for `mpz_powm`: the base must already be reduced, the residues are `Natural`s, and a negative
exponent is spelled as
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
followed by a positive power. FLINT's `abort` on a zero modulus is a panic.

**`fmpz_clog`, `fmpz_flog`.** `x.ceiling_log_base(&b)` and `x.floor_log_base(&b)`. Both panic
when `x < 1` or `b < 2`, inputs FLINT assumes do not occur.

**`fmpz_dlog`.**
[`approx_log`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.approx_log)
on [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
via `Rational::from(&x)`. FLINT's result depends on the platform's C `log`; Malachite's uses the
[libm](https://docs.rs/libm/latest/libm/) crate and is identical on every platform.

### Roots

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_sqrtmod (fmpz_t b, const fmpz_t a, const fmpz_t p)` | [`ModSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSqrt.html) |
| ✓ | `void fmpz_sqrt (fmpz_t f, const fmpz_t g)` | [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html) |
| ✓ | `void fmpz_sqrtrem (fmpz_t f, fmpz_t r, const fmpz_t g)` | [`SqrtRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SqrtRem.html) |
| ✓ | `int fmpz_is_square (const fmpz_t f)` | [`IsSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsSquare.html) |
| ≈ | `int fmpz_root (fmpz_t r, const fmpz_t f, slong n)` | [`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html), [`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html), [`CheckedRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedRoot.html) |
| ✓ | `int fmpz_is_perfect_power (fmpz_t root, const fmpz_t f)` | [`ExpressAsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.ExpressAsPower.html) |

**`fmpz_sqrt`, `fmpz_sqrtrem`, `fmpz_is_square`.** `g.floor_sqrt()`, `g.sqrt_rem()`, and
`f.is_square()`; FLINT's exception on a negative operand is a panic. `sqrt_rem` returns a tuple.
`IsSquare` is implemented only for `Natural`; for an `Integer`, test the sign first.

**`fmpz_root`.** The same ≈ as `mpz_root`,
[explained on the GMP page](/mapping/gmp-integers/#root-extraction-functions): FLINT truncates
toward zero, so for a negative operand with odd `n` use
[`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html),
and for a nonnegative one
[`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html).
For FLINT's exactness flag, use
[`CheckedRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedRoot.html)
(the root only when exact) or
[`RootRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RootRem.html).

**`fmpz_sqrtmod`.** `a.mod_sqrt(&p)`, returning an `Option` where FLINT sets an output and
returns a flag;
[`ModSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSqrt.html)
is implemented for
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and the
unsigned primitives. As in FLINT, `p` is assumed prime but not checked, and for a composite `p`
the results match FLINT's, including missed and spurious roots, with two exceptions where
Malachite returns the mathematically expected value: even moduli between 50 and 600, and
`p = 2^64 - 1` or `2^64 - 3`.

**`fmpz_is_perfect_power`.**
[`ExpressAsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.ExpressAsPower.html):
`express_as_power` on 64 returns `Some((2, 6))`, the root and the exponent, where
`fmpz_is_perfect_power` sets the root and returns the exponent. As in FLINT, 0 and 1 count as
perfect powers, and the root is not necessarily the smallest. For a negative
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), the
exponent is always odd ($$-8 = (-2)^3$$). The predicate alone is
[`IsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPower.html).

### Combinatorial functions and fused operations

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_fac_ui (fmpz_t f, ulong n)` | [`Factorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Factorial.html) |
| ✓ | `void fmpz_fib_ui (fmpz_t f, ulong n)` | [`Fibonacci`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Fibonacci.html) |
| ✓ | `void fmpz_bin_uiui (fmpz_t f, ulong n, ulong k)` | [`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html) |
| ✓ | `void fmpz_rfac_ui (fmpz_t r, const fmpz_t x, ulong k)` | [`RisingFactorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RisingFactorial.html) |
| ✓ | `void fmpz_rfac_uiui (fmpz_t r, ulong x, ulong k)` | [`RisingFactorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RisingFactorial.html) |
| ✓ | `void fmpz_mul_tdiv_q_2exp (fmpz_t f, const fmpz_t g, const fmpz_t h, ulong exp)` | [`MulShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulShrRound.html), [`MulShrRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulShrRoundAssign.html) |
| ✓ | `void fmpz_mul_si_tdiv_q_2exp (fmpz_t f, const fmpz_t g, slong x, ulong exp)` | [`MulShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulShrRound.html), [`MulShrRoundAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulShrRoundAssign.html) |

**`fmpz_fac_ui`, `fmpz_fib_ui`, `fmpz_bin_uiui`.** `Natural::factorial(n)`,
`Natural::fibonacci(n)`, and `Natural::binomial_coefficient(n, k)`.

**`fmpz_rfac_ui`, `fmpz_rfac_uiui`.** The rising factorial $$x (x+1) \cdots (x+k-1)$$ is
`x.rising_factorial(k)`: `Integer::rising_factorial` for `fmpz_rfac_ui`, with the same handling
of negative bases, and `Natural::rising_factorial` for `fmpz_rfac_uiui`, with a base of any size.

**`fmpz_mul_tdiv_q_2exp`, `fmpz_mul_si_tdiv_q_2exp`.** `(&g).mul_shr_round(&h, exp, Down).0`;
`Down` is truncation toward zero. The second return value reports how the result compares with
the exact quotient. A plain `>>` floors, so a negative product needs `Down` explicitly. For the
`si` variant, pass `Integer::from(x)`.

## [Greatest common divisor](https://flintlib.org/doc/fmpz.html#greatest-common-divisor) {#greatest-common-divisor}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_gcd_ui (fmpz_t f, const fmpz_t g, ulong h)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ✓ | `void fmpz_gcd (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ✓ | `void fmpz_gcd3 (fmpz_t f, const fmpz_t a, const fmpz_t b, const fmpz_t c)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html) |
| ✓ | `void fmpz_lcm (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html) |
| ≈ | `void fmpz_gcdinv (fmpz_t d, fmpz_t a, const fmpz_t f, const fmpz_t g)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html), [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) |
| ≈ | `void fmpz_xgcd (fmpz_t d, fmpz_t a, fmpz_t b, const fmpz_t f, const fmpz_t g)` | [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) |
| ✓ | `void fmpz_xgcd_canonical_bezout (fmpz_t d, fmpz_t a, fmpz_t b, const fmpz_t f, const fmpz_t g)` | [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) |
| ⚙ | `void fmpz_xgcd_partial (fmpz_t co2, fmpz_t co1, fmpz_t r2, fmpz_t r1, const fmpz_t L)` | internal |

**`fmpz_gcd`, `fmpz_gcd3`, `fmpz_lcm`.**
[`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html)
and
[`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html)
are defined on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html); since
FLINT's results are nonnegative, take
[`unsigned_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.unsigned_abs)
of each
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) first.
`fmpz_gcd3` is `a.gcd(&b).gcd(&c)`.

**The `xgcd` family.** `fmpz_xgcd_canonical_bezout` normalizes the cofactors by
$$|a| < |g/2d|$$ and $$|b| < |f/2d|$$, the same normalization, edge cases included, as
[`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html),
so `extended_gcd` returns identical cofactors, as a tuple. Plain `fmpz_xgcd` follows the
`fmpz_gcdinv` convention instead, hence ≈.

**`fmpz_gcdinv`.** For `0 <= f < g`, returns `d = gcd(f, g)` and `a` with
$$af \equiv d \pmod{g}$$. For a modular inverse,
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
returns [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) when `f` and `g`
are not coprime. For `d` and a cofactor in every case, `extended_gcd`'s first cofactor satisfies
the same congruence after reduction modulo `g`, but is normalized differently, hence ≈.

**`fmpz_xgcd_partial`.** Malachite has an internal port of this function (Lehmer's extended GCD
with early termination), but it is not public. There is no public way to stop the reduction at a
bound;
[`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html)
computes the complete GCD and cofactors.

## [Modular arithmetic](https://flintlib.org/doc/fmpz.html#modular-arithmetic) {#modular-arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpz_remove (fmpz_t rop, const fmpz_t op, const fmpz_t f)` | [`RemovePower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.RemovePower.html) |
| ≈ | `int fmpz_invmod (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ✓ | `void fmpz_negmod (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html), [`NegMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegMod.html) |
| ≈ | `int fmpz_jacobi (const fmpz_t a, const fmpz_t n)` | [`JacobiSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.JacobiSymbol.html), [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✓ | `int fmpz_kronecker (const fmpz_t a, const fmpz_t n)` | [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✓ | `void fmpz_divides_mod_list (fmpz_t xstart, fmpz_t xstride, fmpz_t xlength, const fmpz_t a, const fmpz_t b, const fmpz_t n)` | [`ModDivList`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDivList.html) |

**`fmpz_remove`.** [`RemovePower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.RemovePower.html),
which returns the reduced number and the count as a tuple. As in FLINT, a zero input removes
nothing, and a factor of 1 or less is rejected (FLINT aborts; Malachite panics).

**`fmpz_invmod`.** [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
returns [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) instead of a
flag. It is ≈ for two reasons: `mod_inverse` requires its input already reduced and nonzero,
where FLINT reduces it; and FLINT treats every value as invertible modulo `h = ±1`, with inverse
0, while `mod_inverse` rejects the only residue, 0. Test for a unit modulus first if one can
occur.

**`fmpz_negmod`.** FLINT requires `g` to be reduced but does not check. `g.mod_neg(&h)`, from
[`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html),
has the same precondition and panics if it fails. `g.neg_mod(&h)`, from
[`NegMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegMod.html),
accepts any `g`. Both agree on reduced inputs.

**`fmpz_jacobi`, `fmpz_kronecker`.** As
[on the GMP page](/mapping/gmp-integers/#number-theoretic-functions), `KroneckerSymbol` matches
for every input, while `JacobiSymbol` requires an odd positive denominator. FLINT's `fmpz_jacobi`
does not check the parity or sign of `n`; where its result would be undefined, `jacobi_symbol`
panics, and `kronecker_symbol` returns the Kronecker symbol.

**`fmpz_divides_mod_list`.** The solutions of $$ax \equiv b \pmod n$$, as an arithmetic
progression. `b.mod_div_list(&a, &n)` returns `Some((start, stride, length))` in place of FLINT's
three outputs and all-zeros-on-failure convention; note that the dividend `b` is the receiver, as
in `mod_div` [on the mod-n page](/mapping/flint-integers-mod-n/#arithmetic). The results agree
exactly with FLINT's. FLINT reduces `a` modulo `n` itself, while `mod_div_list` requires both
inputs already reduced.

## [Bit packing and unpacking](https://flintlib.org/doc/fmpz.html#bit-packing-and-unpacking) {#bit-packing-and-unpacking}

The counterpart is
[`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html),
which reads or writes a block of bits at any position in a `Natural` or an `Integer`. Hold the
buffer as a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
(the [limb-slice conversions](#conversion) convert a raw buffer); FLINT's limb pointer plus
sub-word `shift` becomes a single `u64` bit position.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_bit_pack (ulong * arr, flint_bitcnt_t shift, flint_bitcnt_t bits, const fmpz_t coeff, int negate, int borrow)` | [`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |
| ≈ | `int fmpz_bit_unpack (fmpz_t coeff, ulong * arr, flint_bitcnt_t shift, flint_bitcnt_t bits, int negate, int borrow)` | [`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |
| ✓ | `void fmpz_bit_unpack_unsigned (fmpz_t coeff, const ulong * arr, flint_bitcnt_t shift, flint_bitcnt_t bits)` | [`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |

**`fmpz_bit_unpack_unsigned`.** `buffer.get_bits(pos, pos + bits)`.

**`fmpz_bit_pack`, `fmpz_bit_unpack`.** `assign_bits` and `get_bits` move raw fields, which
covers nonnegative coefficients with no borrow. FLINT's `negate` and `borrow` parameters, which
store negative coefficients in two's complement and carry a borrow into the next field, have no
counterpart, hence ≈. Also, `fmpz_bit_pack` adds its field into the array, assuming the bits
above `shift` are zero, while `assign_bits` overwrites the range.

## [Logic Operations](https://flintlib.org/doc/fmpz.html#logic-operations) {#logic-operations}

In both libraries, negative operands behave as infinitely sign-extended two's complement; see
[the GMP page](/mapping/gmp-integers/#logical-and-bit-manipulation-functions).

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_complement (fmpz_t r, const fmpz_t f)` | [`Not`](https://doc.rust-lang.org/nightly/std/ops/trait.Not.html) |
| ✓ | `void fmpz_clrbit (fmpz_t f, ulong i)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `void fmpz_combit (fmpz_t f, ulong i)` | [`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html) |
| ✓ | `void fmpz_and (fmpz_t r, const fmpz_t a, const fmpz_t b)` | [`BitAnd`](https://doc.rust-lang.org/nightly/std/ops/trait.BitAnd.html) |
| ✓ | `void fmpz_or (fmpz_t r, const fmpz_t a, const fmpz_t b)` | [`BitOr`](https://doc.rust-lang.org/nightly/std/ops/trait.BitOr.html) |
| ✓ | `void fmpz_xor (fmpz_t r, const fmpz_t a, const fmpz_t b)` | [`BitXor`](https://doc.rust-lang.org/nightly/std/ops/trait.BitXor.html) |
| ✓ | `ulong fmpz_popcnt (const fmpz_t a)` | [`CountOnes`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.CountOnes.html) |

**The operators and bits.** `!f` for the ones' complement; `a & b`, `a | b`, and `a ^ b` with
their `Assign` forms; and `f.clear_bit(i)` and `f.flip_bit(i)`, from
[`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html).

**`fmpz_popcnt`.** `count_ones()` on a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html). FLINT
leaves negative input undefined; on an `Integer`,
[`checked_count_ones`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.checked_count_ones)
returns [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) for negative
values.

## [Chinese remaindering](https://flintlib.org/doc/fmpz.html#chinese-remaindering) {#chinese-remaindering}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_CRT_ui (fmpz_t out, const fmpz_t r1, const fmpz_t m1, ulong r2, ulong m2, int sign)` | [`Crt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Crt.html), [`BalancedCrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedCrt.html) |
| ✓ | `void fmpz_CRT (fmpz_t out, const fmpz_t r1, const fmpz_t m1, const fmpz_t r2, const fmpz_t m2, int sign)` | [`Crt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Crt.html), [`BalancedCrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedCrt.html) |

**`fmpz_CRT`, `fmpz_CRT_ui`.** FLINT's `sign` flag selects the representative in
$$[0, m_1m_2)$$ or in $$(-m_1m_2/2, m_1m_2/2]$$; Malachite has a function for each.
`Natural::crt` is `sign = 0`, and `Integer::balanced_crt` is `sign = 1`, taking its first residue
as an `Integer` anywhere in $$[-m_1, m_1)$$, so a balanced result can be fed into the next
combination. FLINT treats coprimality as an unchecked precondition and throws when `m1` is not
invertible; both Malachite functions return an
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), `None` exactly when the
moduli share a factor. On their shared domain, the results agree with FLINT's.

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `void fmpz_multi_CRT_init (fmpz_multi_CRT_t CRT)` | internal |
| ⚙ | `int fmpz_multi_CRT_precompute (fmpz_multi_CRT_t CRT, const fmpz * moduli, slong len)` | internal |
| ⚙ | `void fmpz_multi_CRT_precomp (fmpz_t output, const fmpz_multi_CRT_t P, const fmpz * inputs, int sign)` | internal |
| ✓ | `int fmpz_multi_CRT (fmpz_t output, const fmpz * moduli, const fmpz * values, slong len, int sign)` | [`Natural::multi_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.multi_crt), [`Integer::multi_balanced_crt`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.multi_balanced_crt) |
| ⚙ | `void fmpz_multi_CRT_clear (fmpz_multi_CRT_t P)` | internal |

**The `fmpz_multi_CRT` family.** `Natural::multi_crt` is `fmpz_multi_CRT` with `sign = 0`, and
`Integer::multi_balanced_crt` is `sign = 1`. The precomputed context (`init`, `precompute`,
`precomp`, `clear`) is ported but not public, so each call takes the moduli afresh.
Failure is [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), under FLINT's
conditions: with one modulus, only 0 is rejected; with two or more, any 0 or 1, or any
non-coprime pair, is rejected. FLINT reduces arbitrary inputs, while Malachite requires each
residue already reduced modulo its modulus. Otherwise the results agree with FLINT's, edge cases
included.

| | FLINT | Malachite |
| :---: | --- | --- |
| ⚙ | `void fmpz_multi_mod_ui (ulong * out, const fmpz_t in, const fmpz_comb_t comb, fmpz_comb_temp_t temp)` | internal |
| ⚙ | `void fmpz_multi_CRT_ui (fmpz_t output, nn_srcptr residues, const fmpz_comb_t comb, fmpz_comb_temp_t ctemp, int sign)` | internal |
| ⚙ | `void fmpz_comb_init (fmpz_comb_t comb, nn_srcptr primes, slong num_primes)` | internal |
| ⚙ | `void fmpz_comb_temp_init (fmpz_comb_temp_t temp, const fmpz_comb_t comb)` | internal |
| ⚙ | `void fmpz_comb_clear (fmpz_comb_t comb)` | internal |
| ⚙ | `void fmpz_comb_temp_clear (fmpz_comb_temp_t temp)` | internal |

**The comb.** Malachite has an internal port of FLINT's multimodular comb, but it is not public.
To reduce a value modulo many word-sized moduli, use `Mod` for each; to recombine, use
`Natural::multi_crt` or `Integer::multi_balanced_crt`.

## [Primality testing](https://flintlib.org/doc/fmpz.html#primality-testing) {#primality-testing}

This section is not mapped row by row. Malachite does not test the primality of `Natural`s or
`Integer`s, so `fmpz_is_prime`, `fmpz_is_probabprime` and its variants, `fmpz_nextprime`, the
`fmpz_lucas_chain` functions, and `fmpz_divisor_in_residue_class_lenstra` have no counterparts.

## [Special functions](https://flintlib.org/doc/fmpz.html#special-functions) {#special-functions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_primorial (fmpz_t res, ulong n)` | [`Primorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Primorial.html) |
| ✗ | `void fmpz_euler_phi (fmpz_t res, const fmpz_t n)` | |
| ✗ | `void fmpz_factor_euler_phi (fmpz_t res, const fmpz_factor_t fac)` | |
| ✗ | `int fmpz_moebius_mu (const fmpz_t n)` | |
| ✗ | `int fmpz_factor_moebius_mu (const fmpz_factor_t fac)` | |
| ✗ | `void fmpz_divisor_sigma (fmpz_t res, ulong k, const fmpz_t n)` | |
| ✗ | `void fmpz_factor_divisor_sigma (fmpz_t res, ulong k, const fmpz_factor_t fac)` | |

**`fmpz_primorial`.** `Natural::primorial(n)`, the product of the primes up to and including
`n`, as in FLINT.

**The multiplicative functions.** Euler's totient, the Möbius function, and the divisor sums
$$\sigma_k$$ have no counterparts, nor does a factorization type corresponding to
`fmpz_factor_t`.
