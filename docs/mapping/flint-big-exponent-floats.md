---
layout: default
title: "Malachite for FLINT Users: Big-Exponent Floats"
permalink: /mapping/flint-big-exponent-floats/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Big-Exponent Floats

This page maps the functions of FLINT's arbitrary-precision floating-point type, `arf_t`, onto their
Malachite counterpart:
[`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html),
from the `malachite-float` crate. It follows the organization of the [arf.h
chapter](https://flintlib.org/doc/arf.html) of the FLINT manual, as of FLINT 3.6.0. The type is the
building block of FLINT's ball arithmetic, and
[`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html)
was designed after it, so most functions map directly; the [mapping index](/mapping/) lists the
other pages, and [the MPFR page](/mapping/mpfr-floats/) covers Malachite's other binary float,
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html). Functions
whose names begin with an underscore are omitted.

## Conventions {#conventions}

### One design, two implementations

An `arf_t` and a
[`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html)
share their essential design. Neither stores a precision: a value carries exactly the bits it needs,
and each rounded operation takes the target precision and a rounding mode as arguments. Both have
unbounded exponents, held by an `fmpz` in FLINT and by an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) in
Malachite, so neither overflows nor underflows. Both have the special values NaN, $$+\infty$$, and
$$-\infty$$, and neither has a negative zero or an unsigned infinity. They also share the exponent
convention: a finite nonzero $$x$$ has exponent $$e$$ when $$2^{e-1} \leq |x| < 2^e$$, as returned
by FLINT's `ARF_EXP` and by
[`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.get_exponent).

### Precision and exactness

FLINT's `ARF_PREC_EXACT` asks an operation for its exact result. In Malachite the exact forms are
separate functions: the operators `+`, `-`, `*`, and `%`, and the methods without a precision
argument, such as
[`positive_difference`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.positive_difference),
return the exact result, while the `_prec_round` and `_prec` forms round. Operations whose exact
results are not dyadic in general, such as division and square roots, have only rounded forms.

### Rounding and the inexact flag

