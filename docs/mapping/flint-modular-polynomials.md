---
layout: default
title: "Malachite for FLINT Users: Modular Polynomials"
permalink: /mapping/flint-modular-polynomials/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Modular Polynomials

This page maps the functions of FLINT's modular polynomial type, `fmpz_mod_poly_t` — polynomials
over $$\mathbb{Z}/n\mathbb{Z}$$ for a fixed modulus $$n$$ — onto their Malachite counterpart:
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html),
from the `malachite-nz` crate. It follows the organization of the [fmpz_mod_poly.h
chapter](https://flintlib.org/doc/fmpz_mod_poly.html) of the FLINT manual, as of FLINT 3.6.0. Its
companions are [Malachite for FLINT Users: Integer
Polynomials](/mapping/flint-integer-polynomials/) and [Malachite for FLINT Users: Rational
Polynomials](/mapping/flint-rational-polynomials/); the
[conventions](/mapping/flint-integers/#conventions) of [the fmpz page](/mapping/flint-integers/)
apply here too, and the [mapping index](/mapping/) lists the whole family.

## Conventions {#conventions}

### Where the modulus lives {#modulus}

`fmpz_mod_poly_struct` has no modulus field; the modulus lives in an `fmpz_mod_ctx_t` that almost
every function takes as a trailing argument. Malachite makes the same split without a context
type: a modular polynomial is a
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
and the modulus is an argument to each modular function, as it is for
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) with
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html)
and its relatives. A binary operation takes one modulus that applies to both operands.

### Two moduli, and both are arguments {#two-moduli}

FLINT's arithmetic modulo a polynomial $$f$$, in $$(\mathbb{Z}/n\mathbb{Z})[x]/(f)$$ — `mulmod`,
the `powmod` families, `invmod`, `gcdinv` and modular composition — has no Malachite counterpart.
[`ModMul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html)
and the other `Mod*` traits on
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
take a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
modulus and reduce coefficients only.

### Reduced arguments are checked {#reduced-check}

FLINT expects all inputs to be normalised and reduced modulo $$n$$ and does not check; unreduced
input produces a wrong answer. Every Malachite modular function checks that its arguments are
reduced and panics otherwise; the only exceptions are operations whose purpose is reduction:
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html),
[`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html)
and the
[`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModIsReduced.html)
predicates.

### The context does two jobs {#context}

`fmpz_mod_ctx_struct` holds the modulus and precomputed data. In Malachite the modulus is a plain
argument, and precomputation exists only for scalar
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
arithmetic, as the `Data` of
[`ModMulPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMulPrecomputed.html),
[`ModPowPrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowPrecomputed.html)
and
[`ModSquarePrecomputed`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquarePrecomputed.html).

### Why `NaturalPolynomial` is the target {#why-natural}

Each FLINT coefficient is reduced into $$[0,n)$$, so the counterpart is a polynomial over
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) rather
than over
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).

### The word-sized sibling {#nmod}

FLINT's word-sized `nmod_poly_t` corresponds to
[`UnsignedPolynomial<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
in `malachite-base`; see
[Malachite for FLINT Users: Word-Sized Modular Polynomials](/mapping/flint-word-sized-modular-polynomials/).

### The modulus is often required to be prime {#primality}

FLINT requires, without checking, a prime modulus for "greatest common divisors and extended gcds,
modular inverses, minimal polynomials, division as if over a field, square roots, factorisation
and irreducibility testing"; for composite $$n$$ these fail or silently compute something else.

## [Simple example](https://flintlib.org/doc/fmpz_mod_poly.html#simple-example) {#simple-example}

The chapter's worked example squares $$5x^3 + 6$$ in $$\mathbb{Z}/7\mathbb{Z}[x]$$ and prints:

```
4 7  6 0 0 5
7 7  1 0 0 4 0 0 4
```

The second field is the modulus; a
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) of a
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
shows only the polynomial.

## [Types, macros and constants](https://flintlib.org/doc/fmpz_mod_poly.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpz_mod_poly_struct` | [`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html) |
| — | `fmpz_mod_poly_t` | |

The `_t` form exists for passing by pointer, which `&` and `&mut` provide, so only the `struct`
gets a counterpart. `fmpz_mod_ctx_t`, documented in `fmpz_mod.h`, becomes a plain
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) modulus.

## [Memory management](https://flintlib.org/doc/fmpz_mod_poly.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_init (fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`NaturalPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void fmpz_mod_poly_init2 (fmpz_mod_poly_t poly, slong alloc, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_clear (fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_realloc (fmpz_mod_poly_t poly, slong alloc, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_fit_length (fmpz_mod_poly_t poly, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✓ | `void fmpz_mod_poly_truncate (fmpz_mod_poly_t poly, slong len, const fmpz_mod_ctx_t ctx)` | [`truncate_assign`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.truncate_assign) |
| ✓ | `void fmpz_mod_poly_set_trunc (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, slong n, const fmpz_mod_ctx_t ctx)` | [`truncate`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.truncate) |

