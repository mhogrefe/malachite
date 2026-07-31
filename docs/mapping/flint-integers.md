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
[Conventions](/mapping/gmp-integers/#conventions) of the GMP page, including the
[Allocation](/mapping/gmp-integers/#allocation) discussion, apply here unchanged, since FLINT
follows GMP's calling style: results are written into an output argument you supply, and
Malachite returns them instead. The conventions below are the FLINT-specific ones.

Two groups of functions are omitted throughout. Functions whose names begin with an underscore
are FLINT-internal entry points with preconditions that the public functions establish for their
callers; they are not part of the surface a port works against. And a few self-contained groups,
noted in their sections, are summarized in prose rather than mapped row by row.

## Conventions {#conventions}

### The `fmpz` representation

The manual opens with a section of types, macros, and constants, and it is the one section of
the chapter that maps onto a design rather than onto functions. An `fmpz` is a single machine
word. When its two most significant bits are `00` or `11`, the word is an ordinary signed
integer, so any value whose absolute value is at most $$2^{62}-1$$ (on 64-bit machines) lives
inline, with no allocation. When they are `01`, the rest of the word is a shifted pointer to a
GMP integer. Promotion from one form to the other is automatic, and `COEFF_MAX`, `COEFF_MIN`,
`COEFF_IS_MPZ`, `PTR_TO_COEFF`, and `COEFF_TO_PTR` are the constants and macros that implement
the scheme.

Malachite makes the same design decision, one level up. A
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) is an
enum: a value below $$2^{64}$$ is stored inline, and a larger one owns a vector of limbs. An
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) is a
sign and a `Natural`, so any integer whose absolute value is below $$2^{64}$$ allocates nothing.
(With 32-bit limbs, the FLINT threshold is $$2^{30}-1$$ and the Malachite threshold is $$2^{32}$$.)

The two layouts trade differently, and the difference is worth knowing when porting. An `fmpz`
spends two bits of its word on the tag, so its inline range is a factor of four narrower than
Malachite's; in exchange, every `fmpz` is exactly one word, tagged or not, so an array of a
million of them is a dense array of words. FLINT's manual points out the memory and cache
benefits when very many small integers are in play, and those benefits are much of the reason
`fmpz` exists as a layer over GMP at all: polynomials and matrices are arrays of coefficients.
A Malachite value keeps its discriminant alongside its data rather than folded into it, so its
inline range is the full word, no bits are spent on tagging, and no pointer is ever disguised as
an integer; the value itself is a few words wide rather than one. For a single number the
difference is irrelevant. For aggregates it will come up again when Malachite's polynomial types
exist and are mapped.

None of the five macros and constants has a counterpart, and none needs one: they exist so that
FLINT code can test and build the tagged word, and the tag does not exist in Malachite, whose
representation is not part of its public API.

`fmpz_t` is an array of `fmpz` of length one, the same pass-by-reference device as GMP's
`mpz_t`, and the manual's advice about arrays (`fmpz myarr[100]`, with `myarr + 2` usable as an
`fmpz_t`) is C plumbing that Rust's references and slices replace: a `Vec<Integer>` is indexed,
borrowed, and passed without any device at all.

### Word types

FLINT's `ulong` and `slong` are the machine word on every 64-bit platform, Windows included, so
the `_ui` and `_si` function variants correspond exactly to `u64` and `i64`. As on the GMP page,
most of those variants collapse into their base function's row, since converting a primitive
integer to a `Natural` or an `Integer` costs almost nothing; the collapse is even tidier here,
with no `unsigned long` width variation to think about.

### Aliasing

