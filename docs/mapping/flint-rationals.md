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
here as well.

Functions whose names begin with an underscore are omitted throughout, and this chapter states
its own reason better than a general policy could: FLINT documents them as component-level
variants that "may perform less error checking, and may impose limitations on aliasing", the
internal surface beneath the public one. About twenty such variants are skipped.

## Conventions {#conventions}

### Canonical form, three ways

FLINT defines canonical form as `Rational` does, down to the zero convention: numerator and
denominator without a common factor, a positive denominator, and a denominator of one when the
numerator is zero. The three libraries that share this page's type differ in who maintains it.
GMP's `mpq_t` leaves canonicalization to the caller after several assignment functions, the
situation described at the top of
[the GMP rationals page](/mapping/gmp-rationals/). FLINT moves the responsibility into the
library: every `fmpq` function assumes canonical inputs and produces canonical outputs, and the
caller only takes over after reaching through `fmpq_numref` or `fmpq_denref` to edit a component
directly, at which point "passing a non-canonical `fmpq_t` gives undefined results" until
`fmpq_canonicalise` is called. Malachite closes the remaining gap: component access is mediated,
so a non-canonical
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
cannot be built or observed at all. In practice this means the mapping below carries no
canonicalization caveats: on the shared ground of canonical values, the two libraries' functions
agree about what they consume and produce.

### The `fmpq` representation