FLINT's five rounding modes are among the six of Malachite's
[`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html):
`ARF_RND_DOWN` is `Down`, `ARF_RND_UP` is `Up`, `ARF_RND_FLOOR` is `Floor`, `ARF_RND_CEIL` is
`Ceiling`, and `ARF_RND_NEAR` is `Nearest`, both rounding ties to even. Malachite's sixth mode,
`Exact`, panics if the result is not representable. A rounded FLINT function returns a nonzero `int`
when the result is inexact; the Malachite function returns an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) instead, which also tells
the direction: `Less` if the rounded result is below the exact one, `Greater` if above, and `Equal`
if exact.

### NaN in comparisons

FLINT's comparisons return 0 when an operand is NaN (its documentation calls such comparisons
undefined), and `arf_equal` considers NaN equal to itself. Malachite's
[`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) and
[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) follow IEEE 754
instead: NaN is equal to nothing, and comparisons with it return `None`. The wrapper
[`ComparableBigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.ComparableBigExponentFloat.html)
provides FLINT's structural equality, and a total order, for use in hash maps and sorting.

### Mixed operands

FLINT has many variants that take a word or an `fmpz` operand (`arf_add_ui`, `arf_mul_fmpz`, and so
on). In Malachite, a word or an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) converts
exactly to a
[`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html)
with [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html), after which the
general function applies; the result is the same, rounded once. Operations with a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) operand,
which FLINT does not have, are separate functions such as
[`add_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_rational_prec_round).

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/arf.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `arf_struct` | [`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html) |
| ✓ | `arf_t` | [`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html) |
| ✓ | `arf_rnd_t` | [`RoundingMode`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html) |
| ✓ | `ARF_RND_DOWN` | [`Down`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html#variant.Down) |
| ✓ | `ARF_RND_UP` | [`Up`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html#variant.Up) |
| ✓ | `ARF_RND_FLOOR` | [`Floor`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html#variant.Floor) |
| ✓ | `ARF_RND_CEIL` | [`Ceiling`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html#variant.Ceiling) |
| ✓ | `ARF_RND_NEAR` | [`Nearest`](https://docs.rs/malachite-base/latest/malachite_base/rounding_modes/enum.RoundingMode.html#variant.Nearest) |
| ≈ | `ARF_PREC_EXACT` | the exact operations |

The types and rounding modes are discussed under [Conventions](#conventions), and so is
`ARF_PREC_EXACT`: Malachite has separate exact functions rather than a precision value meaning
"exact".

## [Memory management](https://flintlib.org/doc/arf.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arf_init(arf_t x)` | [`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void arf_clear(arf_t x)` |  |
| — | `slong arf_allocated_bytes(const arf_t x)` |  |

**`arf_init`, `arf_clear`.** `let x = BigExponentFloat::ZERO;` is the zero that `arf_init` produces,
and [`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) returns it too.
Dropping replaces clearing. Malachite does not report allocation sizes;
[`significant_bits`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.significant_bits)
gives the length of the significand.

## [Special values](https://flintlib.org/doc/arf.html#special-values) {#special-values}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arf_zero(arf_t res)` | [`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void arf_one(arf_t res)` | [`ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| ✓ | `void arf_pos_inf(arf_t res)` | [`INFINITY`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Infinity.html) |
| ✓ | `void arf_neg_inf(arf_t res)` | [`NEGATIVE_INFINITY`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NegativeInfinity.html) |
| ✓ | `void arf_nan(arf_t res)` | [`NAN`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.NaN.html) |
| ✓ | `int arf_is_zero(const arf_t x)` | [`is_zero`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.is_zero) |
| ≈ | `int arf_is_one(const arf_t x)` | `x == 1u32` |
| ≈ | `int arf_is_pos_inf(const arf_t x)` | `x == BigExponentFloat::INFINITY` |
| ≈ | `int arf_is_neg_inf(const arf_t x)` | `x == BigExponentFloat::NEGATIVE_INFINITY` |
| ✓ | `int arf_is_nan(const arf_t x)` | [`is_nan`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.is_nan) |
| ✓ | `int arf_is_inf(const arf_t x)` | [`is_infinite`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.is_infinite) |
| ✓ | `int arf_is_normal(const arf_t x)` | [`is_normal`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.is_normal) |
| ≈ | `int arf_is_special(const arf_t x)` | `!x.is_normal()` |
| ✓ | `int arf_is_finite(const arf_t x)` | [`is_finite`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.is_finite) |

**`arf_is_one`, `arf_is_pos_inf`, `arf_is_neg_inf`, `arf_is_special`.** These are comparisons in
Malachite. A normal value is a finite nonzero one, as in FLINT, so the special values are those that
are not normal.
[`classify`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.classify)
returns the class of a value as an enum, when several tests would otherwise be chained.

## [Assignment, rounding and conversions](https://flintlib.org/doc/arf.html#assignment-rounding-and-conversions) {#assignment-rounding-and-conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arf_set(arf_t res, const arf_t x)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| — | `void arf_set_mpz(arf_t res, const mpz_t x)` |  |
| ✓ | `void arf_set_fmpz(arf_t res, const fmpz_t x)` | [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html) |
| ✓ | `void arf_set_ui(arf_t res, ulong x)` | [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html) |
| ✓ | `void arf_set_si(arf_t res, slong x)` | [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html) |
| ≈ | `void arf_set_mpfr(arf_t res, const mpfr_t x)` | [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html)`<Float>` |
| ✓ | `void arf_set_d(arf_t res, double x)` | [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html) |
| — | `void arf_swap(arf_t x, arf_t y)` |  |
| ✓ | `void arf_init_set_ui(arf_t res, ulong x)` | [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html) |
| ✓ | `void arf_init_set_si(arf_t res, slong x)` | [`From`](https://doc.rust-lang.org/nightly/std/convert/trait.From.html) |
| ✓ | `int arf_set_round(arf_t res, const arf_t x, slong prec, arf_rnd_t rnd)` | [`round_to_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_prec_round) |
| ✓ | `int arf_set_round_si(arf_t res, slong x, slong prec, arf_rnd_t rnd)` | [`from_signed_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.from_signed_prec_round) |
| ✓ | `int arf_set_round_ui(arf_t res, ulong x, slong prec, arf_rnd_t rnd)` | [`from_unsigned_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.from_unsigned_prec_round) |
| — | `int arf_set_round_mpz(arf_t res, const mpz_t x, slong prec, arf_rnd_t rnd)` |  |
| ✓ | `int arf_set_round_fmpz(arf_t res, const fmpz_t x, slong prec, arf_rnd_t rnd)` | [`from_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.from_integer_prec_round) |
| ✓ | `void arf_set_si_2exp_si(arf_t res, slong m, slong e)` | [`const_from_signed_times_power_of_2`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.const_from_signed_times_power_of_2) |
| ✓ | `void arf_set_ui_2exp_si(arf_t res, ulong m, slong e)` | [`const_from_unsigned_times_power_of_2`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.const_from_unsigned_times_power_of_2) |
| ≈ | `void arf_set_fmpz_2exp(arf_t res, const fmpz_t m, const fmpz_t e)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ≈ | `int arf_set_round_fmpz_2exp(arf_t res, const fmpz_t x, const fmpz_t e, slong prec, arf_rnd_t rnd)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html), [`round_to_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_prec_round) |
| ≈ | `void arf_get_fmpz_2exp(fmpz_t m, fmpz_t e, const arf_t x)` | [`IntegerMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IntegerMantissaAndExponent.html) |
| ≈ | `void arf_frexp(arf_t m, fmpz_t e, const arf_t x)` | [`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html) |
| ✓ | `double arf_get_d(const arf_t x, arf_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html) |
| ≈ | `int arf_get_mpfr(mpfr_t res, const arf_t x, mpfr_rnd_t rnd)` | [`Float::from_big_exponent_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_big_exponent_float_prec_round) |
| ✓ | `int arf_get_fmpz(fmpz_t res, const arf_t x, arf_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html) |
| ✓ | `slong arf_get_si(const arf_t x, arf_rnd_t rnd)` | [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html) |
| ≈ | `int arf_get_fmpz_fixed_fmpz(fmpz_t res, const arf_t x, const fmpz_t e)` | [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html) |
| ≈ | `int arf_get_fmpz_fixed_si(fmpz_t res, const arf_t x, slong e)` | [`RoundingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.RoundingFrom.html) |
| ✓ | `void arf_floor(arf_t res, const arf_t x)` | [`round_to_integer_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_integer_round) |
| ✓ | `void arf_ceil(arf_t res, const arf_t x)` | [`round_to_integer_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_integer_round) |
| ✓ | `void arf_nint(arf_t res, const arf_t x)` | [`round_to_integer`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_integer) |
| ✓ | `void arf_get_fmpq(fmpq_t res, const arf_t x)` | [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) |

**`arf_set_mpz`, `arf_set_round_mpz`.** Malachite has no GMP types; an `mpz_t` corresponds to an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), for which
`From` and
[`from_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.from_integer_prec_round)
serve.

**`arf_set_mpfr`, `arf_get_mpfr`.** Malachite's MPFR-style type is
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html).
Conversion to a
[`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html)
is exact. Conversion to a
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) rounds to
the requested precision and also handles the bounded exponent of
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html): a value
outside its range overflows to an infinity or underflows to zero, under the rules of the MPFR page,
while `arf_get_mpfr` leaves that to MPFR's flags.

