---
layout: default
title: "Malachite for FLINT Users: Rational Polynomials"
permalink: /mapping/flint-rational-polynomials/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Rational Polynomials

This page maps the functions of FLINT's rational polynomial type, `fmpq_poly_t`, onto their
Malachite counterpart:
[`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html),
from the `malachite-q` crate. It follows the organization of the
[fmpq_poly.h chapter](https://flintlib.org/doc/fmpq_poly.html) of the FLINT manual, as of FLINT
3.6.0. Its companion is
[Malachite for FLINT Users: Integer Polynomials](/mapping/flint-integer-polynomials/), which maps
the same operations over $$\mathbb{Z}$$; the
[conventions](/mapping/flint-integers/#conventions) of [the fmpz page](/mapping/flint-integers/)
apply here too, and the [mapping index](/mapping/) lists the whole family.

The page covers all 32 sections of the chapter and maps every documented public function — 145 of
them, plus the two type rows — as 44 ✓, 19 ≈, 15 —, and 69 ✗. Functions whose names begin with an
underscore (96 of them) are omitted, as on the companion pages. Two of the chapter's headings are
both called "Powering" and cover unrelated subjects; they appear here as [Powering](#powering) and
[precomputed powers for remainders](#powering-precomputed).

## Conventions {#conventions}

### The `fmpq_poly` representation {#representation}

Both libraries store a rational polynomial as an integer numerator polynomial over a single shared
denominator rather than as a list of rational coefficients:
[`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html)
is an
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html)
numerator over a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
denominator.

### Canonical form, and where it lives

FLINT calls a polynomial *canonical* when the numerator and denominator are coprime, the
denominator is positive, the numerator has no trailing zero coefficient, and the zero polynomial
is written $$0/1$$. In Malachite all four conditions hold for every value: the first two by the
choice of a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
denominator and an
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html)
numerator, the other two by the constructors. Canonical form is therefore an invariant of the type
rather than a state to be restored, which is why `fmpq_poly_canonicalise` and
`fmpq_poly_is_canonical` have no counterparts. In FLINT several public functions can leave a
value non-canonical: writing through the `numref`/`denref` accessors, `set_str` on input not in
lowest terms, the `add_can`/`sub_can` family with `can = 0`, and `rem_powers_precomp`.

### What the representation costs

