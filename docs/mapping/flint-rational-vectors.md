---
layout: default
title: "Malachite for FLINT Users: Rational Vectors"
permalink: /mapping/flint-rational-vectors/
theme: jekyll-theme-slate
---

# Malachite for FLINT Users: Rational Vectors

This page maps the functions of FLINT's rational vector module, `fmpq_vec.h`, onto their Malachite
counterpart:
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html),
from the `malachite-q` crate. It follows the organization of the [fmpq_vec.h
chapter](https://flintlib.org/doc/fmpq_vec.html) of the FLINT manual, as of FLINT 3.6.0. It is a
companion to [Malachite for FLINT Users: Integer Vectors](/mapping/flint-integer-vectors/), whose
[conventions](/mapping/flint-integer-vectors/#conventions) apply here as well, and to [Malachite
for FLINT Users: Rationals](/mapping/flint-rationals/); the [mapping index](/mapping/) lists the
whole family.

Every section of the chapter is mapped below: 13 rows across 8 sections, of which 8 are ✓, 3 ≈,
1 —, and 1 ✗. All of the chapter's functions have names beginning with an underscore, and all are
included (see [Representation](#representation)). The one ✗ row is a random generator,
`_fmpq_vec_randtest_uniq_sorted`.

## Conventions {#conventions}

### Representation {#representation}

`fmpq_vec.h` has no vector object. Every function works on a bare array, an `fmpq *` together
with a separate `slong` length, which the caller allocates, tracks, and frees, and every function
name begins with an underscore. As on [the integer vector
page](/mapping/flint-integer-vectors/#underscore-functions), those functions are mapped here.

[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)
is a single public field,
[`elements`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#structfield.elements),
of type [`Vec<Rational>`](https://doc.rust-lang.org/nightly/alloc/vec/struct.Vec.html). Both
libraries store the elements entrywise: each element is a rational number of its own, in lowest
terms, rather than a numerator over a denominator shared by the whole vector, as in `fmpq_poly`.
[`to_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#method.to_numerators_and_denominator)
gives the common-denominator form when it is wanted.

### Categories

Each function falls into one of five categories:

| | meaning |
| :---: | --- |
| ✓ | A Malachite function does the same thing. |
| ≈ | A Malachite function serves the same purpose, but its specification differs. The notes say how. |
| ⚙ | Malachite does not expose this algorithm or helper; the Malachite column or the notes say what to call instead. |
| — | No counterpart is needed, either because Rust handles it for you or because it is outside Malachite's scope. The notes say which. |
| ✗ | Malachite does not fully support this yet, but will in a future version. |

## [Memory management](https://flintlib.org/doc/fmpq_vec.html#memory-management) {#memory-management}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `fmpq * _fmpq_vec_init(slong n)` | [`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero) |
| — | `void _fmpq_vec_clear(fmpq * vec, slong n)` | |

`_fmpq_vec_init` returns `n` zeros, each with denominator 1, which is
[`zero`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.zero), the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) method
that returns the zero vector of a given dimension: `RationalVector::zero(n)`. `_fmpq_vec_clear` is
[`Drop`](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html), which runs when the vector goes out of scope.

## [Randomisation](https://flintlib.org/doc/fmpq_vec.html#randomisation) {#randomisation}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `void _fmpq_vec_randtest(fmpq * f, flint_rand_t state, slong len, flint_bitcnt_t bits)` | [`random_rational_vectors_with_dimension`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/random/fn.random_rational_vectors_with_dimension.html), [`striped_random_rational_vectors_with_dimension`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/random/fn.striped_random_rational_vectors_with_dimension.html), [`random_rational_vectors_with_dimension_from_iterator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/random/fn.random_rational_vectors_with_dimension_from_iterator.html) |
| ✗ | `void _fmpq_vec_randtest_uniq_sorted(fmpq * vec, flint_rand_t state, slong len, flint_bitcnt_t bits)` | |

`_fmpq_vec_randtest` fills a vector of length `len`, drawing each element with
[`fmpq_randtest`](/mapping/flint-rationals/#random-number-generation), which caps the numerator
and denominator at `bits` bits. Half the time it makes the vector sparse, exactly as
[`_fmpz_vec_randtest`](/mapping/flint-integer-vectors/#randomisation) does. Malachite's
counterparts are infinite iterators of vectors of a given dimension. The elements of the first
two come from
[`random_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.random_rationals.html)
or
[`striped_random_rationals`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.striped_random_rationals.html),
which take a *mean* bit length with no cap, and there is no sparse mode, so the distributions
differ. The third takes its elements from any iterator, so a bounded element stream, such as
[`random_rational_range`](https://docs.rs/malachite-q/latest/malachite_q/rational/random/fn.random_rational_range.html),
gives a hard bound on the values.

`_fmpq_vec_randtest_uniq_sorted` produces distinct elements in ascending order. It draws a vector
with `_fmpq_vec_randtest`, then repeatedly sorts it and redraws any element equal to its
neighbour until none are. Malachite has no generator of distinct sorted vectors yet. The
implementation throws an error unless `bits` is at least four times the bit length of `len`, a
requirement the manual does not state; since FLINT counts the bit length of 0 as 1, this means
`bits` must be at least 4 even when `len` is 0 or 1.

## [Bit sizes and heights](https://flintlib.org/doc/fmpq_vec.html#bit-sizes-and-heights) {#bit-sizes-and-heights}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpq_vec_max_height(fmpz_t height, const fmpq * vec, slong len)` | [`Height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html), [`HeightRef`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.HeightRef.html) |
| ✓ | `flint_bitcnt_t _fmpq_vec_max_height_bits(const fmpq * vec, slong len)` | [`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits) |

The height of a rational number $$p/q$$ in lowest terms is $$\max(|p|, q)$$, and `max_height` is
the largest height of any element, with `max_height_bits` its bit length. These are
[`Height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html)'s
[`to_height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.to_height)
(or
[`into_height`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.into_height))
and [`height_significant_bits`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Height.html#tymethod.height_significant_bits).
Since every element is kept in lowest terms, the height is the numerator or denominator of one
of the elements, so [`HeightRef`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.HeightRef.html) lends it rather than building it. The index of an element of largest
height, which this chapter does not provide, is [`height_index`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.height_index).

As in FLINT, the height of the 0-dimensional vector is 0. Every other vector has height at least
1, since every denominator is at least 1; a vector of zeros has height 1, the height of
$$0 = 0/1$$. Malachite's
[`RationalPolynomial`](https://docs.rs/malachite-q/latest/malachite_q/rational_polynomial/struct.RationalPolynomial.html)
gives its zero polynomial, which has no coefficients, height 1 rather than 0.

## [Comparison](https://flintlib.org/doc/fmpq_vec.html#comparison) {#comparison}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `int _fmpq_vec_equal(const fmpq * vec1, const fmpq * vec2, slong len)` | [`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) |

`_fmpq_vec_equal` compares two arrays over a length the caller supplies;
[`PartialEq`](https://doc.rust-lang.org/nightly/core/cmp/trait.PartialEq.html) on
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)
also requires the dimensions to match. Since every element is kept in lowest terms, two vectors
are equal exactly when their elements are identical. A
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)
can also be compared with an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
[`NaturalVector`](https://docs.rs/malachite-nz/latest/malachite_nz/natural_vector/struct.NaturalVector.html),
or
[`UnsignedVector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html),
which it equals when it has the same dimension and the same values.

## [Sorting](https://flintlib.org/doc/fmpq_vec.html#sorting) {#sorting}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpq_vec_sort(fmpq * vec, slong len)` | [`sort`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.sort) |

The manual does not say in which order `_fmpq_vec_sort` sorts; the implementation uses the C
library's `qsort` with `fmpq_cmp`, so the order is ascending by value, as for
[`_fmpz_vec_sort`](/mapping/flint-integer-vectors/#sorting). It is
[`sort`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html#tymethod.sort),
a [`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html)
method, which sorts in place by value.

## [Conversions](https://flintlib.org/doc/fmpq_vec.html#conversions) {#conversions}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpq_vec_set_fmpz_vec(fmpq * res, const fmpz * vec, slong len)` | [`From<IntegerVector>`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) |
| ✓ | `void _fmpq_vec_get_fmpz_vec_fmpz(fmpz * num, fmpz_t den, const fmpq * a, slong len)` | [`to_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#method.to_numerators_and_denominator) |

`_fmpq_vec_set_fmpz_vec` converts each integer to a rational with denominator 1, which is
[`From`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html)`<`[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)`>`;
the conversion takes the
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
by value, so clone it first if it is still needed. The reverse,
[`TryFrom`](https://doc.rust-lang.org/nightly/core/convert/trait.TryFrom.html)`<`[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)`>`
for
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html),
succeeds when every element is an integer, and has no counterpart in this chapter.

`_fmpq_vec_get_fmpz_vec_fmpz` writes a vector over a common denominator: the least common multiple
of the elements' denominators, with each numerator scaled to it. It is 1 for the empty vector.
[`to_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#method.to_numerators_and_denominator)
computes the same denominator and numerators, returning them as an
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html)
and a [`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html);
[`into_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#method.into_numerators_and_denominator)
consumes the vector instead of cloning its elements, and
[`from_numerators_and_denominator`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#method.from_numerators_and_denominator)
converts back, a direction this chapter does not provide. `fmpq_vec.h` declares
`_fmpq_vec_get_fmpz_vec_fmpz` twice, with identical prototypes.

## [Dot product](https://flintlib.org/doc/fmpq_vec.html#dot-product) {#dot-product}

| | FLINT | Malachite |
| :---: | --- | --- |
| ✓ | `void _fmpq_vec_dot(fmpq_t res, const fmpq * vec1, const fmpq * vec2, slong len)` | [`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html) |

`_fmpq_vec_dot` sets `res` to $$\sum_i a_i b_i$$, which is $$0$$ for the empty vector. It is
[`Dot`](https://docs.rs/malachite-base/latest/malachite_base/num/arithmetic/traits/trait.Dot.html)'s `v.dot(w)` for `RationalVector`, with each vector taken by value or by reference;
the vectors must have the same dimension. FLINT computes it with one `fmpq_addmul` per pair, each
reducing to lowest terms (see [`fmpq_addmul`](/mapping/flint-rationals/#arithmetic)). `Dot` instead
writes each vector as integer numerators over a common denominator, takes one integer dot product,
and reduces once.
Unlike [`_fmpz_vec_dot`](/mapping/flint-integer-vectors/#dot-product), there is no
`_fmpq_vec_dot_general` with an initial term, subtraction, or reversal.

## [Input and output](https://flintlib.org/doc/fmpq_vec.html#input-and-output) {#input-and-output}

| | FLINT | Malachite |
| :---: | --- | --- |
| ≈ | `int _fmpq_vec_fprint(FILE * file, const fmpq * vec, slong len)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |
| ≈ | `int _fmpq_vec_print(const fmpq * vec, slong len)` | [`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html), [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html) |

The vector formats differ, hence ≈, as for [integer
vectors](/mapping/flint-integer-vectors/#input-and-output). FLINT writes the length, two spaces,
and then the elements separated by single spaces, so $$(1/2, -2, 3)$$ is `3  1/2 -2 3` and the
empty vector is `0`. Malachite's
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes `(1/2, -2, 3)`,
with `()` for the 0-dimensional vector. The elements themselves are written the same way by both
libraries, as `p/q`, or as just `p` when the denominator is 1. With the `serde` feature,
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html)
is serialized as a plain list of its elements.

Unlike `fmpz_vec.h`, this chapter has no functions for reading a vector. Malachite's
[`FromStr`](https://doc.rust-lang.org/nightly/core/str/trait.FromStr.html) reads the form that
[`Display`](https://doc.rust-lang.org/nightly/core/fmt/trait.Display.html) writes, and
[`Deserialize`](https://docs.rs/serde/latest/serde/trait.Deserialize.html) reads the serialized
form.