**`arf_swap`.** [`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html) swaps any
two values.

**`arf_set_si_2exp_si`, `arf_set_ui_2exp_si`.** The `const_from_*_times_power_of_2` functions are
`const`, so they also build named constants at compile time.

**`arf_set_fmpz_2exp`, `arf_set_round_fmpz_2exp`, `arf_get_fmpz_2exp`.**
`BigExponentFloat::from_integer_mantissa_and_exponent(m, e)` builds $$m \cdot 2^e$$ exactly from a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) mantissa
and an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
exponent, so a negative mantissa is applied with a negation; rounding is then
[`round_to_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_prec_round).
In the other direction, `integer_mantissa_and_exponent` returns the odd mantissa and its exponent,
as `arf_get_fmpz_2exp` does, but with the mantissa's magnitude, the sign being read separately.

**`arf_frexp`.** FLINT's mantissa lies in $$[1/2, 1)$$; Malachite's `sci_mantissa_and_exponent`
returns one in $$[1, 2)$$, with an exponent one less.

**`arf_get_si`, `arf_get_fmpz`, `arf_get_d`.** `i64::rounding_from(x, rm)`,
`Integer::rounding_from(x, rm)`, and `f64::rounding_from(x, rm)` return the rounded value and an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html). Out-of-range and special
inputs follow the documented rules of each conversion, where FLINT aborts or leaves the result
undefined.

**`arf_get_fmpz_fixed_fmpz`, `arf_get_fmpz_fixed_si`.** The integer $$\lfloor x / 2^e \rfloor$$ is
`Integer::rounding_from(x >> e, Floor)` for a word-sized `e`. For an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) exponent,
multiply exactly by the power of 2 built with `from_integer_mantissa_and_exponent` first. The
returned [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) replaces FLINT's
inexact flag.

**`arf_floor`, `arf_ceil`, `arf_nint`.** `x.round_to_integer_round(Floor)` and
`x.round_to_integer_round(Ceiling)` return the exact integer, and
[`round_to_integer`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_integer)
rounds to the nearest integer with ties to even, as `arf_nint` does. The `_prec_round` forms also
round the integer to a precision, in a single rounding.

**`arf_get_fmpq`.** `Rational::try_from(&x)` is exact, and fails for NaN and the infinities.