The chapter states that, unless noted otherwise, its functions permit aliasing between inputs
and outputs, so `fmpz_mul(x, x, x)` squares `x` in place. Rust does not allow an output to alias
an input; the same computations are spelled through the assign and by-value forms described
under [Allocation](/mapping/gmp-integers/#allocation), and the specific case of
`fmpz_mul(x, x, x)` is a named operation,
[`Square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html),
with `x.square_assign()` as the in-place form.

### Categories

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Memory management](https://flintlib.org/doc/fmpz.html#memory-management) {#memory-management}

A leaner section than GMP's: there are no variadic `inits`/`clears` and no `realloc2`. The
[fuller discussion](/mapping/gmp-integers/#initializing-integers) on the GMP page, of why Rust
has no separate initialization step and nothing to clear, applies here without change.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_init (fmpz_t f)` | [`Natural::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html), [`Integer::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void fmpz_init2 (fmpz_t f, ulong limbs)` | |
| — | `void fmpz_clear (fmpz_t f)` | |
| ✓ | `void fmpz_init_set (fmpz_t f, const fmpz_t g)` | [`Clone`](https://doc.rust-lang.org/nightly/std/clone/trait.Clone.html) |
| ✓ | `void fmpz_init_set_ui (fmpz_t f, ulong g)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |
| ✓ | `void fmpz_init_set_si (fmpz_t f, slong g)` | [`From`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html), [`TryFrom`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) |

**`fmpz_init`.** `let x = Integer::ZERO;`, and the correspondence is closer here than it was for
GMP: a freshly initialized `fmpz` is a small one, a single word with no allocation behind it,
and `Natural::ZERO` and `Integer::ZERO` are likewise inline values that allocate nothing.
[`Default`](https://doc.rust-lang.org/nightly/std/default/trait.Default.html) returns zero for
both types too.

**`fmpz_init2`.** A capacity hint, and FLINT's manual is candid about its weight: "It is not
necessary to call this function except to save time. A call to `fmpz_init` will do just fine."
The same is true in Malachite, and the note under
[`mpz_init2`](/mapping/gmp-integers/#initializing-integers) on the GMP page, including the
[`Vec::with_capacity`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.with_capacity)
escape hatch for when a capacity really is worth controlling, applies verbatim.

**`fmpz_clear`.** Memory is released when the value drops out of scope. FLINT's manual notes
that a cleared `fmpz` returns its memory "either back to the stack or the OS, depending on
whether the reentrant or non-reentrant version of FLINT is built"; the non-reentrant build keeps
a global pool of GMP integers, which is the "FLINT wide array" the chapter's introduction
mentions. Malachite has no global state of any kind: each value owns its storage, and dropping
it returns the storage to the allocator, in any build.

**`fmpz_init_set`, `fmpz_init_set_ui`, `fmpz_init_set_si`.** `g.clone()`, `Natural::from(g)`,
and `Integer::from(g)`, exactly as in the
[corresponding GMP section](/mapping/gmp-integers/#combined-initialization-and-assignment),
including [`TryFrom`](https://doc.rust-lang.org/nightly/std/convert/trait.TryFrom.html) being
the way a signed value becomes a `Natural`.

## [Random generation](https://flintlib.org/doc/fmpz.html#random-generation) {#random-generation}

FLINT's random functions thread a `flint_rand_t` state through every call, initialized with
`flint_rand_init` and released with `flint_rand_clear`. That is the same shape as GMP's
`gmp_randstate_t`, and the
[same note](/mapping/gmp-integers/#random-number-functions) applies: Malachite's stream
generators take a [`Seed`](https://docs.rs/malachite-base/latest/malachite_base/random/struct.Seed.html)
and return an infinite iterator, its single-value `get_*` forms borrow a source of random words
mutably (a
[`StripedBitSource`](https://docs.rs/malachite-base/latest/malachite_base/num/random/striped/struct.StripedBitSource.html)
for the striped forms), and in neither case is there an initialize-and-release pair to balance.
Random generation lives behind the `random` feature, in the
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

**`fmpz_randbits`, `fmpz_randbits_unsigned`.** An exact bit length:
`get_random_natural_with_bits` chooses uniformly among the `Natural`s of precisely `bits` bits.
For the signed version, FLINT chooses the sign at random; compose the same thing with
`Integer::from_sign_and_abs` and a random `bool`.

**The `randtest` family.** These exist for the same reason as Malachite's stream generators: test
inputs should not all be large, since edge cases live among the small and the structured. The
distributions differ, which is what keeps the rows at ≈. FLINT draws the bit length uniformly
from `0` to `bits` inclusive, so all magnitudes up to a hard cap are equally represented.
Malachite's streams take a *mean* bit length and draw sizes from a geometric distribution, with
no hard cap; `random_nonzero_integers` covers the `not_zero` variant directly. Malachite also
has a second adversary that FLINT's `fmpz` module does not: the striped generators, described
[on the GMP page](/mapping/gmp-integers/#random-number-functions), which produce values with
long runs of zeros and ones to stress carry and borrow paths. If what you want is FLINT's exact
shape, draw a length and then a value:
[`get_random_natural_with_up_to_bits`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/fn.get_random_natural_with_up_to_bits.html)
caps the length but weights toward full-length values, so it is the uniform-value rather than
the uniform-length cap.

**`fmpz_randm`, `fmpz_randtest_mod`, `fmpz_randtest_mod_signed`.** The uniform one is exact:
`get_random_natural_less_than` for a single value, or the `random_naturals_less_than` stream.
The two `randtest_mod` forms bias toward the endpoints of the range (and toward zero, for the
signed one); Malachite's range generators come in uniform and striped variants instead, so the
adversarial values are the ones with extreme bit patterns rather than the ones near the
endpoints. The signed range `(-m/2, m/2]` is spelled as an ordinary range through
[`uniform_random_integer_inclusive_range`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/fn.uniform_random_integer_inclusive_range.html)
or its striped counterpart.

**`fmpz_randprime`.** A gap: Malachite does not generate random primes at bignum sizes,
because it does not yet test bignum primality; this row will be filled together with the
primality work described under
[Primes and factors](/mapping/gmp-integers/#number-theoretic-functions) on the GMP page. FLINT's
manual is worth reading on the semantics its version chose: the result is the next prime above a
random point, so the distribution over primes is not quite uniform, and the generator is "not
suitable for cryptographic use".

FLINT has more random generation in its limb-level
[mpn_extras](https://flintlib.org/doc/mpn_extras.html) module. That module is not mapped on
these pages: Malachite's limb-level functions are internal, not part of its public API.

## [Conversion](https://flintlib.org/doc/fmpz.html#conversion) {#conversion}

The largest section of the chapter, and three stories run through it. The conversions to and
from primitives follow the same policy-trait scheme as
[on the GMP page](/mapping/gmp-integers/#conversion-functions). The many multi-word forms, which
FLINT needs because C has no integer type wider than a word, collapse into Rust's 128-bit
primitives and Malachite's limb-slice conversions. And the conversions to and from GMP and MPFR
types are a boundary that Malachite does not have.

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
is the fits-test. The difference from GMP is that FLINT leaves more undefined: `fmpz_get_ui` of
a negative number is undefined where `mpz_get_ui` returns the low bits of the absolute value.
Malachite's policies are total; each trait says what happens.

**Words in pairs and arrays.** Eight of these functions exist because C has no integer wider
than a word, and they split into two groups. The two-word group is Rust's 128-bit primitives:
`fmpz_set_uiui(f, hi, lo)` is `Natural::from(x)` for a `u128`, `fmpz_set_signed_uiui` is
`Integer::from` of an `i128`, and the two `neg` forms compose with
[`Neg`](https://doc.rust-lang.org/nightly/std/ops/trait.Neg.html). In the other direction,
`fmpz_get_uiui` is a `u128` conversion under whichever policy you choose, and
`fmpz_get_signed_uiui`, which FLINT specifies as the value modulo $$2^{128}$$ in two's
complement, is `i128::wrapping_from` exactly. When what you have or want really is a pair of
words rather than a `u128`, Malachite mimics the `uiui` shape directly:
[`JoinHalves`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.JoinHalves.html)
builds the wide value as `u128::join_halves(hi, lo)`, and
[`SplitInHalf`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SplitInHalf.html)
takes it back apart with `split_in_half()`, whose pair is ordered `(hi, lo)` just as the FLINT
arguments are. The wider group is Malachite's limb-slice
conversions: `fmpz_set_ui_array` is `Natural::from_limbs_asc`, the `signed` versions (including
the three-word `fmpz_set_signed_uiuiui`) are `Integer::from_twos_complement_limbs_asc`, and the
`get` versions are `to_limbs_asc` and `to_twos_complement_limbs_asc`, which return a
minimal-length [`Vec`](https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html) where FLINT
fills exactly `n` words; extend with zero words (or sign words, in the two's complement case) if
a fixed width is what you need. `fmpz_set_mpn_large` is `from_limbs_asc` plus
`Integer::from_sign_and_abs`, and its preconditions (at least two limbs, normalized top limb)
are not required: `from_limbs_asc` takes any slice, unnormalized or empty. `fmpz_get_mpn`
allocates an array and returns its length, which is `to_limbs_asc` returning a `Vec`.

**`fmpz_get_d`, `fmpz_set_d`, and the `2exp` pair.** `fmpz_get_d` truncates toward zero, so it
is `f64::rounding_from(&f, Down)`; FLINT leaves the result undefined when the value is out of
`double` range, and the
[defined overflow behavior](/mapping/gmp-integers/#conversion-functions) described on the GMP
page applies instead. `fmpz_set_d` also truncates, through `Integer::rounding_from(c, Down)`,
and FLINT's undefined cases are narrowed: an infinity or NaN fails `try_from` and makes
`rounding_from` panic, and a subnormal, which FLINT also declares undefined, is an ordinary tiny
value that truncates to zero. `fmpz_get_d_2exp` is
[`SciMantissaAndExponent`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SciMantissaAndExponent.html),
with the mantissa-normalization difference noted
[on the GMP page](/mapping/gmp-integers/#conversion-functions) for `mpz_get_d_2exp`.
`fmpz_set_d_2exp` rounds $$d \cdot 2^{exp}$$ to the nearest integer; since every finite `double`
is a rational, the exact spelling is `Rational::try_from(d)` shifted by `exp`, then
`Integer::rounding_from(&q, Nearest)`.

**The GMP and MPFR boundary.** Nine functions convert between `fmpz_t` and GMP's `mpz_t` and
`mpf_t`, including the four `readonly` functions, which exist to present a value across that
boundary without copying it. The boundary is there because FLINT is built on GMP; Malachite is
not, so none of these has or needs a counterpart. When a value must cross into C, or into GMP
through Rust bindings such as [rug](https://docs.rs/rug/latest/rug/), the routes are the
limb-slice conversions above, strings, or [serde](https://serde.rs/). The exception in the group
is `fmpz_get_mpfr`: Malachite models MPFR's type as
[`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), so
`Float::from_integer_prec_round(f, prec, rnd)` is a real counterpart, taking the target
precision explicitly where MPFR reads it from `x`, and returning an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) that reports the
rounding direction, the ternary result that FLINT's `void` wrapper discards. For `mpf_t`, see the
[note on the GMP page](/mapping/gmp-integers/#assigning-integers) about `mpf`'s status.

**`fmpz_get_nmod`.** The result is `f` reduced modulo a word: per the
[Conventions](/mapping/gmp-integers/#conventions), spell it with `Natural::from(m)` and
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html),
then convert the word-sized result back down. The `nmod_t` argument is more than a modulus: it
carries a precomputed inverse so that repeated reductions avoid division. Malachite has no
public precomputed-modulus context; that machinery reappears throughout FLINT's `fmpz_mod`
chapter, and the trade is taken up properly on
[its page](/mapping/flint-integers-mod-n/#conventions).

**`fmpz_get_str`, `fmpz_set_str`.** The same pair as GMP's string functions, and the same
match: `to_string_base` and `from_string_base` cover the full base range of 2 through 62 with
the digit alphabet FLINT delegates to GMP for, described in
[the notes on the GMP page](/mapping/gmp-integers/#assigning-integers) along with
[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) in place of a `-1`
return and [`String`](https://doc.rust-lang.org/nightly/std/string/struct.String.html) in place
of a caller-sized buffer. Two differences from GMP do not carry over: FLINT's `fmpz_get_str`
accepts `NULL` and allocates, so its buffer-management hazard is already halfway to Rust's, and
FLINT has no negative-base uppercase mode, so `to_string_base` alone is the whole of
`fmpz_get_str` and `to_string_base_upper` is extra. On the parsing side FLINT skips surrounding
whitespace itself and inherits GMP's tolerance of interior whitespace; Malachite accepts no
whitespace, accepts a single leading `+` that FLINT and GMP reject, and, like FLINT's own
special check, rejects a doubled minus sign.

**`fmpz_set_ui_smod`.** A balanced remainder: the representative of `x` modulo `m` in
`(-m/2, m/2]`, which is
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html).
FLINT states a precondition, that `x` already satisfies `0 <= x < m`, and does not check it;
`balanced_mod` reduces first, so any
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) is
accepted and the result is the same wherever FLINT's precondition holds. Because the
representative may be negative, the output is an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) even
though the inputs are not. The full-size version is `fmpz_smod` in the next section, the same
trait.

## [Input and output](https://flintlib.org/doc/fmpz.html#input-and-output) {#input-and-output}

As on [the GMP page](/mapping/gmp-integers/#input-and-output-functions), there is no `FILE *`
half to port: Rust keeps formatting and I/O separate, so nothing in Malachite needs to know
about streams, and none of it is restricted to `stdio`.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_read (fmpz_t f)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ✓ | `int fmpz_fread (FILE * file, fmpz_t f)` | [`FromStr`](https://doc.rust-lang.org/nightly/std/str/trait.FromStr.html) |
| ≈ | `size_t fmpz_inp_raw (fmpz_t x, FILE * fin)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ✓ | `int fmpz_print (const fmpz_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ✓ | `int fmpz_fprint (FILE * fs, const fmpz_t x)` | [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) |
| ≈ | `size_t fmpz_out_raw (FILE * fout, const fmpz_t x)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |

**`fmpz_read`, `fmpz_fread`.** Pull a token out of your reader and hand it to
`parse::<Integer>()`; these two are decimal-only, with `from_string_base` covering the other
bases as under `fmpz_set_str`. Failure is an `Err` rather than a non-positive return, and
Malachite's parser is the more permissive of the two: FLINT rejects leading zeros, while
`"00123"` parses.

**`fmpz_print`, `fmpz_fprint`.** `print!("{x}")`, or `write!(f, "{x}")` for any
[`Write`](https://doc.rust-lang.org/nightly/std/io/trait.Write.html). The sign-then-digits
format is what [`Display`](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html)
produces. FLINT returns the number of characters written; Rust's formatting reports failure
through a `Result` instead, and `x.to_string().len()` gives the count when the count is what
you want.

**`fmpz_out_raw`, `fmpz_inp_raw`.** FLINT delegates to GMP's raw format, 4 bytes of size and
then big-endian limbs, so this pair is byte-compatible with `mpz_out_raw` and `mpz_inp_raw`, and
the [discussion on the GMP page](/mapping/gmp-integers/#input-and-output-functions) of how that
format corresponds to Malachite's [serde](https://serde.rs/) support, including the encoding and
size differences that keep the rows at ≈, carries over without change.

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

**`fmpz_sizeinbase`, `fmpz_bits`, `fmpz_size`, `fmpz_sgn`.** The measurement functions.
`fmpz_bits` is `significant_bits()`, agreeing at zero too: both return 0. `fmpz_size` is
`limb_count()`, on the `Natural` itself or through
[`unsigned_abs_ref`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.unsigned_abs_ref)
for an `Integer`'s absolute value, and both again report 0 for zero. `fmpz_sgn` is
[`Sign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Sign.html),
returning an [`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) as
[on the GMP page](/mapping/gmp-integers/#comparison-functions). For `fmpz_sizeinbase`,
`f.floor_log_base(&base) + 1` is the digit count; unlike `mpz_sizeinbase`, FLINT's version
states no overshoot, so the exactness caveat in the
[GMP note](/mapping/gmp-integers/#miscellaneous-functions) does not arise, but the zero caveat
does: `floor_log_base` panics on zero, so the zero case is yours to spell out.

**`fmpz_val2`.** The 2-adic valuation, `trailing_zeros()`, on the `Natural` or on an
`Integer`'s absolute value. The ≈ is the zero convention: zero is divisible by every power of
two, and FLINT answers 0 while Malachite answers
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), the same
sentinel-becomes-[`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html)
policy described
[on the GMP page](/mapping/gmp-integers/#logical-and-bit-manipulation-functions).

**`fmpz_swap`, `fmpz_set`, `fmpz_zero`, `fmpz_one`.**
[`std::mem::swap`](https://doc.rust-lang.org/nightly/std/mem/fn.swap.html); `x = y.clone()` or
`x.clone_from(&y)`; `x = Integer::ZERO` and `x = Integer::ONE`. Assigning a constant to an
existing variable is the same operation as creating one, since the old value simply drops.

**`fmpz_abs_fits_ui`, `fmpz_fits_si`.** `u64::convertible_from(f.unsigned_abs_ref())` and
`i64::convertible_from(&f)`. The
[six-functions-into-one-trait note](/mapping/gmp-integers/#miscellaneous-functions) from the GMP
page applies; FLINT documents only these two of the family.

**`fmpz_setbit`, `fmpz_tstbit`.** `f.set_bit(i)` and `f.get_bit(i)`, from
[`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html),
indexed from the least significant bit, with negative values acting as their two's complement.
The rest of the family, clearing and flipping, appears under
[Logic Operations](#logic-operations) below, where FLINT keeps it.

**`fmpz_abs_lbound_ui_2exp`, `fmpz_abs_ubound_ui_2exp`.** A word-sized mantissa of exactly
`bits` bits and an exponent bounding `|x|` from below or above. Malachite's named version of
this operation is
[`sci_mantissa_and_exponent_round`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.sci_mantissa_and_exponent_round):
passing `Floor` or `Ceiling` produces a mantissa-and-exponent pair that bounds the value from
below or above, along with an
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) saying whether the
bound is exact. The difference is the mantissa's form: FLINT returns an integer of a
caller-chosen width up to a word, while Malachite returns an `f32` or `f64` in `[1, 2)`, so the
width is the float format's 24 or 53 bits. For FLINT's exact shape at any width, the spelling is
a rounded shift: with `e = x.significant_bits() as i64 - bits`, the mantissa is
`x.unsigned_abs_ref().shr_round(e, Floor).0` for the lower bound or `Ceiling` for the upper (a
negative `e` shifts left, exactly). One edge matches FLINT's own caveat: when the ceiling
carries to a power of two, the mantissa has gained a bit, which is the case FLINT describes as
the exponent coming out one too large; renormalize with one more shift if you need the mantissa
width exact.

## [Comparison](https://flintlib.org/doc/fmpz.html#comparison) {#comparison}

As on [the GMP page](/mapping/gmp-integers/#comparison-functions), comparison is where Malachite
provides every combination of types, so the `_ui` and `_si` variants land on the same traits as
their base functions; and the discussions there of
[`Ordering`](https://doc.rust-lang.org/nightly/std/cmp/enum.Ordering.html) replacing the
sign-carrying `int`, and of why single-type rows say
[`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) while mixed-type rows say
[`PartialOrd`](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialOrd.html), carry over
unchanged.

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

**`fmpz_cmp2abs`.** [`OrdAbsDouble`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbsDouble.html)'s
`cmp_abs_double` compares `|f|` against `|2g|` without materializing the doubled value, which is the point of it:
the comparison of something against twice something else is the shape of a round-to-nearest
decision, where a remainder is weighed against half a divisor, and it belongs in an inner loop
with no allocation in sight. `f.cmp_abs(&(g << 1u32))` gets the same answer but builds the
temporary the FLINT function exists to avoid. On
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), where
there are no signs to take absolute values of, the same comparison is
[`OrdDouble`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdDouble.html)'s
`cmp_double`. The two traits split by sign exactly as
[`Ord`](https://doc.rust-lang.org/nightly/std/cmp/trait.Ord.html) and
[`OrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.OrdAbs.html)
do, and both are implemented for the primitive integers too, where the doubling would overflow
rather than allocate.
Both sit beside
[`cmp_normalized`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#method.cmp_normalized),
which compares two values as if each were scaled into $$[1, 2)$$, likewise without scaling
anything.

**The predicates.** `fmpz_is_zero` and `fmpz_is_one` are `f == 0` and `f == 1`: mixed equality
with a small constant compares against the inline representation and allocates nothing, so the
dedicated fast-path predicates are not needed as separate functions. `fmpz_is_pm1` is
`f.eq_abs(&1)`, absolute-value equality from
[`EqAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.EqAbs.html);
and the parity pair is `f.even()` and `f.odd()`, as
[on the GMP page](/mapping/gmp-integers/#miscellaneous-functions).

`fmpz_cmpabs` and `fmpz_is_pm1` also mark a small asymmetry with GMP worth knowing when porting
between the three libraries: GMP has `mpz_cmpabs` variants against `double` and `ulong`, which
Malachite covers with
[`PartialOrdAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/comparison/traits/trait.PartialOrdAbs.html),
while FLINT has neither; the mixed absolute-value comparisons are there either way.

## [Basic arithmetic](https://flintlib.org/doc/fmpz.html#basic-arithmetic) {#basic-arithmetic}

The chapter's largest section by far, so it is divided here into themed subsections, in the
manual's own order. Throughout, the general shape is the one described
[on the GMP page](/mapping/gmp-integers/#arithmetic-functions): where FLINT writes into an
output argument, Malachite returns the result, the `*Assign` traits cover the in-place case,
every operator has borrowing forms, and the `_ui` and `_si` variants collapse into their base
function's row.

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
| ✗ | `void fmpz_fmma (fmpz_t f, const fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_t d)` | |
| ✗ | `void fmpz_fmms (fmpz_t f, const fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_t d)` | |

**The operators.** `-g`, `g.abs()`, `g + h`, `g - h`, `g * h`, and `g << e`, with
[`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html)
producing the absolute value as a `Natural` rather than an `Integer`. The subtraction and
`submul` rows are ≈ for the reason
[explained on the GMP page](/mapping/gmp-integers/#arithmetic-functions): on `Natural`, a
difference that would go negative panics, and
[`CheckedSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedSub.html)
or
[`SaturatingSub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SaturatingSub.html)
make the borrow a decision instead. On `Integer`, which is what an `fmpz` is, they are plain ✓.

**`fmpz_mul2_uiui`.** The double word-multiply: form the word product exactly first, since
`x * y` cannot overflow a `u128`, and multiply once at full size:
`g * Natural::from(u128::from(x) * u128::from(y))`. That keeps FLINT's point, one bignum pass
rather than two.

**`fmpz_mul_2exp`, `fmpz_one_2exp`.** `g << e`, and `Integer::power_of_2(e)` from
[`PowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowerOf2.html)
for the power of two itself. FLINT's caveat that `e + FLINT_BITS` must not overflow has no
counterpart; the shift count is an ordinary integer, of
[any primitive type](/mapping/gmp-integers/#arithmetic-functions).

**`fmpz_addmul`, `fmpz_submul`.** `f.add_mul_assign(&g, &h)` and `f.sub_mul_assign(&g, &h)`
accumulate in one step without a temporary, exactly as FLINT's do; the three-operand
`f.add_mul(&g, &h)` forms return the result instead.

**`fmpz_fmma`, `fmpz_fmms`.** The fused double products, `ab + cd` and `ab - cd`, are gaps. The
spelling `&a * &b + &c * &d` computes the right value but allocates both products and the sum
separately, where FLINT's fused versions manage their scratch space once; like `fmpz_cmp2abs`
above, these earn their keep in inner loops, rational arithmetic being the motivating case,
since a sum of fractions has an `ad + bc` numerator, the shape behind
[`fmpq_addmul`](/mapping/flint-rationals/#arithmetic) on the rationals page.

### Division with rounding

FLINT's division matrix uses GMP's naming: `cdiv` rounds the quotient toward positive infinity,
`fdiv` toward negative infinity, and `tdiv` toward zero. The
[trait-per-rounding-mode dictionary](/mapping/gmp-integers/#division-functions) from the GMP
page carries over directly: `cdiv` is
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

**`fmpz_ndiv_qr`.** The member with no GMP ancestor, and the section's ≈. `DivRound` with
`Nearest` rounds the quotient to minimize the remainder, as `ndiv` does, but the two disagree on
ties: FLINT rounds a tied quotient toward zero, while Malachite's `Nearest` rounds it to even.
Dividing 3 by 2 shows the difference: both quotients 1 and 2 leave a remainder of absolute value
1, and `fmpz_ndiv_qr` picks 1 where `(3).div_round(2, Nearest)` picks 2. When the tie rule
matters, break it yourself: compare twice the truncated remainder against the divisor, which is
the comparison `fmpz_cmp2abs` exists for. `DivRound` also returns only the quotient; the
remainder is `g - &q * h`, or one
[`SubMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SubMul.html).

**The `2exp` columns.** Dividing by $$2^{exp}$$ with a rounding mode is
[`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html):
`Ceiling`, `Floor`, and `Down` reproduce `cdiv`, `fdiv`, and `tdiv`, and a plain `g >> exp`
floors, matching `fdiv_q_2exp`. The three remainder versions are the three flavors of reduction
modulo a power of two: `ModPowerOf2` (nonnegative, floor), `CeilingModPowerOf2` (nonpositive,
ceiling), and `RemPowerOf2` (sign of `g`, truncating), an exact one-to-one match.

**`fmpz_cdiv_ui`, `fmpz_fdiv_ui`, `fmpz_tdiv_ui`.** These return the absolute value of the
remainder as a word. The floor remainder is already nonnegative, so `fmpz_fdiv_ui` is `Mod`
directly; the ceiling remainder is nonpositive and the truncated remainder takes `g`'s sign, so
those two compose with
[`UnsignedAbs`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.UnsignedAbs.html)
before converting the word-sized result down.

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
| ✗ | `void fmpz_preinvn_init (fmpz_preinvn_t inv, const fmpz_t f)` | |
| — | `void fmpz_preinvn_clear (fmpz_preinvn_t inv)` | |
| ✗ | `void fmpz_fdiv_qr_preinvn (fmpz_t f, fmpz_t s, const fmpz_t g, const fmpz_t h, const fmpz_preinvn_t hinv)` | |

**The `divexact` family.** `g.div_exact(&h)`, with the same contract as FLINT's: exactness is
the caller's promise, not checked, and the result is undefined if the promise is broken. The
[GMP page's note](/mapping/gmp-integers/#division-functions) on why the unchecked version is
worth having applies. `fmpz_divexact2_uiui` uses the same double-word trick as
`fmpz_mul2_uiui`: form `x * y` exactly as a `u128` and divide once.

**`fmpz_divisible`, `fmpz_divides`.** `f.divisible_by(&g)` answers the question;
`fmpz_divides`, which GMP does not have, answers it and hands over the quotient. One
`(q, r) = f.div_mod(&g)` does both at once, with `r == 0` as the verdict and `q` already in
hand, which is one division, the same work FLINT's version does. The zero case differs: FLINT
defines `fmpz_divides(q, f, 0)` to report whether `f` is zero, while `div_mod`
by zero panics, so test the divisor first when zero can reach this code.

**`fmpz_mod`, `fmpz_mod_ui`.** FLINT's `fmpz_mod` produces a nonnegative remainder whatever the
sign of `h`, and that is exactly
[`ModEuclidean`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModEuclidean.html)'s
convention: `mod_euclidean` returns the always-nonnegative remainder, as a `Natural`, and
`mod_euclidean_assign` leaves it in place. For a positive
divisor, which is all `fmpz_mod_ui` allows, plain
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html)
agrees and is the more common spelling.

**`fmpz_smod`.** The balanced remainder again, in `(-|h|/2, |h|/2]`, the full-size version of
`fmpz_set_ui_smod` [under Conversion](#conversion) and the same trait,
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html),
with an `Assign` form as well. The conventions agree: only the magnitude of `h` matters, and a
remainder of exactly `|h|/2` is the positive one, since the range is closed at the top.

**The `preinvn` trio.** A precomputed Newton inverse of a divisor, built once and reused so
that many divisions by the same `h` skip the setup work. Malachite has preinverted division in
its internals but no public context type, so the operation is a gap; `fmpz_preinvn_clear` alone
is —, since whenever such a type exists, releasing it is what
[`Drop`](https://doc.rust-lang.org/nightly/std/ops/trait.Drop.html) does. This is the second
appearance of FLINT's precomputed-modulus pattern, after `fmpz_get_nmod`
[under Conversion](#conversion), and the right shape for a public version is a question for the
`fmpz_mod` chapter, which is built around exactly such contexts; see
[its page](/mapping/flint-integers-mod-n/#conventions).

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
libraries. `fmpz_pow_fmpz` takes the exponent as a full integer and reports failure instead of
building an impossibly large result; the spelling is `u64::try_from(&x)` followed by `pow`, with
the conversion's `Err` playing the role of FLINT's failure return, and FLINT's throw on a
negative exponent likewise landing in the conversion. One case deserves care when porting:
for `g` in `{-1, 0, 1}` the power fits no matter how large `x` is, and FLINT succeeds there, so
handle those three bases before converting the exponent.

**`fmpz_powm`, `fmpz_powm_ui`.** [`ModPow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html),
with the [three differences explained on the GMP page](/mapping/gmp-integers/#exponentiation-functions)
for `mpz_powm`: the base must be reduced ahead of time, the residues are `Natural`s, and a
negative exponent is spelled as
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
followed by a positive power. FLINT's `abort` on a zero modulus is a panic.

**`fmpz_clog`, `fmpz_flog`.** `x.ceiling_log_base(&b)` and `x.floor_log_base(&b)`, exact
integer logarithms with the same domain FLINT assumes: both panic for `x` outside `x >= 1` or a
base below 2, so the assumption is enforced rather than trusted.
[`CheckedLogBase`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedLogBase.html)
additionally reports whether `x` is an exact power of `b`, a question FLINT answers separately
with `fmpz_is_perfect_power`; and the base-2 and power-of-2 cases have dedicated fast versions,
[`FloorLogBase2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorLogBase2.html)
and its relatives.

**`fmpz_dlog`.** Malachite's
[`approx_log`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.approx_log)
is a port of this very function, reached through `Rational::from(&x)` since it lives on
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html).
One difference in kind: FLINT notes that the accuracy "depends on the implementation of the
floating-point logarithm provided by the C standard library", while Malachite computes through
the [libm](https://docs.rs/libm/latest/libm/) crate, a pure-Rust port of MUSL's math library, so
the approximation is the same bit pattern on every platform.

### Roots

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_sqrtmod (fmpz_t b, const fmpz_t a, const fmpz_t p)` | |
| ✓ | `void fmpz_sqrt (fmpz_t f, const fmpz_t g)` | [`FloorSqrt`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorSqrt.html) |
| ✓ | `void fmpz_sqrtrem (fmpz_t f, fmpz_t r, const fmpz_t g)` | [`SqrtRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SqrtRem.html) |
| ✓ | `int fmpz_is_square (const fmpz_t f)` | [`IsSquare`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsSquare.html) |
| ≈ | `int fmpz_root (fmpz_t r, const fmpz_t f, slong n)` | [`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html), [`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html), [`CheckedRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedRoot.html) |
| ✓ | `int fmpz_is_perfect_power (fmpz_t root, const fmpz_t f)` | [`ExpressAsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.ExpressAsPower.html) |

**`fmpz_sqrt`, `fmpz_sqrtrem`, `fmpz_is_square`.** `g.floor_sqrt()`, `g.sqrt_rem()`, and
`f.is_square()`, with FLINT's exception on a negative operand becoming a panic. FLINT's warning
that `fmpz_sqrtrem`'s two outputs must not alias is the same C rule the
[GMP page notes](/mapping/gmp-integers/#root-extraction-functions) has nothing to correspond to:
`sqrt_rem` returns a tuple. `IsSquare` is `Natural`-only, which costs nothing, since no negative
number is a square; test the sign first when the input is an `Integer`.

**`fmpz_root`.** The same ≈ as `mpz_root`, for the same reason,
[explained on the GMP page](/mapping/gmp-integers/#root-extraction-functions): "the integer part
of the root" truncates toward zero, so on a negative operand with odd `n` it is Malachite's
[`CeilingRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CeilingRoot.html),
and on a nonnegative one it is
[`FloorRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.FloorRoot.html).
FLINT's exactness flag is
[`CheckedRoot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedRoot.html),
which returns the root only when it is exact, or
[`RootRem`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.RootRem.html)
when you want the flag and the truncated root at once.

**`fmpz_sqrtmod`.** A square root modulo a prime is a gap, and it will stay one until the
modular-arithmetic story it belongs to is built out; it is the third entry in the
`fmpz_mod`-shaped queue, after the `nmod` and `preinvn` contexts above. FLINT's caveats are
worth reading before porting code that uses it: primality of `p` is assumed, not checked, and a
composite `p` usually, but not always, reports failure.

**`fmpz_is_perfect_power`.** Malachite's
[`ExpressAsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.ExpressAsPower.html)
has exactly this function's shape, and more precisely than GMP's predicate does:
`express_as_power` on 64 returns `Some((2, 6))`, the root and the exponent, where
`fmpz_is_perfect_power` sets the root and returns the exponent. The conventions agree too: 0 and
1 count as perfect powers, and neither library promises the smallest root. FLINT's negative
operands are covered as well, since the trait is implemented for
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html): a
negative value can only be an odd power, so $$-8$$ counts as $$(-2)^3$$ and the exponent
returned for a negative value is always odd. The bare predicate, without the root, is
[`IsPower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.IsPower.html).

### Combinatorial functions and fused operations

The last stretch of the section: three classics, a pair of rising factorials, and two fused
multiply-shifts.

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_fac_ui (fmpz_t f, ulong n)` | [`Factorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Factorial.html) |
| ✓ | `void fmpz_fib_ui (fmpz_t f, ulong n)` | [`Fibonacci`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Fibonacci.html) |
| ✓ | `void fmpz_bin_uiui (fmpz_t f, ulong n, ulong k)` | [`BinomialCoefficient`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BinomialCoefficient.html) |
| ✗ | `void fmpz_rfac_ui (fmpz_t r, const fmpz_t x, ulong k)` | |
| ✗ | `void fmpz_rfac_uiui (fmpz_t r, ulong x, ulong k)` | |
| ✓ | `void fmpz_mul_tdiv_q_2exp (fmpz_t f, const fmpz_t g, const fmpz_t h, ulong exp)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |
| ✓ | `void fmpz_mul_si_tdiv_q_2exp (fmpz_t f, const fmpz_t g, slong x, ulong exp)` | [`Mul`](https://doc.rust-lang.org/nightly/std/ops/trait.Mul.html), [`ShrRound`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ShrRound.html) |

**`fmpz_fac_ui`, `fmpz_fib_ui`, `fmpz_bin_uiui`.** `Natural::factorial(n)`,
`Natural::fibonacci(n)`, and `Natural::binomial_coefficient(n, k)`. The wider families are
described on the GMP page, under
[Factorials and binomial coefficients](/mapping/gmp-integers/#number-theoretic-functions):
double, multi-, and subfactorials beyond FLINT's `fmpz` offering, and
[`fibonacci_pair`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Fibonacci.html)
with the Lucas variants beside it.

**`fmpz_rfac_ui`, `fmpz_rfac_uiui`.** The rising factorial $$x (x+1) \cdots (x+k-1)$$ has no
Malachite counterpart yet, at either operand size. In the meantime the identity
$$x^{(k)} = \binom{x+k-1}{k} \, k!$$ spells it through `BinomialCoefficient` and `Factorial`,
which handles a bignum `x` as well, since the binomial coefficient takes `Natural` arguments.

**`fmpz_mul_tdiv_q_2exp`, `fmpz_mul_si_tdiv_q_2exp`.** `(&g * &h).shr_round(exp, Down).0`.
Unlike the fused `fmma` pair above, these are not marked as gaps: FLINT's implementation
multiplies in full and then shifts, so the composition is the same work in the same order. The
one detail to keep is the rounding mode: `tdiv` truncates toward zero, and a plain `>>` floors,
so a negative product needs `shr_round` with `Down` rather than the operator.

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
| ✗ | `void fmpz_xgcd_partial (fmpz_t co2, fmpz_t co1, fmpz_t r2, fmpz_t r1, const fmpz_t L)` | |

**`fmpz_gcd`, `fmpz_gcd3`, `fmpz_lcm`.** As
[on the GMP page](/mapping/gmp-integers/#number-theoretic-functions),
[`Gcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Gcd.html)
and
[`Lcm`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Lcm.html)
are defined on
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), and
FLINT documents both results as nonnegative whatever the signs of the inputs, so nothing is
lost: take
[`unsigned_abs`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.unsigned_abs)
of each
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) first
and the answer is the one FLINT would give. `fmpz_gcd3` is `a.gcd(&b).gcd(&c)`, the two calls
FLINT describes its fused version as equivalent to.

**The `xgcd` family.** Three functions, three cofactor conventions, and the canonical one is
the match. `fmpz_xgcd_canonical_bezout` pins down Bézout cofactors by
$$|a| < |g/2d|$$ and $$|b| < |f/2d|$$, which is the same normalization
[`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html)
uses (and GMP too, as
[noted on the GMP page](/mapping/gmp-integers/#number-theoretic-functions)), with the edge cases
agreeing as well, so `extended_gcd` is a drop-in with identical cofactors. It is also the
function FLINT itself recommends, calling it faster than plain `fmpz_xgcd`; the plain version's
cofactors follow the `fmpz_gcdinv` convention instead, which is why its row is ≈. FLINT's
no-aliasing rules for the outputs have nothing to correspond to: the results arrive as a tuple.

**`fmpz_gcdinv`.** The one-cofactor extended GCD, for `0 <= f < g`: it returns
`d = gcd(f, g)` together with `a` satisfying $$af \equiv d \pmod{g}$$. When the answer you want
is the modular inverse,
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
is the direct spelling, returning
[`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) when `f` and `g` are not
coprime instead of handing back a `d > 1` to check. When you want `d` and the cofactor in every
case, `extended_gcd`'s first cofactor satisfies the same congruence, reduced modulo `g` as
needed; its normalization differs from `fmpz_gcdinv`'s, hence the ≈.

**`fmpz_xgcd_partial`.** Lehmer's extended GCD with early termination, stopping once the
remainders fall below a bound: a building block, used by FLINT's binary quadratic form module,
for algorithms such as Cornacchia's that need only the middle of the remainder sequence.
Malachite has half-GCD machinery internally but no public partial GCD, so this is a gap; it
belongs to the same family of algorithm-component exports as the Lucas chains under
[Primality testing](#primality-testing) below. It is also the engine of
[rational reconstruction](/mapping/flint-rationals/#modular-reduction-and-rational-reconstruction),
a gap on the rationals page; the two are one work item.

## [Modular arithmetic](https://flintlib.org/doc/fmpz.html#modular-arithmetic) {#modular-arithmetic}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpz_remove (fmpz_t rop, const fmpz_t op, const fmpz_t f)` | [`RemovePower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.RemovePower.html) |
| ≈ | `int fmpz_invmod (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html) |
| ✓ | `void fmpz_negmod (fmpz_t f, const fmpz_t g, const fmpz_t h)` | [`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html), [`NegMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegMod.html) |
| ≈ | `int fmpz_jacobi (const fmpz_t a, const fmpz_t n)` | [`JacobiSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.JacobiSymbol.html), [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✓ | `int fmpz_kronecker (const fmpz_t a, const fmpz_t n)` | [`KroneckerSymbol`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.KroneckerSymbol.html) |
| ✗ | `void fmpz_divides_mod_list (fmpz_t xstart, fmpz_t xstride, fmpz_t xlength, const fmpz_t a, const fmpz_t b, const fmpz_t n)` | |

**`fmpz_remove`.** [`RemovePower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.RemovePower.html),
described [on the GMP page](/mapping/gmp-integers/#number-theoretic-functions), which returns the
reduced number and the count as a tuple instead of writing one and returning the other. The
conventions line up with FLINT's on both edges it names: a zero input removes nothing, and a
factor of 1 or below is rejected, where FLINT aborts and Malachite panics.

**`fmpz_invmod`.** [`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
returns [`Option`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html), so FLINT's
flag-plus-undefined-output protocol becomes a value that exists exactly when the inverse does,
as with [`mpz_invert`](/mapping/gmp-integers/#number-theoretic-functions). Two differences make
the row ≈. `mod_inverse` requires its input already reduced, nonzero and below the modulus,
where FLINT reduces for you; and FLINT defines a special case at `h = ±1`, declaring everything
invertible with inverse 0, which has no counterpart, since modulo 1 the only residue is 0 and
`mod_inverse` rejects a zero input. Test for a unit modulus first when porting code that can
reach it.

**`fmpz_negmod`.** Two spellings, differing in what they do with FLINT's precondition that `g`
be already reduced. `g.mod_neg(&h)`, from the
[`ModNeg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html)
family of reduced-operand modular functions, shares the precondition but enforces it: where
FLINT does not check and an unreduced `g` gives an undefined result, `mod_neg` panics.
`g.neg_mod(&h)`, from
[`NegMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegMod.html),
drops the precondition instead, reducing any `g`'s negation into `[0, h)`. Reduced inputs give
the same answer either way; `mod_neg` is the closer match in spirit and cost, and `neg_mod` is
the one to reach for when `g` might not be reduced.

**`fmpz_jacobi`, `fmpz_kronecker`.** The
[symbol discussion on the GMP page](/mapping/gmp-integers/#number-theoretic-functions) applies
in full: `KroneckerSymbol` is total and matches for every input, while `JacobiSymbol` enforces
the classical domain, an odd positive denominator. FLINT's `fmpz_jacobi` states that "the parity
and sign of `n` are not checked", so where FLINT would return an undefined answer, Malachite
panics or, through the Kronecker symbol, returns the defined extension.

**`fmpz_divides_mod_list`.** The solution set of the linear congruence $$ax \equiv b \pmod n$$,
returned as an arithmetic progression: start, stride, and count. No Malachite function solves a
linear congruence yet, so this is a gap, and a planned one to fill. The classical recipe
composes from mapped pieces in the meantime: with `(d, s, _) = a.extended_gcd(&n)`, there are
solutions exactly when `d` divides `b`, and then the progression is
`x = s * (b / d) mod (n / d)`, stride `n / d`, count `d`.

## [Bit packing and unpacking](https://flintlib.org/doc/fmpz.html#bit-packing-and-unpacking) {#bit-packing-and-unpacking}

These three functions exist to pack polynomial coefficients into a contiguous bit array and read
them back out, which is how FLINT implements Kronecker-substitution multiplication; the "array"
is a raw limb buffer that the caller walks through. The Malachite counterpart is
[`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html),
which reads or writes a block of adjacent bits at any position in a `Natural` or an `Integer`,
with the buffer itself held as a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html): the
[limb-slice conversions](#conversion) turn a raw buffer into one and back without copying more
than once, and `get_bits`/`assign_bits` take arbitrary `u64` bit positions, so the walking that
FLINT does with a limb pointer plus a sub-word `shift` is a single index here.

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int fmpz_bit_pack (ulong * arr, flint_bitcnt_t shift, flint_bitcnt_t bits, const fmpz_t coeff, int negate, int borrow)` | [`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |
| ≈ | `int fmpz_bit_unpack (fmpz_t coeff, ulong * arr, flint_bitcnt_t shift, flint_bitcnt_t bits, int negate, int borrow)` | [`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |
| ✓ | `void fmpz_bit_unpack_unsigned (fmpz_t coeff, const ulong * arr, flint_bitcnt_t shift, flint_bitcnt_t bits)` | [`BitBlockAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitBlockAccess.html) |

**`fmpz_bit_unpack_unsigned`.** The exact match: extracting `bits` bits starting at a given
position is `buffer.get_bits(pos, pos + bits)`.

**`fmpz_bit_pack`, `fmpz_bit_unpack`.** The ≈ is the signed-coefficient protocol. FLINT's
`negate` and `borrow` parameters implement a running two's-complement encoding across the packed
fields: a negative coefficient is stored as its complement within the field, and the deficit is
carried into the next field as a borrow, threaded through the whole array by the polynomial
code. `assign_bits` and `get_bits` move the raw fields, and for nonnegative coefficients with no
borrow in play they are the whole story; the borrow-and-negate thread itself is
polynomial-packing plumbing with no counterpart, and it will matter, if at all, only when
Malachite's polynomial types exist and choose their own packing. One more small difference
favors the caller here: `fmpz_bit_pack` adds its field into the array and assumes the bits above
`shift` are zero on entry, while `assign_bits` overwrites the range unconditionally, holes or
not.

## [Logic Operations](https://flintlib.org/doc/fmpz.html#logic-operations) {#logic-operations}

Negative operands behave as their two's complement, sign-extended without end, in both
libraries; the [worked account](/mapping/gmp-integers/#logical-and-bit-manipulation-functions)
on the GMP page applies verbatim.

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
their `Assign` forms; and `f.clear_bit(i)` and `f.flip_bit(i)`, completing the
[`BitAccess`](https://docs.rs/malachite-base/latest/malachite_base/num/logic/traits/trait.BitAccess.html)
family whose `set_bit` and `get_bit` FLINT keeps under
[Basic properties and manipulation](#basic-properties-and-manipulation) above.

**`fmpz_popcnt`.** `count_ones()` on a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html). FLINT
declares the result undefined for negative input, GMP returns a sentinel, and Malachite's
`Integer` has
[`checked_count_ones`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#method.checked_count_ones),
whose [`None`](https://doc.rust-lang.org/nightly/std/option/enum.Option.html) covers the case
both C libraries leave to the caller: a negative number's two's complement has infinitely many
ones.

## [Chinese remaindering](https://flintlib.org/doc/fmpz.html#chinese-remaindering) {#chinese-remaindering}

This section's thirteen functions are not mapped row by row; they are one facility, and
Malachite does not have it yet. The facility is residue recombination: `fmpz_CRT` and
`fmpz_CRT_ui` combine a pair of congruences, and the rest of the section serves the many-moduli
case, with `fmpz_comb_t` structures that precompute a tree of products so that reducing one
integer modulo many primes, or reassembling it from many residues, costs less than doing each
modulus separately. It is the largest instance of the precomputed-context pattern this page has
met as `nmod_t` and `fmpz_preinvn_t`, and the design of such contexts belongs to
[the `fmpz_mod` page](/mapping/flint-integers-mod-n/#conventions). Until the facility exists,
every row here would read ✗ for the same
reason, which is no more informative than this paragraph; the two-congruence case composes
today from mapped pieces, through
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
and the arithmetic above.

## [Primality testing](https://flintlib.org/doc/fmpz.html#primality-testing) {#primality-testing}

Not mapped yet. Malachite does not test bignum primality today, so the headline functions,
`fmpz_is_prime`, `fmpz_is_probabprime` with its named variants (BPSW, Lucas, strong
probable-prime), and `fmpz_nextprime`, are all gaps of one family; the
[Primes and factors discussion](/mapping/gmp-integers/#number-theoretic-functions) on the GMP
page describes the planned work, and FLINT's API is the model named there. The section also
exports building blocks, the six `fmpz_lucas_chain` functions and
`fmpz_divisor_in_residue_class_lenstra`, which sit with `fmpz_xgcd_partial` in the
algorithm-component family. The section will be mapped in full when the primality work lands.

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
`n`, exactly as FLINT defines $$n\#$$. Malachite adds the other convention as a separate
function, `product_of_first_n_primes`, along with
[`CheckedPrimorial`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.CheckedPrimorial.html)
variants for the primitive types.

**The multiplicative functions.** Euler's totient, the Möbius function, and the divisor sums
$$\sigma_k$$ are all gaps, and they are one gap rather than three: each is read off a prime
factorization, which is why FLINT provides every one in two forms, taking `n` itself or a
precomputed `fmpz_factor_t`. Malachite factors primitive integers, through
[`Factor`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.Factor.html),
but not yet `Natural`s, and the multiplicative functions belong downstream of that machinery, at
both operand sizes. The `fmpz_factor_t`-taking forms raise a further design question, what a
public factorization type should look like, that is deferred along with FLINT's `fmpz_factor`
module itself; when that module is taken up, these six rows come with it.
