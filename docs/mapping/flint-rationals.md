---
layout: default
title: "Malachite for FLINT Users: Rationals"
permalink: /mapping/flint-rationals/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Rationals

This page maps the functions of FLINT's rational number type, `fmpq_t`, onto their Malachite
counterpart:
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), from
the `malachite-q` crate. It follows the organization of the
[fmpq.h chapter](https://flintlib.org/doc/fmpq.html) of the FLINT manual, as of FLINT 3.6.0. It
is a companion to [Malachite for FLINT Users: Integers](/mapping/flint-integers/) on the FLINT
side and to [Malachite for GMP Users: Rationals](/mapping/gmp-rationals/) on the type's side,
since both map onto the same `Rational`; the [mapping index](/mapping/) lists the whole family.
The conventions of [the GMP rationals page](/mapping/gmp-rationals/) and
[the fmpz page](/mapping/flint-integers/#conventions), including word types and aliasing, apply
here as well. Functions whose names begin with an underscore are omitted.

## Conventions {#conventions}

### Canonical form, three ways

FLINT defines canonical form as `Rational` does: numerator and denominator coprime, a positive
denominator, and a denominator of one when the numerator is zero. GMP leaves canonicalization to
the caller after several assignment functions, as described on
[the GMP rationals page](/mapping/gmp-rationals/). Every `fmpq` function assumes canonical inputs
and produces canonical outputs, but editing a component through `fmpq_numref` or `fmpq_denref`
leaves the caller responsible until `fmpq_canonicalise` is called. In Malachite component access
is mediated, so a non-canonical
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
cannot be built or observed, and the mapping below carries no canonicalization caveats.

### The `fmpq` representation

An `fmpq` is a pair of `fmpz`s, each storing small values inline. A
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) is a
sign and two
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, each
inline below $$2^{64}$$. The visible difference is where the sign lives: FLINT's numerator is a
signed integer, while Malachite's numerator and denominator are magnitudes with the sign held
separately, as discussed in
[the accessor section](/mapping/gmp-rationals/#applying-integer-functions-to-rationals) of the
GMP rationals page.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/fmpq.html#types-macros-and-constants) {#types-macros-and-constants}

The types are covered under [Conventions](#conventions).

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `fmpz * fmpq_numref (const fmpq_t x)` | [`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.numerator_ref), [`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator) |
| ≈ | `fmpz * fmpq_denref (const fmpq_t x)` | [`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.denominator_ref), [`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_denominator) |

**`fmpq_numref`, `fmpq_denref`.** As for GMP's `mpq_numref` and `mpq_denref`
([discussion](/mapping/gmp-rationals/#applying-integer-functions-to-rationals)): reading is
`numerator_ref` and `denominator_ref`, with the sign asked for separately; writing goes through
the `mutate_*` closures, which reduce the value when the closure returns, so the
canonicalisation FLINT leaves to the caller after such an edit is automatic.

## [Memory management](https://flintlib.org/doc/fmpq.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_init (fmpq_t x)` | [`Rational::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void fmpq_clear (fmpq_t x)` | |

**`fmpq_init`, `fmpq_clear`.** `let x = Rational::ZERO;`, the canonical zero, 0/1, that
`fmpq_init` produces; [`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html)
also returns it. Dropping replaces clearing, as
[on the GMP page](/mapping/gmp-integers/#initializing-integers).

## [Canonicalisation](https://flintlib.org/doc/fmpq.html#canonicalisation) {#canonicalisation}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void fmpq_canonicalise (fmpq_t res)` | |
| — | `int fmpq_is_canonical (const fmpq_t x)` | |

Neither is needed, since a non-canonical
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
[cannot exist](#conventions): the
[`mutate_*` closures](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator)
reduce on the way out, and panic rather than leave a zero denominator. Values arriving from
outside are checked: deserialization
[validates canonical form](/mapping/gmp-rationals/#input-and-output-functions).

## [Basic assignment](https://flintlib.org/doc/fmpq.html#basic-assignment) {#basic-assignment}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_set (fmpq_t dest, const fmpq_t src)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| — | `void fmpq_swap (fmpq_t op1, fmpq_t op2)` | |
| ✓ | `void fmpq_neg (fmpq_t dest, const fmpq_t src)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `void fmpq_abs (fmpq_t dest, const fmpq_t src)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ✓ | `void fmpq_zero (fmpq_t res)` | [`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void fmpq_one (fmpq_t res)` | [`ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |

`dest = src.clone()`, or `dest.clone_from(&src)` to reuse `dest`'s allocations;
[`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html) for the exchange;
`-src` and `src.abs()`, with their `Assign` forms in place; and `res = Rational::ZERO` or
`Rational::ONE`.

## [Comparison](https://flintlib.org/doc/fmpq.html#comparison) {#comparison}

The discussions on [the GMP rationals page](/mapping/gmp-rationals/#comparison-functions), of
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) replacing the
sign-carrying `int` and of the
[`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html)/[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html)
split, carry over. FLINT's `_fmpz`, `_si`, and `_ui` forms compare against plain integers,
which the mixed
[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) and
[`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) implementations
take directly.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_is_zero (const fmpq_t res)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_is_one (const fmpq_t res)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_is_pm1 (const fmpq_t res)` | [`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html) |
| ✓ | `int fmpq_equal (const fmpq_t x, const fmpq_t y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_equal_fmpz (const fmpq_t x, const fmpz_t y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_equal_si (fmpq_t x, slong y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_equal_ui (fmpq_t x, ulong y)` | [`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_sgn (const fmpq_t x)` | [`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html) |
| ✓ | `int fmpq_cmp (const fmpq_t x, const fmpq_t y)` | [`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) |
| ✓ | `int fmpq_cmp_fmpz (const fmpq_t x, const fmpz_t y)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int fmpq_cmp_si (const fmpq_t x, slong y)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `int fmpq_cmp_ui (const fmpq_t x, ulong y)` | [`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) |
| ✓ | `void fmpq_height (fmpz_t height, const fmpq_t x)` | [`Rational::to_height`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_height), [`Rational::into_height`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.into_height) |
| ✓ | `flint_bitcnt_t fmpq_height_bits (const fmpq_t x)` | [`Rational::height_significant_bits`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.height_significant_bits) |

**The predicates and comparisons.** `x == 0`, `x == 1`, and `x.eq_abs(&1)` for the `pm1` test;
`==` against another
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), or
any primitive; `x.sign()`; and `<` and friends for the orderings.

**`fmpq_height`, `fmpq_height_bits`.** The height is the larger of `|p|` and `q`. `to_height`
takes a reference and `into_height` consumes the value. `height_significant_bits` is
`fmpq_height_bits`, computed without materializing the height. Do not confuse it with
[`significant_bits`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-SignificantBits-for-%26Rational),
which is the *sum* of the components' bit counts.

## [Conversion](https://flintlib.org/doc/fmpq.html#conversion) {#conversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_set_fmpz_frac (fmpq_t res, const fmpz_t p, const fmpz_t q)` | [`from_integers`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_integers) |
| ≈ | `void fmpq_get_mpz_frac (mpz_t a, mpz_t b, fmpq_t c)` | [`to_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_numerator_and_denominator) |
| ✓ | `void fmpq_set_si (fmpq_t res, slong p, ulong q)` | [`from_signeds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_signeds), [`from_sign_and_unsigneds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_sign_and_unsigneds) |
| ✓ | `void fmpq_set_ui (fmpq_t res, ulong p, ulong q)` | [`from_unsigneds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_unsigneds) |
| — | `void fmpq_set_mpq (fmpq_t dest, const mpq_t src)` | |
| ✓ | `int fmpq_set_str (fmpq_t dest, const char * s, int base)` | [`FromStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.FromStringBase.html), [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ≈ | `double fmpq_get_d (const fmpq_t f)` | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html), [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html) |
| — | `void fmpq_get_mpq (mpq_t dest, const fmpq_t src)` | |
| ✓ | `int fmpq_get_mpfr (mpfr_t dest, const fmpq_t src, mpfr_rnd_t rnd)` | [`from_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_rational_prec_round) |
| ✓ | `char * fmpq_get_str (char * str, int b, const fmpq_t x)` | [`ToStringBase`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.ToStringBase.html), [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| — | `void flint_mpq_init_set_readonly (mpq_t z, const fmpq_t f)` | |
| — | `void flint_mpq_clear_readonly (mpq_t z)` | |
| — | `void fmpq_init_set_readonly (fmpq_t f, const mpq_t z)` | |
| — | `void fmpq_clear_readonly (fmpq_t f)` | |

**The fraction constructors.** `fmpq_set_fmpz_frac` is `Rational::from_integers(p, q)`, and the
word-sized forms are `from_unsigneds` and `from_signeds`; all reduce on the way in, and a zero
denominator panics. `fmpq_set_si` takes a signed `p` over an unsigned `q`, and a `q` above
`i64::MAX` does not fit `from_signeds`; `from_sign_and_unsigneds(p >= 0, p.unsigned_abs(), q)`
covers the full range.

**`fmpq_get_mpz_frac`.** `to_numerator_and_denominator`, or `into_numerator_and_denominator` to
avoid the copy; the ≈ is the sign placement, FLINT's `a` being signed where Malachite returns
magnitudes.

**`fmpq_get_d`.** Truncation toward zero: `f64::rounding_from(&f, Down)`. Out of `double` range,
where FLINT's result is "system dependent", the
[defined behavior](/mapping/gmp-rationals/#conversion-functions) described for `mpq_get_d`
applies.

**`fmpq_get_mpfr`.** `Float::from_rational_prec_round(src, prec, rnd)`; the returned
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) is the sign of the
rounding that `fmpq_get_mpfr` returns.

**`fmpq_set_str`, `fmpq_get_str`.** Base 10 is
[`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) and
[`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html); other bases, 2 through
62, are `from_string_base` and `to_string_base`, as described on
[the GMP rationals page](/mapping/gmp-rationals/#conversion-functions). `fmpq_set_str` stores the
parsed numerator and denominator as given, leaving `"2/4"` unreduced and accepting a zero
denominator, where `from_string_base` reduces and returns `None` for a zero denominator. A bad
string is a `-1` return there and `None` here, and `fmpq_get_str`'s allocated result becomes a
[`String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html).

**The GMP boundary.** `fmpq_set_mpq`, `fmpq_get_mpq`, and the four `readonly` functions are the
`mpq_t` edition of the boundary
[described on the fmpz page](/mapping/flint-integers/#conversion): Malachite is not built on GMP,
so nothing corresponds, and interop routes through numerators and denominators, strings, or
serde.

## [Input and output](https://flintlib.org/doc/fmpq.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_fprint (FILE * file, const fmpq_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✓ | `int fmpq_print (const fmpq_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |

**`fmpq_print`, `fmpq_fprint`.** `print!("{x}")`, or `write!(f, "{x}")` for any
[`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html), as
[on the fmpz page](/mapping/flint-integers/#input-and-output); the success-or-failure `int`
becomes a `Result`. The formats agree exactly, including a bare integer when the denominator is
one (which FLINT's manual does not mention).

## [Random number generation](https://flintlib.org/doc/fmpq.html#random-number-generation) {#random-number-generation}

The `flint_rand_t` state maps as it did
[on the fmpz page](/mapping/flint-integers/#random-generation): Malachite's generators take a
[`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html) and return
infinite iterators. They live in
[`rational::random`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/index.html),
behind the `random` feature.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpq_randtest (fmpq_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`random_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.random_rationals.html), [`striped_random_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.striped_random_rationals.html) |
| ≈ | `void fmpq_randtest_not_zero (fmpq_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`random_nonzero_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.random_nonzero_rationals.html) |
| ✓ | `void fmpq_randbits (fmpq_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`get_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_bits.html), [`from_sign_and_naturals`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_sign_and_naturals) |

**`fmpq_randtest`, `fmpq_randtest_not_zero`.** FLINT caps the components at `bits` bits and
mixes in special values; Malachite's generators draw component sizes from a geometric
distribution around a mean, with no hard cap, and the corner-case values come from the striped
generators, described [on the GMP integers page](/mapping/gmp-integers/#random-number-functions).
`random_nonzero_rationals` is the `not_zero` variant.

**`fmpq_randbits`.** Draw the numerator and denominator with `get_random_natural_with_bits`,
choose a sign, and assemble with `from_sign_and_naturals`. As in FLINT, reduction can leave the
components "slightly smaller than `bits` bits".

## [Arithmetic](https://flintlib.org/doc/fmpq.html#arithmetic) {#arithmetic}

As [for GMP's rationals](/mapping/gmp-rationals/#arithmetic-functions), results are returned,
the `*Assign` traits cover the in-place case, and every operator has borrowing forms.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_add (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.AddAssign.html) |
| ✓ | `void fmpq_sub (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.SubAssign.html) |
| ✓ | `void fmpq_mul (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`MulAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.MulAssign.html) |
| ✓ | `void fmpq_div (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html), [`DivAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.DivAssign.html) |
| ✓ | `void fmpq_add_si (fmpq_t res, const fmpq_t op1, slong c)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `void fmpq_sub_si (fmpq_t res, const fmpq_t op1, slong c)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `void fmpq_add_ui (fmpq_t res, const fmpq_t op1, ulong c)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `void fmpq_sub_ui (fmpq_t res, const fmpq_t op1, ulong c)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `void fmpq_add_fmpz (fmpq_t res, const fmpq_t op1, const fmpz_t c)` | [`Add`](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html) |
| ✓ | `void fmpq_sub_fmpz (fmpq_t res, const fmpq_t op1, const fmpz_t c)` | [`Sub`](https://doc.rust-lang.org/nightly/std/ops/trait.Sub.html) |
| ✓ | `void fmpq_mul_si (fmpq_t res, const fmpq_t op1, slong c)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `void fmpq_mul_ui (fmpq_t res, const fmpq_t op1, ulong c)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `void fmpq_addmul (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | [`AddMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMul.html), [`AddMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AddMulAssign.html) |
| ✓ | `void fmpq_submul (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | [`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html), [`SubMulAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMulAssign.html) |
| ✓ | `void fmpq_inv (fmpq_t dest, const fmpq_t src)` | [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html), [`ReciprocalAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalAssign.html) |
| ✓ | `void fmpq_pow_si (fmpq_t res, const fmpq_t op, slong e)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ✓ | `int fmpq_pow_fmpz (fmpq_t a, const fmpq_t b, const fmpz_t e)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) |
| ✓ | `void fmpq_mul_fmpz (fmpq_t res, const fmpq_t op, const fmpz_t x)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `void fmpq_div_fmpz (fmpq_t res, const fmpq_t op, const fmpz_t x)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `void fmpq_mul_2exp (fmpq_t res, const fmpq_t x, flint_bitcnt_t exp)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `void fmpq_div_2exp (fmpq_t res, const fmpq_t x, flint_bitcnt_t exp)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| ✓ | `void fmpq_gcd (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`GcdAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GcdAssign.html) |
| ≈ | `void fmpq_gcd_cofactors (fmpq_t g, fmpz_t abar, fmpz_t bbar, const fmpq_t a, const fmpq_t b)` | [`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html) |

**The operators and mixed forms.** `op1 + op2` through `op1 / op2`, with division by zero a
panic where FLINT raises an error, and FLINT's blanket aliasing permission answered by the
assign and by-value forms as
[on the fmpz page](/mapping/flint-integers/#conventions). The ten mixed rows are one habit:
`op1 + Rational::from(c)`, for `c` of any primitive or
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) size,
with a by-value `Integer` donating its storage to the numerator.

**`fmpq_addmul`, `fmpq_submul`.** `res += op1 * op2`, or `res.add_mul_assign(op1, op2)`;
neither library fuses the two steps.

**`fmpq_inv`, `fmpq_pow_si`, `fmpq_pow_fmpz`.** `src.reciprocal()`, and `op.pow(e)` with a
signed exponent; both libraries take $$0^0 = 1$$. `fmpq_pow_fmpz` takes the exponent as a full
integer and reports failure instead of building an impossibly large result; as for
[`fmpz_pow_fmpz`](/mapping/flint-integers/#basic-arithmetic), convert with `i64::try_from(&e)`,
let the conversion's `Err` play the failure return, and handle the bases `0` and `±1` before
converting.

**`fmpq_mul_2exp`, `fmpq_div_2exp`.** `x << exp` and `x >> exp`, both exact.

**`fmpq_gcd`, `fmpq_gcd_cofactors`.** `Gcd` uses FLINT's definition, the canonical form of
$$\gcd(ps, qr)/(qs)$$ for $$p/q$$ and $$r/s$$, which for canonical values is
$$\gcd(p, r)/\operatorname{lcm}(q, s)$$. The cofactor row is ≈ because the cofactors differ:
FLINT returns the integer quotients $$a/g$$ and $$b/g$$, while `ExtendedGcd` returns integer
Bézout cofactors $$u$$ and $$v$$ with $$ua + vb = g$$. FLINT's quotients are the exact divisions
`a / &g` and `b / &g`.

## [Modular reduction and rational reconstruction](https://flintlib.org/doc/fmpq.html#modular-reduction-and-rational-reconstruction) {#modular-reduction-and-rational-reconstruction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_mod_fmpz (fmpz_t res, const fmpq_t x, const fmpz_t mod)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html), [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html) |
| ✓ | `int fmpq_reconstruct_fmpz_2 (fmpq_t res, const fmpz_t a, const fmpz_t m, const fmpz_t N, const fmpz_t D)` | [`Rational::reconstruct_with_bounds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.reconstruct_with_bounds) |
| ✓ | `int fmpq_reconstruct_fmpz (fmpq_t res, const fmpz_t a, const fmpz_t m)` | [`Rational::reconstruct`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.reconstruct) |

**`fmpq_mod_fmpz`.** The residue `a` with $$n \equiv a d \pmod m$$: reduce the denominator,
invert it, and multiply by the reduced numerator, as on
[the fmpz_mod page](/mapping/flint-integers-mod-n/#arithmetic). FLINT's failure return is
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)'s
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html): FLINT fails exactly when
the denominator has no inverse.

**`fmpq_reconstruct_fmpz_2`, `fmpq_reconstruct_fmpz`.** Given a residue `a` modulo `m` and
positive bounds `N`, `D` with $$2ND < m$$, reconstruction finds the unique fraction `n/d` with
$$|n| \le N$$, $$0 < d \le D$$, $$\gcd(n, d) = 1$$, and $$n \equiv ad \pmod m$$, if one exists;
the shorter form uses $$N = D = \lfloor\sqrt{(m-1)/2}\rfloor$$.
[`Rational::reconstruct_with_bounds_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.reconstruct_with_bounds_ref)
and
[`Rational::reconstruct_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.reconstruct_ref)
take their arguments by reference. FLINT's success flag and out-parameter become an
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html): `None` is FLINT's 0.
The residue is a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), so
$$a \ge 0$$ is carried by the type; the other preconditions, $$a < m$$, positive bounds, and $$m > 2$$ for the balanced form, are
panics. Within $$2ND < m$$ the two libraries agree value for value. Outside it the answer is not
unique; there the mapped functions return what FLINT's reference implementation,
`_fmpq_reconstruct_fmpz_2_naive`, returns, which `fmpq_reconstruct_fmpz_2` may not.

## [Rational enumeration](https://flintlib.org/doc/fmpq.html#rational-enumeration) {#rational-enumeration}

FLINT's `next` functions become infinite iterators in
[`rational::exhaustive`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/index.html),
each step an iterator's `next`.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_next_minimal (fmpq_t res, const fmpq_t x)` | [`exhaustive_non_negative_rationals_by_height`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/fn.exhaustive_non_negative_rationals_by_height.html) |
| ✓ | `void fmpq_next_signed_minimal (fmpq_t res, const fmpq_t x)` | [`exhaustive_rationals_by_height`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/fn.exhaustive_rationals_by_height.html) |
| ✓ | `void fmpq_next_calkin_wilf (fmpq_t res, const fmpq_t x)` | [`exhaustive_non_negative_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/fn.exhaustive_non_negative_rationals.html) |
| ✓ | `void fmpq_next_signed_calkin_wilf (fmpq_t res, const fmpq_t x)` | [`exhaustive_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/fn.exhaustive_rationals.html) |
| ✓ | `void fmpq_farey_neighbors (fmpq_t l, fmpq_t r, const fmpq_t x, const fmpz_t Q)` | [`Rational::farey_neighbors`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.farey_neighbors) |
| ≈ | `void fmpq_simplest_between (fmpq_t x, const fmpq_t l, const fmpq_t r)` | [`SimplestRationalInInterval`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/traits/trait.SimplestRationalInInterval.html) |

**`fmpq_next_calkin_wilf`, `fmpq_next_signed_calkin_wilf`, `fmpq_next_minimal`,
`fmpq_next_signed_minimal`.** Each iterator yields FLINT's sequence term for term, from the
leading 0 on.

**`fmpq_farey_neighbors`.**
[`Rational::farey_neighbors`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.farey_neighbors)
returns both neighbors as a pair; FLINT's `Q` is `max_denominator`. FLINT's throw on an input
whose denominator exceeds `Q` becomes a panic. Both libraries extend the Farey sequence to all
of $$\mathbb{Q}$$, so the input may be negative or exceed 1.

**`fmpq_simplest_between`.** `Rational::simplest_rational_in_closed_interval(&l, &r)`. The ≈ is
the tie-break when the minimal denominator is achieved more than once: FLINT takes the least
signed numerator, returning $$-1$$ for $$[-1, 1]$$ and $$-7$$ for $$[-7, -3]$$, while Malachite
prefers the numerator closest to zero and then the positive sign, returning 0 and $$-3$$. When
only one fraction in the interval has the minimal denominator, the two agree.

## [Continued fractions](https://flintlib.org/doc/fmpq.html#continued-fractions) {#continued-fractions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpq_get_cfrac (fmpz * c, fmpq_t rem, const fmpq_t x, slong n)` | [`ContinuedFraction`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/traits/trait.ContinuedFraction.html) |
| ⚙ | `slong fmpq_get_cfrac_naive (fmpz * c, fmpq_t rem, const fmpq_t x, slong n)` | [`ContinuedFraction`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/traits/trait.ContinuedFraction.html) |
| ✓ | `void fmpq_set_cfrac (fmpq_t x, const fmpz * c, slong n)` | [`from_continued_fraction`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_continued_fraction) |
| — | `slong fmpq_cfrac_bound (const fmpq_t x)` | |

**`fmpq_get_cfrac`, `fmpq_get_cfrac_naive`.** `x.continued_fraction()` returns the floor as an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) and
the remaining terms as an iterator of
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s,
where FLINT writes all coefficients into one vector with the possibly-negative $$c_0$$ first.
FLINT's incremental extraction, taking `n` terms and feeding the remainder back in, is the
suspended iterator. Both libraries generate the shorter of the two expansions every rational
has.

**`fmpq_set_cfrac`.** `Rational::from_continued_fraction(floor, terms)`, with a `_ref` variant
that borrows the terms. FLINT asks that the coefficients after $$c_0$$ be nonnegative, while
Malachite requires them positive.

**`fmpq_cfrac_bound`.** A preallocation bound for the coefficient vector; an iterator allocates
nothing ahead of time, so there is nothing to bound.

## [Special functions](https://flintlib.org/doc/fmpq.html#special-functions) {#special-functions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_harmonic_ui (fmpq_t x, ulong n)` | `Rational::harmonic_number(n)` |

**`fmpq_harmonic_ui`.** Both libraries reject $$n \geq 2^{63}$$.
[`arith_harmonic_number`](/mapping/flint-arithmetic-functions/#harmonic-numbers) maps to the same
function.

## [Dedekind sums](https://flintlib.org/doc/fmpq.html#dedekind-sums) {#dedekind-sums}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_dedekind_sum (fmpq_t s, const fmpz_t h, const fmpz_t k)` | `Rational::dedekind_sum(&h, &k)` |
| — | `void fmpq_dedekind_sum_naive (fmpq_t s, const fmpz_t h, const fmpz_t k)` | |

**`fmpq_dedekind_sum`.** The results agree exactly; both libraries return 0 for $$k \leq 2$$,
negative $$k$$ included.

**`fmpq_dedekind_sum_naive`.** A slow reference implementation of the defining sum; Malachite
keeps such implementations out of its public API.