## [Comparisons and bounds](https://flintlib.org/doc/arf.html#comparisons-and-bounds) {#comparisons-and-bounds}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int arf_equal(const arf_t x, const arf_t y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int arf_equal_si(const arf_t x, slong y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int arf_equal_ui(const arf_t x, ulong y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ≈ | `int arf_equal_d(const arf_t x, double y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ≈ | `int arf_cmp(const arf_t x, const arf_t y)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ≈ | `int arf_cmp_si(const arf_t x, slong y)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ≈ | `int arf_cmp_ui(const arf_t x, ulong y)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ≈ | `int arf_cmp_d(const arf_t x, double y)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ≈ | `int arf_cmpabs(const arf_t x, const arf_t y)` | [`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) |
| ≈ | `int arf_cmpabs_ui(const arf_t x, ulong y)` | [`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) |
| ≈ | `int arf_cmpabs_d(const arf_t x, double y)` | [`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) |
| — | `int arf_cmpabs_mag(const arf_t x, const mag_t y)` |  |
| ≈ | `int arf_cmp_2exp_si(const arf_t x, slong e)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ≈ | `int arf_cmpabs_2exp_si(const arf_t x, slong e)` | [`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html) |
| ≈ | `int arf_sgn(const arf_t x)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |
| ≈ | `void arf_min(arf_t res, const arf_t a, const arf_t b)` | [`min`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.min) |
| ≈ | `void arf_max(arf_t res, const arf_t a, const arf_t b)` | [`max`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.max) |
| ≈ | `slong arf_bits(const arf_t x)` | [`get_prec`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.get_prec) |
| ✓ | `int arf_is_int(const arf_t x)` | [`IsInteger`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IsInteger.html) |
| ≈ | `int arf_is_int_2exp_si(const arf_t x, slong e)` | [`IsInteger`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.IsInteger.html) |
| ≈ | `void arf_abs_bound_lt_2exp_fmpz(fmpz_t res, const arf_t x)` | [`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.get_exponent) |
| ≈ | `void arf_abs_bound_le_2exp_fmpz(fmpz_t res, const arf_t x)` | [`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.get_exponent) |
| ≈ | `slong arf_abs_bound_lt_2exp_si(const arf_t x)` | [`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.get_exponent) |

**Comparisons with NaN.** Every ≈ among the comparisons is the NaN difference described under
[Conventions](#nan-in-comparisons): Malachite's comparisons return `None` and its equality `false`
where FLINT returns 0 and, for `arf_equal` and `arf_equal_d`, considers NaN equal to NaN.
Comparisons with
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html),
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html),
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), and the
Gaussian types are also implemented, both by value and by absolute value.

**`arf_cmpabs_mag`.** Malachite's counterpart of `mag_t` is internal; see [Magnitude
functions](#magnitude-functions).

**`arf_cmp_2exp_si`, `arf_cmpabs_2exp_si`.** Compare with `BigExponentFloat::power_of_2(e)`, which
is exact for any `i64` exponent; for the absolute comparison, use `partial_cmp_abs`.

**`arf_sgn`.**
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html)
returns an [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html), and panics on
NaN, where FLINT returns 0 (and documents the sign of NaN as undefined).

**`arf_min`, `arf_max`.** Malachite's `min` and `max` follow IEEE 754's minNum and maxNum: if one
argument is NaN, the other is returned. FLINT's choose with `arf_cmp`, which returns 0 when an
argument is NaN, so they return the first argument. Malachite's also have rounded forms, and forms
that compare with a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).

**`arf_bits`.**
[`get_prec`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.get_prec)
returns the number of bits from the highest set bit to the lowest, as `arf_bits` does, but returns
`None` for the special values, where FLINT returns 0.

**`arf_is_int_2exp_si`.** `(x >> e).is_integer()`; the shift is exact.