`init` is
[`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) or
[`Default`](https://doc.rust-lang.org/nightly/core/default/trait.Default.html), `clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html), and `init2`, `realloc` and
`fit_length` are capacity management that
[`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html) performs on its own. `truncate`
is
[`truncate_assign`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.truncate_assign)
and `set_trunc` is
[`truncate`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.truncate);
both libraries normalise the result, and no modulus is needed.

## [Randomisation](https://flintlib.org/doc/fmpz_mod_poly.html#randomisation) {#randomisation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_poly_randtest (fmpz_mod_poly_t f, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | [`random_natural_polynomials_reduced_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/random/fn.random_natural_polynomials_reduced_mod.html), [`striped_random_natural_polynomials_reduced_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/random/fn.striped_random_natural_polynomials_reduced_mod.html) |
| ✗ | `void fmpz_mod_poly_randtest_not_zero (fmpz_mod_poly_t f, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_randtest_monic (fmpz_mod_poly_t poly, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_randtest_irreducible (fmpz_mod_poly_t f, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_randtest_monic_irreducible (fmpz_mod_poly_t poly, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_randtest_monic_primitive (fmpz_mod_poly_t poly, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_randtest_trinomial (fmpz_mod_poly_t poly, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_poly_randtest_trinomial_irreducible (fmpz_mod_poly_t poly, flint_rand_t state, slong len, slong max_attempts, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_randtest_pentomial (fmpz_mod_poly_t poly, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_poly_randtest_pentomial_irreducible (fmpz_mod_poly_t poly, flint_rand_t state, slong len, slong max_attempts, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_randtest_sparse_irreducible (fmpz_mod_poly_t poly, flint_rand_t state, slong len, const fmpz_mod_ctx_t ctx)` | |

[`random_natural_polynomials_reduced_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/random/fn.random_natural_polynomials_reduced_mod.html)
takes the modulus and a mean length with unbounded support, where FLINT bounds the length, hence
≈; its
[striped](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/random/fn.striped_random_natural_polynomials_reduced_mod.html)
sibling biases the bit patterns, and
[`exhaustive_natural_polynomials_reduced_mod`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/exhaustive/fn.exhaustive_natural_polynomials_reduced_mod.html)
enumerates all $$n^{d+1}$$ polynomials of degree at most $$d$$. The generators panic unless the
modulus is at least 2. There are no counterparts generating only nonzero, monic, or irreducible
polynomials.

## [Attributes](https://flintlib.org/doc/fmpz_mod_poly.html#attributes) {#attributes}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong fmpz_mod_poly_degree (const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`degree`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.degree) |
| ✓ | `slong fmpz_mod_poly_length (const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`len`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.len) |
| ≈ | `fmpz * fmpz_mod_poly_lead (const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`leading_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.leading_coefficient) |

`length` is the coefficient-vector length on both sides. `degree` returns `length - 1`, so $$-1$$
for the zero polynomial, where Malachite's
[`degree`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.degree)
returns [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None).
`lead` returns a writable `fmpz *`, or `NULL` for the zero polynomial;
[`leading_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.leading_coefficient)
returns a shared
`&`[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) that
is zero for the zero polynomial, and writes go through
[`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.mutate_coefficient),
which re-trims when the closure returns; a value $$\geq n$$ written there is not caught until the
next modular operation panics.

## [Assignment and basic manipulation](https://flintlib.org/doc/fmpz_mod_poly.html#assignment-and-basic-manipulation) {#assignment}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_set (fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpz_mod_poly_swap (fmpz_mod_poly_t poly1, fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx)` | [`swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| ✓ | `void fmpz_mod_poly_zero (fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`NaturalPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ≈ | `void fmpz_mod_poly_one (fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`NaturalPolynomial::one`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.one) |
| ✓ | `void fmpz_mod_poly_zero_coeffs (fmpz_mod_poly_t poly, slong i, slong j, const fmpz_mod_ctx_t ctx)` | [`zero_coefficients`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.zero_coefficients) |
| ✓ | `void fmpz_mod_poly_reverse (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, slong n, const fmpz_mod_ctx_t ctx)` | [`reverse`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.reverse) |

`set` is [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) (with
[`clone_from`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html#method.clone_from) to
reuse an allocation), `swap` is
[`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html), and `zero` assigns
[`ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html).
FLINT's `one` consults the modulus and returns the zero polynomial when $$n = 1$$;
[`NaturalPolynomial::one`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.one)
gives the constant 1 unconditionally, which is not reduced modulo 1, hence ≈; [the
reduced-argument check](#reduced-check) rejects it at the first modular operation. `zero_coeffs`
is
[`zero_coefficients`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.zero_coefficients).
`reverse` is
[`reverse`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.reverse)
(or [`reverse_assign`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.reverse_assign)
in place); both libraries truncate or zero-pad to length $$n$$, reverse, then normalise, so the
result may be shorter than $$n$$. None of these needs a modulus.

Functions declared in `fmpz_mod_poly.h` but not documented in the chapter, such as
`fmpz_mod_poly_gen`, `is_monic`, `is_unit`, `is_canonical`, `hamming_weight`, `set_coeff_si` and
several `add_fmpz` / `sub_si` variants, have no rows here.

## [Conversion](https://flintlib.org/doc/fmpz_mod_poly.html#conversion) {#conversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_poly_set_ui (fmpz_mod_poly_t f, ulong c, const fmpz_mod_ctx_t ctx)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ≈ | `void fmpz_mod_poly_set_fmpz (fmpz_mod_poly_t f, const fmpz_t c, const fmpz_mod_ctx_t ctx)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpz_mod_poly_set_fmpz_poly (fmpz_mod_poly_t f, const fmpz_poly_t g, const fmpz_mod_ctx_t ctx)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_mod_poly_get_fmpz_poly (fmpz_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_ctx_t ctx)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<`[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)`>` |
| ✓ | `void fmpz_mod_poly_get_nmod_poly (nmod_poly_t f, const fmpz_mod_poly_t g)` | [`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)`<&`[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)`>` |
| ✓ | `void fmpz_mod_poly_set_nmod_poly (fmpz_mod_poly_t f, const nmod_poly_t g)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<`[`UnsignedPolynomial<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)`>` |

`set_ui` and `set_fmpz` reduce the constant and
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) does not, so the faithful
spelling is `NaturalPolynomial::from(c % n)`, hence ≈. `set_fmpz_poly`, the reduction of an
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html),
is `g.mod_op(&n)`, whose result is a
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html).
`get_fmpz_poly` lifts with representatives in $$[0, p)$$, as
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) does.

The two `nmod` bridges assume, without checking, that both moduli are equal;
[`From<UnsignedPolynomial<T>>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
likewise carries no modulus. The narrowing direction is
[`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html), which fails only if
a coefficient does not fit in `T`, so it always succeeds on the inputs `get_nmod_poly` accepts.

## [Comparison](https://flintlib.org/doc/fmpz_mod_poly.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_mod_poly_equal (const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx)` | [`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html) |
| ✓ | `int fmpz_mod_poly_equal_trunc (const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, slong n, const fmpz_mod_ctx_t ctx)` | [`eq_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.EqTruncated.html#tymethod.eq_truncated) |
| ✓ | `int fmpz_mod_poly_is_zero (const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | `p == 0u32` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ≈ | `int fmpz_mod_poly_is_one (const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | `p == 1u32` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ≈ | `int fmpz_mod_poly_is_gen (const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | `== `[`x()`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.x) |

[`Eq`](https://doc.rust-lang.org/nightly/core/cmp/trait.Eq.html) and
[`eq_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.EqTruncated.html#tymethod.eq_truncated)
need no modulus. A `NaturalPolynomial` also compares with a value of any unsigned primitive type,
in either order: `p == c` holds exactly when `p` is the constant polynomial `c`, so `is_zero` and
`is_one` are `p == 0u32` and `p == 1u32`. The ≈ rows differ only at $$n = 1$$: FLINT's `one`
produces the zero polynomial there, which `is_one` rejects, and `is_gen` accepts anything, where
`== NaturalPolynomial::x()` does not.

## [Getting and setting coefficients](https://flintlib.org/doc/fmpz_mod_poly.html#getting-and-setting-coefficients) {#getting-and-setting-coefficients}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_poly_set_coeff_fmpz (fmpz_mod_poly_t poly, slong n, const fmpz_t x, const fmpz_mod_ctx_t ctx)` | [`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.mutate_coefficient) |
| ≈ | `void fmpz_mod_poly_set_coeff_ui (fmpz_mod_poly_t poly, slong n, ulong x, const fmpz_mod_ctx_t ctx)` | [`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.mutate_coefficient) |
| ✓ | `void fmpz_mod_poly_get_coeff_fmpz (fmpz_t x, const fmpz_mod_poly_t poly, slong n, const fmpz_mod_ctx_t ctx)` | [`coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.coefficient) |
| — | `void fmpz_mod_poly_set_coeff_mpz (fmpz_mod_poly_t poly, slong n, const mpz_t x, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_get_coeff_mpz (mpz_t x, const fmpz_mod_poly_t poly, slong n, const fmpz_mod_ctx_t ctx)` | |

Setters reduce the value on the way in and getters need no modulus.
[`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.mutate_coefficient)
hands the closure a
`&mut `[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
and does not reduce, so the faithful spelling of `set_coeff` is
`p.mutate_coefficient(i, |c| *c = x % n)`, hence ≈. Both libraries grow the polynomial with zero
fill when the index is past the degree and re-trim after a write. A
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) cannot
hold a negative value, so balanced representatives must be reduced first. The two `mpz` entries
are — because the header defines them as deprecation errors pointing at the `fmpz` versions.

## [Shifting](https://flintlib.org/doc/fmpz_mod_poly.html#shifting) {#shifting}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_shift_left (fmpz_mod_poly_t f, const fmpz_mod_poly_t g, slong n, const fmpz_mod_ctx_t ctx)` | [`mul_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulPowerOfX.html#tymethod.mul_power_of_x) |
| ✓ | `void fmpz_mod_poly_shift_right (fmpz_mod_poly_t f, const fmpz_mod_poly_t g, slong n, const fmpz_mod_ctx_t ctx)` | [`div_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DivPowerOfX.html#tymethod.div_power_of_x) |

`shift_left` is
[`mul_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulPowerOfX.html#tymethod.mul_power_of_x)
and `shift_right` is
[`div_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DivPowerOfX.html#tymethod.div_power_of_x);
neither needs a modulus.
[`ModShl`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShl.html)
and
[`ModShr`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModShr.html)
mean multiplication and division by $$2^k$$ modulo $$m$$, not coefficient shifts.

## [Addition and subtraction](https://flintlib.org/doc/fmpz_mod_poly.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_add (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx)` | [`mod_add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html#tymethod.mod_add) |
| ✓ | `void fmpz_mod_poly_add_series (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, slong n, const fmpz_mod_ctx_t ctx)` | [`mod_add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModAddTruncated.html#tymethod.mod_add_truncated) |
| ✓ | `void fmpz_mod_poly_sub (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx)` | [`mod_sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html#tymethod.mod_sub) |
| ✓ | `void fmpz_mod_poly_sub_series (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, slong n, const fmpz_mod_ctx_t ctx)` | [`mod_sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModSubTruncated.html#tymethod.mod_sub_truncated) |
| ✓ | `void fmpz_mod_poly_neg (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`mod_neg`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModNeg.html#tymethod.mod_neg) |

[`mod_add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html#tymethod.mod_add),
[`mod_sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSub.html#tymethod.mod_sub)
and `mod_neg` on
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
take the modulus as a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) in place
of a context, check that the coefficients are reduced, and normalise the result as FLINT does;
[`mod_power_of_2_add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Add.html#tymethod.mod_power_of_2_add)
and
[`mod_power_of_2_sub`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Sub.html#tymethod.mod_power_of_2_sub)
do the same for a modulus that is a power of 2. The `_series` pair are
[`mod_add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModAddTruncated.html#tymethod.mod_add_truncated)
and
[`mod_sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModSubTruncated.html#tymethod.mod_sub_truncated),
and for a power of 2
[`mod_power_of_2_add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2AddTruncated.html#tymethod.mod_power_of_2_add_truncated)
and
[`mod_power_of_2_sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2SubTruncated.html#tymethod.mod_power_of_2_sub_truncated),
all of which check that the whole of each operand is reduced.

## [Scalar multiplication and division](https://flintlib.org/doc/fmpz_mod_poly.html#scalar-multiplication-and-division) {#scalar}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_scalar_mul_fmpz (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_t x, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_scalar_mul_ui (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, ulong x, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_scalar_addmul_fmpz (fmpz_mod_poly_t rop, const fmpz_mod_poly_t op, const fmpz_t x, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_scalar_div_fmpz (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_t x, const fmpz_mod_ctx_t ctx)` | |

`scalar_div_fmpz` aborts the process ("Impossible inverse") when $$x$$ is not invertible modulo
$$p$$; for scalars,
[`ModDiv`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModDiv.html)
and
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html)
on [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
return an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html) instead.

## [Multiplication](https://flintlib.org/doc/fmpz_mod_poly.html#multiplication) {#multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_mul (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx)` | [`mod_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html#tymethod.mod_mul) |
| ✓ | `void fmpz_mod_poly_mullow (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, slong n, const fmpz_mod_ctx_t ctx)` | [`mod_mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMulTruncated.html#tymethod.mod_mul_truncated) |
| ✗ | `void fmpz_mod_poly_mulmid (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, slong nlo, slong nhi, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_mulhigh (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, slong start, const fmpz_mod_ctx_t ctx)` | |
| ✓ | `void fmpz_mod_poly_sqr (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`mod_square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModSquare.html#tymethod.mod_square) |
| ✗ | `void fmpz_mod_poly_mulmod (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_poly_t f, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_mulmod_preinv (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_poly_t f, const fmpz_mod_poly_t finv, const fmpz_mod_ctx_t ctx)` | |

`mulmod` and `mulmod_preinv` reduce modulo a polynomial; see [Two moduli](#two-moduli).

## [Products](https://flintlib.org/doc/fmpz_mod_poly.html#products) {#products}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_product_roots_fmpz_vec (fmpz_mod_poly_t poly, const fmpz * xs, slong n, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_poly_find_distinct_nonzero_roots (fmpz * roots, const fmpz_mod_poly_t A, const fmpz_mod_ctx_t ctx)` | |

The rendered manual also lists the powering functions under this heading, because the chapter's
Powering title does not render; this page gives [Powering](#powering) its own section.

## [Powering](https://flintlib.org/doc/fmpz_mod_poly.html#products) {#powering}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_pow (fmpz_mod_poly_t rop, const fmpz_mod_poly_t op, ulong e, const fmpz_mod_ctx_t ctx)` | [`mod_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html#tymethod.mod_pow) |
| ✓ | `void fmpz_mod_poly_pow_trunc (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, ulong e, slong trunc, const fmpz_mod_ctx_t ctx)` | [`mod_pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowTruncated.html#tymethod.mod_pow_truncated) |
| ⚙ | `void fmpz_mod_poly_pow_trunc_binexp (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, ulong e, slong trunc, const fmpz_mod_ctx_t ctx)` | [`mod_pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowTruncated.html#tymethod.mod_pow_truncated) |
| ✗ | `void fmpz_mod_poly_powmod_ui_binexp (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, ulong e, const fmpz_mod_poly_t f, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_powmod_ui_binexp_preinv (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, ulong e, const fmpz_mod_poly_t f, const fmpz_mod_poly_t finv, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_powmod_fmpz_binexp (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_t e, const fmpz_mod_poly_t f, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_powmod_fmpz_binexp_preinv (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_t e, const fmpz_mod_poly_t f, const fmpz_mod_poly_t finv, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_powmod_x_fmpz_preinv (fmpz_mod_poly_t res, const fmpz_t e, const fmpz_mod_poly_t f, const fmpz_mod_poly_t finv, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_powers_mod_naive (fmpz_mod_poly_struct * res, const fmpz_mod_poly_t f, slong n, const fmpz_mod_poly_t g, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_powers_mod_bsgs (fmpz_mod_poly_struct * res, const fmpz_mod_poly_t f, slong n, const fmpz_mod_poly_t g, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_frobenius_powers_2exp_precomp (fmpz_mod_poly_frobenius_powers_2exp_t pow, const fmpz_mod_poly_t f, const fmpz_mod_poly_t finv, ulong m, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_frobenius_powers_2exp_clear (fmpz_mod_poly_frobenius_powers_2exp_t pow, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_frobenius_power (fmpz_mod_poly_t res, fmpz_mod_poly_frobenius_powers_2exp_t pow, const fmpz_mod_poly_t f, ulong m, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_frobenius_powers_precomp (fmpz_mod_poly_frobenius_powers_t pow, const fmpz_mod_poly_t f, const fmpz_mod_poly_t finv, ulong m, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_frobenius_powers_clear (fmpz_mod_poly_frobenius_powers_t pow, const fmpz_mod_ctx_t ctx)` | |

The heading links to `#products` because that is where these entries appear in the manual. Both
`_clear` rows are [`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html). `pow` is
[`mod_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPow.html#tymethod.mod_pow)
on
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html),
and `pow_trunc` and its `binexp` algorithm are
[`mod_pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowTruncated.html#tymethod.mod_pow_truncated);
for a modulus that is a power of 2,
[`mod_power_of_2_pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2Pow.html#tymethod.mod_power_of_2_pow)
and
[`mod_power_of_2_pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2PowTruncated.html#tymethod.mod_power_of_2_pow_truncated)
do the same. Unlike `fmpz_mod_poly_pow_trunc`, which gives 0 for the zeroth power of the zero
polynomial, the truncated forms give 1 (reduced), as every other power does.

## [Division](https://flintlib.org/doc/fmpz_mod_poly.html#division) {#division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_divrem (fmpz_mod_poly_t Q, fmpz_mod_poly_t R, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_divrem_basecase (fmpz_mod_poly_t Q, fmpz_mod_poly_t R, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_divrem_newton_n_preinv (fmpz_mod_poly_t Q, fmpz_mod_poly_t R, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_poly_t Binv, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_divrem_f (fmpz_t f, fmpz_mod_poly_t Q, fmpz_mod_poly_t R, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_div (fmpz_mod_poly_t Q, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_div_newton_n_preinv (fmpz_mod_poly_t Q, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_poly_t Binv, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_rem (fmpz_mod_poly_t R, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_rem_basecase (fmpz_mod_poly_t R, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_rem_f (fmpz_t f, fmpz_mod_poly_t R, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `ulong fmpz_mod_poly_remove (fmpz_mod_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_ctx_t ctx)` | |

Every entry gives $$A = BQ + R$$ with $$\deg R < \deg B$$ and requires the leading coefficient of
$$B$$ to be invertible modulo $$n$$, not that $$n$$ be prime; where it is not, `divrem_f` and
`rem_f` set `f` to a nontrivial factor of $$n$$ instead. The `_newton_n_preinv` rows also take
`Binv`, the inverse of the reverse of $$B$$ modulo $$x^{\operatorname{len}(B)}$$, and require
$$\operatorname{len}(A) \leq 2\operatorname{len}(B) - 2$$; `remove` does not terminate when `g` is
a unit, including a non-constant unit such as $$1 + 2x$$ modulo 4.

## [Divisibility testing](https://flintlib.org/doc/fmpz_mod_poly.html#divisibility-testing) {#divisibility-testing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_mod_poly_divides (fmpz_mod_poly_t Q, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_poly_divides_classical (fmpz_mod_poly_t Q, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |

`remove` is under [Division](#division).

## [Power series inversion](https://flintlib.org/doc/fmpz_mod_poly.html#power-series-inversion) {#power-series-inversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_inv_series (fmpz_mod_poly_t Qinv, const fmpz_mod_poly_t Q, slong n, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_inv_series_f (fmpz_t f, fmpz_mod_poly_t Qinv, const fmpz_mod_poly_t Q, slong n, const fmpz_mod_ctx_t ctx)` | |

`inv_series` computes the first `n` coefficients of $$1/Q$$ and aborts unless the constant term of
$$Q$$ is a unit, which over a composite modulus excludes nonzero non-invertible values;
`inv_series_f` instead sets `f` to a nontrivial factor of the modulus. Whether the constant term
is a unit can be tested beforehand with
[`ModInverse`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModInverse.html),
which returns an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html).

## [Power series division](https://flintlib.org/doc/fmpz_mod_poly.html#power-series-division) {#power-series-division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_div_series (fmpz_mod_poly_t Q, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, slong n, const fmpz_mod_ctx_t ctx)` | |

`div_series` computes the first `n` coefficients of $$A/B$$ and, as in [Power series
inversion](#power-series-inversion), aborts unless the constant term of $$B$$ is a unit; there is
no `_f` form that returns a factor of the modulus instead.

## [Greatest common divisor](https://flintlib.org/doc/fmpz_mod_poly.html#greatest-common-divisor) {#gcd}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_make_monic (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`mod_make_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMakeMonic.html#tymethod.mod_make_monic) |
| ≈ | `void fmpz_mod_poly_make_monic_f (fmpz_t f, fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`mod_make_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMakeMonic.html#tymethod.mod_make_monic) |
| ✗ | `void fmpz_mod_poly_gcd (fmpz_mod_poly_t G, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_gcd_f (fmpz_t f, fmpz_mod_poly_t G, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_gcd_euclidean_f (fmpz_t f, fmpz_mod_poly_t G, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_xgcd (fmpz_mod_poly_t G, fmpz_mod_poly_t S, fmpz_mod_poly_t T, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_xgcd_f (fmpz_t f, fmpz_mod_poly_t G, fmpz_mod_poly_t S, fmpz_mod_poly_t T, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_xgcd_euclidean_f (fmpz_t f, fmpz_mod_poly_t G, fmpz_mod_poly_t S, fmpz_mod_poly_t T, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_gcdinv (fmpz_mod_poly_t G, fmpz_mod_poly_t S, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_gcdinv_f (fmpz_t f, fmpz_mod_poly_t G, fmpz_mod_poly_t S, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_gcdinv_euclidean (fmpz_mod_poly_t G, fmpz_mod_poly_t S, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_gcdinv_euclidean_f (fmpz_t f, fmpz_mod_poly_t G, fmpz_mod_poly_t S, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_poly_invmod (fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_poly_t P, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_poly_invmod_f (fmpz_t f, fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_poly_t P, const fmpz_mod_ctx_t ctx)` | |

`make_monic` succeeds exactly when the leading coefficient is invertible, and `make_monic_f`
otherwise sets `f` to a nontrivial factor of $$p$$;
[`mod_make_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModMakeMonic.html#tymethod.mod_make_monic)
does both, returning the factor as the error of a
[`Result`](https://doc.rust-lang.org/nightly/core/result/enum.Result.html).

## [Minpoly](https://flintlib.org/doc/fmpz_mod_poly.html#minpoly) {#minpoly}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_minpoly (fmpz_mod_poly_t poly, const fmpz * seq, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_minpoly_bm (fmpz_mod_poly_t poly, const fmpz * seq, slong len, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_minpoly_hgcd (fmpz_mod_poly_t poly, const fmpz * seq, slong len, const fmpz_mod_ctx_t ctx)` | |

These compute the minimal generating polynomial of a linear recurrence sequence given as an array
of scalars, as in [the Berlekamp–Massey section](#berlekamp-massey), not the minimal polynomial of
an algebraic element. All three require a [prime modulus](#primality) and return a result that is
not unique; `minpoly_bm` and `minpoly_hgcd` are algorithm variants of `minpoly`.

## [Resultant](https://flintlib.org/doc/fmpz_mod_poly.html#resultant) {#resultant}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_resultant (fmpz_t res, const fmpz_mod_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_ctx_t ctx)` | |

The resultant is the standard $$\operatorname{lc}(f)^{\deg g} \operatorname{lc}(g)^{\deg f} \prod
(x - y)$$, the product running over the roots $$x$$ of $$f$$ and $$y$$ of $$g$$, and the function
is correct for any modulus, not only a prime one.

## [Discriminant](https://flintlib.org/doc/fmpz_mod_poly.html#discriminant) {#discriminant}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_discriminant (fmpz_t d, const fmpz_mod_poly_t f, const fmpz_mod_ctx_t ctx)` | |

`discriminant` computes the standard $$\operatorname{disc}(f) = (-1)^{d(d-1)/2}
\operatorname{res}(f, f') / \operatorname{lc}(f)$$ with $$d = \deg f$$. It is zero exactly when $$f$$ has a repeated
root, which over $$\mathbb{F}_p$$ includes every $$f$$ with $$f' = 0$$.

## [Derivative](https://flintlib.org/doc/fmpz_mod_poly.html#derivative) {#derivative}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_derivative (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`mod_derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModDerivative.html#tymethod.mod_derivative) |

For a modulus that is a power of 2, use
[`mod_power_of_2_derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2Derivative.html#tymethod.mod_power_of_2_derivative).

## [Evaluation](https://flintlib.org/doc/fmpz_mod_poly.html#evaluation) {#evaluation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_poly_evaluate_fmpz (fmpz_t res, const fmpz_mod_poly_t poly, const fmpz_t a, const fmpz_mod_ctx_t ctx)` | [`mod_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluate.html#tymethod.mod_evaluate) |

FLINT reduces the point on the way in, whereas under [Malachite's rule](#reduced-check) an
unreduced point panics, hence ≈. The evaluation is
[`mod_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluate.html#tymethod.mod_evaluate),
and for a modulus that is a power of 2,
[`mod_power_of_2_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModPowerOf2Evaluate.html#tymethod.mod_power_of_2_evaluate);
both panic unless the coefficients and the point are reduced.

## [Multipoint evaluation](https://flintlib.org/doc/fmpz_mod_poly.html#multipoint-evaluation) {#multipoint-evaluation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_mod_poly_evaluate_fmpz_vec (fmpz * ys, const fmpz_mod_poly_t poly, const fmpz * xs, slong n, const fmpz_mod_ctx_t ctx)` | [`mod_evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateMany.html#tymethod.mod_evaluate_many) |
| ⚙ | `void fmpz_mod_poly_evaluate_fmpz_vec_iter (fmpz * ys, const fmpz_mod_poly_t poly, const fmpz * xs, slong n, const fmpz_mod_ctx_t ctx)` | [`mod_evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateMany.html#tymethod.mod_evaluate_many) |
| ⚙ | `void fmpz_mod_poly_evaluate_fmpz_vec_fast (fmpz * ys, const fmpz_mod_poly_t poly, const fmpz * xs, slong n, const fmpz_mod_ctx_t ctx)` | [`mod_evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateMany.html#tymethod.mod_evaluate_many) |

All three compute the same values and require the points to be reduced, so
[`mod_evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluateMany.html#tymethod.mod_evaluate_many)
gives the result of `evaluate_fmpz_vec` and `evaluate_fmpz_vec_fast` as well; it panics on an
unreduced point.

## [Composition](https://flintlib.org/doc/fmpz_mod_poly.html#composition) {#composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_compose (fmpz_mod_poly_t res, const fmpz_mod_poly_t poly1, const fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx)` | |

`compose(res, poly1, poly2)` sets `res` to $$g(h(t))$$, where $$g$$ is `poly1` and $$h$$ is
`poly2`; over a composite modulus its degree can be less than $$\deg g \cdot \deg h$$, and it can
be zero (modulo 4, $$2x \circ 2x = 0$$). Horner's rule with
[`mod_mul`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModMul.html#tymethod.mod_mul)
by $$h$$ and
[`mod_add`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModAdd.html#tymethod.mod_add)
of each coefficient of $$g$$, as a constant polynomial, computes the same result. [Modular
composition](#modular-composition) is composition modulo a polynomial, a different operation.

## [Square roots](https://flintlib.org/doc/fmpz_mod_poly.html#square-roots) {#square-roots}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_sqrt_series (fmpz_mod_poly_t g, const fmpz_mod_poly_t h, slong n, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_invsqrt_series (fmpz_mod_poly_t g, const fmpz_mod_poly_t h, slong n, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_poly_sqrt (fmpz_mod_poly_t s, const fmpz_mod_poly_t p, const fmpz_mod_ctx_t ctx)` | |

`sqrt_series` and `invsqrt_series` compute the first `n` coefficients of $$\sqrt{h}$$ and
$$1/\sqrt{h}$$; they require the constant term of $$h$$ to be 1 and 2 to be invertible, and abort
modulo 2. `sqrt` requires a [prime modulus](#primality), including 2, and returns 1 after setting
$$s$$ to a square root of $$p$$, which is not unique, or 0 if there is none.

## [Modular composition](https://flintlib.org/doc/fmpz_mod_poly.html#modular-composition) {#modular-composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_compose_mod (fmpz_mod_poly_t res, const fmpz_mod_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_poly_t h, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_compose_mod_horner (fmpz_mod_poly_t res, const fmpz_mod_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_poly_t h, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_compose_mod_brent_kung (fmpz_mod_poly_t res, const fmpz_mod_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_poly_t h, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_compose_mod_brent_kung_preinv (fmpz_mod_poly_t res, const fmpz_mod_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_poly_t h, const fmpz_mod_poly_t hinv, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv (fmpz_mod_poly_t res, const fmpz_mod_poly_t f, const fmpz_mod_mat_t A, const fmpz_mod_poly_t h, const fmpz_mod_poly_t hinv, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_compose_mod_brent_kung_vec_preinv (fmpz_mod_poly_struct * res, const fmpz_mod_poly_struct * polys, slong len1, slong l, const fmpz_mod_poly_t g, const fmpz_mod_poly_t poly, const fmpz_mod_poly_t polyinv, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded (fmpz_mod_poly_struct * res, const fmpz_mod_poly_struct * polys, slong len1, slong n, const fmpz_mod_poly_t g, const fmpz_mod_poly_t poly, const fmpz_mod_poly_t polyinv, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded_pool (fmpz_mod_poly_struct * res, const fmpz_mod_poly_struct * polys, slong len1, slong n, const fmpz_mod_poly_t g, const fmpz_mod_poly_t poly, const fmpz_mod_poly_t polyinv, const fmpz_mod_ctx_t ctx, thread_pool_handle * threads, slong num_threads)` | |
| ✗ | `void fmpz_mod_poly_precompute_matrix (fmpz_mod_mat_t A, const fmpz_mod_poly_t f, const fmpz_mod_poly_t g, const fmpz_mod_poly_t ginv, const fmpz_mod_ctx_t ctx)` | |

The threaded rows are — because Malachite is single-threaded.

## [Subproduct trees](https://flintlib.org/doc/fmpz_mod_poly.html#subproduct-trees) {#subproduct-trees}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `fmpz_poly_struct ** _fmpz_mod_poly_tree_alloc (slong len)` | |
| — | `void _fmpz_mod_poly_tree_free (fmpz_poly_struct ** tree, slong len)` | |
| — | `void _fmpz_mod_poly_tree_build (fmpz_poly_struct ** tree, const fmpz * roots, slong len, const fmpz_mod_ctx_t ctx)` | |

These allocate, free and fill a tree's buffer; a Rust tree allocates and frees itself, so all
three are —.

## [Radix conversion](https://flintlib.org/doc/fmpz_mod_poly.html#radix-conversion) {#radix-conversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_poly_radix_init (fmpz_mod_poly_radix_t D, const fmpz_mod_poly_t R, slong degF, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_poly_radix (fmpz_mod_poly_struct ** B, const fmpz_mod_poly_t F, const fmpz_mod_poly_radix_t D, const fmpz_mod_ctx_t ctx)` | |

`radix` writes $$F = B_0 + B_1 R + \dots + B_N R^N$$ with $$\deg B_i < \deg R$$, using the data
that `radix_init` precomputes from $$R$$ for every $$F$$ of degree at most `degF`. It requires the
leading coefficient of $$R$$ to be a unit, not a prime modulus.

## [Input and output](https://flintlib.org/doc/fmpz_mod_poly.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_mod_poly_print_pretty (const fmpz_mod_poly_t poly, const char * x, const fmpz_mod_ctx_t ctx)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`to_string_with`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.to_string_with) |
| ✓ | `int fmpz_mod_poly_fprint_pretty (FILE * file, const fmpz_mod_poly_t poly, const char * x, const fmpz_mod_ctx_t ctx)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int fmpz_mod_poly_print (const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int fmpz_mod_poly_fprint (FILE * file, const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |

The plain format writes length, modulus and coefficients (`4 6  1 2 0 5` for $$5x^3 + 2x + 1$$
over $$\mathbb{Z}/6\mathbb{Z}$$); the pretty format writes `5*x^3+2*x+1` with no modulus.
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) and
[`to_string_with`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html#method.to_string_with)
give the pretty form, and the serde encoding carries only coefficients, so FLINT's plain format
does not round-trip into a
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
alone, hence ≈. The undocumented `get_str_pretty` is
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), and `fread` is
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html).

## [Inflation and deflation](https://flintlib.org/doc/fmpz_mod_poly.html#inflation-and-deflation) {#inflation-and-deflation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_mod_poly_inflate (fmpz_mod_poly_t result, const fmpz_mod_poly_t input, ulong inflation, const fmpz_mod_ctx_t ctx)` | [`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x) |
| ≈ | `void fmpz_mod_poly_deflate (fmpz_mod_poly_t result, const fmpz_mod_poly_t input, ulong deflation, const fmpz_mod_ctx_t ctx)` | [`deflate_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DeflatePowerOfX.html#tymethod.deflate_power_of_x) |
| ≈ | `ulong fmpz_mod_poly_deflation (const fmpz_mod_poly_t input, const fmpz_mod_ctx_t ctx)` | [`exponent_gcd`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ExponentGcd.html#tymethod.exponent_gcd) |

`deflation` returns the largest $$n$$ by which the polynomial can be deflated, the GCD of its
exponents, with 0 for the zero polynomial and 1 for a constant.
[`exponent_gcd`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ExponentGcd.html#tymethod.exponent_gcd)
is the same, except that a nonzero constant gives 0, where FLINT gives 1, hence ≈. `deflate`
by an $$n$$ that does not divide every exponent silently drops the offending terms (modulo 5,
`deflate(x^2 + x, 2)` is $$x$$), where
[`deflate_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DeflatePowerOfX.html#tymethod.deflate_power_of_x)
panics, hence ≈.
[`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x)
needs no modulus and matches `inflate` except at $$n = 0$$, where it gives $$p(1)$$ as an
unreduced sum rather than modulo $$m$$, hence ≈.

## [Berlekamp–Massey Algorithm](https://flintlib.org/doc/fmpz_mod_poly.html#berlekamp-massey-algorithm) {#berlekamp-massey}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_mod_berlekamp_massey_init (fmpz_mod_berlekamp_massey_t B, const fmpz_mod_ctx_t ctx)` | |
| — | `void fmpz_mod_berlekamp_massey_clear (fmpz_mod_berlekamp_massey_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_berlekamp_massey_start_over (fmpz_mod_berlekamp_massey_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_berlekamp_massey_add_point (fmpz_mod_berlekamp_massey_t B, const fmpz_t a, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_berlekamp_massey_add_points (fmpz_mod_berlekamp_massey_t B, const fmpz * a, slong count, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `void fmpz_mod_berlekamp_massey_add_zeros (fmpz_mod_berlekamp_massey_t B, slong count, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `int fmpz_mod_berlekamp_massey_reduce (fmpz_mod_berlekamp_massey_t B, const fmpz_mod_ctx_t ctx)` | |
| ✗ | `slong fmpz_mod_berlekamp_massey_point_count (const fmpz_mod_berlekamp_massey_t B)` | |
| ✗ | `const fmpz * fmpz_mod_berlekamp_massey_points (const fmpz_mod_berlekamp_massey_t B)` | |
| ✗ | `const fmpz_mod_poly_struct * fmpz_mod_berlekamp_massey_V_poly (const fmpz_mod_berlekamp_massey_t B)` | |
| ✗ | `const fmpz_mod_poly_struct * fmpz_mod_berlekamp_massey_R_poly (const fmpz_mod_berlekamp_massey_t B)` | |

These functions use the prefix `fmpz_mod_berlekamp_massey_`; `clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html).
