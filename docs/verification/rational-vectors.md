---
layout: default
title: "How Malachite Is Tested: Rational Vectors"
permalink: /verification/rational-vectors/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Rational Vectors

This page lists every public function of
[`RationalVector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html), the vector of
[`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html)s of the
`malachite-q` crate, and records which independent implementations it is checked against.
[The introduction](/verification/) describes the oracles and the testing they sit in; this page is
the ledger. It follows the organization of the crate's documentation, one section per module, so
that a function is where its documentation is. The conversions between `RationalVector` and the
[`NaturalVector`](/verification/natural-vectors/),
[`IntegerVector`](/verification/integer-vectors/), and
[`UnsignedVector`](/verification/unsigned-vectors/) types are listed here, since they are defined
in `malachite-q`.

## Reading the tables {#reading-the-tables}

Each row is one operation, named after the module that implements it and linked to that module's
documentation. The **Functions** column lists the traits and methods the module implements; a
trait's by-value and by-reference implementations, and its `*Assign` form, count as one function,
since they compute the same thing. The remaining columns are the oracles of
[the introduction](/verification/#the-oracles) that have vector functions: Azurite, whose
`AzVector AzRat n` is mapped [here](/mapping/azurite-rational-vectors/), FLINT's `fmpq_vec.h`,
mapped [here](/mapping/flint-rational-vectors/), and the reference implementations in
`test_util`. Neither GMP nor num has a vector type, so their columns are left out.

| | meaning |
| :---: | --- |
| ✓ | The oracle computes this function and agrees with Malachite on every input tried. |
| ≈ | The oracle agrees, after an adaptation on the oracle side that is more than a change of spelling: a convention of Malachite's that the oracle does not share is applied to the oracle's output before the comparison. The notes say what the adaptation is. |
| | The oracle does not check this function. |

Of the 80 rows, 14 have an Azurite check, 0 a FLINT check, and 7 a reference implementation;
62 are checked only by Malachite's own unit and property tests. Those rows are collected at the end
of the page, in [What is not yet cross-checked](#not-yet-cross-checked).

Functions that are not part of the public interface are not listed, such as the helpers on
coefficient slices that the polynomial and matrix types share with `RationalVector`, and the
alternative algorithms that the tests compare with each other; they are exercised by the same runs
as the functions built on them.

## Basic {#basic}

The standard traits of the type itself, from
[`malachite_q::rational_vector`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [named](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Named-for-RationalVector) | `Named` |   |   |   |
| [hash](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Hash-for-RationalVector) | `Hash` |   |   |   |
| [clone](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Clone-for-RationalVector) | `Clone` |   |   |   |

`Named` gives the type's name as a string, and `Hash` and `Clone` are derived from the `Vec` of
entries, so none has anything to compare against.

## The vector interface {#the-vector-interface}

The methods of the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) trait,
which Malachite's vector types share, and the indexing and selection traits.

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `from_elements`, `from_owned_elements` |   |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `zero` | ✓ |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `set_zero`, `is_zero` |   |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `standard_basis_vector` | ✓ |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `standard_basis_index` |   |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `to_elements`, `into_elements`, `elements_ref` |   |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `dimension`, `push`, `set_dimension` |   |   |   |
| [extend](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/extend/index.html) | `Extend` |   |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `sort` |   |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `pivot`, `pivot_index`, `height_index` |   |   |   |
| [vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-Vector-for-RationalVector) | `max_bits`, `sum_max_bits` |   |   |   |
| [access](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/access/index.html) | `Index` | ✓ |   |   |
| [access](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/access/index.html) | `IndexMut` | ✓ |   |   |
| [select_coordinates](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/select_coordinates/index.html) | `SelectCoordinates`, `SelectCoordinatesAssign` |   |   |   |

Most of these construct, read, and resize vectors and need no oracle beyond their own tests. The
zero vector, the standard basis vectors, and indexing are checked against Azurite's `Zero`
instance, `AzVector.stdBasis`, and `AzVector.get`, and `IndexMut`, through a demo that adds 1 to
one entry, against `AzRat`'s sum.

## Comparison {#comparison}

From [`malachite_q::rational_vector::comparison`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/comparison/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [eq](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/struct.RationalVector.html#impl-PartialEq-for-RationalVector) | `PartialEq`, `Eq` |   |   |   |
| [partial_eq_natural_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/comparison/partial_eq_natural_vector/index.html) | `PartialEq<NaturalVector>` and the reverse direction |   |   |   |
| [partial_eq_integer_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/comparison/partial_eq_integer_vector/index.html) | `PartialEq<IntegerVector>` and the reverse direction |   |   |   |
| [partial_eq_unsigned_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/comparison/partial_eq_unsigned_vector/index.html) | `PartialEq<UnsignedVector<T>>` and the reverse direction |   |   |   |
| [shortlex_cmp](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/comparison/shortlex_cmp/index.html) | `Ord` and `PartialOrd` for `ShortlexRationalVector` and `ShortlexRationalVectorRef` |   |   | ✓ |
| [entrywise_max](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/comparison/entrywise_max/index.html) | `EntrywiseMax`, `EntrywiseMaxAssign` |   |   |   |
| [entrywise_min](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/comparison/entrywise_min/index.html) | `EntrywiseMin`, `EntrywiseMinAssign` |   |   |   |

`RationalVector` has no order of its own; the `Shortlex` wrappers order vectors by dimension and
then lexicographically, and that order is compared with a reference that builds the comparison
from the dimensions and the entries directly.

## Arithmetic {#arithmetic}

From [`malachite_q::rational_vector::arithmetic`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/index.html).

### Addition and multiplication {#addition-and-multiplication}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [add](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/add/index.html) | `Add`, `AddAssign` | ✓ |   |   |
| [sub](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/sub/index.html) | `Sub`, `SubAssign` | ✓ |   |   |
| [neg](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/neg/index.html) | `Neg`, `NegAssign` | ✓ |   |   |
| [sum](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/sum/index.html) | `Sum` |   |   |   |
| [scalar_mul](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/scalar_mul/index.html) | `Mul<Rational>`, `MulAssign<Rational>`, and `Mul<RationalVector>` for `Rational` | ✓ |   |   |
| [scalar_div](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/scalar_div/index.html) | `Div<Rational>`, `DivAssign<Rational>` |   |   |   |
| [add_mul](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/add_mul/index.html) | `AddMul`, `AddMulAssign` |   |   |   |
| [sub_mul](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/sub_mul/index.html) | `SubMul`, `SubMulAssign` |   |   |   |
| [add_mul_shl](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/add_mul_shl/index.html) | `AddMulShl`, `AddMulShlAssign` |   |   |   |
| [sub_mul_shl](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/sub_mul_shl/index.html) | `SubMulShl`, `SubMulShlAssign` |   |   |   |
| [entrywise_mul](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/entrywise_mul/index.html) | `EntrywiseMul`, `EntrywiseMulAssign` |   |   |   |
| [entrywise_abs](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/entrywise_abs/index.html) | `EntrywiseAbs`, `EntrywiseAbsAssign` |   |   |   |
| [canonicalize_sign](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/canonicalize_sign/index.html) | `CanonicalizeSign`, `CanonicalizeSignAssign` |   |   |   |
| [checked_abs](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/checked_abs/index.html) | `CheckedAbs` |   |   | ✓ |

The entrywise sum, difference, and negation, in every by-value, by-reference, and in-place form,
are checked against `AzVector`'s `Add`, `Sub`, and `Neg` instances, and multiplication by a
scalar, on either side, against its `SMul` instance, all of which Azurite proves to be the
entrywise operations on `Fin n → AzRat`. `CheckedAbs`, the Euclidean norm when it is rational and
`None` otherwise, is compared with a reference that takes the floor square roots of the numerator
and the denominator of the squared length and checks that both are exact.

### Shifts {#shifts}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [shl](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/shl/index.html) | `Shl`, `ShlAssign` |   |   |   |
| [shr](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/shr/index.html) | `Shr`, `ShrAssign` |   |   |   |

### Dot products and norms {#dot-products-and-norms}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [dot](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/dot/index.html) | `Dot` | ✓ |   | ✓ |
| [dot_reversed](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/dot_reversed/index.html) | `DotReversed` |   |   |   |
| [add_dot](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/add_dot/index.html) | `AddDot`, `AddDotAssign` |   |   |   |
| [add_dot_reversed](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/add_dot_reversed/index.html) | `AddDotReversed`, `AddDotReversedAssign` |   |   |   |
| [sub_dot](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/sub_dot/index.html) | `SubDot`, `SubDotAssign` |   |   |   |
| [sub_dot_reversed](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/sub_dot_reversed/index.html) | `SubDotReversed`, `SubDotReversedAssign` |   |   |   |
| [dot_sign](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/dot_sign/index.html) | `DotSign` | ✓ |   |   |
| [abs_squared](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/abs_squared/index.html) | `AbsSquared` | ✓ |   | ✓ |
| [l1_norm](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/l1_norm/index.html) | `L1Norm` |   |   |   |
| [height](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/height/index.html) | `Height`, `HeightRef` |   |   |   |
| [cross](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/cross/index.html) | `Cross`, `CrossAssign` | ✓ |   | ✓ |
| [generalized_cross](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/generalized_cross/index.html) | `GeneralizedCross` |   |   | ✓ |

The dot product, the squared length, and the cross product are checked against Azurite's
`AzVector.dot`, `AzVector.normSq`, and `AzVector.cross`, which are proven equal to Mathlib's
`dotProduct` and cross product, and the dot product and squared length also against schoolbook
references. `DotSign` is checked against Azurite's `AzRat.dotSign`, whose screens on bit lengths
and whose final comparison of unreduced fractions are proven correct;
[the mapping page](/mapping/azurite-rational-vectors/#arithmetic) describes the algorithm. The
cross product is also compared with its defining formula, and the generalized cross product with
the determinants that define its entries.

### Content and products of entries {#content-and-products-of-entries}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [content](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/content/index.html) | `Content`, `PrimitivePart`, `ContentAndPrimitivePart` |   |   |   |
| [canonical_primitive_part](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/canonical_primitive_part/index.html) | `CanonicalPrimitivePart`, `ContentAndCanonicalPrimitivePart` |   |   |   |
| [content_chained](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/content_chained/index.html) | `ContentChained` |   |   |   |
| [element_sum](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/element_sum/index.html) | `ElementSum` |   |   |   |
| [element_product](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/arithmetic/element_product/index.html) | `ElementProduct` |   |   |   |

## Linear algebra {#linear-algebra}

From [`malachite_q::rational_vector::linear_algebra`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/linear_algebra/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [are_linearly_independent](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/linear_algebra/are_linearly_independent/index.html) | `AreLinearlyIndependent` |   |   |   |
| [are_affinely_independent](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/linear_algebra/are_affinely_independent/index.html) | `AreAffinelyIndependent` |   |   |   |

These are decided by the rank of a matrix built from the vectors, whose checks will be on the
matrix pages; the tests compare the alternative algorithms for each with one another.

## Logic {#logic}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [significant_bits](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/logic/significant_bits/index.html) | `SignificantBits` |   |   |   |

## Conversion {#conversion}

From [`malachite_q::rational_vector::conversion`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [from_natural_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/from_natural_vector/index.html) | `From<NaturalVector>` |   |   |   |
| [from_integer_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/from_integer_vector/index.html) | `From<IntegerVector>` |   |   |   |
| [from_unsigned_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/from_unsigned_vector/index.html) | `From<UnsignedVector<T>>` |   |   |   |
| [natural_vector_from_rational_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/natural_vector_from_rational_vector/index.html) | `TryFrom<RationalVector>` and `ConvertibleFrom<&RationalVector>` for `NaturalVector` |   |   |   |
| [integer_vector_from_rational_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/integer_vector_from_rational_vector/index.html) | `TryFrom<RationalVector>` and `ConvertibleFrom<&RationalVector>` for `IntegerVector` |   |   |   |
| [unsigned_vector_from_rational_vector](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/unsigned_vector_from_rational_vector/index.html) | `TryFrom<RationalVector>` and `ConvertibleFrom<&RationalVector>` for `UnsignedVector<T>` |   |   |   |
| [from_numerators_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/from_numerators_and_denominator/index.html) | `from_numerators_and_denominator` |   |   |   |
| [to_numerators_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/to_numerators_and_denominator/index.html) | `to_numerators_and_denominator`, `into_numerators_and_denominator` |   |   | ✓ |
| [from_iterator](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/from_iterator/index.html) | `FromIterator<Rational>` |   |   |   |
| [into_iterator](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/into_iterator/index.html) | `IntoIterator` |   |   |   |

`to_numerators_and_denominator`, which writes a vector as an `IntegerVector` of numerators over
one common `Natural` denominator, is compared with a reference that multiplies every entry by the
product of the denominators and then divides out the greatest common divisor of the results.

### Strings {#strings}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [to_string](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/string/to_string/index.html) | `Display`, `Debug` | ✓ |   |   |
| [from_string](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/string/from_string/index.html) | `FromStr` | ✓ |   |   |
| [latex](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/string/latex/index.html) | `ToLatex` |   |   |   |
| [typst](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/conversion/string/typst/index.html) | `ToTypst` |   |   |   |

Vectors are written as `(1/2, -3, 0)`, with each entry written by `Rational`'s `Display`, which is
also Azurite's format. Since the output demos print only the vector, Azurite's check of `Display`
and `Debug` is that its own parser reads each printed vector and its printer reproduces it
exactly, which rejects any non-canonical spacing, sign, or fraction. `FromStr` is checked on random
strings and on printed vectors, with an oracle-side implementation of its documented grammar (the
entries separated by `", "` inside parentheses, each read by `Rational::from_str`'s rule) and
Azurite's rationals and printer.

## Exhaustive generation {#exhaustive-generation}

From [`malachite_q::rational_vector::exhaustive`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/exhaustive/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [exhaustive](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/exhaustive/index.html) | `exhaustive_rational_vectors`, `exhaustive_rational_vectors_from_iterator` |   |   |   |
| [exhaustive](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/exhaustive/index.html) | `exhaustive_rational_vectors_with_dimension`, `exhaustive_rational_vectors_with_dimension_from_iterator` |   |   |   |
| [exhaustive](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/exhaustive/index.html) | `exhaustive_ordered_unique_rational_vectors_with_dimension` |   |   |   |

## Random generation {#random-generation}

From [`malachite_q::rational_vector::random`](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/random/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [random](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/random/index.html) | `random_rational_vectors`, `random_rational_vectors_from_iterator`, `striped_random_rational_vectors` |   |   |   |
| [random](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/random/index.html) | `random_rational_vectors_with_dimension`, `random_rational_vectors_with_dimension_from_iterator`, `striped_random_rational_vectors_with_dimension` |   |   |   |
| [random](https://docs.rs/malachite-q/latest/malachite_q/rational_vector/random/index.html) | `random_ordered_unique_rational_vectors_with_dimension`, `random_ordered_unique_rational_vectors_with_dimension_from_iterator`, `striped_random_ordered_unique_rational_vectors_with_dimension` |   |   |   |

## What is not yet cross-checked {#not-yet-cross-checked}

The rows above with no mark, and the rows with no Azurite mark, sort into two groups by what it
would take to give them a verified oracle, after which come the rows outside the scope of an
oracle. Within each group the most widely used functions come first.

**A few lines of oracle code around Azurite functions.** These need no new Azurite function, but
the oracle would compose existing ones over the entries, so the composition itself is unverified
code: the division by a scalar, the entrywise products, absolute values, minima, and maxima, and
the shifts (each `AzRat` operation `map`ped or `zip`ped over the vector); the fused operations and
the fused and reversed dot products; the sums of vectors and the sums and products of the entries;
the content and primitive parts and `CanonicalizeSign`; the L1 norm, the height, and `CheckedAbs`;
equality with the other vector types; and the conversions to and from them and to and from
numerators and a denominator.

**Azurite would need new functions.** The generalized cross product and the tests of linear and
affine independence, which need determinants or ranks of matrices; the `Shortlex` order, which
Azurite's vectors do not have; and the exhaustive generators, for which `AzVector` has no
`ExhaustiveGenerator` instance.

**Outside the scope of an oracle.** `Named`, `Hash`, `Clone`, the construction and access
functions of the vector interface, `Extend`, `FromIterator`, `IntoIterator`, LaTeX and Typst
output, serde, and the random generators. These are trivial or have no independent
implementation to compare with, and are tested by their own unit and property tests, the random
generators by the distribution of large samples.
