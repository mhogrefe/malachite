---
layout: default
title: "Malachite for FLINT Users: Integer Polynomials"
permalink: /mapping/flint-integer-polynomials/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Integer Polynomials

This page maps the functions of FLINT's integer polynomial type, `fmpz_poly_t`, onto their
Malachite counterpart:
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html),
from the `malachite-nz` crate. It follows the organization of the
[fmpz_poly.h chapter](https://flintlib.org/doc/fmpz_poly.html) of the FLINT manual, as of FLINT
3.6.0, and is a companion to [Malachite for FLINT Users: Integers](/mapping/flint-integers/),
whose [conventions](/mapping/flint-integers/#conventions), including word types and aliasing,
apply here as well; the [mapping index](/mapping/) lists the whole family.

Every section of the chapter is mapped below: 254 rows across 49 sections, of which 51 are ✓,
22 ⚙, 26 ≈, 25 —, and 130 ✗. Functions whose names begin with an underscore are omitted, as on
[the fmpq page](/mapping/flint-rationals/), except in [Newton basis](#newton-basis) and
[Subproduct trees](#subproduct-trees), which have no public functions.

## Conventions {#conventions}

### The `fmpz_poly` representation {#representation}

An `fmpz_poly_struct` holds `coeffs` (ascending, so `coeffs[i]` belongs to $$x^i$$), `alloc`, and
`length`. An
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html)
is a single
[`Vec<Integer>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html) in the same
ascending order; `alloc` and `length` are the
[`Vec`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html)'s own.

FLINT expects every input to be *normalised* (length zero or nonzero leading coefficient) and
leaves that to the caller. In Malachite it is an invariant: trailing zero coefficients are never
stored, the field is private, and
[`from_coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.from_coefficients_asc)
drops trailing zeros as it builds.

### Coefficients that are not `Integer`s

Malachite has four polynomial types differing only in their coefficients:
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html)
over $$\mathbb{Z}$$,
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
over $$\mathbb{N}$$,
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
with [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) coefficients (in
`malachite-base`), and
[`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html)
over $$\mathbb{Q}$$, the counterpart of `fmpq_poly` and the subject of its own page. Every row
below names the
[`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html)
version; the other types use the same method names wherever the coefficients allow it.

`NaturalPolynomial` and `UnsignedPolynomial` hold residues for modular arithmetic (the `Mod*` and
`ModPowerOf2*` traits), which is the only arithmetic `UnsignedPolynomial` has. Every modular
operation checks that its arguments are reduced and panics otherwise.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Types, macros and constants](https://flintlib.org/doc/fmpz_poly.html#types-macros-and-constants) {#types-macros-and-constants}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpz_poly_struct` | [`IntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html) |
| — | `fmpz_poly_t` | |

`fmpz_poly_t` is the pass-by-reference device that `&` and `&mut` already are.

## [Polynomial parameters](https://flintlib.org/doc/fmpz_poly.html#polynomial-parameters) {#polynomial-parameters}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `slong fmpz_poly_length (const fmpz_poly_t poly)` | [`len`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.len) |
| ≈ | `slong fmpz_poly_degree (const fmpz_poly_t poly)` | [`degree`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.degree) |

[`len`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.len)
returns a [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html). `fmpz_poly_degree`
returns $$-1$$ for the zero polynomial;
[`degree`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.degree)
returns
[`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)`<`[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html)`>`,
and the zero polynomial's degree is
[`None`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#variant.None), so a
comparison against $$-1$$ becomes a match on `None`.

## [Assignment and basic manipulation](https://flintlib.org/doc/fmpz_poly.html#assignment-and-basic-manipulation) {#assignment-and-basic-manipulation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_set (fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html) |
| ✓ | `void fmpz_poly_set_si (fmpz_poly_t poly, slong c)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpz_poly_set_ui (fmpz_poly_t poly, ulong c)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void fmpz_poly_set_fmpz (fmpz_poly_t poly, const fmpz_t c)` | [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ≈ | `int fmpz_poly_set_str (fmpz_poly_t poly, const char * str)` | [`from_coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.from_coefficients_asc), [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |
| ≈ | `char * fmpz_poly_get_str (const fmpz_poly_t poly)` | [`coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficients_asc), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ✓ | `char * fmpz_poly_get_str_pretty (const fmpz_poly_t poly, const char * x)` | [`to_string_with`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.to_string_with), [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ✓ | `void fmpz_poly_zero (fmpz_poly_t poly)` | [`IntegerPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html) |
| ✓ | `void fmpz_poly_one (fmpz_poly_t poly)` | [`one`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.one) |
| ✓ | `void fmpz_poly_zero_coeffs (fmpz_poly_t poly, slong i, slong j)` | [`zero_coefficients`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.zero_coefficients) |
| ✓ | `void fmpz_poly_swap (fmpz_poly_t poly1, fmpz_poly_t poly2)` | [`mem::swap`](https://doc.rust-lang.org/nightly/core/mem/fn.swap.html) |
| ✓ | `void fmpz_poly_reverse (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`reverse`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.reverse) |
| ✓ | `void fmpz_poly_truncate (fmpz_poly_t poly, slong newlen)` | [`truncate_assign`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.truncate_assign) |
| ✓ | `void fmpz_poly_set_trunc (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`truncate`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.truncate) |

`fmpz_poly_set` is [`Clone`](https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html), or
a move when the original is not needed afterwards. The three scalar constructors are one
generic [`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) over anything
convertible into an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).
[`IntegerPolynomial::ZERO`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.Zero.html)
is an associated constant, while
[`one`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.one)
(like
[`two`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.two),
[`negative_one`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.negative_one),
and
[`x`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.x))
is a function; the
[`One`](https://docs.rs/malachite-base/latest/malachite_base/num/basic/traits/trait.One.html)
trait is not implemented.

FLINT's *plain* string format is a serialization (the length, then the coefficients in ascending
order, so $$5x^3 - 1$$ is `4  -1 0 0 5`, with the zero polynomial written `0`). Malachite has no
such format; the counterparts are
[`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) and
[`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) under the
`enable_serde` feature, or the pair
[`coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficients_asc)
/
[`from_coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.from_coefficients_asc),
which carry no length and cannot fail. The *pretty* format agrees character for character with
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) when the variable is
`x`: $$5x^3 - 1$$ is `5*x^3-1` in both. FLINT takes the variable as an arbitrary C string;
[`to_string_with`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.to_string_with)
takes a [`Var`](https://docs.rs/malachite-base/latest/malachite_base/vars/struct.Var.html) from a
[`VarScheme`](https://docs.rs/malachite-base/latest/malachite_base/vars/trait.VarScheme.html)
(for instance
[`ListVars::new`](https://docs.rs/malachite-base/latest/malachite_base/vars/list/struct.ListVars.html#method.new)`(["t"])`),
whose names may not contain an ASCII digit, whitespace, or `+ - * / ^ ( ) ,` (see
[`char_is_reserved`](https://docs.rs/malachite-base/latest/malachite_base/vars/fn.char_is_reserved.html)).

[`zero_coefficients`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.zero_coefficients)
takes [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) indices and panics if
`start` is greater than `end`. When `res` and `poly` are the same polynomial, `fmpz_poly_reverse`
is
[`reverse_assign`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.reverse_assign).
Malachite's `truncate` returns a new polynomial, as `set_trunc` does, rather than shortening in
place as [`Vec::truncate`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html#method.truncate)
does; the in-place form is `truncate_assign`.

## [Randomisation](https://flintlib.org/doc/fmpz_poly.html#randomisation) {#randomisation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_poly_randtest (fmpz_poly_t f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_integer_polynomials`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/random/fn.random_integer_polynomials.html), [`striped_random_integer_polynomials`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/random/fn.striped_random_integer_polynomials.html) |
| ≈ | `void fmpz_poly_randtest_unsigned (fmpz_poly_t f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_natural_polynomials`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/random/fn.random_natural_polynomials.html) |
| ≈ | `void fmpz_poly_randtest_not_zero (fmpz_poly_t f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_integer_polynomials_min_degree`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/random/fn.random_integer_polynomials_min_degree.html) |
| ✗ | `void fmpz_poly_randtest_no_real_root (fmpz_poly_t p, flint_rand_t state, slong len, flint_bitcnt_t bits)` | |
| ✗ | `void fmpz_poly_randtest_irreducible1 (fmpz_poly_t pol, flint_rand_t state, slong len, flint_bitcnt_t bits)` | |
| ✗ | `void fmpz_poly_randtest_irreducible2 (fmpz_poly_t pol, flint_rand_t state, slong len, flint_bitcnt_t bits)` | |
| ✗ | `void fmpz_poly_randtest_irreducible (fmpz_poly_t pol, flint_rand_t state, slong len, flint_bitcnt_t bits)` | |

FLINT's generators take *bounds* (`len` caps the length, `bits` caps each coefficient's bit
length); Malachite's take *means*, sampling the degree and the coefficient bit length from
geometric distributions with unbounded support, so every row here is ≈ at best. The degree can
be constrained with the `_with_degree`, `_min_degree`, `_degree_range`, and
`_degree_inclusive_range` variants, and
[`_from_iterators`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/random/fn.random_integer_polynomials_from_iterators.html)
takes arbitrary coefficient and leading-coefficient iterators. `randtest_unsigned` is a change of
type, to
[`random_natural_polynomials`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/random/fn.random_natural_polynomials.html).
`randtest_not_zero` is
[`random_integer_polynomials_min_degree`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/random/fn.random_integer_polynomials_min_degree.html)
with minimum degree 0.

## [Getting and setting coefficients](https://flintlib.org/doc/fmpz_poly.html#getting-and-setting-coefficients) {#getting-and-setting-coefficients}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_get_coeff_fmpz (fmpz_t x, const fmpz_poly_t poly, slong n)` | [`coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficient) |
| ≈ | `slong fmpz_poly_get_coeff_si (const fmpz_poly_t poly, slong n)` | [`coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficient), [`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html) |
| ≈ | `ulong fmpz_poly_get_coeff_ui (const fmpz_poly_t poly, slong n)` | [`coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficient), [`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html) |
| ≈ | `fmpz * fmpz_poly_get_coeff_ptr (const fmpz_poly_t poly, slong n)` | [`coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficient), [`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.mutate_coefficient) |
| ≈ | `fmpz * fmpz_poly_lead (const fmpz_poly_t poly)` | [`leading_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.leading_coefficient) |
| ≈ | `void fmpz_poly_set_coeff_fmpz (fmpz_poly_t poly, slong n, const fmpz_t x)` | [`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.mutate_coefficient) |
| ≈ | `void fmpz_poly_set_coeff_si (fmpz_poly_t poly, slong n, slong x)` | [`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.mutate_coefficient) |
| ≈ | `void fmpz_poly_set_coeff_ui (fmpz_poly_t poly, slong n, ulong x)` | [`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.mutate_coefficient) |

The four getters collapse onto
[`coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficient),
which returns `&`[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
in constant time; `_si` and `_ui`, whose result FLINT leaves undefined when the coefficient does
not fit, become `i64::try_from(p.coefficient(n))` or the
[`WrappingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.WrappingFrom.html)
/
[`SaturatingFrom`](https://docs.rs/malachite-base/latest/malachite_base/num/conversion/traits/trait.SaturatingFrom.html)
family. Past the degree, `get_coeff_fmpz`, `_si`, and `_ui` return zero but `get_coeff_ptr`
returns `NULL`, and `fmpz_poly_lead` returns `NULL` for the zero polynomial; Malachite returns a
reference to a static zero in all of those cases, including
[`leading_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.leading_coefficient)
of the zero polynomial, so a translated `NULL` check becomes a comparison against zero.

The three setters and in-place mutation through `get_coeff_ptr` are all
[`mutate_coefficient`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.mutate_coefficient),
which takes a closure over `&mut `[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)
(assignment is `p.mutate_coefficient(n, |c| *c = x)`) so that the invariant can be restored after
the mutation. The index may run past the degree; the polynomial grows to reach it and trailing
zeros are dropped afterwards.

## [Comparison](https://flintlib.org/doc/fmpz_poly.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_poly_equal (const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |
| ✓ | `int fmpz_poly_equal_trunc (const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`eq_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.EqTruncated.html#tymethod.eq_truncated) |
| ✓ | `int fmpz_poly_is_zero (const fmpz_poly_t poly)` | `p == 0u32` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ✓ | `int fmpz_poly_is_one (const fmpz_poly_t poly)` | `p == 1u32` ([`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html)) |
| ✓ | `int fmpz_poly_is_unit (const fmpz_poly_t poly)` | [`IsUnit`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.IsUnit.html) |
| ✓ | `int fmpz_poly_is_gen (const fmpz_poly_t poly)` | [`x`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.x) |

An `IntegerPolynomial` compares with a value of any primitive integer type, in either order:
`p == c` holds exactly when `p` is the constant polynomial `c`. So `is_zero` and `is_one` are
`*p == 0u32` and `*p == 1u32`, and `is_gen` is `*p == IntegerPolynomial::x()`. For
`equal_trunc`, comparing `coefficients_asc().iter().take(n)` with
[`Iterator::eq`](https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html#method.eq) is
wrong because it also compares lengths; use
[`eq_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.EqTruncated.html#tymethod.eq_truncated).
FLINT has no `fmpz_poly_cmp`; Malachite's default
[`Ord`](https://doc.rust-lang.org/nightly/core/cmp/trait.Ord.html) is asymptotic ($$p < q$$ when
$$p(x) < q(x)$$ for all sufficiently large $$x$$), and
[`ShortlexIntegerPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.ShortlexIntegerPolynomial.html)
compares degree first and then coefficients from the leading one down, as `fmpq_poly_cmp` does.

## [Addition and subtraction](https://flintlib.org/doc/fmpz_poly.html#addition-and-subtraction) {#addition-and-subtraction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_add (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`Add`](https://doc.rust-lang.org/nightly/core/ops/trait.Add.html) |
| ✓ | `void fmpz_poly_add_series (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.AddTruncated.html#tymethod.add_truncated) |
| ✓ | `void fmpz_poly_sub (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`Sub`](https://doc.rust-lang.org/nightly/core/ops/trait.Sub.html) |
| ✓ | `void fmpz_poly_sub_series (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SubTruncated.html#tymethod.sub_truncated) |
| ✓ | `void fmpz_poly_neg (fmpz_poly_t res, const fmpz_poly_t poly)` | [`Neg`](https://doc.rust-lang.org/nightly/core/ops/trait.Neg.html) |

Each operator takes either operand by value or by reference and has an `Assign` form
([`NegAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.NegAssign.html)
for negation). FLINT's `_series` suffix becomes `_truncated` throughout this page:
[`add_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.AddTruncated.html#tymethod.add_truncated)
and
[`sub_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SubTruncated.html#tymethod.sub_truncated)
read only the first `len` coefficients of each operand. FLINT clamps a negative `n` to zero;
`len` is a `u64`, so it cannot be negative.

## [Scalar absolute value, multiplication and division](https://flintlib.org/doc/fmpz_poly.html#scalar-absolute-value-multiplication-and-division) {#scalar}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_scalar_abs (fmpz_poly_t res, const fmpz_poly_t poly)` | |
| ✗ | `void fmpz_poly_scalar_mul_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t x)` | |
| ✗ | `void fmpz_poly_scalar_mul_si (fmpz_poly_t poly1, const fmpz_poly_t poly2, slong x)` | |
| ✗ | `void fmpz_poly_scalar_mul_ui (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong x)` | |
| ✓ | `void fmpz_poly_scalar_mul_2exp (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong exp)` | [`Shl`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#impl-Shl%3Cu64%3E-for-%26IntegerPolynomial) |
| ✗ | `void fmpz_poly_scalar_addmul_si (fmpz_poly_t poly1, const fmpz_poly_t poly2, slong x)` | |
| ✗ | `void fmpz_poly_scalar_addmul_ui (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong x)` | |
| ✗ | `void fmpz_poly_scalar_addmul_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t x)` | |
| ✗ | `void fmpz_poly_scalar_submul_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t x)` | |
| ✗ | `void fmpz_poly_scalar_fdiv_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t x)` | |
| ✗ | `void fmpz_poly_scalar_fdiv_si (fmpz_poly_t poly1, const fmpz_poly_t poly2, slong x)` | |
| ✗ | `void fmpz_poly_scalar_fdiv_ui (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong x)` | |
| ✗ | `void fmpz_poly_scalar_fdiv_2exp (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong x)` | |
| ✗ | `void fmpz_poly_scalar_tdiv_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t x)` | |
| ✗ | `void fmpz_poly_scalar_tdiv_si (fmpz_poly_t poly1, const fmpz_poly_t poly2, slong x)` | |
| ✗ | `void fmpz_poly_scalar_tdiv_ui (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong x)` | |
| ✗ | `void fmpz_poly_scalar_tdiv_2exp (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong x)` | |
| ✓ | `void fmpz_poly_scalar_divexact_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t x)` | [`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html) |
| ✗ | `void fmpz_poly_scalar_divexact_si (fmpz_poly_t poly1, const fmpz_poly_t poly2, slong x)` | |
| ✗ | `void fmpz_poly_scalar_divexact_ui (fmpz_poly_t poly1, const fmpz_poly_t poly2, ulong x)` | |
| ✓ | `void fmpz_poly_scalar_mod_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t p)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✓ | `void fmpz_poly_scalar_smod_fmpz (fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t p)` | [`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html) |

`scalar_mul_2exp` is `<<` by a
[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html): throughout Malachite `<<` and
`>>` scale by a power of two and never mean multiplication by $$x^k$$ (see
[Shifting](#shifting)). `scalar_divexact_fmpz` is
[`DivExact`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.DivExact.html)
by an [`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html).

`scalar_mod_fmpz` reduces each coefficient into $$[0, p)$$. On `IntegerPolynomial`,
[`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html)
and
[`ModPowerOf2`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModPowerOf2.html)
take a positive [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html)
or a power of 2 and return a
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html),
not an `IntegerPolynomial`.
[`Rem`](https://doc.rust-lang.org/nightly/core/ops/trait.Rem.html) instead takes an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), returns
an `IntegerPolynomial`, and keeps each coefficient's sign (the remainder of `scalar_tdiv_fmpz`,
which FLINT does not provide). `scalar_smod_fmpz` takes the representative in $$(-p/2, p/2]$$, as
[`BalancedMod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.BalancedMod.html)
does.

## [Bit packing](https://flintlib.org/doc/fmpz_poly.html#bit-packing) {#bit-packing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void fmpz_poly_bit_pack (fmpz_t f, const fmpz_poly_t poly, flint_bitcnt_t bit_size)` | [`bit_pack`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.BitPack.html#tymethod.bit_pack) |
| ≈ | `void fmpz_poly_bit_unpack (fmpz_poly_t poly, const fmpz_t f, flint_bitcnt_t bit_size)` | [`bit_unpack`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.BitUnpack.html#tymethod.bit_unpack) |
| ≈ | `void fmpz_poly_bit_unpack_unsigned (fmpz_poly_t poly, const fmpz_t f, flint_bitcnt_t bit_size)` | [`bit_unpack`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.BitUnpack.html#tymethod.bit_unpack) |

[`bit_pack`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.BitPack.html#tymethod.bit_pack)
always returns $$p(2^b)$$, hence ≈: it agrees with `fmpz_poly_bit_pack` when
$$b > 0$$ and every coefficient's absolute value is less than $$2^b$$, but FLINT gives 0 for
$$b = 0$$ and truncates wider coefficients, while `bit_pack` lets them overlap the fields above.
[`bit_unpack`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.BitUnpack.html#tymethod.bit_unpack) reads the fields as signed on
`IntegerPolynomial`, exactly as `fmpz_poly_bit_unpack` does (each negative field borrows from the
one above, and a negative input negates every coefficient), and as unsigned on
`NaturalPolynomial`, as `fmpz_poly_bit_unpack_unsigned` does; both are ≈ only because they panic
for $$b = 0$$, where FLINT returns the zero polynomial.

## [Multiplication](https://flintlib.org/doc/fmpz_poly.html#multiplication) {#multiplication}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_mul (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`*`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ⚙ | `void fmpz_poly_mul_classical (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`*`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ⚙ | `void fmpz_poly_mul_karatsuba (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`*`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ⚙ | `void fmpz_poly_mul_KS (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`*`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ⚙ | `void fmpz_poly_mul_SS (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | [`*`](https://doc.rust-lang.org/nightly/core/ops/trait.Mul.html) |
| ✓ | `void fmpz_poly_mullow (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated) |
| ⚙ | `void fmpz_poly_mullow_classical (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated) |
| ⚙ | `void fmpz_poly_mullow_karatsuba_n (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated) |
| ⚙ | `void fmpz_poly_mullow_KS (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated) |
| ⚙ | `void fmpz_poly_mullow_SS (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | [`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated) |
| ✗ | `void fmpz_poly_mulhigh_n (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | |
| ✗ | `void fmpz_poly_mulhigh_classical (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong start)` | |
| ✗ | `void fmpz_poly_mulhigh_karatsuba_n (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong len)` | |
| ✗ | `void fmpz_poly_mulmid (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_mulmid_classical (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_mulmid_KS (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_mulmid_SS (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |

Malachite chooses the multiplication algorithm itself, so the `_classical`, `_karatsuba`, `_KS`,
and `_SS` functions map to the same operation as their dispatcher: `*` (with `*=`) for the whole
product, and
[`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated)
(with `mul_truncated_assign`) for the low $$n$$ coefficients.

## [FFT precached multiplication](https://flintlib.org/doc/fmpz_poly.html#fft-precached-multiplication) {#fft-precached}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void fmpz_poly_mul_SS_precache_init (fmpz_poly_mul_precache_t pre, slong len1, slong bits1, const fmpz_poly_t poly2)` | |
| — | `void fmpz_poly_mul_precache_clear (fmpz_poly_mul_precache_t pre)` | |
| — | `void fmpz_poly_mul_SS_precache (fmpz_poly_t res, const fmpz_poly_t poly1, fmpz_poly_mul_precache_t pre)` | |
| — | `void fmpz_poly_mullow_SS_precache (fmpz_poly_t res, const fmpz_poly_t poly1, fmpz_poly_mul_precache_t pre, slong n)` | |

These cache the transform of a fixed operand for one algorithm. Malachite does not expose its
multiplication algorithms, so there is nothing to cache; `_precache_clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html).

## [Squaring](https://flintlib.org/doc/fmpz_poly.html#squaring) {#squaring}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_sqr (fmpz_poly_t rop, const fmpz_poly_t op)` | [`square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html#tymethod.square) |
| ⚙ | `void fmpz_poly_sqr_classical (fmpz_poly_t rop, const fmpz_poly_t op)` | [`square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html#tymethod.square) |
| ⚙ | `void fmpz_poly_sqr_karatsuba (fmpz_poly_t rop, const fmpz_poly_t op)` | [`square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html#tymethod.square) |
| ⚙ | `void fmpz_poly_sqr_KS (fmpz_poly_t rop, const fmpz_poly_t op)` | [`square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html#tymethod.square) |
| ✓ | `void fmpz_poly_sqrlow (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SquareTruncated.html#tymethod.square_truncated) |
| ⚙ | `void fmpz_poly_sqrlow_classical (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SquareTruncated.html#tymethod.square_truncated) |
| ⚙ | `void fmpz_poly_sqrlow_karatsuba_n (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SquareTruncated.html#tymethod.square_truncated) |
| ⚙ | `void fmpz_poly_sqrlow_KS (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SquareTruncated.html#tymethod.square_truncated) |

As with [multiplication](#multiplication), the algorithm variants map to
[`square`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Square.html#tymethod.square)
(with `square_assign`) and
[`square_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.SquareTruncated.html#tymethod.square_truncated)
(with `square_truncated_assign`).

## [Powering](https://flintlib.org/doc/fmpz_poly.html#powering) {#powering}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_pow (fmpz_poly_t res, const fmpz_poly_t poly, ulong e)` | [`pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html#tymethod.pow) |
| ⚙ | `void fmpz_poly_pow_multinomial (fmpz_poly_t res, const fmpz_poly_t poly, ulong e)` | [`pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html#tymethod.pow) |
| ⚙ | `void fmpz_poly_pow_binomial (fmpz_poly_t res, const fmpz_poly_t poly, ulong e)` | [`pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html#tymethod.pow) |
| ⚙ | `void fmpz_poly_pow_addchains (fmpz_poly_t res, const fmpz_poly_t poly, ulong e)` | [`pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html#tymethod.pow) |
| ⚙ | `void fmpz_poly_pow_binexp (fmpz_poly_t res, const fmpz_poly_t poly, ulong e)` | [`pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html#tymethod.pow) |
| ✓ | `void fmpz_poly_pow_trunc (fmpz_poly_t res, const fmpz_poly_t poly, ulong e, slong n)` | [`pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.PowTruncated.html#tymethod.pow_truncated) |

[`Pow`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Pow.html)
and
[`PowAssign`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.PowAssign.html)
take a [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) exponent. `pow`
chooses among several algorithms itself, so FLINT's four algorithm-specific functions all map to
it, without the domain restrictions of `pow_binomial` (length exactly 2) or `pow_addchains`
($$e \leq 148$$).
[`PowTruncatedAssign`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.PowTruncatedAssign.html)
is the in-place form of `pow_truncated`, which reads only the first `len` coefficients. Both
libraries take $$0^0 = 1$$.

## [Shifting](https://flintlib.org/doc/fmpz_poly.html#shifting) {#shifting}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_shift_left (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`mul_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulPowerOfX.html#tymethod.mul_power_of_x) |
| ✓ | `void fmpz_poly_shift_right (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | [`div_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DivPowerOfX.html#tymethod.div_power_of_x) |

`div_power_of_x`, like `shift_right`, discards the low coefficients, and yields zero when `n` is
at or beyond the length. Neither operation is `<<` or `>>`, which throughout Malachite scale by a
power of two (see [the scalar section](#scalar)).

## [Bit sizes and norms](https://flintlib.org/doc/fmpz_poly.html#bit-sizes-and-norms) {#bit-sizes-and-norms}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `ulong fmpz_poly_max_limbs (const fmpz_poly_t poly)` | |
| ≈ | `slong fmpz_poly_max_bits (const fmpz_poly_t poly)` | [`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits) |
| ✓ | `void fmpz_poly_height (fmpz_t height, const fmpz_poly_t poly)` | [`Height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html), [`HeightRef`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.HeightRef.html) |
| ✓ | `void fmpz_poly_2norm (fmpz_t res, const fmpz_poly_t poly)` | [`floor_l2_norm`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.FloorL2Norm.html#tymethod.floor_l2_norm) |

`fmpz_poly_height` is
[`Height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html)'s
[`to_height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.to_height)
or
[`into_height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.into_height);
[`HeightRef`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.HeightRef.html)
returns it by reference. `max_bits` returns the bit length $$b$$ of the height, *negated* when any
coefficient is negative;
[`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits)
returns $$|b|$$ as a plain
[`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) (0 for the zero polynomial)
and the sign flag must come from elsewhere, hence ≈. `max_limbs` is a memory-estimation helper;
limbs are an implementation detail in Malachite. `2norm` (the integer square root of the sum of
squared coefficients) is
[`floor_l2_norm`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.FloorL2Norm.html#tymethod.floor_l2_norm);
the exact sum of squares is
[`l2_norm_squared`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.L2NormSquared.html#tymethod.l2_norm_squared).

## [Greatest common divisor](https://flintlib.org/doc/fmpz_poly.html#greatest-common-divisor) {#gcd}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_gcd (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_gcd_subresultant (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `int fmpz_poly_gcd_heuristic (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_gcd_modular (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_lcm (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_xgcd (fmpz_t r, fmpz_poly_t s, fmpz_poly_t t, const fmpz_poly_t f, const fmpz_poly_t g)` | |
| ✗ | `void fmpz_poly_xgcd_modular (fmpz_t r, fmpz_poly_t s, fmpz_poly_t t, const fmpz_poly_t f, const fmpz_poly_t g)` | |
| ✗ | `void fmpz_poly_resultant (fmpz_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_resultant_euclidean (fmpz_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_resultant_modular (fmpz_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| — | `void fmpz_poly_resultant_modular_div (fmpz_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, const fmpz_t div, slong nbits)` | |
| ✗ | `void fmpz_poly_squarefree_part (fmpz_poly_t res, const fmpz_poly_t poly)` | |

`fmpz_poly_xgcd` is not an extended gcd: it sets $$r$$ to the resultant of $$f$$ and $$g$$ and
finds $$s, t$$ with $$sf + tg = r$$, so it does not correspond to
[`ExtendedGcd`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ExtendedGcd.html)
on the scalar types. `resultant_modular_div` is — because it asks the caller to supply
unverifiable facts (that `div` divides the resultant exactly and that the result fits in `nbits`)
in exchange for skipping a bound computation.

## [Discriminant](https://flintlib.org/doc/fmpz_poly.html#discriminant) {#discriminant}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_discriminant (fmpz_t res, const fmpz_poly_t poly)` | |

FLINT normalises the discriminant as $$(-1)^{n(n-1)/2} \operatorname{res}(f, f') /
\operatorname{lc}(f)$$, with $$n$$ the degree, and gives 0 for the zero polynomial and for every
constant.

## [Gaussian content](https://flintlib.org/doc/fmpz_poly.html#gaussian-content) {#gaussian-content}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_content (fmpz_t res, const fmpz_poly_t poly)` | [`content`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Content.html#tymethod.content) |
| ✓ | `void fmpz_poly_primitive_part (fmpz_poly_t res, const fmpz_poly_t poly)` | [`primitive_part`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.PrimitivePart.html#tymethod.primitive_part) |

Malachite's content is a
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html). The
primitive part follows FLINT's sign convention (non-negative leading coefficient), so for a
negative leading coefficient the identity is

$$f = \operatorname{sgn}(\operatorname{lc}(f)) \cdot \operatorname{cont}(f) \cdot \operatorname{pp}(f).$$

Both functions return zero for the zero polynomial.
[`content_and_primitive_part`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ContentAndPrimitivePart.html#tymethod.content_and_primitive_part)
replaces a call to both.

## [Square-free](https://flintlib.org/doc/fmpz_poly.html#square-free) {#square-free}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_poly_is_squarefree (const fmpz_poly_t poly)` | |

`fmpz_poly_is_squarefree` tests for repeated roots, that is, square-freeness over
$$\mathbb{Q}[x]$$: it checks that $$\gcd(f, f')$$ is constant, so $$4$$ and $$4x + 4$$ pass, and
the zero polynomial passes by fiat. This is the condition that [`fmpz_poly_signature`](#signature)
requires of its input.

## [Euclidean division](https://flintlib.org/doc/fmpz_poly.html#euclidean-division) {#euclidean-division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_divrem (fmpz_poly_t Q, fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_divrem_basecase (fmpz_poly_t Q, fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_divrem_divconquer (fmpz_poly_t Q, fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_div (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_div_basecase (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_div_divconquer (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_rem (fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_rem_basecase (fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_divexact (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_div_root_fmpz (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_t c)` | |
| ✗ | `void fmpz_poly_divexact_root_fmpq (fmpz_poly_t Q, const fmpz_poly_t A, const fmpq_t c)` | |

Over $$\mathbb{Z}[x]$$ FLINT's `divrem` is $$A = BQ + R$$ where each coefficient of $$R$$ beyond
$$\operatorname{len}(B) - 1$$ is reduced modulo the leading coefficient of $$B$$, so the remainder
may keep high-degree terms; see also [pseudo-division](#pseudo-division).

## [Division with precomputed inverse](https://flintlib.org/doc/fmpz_poly.html#division-with-precomputed-inverse) {#division-preinv}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_preinvert (fmpz_poly_t B_inv, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_div_preinv (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B, const fmpz_poly_t B_inv)` | |
| ✗ | `void fmpz_poly_divrem_preinv (fmpz_poly_t Q, fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B, const fmpz_poly_t B_inv)` | |
| ✗ | `void fmpz_poly_powers_precompute (fmpz_poly_powers_precomp_t pinv, fmpz_poly_t poly)` | |
| ✗ | `void fmpz_poly_rem_powers_precomp (fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B, fmpz_poly_powers_precomp_t B_inv)` | |
| — | `void fmpz_poly_powers_clear (fmpz_poly_powers_precomp_t pinv)` | |

`powers_clear` frees the powers table, which is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html).

## [Divisibility testing](https://flintlib.org/doc/fmpz_poly.html#divisibility-testing) {#divisibility-testing}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_poly_divides (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `slong fmpz_poly_remove (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |

`fmpz_poly_divides` returns a flag and, on success, the quotient. `fmpz_poly_remove` returns the
quotient through `res` and the exponent as its return value, and raises an exception for a
divisor of 0 or $$\pm 1$$, the inputs on which the scalar
[`RemovePower`](https://docs.rs/malachite-base/latest/malachite_base/num/factorization/traits/trait.RemovePower.html)
panics.

## [Division mod p](https://flintlib.org/doc/fmpz_poly.html#division-mod-p) {#division-mod-p}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `void fmpz_poly_divlow_smodp (fmpz * res, const fmpz_poly_t f, const fmpz_poly_t g, const fmpz_t p, slong n)` | |
| — | `void fmpz_poly_divhigh_smodp (fmpz * res, const fmpz_poly_t f, const fmpz_poly_t g, const fmpz_t p, slong n)` | |

Both are, in FLINT's words, "a bespoke function used by factoring": internal helpers with
unchecked preconditions, so they are —.

## [Power series division](https://flintlib.org/doc/fmpz_poly.html#power-series-division) {#power-series-division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_inv_series (fmpz_poly_t Qinv, const fmpz_poly_t Q, slong n)` | |
| ✗ | `void fmpz_poly_inv_series_basecase (fmpz_poly_t Qinv, const fmpz_poly_t Q, slong n)` | |
| ✗ | `void fmpz_poly_inv_series_newton (fmpz_poly_t Qinv, const fmpz_poly_t Q, slong n)` | |
| ✗ | `void fmpz_poly_div_series (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B, slong n)` | |
| ✗ | `void fmpz_poly_div_series_basecase (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B, slong n)` | |
| ✗ | `void fmpz_poly_div_series_divconquer (fmpz_poly_t Q, const fmpz_poly_t A, const fmpz_poly_t B, slong n)` | |

These work in $$\mathbb{Z}[[x]]/(x^n)$$ and require the constant term of the series being
inverted (`Q` or `B`) to be $$\pm 1$$, the only units of $$\mathbb{Z}$$, without checking it.

## [Pseudo division](https://flintlib.org/doc/fmpz_poly.html#pseudo-division) {#pseudo-division}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_pseudo_divrem (fmpz_poly_t Q, fmpz_poly_t R, ulong * d, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_pseudo_divrem_basecase (fmpz_poly_t Q, fmpz_poly_t R, ulong * d, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_pseudo_divrem_divconquer (fmpz_poly_t Q, fmpz_poly_t R, ulong * d, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_pseudo_divrem_cohen (fmpz_poly_t Q, fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_pseudo_div (fmpz_poly_t Q, ulong * d, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_pseudo_rem (fmpz_poly_t R, ulong * d, const fmpz_poly_t A, const fmpz_poly_t B)` | |
| ✗ | `void fmpz_poly_pseudo_rem_cohen (fmpz_poly_t R, const fmpz_poly_t A, const fmpz_poly_t B)` | |

Pseudo-division computes $$Q$$, $$R$$, and $$d$$ with $$\ell^d A = BQ + R$$ and $$\deg R <
\deg B$$, where $$\ell$$ is the leading coefficient of $$B$$:

| | dividend | remainder |
| --- | :--- | :--- |
| [`divrem`](#euclidean-division) | $$A$$ itself | may keep high-degree terms, reduced modulo $$\ell$$ |
| `pseudo_divrem` | scaled to $$\ell^d A$$ | $$\deg R < \deg B$$, as in a field |

## [Derivative](https://flintlib.org/doc/fmpz_poly.html#derivative) {#derivative}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_derivative (fmpz_poly_t res, const fmpz_poly_t poly)` | [`derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Derivative.html#tymethod.derivative) |
| ✓ | `void fmpz_poly_nth_derivative (fmpz_poly_t res, const fmpz_poly_t poly, ulong n)` | [`nth_derivative`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.NthDerivative.html#tymethod.nth_derivative) |

Both return zero when the length is at most `n` (at most 1 for `derivative`). The chapter has no
integral, because integration divides by $$i + 1$$ and leaves $$\mathbb{Z}$$; `fmpq_poly` has one.

## [Evaluation](https://flintlib.org/doc/fmpz_poly.html#evaluation) {#evaluation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_evaluate_fmpz (fmpz_t res, const fmpz_poly_t f, const fmpz_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |
| ⚙ | `void fmpz_poly_evaluate_horner_fmpz (fmpz_t res, const fmpz_poly_t f, const fmpz_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |
| ⚙ | `void fmpz_poly_evaluate_divconquer_fmpz (fmpz_t res, const fmpz_poly_t poly, const fmpz_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |
| ✓ | `void fmpz_poly_evaluate_fmpq (fmpq_t res, const fmpz_poly_t f, const fmpq_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |
| ⚙ | `void fmpz_poly_evaluate_horner_fmpq (fmpq_t res, const fmpz_poly_t f, const fmpq_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |
| ⚙ | `void fmpz_poly_evaluate_divconquer_fmpq (fmpq_t res, const fmpz_poly_t poly, const fmpq_t a)` | [`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate) |
| ≈ | `ulong fmpz_poly_evaluate_mod (const fmpz_poly_t poly, ulong a, ulong n)` | [`mod_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluate.html#tymethod.mod_evaluate) |
| ✓ | `void fmpz_poly_evaluate_fmpz_vec (fmpz * res, const fmpz_poly_t f, const fmpz * a, slong n)` | [`evaluate_many`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.EvaluateMany.html#tymethod.evaluate_many) |
| — | `double fmpz_poly_evaluate_horner_d (const fmpz_poly_t poly, double d)` | |
| — | `double fmpz_poly_evaluate_horner_d_2exp (slong * exp, const fmpz_poly_t poly, double d)` | |

[`evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.Evaluate.html#tymethod.evaluate)
chooses between Horner's rule and divide and conquer itself, so the `_horner` and `_divconquer`
rows map to it as well. Its result type is its argument type: at an
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) it
returns an `Integer`, and at a
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) (the
`_fmpq` rows) a `Rational` in lowest terms.
[`mod_evaluate`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ModEvaluate.html#tymethod.mod_evaluate)
evaluates at a [`u64`](https://doc.rust-lang.org/nightly/core/primitive.u64.html) modulo a `u64`,
reducing each coefficient (negative ones into $$[0, n)$$) as FLINT does, but it panics unless `a`
is already reduced, hence ≈. The two `double` rows are — because FLINT itself says they make "no
attempt" at efficiency or numerical stability and exist only for quick evaluations of polynomials
with positive coefficients.

## [Newton basis](https://flintlib.org/doc/fmpz_poly.html#newton-basis) {#newton-basis}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void _fmpz_poly_monomial_to_newton (fmpz * poly, const fmpz * roots, slong n)` | |
| ✗ | `void _fmpz_poly_newton_to_monomial (fmpz * poly, const fmpz * roots, slong n)` | |

These convert a coefficient list in place between the monomial basis and the Newton basis
$$1,\; (x - r_0),\; (x - r_0)(x - r_1),\; \ldots$$ for a caller-supplied root sequence. The
conversion is integral in both directions with no precondition, and both directions must be
given the same `roots`.

## [Interpolation](https://flintlib.org/doc/fmpz_poly.html#interpolation) {#interpolation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_poly_interpolate (fmpz_poly_t poly, const fmpz * xs, const fmpz * ys, slong n)` | |
| ✗ | `int fmpz_poly_interpolate_newton (fmpz_poly_t poly, const fmpz * xs, const fmpz * ys, slong n)` | |
| ✗ | `int fmpz_poly_interpolate_multi_mod (fmpz_poly_t poly, const fmpz * xs, const fmpz * ys, slong n)` | |
| — | `void fmpz_poly_interpolate_exact (fmpz_poly_t poly, const fmpz * xs, const fmpz * ys, slong n)` | |
| — | `void fmpz_poly_interpolate_exact_newton (fmpz_poly_t poly, const fmpz * xs, const fmpz * ys, slong n)` | |
| — | `void fmpz_poly_interpolate_fmpz_vec (fmpz_poly_t poly, const fmpz * xs, const fmpz * ys, slong n)` | |

An interpolant with integer coefficients may not exist. On failure `interpolate` returns 0,
`interpolate_exact` leaves the behaviour undefined, and `interpolate_fmpz_vec` throws
`FLINT_INEXACT`. The `_exact` and `_fmpz_vec` rows are — because they differ from `interpolate`
only in error discipline, which in Rust is the caller's choice of `match`,
[`unwrap`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#method.unwrap), or
[`unwrap_unchecked`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html#method.unwrap_unchecked)
on one [`Option`](https://doc.rust-lang.org/nightly/core/option/enum.Option.html)-returning
function.

## [Composition](https://flintlib.org/doc/fmpz_poly.html#composition) {#composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_compose (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_compose_horner (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |
| ✗ | `void fmpz_poly_compose_divconquer (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2)` | |

`fmpz_poly_compose` sets `res` to $$g(h(x))$$ for `poly1` $$= g$$ and `poly2` $$= h$$; the
`_horner` and `_divconquer` rows are algorithm choices. The same result comes from Horner's rule,
running over the coefficients $$c$$ of $$g$$ from the top with
`acc = acc * h + IntegerPolynomial::from(c)`; when $$h$$ is $$x^k$$, `compose` is
[`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x).

## [Inflation and deflation](https://flintlib.org/doc/fmpz_poly.html#inflation-and-deflation) {#inflation-and-deflation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_inflate (fmpz_poly_t result, const fmpz_poly_t input, ulong inflation)` | [`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x) |
| ≈ | `void fmpz_poly_deflate (fmpz_poly_t result, const fmpz_poly_t input, ulong deflation)` | [`deflate_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DeflatePowerOfX.html#tymethod.deflate_power_of_x) |
| ≈ | `ulong fmpz_poly_deflation (const fmpz_poly_t input)` | [`exponent_gcd`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ExponentGcd.html#tymethod.exponent_gcd) |

[`compose_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ComposePowerOfX.html#tymethod.compose_power_of_x),
like `fmpz_poly_inflate`, gives the constant $$p(1)$$ for $$n = 0$$. `fmpz_poly_deflate` silently
discards the coefficients at exponents that are not multiples of $$n$$, so deflating $$x^2 + x$$
by 2 returns $$x$$;
[`deflate_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.DeflatePowerOfX.html#tymethod.deflate_power_of_x)
panics instead, hence ≈. `fmpz_poly_deflation` returns 0 for the zero polynomial and 1 for a
constant;
[`exponent_gcd`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.ExponentGcd.html#tymethod.exponent_gcd)
returns 0 for every constant, hence ≈.

## [Taylor shift](https://flintlib.org/doc/fmpz_poly.html#taylor-shift) {#taylor-shift}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_taylor_shift (fmpz_poly_t g, const fmpz_poly_t f, const fmpz_t c)` | |
| ✗ | `void fmpz_poly_taylor_shift_horner (fmpz_poly_t g, const fmpz_poly_t f, const fmpz_t c)` | |
| ✗ | `void fmpz_poly_taylor_shift_divconquer (fmpz_poly_t g, const fmpz_poly_t f, const fmpz_t c)` | |
| ✗ | `void fmpz_poly_taylor_shift_multi_mod (fmpz_poly_t g, const fmpz_poly_t f, const fmpz_t c)` | |

`fmpz_poly_taylor_shift` computes $$f(x + c)$$; the other three rows are algorithm choices. The
same result comes from the Horner recipe under [composition](#composition), with inner polynomial
$$x + c$$.

## [Power series composition](https://flintlib.org/doc/fmpz_poly.html#power-series-composition) {#power-series-composition}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_compose_series (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | |
| ✗ | `void fmpz_poly_compose_series_horner (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | |
| ✗ | `void fmpz_poly_compose_series_brent_kung (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_poly_t poly2, slong n)` | |

`compose_series` is [composition](#composition) truncated to $$n$$ terms, and requires the inner
polynomial to have zero constant term. The same result comes from the Horner recipe under
composition with
[`mul_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulTruncated.html#tymethod.mul_truncated)
in place of `*`.

## [Power series reversion](https://flintlib.org/doc/fmpz_poly.html#power-series-reversion) {#power-series-reversion}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_revert_series (fmpz_poly_t Qinv, const fmpz_poly_t Q, slong n)` | |

`fmpz_poly_revert_series` computes the *compositional* inverse, with $$Q(Q^{-1}(x)) = x \bmod
x^n$$, not the multiplicative one that [`inv_series`](#power-series-division) computes. It
requires $$Q_0 = 0$$ and $$Q_1 = \pm 1$$, and checks neither.

## [Square root](https://flintlib.org/doc/fmpz_poly.html#square-root) {#square-root}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `int fmpz_poly_sqrt (fmpz_poly_t b, const fmpz_poly_t a)` | |
| ✗ | `int fmpz_poly_sqrt_classical (fmpz_poly_t b, const fmpz_poly_t a)` | |
| ✗ | `int fmpz_poly_sqrt_divconquer (fmpz_poly_t b, const fmpz_poly_t a)` | |
| — | `int fmpz_poly_sqrt_KS (fmpz_poly_t b, const fmpz_poly_t a)` | |
| ✗ | `int fmpz_poly_sqrtrem_classical (fmpz_poly_t b, fmpz_poly_t r, const fmpz_poly_t a)` | |
| ✗ | `int fmpz_poly_sqrtrem_divconquer (fmpz_poly_t b, fmpz_poly_t r, const fmpz_poly_t a)` | |
| ✗ | `int fmpz_poly_sqrt_series (fmpz_poly_t b, const fmpz_poly_t a, slong n)` | |

`sqrt_KS` returns $$-1$$ when its heuristic cannot decide; it is — because `fmpz_poly_sqrt`
already handles that case.

## [Power sums](https://flintlib.org/doc/fmpz_poly.html#power-sums) {#power-sums}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_power_sums (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | |
| ✗ | `void fmpz_poly_power_sums_naive (fmpz_poly_t res, const fmpz_poly_t poly, slong n)` | |
| ✗ | `void fmpz_poly_power_sums_to_poly (fmpz_poly_t res, const fmpz_poly_t Q)` | |

`fmpz_poly_power_sums` returns the power sums $$p_i = \sum_j r_j^i$$ of the complex roots, for
$$0 \le i < n$$, as the coefficients of a series, and `power_sums_to_poly` converts back, reading
the degree from $$p_0$$. Both directions require a monic polynomial, and neither checks its
preconditions.

## [Signature](https://flintlib.org/doc/fmpz_poly.html#signature) {#signature}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_signature (slong * r1, slong * r2, const fmpz_poly_t poly)` | |

`fmpz_poly_signature` returns $$(r_1, r_2)$$, with $$r_1$$ real roots and $$2r_2$$ non-real
ones. The input must be square-free over $$\mathbb{Q}$$, the condition that
[`fmpz_poly_is_squarefree`](#square-free) tests, and the behaviour is undefined otherwise; the
zero polynomial gives $$(0, 0)$$.

## [Hensel lifting](https://flintlib.org/doc/fmpz_poly.html#hensel-lifting) {#hensel-lifting}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_hensel_lift_once (fmpz_poly_factor_t lifted_fac, const fmpz_poly_t f, const nmod_poly_factor_t local_fac, slong N)` | |
| — | `void fmpz_poly_hensel_lift (fmpz_poly_t Gout, fmpz_poly_t Hout, fmpz_poly_t Aout, fmpz_poly_t Bout, ...)` | |
| — | `void fmpz_poly_hensel_lift_without_inverse (fmpz_poly_t Gout, fmpz_poly_t Hout, ...)` | |
| — | `void fmpz_poly_hensel_lift_only_inverse (fmpz_poly_t Aout, fmpz_poly_t Bout, ...)` | |
| — | `void fmpz_poly_hensel_build_tree (slong * link, fmpz_poly_t * v, fmpz_poly_t * w, const nmod_poly_factor_t fac)` | |
| — | `void fmpz_poly_hensel_lift_tree (slong * link, fmpz_poly_t * v, fmpz_poly_t * w, ...)` | |
| — | `void fmpz_poly_hensel_lift_tree_recursive (slong * link, fmpz_poly_t * v, fmpz_poly_t * w, ...)` | |

FLINT labels `hensel_lift_once` as the one entry "intended for end users"; the other six are
internal tree and step functions, so they are —.

## [Input and output](https://flintlib.org/doc/fmpz_poly.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int fmpz_poly_print_pretty (const fmpz_poly_t poly, const char * x)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`to_string_with`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.to_string_with) |
| ✓ | `int fmpz_poly_fprint_pretty (FILE * file, const fmpz_poly_t poly, const char * x)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) |
| ≈ | `int fmpz_poly_read_pretty (fmpz_poly_t poly, char ** x)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html), [`from_string_with`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.from_string_with) |
| ≈ | `int fmpz_poly_fread_pretty (FILE * file, fmpz_poly_t poly, char ** x)` | [`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) |
| ≈ | `int fmpz_poly_print (const fmpz_poly_t poly)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html), [`coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.coefficients_asc) |
| ≈ | `int fmpz_poly_fprint (FILE * file, const fmpz_poly_t poly)` | [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int fmpz_poly_read (fmpz_poly_t poly)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html), [`from_coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.from_coefficients_asc) |
| ≈ | `int fmpz_poly_fread (FILE * file, fmpz_poly_t poly)` | [`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) |

The `print`/`fprint` split disappears because
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes to any
[`Formatter`](https://doc.rust-lang.org/nightly/core/fmt/struct.Formatter.html). The plain format
is handled as in [the assignment section](#assignment-and-basic-manipulation). The pretty read
rows are ≈ because `fread_pretty` reports the variable name it found through `char ** x`, whereas
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) assumes `x` and
[`from_string_with`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.from_string_with)
takes the
[`Var`](https://docs.rs/malachite-base/latest/malachite_base/vars/struct.Var.html) up front.

## [Modular reduction and reconstruction](https://flintlib.org/doc/fmpz_poly.html#modular-reduction-and-reconstruction) {#modular-reduction}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void fmpz_poly_get_nmod_poly (nmod_poly_t Amod, const fmpz_poly_t A)` | [`Mod`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Mod.html) |
| ✗ | `void fmpz_poly_set_nmod_poly (fmpz_poly_t A, const nmod_poly_t Amod)` | |
| ✗ | `void fmpz_poly_set_nmod_poly_unsigned (fmpz_poly_t A, const nmod_poly_t Amod)` | |
| ✗ | `void fmpz_poly_CRT_ui (fmpz_poly_t res, const fmpz_poly_t poly1, const fmpz_t m, const nmod_poly_t poly2, int sign)` | |

Malachite has no `nmod_poly` type: a polynomial with reduced coefficients is a
[`NaturalPolynomial`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_polynomial/struct.NaturalPolynomial.html)
or
[`UnsignedPolynomial`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_polynomial/struct.UnsignedPolynomial.html)
satisfying
[`ModIsReduced`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.ModIsReduced.html).
`get_nmod_poly` is `(&a).mod_op(m)` with `m` of the same type as the result's coefficients (a
`u64` modulus gives an `UnsignedPolynomial<u64>`).

## [Products](https://flintlib.org/doc/fmpz_poly.html#products) {#products}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_product_roots_fmpz_vec (fmpz_poly_t poly, const fmpz * xs, slong n)` | |
| ✗ | `void fmpz_poly_product_roots_fmpq_vec (fmpz_poly_t poly, const fmpq * xs, slong n)` | |

`product_roots_fmpz_vec` builds $$\prod_i (x - x_i)$$. The rational variant clears
denominators, building $$\prod_i (q_i x - p_i)$$ for $$x_i = p_i/q_i$$, so its leading
coefficient is the product of the denominators. Either product can be formed with `*` from linear
factors built by
[`from_coefficients_asc`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_polynomial/struct.IntegerPolynomial.html#method.from_coefficients_asc).

## [Subproduct trees](https://flintlib.org/doc/fmpz_poly.html#subproduct-trees) {#subproduct-trees}

| | FLINT | Malachite |
| :---: | --- | --- |
| — | `fmpz ** _fmpz_poly_tree_alloc (slong len)` | |
| — | `void _fmpz_poly_tree_free (fmpz ** tree, slong len)` | |
| — | `void _fmpz_poly_tree_build_fmpq_vec (fmpz ** tree, const fmpq * roots, slong len)` | |

These allocate, free, and fill a caller-allocated `fmpz **` buffer, so they are —; allocation and
freeing are [`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html)'s job.

## [Roots](https://flintlib.org/doc/fmpz_poly.html#roots) {#roots}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_bound_roots (fmpz_t bound, const fmpz_poly_t poly)` | |
| ✗ | `slong fmpz_poly_positive_root_upper_bound_2exp (const fmpz_poly_t pol)` | |
| ✗ | `int fmpz_poly_has_real_root (const fmpz_poly_t pol)` | |
| ✗ | `slong fmpz_poly_num_real_roots (const fmpz_poly_t pol)` | |
| ✗ | `slong fmpz_poly_num_real_roots_sturm (const fmpz_poly_t pol)` | |
| ✗ | `slong fmpz_poly_num_real_roots_vca (const fmpz_poly_t pol)` | |
| ✗ | `slong fmpz_poly_num_real_roots_0_1 (const fmpz_poly_t pol)` | |
| ✗ | `slong fmpz_poly_num_real_roots_0_1_sturm (const fmpz_poly_t pol)` | |
| ✗ | `slong fmpz_poly_num_real_roots_0_1_vca (const fmpz_poly_t pol)` | |
| ✗ | `void fmpz_poly_isolate_real_roots (fmpz * exact_roots, slong * n_exact, fmpz * c_array, slong * k_array, slong * n_interval, const fmpz_poly_t pol)` | |
| ✗ | `void fmpz_poly_isolate_positive_roots (fmpz * exact_roots, slong * n_exact, fmpz * c_array, slong * k_array, slong * n_interval, const fmpz_poly_t pol)` | |

`bound_roots` is Fujiwara's bound on the absolute values of the complex roots, and
`positive_root_upper_bound_2exp` returns an exponent $$e$$ such that $$2^e$$ bounds the positive
roots. The isolation functions return the exact integer roots and dyadic intervals
$$(c \cdot 2^k, (c+1) \cdot 2^k)$$ through five output parameters, and the counting functions
assume a square-free input.

## [Minimal polynomials](https://flintlib.org/doc/fmpz_poly.html#minimal-polynomials) {#minimal-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_cyclotomic (fmpz_poly_t poly, ulong n)` | |
| ✗ | `ulong fmpz_poly_is_cyclotomic (const fmpz_poly_t poly)` | |
| ✗ | `void fmpz_poly_cos_minpoly (fmpz_poly_t poly, ulong n)` | |
| ✗ | `void fmpz_poly_swinnerton_dyer (fmpz_poly_t poly, ulong n)` | |

`is_cyclotomic` returns the index $$n$$, or 0 when the polynomial is not cyclotomic.
`cos_minpoly` gives the minimal polynomial of $$2\cos(2\pi/n)$$, not $$\cos(2\pi/n)$$, so that
it is monic with integer coefficients, and `swinnerton_dyer` has degree $$2^n$$.

## [Orthogonal polynomials](https://flintlib.org/doc/fmpz_poly.html#orthogonal-polynomials) {#orthogonal-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_chebyshev_t (fmpz_poly_t poly, ulong n)` | |
| ✗ | `void fmpz_poly_chebyshev_u (fmpz_poly_t poly, ulong n)` | |
| ✗ | `void fmpz_poly_legendre_pt (fmpz_poly_t poly, ulong n)` | |
| ✗ | `void fmpz_poly_hermite_h (fmpz_poly_t poly, ulong n)` | |
| ✗ | `void fmpz_poly_hermite_he (fmpz_poly_t poly, ulong n)` | |

`hermite_h` and `hermite_he` are the two Hermite conventions, $$H_n$$ and $$He_n$$, related by
$$He_n(x) = 2^{-n/2} H_n(x/\sqrt 2)$$. `legendre_pt` is the *shifted* Legendre polynomial
$$\tilde{P}_n(x) = P_n(2x-1)$$; the ordinary Legendre polynomials, which do not have integer
coefficients, are in `fmpq_poly`.

## [Fibonacci polynomials](https://flintlib.org/doc/fmpz_poly.html#fibonacci-polynomials) {#fibonacci-polynomials}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_fibonacci (fmpz_poly_t poly, ulong n)` | |

FLINT's convention is $$F_0 = 0$$, $$F_1 = 1$$, $$F_n(x) = x F_{n-1}(x) + F_{n-2}(x)$$, a
recurrence that can be run with
[`mul_power_of_x`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.MulPowerOfX.html#tymethod.mul_power_of_x)
and `+`. $$F_n(1)$$ is the $$n$$th Fibonacci number, which
[`Fibonacci`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Fibonacci.html)
provides directly.

## [Eulerian numbers and polynomials](https://flintlib.org/doc/fmpz_poly.html#eulerian-numbers-and-polynomials) {#eulerian}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_eulerian_polynomial (fmpz_poly_t res, ulong n)` | |

$$A_n(x) = \sum_m A(n, m) x^m$$, where the Eulerian number $$A(n, m)$$ counts permutations of
$$n$$ elements with $$m$$ descents; these are not the Euler numbers and polynomials.

## [Modular forms and q-series](https://flintlib.org/doc/fmpz_poly.html#modular-forms-and-q-series) {#modular-forms}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_eta_qexp (fmpz_poly_t f, slong r, slong n)` | |
| ✗ | `void fmpz_poly_theta_qexp (fmpz_poly_t f, slong r, slong n)` | |

`eta_qexp` gives the $$q$$-expansion of $$\prod_{k \geq 1}(1 - q^k)^r$$ (eta without its
fractional power of $$q$$) and `theta_qexp` that of $$\vartheta(q)^r$$, with
$$\vartheta(q) = 1 + 2\sum_{k \geq 1} q^{k^2}$$, both truncated to $$n$$ terms. The exponent
$$r$$ may be negative, and $$r = -1$$ in `eta_qexp` gives the partition numbers. For
$$r \geq 0$$, raising the $$r = 1$$ series to the $$r$$th power with
[`pow_truncated`](https://docs.rs/malachite-base/latest/malachite_base/polynomial/trait.PowTruncated.html#tymethod.pow_truncated)
gives the same result.

## [CLD bounds](https://flintlib.org/doc/fmpz_poly.html#cld-bounds) {#cld-bounds}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✗ | `void fmpz_poly_CLD_bound (fmpz_t res, const fmpz_poly_t f, slong n)` | |

`fmpz_poly_CLD_bound` bounds the $$n$$th coefficient of $$fg'/g$$ for every factor $$g$$ of
$$f$$; it serves van Hoeij's recombination step in factorisation.