**`arf_abs_bound_lt_2exp_fmpz`, `arf_abs_bound_le_2exp_fmpz`, `arf_abs_bound_lt_2exp_si`.** For a
finite nonzero $$x$$, $$|x| < 2^e$$ for the exponent $$e$$ that
[`get_exponent`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.get_exponent)
returns, so that is the strict bound. The non-strict bound is one less when $$|x|$$ is a power of 2,
which
[`IsPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.IsPowerOf2.html)
tests. For a word-sized bound, convert the exponent with `i64::try_from`.

## [Magnitude functions](https://flintlib.org/doc/arf.html#magnitude-functions) {#magnitude-functions}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void arf_get_mag(mag_t res, const arf_t x)` |  |
| — | `void arf_get_mag_lower(mag_t res, const arf_t x)` |  |
| — | `void arf_set_mag(arf_t res, const mag_t x)` |  |
| — | `void mag_init_set_arf(mag_t res, const arf_t x)` |  |
| — | `void mag_fast_init_set_arf(mag_t res, const arf_t x)` |  |
| — | `void arf_mag_set_ulp(mag_t res, const arf_t x, slong prec)` |  |
| — | `void arf_mag_add_ulp(mag_t res, const mag_t x, const arf_t y, slong prec)` |  |
| — | `void arf_mag_fast_add_ulp(mag_t res, const mag_t x, const arf_t y, slong prec)` |  |

Malachite's counterpart of FLINT's `mag_t`, a fixed-precision upper bound used for the radii of
balls, is internal to `malachite-nz`, where it serves error bounds in exact algorithms. It will
become public together with Malachite's ball arithmetic, the counterpart of FLINT's `arb_t`, and
these functions will be mapped then.

## [Shallow assignment](https://flintlib.org/doc/arf.html#shallow-assignment) {#shallow-assignment}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void arf_init_set_shallow(arf_t z, const arf_t x)` |  |
| — | `void arf_init_set_mag_shallow(arf_t z, const mag_t x)` |  |
| — | `void arf_init_neg_shallow(arf_t z, const arf_t x)` |  |
| — | `void arf_init_neg_mag_shallow(arf_t z, const mag_t x)` |  |

Shallow copies share a significand without copying it. In Rust, a reference does that safely, and
Malachite's functions take their arguments by value or by reference for this reason; a negated view
is `-&x`, which copies, or a call to a function that takes the sign into account.

## [Random number generation](https://flintlib.org/doc/arf.html#random-number-generation) {#random-number-generation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void arf_randtest(arf_t res, flint_rand_t state, slong bits, slong mag_bits)` | [`random_finite_big_exponent_floats`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/random/index.html) |
| ≈ | `void arf_randtest_not_zero(arf_t res, flint_rand_t state, slong bits, slong mag_bits)` | [`random_nonzero_finite_big_exponent_floats`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/random/index.html) |
| ≈ | `void arf_randtest_special(arf_t res, flint_rand_t state, slong bits, slong mag_bits)` | [`random_big_exponent_floats`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/random/index.html) |
| ✗ | `void arf_urandom(arf_t res, flint_rand_t state, slong bits, arf_rnd_t rnd)` |  |

**`arf_randtest` and its variants.** Malachite's generators, in the
[`random`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/random/index.html)
module, are iterators, parameterized by a seed and by the mean exponent size and mean precision
rather than by bit bounds. The `striped_` versions produce long runs of equal bits, like FLINT's
test generators, and there are versions for each sign class and for a fixed precision.

**`arf_urandom`.** A generator of values uniformly distributed in $$[0, 1)$$, rounded to a
precision, is planned, along with exponentially and normally distributed generators.

## [Input and output](https://flintlib.org/doc/arf.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void arf_debug(const arf_t x)` | [`Debug`](https://doc.rust-lang.org/nightly/std/fmt/trait.Debug.html) |
| ≈ | `void arf_print(const arf_t x)` | [`LowerHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.LowerHex.html) |
| ≈ | `void arf_printd(const arf_t x, slong d)` | [`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html) |
| ≈ | `char * arf_get_str(const arf_t x, slong d)` | [`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html) |
| ≈ | `void arf_fprint(FILE * file, const arf_t x)` | [`LowerHex`](https://doc.rust-lang.org/nightly/std/fmt/trait.LowerHex.html) |
| ≈ | `void arf_fprintd(FILE * file, const arf_t y, slong d)` | [`ToSci`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToSci.html) |
| ≈ | `char * arf_dump_str(const arf_t x)` | [`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html) |
| ≈ | `int arf_load_str(arf_t x, const char * str)` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) |
| ≈ | `int arf_dump_file(FILE * stream, const arf_t x)` | [`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html) |
| ≈ | `int arf_load_file(arf_t x, FILE * stream)` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html) |

**`arf_debug`, `arf_print`, `arf_fprint`.** FLINT prints a value as its exact mantissa and exponent.
Malachite's exact output is in a power-of-2 base: `{x:#x}` and `{x:?}` write hexadecimal, such as
`0x1.8E+3`, with an exponent of any size, and
[`Binary`](https://doc.rust-lang.org/nightly/std/fmt/trait.Binary.html) and
[`Octal`](https://doc.rust-lang.org/nightly/std/fmt/trait.Octal.html) write the other bases. Writing
to a file is `write!` on any [`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html).

**`arf_printd`, `arf_get_str`, `arf_fprintd`.** Decimal output with $d$ significant digits is
`x.to_sci_with_options(options)` after `options.set_precision(d)`, with an exponent of any size;
FLINT rounds faithfully, and Malachite rounds correctly, in the rounding mode set by
`options.set_rounding_mode`. Printing it, or writing it to a file, is `print!` or `write!` on the
result.

**`arf_dump_str`, `arf_load_str`, `arf_dump_file`, `arf_load_file`.** FLINT's dump format is its own
exact text format. Malachite's exact text formats are the power-of-2 bases of `to_string_base` and
`from_string_base`, which round-trip, and serialization with [serde](https://serde.rs/), which
writes the hexadecimal form.

## [Addition and multiplication](https://flintlib.org/doc/arf.html#addition-and-multiplication) {#addition-and-multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void arf_abs(arf_t res, const arf_t x)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html) |
| ✓ | `void arf_neg(arf_t res, const arf_t x)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html) |
| ≈ | `int arf_neg_round(arf_t res, const arf_t x, slong prec, arf_rnd_t rnd)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`round_to_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_prec_round) |
| ✓ | `int arf_add(arf_t res, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`add_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_prec_round) |
| ≈ | `int arf_add_si(arf_t res, const arf_t x, slong y, slong prec, arf_rnd_t rnd)` | [`add_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_prec_round) |
| ≈ | `int arf_add_ui(arf_t res, const arf_t x, ulong y, slong prec, arf_rnd_t rnd)` | [`add_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_prec_round) |
| ≈ | `int arf_add_fmpz(arf_t res, const arf_t x, const fmpz_t y, slong prec, arf_rnd_t rnd)` | [`add_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_prec_round) |
| ≈ | `int arf_add_fmpz_2exp(arf_t res, const arf_t x, const fmpz_t y, const fmpz_t e, slong prec, arf_rnd_t rnd)` | [`add_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_prec_round) |
| ✓ | `int arf_sub(arf_t res, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`sub_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_prec_round) |
| ≈ | `int arf_sub_si(arf_t res, const arf_t x, slong y, slong prec, arf_rnd_t rnd)` | [`sub_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_prec_round) |
| ≈ | `int arf_sub_ui(arf_t res, const arf_t x, ulong y, slong prec, arf_rnd_t rnd)` | [`sub_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_prec_round) |
| ≈ | `int arf_sub_fmpz(arf_t res, const arf_t x, const fmpz_t y, slong prec, arf_rnd_t rnd)` | [`sub_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_prec_round) |
| ✓ | `void arf_mul_2exp_si(arf_t res, const arf_t x, slong e)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html) |
| ≈ | `void arf_mul_2exp_fmpz(arf_t res, const arf_t x, const fmpz_t e)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `int arf_mul(arf_t res, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_prec_round) |
| ≈ | `int arf_mul_ui(arf_t res, const arf_t x, ulong y, slong prec, arf_rnd_t rnd)` | [`mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_prec_round) |
| ≈ | `int arf_mul_si(arf_t res, const arf_t x, slong y, slong prec, arf_rnd_t rnd)` | [`mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_prec_round) |
| — | `int arf_mul_mpz(arf_t res, const arf_t x, const mpz_t y, slong prec, arf_rnd_t rnd)` |  |
| ≈ | `int arf_mul_fmpz(arf_t res, const arf_t x, const fmpz_t y, slong prec, arf_rnd_t rnd)` | [`mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_prec_round) |
| ✓ | `int arf_addmul(arf_t z, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_mul_prec_round) |
| ≈ | `int arf_addmul_ui(arf_t z, const arf_t x, ulong y, slong prec, arf_rnd_t rnd)` | [`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_mul_prec_round) |
| ≈ | `int arf_addmul_si(arf_t z, const arf_t x, slong y, slong prec, arf_rnd_t rnd)` | [`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_mul_prec_round) |
| — | `int arf_addmul_mpz(arf_t z, const arf_t x, const mpz_t y, slong prec, arf_rnd_t rnd)` |  |
| ≈ | `int arf_addmul_fmpz(arf_t z, const arf_t x, const fmpz_t y, slong prec, arf_rnd_t rnd)` | [`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_mul_prec_round) |
| ✓ | `int arf_submul(arf_t z, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_mul_prec_round) |
| ≈ | `int arf_submul_ui(arf_t z, const arf_t x, ulong y, slong prec, arf_rnd_t rnd)` | [`sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_mul_prec_round) |
| ≈ | `int arf_submul_si(arf_t z, const arf_t x, slong y, slong prec, arf_rnd_t rnd)` | [`sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_mul_prec_round) |
| — | `int arf_submul_mpz(arf_t z, const arf_t x, const mpz_t y, slong prec, arf_rnd_t rnd)` |  |
| ≈ | `int arf_submul_fmpz(arf_t z, const arf_t x, const fmpz_t y, slong prec, arf_rnd_t rnd)` | [`sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sub_mul_prec_round) |
| ✓ | `int arf_fma(arf_t res, const arf_t x, const arf_t y, const arf_t z, slong prec, arf_rnd_t rnd)` | [`add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.add_mul_prec_round) |
| ✓ | `int arf_sosq(arf_t res, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`mul_add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_add_mul_prec_round) |

**Exact forms.** With `ARF_PREC_EXACT`, `arf_add`, `arf_sub`, and `arf_mul` are the operators `+`,
`-`, and `*`, which are exact for
[`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html);
see [Conventions](#precision-and-exactness). An exact sum of operands whose exponents are far apart
has as many bits as the gap, as in FLINT.

**Word and `fmpz` operands.** Each `_si`, `_ui`, and `_fmpz` variant is the general function applied
to `BigExponentFloat::from(y)`, which is exact; see [Conventions](#mixed-operands). The `_mpz`
variants have no counterpart, since Malachite has no GMP types, and `arf_add_fmpz_2exp` is the sum
with $$y \cdot 2^e$$ built by `from_integer_mantissa_and_exponent`.

**`arf_neg_round`.** Negation is exact, so this is negation followed by
[`round_to_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.round_to_prec_round),
with the rounding mode applying to the negated value.

**`arf_mul_2exp_si`, `arf_mul_2exp_fmpz`.** Shifts by any primitive integer, signed or unsigned, are
exact: `x << e` and `x >> e`, and the `ShlAssign` and `ShrAssign` forms. For an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) exponent,
multiply exactly by the power of 2 built with `from_integer_mantissa_and_exponent`.

**`arf_addmul`, `arf_submul`, `arf_fma`.** `z.add_mul_prec_round(x, y, prec, rm)` is $$z + xy$$ with
one rounding, and `sub_mul_prec_round` is $$z - xy$$; `arf_fma` is the same operation with the roles
named differently. The
[`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html)
and
[`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html)
traits give the exact forms.

**`arf_sosq`.** `x.mul_add_mul_prec_round(x, y, y, prec, rm)` is $$x^2 + y^2$$ with one rounding.
Malachite also has
[`mul_sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_sub_mul_prec_round),
[`hypot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.hypot_prec_round),
and
[`AbsSquared`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsSquared.html).

## [Summation](https://flintlib.org/doc/arf.html#summation) {#summation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int arf_sum(arf_t res, arf_srcptr terms, slong len, slong prec, arf_rnd_t rnd)` | [`sum_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sum_prec_round) |

`BigExponentFloat::sum_prec_round(xs, prec, rm)` takes any iterator of values; the exact sum is
[`Sum`](https://doc.rust-lang.org/nightly/std/iter/trait.Sum.html).
[`product_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.product_prec_round)
is the product's counterpart, which FLINT does not have.

## [Dot products](https://flintlib.org/doc/arf.html#dot-products) {#dot-products}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int arf_dot(arf_t res, const arf_t initial, int subtract, arf_srcptr x, slong xstep, arf_srcptr y, slong ystep, slong len, slong prec, arf_rnd_t rnd)` | [`dot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.dot_prec_round) |
| ≈ | `void arf_approx_dot(arf_t res, const arf_t initial, int subtract, arf_srcptr x, slong xstep, arf_srcptr y, slong ystep, slong len, slong prec, arf_rnd_t rnd)` | [`dot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.dot_prec_round) |

**`arf_dot`.**
[`dot_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.dot_prec_round)
computes $$\sum x_i y_i$$ over two iterators with one rounding. FLINT's `initial` value and
`subtract` flag are expressed by adding an initial term to the iterators and by negating one of
them, and the strides by the iterators themselves.

**`arf_approx_dot`.** FLINT's approximate dot product drops the correct-rounding guarantee for
speed. Malachite has only the correctly rounded one, which serves the same purpose.

## [Division](https://flintlib.org/doc/arf.html#division) {#division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int arf_div(arf_t res, const arf_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round) |
| ≈ | `int arf_div_ui(arf_t res, const arf_t x, ulong y, slong prec, arf_rnd_t rnd)` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round) |
| ≈ | `int arf_ui_div(arf_t res, ulong x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round) |
| ≈ | `int arf_div_si(arf_t res, const arf_t x, slong y, slong prec, arf_rnd_t rnd)` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round) |
| ≈ | `int arf_si_div(arf_t res, slong x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round) |
| ≈ | `int arf_div_fmpz(arf_t res, const arf_t x, const fmpz_t y, slong prec, arf_rnd_t rnd)` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round) |
| ≈ | `int arf_fmpz_div(arf_t res, const fmpz_t x, const arf_t y, slong prec, arf_rnd_t rnd)` | [`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round) |
| ≈ | `int arf_fmpz_div_fmpz(arf_t res, const fmpz_t x, const fmpz_t y, slong prec, arf_rnd_t rnd)` | [`from_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.from_rational_prec_round) |

**Word and `fmpz` operands.** As for addition, these are
[`div_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_prec_round)
with the integer operand converted exactly. A reciprocal is
[`reciprocal_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.reciprocal_prec_round),
and quotients by and of a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) are
[`div_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.div_rational_prec_round)
and
[`rational_div_big_exponent_float_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.rational_div_big_exponent_float_prec_round).

**`arf_fmpz_div_fmpz`.** The correctly rounded quotient of two integers is the rounding of the
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) they
form: `BigExponentFloat::from_rational_prec_round(Rational::from_integers(x, y), prec, rm)`.

## [Square roots](https://flintlib.org/doc/arf.html#square-roots) {#square-roots}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int arf_sqrt(arf_t res, const arf_t x, slong prec, arf_rnd_t rnd)` | [`sqrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sqrt_prec_round) |
| ≈ | `int arf_sqrt_ui(arf_t res, ulong x, slong prec, arf_rnd_t rnd)` | [`sqrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sqrt_prec_round) |
| ≈ | `int arf_sqrt_fmpz(arf_t res, const fmpz_t x, slong prec, arf_rnd_t rnd)` | [`sqrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.sqrt_prec_round) |
| ✓ | `int arf_rsqrt(arf_t res, const arf_t x, slong prec, arf_rnd_t rnd)` | [`reciprocal_sqrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.reciprocal_sqrt_prec_round) |
| ≈ | `int arf_root(arf_t res, const arf_t x, ulong k, slong prec, arf_rnd_t rnd)` | [`root_u_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.root_u_prec_round) |

**`arf_sqrt_ui`, `arf_sqrt_fmpz`.** The square root of the integer converted exactly.

**`arf_root`.**
[`root_u_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.root_u_prec_round)
returns the real $$k$$-th root, which for a negative $$x$$ and an odd $$k$$ is negative, where FLINT
returns NaN for any negative $$x$$.
[`root_s_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.root_s_prec_round)
takes a signed $$k$$, and
[`cbrt_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.cbrt_prec_round)
is the cube root. Integer powers are
[`pow_u_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.pow_u_prec_round),
[`pow_s_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.pow_s_prec_round),
and
[`pow_integer_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.pow_integer_prec_round).

## [Complex arithmetic](https://flintlib.org/doc/arf.html#complex-arithmetic) {#complex-arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int arf_complex_mul(arf_t e, arf_t f, const arf_t a, const arf_t b, const arf_t c, const arf_t d, slong prec, arf_rnd_t rnd)` | [`mul_sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_sub_mul_prec_round), [`mul_add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_add_mul_prec_round) |
| ⚙ | `int arf_complex_mul_fallback(arf_t e, arf_t f, const arf_t a, const arf_t b, const arf_t c, const arf_t d, slong prec, arf_rnd_t rnd)` | [`mul_sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_sub_mul_prec_round), [`mul_add_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_add_mul_prec_round) |
| ≈ | `int arf_complex_sqr(arf_t e, arf_t f, const arf_t a, const arf_t b, slong prec, arf_rnd_t rnd)` | [`mul_sub_mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_sub_mul_prec_round), [`mul_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html#method.mul_prec_round) |

**`arf_complex_mul`, `arf_complex_sqr`.** The real part of $$(a + bi)(c + di)$$ is
`a.mul_sub_mul_prec_round(c, b, d, prec, rm)`, $$ac - bd$$ with one rounding, and the imaginary part
is `a.mul_add_mul_prec_round(d, b, c, prec, rm)`. Each part is correctly rounded, as in FLINT, and
the two calls return separate orderings, where FLINT packs the two inexact flags into the bits of
one return code. For a square, the real part is $$a^2 - b^2$$ and the imaginary part $$2ab$$, the
latter exact before rounding since doubling is a shift. `arf_complex_mul_fallback` is the algorithm
`arf_complex_mul` uses for large operands, which Malachite does not expose separately.

## [Low-level methods](https://flintlib.org/doc/arf.html#low-level-methods) {#low-level-methods}

All of the functions in this section begin with an underscore, and are omitted.