No coefficient is stored as such: the coefficient of $$x^i$$ is $$n_i/d$$, so where
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html)
lends a coefficient by reference, this type builds a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) and
returns it by value, and where that type lends a slice this one returns a
[`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html).

### Categories

Each function falls into one of four categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/fmpq_poly.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpq_poly_struct` | [`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html) |
| — | `fmpq_poly_t` | |

The `_t` form exists so a polynomial can be passed and mutated through a pointer, which `&` and
`&mut` already provide, so only the `struct` has a counterpart.

## [Memory management](https://flintlib.org/doc/fmpq_poly.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_init (fmpq_poly_t poly)` | [`RationalPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| — | `void fmpq_poly_init2 (fmpq_poly_t poly, slong alloc)` | |
| — | `void fmpq_poly_realloc (fmpq_poly_t poly, slong alloc)` | |
| — | `void fmpq_poly_fit_length (fmpq_poly_t poly, slong len)` | |
| — | `void fmpq_poly_clear (fmpq_poly_t poly)` | |
| — | `void fmpq_poly_canonicalise (fmpq_poly_t poly)` | |
| — | `int fmpq_poly_is_canonical (const fmpq_poly_t poly)` | |

`init` gives the zero polynomial,
[`RationalPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html);
`init2`, `realloc` and `fit_length` are capacity management, which a
[`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html) does for itself, and `clear`
is [`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html). `canonicalise` and
`is_canonical` are unneeded because [canonical form is an invariant](#representation) of the type.

## [Polynomial parameters](https://flintlib.org/doc/fmpq_poly.html#polynomial-parameters) {#polynomial-parameters}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `slong fmpq_poly_degree (const fmpq_poly_t poly)` | [`degree`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.degree) |
| ✓ | `slong fmpq_poly_length (const fmpq_poly_t poly)` | [`len`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.len) |

`fmpq_poly_degree` returns `length - 1`, so $$-1$$ for the zero polynomial, where
[`degree`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.degree)
returns an [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)`<`[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html)`>`
with [`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None) for
zero. `fmpq_poly_length` is
[`len`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.len),
not
[`to_coefficients_asc`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.to_coefficients_asc)`().len()`,
which builds every coefficient.

## [Accessing the numerator and denominator](https://flintlib.org/doc/fmpq_poly.html#accessing-the-numerator-and-denominator) {#accessing-numerator-denominator}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `fmpz * fmpq_poly_numref (fmpq_poly_t poly)` | [`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.numerator_ref), [`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_numerator) |
| ≈ | `fmpz_t fmpq_poly_denref (fmpq_poly_t poly)` | [`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.denominator_ref), [`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_denominator) |
| ≈ | `void fmpq_poly_get_numerator (fmpz_poly_t res, const fmpq_poly_t poly)` | [`into_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.into_numerator_and_denominator) |
| ≈ | `void fmpq_poly_get_denominator (fmpz_t den, const fmpq_poly_t poly)` | [`into_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.into_numerator_and_denominator) |

`numref` and `denref` are writable, so a caller can leave the polynomial non-canonical through
them.
[`numerator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.numerator_ref)
and
[`denominator_ref`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.denominator_ref)
are read-only. To modify them, use
[`mutate_numerator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_numerator),
[`mutate_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_denominator), or
[`mutate_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_numerator_and_denominator),
which pass the halves to a closure and reduce the polynomial when it returns, so it never becomes
non-canonical. The copying pair map to
[`into_numerator_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.into_numerator_and_denominator),
which returns both halves at once and consumes the polynomial. Despite `get_numerator`'s
description, the numerator need not be primitive: $$(2x + 4)/3$$ is canonical.

## [Random testing](https://flintlib.org/doc/fmpq_poly.html#random-testing) {#random-testing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpq_poly_randtest (fmpq_poly_t f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_rational_polynomials`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/random/fn.random_rational_polynomials.html), [`striped_random_rational_polynomials`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/random/fn.striped_random_rational_polynomials.html) |
| ≈ | `void fmpq_poly_randtest_unsigned (fmpq_poly_t f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_rational_polynomials_from_iterators`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/random/fn.random_rational_polynomials_from_iterators.html) |
| ≈ | `void fmpq_poly_randtest_not_zero (fmpq_poly_t f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_rational_polynomials_min_degree`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/random/fn.random_rational_polynomials_min_degree.html) |

FLINT bounds the length and bit size; Malachite's generators take mean parameters with unbounded
support. `randtest_not_zero` is
[`random_rational_polynomials_min_degree`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/random/fn.random_rational_polynomials_min_degree.html)
with a minimum degree of 0, and `randtest_unsigned` is
[`random_rational_polynomials_from_iterators`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/random/fn.random_rational_polynomials_from_iterators.html)
over non-negative
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s, since
there is no non-negative rational type.

## [Assignment, swap, negation](https://flintlib.org/doc/fmpq_poly.html#assignment-swap-negation) {#assignment}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_set (fmpq_poly_t poly1, const fmpq_poly_t poly2)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpq_poly_set_si (fmpq_poly_t poly, slong x)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpq_poly_set_ui (fmpq_poly_t poly, ulong x)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpq_poly_set_fmpz (fmpq_poly_t poly, const fmpz_t x)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpq_poly_set_fmpq (fmpq_poly_t poly, const fmpq_t x)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpq_poly_set_fmpz_poly (fmpq_poly_t rop, const fmpz_poly_t op)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpq_poly_zero (fmpq_poly_t poly)` | [`RationalPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void fmpq_poly_one (fmpq_poly_t poly)` | [`one`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.one) |
| ✓ | `void fmpq_poly_swap (fmpq_poly_t poly1, fmpq_poly_t poly2)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| ✓ | `char * fmpq_poly_get_str_pretty (const fmpq_poly_t poly, const char * var)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`to_string_with`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.to_string_with) |
| ≈ | `char * fmpq_poly_get_str (const fmpq_poly_t poly)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int fmpq_poly_set_str (fmpq_poly_t poly, const char * str)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ✓ | `void fmpq_poly_neg (fmpq_poly_t poly1, const fmpq_poly_t poly2)` | [`Neg`](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html) |
| ✗ | `void fmpq_poly_inv (fmpq_poly_t poly1, const fmpq_poly_t poly2)` | |
| ✓ | `void fmpq_poly_truncate (fmpq_poly_t poly, slong n)` | [`truncate_assign`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.truncate_assign) |
| ✓ | `void fmpq_poly_set_trunc (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | [`truncate`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.truncate) |
| ✓ | `void fmpq_poly_reverse (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | [`reverse`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.reverse) |
| ✗ | `void fmpq_poly_get_slice (fmpq_poly_t rop, const fmpq_poly_t op, slong i, slong j)` | |
| ✗ | `void fmpq_poly_get_nmod_poly (nmod_poly_t rop, const fmpq_poly_t op)` | |
| ✗ | `void fmpq_poly_get_nmod_poly_den (nmod_poly_t rop, const fmpq_poly_t op, int den)` | |
| ✗ | `void fmpq_poly_set_nmod_poly (fmpq_poly_t rop, const nmod_poly_t op)` | |

Assignment is [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html), the
scalar constructors are one blanket
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) over anything convertible
into a [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html),
`swap` is [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html), and
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<`[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html)`>`
covers `set_fmpz_poly`. `get_str` and `set_str` use the plain one-rational-per-coefficient format
described under [input and output](#input-and-output), which differs from Malachite's
serialisation, hence ≈; `set_str` guarantees canonical form only when every coefficient in the
string is in lowest terms, whereas
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) and
[`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) reduce as they build.
`fmpq_poly_inv` calls `abort()` unless its argument is a nonzero constant $$c$$, whose inverse is
`RationalPolynomial::from(c.reciprocal())`. `fmpq_poly_reverse` is
[`reverse`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.reverse),
or [`reverse_assign`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.reverse_assign)
in place. `get_slice(i, j)` keeps the terms of degree $$i$$ through $$j - 1$$ in place, which is
`p.truncate(j) - p.truncate(i)`.

## [Getting and setting coefficients](https://flintlib.org/doc/fmpq_poly.html#getting-and-setting-coefficients) {#getting-and-setting-coefficients}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_get_coeff_fmpq (fmpq_t x, const fmpq_poly_t poly, slong n)` | [`coefficient`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.coefficient) |
| — | `void fmpq_poly_get_coeff_fmpz (fmpz_t x, const fmpq_poly_t poly, slong n)` | |
| ≈ | `void fmpq_poly_set_coeff_si (fmpq_poly_t poly, slong n, slong x)` | [`mutate_coefficient`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_coefficient) |
| ≈ | `void fmpq_poly_set_coeff_ui (fmpq_poly_t poly, slong n, ulong x)` | [`mutate_coefficient`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_coefficient) |
| ≈ | `void fmpq_poly_set_coeff_fmpz (fmpq_poly_t poly, slong n, const fmpz_t x)` | [`mutate_coefficient`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_coefficient) |
| ≈ | `void fmpq_poly_set_coeff_fmpq (fmpq_poly_t poly, slong n, const fmpq_t x)` | [`mutate_coefficient`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_coefficient) |

`get_coeff_fmpz` returns the $$n$$th coefficient of the *numerator*, which is the coefficient only
when the denominator is 1; it is `p.numerator_ref().coefficient(n)`.
[`mutate_coefficient`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.mutate_coefficient)
hands its closure the coefficient as a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) and
rebuilds the polynomial around what comes back, since the common denominator may change.
Building a polynomial coefficient by coefficient is quadratic;
[`from_coefficients_asc`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.from_coefficients_asc)
takes them all at once.

## [Comparison](https://flintlib.org/doc/fmpq_poly.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_poly_equal (const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int fmpq_poly_cmp (const fmpq_poly_t left, const fmpq_poly_t right)` | [`ShortlexRationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.ShortlexRationalPolynomial.html) |
| ✓ | `int fmpq_poly_is_zero (const fmpq_poly_t poly)` | `p == 0u32` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ✓ | `int fmpq_poly_is_one (const fmpq_poly_t poly)` | `p == 1u32` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ✓ | `int fmpq_poly_equal_trunc (const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | [`eq_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.EqTruncated.html#tymethod.eq_truncated) |
| ✓ | `int fmpq_poly_is_gen (const fmpq_poly_t poly)` | [`x`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.x) |

`fmpq_poly_cmp` orders first by degree and then by coefficients from highest to lowest, which is
the [`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html) of
[`ShortlexRationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.ShortlexRationalPolynomial.html)
(and
[`ShortlexRationalPolynomialRef`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.ShortlexRationalPolynomialRef.html)
for borrowing); the default
[`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html) on
[`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html)
is the asymptotic order. `is_gen` is `*p == RationalPolynomial::x()`. A `RationalPolynomial`
compares with a value of any primitive integer type, `p == c` holding exactly when `p` is the
constant polynomial `c`, so `is_zero` and `is_one` are `*p == 0u32` and `*p == 1u32`.

## [Addition and subtraction](https://flintlib.org/doc/fmpq_poly.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_add (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | [`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html), [`AddAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.AddAssign.html) |
| ✓ | `void fmpq_poly_sub (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | [`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html), [`SubAssign`](https://doc.rust-lang.org/nightly/core/ops/trait.SubAssign.html) |
| — | `void fmpq_poly_add_can (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, int can)` | |
| — | `void fmpq_poly_sub_can (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, int can)` | |
| ✓ | `void fmpq_poly_add_series (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | [`add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.AddTruncated.html#tymethod.add_truncated) |
| ✓ | `void fmpq_poly_sub_series (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | [`sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SubTruncated.html#tymethod.sub_truncated) |
| — | `void fmpq_poly_add_series_can (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n, int can)` | |
| — | `void fmpq_poly_sub_series_can (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n, int can)` | |

The `_can` variants take a flag selecting whether the result is canonicalised; with `can = 0` the
result may not be in lowest terms. Malachite has no such flag because every value is reduced, so
those rows are —. The `_series` pair are
[`add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.AddTruncated.html#tymethod.add_truncated)
and
[`sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SubTruncated.html#tymethod.sub_truncated).

## [Scalar multiplication and division](https://flintlib.org/doc/fmpq_poly.html#scalar-multiplication-and-division) {#scalar}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_scalar_mul_si (fmpq_poly_t rop, const fmpq_poly_t op, slong c)` | |
| ✗ | `void fmpq_poly_scalar_mul_ui (fmpq_poly_t rop, const fmpq_poly_t op, ulong c)` | |
| ✗ | `void fmpq_poly_scalar_mul_fmpz (fmpq_poly_t rop, const fmpq_poly_t op, const fmpz_t c)` | |
| ✗ | `void fmpq_poly_scalar_mul_fmpq (fmpq_poly_t rop, const fmpq_poly_t op, const fmpq_t c)` | |
| ✗ | `void fmpq_poly_scalar_div_si (fmpq_poly_t rop, const fmpq_poly_t op, slong c)` | |
| ✗ | `void fmpq_poly_scalar_div_ui (fmpq_poly_t rop, const fmpq_poly_t op, ulong c)` | |
| ✗ | `void fmpq_poly_scalar_div_fmpz (fmpq_poly_t rop, const fmpq_poly_t op, const fmpz_t c)` | |
| ✗ | `void fmpq_poly_scalar_div_fmpq (fmpq_poly_t rop, const fmpq_poly_t op, const fmpq_t c)` | |

To scale by a rational `c`, multiply by `RationalPolynomial::from(c)`, or by
`RationalPolynomial::from(c.reciprocal())` to divide; `scalar_div` assumes a nonzero divisor
without checking, whereas the reciprocal of zero panics. Scaling by a power of 2 is `<<` and `>>`:
[`Shl`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#impl-Shl%3Ci64%3E-for-%26RationalPolynomial)
multiplies and
[`Shr`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#impl-Shr%3Ci64%3E-for-%26RationalPolynomial)
divides, a negative signed amount doing the opposite.

## [Multiplication](https://flintlib.org/doc/fmpq_poly.html#multiplication) {#multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_mul (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | [`*`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ✓ | `void fmpq_poly_mullow (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | [`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated) |
| ✗ | `void fmpq_poly_addmul (fmpq_poly_t rop, const fmpq_poly_t op1, const fmpq_poly_t op2)` | |
| ✗ | `void fmpq_poly_submul (fmpq_poly_t rop, const fmpq_poly_t op1, const fmpq_poly_t op2)` | |

`addmul` and `submul` are unfused in FLINT; write `rop += op1 * op2` and `rop -= op1 * op2`.
There is no `sqr`; Malachite's squaring functions are
[`square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html#tymethod.square)
and, for `mullow`,
[`square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SquareTruncated.html#tymethod.square_truncated).

## [Powering](https://flintlib.org/doc/fmpq_poly.html#powering) {#powering}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_pow (fmpq_poly_t res, const fmpq_poly_t poly, ulong e)` | [`pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html#tymethod.pow) |
| ✓ | `void fmpq_poly_pow_trunc (fmpq_poly_t res, const fmpq_poly_t poly, ulong e, slong n)` | [`pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.PowTruncated.html#tymethod.pow_truncated) |

This section is exponentiation; the chapter's other "Powering" is the
[precomputed-remainder scheme](#powering-precomputed). $$0^0 = 1$$, matching
[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
for [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html).

## [Shifting](https://flintlib.org/doc/fmpq_poly.html#shifting) {#shifting}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_shift_left (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | [`mul_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulPowerOfX.html#tymethod.mul_power_of_x) |
| ✓ | `void fmpq_poly_shift_right (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | [`div_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DivPowerOfX.html#tymethod.div_power_of_x) |

Shifting by $$n$$ multiplies or divides by $$x^n$$, the right shift discarding the low
coefficients. In Malachite `<<` and `>>` mean scaling by a power of two, not this operation.

## [Euclidean division](https://flintlib.org/doc/fmpq_poly.html#euclidean-division) {#euclidean-division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_divrem (fmpq_poly_t Q, fmpq_poly_t R, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | |
| ✗ | `void fmpq_poly_div (fmpq_poly_t Q, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | |
| ✗ | `void fmpq_poly_rem (fmpq_poly_t R, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | |

`fmpq_poly_divrem`, `div` and `rem` are ordinary Euclidean division in $$\mathbb{Q}[x]$$:
$$A = BQ + R$$ with $$\deg R < \deg B$$. Over $$\mathbb{Q}$$ both of the integer chapter's
compromises hold at once:

| | dividend | remainder |
| --- | :--- | :--- |
| [`fmpz_poly_divrem`](/mapping/flint-integer-polynomials/#euclidean-division) | $$A$$ itself | may keep high-degree terms, reduced modulo $$\ell$$ |
| [`fmpz_poly_pseudo_divrem`](/mapping/flint-integer-polynomials/#pseudo-division) | scaled to $$\ell^d A$$ | $$\deg R < \deg B$$, as in a field |

## [Powering (precomputed powers for remainders)](https://flintlib.org/doc/fmpq_poly.html#id1) {#powering-precomputed}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_powers_precompute (fmpq_poly_powers_precomp_t pinv, fmpq_poly_t poly)` | |
| — | `void fmpq_poly_powers_clear (fmpq_poly_powers_precomp_t pinv)` | |
| ✗ | `void fmpq_poly_rem_powers_precomp (fmpq_poly_t R, const fmpq_poly_t A, const fmpq_poly_t B, const fmpq_poly_powers_precomp_t B_inv)` | |

`powers_clear` is — because freeing a cache is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html).

## [Divisibility testing](https://flintlib.org/doc/fmpq_poly.html#divisibility-testing) {#divisibility-testing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpq_poly_divides (fmpq_poly_t q, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | |
| ✗ | `slong fmpq_poly_remove (fmpq_poly_t q, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | |

`divides` returns a flag and the quotient; a zero divisor divides a zero dividend and nothing else,
the convention
[`DivisibleBy`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivisibleBy.html)
uses for Malachite's integer types. `remove` returns the multiplicity of `poly2` in `poly1` and the
remaining cofactor, and aborts if `poly2` is zero or a nonzero constant.

## [Power series division](https://flintlib.org/doc/fmpq_poly.html#power-series-division) {#power-series-division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_inv_series_newton (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | |
| ✗ | `void fmpq_poly_inv_series (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | |
| ✗ | `void fmpq_poly_div_series (fmpq_poly_t Q, const fmpq_poly_t A, const fmpq_poly_t B, slong n)` | |

`inv_series` and `inv_series_newton` are the same function, and `div_series` computes
$$A/B \bmod x^n$$. All three require only a nonzero constant term in the divisor, where the
[integer chapter](/mapping/flint-integer-polynomials/) requires $$\pm 1$$, and abort otherwise.

## [Greatest common divisor](https://flintlib.org/doc/fmpq_poly.html#greatest-common-divisor) {#gcd}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_gcd (fmpq_poly_t G, const fmpq_poly_t A, const fmpq_poly_t B)` | |
| ✗ | `void fmpq_poly_xgcd (fmpq_poly_t G, fmpq_poly_t S, fmpq_poly_t T, const fmpq_poly_t A, const fmpq_poly_t B)` | |
| ✗ | `void fmpq_poly_lcm (fmpq_poly_t L, const fmpq_poly_t A, const fmpq_poly_t B)` | |
| ✗ | `void fmpq_poly_resultant (fmpq_t r, const fmpq_poly_t f, const fmpq_poly_t g)` | |
| — | `void fmpq_poly_resultant_div (fmpq_t r, const fmpq_poly_t f, const fmpq_poly_t g, const fmpz_t div, slong nbits)` | |

`resultant_div` is — because it asks the caller to supply facts about the result (exact
divisibility by `div`, a bit bound), an internal entry point rather than an operation.

## [Discriminant](https://flintlib.org/doc/fmpq_poly.html#discriminant) {#discriminant}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_discriminant (fmpq_t res, const fmpq_poly_t poly)` | |

For $$f$$ of degree $$n$$ the result is
$$(-1)^{n(n-1)/2} \operatorname{res}(f, f')/\operatorname{lc}(f)$$, for the polynomial as given
rather than its monic associate; for $$f = A/d$$ it equals $$\operatorname{disc}(A)/d^{2n-2}$$.
Constants and the zero polynomial have discriminant 0, and every linear polynomial has
discriminant 1.

## [Derivative and integral](https://flintlib.org/doc/fmpq_poly.html#derivative-and-integral) {#derivative-and-integral}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_derivative (fmpq_poly_t res, const fmpq_poly_t poly)` | [`derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Derivative.html#tymethod.derivative) |
| ✓ | `void fmpq_poly_nth_derivative (fmpq_poly_t res, const fmpq_poly_t poly, ulong n)` | [`nth_derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.NthDerivative.html#tymethod.nth_derivative) |
| ✓ | `void fmpq_poly_integral (fmpq_poly_t res, const fmpq_poly_t poly)` | [`integral`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Integral.html#tymethod.integral) |

The constant of integration is zero.

## [Square roots](https://flintlib.org/doc/fmpq_poly.html#square-roots) {#square-roots}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_sqrt_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_invsqrt_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |

Both require the constant term to be exactly 1 and abort otherwise, even when it is another
nonzero square; for a constant term $$c^2$$, use $$\sqrt{f} = c\sqrt{f/c^2}$$, so that
$$\sqrt{4 + x} = 2\sqrt{1 + x/4}$$.

## [Power sums](https://flintlib.org/doc/fmpq_poly.html#power-sums) {#power-sums}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_power_sums (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | |
| ✗ | `void fmpq_poly_power_sums_to_fmpz_poly (fmpz_poly_t res, const fmpq_poly_t Q)` | |
| ✗ | `void fmpq_poly_power_sums_to_poly (fmpq_poly_t res, const fmpq_poly_t Q)` | |

`power_sums` accepts any nonzero polynomial, not only a monic one. The two inverses differ only in
normalisation: `power_sums_to_poly` returns the monic polynomial and `power_sums_to_fmpz_poly` the
primitive one with positive leading coefficient, so either converts to the other with
[`make_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MakeMonic.html#tymethod.make_monic)
or
[`primitive_part`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.PrimitivePart.html#tymethod.primitive_part).
Both read the degree from the constant term $$p_0$$ without checking that it is a non-negative
integer.

## [Transcendental functions](https://flintlib.org/doc/fmpq_poly.html#transcendental-functions) {#transcendental}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_log_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_exp_series (fmpq_poly_t res, const fmpq_poly_t h, slong n)` | |
| ✗ | `void fmpq_poly_exp_expinv_series (fmpq_poly_t res1, fmpq_poly_t res2, const fmpq_poly_t h, slong n)` | |
| ✗ | `void fmpq_poly_atan_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_atanh_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_asin_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_asinh_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_tan_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_sin_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_cos_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_sin_cos_series (fmpq_poly_t res1, fmpq_poly_t res2, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_sinh_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_cosh_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_sinh_cosh_series (fmpq_poly_t res1, fmpq_poly_t res2, const fmpq_poly_t f, slong n)` | |
| ✗ | `void fmpq_poly_tanh_series (fmpq_poly_t res, const fmpq_poly_t f, slong n)` | |

All fifteen require constant term 0, except `log_series`, which requires constant term 1; these are
the only rational points at which the functions take rational values. `exp_expinv_series`,
`sin_cos_series` and `sinh_cosh_series` return two results from one call, as
[`SinCos`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.SinCos.html)
does for scalars.

## [Orthogonal polynomials](https://flintlib.org/doc/fmpq_poly.html#orthogonal-polynomials) {#orthogonal-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_legendre_p (fmpq_poly_t poly, ulong n)` | |
| ✗ | `void fmpq_poly_laguerre_l (fmpq_poly_t poly, ulong n)` | |
| ✗ | `void fmpq_poly_gegenbauer_c (fmpq_poly_t poly, ulong n, const fmpq_t a)` | |

These are the families with non-integral coefficients; Chebyshev, Hermite and shifted Legendre
polynomials are in the
[integer chapter](/mapping/flint-integer-polynomials/#orthogonal-polynomials). `gegenbauer_c`
takes a rational parameter $$\alpha$$, with $$C^{(1/2)}_n$$ the Legendre polynomial; its
documented domain $$\alpha > 0$$ is not enforced, and $$\alpha = 0$$ returns 0.

## [Evaluation](https://flintlib.org/doc/fmpq_poly.html#evaluation) {#evaluation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_evaluate_fmpz (fmpq_t res, const fmpq_poly_t poly, const fmpz_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |
| ✓ | `void fmpq_poly_evaluate_fmpq (fmpq_t res, const fmpq_poly_t poly, const fmpq_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |

[`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate)
is implemented at both a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) and an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), and
returns a `Rational` either way, as both FLINT functions do.

## [Interpolation](https://flintlib.org/doc/fmpq_poly.html#interpolation) {#interpolation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_interpolate_fast (fmpq_poly_t poly, const fmpq * xs, const fmpq * ys, slong n)` | |
| ✗ | `void fmpq_poly_interpolate_barycentric (fmpq_poly_t poly, const fmpq * xs, const fmpq * ys, slong n)` | |
| ✗ | `void fmpq_poly_interpolate_multi_mod (fmpq_poly_t poly, const fmpq * xs, const fmpq * ys, slong n)` | |
| ✗ | `int fmpq_poly_interpolate_fmpq_vec (fmpq_poly_t poly, const fmpq * xs, const fmpq * ys, slong n)` | |
| ✗ | `int fmpq_poly_interpolate_fmpz_fmpq_vec (fmpq_poly_t poly, const fmpz * xs, const fmpq * ys, slong n)` | |
| ✗ | `int fmpq_poly_interpolate_fmpz_vec (fmpq_poly_t poly, const fmpz * xs, const fmpz * ys, slong n)` | |

`fast`, `barycentric` and `multi_mod` return `void` and assume distinct $$x_i$$; the three
`interpolate_*_vec` functions return 0 when two $$x_i$$ coincide. The interpolant always exists
over $$\mathbb{Q}$$, so unlike
[`fmpz_poly_interpolate`](/mapping/flint-integer-polynomials/#interpolation) distinct points never
fail.

## [Composition](https://flintlib.org/doc/fmpq_poly.html#composition) {#composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_compose (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2)` | |
| ✗ | `void fmpq_poly_rescale (fmpq_poly_t res, const fmpq_poly_t poly, const fmpq_t a)` | |

When the inner polynomial is $$x^k$$, `compose` is
[`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x).

## [Power series composition](https://flintlib.org/doc/fmpq_poly.html#power-series-composition) {#power-series-composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_compose_series (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | |
| ✗ | `void fmpq_poly_compose_series_horner (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | |
| ✗ | `void fmpq_poly_compose_series_brent_kung (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | |
| ✗ | `void fmpq_poly_compose_series_kinoshita_li (fmpq_poly_t res, const fmpq_poly_t poly1, const fmpq_poly_t poly2, slong n)` | |

The inner polynomial must have zero constant term. Three sections place a condition on a
constant term, for three different reasons:

| | condition | why |
| --- | :--- | :--- |
| [Transcendental functions](#transcendental) | 0, or 1 for `log` | the function's value there must be rational |
| [Square roots](#square-roots) | 1 | the value there must be a *square* in $$\mathbb{Q}$$ |
| Power series composition | 0 | otherwise $$g(h) \bmod x^n$$ is not defined at all |

## [Power series reversion](https://flintlib.org/doc/fmpq_poly.html#power-series-reversion) {#power-series-reversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpq_poly_revert_series (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | |
| — | `void fmpq_poly_revert_series_lagrange (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | |
| ✗ | `void fmpq_poly_revert_series_lagrange_fast (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | |
| ✗ | `void fmpq_poly_revert_series_newton (fmpq_poly_t res, const fmpq_poly_t poly, slong n)` | |

`revert_series_lagrange` is — because FLINT's dispatcher never selects it.

## [Gaussian content](https://flintlib.org/doc/fmpq_poly.html#gaussian-content) {#gaussian-content}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpq_poly_content (fmpq_t res, const fmpq_poly_t poly)` | [`content`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Content.html#tymethod.content) |
| ≈ | `void fmpq_poly_primitive_part (fmpq_poly_t res, const fmpq_poly_t poly)` | [`primitive_part`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.PrimitivePart.html#tymethod.primitive_part) |
| ✓ | `int fmpq_poly_is_monic (const fmpq_poly_t poly)` | [`is_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Polynomial.html#tymethod.is_monic) |
| ✓ | `void fmpq_poly_make_monic (fmpq_poly_t res, const fmpq_poly_t poly)` | [`make_monic`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MakeMonic.html#tymethod.make_monic) |

The content is the non-negative rational $$c$$ for which $$f/c$$ is a primitive integer
polynomial, $$\operatorname{cont}(A)/d$$ for $$f = A/d$$. The primitive part has non-negative
leading coefficient, so
$$f = \operatorname{sgn}(\operatorname{lc}(f)) \cdot \operatorname{cont}(f) \cdot \operatorname{pp}(f)$$.
`primitive_part` is ≈ only in its type: the result always has denominator 1, and Malachite
returns an [`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html).
The zero polynomial is not monic, and `make_monic(0) = 0`.

## [Square-free](https://flintlib.org/doc/fmpq_poly.html#square-free) {#square-free}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpq_poly_is_squarefree (const fmpq_poly_t poly)` | |

The test is for repeated roots, that is, for a square factor of positive degree; the denominator
is irrelevant, and the zero polynomial counts as square-free.

## [Input and output](https://flintlib.org/doc/fmpq_poly.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpq_poly_print_pretty (const fmpq_poly_t poly, const char * var)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`to_string_with`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.to_string_with) |
| ✓ | `int fmpq_poly_fprint_pretty (FILE * file, const fmpq_poly_t poly, const char * var)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int fmpq_poly_print (const fmpq_poly_t poly)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html), [`to_coefficients_asc`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.to_coefficients_asc) |
| ≈ | `int fmpq_poly_fprint (FILE * file, const fmpq_poly_t poly)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int fmpq_poly_read (fmpq_poly_t poly)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html), [`from_coefficients_asc`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.from_coefficients_asc) |
| ≈ | `int fmpq_poly_fread (FILE * file, fmpq_poly_t poly)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |

The pretty format is
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) and
[`to_string_with`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.to_string_with),
which serve `stdout`, a file and a
[`String`](https://doc.rust-lang.org/nightly/alloc/string/struct.String.html) alike;
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) and
[`from_string_with`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.from_string_with)
read it back. The plain `print`/`read` format is a length followed by one rational per
coefficient (`4  1/3 -1/3 0 5/3` for $$(5x^3 - x + 1)/3$$), whereas Malachite serialises a
numerator polynomial and a denominator under the names `n` and `d`, hence ≈; to read or write
FLINT's format, go through
[`from_coefficients_asc`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.from_coefficients_asc)
and
[`to_coefficients_asc`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html#method.to_coefficients_asc).
