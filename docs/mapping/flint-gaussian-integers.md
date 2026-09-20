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
here as well; the [mapping index](/mapping/) lists the whole family. FLINT describes this
module as "a minimal interface" to the ring $$\mathbb{Z}[i]$$, and it is short enough that
every function is mapped below, including the three that the header declares but the manual
chapter does not list.

Malachite's Gaussian integers were designed on their own terms rather than ported from this
module, so the two libraries agree on the operations but not always on their shape: where
FLINT exposes a function, Malachite usually exposes a trait shared with its real types, and
several FLINT functions dissolve into struct fields or into the operators of the component
type. The arithmetic kernels are the exception: Malachite's multiplication and squaring are
ports of FLINT's, as noted in [their section](#arithmetic).

## Conventions {#conventions}

### The `fmpzi` representation

An `fmpzi` is a pair of `fmpz`s, the real and imaginary parts, each the tagged word described
under [the fmpz representation](/mapping/flint-integers/#conventions), so a Gaussian integer
with word-sized parts occupies two words and no allocated memory. A
[`GaussianInteger`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html)
is a pair of
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s in
public fields, `real` and `imaginary`, each inline below $$2^{64}$$ in magnitude. There is no
constructor: the struct literal is the constructor, because every pair of parts is a valid
value and there is no invariant for a constructor to guard. `fmpzi_t` is an array of length
one, the same pass-by-reference device as every `_t` type before it.

### Categories

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/fmpzi.html#types-macros-and-constants) {#types-macros-and-constants}

The types themselves are covered under [Conventions](#conventions); the section's two macros
are the component accessors.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpzi_realref(x)` | [`x.real`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#structfield.real) |
| ✓ | `fmpzi_imagref(x)` | [`x.imaginary`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#structfield.imaginary) |

**`fmpzi_realref`, `fmpzi_imagref`.** Pointers into the pair, usable for reading and writing.
The Malachite fields are the same thing without the macro: `x.real` reads, `x.real = ...` and
`x.real += ...` write, and `&mut x.real` hands the part to any
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
function that mutates in place. Unlike the rational case on
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

**`fmpzi_init`, `fmpzi_clear`.** `let x = GaussianInteger::ZERO;`, a pair of inline zeros with
nothing allocated, as `fmpzi_init` produces. Dropping replaces clearing; the
[fuller discussion](/mapping/gmp-integers/#initializing-integers) of why Rust has neither step
is on the GMP page.
[`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) returns zero
here too.

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

**`fmpzi_print`.** `print!("{x}")`. The formats differ. FLINT prints the real part, then the
imaginary part with an explicit sign, then `*I`, unconditionally: `3+4*I`, `3+0*I`, `0-1*I`.
Malachite's
[`Display`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#impl-Display-for-GaussianInteger)
writes the value the way it is written by hand, `3+4i`, `3`, `-i`, omitting a zero part and a
unit coefficient.

## [Random number generation](https://flintlib.org/doc/fmpzi.html#random-number-generation) {#random-number-generation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpzi_randtest (fmpzi_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`random_gaussian_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/random/fn.random_gaussian_integers.html), [`striped_random_gaussian_integers`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/random/fn.striped_random_gaussian_integers.html) |

**`fmpzi_randtest`.** Two independent `fmpz_randtest` draws at the same bit bound, one per
part, so everything said about `fmpz_randtest` on
[the fmpz page](/mapping/flint-integers/#random-generation) applies to each part: a test
generator biased toward small values and special bit patterns, not a uniform sampler.
Malachite's generators draw the two parts independently as well, from the corresponding
`Integer` streams, and come in the same plain and striped flavors, with purely real and purely
imaginary variants alongside; they are infinite iterators over a
[`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html) rather
than calls against a state, and their size parameter is a mean bit length rather than a bound.
Random generation lives behind the `random` feature.

## [Properties](https://flintlib.org/doc/fmpzi.html#properties) {#properties}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpzi_equal (const fmpzi_t x, const fmpzi_t y)` | [`==`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int fmpzi_is_zero (const fmpzi_t x)` | `x == 0u32` |
| ✓ | `int fmpzi_is_one (const fmpzi_t x)` | `x == 1u32` |

**`fmpzi_equal`, `fmpzi_is_zero`, `fmpzi_is_one`.** Equality is derived componentwise, and the
comparisons against 0 and 1 are the mixed-type
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) implementations
against primitive integers, which hold exactly when the imaginary part is zero and the real
part matches.

## [Units](https://flintlib.org/doc/fmpzi.html#units) {#units}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpzi_is_unit (const fmpzi_t x)` | [`is_unit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.IsUnit.html) |
| ✓ | `slong fmpzi_canonical_unit_i_pow (const fmpzi_t x)` | [`canonical_unit_i_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalUnitIPow.html) |
| ✓ | `void fmpzi_canonicalise_unit (fmpzi_t res, const fmpzi_t x)` | [`canonicalize_unit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalizeUnit.html) |

**Units.** $$\mathbb{Z}[i]$$ has four units, $$\pm 1$$ and $$\pm i$$, so every nonzero value
has four associates and a canonical representative must be chosen among them wherever an
answer is defined only up to a unit, as a GCD is. FLINT's choice, read from the source of
`fmpzi_canonical_unit_i_pow`, is the associate whose real part is positive and at least as
large as the imaginary part in absolute value, with the tie on the positive diagonal resolved
in favor of $$a + ai$$: the associate whose argument lies in $$(-\pi/4, \pi/4]$$. The function
returns the exponent $$k \in \{0, 1, 2, 3\}$$ with $$x i^k$$ canonical, and
`fmpzi_canonicalise_unit` applies it; zero is its own canonical form. `fmpzi_is_unit` tests
whether one part is zero and the other is $$\pm 1$$. Malachite's three traits make the same
choices, tie for tie, so canonical forms agree between the libraries;
[`canonicalize_unit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalizeUnit.html)
also has an in-place
[`_assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CanonicalizeUnitAssign.html)
form, and the exponent comes back as a `u64`.

## [Norms](https://flintlib.org/doc/fmpzi.html#norms) {#norms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpzi_bits (const fmpzi_t x)` | [`max_significant_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.max_significant_bits) |
| ✓ | `void fmpzi_norm (fmpz_t res, const fmpzi_t x)` | [`abs_squared`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsSquared.html) |

**`fmpzi_bits`.** The bit length of the larger part in absolute value, which is how FLINT
sizes its own operands for algorithm selection; the source computes it as the bit count of
the bitwise or of the two magnitudes, which is the same number, and
[`max_significant_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.max_significant_bits)
computes it as the maximum. It is not what
[`significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.SignificantBits.html)
returns for a `GaussianInteger`: that trait, which measures a value's total size across all of
Malachite's types, sums the two parts' counts.

**`fmpzi_norm`.** The norm $$N(a + bi) = a^2 + b^2$$, the squared absolute value, as an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
Malachite's name for it is
[`abs_squared`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsSquared.html),
a trait shared with every numeric type, real types included, so that the same call means
$$|x|^2$$ everywhere.

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

**`fmpzi_conj`, `fmpzi_neg`, `fmpzi_add`, `fmpzi_sub`.** The componentwise operations. Each
comes in the usual ownership variants, `x + y`, `x + &y`, `&x + y`, `&x + &y`, and `x += y`,
with the by-value forms reusing their operand's storage;
[`conjugate`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Conjugate.html)
and its `_assign` form are traits shared with the real types, for which they are the identity.

**`fmpzi_mul`, `fmpzi_sqr`.** Malachite's multiplication and squaring are ports of these two
functions. Multiplication takes a double-word path when all four parts fit in a signed word, a
three-multiplication Karatsuba scheme when both operands are large and balanced, and the fused
`mul_add_mul`/`mul_sub_mul` kernels otherwise, with FLINT's word-count thresholds expressed in
bits so that they do not shift on 32-bit builds. Squaring prefers squarings to general
multiplications at every size and short-circuits purely real and purely imaginary values, as
`fmpzi_sqr` does. One detail carries over in Rust's terms: `fmpzi_mul` diverts to `fmpzi_sqr`
when its two operands are the same object, and `&x * &x` does the same, detecting the aliasing
by address. Both operations are in place through `*=` and
[`square_assign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SquareAssign.html).

**`fmpzi_pow_ui`.** Binary exponentiation over `fmpzi_sqr` and `fmpzi_mul`, with a purely
real base delegated to `fmpz_pow_ui` and a purely imaginary one to the same followed by the unit
$$i^n$$. Malachite's
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
for `GaussianInteger` is a port with the same special cases, the exponent a `u64` as for the
other bignum types.

**`fmpzi_mul_i`, `fmpzi_div_i`, `fmpzi_mul_i_pow_si`.** The three quarter-turn functions
the header declares beyond the manual chapter. Multiplying by $$i$$ sends $$a + bi$$ to
$$-b + ai$$ and dividing by $$i$$ sends it to $$b - ai$$, a swap of the parts and one
negation; Malachite's
[`MulI`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulI.html)
and
[`DivI`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivI.html)
traits do exactly that, with `_assign` forms. `fmpzi_mul_i_pow_si` multiplies by $$i^k$$ for any
signed $$k$$, reduced modulo 4 to one of the identity, `mul_i`, negation, or `div_i`; it is the
workhorse of unit canonicalization and of the $$(1 + i)$$-removal below. Malachite's
[`MulIPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.MulIPow.html)
takes a `u64` exponent, which loses nothing: only $$k$$ modulo 4 matters, and a negative FLINT
exponent is the same as $$3k$$. As in FLINT, canonicalization is defined as `mul_i_pow` of
`canonical_unit_i_pow`.

## [Division](https://flintlib.org/doc/fmpzi.html#division) {#division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpzi_divexact (fmpzi_t q, const fmpzi_t x, const fmpzi_t y)` | [`div_exact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✓ | `void fmpzi_divrem (fmpzi_t q, fmpzi_t r, const fmpzi_t x, const fmpzi_t y)` | [`div_rem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html), [`div_assign_rem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivAssignRem.html) |
| — | `void fmpzi_divrem_approx (fmpzi_t q, fmpzi_t r, const fmpzi_t x, const fmpzi_t y)` | |
| ✓ | `slong fmpzi_remove_one_plus_i (fmpzi_t res, const fmpzi_t x)` | [`remove_one_plus_i`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.remove_one_plus_i), [`remove_one_plus_i_assign`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.remove_one_plus_i_assign) |

**Division.** All three quotients are built from the same identity,
$$x / y = x \overline{y} / N(y)$$: multiply by the conjugate, then divide both parts by the
norm. For `fmpzi_divexact` the two integer divisions are exact, with a purely real divisor
short-cut to two `fmpz_divexact` calls, a purely imaginary one to the same followed by a
quarter turn, and a quotient below $$2^{45}$$ evaluated in double precision, which the exactness
contract makes safe; Malachite's
[`div_exact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html)
is a port of that function with the same tiers and the same contract (an inexact division may
panic or return nonsense), except that FLINT's remaining tier, an approximate division of
truncated operands for very unbalanced sizes, is not ported yet, and such operands take the
general path meanwhile. For `fmpzi_divrem` the divisions round: the source forms
$$2 x \overline{y} + N(y)(1 + i)$$ and takes the floor of each part divided by $$2 N(y)$$,
which rounds each coordinate of the exact quotient to the nearest integer with ties resolved
upward, so the remainder satisfies $$N(r) \leq N(y) / 2$$ and the choice at a tie is fixed, the
"canonical choice" the manual promises. Malachite's
[`div_rem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivRem.html)
and
[`div_assign_rem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivAssignRem.html)
are a port of that function, rounding the same way and returning the same pair, including
FLINT's short-cut that a dividend more than two bits smaller than the divisor has quotient zero;
the `/` and `%` operators are the same division, returning one half of the pair each.
`fmpzi_divrem_approx` estimates the rounded quotient in double precision when the operands are
within 45 bits of each other, giving $$N(r) < N(y)$$ with no promise about which remainder. It
exists to feed the GCD, and Malachite's GCD uses a private port of it; a division whose result
depends on the rounding of a double is not something Malachite will offer publicly, so the row is
marked as outside scope rather than as a gap.

**`fmpzi_remove_one_plus_i`.** $$1 + i$$ is the prime above 2, with $$(1 + i)^2 = 2i$$, so
its multiplicity in $$x$$ is $$2v$$ for the common power $$2^v$$ dividing both parts, plus one
more when the parts have the same 2-adic valuation and are both odd after removing it. FLINT
strips the power of 2 by shifting, corrects the unit with `fmpzi_mul_i_pow_si`, and applies
$$(a + bi) / (1 + i) = ((a + b) + (b - a) i) / 2$$ for the odd step; the return value is the
exponent, and zero maps to zero with exponent 0. Malachite's
[`remove_one_plus_i`](https://docs.rs/malachite-nz/latest/malachite_nz/gaussian_integer/struct.GaussianInteger.html#method.remove_one_plus_i)
is a port of that function, returning the reduced number and the exponent as a tuple, as
`RemovePower` does for the integer `fmpz_remove`; the `_assign` form reduces in place and
returns the exponent. It is the companion of the binary Gaussian GCD below.

## [GCD](https://flintlib.org/doc/fmpzi.html#gcd) {#gcd}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void fmpzi_gcd_euclidean (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | |
| ✓ | `void fmpzi_gcd_euclidean_improved (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`GcdAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GcdAssign.html) |
| — | `void fmpzi_gcd_binary (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | |
| ✗ | `void fmpzi_gcd_shortest (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | |
| ✓ | `void fmpzi_gcd (fmpzi_t res, const fmpzi_t x, const fmpzi_t y)` | [`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html), [`GcdAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.GcdAssign.html) |

**The GCD family.** $$\mathbb{Z}[i]$$ is a Euclidean domain under the norm, so it has GCDs,
defined up to a unit and returned in FLINT's [canonical unit form](#units). The four named
algorithms are the plain Euclidean algorithm over `fmpzi_divrem`, the same over
`fmpzi_divrem_approx`, a $$(1 + i)$$-ary analogue of the binary GCD built on
`fmpzi_remove_one_plus_i`, and a lattice method that reads the GCD off the shortest vector of
the lattice generated by the two inputs. The default dispatches on size, read from the source:
a fixed-size routine when all four parts fit in 50 bits, the improved Euclidean algorithm when
either operand is below 30,000 bits, and the lattice method above that, after one approximate
division to balance badly unbalanced operands. Every variant ends by putting the result in
canonical unit form, so all five return the same value. Malachite's
[`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html)
for `GaussianInteger` is a port of the default: the 50-bit double-precision kernel, and the
Euclidean algorithm over the approximate division (a private port of `fmpzi_divrem_approx`)
otherwise. The lattice method is not ported, so operands beyond 30,000 bits take the Euclidean
path too; the plain Euclidean and $$(1 + i)$$-ary variants are kept as test oracles rather than
public functions, since they are FLINT's internal algorithm menu rather than distinct operations
(the oracle's $$(1 + i)$$-ary variant chooses among $$x \pm y$$ and $$x \pm iy$$ by exact norm,
where FLINT's uses a double-precision estimate).

## [Primality testing](https://flintlib.org/doc/fmpzi.html#primality-testing) {#primality-testing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpzi_is_prime (const fmpzi_t n)` | |
| ✗ | `int fmpzi_is_probabprime (const fmpzi_t n)` | |

**`fmpzi_is_prime`, `fmpzi_is_probabprime`.** The Gaussian primes are, up to units, the
rational primes congruent to 3 modulo 4, and the values whose norm is a rational prime, which
covers $$1 + i$$ and the factors of the primes congruent to 1 modulo 4. Both functions follow
that classification directly, read from the source: for a purely real or purely imaginary
value, the nonzero part must be congruent to 3 modulo 4 and prime in absolute value, and
otherwise the norm must be prime; the two differ only in whether the integer test is
`fmpz_is_prime` or `fmpz_is_probabprime`. Malachite has neither, because it has no integer
primality test yet, the gap recorded on
[the fmpz page](/mapping/flint-integers/#primality-testing); when that lands, these two are a
few lines on top of it.
