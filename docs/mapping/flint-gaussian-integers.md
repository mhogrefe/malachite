---
layout: default
title: "Malachite for FLINT Users: Gaussian Integers"
permalink: /mapping/flint-gaussian-integers/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Gaussian Integers

This page maps the functions of FLINT's Gaussian integer type, `fmpzi_t`, onto their Malachite
counterpart:
[`GaussianInteger`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html),
from the `malachite-nz` crate. It follows the organization of the
[fmpzi.h chapter](https://flintlib.org/doc/fmpzi.html) of the FLINT manual, as of FLINT 3.6.0,
and is a companion to [Malachite for FLINT Users: Integers](/mapping/flint-integers/), whose
[conventions](/mapping/flint-integers/#conventions), including word types and aliasing, apply
here as well; the [mapping index](/mapping/) lists the whole family. Every function is mapped,
including three that the header declares but the manual chapter does not list. Where FLINT
exposes a function, Malachite often exposes a struct field, an operator, or a trait shared with
its real types.

## Conventions {#conventions}

### The `fmpzi` representation

A
[`GaussianInteger`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html)
is a pair of
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s in
public fields, `real` and `imaginary`. Every pair of parts is a valid value, so there is no
constructor: the struct literal is the constructor.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/fmpzi.html#types-macros-and-constants) {#types-macros-and-constants}

The types are covered under [Conventions](#conventions).

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpzi_realref(x)` | [`x.real`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#structfield.real) |
| ✓ | `fmpzi_imagref(x)` | [`x.imaginary`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#structfield.imaginary) |

**`fmpzi_realref`, `fmpzi_imagref`.** The fields are read and written directly: `x.real`,
`x.real = ...`, `x.real += ...`, and `&mut x.real` for any in-place
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
function. Unlike the rational case on
[the fmpq page](/mapping/flint-rationals/#types-macros-and-constants), writing a part directly
is safe, because there is no canonical form to disturb.

## [Basic manipulation](https://flintlib.org/doc/fmpzi.html#basic-manipulation) {#basic-manipulation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpzi_init (fmpzi_t x)` | [`GaussianInteger::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void fmpzi_clear (fmpzi_t x)` | |
| ✓ | `void fmpzi_swap (fmpzi_t x, fmpzi_t y)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| ✓ | `void fmpzi_zero (fmpzi_t x)` | [`GaussianInteger::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void fmpzi_one (fmpzi_t x)` | [`GaussianInteger::ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |
| ✓ | `void fmpzi_set (fmpzi_t res, const fmpzi_t x)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpzi_set_si_si (fmpzi_t res, slong a, slong b)` | `GaussianInteger { real: Integer::from(a), imaginary: Integer::from(b) }` |

**`fmpzi_init`, `fmpzi_clear`.** `let x = GaussianInteger::ZERO;`, or
[`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html). Dropping
replaces clearing; see [the GMP page](/mapping/gmp-integers/#initializing-integers).

**`fmpzi_swap`, `fmpzi_zero`, `fmpzi_one`, `fmpzi_set`.** `mem::swap(&mut x, &mut y)`,
`x = GaussianInteger::ZERO`, `x = GaussianInteger::ONE`, and `res = x.clone()` or
`res.clone_from(&x)`, the latter reusing the destination's storage as `fmpzi_set` does.

**`fmpzi_set_si_si`.** The struct literal, with each part converted from any primitive integer
or from a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html). A
purely real value can also be built with `GaussianInteger::from(a)`.

## [Input and output](https://flintlib.org/doc/fmpzi.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpzi_print (const fmpzi_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |

**`fmpzi_print`.** `print!("{x}")`. The formats differ: FLINT always prints both parts and
`*I`, as in `3+4*I`, `3+0*I`, `0-1*I`, while Malachite's
[`Display`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#impl-Display-for-GaussianInteger)
omits a zero part and a unit coefficient, as in `3+4i`, `3`, `-i`.

## [Random number generation](https://flintlib.org/doc/fmpzi.html#random-number-generation) {#random-number-generation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpzi_randtest (fmpzi_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`random_gaussian_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/random/fn.random_gaussian_integers.html), [`striped_random_gaussian_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/random/fn.striped_random_gaussian_integers.html) |

**`fmpzi_randtest`.** Two independent `fmpz_randtest` draws, one per part, so the notes on
[the fmpz page](/mapping/flint-integers/#random-generation) apply to each part. Malachite's
generators also draw the parts independently, in plain and striped flavors; they are infinite
iterators over a
[`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html) rather
than calls against a state, their size parameter is a mean bit length rather than a bound, and
they require the `random` feature.

## [Properties](https://flintlib.org/doc/fmpzi.html#properties) {#properties}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpzi_equal (const fmpzi_t x, const fmpzi_t y)` | [`==`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int fmpzi_is_zero (const fmpzi_t x)` | `x == 0u32` |
| ✓ | `int fmpzi_is_one (const fmpzi_t x)` | `x == 1u32` |

**`fmpzi_is_zero`, `fmpzi_is_one`.** Mixed-type
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) against a
primitive integer, which holds when the imaginary part is zero and the real part matches.

## [Units](https://flintlib.org/doc/fmpzi.html#units) {#units}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpzi_is_unit (const fmpzi_t x)` | [`is_unit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.IsUnit.html) |
| ✓ | `slong fmpzi_canonical_unit_i_pow (const fmpzi_t x)` | [`canonical_unit_i_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalUnitIPow.html) |
| ✓ | `void fmpzi_canonicalise_unit (fmpzi_t res, const fmpzi_t x)` | [`canonicalize_unit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalizeUnit.html) |

**Units.** Both libraries choose the same canonical associate, tie for tie: the one whose
argument lies in $$(-\pi/4, \pi/4]$$, that is, whose real part is positive and at least the
imaginary part in absolute value, with $$a + ai$$ chosen on the diagonal. Zero is its own
canonical form. `canonical_unit_i_pow` returns the $$k \in \{0, 1, 2, 3\}$$ with $$x i^k$$
canonical, as a `u64`;
[`canonicalize_unit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalizeUnit.html)
has an in-place
[`_assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalizeUnitAssign.html)
form.

## [Norms](https://flintlib.org/doc/fmpzi.html#norms) {#norms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpzi_bits (const fmpzi_t x)` | [`max_significant_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.max_significant_bits) |
| ✓ | `void fmpzi_norm (fmpz_t res, const fmpzi_t x)` | [`abs_squared`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsSquared.html) |

**`fmpzi_bits`.** The bit length of the larger part in absolute value. It is not what
[`significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html)
returns for a `GaussianInteger`, which is the sum of the two parts' counts.

**`fmpzi_norm`.** $$a^2 + b^2$$, returned as an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).

## [Arithmetic](https://flintlib.org/doc/fmpzi.html#arithmetic) {#arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpzi_conj (fmpzi_t res, const fmpzi_t x)` | [`conjugate`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Conjugate.html) |
| ✓ | `void fmpzi_neg (fmpzi_t res, const fmpzi_t x)` | [`-`](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html) |
| ✓ | `void fmpzi_add (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | [`+`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html) |
| ✓ | `void fmpzi_sub (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | [`-`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html) |
| ✓ | `void fmpzi_sqr (fmpzi_t res, const fmpzi_t x)` | [`square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html) |
| ✓ | `void fmpzi_mul (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | [`*`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ✓ | `void fmpzi_pow_ui (fmpzi_t res, const fmpzi_t x, ulong exp)` | [`pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`pow_assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ✓ | `void fmpzi_mul_i (fmpzi_t z, const fmpzi_t x)` | [`mul_i`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulI.html) |
| ✓ | `void fmpzi_div_i (fmpzi_t z, const fmpzi_t x)` | [`div_i`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivI.html) |
| ✓ | `void fmpzi_mul_i_pow_si (fmpzi_t res, const fmpzi_t z, slong k)` | [`mul_i_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulIPow.html), [`mul_i_pow_assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulIPowAssign.html) |

**Ownership.** Each operation comes in the usual ownership variants, `x + y`, `x + &y`,
`&x + y`, `&x + &y`, and `x += y`, with the by-value forms reusing their operand's storage;
the trait methods have `_assign` forms, such as
[`square_assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SquareAssign.html).
`pow` takes a `u64` exponent.

**`fmpzi_mul_i`, `fmpzi_div_i`, `fmpzi_mul_i_pow_si`.** Declared in the header but not listed
in the manual chapter.
[`MulIPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulIPow.html)
takes a `u64` exponent; since only $$k \bmod 4$$ matters, pass that for a negative FLINT
exponent $$k$$.

## [Division](https://flintlib.org/doc/fmpzi.html#division) {#division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpzi_divexact (fmpzi_t q, const fmpzi_t x, const fmpzi_t y)` | [`div_exact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✓ | `void fmpzi_divrem (fmpzi_t q, fmpzi_t r, const fmpzi_t x, const fmpzi_t y)` | [`div_rem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html), [`div_assign_rem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivAssignRem.html) |
| — | `void fmpzi_divrem_approx (fmpzi_t q, fmpzi_t r, const fmpzi_t x, const fmpzi_t y)` | |
| ✓ | `slong fmpzi_remove_one_plus_i (fmpzi_t res, const fmpzi_t x)` | [`remove_one_plus_i`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.remove_one_plus_i), [`remove_one_plus_i_assign`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.remove_one_plus_i_assign) |

**`fmpzi_divexact`.** As in FLINT, the division must be exact; otherwise the result is
unspecified and may be a panic.

**`fmpzi_divrem`.** Malachite rounds as FLINT does: each part of the exact quotient is rounded
to the nearest integer, with ties rounded up, so the remainder's norm satisfies
$$N(r) \leq N(y) / 2$$ and the result agrees with FLINT's at ties. The `/` and `%` operators
return the two halves of the same pair.

**`fmpzi_divrem_approx`.** Its quotient depends on a double-precision estimate and guarantees
only $$N(r) < N(y)$$, without fixing which remainder; Malachite has no public counterpart, and
`div_rem` satisfies the stronger bound.

**`fmpzi_remove_one_plus_i`.** Returns the reduced number and the exponent as a tuple, as
`RemovePower` does for `fmpz_remove`; the `_assign` form reduces in place and returns the
exponent. Zero maps to zero with exponent 0, as in FLINT.

## [GCD](https://flintlib.org/doc/fmpzi.html#gcd) {#gcd}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void fmpzi_gcd_euclidean (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | |
| ⚙ | `void fmpzi_gcd_euclidean_improved (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`GcdAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GcdAssign.html) |
| — | `void fmpzi_gcd_binary (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | |
| — | `void fmpzi_gcd_shortest (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | |
| ✓ | `void fmpzi_gcd (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`GcdAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GcdAssign.html) |

**The GCD family.** Every FLINT variant returns the GCD in [canonical unit form](#units), so
all five return the same value, and so does Malachite's
[`gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html).
The named variants differ only in algorithm, and Malachite does not expose them separately.

## [Primality testing](https://flintlib.org/doc/fmpzi.html#primality-testing) {#primality-testing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpzi_is_prime (const fmpzi_t n)` | |
| ✗ | `int fmpzi_is_probabprime (const fmpzi_t n)` | |

**`fmpzi_is_prime`, `fmpzi_is_probabprime`.** No counterpart, because Malachite has no bignum
primality test; see [the fmpz page](/mapping/flint-integers/#primality-testing). FLINT's test:
a purely real or purely imaginary value is prime when its nonzero part is, in absolute value, a
prime congruent to 3 modulo 4; any other value is prime when its norm is prime.