An `fmpq` is a pair of `fmpz`s, numerator and denominator, each a tagged word that stores small
values inline, so a rational with word-sized components occupies two words and no allocated
memory, and each component promotes to a GMP integer independently as it grows. The trade-offs
of that tagged-word scheme are analyzed under
[the fmpz representation](/mapping/flint-integers/#conventions); they apply per component here.
A [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) is
a sign and two
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s, each
inline below $$2^{64}$$. The visible difference is where the sign lives: FLINT's numerator is a
signed integer, while Malachite's numerator and denominator are magnitudes with the sign held
separately, the difference that runs through
[the accessor section](/mapping/gmp-rationals/#applying-integer-functions-to-rationals) of the
GMP rationals page and reappears wherever this chapter touches components. `fmpq_t` is an
array of length one, the same pass-by-reference device as every `_t` before it.

### Categories

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/fmpq.html#types-macros-and-constants) {#types-macros-and-constants}

The types themselves are covered under [Conventions](#conventions); the section's two functions
are the component accessors.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `fmpz * fmpq_numref (const fmpq_t x)` | [`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.numerator_ref), [`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator) |
| ≈ | `fmpz * fmpq_denref (const fmpq_t x)` | [`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.denominator_ref), [`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_denominator) |

**`fmpq_numref`, `fmpq_denref`.** The same pair as GMP's `mpq_numref` and `mpq_denref`, and the
[full discussion on the GMP rationals page](/mapping/gmp-rationals/#applying-integer-functions-to-rationals)
carries over: reading is free, through `numerator_ref` and `denominator_ref`, with the sign
asked for separately; writing goes through the `mutate_*` closures, which reduce the value when
the closure returns. The closures are the answer to this chapter's sharpest caveat: FLINT's
"user becomes responsible for canonicalising the number" after editing through these pointers is
exactly the responsibility the mediated access discharges automatically.

## [Memory management](https://flintlib.org/doc/fmpq.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_init (fmpq_t x)` | [`Rational::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void fmpq_clear (fmpq_t x)` | |

**`fmpq_init`, `fmpq_clear`.** `let x = Rational::ZERO;`, holding the canonical zero, 0/1, that
`fmpq_init` produces; both are a few inline words with nothing allocated. Dropping replaces
clearing, and the [fuller discussion](/mapping/gmp-integers/#initializing-integers) of why Rust
has neither step is on the GMP page.
[`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) returns zero here
too.

## [Canonicalisation](https://flintlib.org/doc/fmpq.html#canonicalisation) {#canonicalisation}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void fmpq_canonicalise (fmpq_t res)` | |
| — | `int fmpq_is_canonical (const fmpq_t x)` | |

This is where the [three-ways spectrum](#conventions) settles its accounts. FLINT needs these
two functions because one road to a non-canonical `fmpq_t` stays open, editing components
through `fmpq_numref` and `fmpq_denref`; the repair is `fmpq_canonicalise` and the audit is
`fmpq_is_canonical`. In Malachite that road is closed, so there is nothing to repair and
nothing to audit: the
[`mutate_*` closures](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.mutate_numerator)
reduce on the way out, and every
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) a
program can observe would answer `fmpq_is_canonical` with yes. Even the corner FLINT leaves
undefined, canonicalising a fraction whose denominator has been set to zero, is closed: the
closure that could produce such a denominator panics instead of returning. Malachite does check
the invariant, in one place, for the same reason FLINT provides the audit function: values
arriving from outside. Deserialization
[validates canonical form](/mapping/gmp-rationals/#input-and-output-functions), so the check
runs exactly where an unchecked value could otherwise enter.

## [Basic assignment](https://flintlib.org/doc/fmpq.html#basic-assignment) {#basic-assignment}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_set (fmpq_t dest, const fmpq_t src)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| — | `void fmpq_swap (fmpq_t op1, fmpq_t op2)` | |
| ✓ | `void fmpq_neg (fmpq_t dest, const fmpq_t src)` | [`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html), [`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html) |
| ✓ | `void fmpq_abs (fmpq_t dest, const fmpq_t src)` | [`Abs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Abs.html), [`AbsAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.AbsAssign.html) |
| ✓ | `void fmpq_zero (fmpq_t res)` | [`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void fmpq_one (fmpq_t res)` | [`ONE`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html) |

Six functions, one note. `dest = src.clone()`, or `dest.clone_from(&src)` to reuse `dest`'s
allocations; [`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html) for the
exchange; `-src` and `src.abs()`, sign flips that leave every
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
untouched, with their `Assign` forms in place; and `res = Rational::ZERO` or `Rational::ONE`,
assignment of a constant being the same act as creation. FLINT's remark that `fmpq_set`
performs no canonicalisation is not a hazard but a consequence of the chapter's contract: a
copy of a canonical value is canonical.

## [Comparison](https://flintlib.org/doc/fmpq.html#comparison) {#comparison}

As on [the GMP rationals page](/mapping/gmp-rationals/#comparison-functions), comparison is
where Malachite provides every combination of types, and the discussions there, of
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) replacing the
sign-carrying `int` and of the
[`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html)/[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html)
split, carry over. The collapse is even more direct here than it was for GMP: where
`mpq_cmp_ui` compares against a fraction passed as separate numerator and denominator, FLINT's
`_fmpz`, `_si`, and `_ui` forms compare against plain integers, which is exactly what the mixed
[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html) and
[`PartialEq`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html) implementations
take.

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
| ✗ | `void fmpq_height (fmpz_t height, const fmpq_t x)` | |
| ✗ | `flint_bitcnt_t fmpq_height_bits (const fmpq_t x)` | |

**The predicates and comparisons.** `x == 0`, `x == 1`, and `x.eq_abs(&1)` for the `pm1` test;
`==` against another
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), or
any primitive; `x.sign()`; and `<` and friends for the orderings, with primitive floats also
comparable, exactly, a combination neither C library offers. On canonical values, `==` is the
[structural equality](/mapping/gmp-rationals/#comparison-functions) described on the GMP
rationals page, and `fmpq_equal`'s implementation makes the same component-by-component
comparison.

**`fmpq_height`, `fmpq_height_bits`.** The height of a rational, the larger of `|p|` and `q`,
is the measure by which Diophantine results are stated: bounds on heights are what rational
reconstruction consumes, later in this chapter, and what approximation theorems are written in.
Malachite has no height function yet; a dedicated one is planned. Until then the composition is
short, since the components are already magnitudes:
`max(x.numerator_ref(), x.denominator_ref())` is the height, and `.significant_bits()` of that
is `fmpq_height_bits`.

## [Conversion](https://flintlib.org/doc/fmpq.html#conversion) {#conversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_set_fmpz_frac (fmpq_t res, const fmpz_t p, const fmpz_t q)` | [`from_integers`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_integers) |
| ≈ | `void fmpq_get_mpz_frac (mpz_t a, mpz_t b, fmpq_t c)` | [`to_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.to_numerator_and_denominator) |
| ✓ | `void fmpq_set_si (fmpq_t res, slong p, ulong q)` | [`from_signeds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_signeds), [`from_sign_and_unsigneds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_sign_and_unsigneds) |
| ✓ | `void fmpq_set_ui (fmpq_t res, ulong p, ulong q)` | [`from_unsigneds`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_unsigneds) |
| — | `void fmpq_set_mpq (fmpq_t dest, const mpq_t src)` | |
| ✗ | `int fmpq_set_str (fmpq_t dest, const char * s, int base)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ≈ | `double fmpq_get_d (const fmpq_t f)` | [`RoundingFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html), [`TryFrom`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html) |
| — | `void fmpq_get_mpq (mpq_t dest, const fmpq_t src)` | |
| ✓ | `int fmpq_get_mpfr (mpfr_t dest, const fmpq_t src, mpfr_rnd_t rnd)` | [`from_rational_prec_round`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#method.from_rational_prec_round) |
| ✗ | `char * fmpq_get_str (char * str, int b, const fmpq_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| — | `void flint_mpq_init_set_readonly (mpq_t z, const fmpq_t f)` | |
| — | `void flint_mpq_clear_readonly (mpq_t z)` | |
| — | `void fmpq_init_set_readonly (fmpq_t f, const mpq_t z)` | |
| — | `void fmpq_clear_readonly (fmpq_t f)` | |

**The fraction constructors.** `fmpq_set_fmpz_frac` is `Rational::from_integers(p, q)`, and the
word-sized forms are `from_unsigneds` and `from_signeds`: all of them, in both libraries,
produce "the canonical form of the fraction `p / q`", reducing on the way in. One signature
mixes types: `fmpq_set_si` takes a signed `p` over an unsigned `q`, and a `q` above `i64::MAX`
does not fit `from_signeds`; `from_sign_and_unsigneds(p >= 0, p.unsigned_abs(), q)` covers the
full range. A zero denominator panics at the construction site.

**`fmpq_get_mpz_frac`.** The pair coming back out is `to_numerator_and_denominator`, with
`into_numerator_and_denominator` to avoid the copy; the ≈ is the usual sign placement, FLINT's
`a` being signed where Malachite returns magnitudes, as discussed in
[the accessor section](/mapping/gmp-rationals/#applying-integer-functions-to-rationals) of the
GMP rationals page.

**`fmpq_get_d`.** Truncation toward zero: `f64::rounding_from(&f, Down)`. FLINT calls the
result "system dependent" out of `double` range, and the
[defined behavior](/mapping/gmp-rationals/#conversion-functions) described for `mpq_get_d`
applies in its place, at both ends of the range.

**`fmpq_get_mpfr`.** `Float::from_rational_prec_round(src, prec, rnd)`, a full match down to
the return value: `fmpq_get_mpfr` reports "the sign of the rounding, according to MPFR
conventions", which is the
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) in Malachite's
return, and this time nothing is discarded, unlike
[`fmpz_get_mpfr`](/mapping/flint-integers/#conversion), whose `void` wrapper dropped it.

**`fmpq_set_str`, `fmpq_get_str`.** The string pair carries the same base gap as every page so
far, with one asymmetry gone: FLINT's rational strings themselves stop at base 36, so when
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) grows
the `from_string_base` and `to_string_base` promised on
[the GMP rationals page](/mapping/gmp-rationals/#conversion-functions), the two ranges will
coincide exactly; there is no base-62 remainder here. Base 10 works today through
[`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) and
[`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html). Two small
conventions differ in the usual directions: `fmpq_set_str` zeroes `dest` and returns `-1` on a
bad string where parsing returns an `Err` and no value at all, and `fmpq_get_str` accepts
`NULL` and allocates, halfway to returning a
[`String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html).

**The GMP boundary.** `fmpq_set_mpq`, `fmpq_get_mpq`, and the four `readonly` functions are the
`mpq_t` edition of the boundary
[described on the fmpz page](/mapping/flint-integers/#conversion): FLINT is built on GMP and
Malachite is not, so nothing corresponds, and interop routes through numerators and
denominators, strings, or serde.

## [Input and output](https://flintlib.org/doc/fmpq.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_fprint (FILE * file, const fmpq_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✓ | `int fmpq_print (const fmpq_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |

**`fmpq_print`, `fmpq_fprint`.** `print!("{x}")`, or `write!(f, "{x}")` for any
[`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html); as on
[every I/O section before this one](/mapping/flint-integers/#input-and-output), there is no
`FILE *` half to port, and the success-or-failure `int` becomes a `Result`. The formats agree
exactly, though the manual undersells it: FLINT's doc describes numerator, slash, denominator
unconditionally, but the implementation prints a bare integer when the denominator is one, which
is [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html)'s format to the
character.

## [Random number generation](https://flintlib.org/doc/fmpq.html#random-number-generation) {#random-number-generation}

A section GMP's rational chapter does not have. The `flint_rand_t` state maps as it did
[on the fmpz page](/mapping/flint-integers/#random-generation): Malachite's stream generators
take a [`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html)
and return infinite iterators, with nothing to initialize and release in pairs. The generators
live in
[`rational::random`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/index.html),
behind the `random` feature, and every value they produce is canonical, as FLINT's are.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpq_randtest (fmpq_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`random_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.random_rationals.html), [`striped_random_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.striped_random_rationals.html) |
| ≈ | `void fmpq_randtest_not_zero (fmpq_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`random_nonzero_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.random_nonzero_rationals.html) |
| ✓ | `void fmpq_randbits (fmpq_t res, flint_rand_t state, flint_bitcnt_t bits)` | [`get_random_natural_with_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_bits.html), [`from_sign_and_naturals`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_sign_and_naturals) |

**`fmpq_randtest`, `fmpq_randtest_not_zero`.** FLINT says plainly what this kind of generator
is for: it "has an increased probability of generating special values which are likely to
trigger corner cases". That is the philosophy of Malachite's test generators too, pursued with
different machinery, which keeps the rows at ≈. FLINT caps the components at `bits` bits and
mixes in its special values; Malachite's streams draw component sizes from a geometric
distribution around a mean, with no hard cap, and the deliberately-adversarial values come from
the striped generators, whose long runs of ones and zeros are described
[on the GMP integers page](/mapping/gmp-integers/#random-number-functions).
`random_nonzero_rationals` covers the `not_zero` variant directly.

**`fmpq_randbits`.** Exact component sizes: draw the numerator and denominator with
`get_random_natural_with_bits`, choose a sign, and let `from_sign_and_naturals` assemble the
fraction. The caveat in FLINT's doc, that canonicalisation can leave the components "slightly
smaller than `bits` bits", holds identically here, and for the same reason: the constructor
reduces, just as `fmpq_randbits` canonicalises after sampling.

## [Arithmetic](https://flintlib.org/doc/fmpq.html#arithmetic) {#arithmetic}

The operators do the core of this section, as they did
[for GMP's rationals](/mapping/gmp-rationals/#arithmetic-functions): results are returned, the
`*Assign` traits cover the in-place case, every operator has borrowing forms, and canonical in,
canonical out is the invariant on both sides. What FLINT adds over GMP is the family of mixed
rational-and-integer forms, and those collapse the usual way: converting an integer of any size
to a [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)
costs almost nothing, so each `_si`, `_ui`, and `_fmpz` variant lands on its base operator's
trait.

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
| ✗ | `void fmpq_addmul (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | |
| ✗ | `void fmpq_submul (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | |
| ✓ | `void fmpq_inv (fmpq_t dest, const fmpq_t src)` | [`Reciprocal`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Reciprocal.html), [`ReciprocalAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ReciprocalAssign.html) |
| ✓ | `void fmpq_pow_si (fmpq_t res, const fmpq_t op, slong e)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html) |
| ✓ | `int fmpq_pow_fmpz (fmpq_t a, const fmpq_t b, const fmpz_t e)` | [`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html), [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) |
| ✓ | `void fmpq_mul_fmpz (fmpq_t res, const fmpq_t op, const fmpz_t x)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html) |
| ✓ | `void fmpq_div_fmpz (fmpq_t res, const fmpq_t op, const fmpz_t x)` | [`Div`](https://doc.rust-lang.org/nightly/std/ops/trait.Div.html) |
| ✓ | `void fmpq_mul_2exp (fmpq_t res, const fmpq_t x, flint_bitcnt_t exp)` | [`Shl`](https://doc.rust-lang.org/nightly/std/ops/trait.Shl.html), [`ShlAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShlAssign.html) |
| ✓ | `void fmpq_div_2exp (fmpq_t res, const fmpq_t x, flint_bitcnt_t exp)` | [`Shr`](https://doc.rust-lang.org/nightly/std/ops/trait.Shr.html), [`ShrAssign`](https://doc.rust-lang.org/nightly/std/ops/trait.ShrAssign.html) |
| ✗ | `void fmpq_gcd (fmpq_t res, const fmpq_t op1, const fmpq_t op2)` | |
| ✗ | `void fmpq_gcd_cofactors (fmpq_t g, fmpz_t abar, fmpz_t bbar, const fmpq_t a, const fmpq_t b)` | |

**The operators and mixed forms.** `op1 + op2` through `op1 / op2`, with division by zero a
panic where FLINT raises an error, and FLINT's blanket aliasing permission answered by the
assign and by-value forms as
[on the fmpz page](/mapping/flint-integers/#conventions). The ten mixed rows are one habit:
`op1 + Rational::from(c)`, for `c` of any primitive or
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) size,
with a by-value `Integer` donating its storage to the numerator.

**`fmpq_addmul`, `fmpq_submul`.** The fused accumulate, `res += op1 * op2`, is a gap, and a
familiar one: it is the rational operation whose numerator arithmetic is the `ad + bc` shape
that `fmpz_fmma` [exists to fuse](/mapping/flint-integers/#basic-arithmetic), so the two gaps
are close kin and are likely to be filled together. The spelling `res += &op1 * &op2` computes
the right value with a separately allocated product in the middle.

**`fmpq_inv`, `fmpq_pow_si`, `fmpq_pow_fmpz`.** `src.reciprocal()`, a numerator-denominator
swap [as on the GMP rationals page](/mapping/gmp-rationals/#arithmetic-functions); and
`op.pow(e)` with a signed exponent, agreeing that $$0^0 = 1$$. `fmpq_pow_fmpz` takes the
exponent as a full integer and reports failure instead of building an impossibly large result;
the treatment written for
[`fmpz_pow_fmpz`](/mapping/flint-integers/#basic-arithmetic) transfers whole: convert with
`i64::try_from(&e)`, let the conversion's `Err` play the failure return, and handle the bases
whose powers stay small, here `0` and `±1`, before converting.

**`fmpq_mul_2exp`, `fmpq_div_2exp`.** `x << exp` and `x >> exp`, both exact, a rational having
[nowhere to lose bits to](/mapping/gmp-rationals/#arithmetic-functions).

**`fmpq_gcd`, `fmpq_gcd_cofactors`.** The GCD of two rationals, which FLINT defines as the
canonical form of $$\gcd(ps, qr)/(qs)$$ for $$p/q$$ and $$r/s$$, noting that this "is apparently
Euclid's original definition and is stable under scaling of numerator and denominator". No
Malachite counterpart exists yet; one is planned. For canonical inputs the definition reduces
to a composition over components,
`Rational::from_naturals(p.gcd(&r), q.lcm(&s))` with numerators `p`, `r` and denominators `q`,
`s`, and the cofactors that `fmpq_gcd_cofactors` also returns, `a/g` and `b/g`, are exact
integers by construction.

## [Modular reduction and rational reconstruction](https://flintlib.org/doc/fmpq.html#modular-reduction-and-rational-reconstruction) {#modular-reduction-and-rational-reconstruction}

The round trip at the heart of multimodular algorithms: project a rational to a residue, compute
with residues, and recover the rational at the end. The projection maps; the recovery is this
page's largest gap.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_mod_fmpz (fmpz_t res, const fmpq_t x, const fmpz_t mod)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html), [`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html) |
| ✗ | `int fmpq_reconstruct_fmpz_2 (fmpq_t res, const fmpz_t a, const fmpz_t m, const fmpz_t N, const fmpz_t D)` | |
| ✗ | `int fmpq_reconstruct_fmpz (fmpq_t res, const fmpz_t a, const fmpz_t m)` | |

**`fmpq_mod_fmpz`.** The residue `a` with $$n \equiv a d \pmod m$$: reduce the denominator,
invert it, and multiply by the reduced numerator, on the machinery of
[the fmpz_mod page](/mapping/flint-integers-mod-n/#arithmetic). That composition is FLINT's own
algorithm, call for call: its implementation is `fmpz_invmod`, a multiplication, and a
reduction, with the success flag being the inverse's existence, which is
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)'s
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html). One reading note: the
doc's "if such an `a` exists" sounds broader than invertibility, but the implementation fails
exactly when the denominator has no inverse, so the mapped composition and FLINT agree on every
input.

**`fmpq_reconstruct_fmpz_2`, `fmpq_reconstruct_fmpz`.** The way back. Given a residue `a`
modulo `m` and height bounds `N`, `D` with $$2ND < m$$, reconstruction finds the unique
fraction `n/d` with $$|n| \le N$$, $$0 < d \le D$$, and $$n \equiv ad \pmod m$$, if one exists;
the shorter form uses the balanced bounds $$N = D = \lfloor\sqrt{(m-1)/2}\rfloor$$. This is how a multimodular computation, many images
under `fmpq_mod_fmpz` combined by CRT, becomes a rational answer again. Malachite has none of
it yet, and the missing engine is one already on the books: reconstruction is a half-GCD run
with early termination, precisely the algorithm exported as `fmpz_xgcd_partial`,
[a gap on the fmpz page](/mapping/flint-integers/#greatest-common-divisor), so the two are one
work item. The bounds `N` and `D` are height bounds, the quantity whose function is
[committed to under Comparison](#comparison); the pieces are planned together.

## [Rational enumeration](https://flintlib.org/doc/fmpq.html#rational-enumeration) {#rational-enumeration}

FLINT enumerates the rationals one step at a time, with `next` functions; Malachite enumerates
them as infinite iterators, in
[`rational::exhaustive`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/index.html).
For the Calkin-Wilf order the two agree term for term.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_next_minimal (fmpq_t res, const fmpq_t x)` | |
| ✗ | `void fmpq_next_signed_minimal (fmpq_t res, const fmpq_t x)` | |
| ✓ | `void fmpq_next_calkin_wilf (fmpq_t res, const fmpq_t x)` | [`exhaustive_non_negative_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/fn.exhaustive_non_negative_rationals.html) |
| ✓ | `void fmpq_next_signed_calkin_wilf (fmpq_t res, const fmpq_t x)` | [`exhaustive_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/exhaustive/fn.exhaustive_rationals.html) |
| ✗ | `void fmpq_farey_neighbors (fmpq_t l, fmpq_t r, const fmpq_t x, const fmpz_t Q)` | [`Approximate`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/traits/trait.Approximate.html) |
| ≈ | `void fmpq_simplest_between (fmpq_t x, const fmpq_t l, const fmpq_t r)` | [`SimplestRationalInInterval`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/traits/trait.SimplestRationalInInterval.html) |

**`fmpq_next_calkin_wilf`, `fmpq_next_signed_calkin_wilf`.** The breadth-first traversal of the
Calkin-Wilf tree is `exhaustive_non_negative_rationals()`, and the signed interleaving is
`exhaustive_rationals()`; the sequences are identical, element by element, from the leading 0
on. FLINT's stepping function becomes an iterator's `next`.

**`fmpq_next_minimal`, `fmpq_next_signed_minimal`.** Enumeration in order of height, the same
quantity behind this page's other gaps, and no Malachite iterator produces this order yet; a
height-ordered enumeration is a natural companion to the committed height function. Malachite
does have a different canonical well-order, the complexity order of
[`cmp_complexity`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.cmp_complexity),
the order behind its simplest-in-interval machinery, which sorts by denominator before
numerator rather than by height; neither is a substitute for the other.

**`fmpq_farey_neighbors`.** The fractions directly below and above `x` in the Farey sequence of
order `Q` have no direct counterpart. The nearest relative is
[`Approximate`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/traits/trait.Approximate.html),
which returns the best approximation with denominator at most `Q`, one of the two neighbors;
the pair as a pair is a gap.

**`fmpq_simplest_between`.** `Rational::simplest_rational_in_closed_interval(&l, &r)`, with an
open-interval variant FLINT does not have. The ≈ is a genuine difference in what "simplest"
means when the minimal denominator is achieved more than once. FLINT breaks ties by the signed
numerator, and its behavior matches its documentation: asked for the simplest fraction in
$$[-1, 1]$$ it returns $$-1$$, and in $$[-7, -3]$$ it returns $$-7$$. Malachite prefers the
numerator closest to zero and then the positive sign, returning 0 and $$-3$$ for the same
intervals. On intervals containing a single fraction of minimal denominator, which is the
typical case for the intervals these functions are pointed at, the two agree exactly.

## [Continued fractions](https://flintlib.org/doc/fmpq.html#continued-fractions) {#continued-fractions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpq_get_cfrac (fmpz * c, fmpq_t rem, const fmpq_t x, slong n)` | [`ContinuedFraction`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/traits/trait.ContinuedFraction.html) |
| ✓ | `slong fmpq_get_cfrac_naive (fmpz * c, fmpq_t rem, const fmpq_t x, slong n)` | [`ContinuedFraction`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/traits/trait.ContinuedFraction.html) |
| ✓ | `void fmpq_set_cfrac (fmpq_t x, const fmpz * c, slong n)` | [`from_continued_fraction`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.from_continued_fraction) |
| — | `slong fmpq_cfrac_bound (const fmpq_t x)` | |

**`fmpq_get_cfrac`, `fmpq_get_cfrac_naive`.** `x.continued_fraction()` returns the floor as an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) and
the remaining terms as an iterator of
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)s,
where FLINT writes all coefficients into one vector with the possibly-negative $$c_0$$ first.
FLINT's protocol for incremental extraction, take `n` terms and feed the remainder back in to
continue, is what a lazy iterator does by existing: the suspended iterator is the remainder.
Both libraries document the same convention for the expansion itself: of the two continued
fractions every rational has, the shorter one is generated.

**`fmpq_set_cfrac`.** `Rational::from_continued_fraction(floor, terms)`, with a `_ref` variant
that borrows the terms. One requirement is stated more strictly: FLINT asks that the
coefficients after $$c_0$$ "should be nonnegative", while Malachite requires them positive,
which the shorter canonical expansion always satisfies. Malachite also has
[`Convergents`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/traits/trait.Convergents.html),
an iterator over the partial values of the expansion, a function FLINT's `fmpq` module does not
offer.

**`fmpq_cfrac_bound`.** A preallocation bound for the coefficient vector, which FLINT
justifies with the fact that "the smallest denominator that can give a continued fraction of
length `n` is the Fibonacci number" $$F_{n+1}$$; an iterator allocates nothing ahead of time,
so there is nothing to bound.

## [Special functions](https://flintlib.org/doc/fmpq.html#special-functions) {#special-functions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_harmonic_ui (fmpq_t x, ulong n)` | |

**`fmpq_harmonic_ui`.** The harmonic number $$H_n = 1 + 1/2 + \cdots + 1/n$$, a gap. FLINT
evaluates it by table lookup at word size and divide-and-conquer beyond, which is the
binary-splitting shape Malachite already uses elsewhere; until the function exists, a fold of
`Rational::from_unsigneds(1, i)` computes the value without the divide-and-conquer speed.

## [Dedekind sums](https://flintlib.org/doc/fmpq.html#dedekind-sums) {#dedekind-sums}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_dedekind_sum (fmpq_t s, const fmpz_t h, const fmpz_t k)` | |
| — | `void fmpq_dedekind_sum_naive (fmpq_t s, const fmpz_t h, const fmpz_t k)` | |

**`fmpq_dedekind_sum`.** The Dedekind sum $$s(h, k)$$, the sawtooth correlation sum that
appears in the transformation law of the Dedekind eta function and downstream of it, in the
Hardy-Ramanujan-Rademacher evaluation of the partition function. The fast version is a gap.
FLINT's manual lays out its evaluation in full: a walk down the Euclidean remainder sequence of
`h` and `k`, accumulating a rational term at each step, which makes it one more algorithm over
the GCD remainder sequence, the recurring machinery of this page's gaps. The formula in
FLINT's documentation composes directly from mapped arithmetic when the sum is needed before
the function exists.

**`fmpq_dedekind_sum_naive`.** The reference implementation of the defining sum, slow for
large `k` by FLINT's own description. Malachite keeps reference implementations of this kind
out of its public API: they live in its test utilities as oracles that the fast versions are
property-tested against, which is where a naive Dedekind sum will go when the fast one is
built. The row is outside the public mapping rather than a gap.