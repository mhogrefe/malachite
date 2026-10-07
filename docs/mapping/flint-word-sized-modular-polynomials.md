---
layout: default
title: "Malachite for FLINT Users: Word-Sized Modular Polynomials"
permalink: /mapping/flint-word-sized-modular-polynomials/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Word-Sized Modular Polynomials

This page maps the functions of FLINT's word-sized modular polynomial type, `nmod_poly_t` —
polynomials over $$\mathbb{Z}/n\mathbb{Z}$$ for a fixed modulus $$n$$ that fits in a machine word
— onto their Malachite counterpart:
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html),
from the `malachite-base` crate. It follows the organization of the [nmod_poly.h
chapter](https://flintlib.org/doc/nmod_poly.html) of the FLINT manual, as of FLINT 3.6.0. Its
companions are [Malachite for FLINT Users: Integer
Polynomials](/mapping/flint-integer-polynomials/), [Malachite for FLINT Users: Rational
Polynomials](/mapping/flint-rational-polynomials/) and [Malachite for FLINT Users: Modular
Polynomials](/mapping/flint-modular-polynomials/); the
[conventions](/mapping/flint-integers/#conventions) of [the fmpz page](/mapping/flint-integers/)
apply here too, and the [mapping index](/mapping/) lists the whole family.

The page covers all 42 sections of `nmod_poly.h` and maps every documented public function, 204
in all, as 32 ✓, 4 ⚙, 21 ≈, 18 — and 129 ✗. Five underscore-level functions are shown as well (the
[Conway polynomial](#special-polynomials) lookup and the [subproduct tree](#subproduct-trees)),
along with four type rows, for 213 rows in all; the other 154 documented underscore functions are
omitted.

## Conventions {#conventions}

### Where the modulus lives, and FLINT answers twice {#modulus}

`nmod_poly_struct` stores its modulus as an `nmod_t` field, so no function in the chapter takes a
context argument and `nmod_poly_init` takes an `n`. FLINT never checks that the moduli of its
operands agree: [`add`](#addition-and-subtraction) uses the first input's modulus,
[`scalar_addmul`](#scalar) and [`compose_series`](#power-series-composition) use the output's,
[`equal`](#comparison) ignores it, [`swap`](#assignment) carries it with the value and
[`set`](#assignment) leaves it behind. In Malachite the modulus is an argument written at each
call, and a polynomial is only its list of coefficients, so a mismatch is not expressible.

### One FLINT chapter, six Malachite types {#word-sized}

`nmod_poly_t` has one coefficient type, `ulong`, a 64-bit word.
[`UnsignedPolynomial<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
is generic over `T: PrimitiveUnsigned`
([`u8`](https://doc.rust-lang.org/nightly/core/primitive.u8.html),
[`u16`](https://doc.rust-lang.org/nightly/core/primitive.u16.html),
[`u32`](https://doc.rust-lang.org/nightly/core/primitive.u32.html),
[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html),
[`u128`](https://doc.rust-lang.org/nightly/core/primitive.u128.html),
[`usize`](https://doc.rust-lang.org/nightly/core/primitive.usize.html)), and this chapter is the
`T = u64` instance; every bound stated in terms of `FLINT_BITS` becomes a bound in terms of
`T::WIDTH`.

### No `Natural` below this line {#no-natural}

`malachite-base` sits below `malachite-nz`, so
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
never uses
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), and
anything that needs a multi-word value — the `fmpz` exponents of [powering](#powering), Kronecker
[packing](#bit-packing), and lifting to $$\mathbb{Z}$$ — has no counterpart in this crate.

### `nmod_t` is Malachite's precomputed data {#precomputed}

`nmod_t` holds the modulus `n` and the derived `norm` and `ninv`. The `Data` of Malachite's
[`ModPowPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowPrecomputed.html)
for [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) is the same pair, so the
chapter's `_preinv` suffix maps onto the
[`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html),
[`ModPowPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowPrecomputed.html)
and
[`ModSquarePrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquarePrecomputed.html)
family rather than onto anything stored. `nmod_poly_init_preinv` has no counterpart because a
Malachite polynomial has nowhere to keep an inverse.

### Reduced arguments are checked {#reduced-check}

Every Malachite modular function asserts that its arguments are already reduced, with a
diagnostic naming the offender and a `# Panics` line saying so; the exceptions are the operations
whose purpose is reduction,
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html),
[`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html)
and the
[`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModIsReduced.html)
predicates. FLINT assumes reducedness and does not check it. Its public setters do reduce
(`nmod_poly_set_coeff_ui(p, 0, 100)` with $$n = 7$$ stores 2), but the underscore layer, the
undocumented `nmod_poly_set_mod`, [`set_str`](#input-and-output), [`set`](#assignment) and
[mismatched moduli](#modulus) all produce unreduced values without error.

### Why `UnsignedPolynomial` is the target {#why-unsigned}

A coefficient of an `nmod_poly_t` is a `ulong` in $$[0, n)$$, an unsigned primitive integer, so
the polynomial over it is
[`UnsignedPolynomial<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
rather than a polynomial over an arbitrary-precision type. Its arithmetic is modular only: there
are no `Checked`, `Wrapping`, `Saturating` or `Overflowing` families.

### Degenerate moduli {#degenerate}

$$n = 1$$: `nmod_poly_one` produces a polynomial of length 0, and `is_one` and `is_zero` are both
true for it; Malachite accepts a modulus of 1, with only the zero polynomial reduced modulo it.
$$n = 0$$: FLINT accepts it and then does arithmetic without reducing, so a coefficient can wrap.
In Malachite `m = 0` fails the reducedness check of every modular function; for $$\mathbb{Z}$$ use
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html).

### The modulus is often required to be prime {#primality}

The chapter preamble says that gcds, modular inverses, division as if over a field, resultants
and discriminants, factorisation, irreducibility testing, root finding, square roots and the
transcendental series require a prime modulus, "assumed and not checked". For a composite
modulus most of those functions abort on the first non-invertible element, and a few answer
wrongly with no error. Unlike `fmpz_mod_poly.h`, this chapter has no `_f` family returning a
factor of the modulus.

## [Simple example](https://flintlib.org/doc/nmod_poly.html#simple-example) {#simple-example}

The chapter squares $$5x^3 + 6$$ in $$\mathbb{Z}/7\mathbb{Z}[x]$$, the same example as the
`fmpz_mod_poly` chapter's, with the output:

```
4 7  6 0 0 5
7 7  1 0 0 4 0 0 4
```

Each line is the length, the modulus, two spaces, and the coefficients in ascending order.

## [Types, macros and constants](https://flintlib.org/doc/nmod_poly.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `nmod_poly_struct` | [`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html) |
| — | `nmod_poly_t` | |

The `_t` form is an array of length 1, so that a polynomial is passed and mutated through a
pointer; `&` and `&mut` serve. The `struct` row is ≈ rather than ✓ because
[`UnsignedPolynomial<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
is a `Vec<T>` while `nmod_poly_struct` is that plus a modulus and two derived quantities. For
`nmod_t`, from [the `nmod` module](https://flintlib.org/doc/nmod.html), see
[above](#precomputed).

## [Memory management](https://flintlib.org/doc/nmod_poly.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_poly_init (nmod_poly_t poly, ulong n)` | [`UnsignedPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void nmod_poly_init_preinv (nmod_poly_t poly, ulong n, ulong ninv)` | |
| — | `void nmod_poly_init_mod (nmod_poly_t poly, const nmod_t mod)` | |
| — | `void nmod_poly_init2 (nmod_poly_t poly, ulong n, slong alloc)` | |
| — | `void nmod_poly_init2_preinv (nmod_poly_t poly, ulong n, ulong ninv, slong alloc)` | |
| — | `void nmod_poly_realloc (nmod_poly_t poly, slong alloc)` | |
| — | `void nmod_poly_clear (nmod_poly_t poly)` | |
| — | `void nmod_poly_fit_length (nmod_poly_t poly, slong alloc)` | |

`init` is
[`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html)
or [`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html) (≈ because it
also stores a modulus), `clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html), and `init2`, `realloc` and
`fit_length` are capacity management that
[`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html) performs on its own.
`init_preinv`, `init2_preinv` and `init_mod` supply a precomputed inverse, which a Malachite
polynomial has nowhere to store; the [`Precomputed` traits'](#precomputed) data is passed to the
calls that use it, and is obtained from
[`precompute_mod_mul_data`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html#tymethod.precompute_mod_mul_data)
rather than supplied by the caller.

## [Polynomial properties](https://flintlib.org/doc/nmod_poly.html#polynomial-properties) {#polynomial-properties}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong nmod_poly_length (const nmod_poly_t poly)` | [`len`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.len) |
| ≈ | `slong nmod_poly_degree (const nmod_poly_t poly)` | [`degree`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.degree) |
| — | `ulong nmod_poly_modulus (const nmod_poly_t poly)` | |
| ✓ | `flint_bitcnt_t nmod_poly_max_bits (const nmod_poly_t poly)` | [`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits) |
| ✗ | `int nmod_poly_is_unit (const nmod_poly_t poly)` | |
| ✓ | `int nmod_poly_is_monic (const nmod_poly_t poly)` | [`is_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Polynomial.html#tymethod.is_monic) |

FLINT reports the zero polynomial's degree as $$-1$$, while Malachite's
[`degree`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.degree)
returns
[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)`<`[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html)`>`
and gives [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None)
for it.
[`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits)
is the bit count of the largest coefficient, 0 for the zero polynomial. `modulus` is — because a
Malachite caller already holds `m`.

`is_unit` returns whether the polynomial is a nonzero constant, which is the unit test only for a
prime modulus. Malachite's
[`IsUnit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.IsUnit.html)
for `UnsignedPolynomial` takes no modulus and is true only for the polynomial 1, so it is not a
counterpart.

## [Assignment and basic manipulation](https://flintlib.org/doc/nmod_poly.html#assignment-and-basic-manipulation) {#assignment}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_poly_set (nmod_poly_t a, const nmod_poly_t b)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void nmod_poly_swap (nmod_poly_t poly1, nmod_poly_t poly2)` | [`swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| ✓ | `void nmod_poly_zero (nmod_poly_t res)` | [`UnsignedPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void nmod_poly_truncate (nmod_poly_t poly, slong len)` | [`truncate_assign`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.truncate_assign) |
| ✓ | `void nmod_poly_set_trunc (nmod_poly_t res, const nmod_poly_t poly, slong len)` | [`truncate`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.truncate) |
| ✓ | `void nmod_poly_reverse (nmod_poly_t output, const nmod_poly_t input, slong m)` | [`reverse`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.reverse) |

`set` is [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html), with
[`clone_from`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from)
to reuse an allocation. The truncation names are swapped: FLINT's in-place `truncate` is
[`truncate_assign`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.truncate_assign)
and `set_trunc` is
[`truncate`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.truncate).
`reverse` has an in-place form,
[`reverse_assign`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.reverse_assign).

## [Randomization](https://flintlib.org/doc/nmod_poly.html#randomization) {#randomization}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_poly_randtest (nmod_poly_t poly, flint_rand_t state, slong len)` | [`random_unsigned_polynomials_reduced_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/random/fn.random_unsigned_polynomials_reduced_mod.html), [`striped_random_unsigned_polynomials_reduced_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/random/fn.striped_random_unsigned_polynomials_reduced_mod.html) |
| ✗ | `void nmod_poly_randtest_monic (nmod_poly_t poly, flint_rand_t state, slong len)` | |
| ✗ | `void nmod_poly_randtest_trinomial (nmod_poly_t poly, flint_rand_t state, slong len)` | |
| ✗ | `void nmod_poly_randtest_pentomial (nmod_poly_t poly, flint_rand_t state, slong len)` | |

`randtest` produces a test input rather than a uniform sample: length up to `len`, sparse half
the time, and sometimes the zero polynomial. Malachite's
[`random_unsigned_polynomials_reduced_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/random/fn.random_unsigned_polynomials_reduced_mod.html)
takes a mean length as a numerator and denominator with no upper bound, and
[`striped_random_unsigned_polynomials_reduced_mod`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/random/fn.striped_random_unsigned_polynomials_reduced_mod.html)
produces long runs of equal bits; both require `m >= 2`. Malachite has no degree-constrained
generators.

## [Construction of irreducible polynomials](https://flintlib.org/doc/nmod_poly.html#construction-of-irreducible-polynomials) {#irreducible}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_minimal_irreducible (nmod_poly_t res, ulong n)` | |
| ✗ | `void nmod_poly_randtest_irreducible (nmod_poly_t poly, flint_rand_t state, slong len)` | |
| ✗ | `void nmod_poly_randtest_monic_irreducible (nmod_poly_t poly, flint_rand_t state, slong len)` | |
| ✗ | `void nmod_poly_randtest_monic_primitive (nmod_poly_t poly, flint_rand_t state, slong len)` | |
| ✗ | `int nmod_poly_randtest_trinomial_irreducible (nmod_poly_t poly, flint_rand_t state, slong len, slong max_attempts)` | |
| ✗ | `int nmod_poly_randtest_pentomial_irreducible (nmod_poly_t poly, flint_rand_t state, slong len, slong max_attempts)` | |
| ✗ | `void nmod_poly_randtest_sparse_irreducible (nmod_poly_t poly, flint_rand_t state, slong len)` | |

Nothing in `malachite-base` factors a polynomial or tests one for irreducibility.

## [Getting and setting coefficients](https://flintlib.org/doc/nmod_poly.html#getting-and-setting-coefficients) {#coefficients}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `ulong nmod_poly_get_coeff_ui (const nmod_poly_t poly, slong j)` | [`coefficient`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.coefficient) |
| ≈ | `void nmod_poly_set_coeff_ui (nmod_poly_t poly, slong j, ulong c)` | [`mutate_coefficient`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.mutate_coefficient) |

`set_coeff_ui` is ≈ because
[`mutate_coefficient`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.mutate_coefficient)
hands a closure a `&mut T` and trims afterwards, and because FLINT reduces `c` modulo the stored
modulus while Malachite cannot: the equivalent call is `p.mutate_coefficient(j, |c| *c = x % m)`.

## [Input and output](https://flintlib.org/doc/nmod_poly.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `char * nmod_poly_get_str (const nmod_poly_t poly)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ✓ | `char * nmod_poly_get_str_pretty (const nmod_poly_t poly, const char * x)` | [`to_string_with`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.to_string_with) |
| ≈ | `int nmod_poly_set_str (nmod_poly_t poly, const char * s)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ✓ | `int nmod_poly_print_pretty (const nmod_poly_t a, const char * x)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`to_string_with`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.to_string_with) |
| ≈ | `int nmod_poly_print (const nmod_poly_t a)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ✓ | `int nmod_poly_fprint_pretty (FILE * f, const nmod_poly_t poly, const char * x)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int nmod_poly_fprint (FILE * f, const nmod_poly_t poly)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int nmod_poly_fread (FILE * f, nmod_poly_t poly)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ≈ | `int nmod_poly_read (nmod_poly_t poly)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |

The format is the length, the modulus, two spaces, and the coefficients in ascending order; the
zero polynomial prints as `0 7`. The `_pretty` writers are
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) and
[`to_string_with`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.to_string_with).
The plain writers and the three readers are ≈ because
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
serializes as a transparent list of its coefficients with no modulus, the modulus belonging to
the operation; `set_str` reads the modulus field and discards it, and does not reduce the
coefficients. Malachite's deserialization rejects a trailing zero coefficient and does not check
reduction. `get_str_pretty` accepts an empty variable name; a Malachite
[`VarScheme`](https://docs.rs/malachite-base/latest/malachite_base/vars/trait.VarScheme.html)
rejects empty, reserved and duplicate names.

## [Comparison](https://flintlib.org/doc/nmod_poly.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int nmod_poly_equal (const nmod_poly_t a, const nmod_poly_t b)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int nmod_poly_equal_nmod (const nmod_poly_t poly, ulong cst)` | `p == c` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ≈ | `int nmod_poly_equal_ui (const nmod_poly_t poly, ulong cst)` | `p == c` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ✓ | `int nmod_poly_equal_trunc (const nmod_poly_t poly1, const nmod_poly_t poly2, slong n)` | [`eq_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.EqTruncated.html#tymethod.eq_truncated) |
| ✓ | `int nmod_poly_is_zero (const nmod_poly_t poly)` | `p == 0` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ≈ | `int nmod_poly_is_one (const nmod_poly_t poly)` | `p == 1` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ≈ | `int nmod_poly_is_gen (const nmod_poly_t poly)` | `== `[`x`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html#method.x)`()` |

`equal` never reads the moduli, like
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html). A polynomial
compares with a value of its coefficient type: `p == c` holds exactly when `p` is the constant
polynomial `c`, so an unreduced `c` compares unequal, as with `equal_nmod`. `equal_ui` reduces
`cst` first, so it is `p == c % n`, hence ≈. Modulo 1, `is_one` and `is_gen` return true for the
zero polynomial, hence ≈.

## [Shifting](https://flintlib.org/doc/nmod_poly.html#shifting) {#shifting}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_poly_shift_left (nmod_poly_t res, const nmod_poly_t poly, slong k)` | [`mul_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulPowerOfX.html#tymethod.mul_power_of_x) |
| ✓ | `void nmod_poly_shift_right (nmod_poly_t res, const nmod_poly_t poly, slong k)` | [`div_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DivPowerOfX.html#tymethod.div_power_of_x) |

[`ModShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShl.html)
and
[`ModShr`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShr.html)
are not counterparts: they mean scaling by $$2^k$$, not moving coefficients.

## [Addition and subtraction](https://flintlib.org/doc/nmod_poly.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_poly_add (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | [`mod_add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html#tymethod.mod_add) |
| ✓ | `void nmod_poly_add_series (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong n)` | [`mod_add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModAddTruncated.html#tymethod.mod_add_truncated) |
| ✓ | `void nmod_poly_sub (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | [`mod_sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html#tymethod.mod_sub) |
| ✓ | `void nmod_poly_sub_series (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong n)` | [`mod_sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModSubTruncated.html#tymethod.mod_sub_truncated) |
| ✓ | `void nmod_poly_neg (nmod_poly_t res, const nmod_poly_t poly)` | [`mod_neg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html#tymethod.mod_neg) |

For a modulus that is a power of 2, the counterparts are
[`mod_power_of_2_add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html#tymethod.mod_power_of_2_add),
[`mod_power_of_2_sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Sub.html#tymethod.mod_power_of_2_sub),
[`mod_power_of_2_neg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Neg.html#tymethod.mod_power_of_2_neg),
[`mod_power_of_2_add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2AddTruncated.html#tymethod.mod_power_of_2_add_truncated)
and
[`mod_power_of_2_sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2SubTruncated.html#tymethod.mod_power_of_2_sub_truncated).
The truncated forms check that the whole of each operand is reduced, not just the part below the
truncation length.

## [Scalar multiplication and division](https://flintlib.org/doc/nmod_poly.html#scalar-multiplication-and-division) {#scalar}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_scalar_mul_nmod (nmod_poly_t res, const nmod_poly_t poly, ulong c)` | |
| ✗ | `void nmod_poly_scalar_addmul_nmod (nmod_poly_t res, const nmod_poly_t poly, ulong c)` | |
| ≈ | `void nmod_poly_make_monic (nmod_poly_t res, const nmod_poly_t poly)` | [`mod_make_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMakeMonic.html#tymethod.mod_make_monic) |

There is no polynomial-by-scalar multiplication; `scalar_mul_nmod(res, p, c)` can be written
`p.mod_mul(UnsignedPolynomial::from(c), m)` with
[`mod_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html#tymethod.mod_mul),
and division by a scalar as multiplication by its
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html).
`make_monic` aborts on the zero polynomial and on a non-invertible leading coefficient.
[`mod_make_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMakeMonic.html#tymethod.mod_make_monic)
returns the zero polynomial unchanged and, for a non-invertible leading coefficient, returns its
gcd with the modulus, a nontrivial factor, as an error, as `fmpz_mod_poly_make_monic_f` does;
hence ≈.

## [Bit packing and unpacking](https://flintlib.org/doc/nmod_poly.html#bit-packing-and-unpacking) {#bit-packing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_poly_bit_pack (fmpz_t f, const nmod_poly_t poly, flint_bitcnt_t bit_size)` | [`Natural::from_power_of_2_digits_asc`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html#tymethod.from_power_of_2_digits_asc) on the coefficients |
| ≈ | `void nmod_poly_bit_unpack (nmod_poly_t poly, const fmpz_t f, flint_bitcnt_t bit_size)` | [`to_power_of_2_digits_asc`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html#tymethod.to_power_of_2_digits_asc), then [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |

The packed integer is an `fmpz`, so neither function can be a method of
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html);
from the `malachite-nz` side both directions are:

```rust
// pack
let f = Natural::from_power_of_2_digits_asc(bits, p.coefficients_asc().iter().copied()).unwrap();
// unpack
let p = UnsignedPolynomial::from_coefficients_asc(
    f.to_power_of_2_digits_asc(bits).into_iter().map(|c: u64| c % m).collect(),
);
```

[`PowerOf2Digits`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.PowerOf2Digits.html)
limits the field width to the digit type, so `u64` coefficients pack into fields of at most 64
bits. `nmod_poly_bit_pack` does not check that the coefficients fit `bit_size` and silently
produces a different integer if they overlap; `from_power_of_2_digits_asc` returns
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) instead.
`bit_unpack` reduces modulo the destination's modulus and aborts on a negative integer; in
Malachite the input is a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) and the
reduction is an explicit
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html).

## [KS2/KS4 Reduction](https://flintlib.org/doc/nmod_poly.html#ks2-ks4-reduction) {#ks2-ks4-reduction}

This section has no public functions and so no table; its six entries are underscore-level
internals of the KS2 and KS4 multiplication algorithms.

## [Multiplication](https://flintlib.org/doc/nmod_poly.html#multiplication) {#multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_poly_mul (nmod_poly_t res, const nmod_poly_t poly, const nmod_poly_t poly2)` | [`mod_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html#tymethod.mod_mul) |
| ✓ | `void nmod_poly_mullow (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong trunc)` | [`mod_mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMulTruncated.html#tymethod.mod_mul_truncated) |
| ✗ | `void nmod_poly_mulmid (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong nlo, slong nhi)` | |
| ✗ | `void nmod_poly_mulhigh (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong n)` | |
| ✗ | `void nmod_poly_mul_classical (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | |
| ✗ | `void nmod_poly_mullow_classical (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong trunc)` | |
| ✗ | `void nmod_poly_mulmid_classical (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong nlo, slong nhi)` | |
| ✗ | `void nmod_poly_mulhigh_classical (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong start)` | |
| ✗ | `void nmod_poly_mul_KS (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | |
| ✗ | `void nmod_poly_mullow_KS (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong n)` | |
| ✗ | `void nmod_poly_mulmid_KS (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong nlo, slong nhi)` | |
| ✗ | `void nmod_poly_mul_KS2 (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | |
| ✗ | `void nmod_poly_mul_KS4 (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | |
| ✗ | `void nmod_poly_mulmod (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, const nmod_poly_t f)` | |
| ✗ | `void nmod_poly_mulmod_preinv (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, const nmod_poly_t f, const nmod_poly_t finv)` | |

`nmod_poly_mulmid_classical` and `nmod_poly_mulmid_KS` are documented with the underscore
functions' signatures; the table uses the header's `nmod_poly_t` signatures, which are the ones
that compile. The algorithm-specific variants have no separate counterparts;
[`mod_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html#tymethod.mod_mul)
chooses its algorithm itself. FLINT has no `sqr` (`_nmod_poly_mul` detects squaring by pointer
identity); Malachite has
[`mod_square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquare.html#tymethod.mod_square)
and
[`mod_square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModSquareTruncated.html#tymethod.mod_square_truncated).
For a modulus that is a power of 2, the counterparts are
[`mod_power_of_2_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Mul.html#tymethod.mod_power_of_2_mul),
[`mod_power_of_2_mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2MulTruncated.html#tymethod.mod_power_of_2_mul_truncated),
[`mod_power_of_2_square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Square.html#tymethod.mod_power_of_2_square)
and
[`mod_power_of_2_square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2SquareTruncated.html#tymethod.mod_power_of_2_square_truncated).

## [Preconditioned modular multiplication](https://flintlib.org/doc/nmod_poly.html#preconditioned-modular-multiplication) {#precond}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `nmod_poly_mulmod_precond_struct` | |
| — | `nmod_poly_mulmod_precond_t` | |
| ✗ | `void nmod_poly_mulmod_precond_init_method (nmod_poly_mulmod_precond_t precond, const nmod_poly_t a, const nmod_poly_t d, const nmod_poly_t dinv, int method)` | |
| ✗ | `void nmod_poly_mulmod_precond_init_num (nmod_poly_mulmod_precond_t precond, const nmod_poly_t a, const nmod_poly_t d, const nmod_poly_t dinv, slong num)` | |
| — | `void nmod_poly_mulmod_precond_clear (nmod_poly_mulmod_precond_t precond)` | |
| ✗ | `void nmod_poly_mulmod_precond (nmod_poly_t res, const nmod_poly_mulmod_precond_t precond, const nmod_poly_t b)` | |

The `_t` row and `clear` are — for the reasons given under [types](#types-macros-and-constants)
and [memory management](#memory-management).

## [Powering](https://flintlib.org/doc/nmod_poly.html#powering) {#powering}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_poly_pow (nmod_poly_t res, const nmod_poly_t poly, ulong e)` | [`mod_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html#tymethod.mod_pow) |
| ⚙ | `void nmod_poly_pow_binexp (nmod_poly_t res, const nmod_poly_t poly, ulong e)` | [`mod_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html#tymethod.mod_pow) |
| ✓ | `void nmod_poly_pow_trunc (nmod_poly_t res, const nmod_poly_t poly, ulong e, slong trunc)` | [`mod_pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowTruncated.html#tymethod.mod_pow_truncated) |
| ⚙ | `void nmod_poly_pow_trunc_binexp (nmod_poly_t res, const nmod_poly_t poly, ulong e, slong trunc)` | [`mod_pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowTruncated.html#tymethod.mod_pow_truncated) |
| ✗ | `void nmod_poly_powmod_ui_binexp (nmod_poly_t res, const nmod_poly_t poly, ulong e, const nmod_poly_t f)` | |
| ✗ | `void nmod_poly_powmod_ui_binexp_preinv (nmod_poly_t res, const nmod_poly_t poly, ulong e, const nmod_poly_t f, const nmod_poly_t finv)` | |
| ✗ | `void nmod_poly_powmod_fmpz_binexp (nmod_poly_t res, const nmod_poly_t poly, fmpz_t e, const nmod_poly_t f)` | |
| ✗ | `void nmod_poly_powmod_fmpz_binexp_preinv (nmod_poly_t res, const nmod_poly_t poly, fmpz_t e, const nmod_poly_t f, const nmod_poly_t finv)` | |
| ✗ | `void nmod_poly_powmod_x_ui_preinv (nmod_poly_t res, ulong e, const nmod_poly_t f, const nmod_poly_t finv)` | |
| ✗ | `void nmod_poly_powmod_x_fmpz_preinv (nmod_poly_t res, fmpz_t e, const nmod_poly_t f, const nmod_poly_t finv)` | |
| ✗ | `void nmod_poly_powers_mod_naive (nmod_poly_struct * res, const nmod_poly_t f, slong n, const nmod_poly_t g)` | |
| ✗ | `void nmod_poly_powers_mod_bsgs (nmod_poly_struct * res, const nmod_poly_t f, slong n, const nmod_poly_t g)` | |

For a modulus that is a power of 2, the counterparts are
[`mod_power_of_2_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Pow.html#tymethod.mod_power_of_2_pow)
and
[`mod_power_of_2_pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2PowTruncated.html#tymethod.mod_power_of_2_pow_truncated).
The zeroth power of the zero polynomial differs: `pow` gives 1 but `pow_binexp` and `pow_trunc`
give 0, while all the Malachite functions give 1 (then reduced and truncated as usual), as
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
does for every numeric type. Malachite's exponent type is
[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html), so the `fmpz` exponents have no
counterpart.

## [Division](https://flintlib.org/doc/nmod_poly.html#division) {#division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_divrem (nmod_poly_t Q, nmod_poly_t R, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_divrem_basecase (nmod_poly_t Q, nmod_poly_t R, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_divrem_newton_n_preinv (nmod_poly_t Q, nmod_poly_t R, const nmod_poly_t A, const nmod_poly_t B, const nmod_poly_t Binv)` | |
| ✗ | `void nmod_poly_div (nmod_poly_t Q, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_div_newton_n_preinv (nmod_poly_t Q, const nmod_poly_t A, const nmod_poly_t B, const nmod_poly_t Binv)` | |
| ✗ | `void nmod_poly_rem (nmod_poly_t R, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_divexact (nmod_poly_t Q, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `ulong nmod_poly_div_root (nmod_poly_t Q, const nmod_poly_t A, ulong c)` | |
| ✗ | `void nmod_poly_inv_series (nmod_poly_t Qinv, const nmod_poly_t Q, slong n)` | |
| ✗ | `void nmod_poly_inv_series_basecase (nmod_poly_t Qinv, const nmod_poly_t Q, slong n)` | |
| ✗ | `void nmod_poly_inv_series_newton (nmod_poly_t Qinv, const nmod_poly_t Q, slong n)` | |
| ✗ | `void nmod_poly_div_series (nmod_poly_t Q, const nmod_poly_t A, const nmod_poly_t B, slong n)` | |
| ✗ | `void nmod_poly_div_series_basecase (nmod_poly_t Q, const nmod_poly_t A, const nmod_poly_t B, slong n)` | |

The remainder returned by `div_root` is
[`mod_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluate.html#tymethod.mod_evaluate)`(c, m)`.

## [Divisibility testing](https://flintlib.org/doc/nmod_poly.html#divisibility-testing) {#divisibility-testing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int nmod_poly_divides (nmod_poly_t Q, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `int nmod_poly_divides_classical (nmod_poly_t Q, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `ulong nmod_poly_remove (nmod_poly_t f, const nmod_poly_t p)` | |

`divides` returns whether $$B$$ divides $$A$$ and sets the quotient; it aborts when the leading
coefficient of $$B$$ is not a unit. `remove` divides the highest power of $$p$$ out of $$f$$ and
returns the exponent; it never returns when $$p$$ is a nonzero constant, and aborts when $$p$$ is
zero.

## [Derivative and integral](https://flintlib.org/doc/nmod_poly.html#derivative-and-integral) {#derivative}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_poly_derivative (nmod_poly_t x_prime, const nmod_poly_t x)` | [`mod_derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModDerivative.html#tymethod.mod_derivative) |
| ✓ | `void nmod_poly_integral (nmod_poly_t x_int, const nmod_poly_t x)` | [`mod_integral`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModIntegral.html#tymethod.mod_integral) |

For a modulus that is a power of 2, the counterparts are
[`mod_power_of_2_derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2Derivative.html#tymethod.mod_power_of_2_derivative)
and
[`mod_power_of_2_integral`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2Integral.html#tymethod.mod_power_of_2_integral).

The integral divides the coefficient of $$x^{k-1}$$ by $$k$$. FLINT requires every $$k$$ from 1 to
$$\deg x + 1$$ to be a unit modulo $$n$$ and aborts otherwise (the entries' "prime strictly larger
than the degree" is off by one and too strict).
[`mod_integral`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModIntegral.html#tymethod.mod_integral)
requires only the $$k$$ whose coefficients are nonzero to be units, and panics otherwise: modulo 8,
FLINT aborts on $$x^2$$ while Malachite returns $$3x^3$$. The integral of the zero polynomial is
zero for every modulus. `mod_power_of_2_integral` is defined whenever every nonzero coefficient
belongs to an even power of $$x$$, where FLINT can integrate nothing of degree 1 or more.

## [Evaluation](https://flintlib.org/doc/nmod_poly.html#evaluation) {#evaluation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `ulong nmod_poly_evaluate_nmod (const nmod_poly_t poly, ulong c)` | [`mod_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluate.html#tymethod.mod_evaluate) |
| ✗ | `void nmod_poly_evaluate_mat (nmod_mat_t dest, const nmod_poly_t poly, const nmod_mat_t c)` | |
| ✗ | `void nmod_poly_evaluate_mat_horner (nmod_mat_t dest, const nmod_poly_t poly, const nmod_mat_t c)` | |
| ✗ | `void nmod_poly_evaluate_mat_paterson_stockmeyer (nmod_mat_t dest, const nmod_poly_t poly, const nmod_mat_t c)` | |

`evaluate_nmod` requires `c` reduced and returns a wrong value otherwise;
[`mod_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluate.html#tymethod.mod_evaluate)
panics unless `c` and every coefficient are reduced. For a modulus that is a power of 2, the
counterpart is
[`mod_power_of_2_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2Evaluate.html#tymethod.mod_power_of_2_evaluate).
Malachite has no matrix type.

## [Multipoint evaluation](https://flintlib.org/doc/nmod_poly.html#multipoint-evaluation) {#multipoint-evaluation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void nmod_poly_evaluate_nmod_vec (nn_ptr ys, const nmod_poly_t poly, nn_srcptr xs, slong olen)` | [`mod_evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateMany.html#tymethod.mod_evaluate_many) |
| ⚙ | `void nmod_poly_evaluate_nmod_vec_iter (nn_ptr ys, const nmod_poly_t poly, nn_srcptr xs, slong olen)` | [`mod_evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateMany.html#tymethod.mod_evaluate_many) |
| ⚙ | `void nmod_poly_evaluate_nmod_vec_fast (nn_ptr ys, const nmod_poly_t poly, nn_srcptr xs, slong olen)` | [`mod_evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateMany.html#tymethod.mod_evaluate_many) |
| ≈ | `void nmod_poly_evaluate_geometric_nmod_vec_iter (nn_ptr ys, const nmod_poly_t poly, ulong r, slong olen)` | [`mod_evaluate_geometric`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateGeometric.html#tymethod.mod_evaluate_geometric) |
| ≈ | `void nmod_poly_evaluate_geometric_nmod_vec_fast (nn_ptr ys, const nmod_poly_t poly, ulong r, slong olen)` | [`mod_evaluate_geometric`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateGeometric.html#tymethod.mod_evaluate_geometric) |

FLINT's geometric functions evaluate at $$1, r^2, r^4, \dots$$, not at powers of $$r$$, while
[`mod_evaluate_geometric`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateGeometric.html#tymethod.mod_evaluate_geometric)
takes the ratio $$q$$ of the progression itself and evaluates at $$1, q, q^2, \dots$$; so
`geometric_nmod_vec_iter(r)` and `geometric_nmod_vec_fast(r)` are both
`mod_evaluate_geometric(r^2)`. FLINT's `_fast` version aborts unless $$r$$ is invertible;
`mod_evaluate_geometric` accepts any reduced ratio. In FLINT the points must be reduced, and one
unreduced point corrupts the answers at other points.

## [Interpolation](https://flintlib.org/doc/nmod_poly.html#interpolation) {#interpolation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_interpolate_nmod_vec (nmod_poly_t poly, nn_srcptr xs, nn_srcptr ys, slong len)` | |
| ✗ | `void nmod_poly_interpolate_nmod_vec_fast (nmod_poly_t poly, nn_srcptr xs, nn_srcptr ys, slong len)` | |
| ✗ | `void nmod_poly_interpolate_nmod_vec_newton (nmod_poly_t poly, nn_srcptr xs, nn_srcptr ys, slong len)` | |
| ✗ | `void nmod_poly_interpolate_nmod_vec_barycentric (nmod_poly_t poly, nn_srcptr xs, nn_srcptr ys, slong len)` | |
| ✗ | `void nmod_poly_interpolate_geometric_nmod_vec_fast (nmod_poly_t poly, ulong r, nn_srcptr ys, slong len)` | |
| ✗ | `void nmod_poly_interpolate_geometric_nmod_vec_fast_precomp (nmod_poly_t poly, nn_srcptr v, const nmod_geometric_progression_t G, slong len)` | |

All six return the polynomial of length at most `len` through the given values; the geometric
ones use the points $$1, r^2, \dots, r^{2(\text{len}-1)}$$ of
[multipoint evaluation](#multipoint-evaluation), which need $$r$$ to be a unit and the points to
be distinct. For a composite modulus every pairwise difference of the points must be a unit, not
just nonzero, and otherwise the functions abort.

## [Extrapolation](https://flintlib.org/doc/nmod_poly.html#extrapolation) {#extrapolation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_extrapolate_geometric (nn_ptr oval, slong olen, nn_srcptr ival, slong ilen, slong offset, ulong r, nmod_t mod)` | |
| ✗ | `void nmod_poly_extrapolate_geometric_precomp (nn_ptr oval, slong olen, nn_srcptr ival, slong ilen, slong offset, const nmod_geometric_progression_t G)` | |

Neither function takes a polynomial: given the values of an $$f$$ of degree below `ilen` at
consecutive points $$c \cdot r^{2i}$$ of a geometric progression, they return its values at
`olen` further consecutive points `offset` steps along, without computing $$f$$. The output range
must not overlap the input (`offset >= ilen` forward, `offset + olen <= 0` backward); this is not
checked, and an overlapping range gives wrong values silently.

## [Composition](https://flintlib.org/doc/nmod_poly.html#composition) {#composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_compose (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | |
| ✗ | `void nmod_poly_compose_horner (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | |
| ✗ | `void nmod_poly_compose_divconquer (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2)` | |

All three evaluate `poly1` at `poly2`. Under a composite modulus the result's degree can be less
than the product of the degrees ($$(x^2 + 1) \circ 2x = 1$$ modulo 4), and FLINT normalises it.
The same result is Horner's rule: starting from zero, for each coefficient of `poly1` from the
highest down,
[`mod_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html#tymethod.mod_mul)
by `poly2` and
[`mod_add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html#tymethod.mod_add)
the coefficient as `UnsignedPolynomial::from(c)`.

## [Taylor shift](https://flintlib.org/doc/nmod_poly.html#taylor-shift) {#taylor-shift}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_taylor_shift (nmod_poly_t g, const nmod_poly_t f, ulong c)` | |
| ✗ | `void nmod_poly_taylor_shift_horner (nmod_poly_t g, const nmod_poly_t f, ulong c)` | |
| ✗ | `void nmod_poly_taylor_shift_convolution (nmod_poly_t g, const nmod_poly_t f, ulong c)` | |

All three compute $$f(x + c)$$ and reduce `c` themselves. The Horner version has no precondition;
the convolution version needs every $$k < \operatorname{len}(f)$$ to be a unit and aborts
otherwise, so the default, which uses it for some lengths, can abort under a composite modulus.
The same result is the [composition](#composition) of $$f$$ with
`UnsignedPolynomial::from_coefficients_asc(vec![c, 1])`.

## [Modular composition](https://flintlib.org/doc/nmod_poly.html#modular-composition) {#modular-composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_compose_mod (nmod_poly_t res, const nmod_poly_t f, const nmod_poly_t g, const nmod_poly_t h)` | |
| ✗ | `void nmod_poly_compose_mod_horner (nmod_poly_t res, const nmod_poly_t f, const nmod_poly_t g, const nmod_poly_t h)` | |
| ✗ | `void nmod_poly_compose_mod_brent_kung (nmod_poly_t res, const nmod_poly_t f, const nmod_poly_t g, const nmod_poly_t h)` | |
| ✗ | `void nmod_poly_compose_mod_brent_kung_preinv (nmod_poly_t res, const nmod_poly_t f, const nmod_poly_t g, const nmod_poly_t h, const nmod_poly_t hinv)` | |
| ✗ | `void nmod_poly_precompute_matrix (nmod_mat_t A, const nmod_poly_t f, const nmod_poly_t g, const nmod_poly_t ginv)` | |
| ✗ | `void nmod_poly_compose_mod_brent_kung_precomp_preinv (nmod_poly_t res, const nmod_poly_t f, const nmod_mat_t A, const nmod_poly_t h, const nmod_poly_t hinv)` | |
| ✗ | `void nmod_poly_compose_mod_brent_kung_vec_preinv (nmod_poly_struct * res, const nmod_poly_struct * polys, slong len1, slong n, const nmod_poly_t g, const nmod_poly_t h, const nmod_poly_t hinv)` | |
| — | `void nmod_poly_compose_mod_brent_kung_vec_preinv_threaded (nmod_poly_struct * res, const nmod_poly_struct * polys, slong len1, slong n, const nmod_poly_t g, const nmod_poly_t poly, const nmod_poly_t polyinv)` | |
| — | `void nmod_poly_compose_mod_brent_kung_vec_preinv_threaded_pool (nmod_poly_struct * res, const nmod_poly_struct * polys, slong len1, slong n, const nmod_poly_t g, const nmod_poly_t poly, const nmod_poly_t polyinv, thread_pool_handle * threads, slong num_threads)` | |

The threaded rows are — because threading is not part of the operation.

## [Greatest common divisor](https://flintlib.org/doc/nmod_poly.html#greatest-common-divisor) {#gcd}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_gcd (nmod_poly_t G, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_gcd_euclidean (nmod_poly_t G, const nmod_poly_t A, const nmod_poly_t B)` | |
| — | `void nmod_poly_gcd_euclidean_redc_half (nmod_poly_t G, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_gcd_hgcd (nmod_poly_t G, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_xgcd (nmod_poly_t G, nmod_poly_t S, nmod_poly_t T, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_xgcd_euclidean (nmod_poly_t G, nmod_poly_t S, nmod_poly_t T, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `void nmod_poly_xgcd_hgcd (nmod_poly_t G, nmod_poly_t S, nmod_poly_t T, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `ulong nmod_poly_resultant (const nmod_poly_t f, const nmod_poly_t g)` | |
| ✗ | `ulong nmod_poly_resultant_euclidean (const nmod_poly_t f, const nmod_poly_t g)` | |
| ✗ | `ulong nmod_poly_resultant_hgcd (const nmod_poly_t f, const nmod_poly_t g)` | |
| ✗ | `void nmod_poly_gcdinv (nmod_poly_t G, nmod_poly_t S, const nmod_poly_t A, const nmod_poly_t B)` | |
| ✗ | `int nmod_poly_invmod (nmod_poly_t A, const nmod_poly_t B, const nmod_poly_t P)` | |

`gcd_euclidean_redc_half` is documented but absent from the header, which instead declares the
undocumented `gcd_euclidean_redc_fast`, hence —.

## [Discriminant](https://flintlib.org/doc/nmod_poly.html#discriminant) {#discriminant}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `ulong nmod_poly_discriminant (const nmod_poly_t f)` | |

The result is the discriminant $$\pm\operatorname{res}(f, f')/\operatorname{lc}(f)$$ reduced
modulo $$n$$, and 0 when $$f' = 0$$. For a composite modulus it can abort wherever
[the resultant](#gcd) does.

## [Power series composition](https://flintlib.org/doc/nmod_poly.html#power-series-composition) {#power-series-composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_compose_series (nmod_poly_t res, const nmod_poly_t poly1, const nmod_poly_t poly2, slong n)` | |

The result is `poly1(poly2)` truncated to length `n`. `poly2` must have zero constant term, and
FLINT aborts otherwise; any modulus works, and the output's modulus is the one used. The same
result is the Horner recipe under [composition](#composition) with
[`mod_mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMulTruncated.html#tymethod.mod_mul_truncated)
in place of `mod_mul`.

## [Power series reversion](https://flintlib.org/doc/nmod_poly.html#power-series-reversion) {#power-series-reversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_revert_series (nmod_poly_t Qinv, const nmod_poly_t Q, slong n)` | |

The result is the compositional inverse, $$Q(Q^{-1}(x)) = x \bmod x^n$$. It needs $$Q_0 = 0$$ and
$$Q_1$$ a unit; FLINT checks only $$Q_1 \ne 0$$, so a nonzero non-unit $$Q_1$$ aborts later with
an uninformative message.

## [Square roots](https://flintlib.org/doc/nmod_poly.html#square-roots) {#square-roots}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_sqrt_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_invsqrt_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `int nmod_poly_sqrt (nmod_poly_t s, const nmod_poly_t p)` | |

`sqrt` returns whether $$p$$ is a square and, if so, sets $$s$$ to a square root. The series
functions need 2 to be invertible (modulo 2 they abort) and the constant term of $$h$$ to be a
square, not necessarily 1. For a composite modulus `sqrt` can report a square as a non-square
($$6x^3 + x^2 = (3x^2 + x)^2$$ modulo 9) with no error.

## [Power sums](https://flintlib.org/doc/nmod_poly.html#power-sums) {#power-sums}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_power_sums (nmod_poly_t res, const nmod_poly_t poly, slong n)` | |
| ✗ | `void nmod_poly_power_sums_naive (nmod_poly_t res, const nmod_poly_t poly, slong n)` | |
| ✗ | `void nmod_poly_power_sums_schoenhage (nmod_poly_t res, const nmod_poly_t poly, slong n)` | |
| ✗ | `void nmod_poly_power_sums_to_poly (nmod_poly_t res, const nmod_poly_t Q)` | |
| ✗ | `void nmod_poly_power_sums_to_poly_naive (nmod_poly_t res, const nmod_poly_t Q)` | |
| ✗ | `void nmod_poly_power_sums_to_poly_schoenhage (nmod_poly_t res, const nmod_poly_t Q)` | |

The power sums of $$f$$ are $$p_k = \sum_i r_i^k$$ over its roots, returned as a series of length
`n` with $$p_0 = \deg f$$, and `to_poly` recovers the monic $$f$$ from them. Both need $$\deg f$$
below the modulus: `to_poly` divides by $$k$$ up to the degree and fails silently beyond it, and
`power_sums` (through `power_sums_naive`) can return wrong values once the degree reaches the
modulus. `power_sums` aborts on the zero polynomial.

## [Transcendental functions](https://flintlib.org/doc/nmod_poly.html#transcendental-functions) {#transcendental}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_log_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_exp_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_sin_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_cos_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_tan_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_sinh_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_cosh_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_tanh_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_atan_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_atanh_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_asin_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |
| ✗ | `void nmod_poly_asinh_series (nmod_poly_t g, const nmod_poly_t h, slong n)` | |

The logarithm needs $$h$$ to have constant term 1 and the others constant term 0, which FLINT
checks. The integers $$2, \dots, n - 1$$ must also be units modulo the modulus; this is not
checked, and a violation either aborts or succeeds when the missing inverse is never needed.

## [Special polynomials](https://flintlib.org/doc/nmod_poly.html#special-polynomials) {#special-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int _nmod_poly_conway (nn_ptr op, ulong prime, slong deg)` | |
| ✗ | `ulong _nmod_poly_conway_rand (slong * degree, flint_rand_t state, int type)` | |

Both rows are underscore-level and are included because a Conway polynomial is an object users
ask for by name; here the underscore means "raw coefficient array", not "internal".

## [Products](https://flintlib.org/doc/nmod_poly.html#products) {#products}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_poly_product_roots_nmod_vec (nmod_poly_t poly, nn_srcptr xs, slong n)` | |
| ✗ | `int nmod_poly_find_distinct_nonzero_roots (ulong * roots, const nmod_poly_t A)` | |

`product_roots_nmod_vec` builds $$\prod_i (x - x_i)$$ and needs the roots reduced, giving
coefficients outside $$[0, n)$$ otherwise; the same polynomial is the product, by
[`mod_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html#tymethod.mod_mul),
of the polynomials `UnsignedPolynomial::from_coefficients_asc(vec![x_i.mod_neg(m), 1])`.
`find_distinct_nonzero_roots` returns 1 and writes the roots when $$A$$ has $$\deg A$$ distinct
nonzero roots, and 0 otherwise; it returns 1 for the zero polynomial, and for a composite modulus
it returns 0 or aborts.

## [Subproduct trees](https://flintlib.org/doc/nmod_poly.html#subproduct-trees) {#subproduct-trees}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `nn_ptr * _nmod_poly_tree_alloc (slong len)` | |
| — | `void _nmod_poly_tree_free (nn_ptr * tree, slong len)` | |
| — | `void _nmod_poly_tree_build (nn_ptr * tree, nn_srcptr roots, slong len, nmod_t mod)` | |

The rows are — because allocating, freeing and filling a caller-allocated buffer is a memory
layout rather than an operation, which Rust ownership makes implicit.

## [Geometric progression](https://flintlib.org/doc/nmod_poly.html#geometric-progression) {#geometric-progression}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_geometric_progression_init (nmod_geometric_progression_t G, ulong r, slong len, nmod_t mod)` | |
| — | `void nmod_geometric_progression_clear (nmod_geometric_progression_t G)` | |

This is the precomputation behind the `_fast_precomp` forms of geometric
[evaluation](#multipoint-evaluation), [interpolation](#interpolation) and
[extrapolation](#extrapolation); `clear` is — because Rust drops the value.

## [Inflation and deflation](https://flintlib.org/doc/nmod_poly.html#inflation-and-deflation) {#inflation-and-deflation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void nmod_poly_inflate (nmod_poly_t result, const nmod_poly_t input, slong inflation)` | [`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x) |
| ≈ | `void nmod_poly_deflate (nmod_poly_t result, const nmod_poly_t input, slong deflation)` | [`deflate_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DeflatePowerOfX.html#tymethod.deflate_power_of_x) |
| ≈ | `slong nmod_poly_deflation (const nmod_poly_t input)` | [`exponent_gcd`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ExponentGcd.html#tymethod.exponent_gcd) |

`inflate(p, 0)` is $$p(1)$$ modulo $$n$$;
[`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x)
with 0 gives $$p(1)$$ as an unreduced sum, panicking if it overflows the coefficient type, hence
≈. `deflate` with an $$n$$ that does not divide the deflation drops terms and returns a
non-normalised polynomial;
[`deflate_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DeflatePowerOfX.html#tymethod.deflate_power_of_x)
panics instead, hence ≈.
[`exponent_gcd`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ExponentGcd.html#tymethod.exponent_gcd)
gives 0 for a nonzero constant, where `deflation` gives 1, hence ≈.

## [Chinese Remaindering](https://flintlib.org/doc/nmod_poly.html#chinese-remaindering) {#crt}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int nmod_poly_multi_crt (nmod_poly_t output, const nmod_poly_struct * moduli, const nmod_poly_struct * values, slong len)` | |
| ✗ | `int nmod_poly_multi_crt_precompute (nmod_poly_multi_crt_t CRT, const nmod_poly_struct * moduli, slong len)` | |
| — | `int nmod_poly_multi_crt_precompute_p (nmod_poly_multi_crt_t CRT, const nmod_poly_struct * const * moduli, slong len)` | |
| ✗ | `void nmod_poly_multi_crt_precomp (nmod_poly_t output, const nmod_poly_multi_crt_t CRT, const nmod_poly_struct * values)` | |
| — | `void nmod_poly_multi_crt_precomp_p (nmod_poly_t output, const nmod_poly_multi_crt_t CRT, const nmod_poly_struct * const * values)` | |
| — | `void nmod_poly_multi_crt_init (nmod_poly_multi_crt_t CRT)` | |
| — | `void nmod_poly_multi_crt_clear (nmod_poly_multi_crt_t CRT)` | |

The `_p` variants only take an array of pointers, so they are —, and so are `init` and `clear`.

## [Berlekamp–Massey Algorithm](https://flintlib.org/doc/nmod_poly.html#berlekamp-massey-algorithm) {#berlekamp-massey}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void nmod_berlekamp_massey_init (nmod_berlekamp_massey_t B, ulong p)` | |
| — | `void nmod_berlekamp_massey_clear (nmod_berlekamp_massey_t B)` | |
| ✗ | `void nmod_berlekamp_massey_start_over (nmod_berlekamp_massey_t B)` | |
| ✗ | `void nmod_berlekamp_massey_set_prime (nmod_berlekamp_massey_t B, ulong p)` | |
| ✗ | `void nmod_berlekamp_massey_add_points (nmod_berlekamp_massey_t B, const ulong * a, slong count)` | |
| ✗ | `void nmod_berlekamp_massey_add_zeros (nmod_berlekamp_massey_t B, slong count)` | |
| ✗ | `void nmod_berlekamp_massey_add_point (nmod_berlekamp_massey_t B, ulong a)` | |
| ✗ | `int nmod_berlekamp_massey_reduce (nmod_berlekamp_massey_t B)` | |
| ✗ | `slong nmod_berlekamp_massey_point_count (const nmod_berlekamp_massey_t B)` | |
| ✗ | `const ulong * nmod_berlekamp_massey_points (const nmod_berlekamp_massey_t B)` | |
| ✗ | `const nmod_poly_struct * nmod_berlekamp_massey_V_poly (const nmod_berlekamp_massey_t B)` | |
| ✗ | `const nmod_poly_struct * nmod_berlekamp_massey_R_poly (const nmod_berlekamp_massey_t B)` | |

The prefix is `nmod_berlekamp_massey_`, not `nmod_poly_`; `clear` is — because Rust drops the
value.
