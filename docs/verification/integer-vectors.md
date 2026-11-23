---
layout: default
title: "How Malachite Is Tested: Integer Vectors"
permalink: /verification/integer-vectors/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Integer Vectors

This page lists every public function of
[`IntegerVector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html), the vector of
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html)s of the
`malachite-nz` crate, and records which independent implementations it is checked against.
[The introduction](/verification/) describes the oracles and the testing they sit in; this page is
the ledger. It follows the organization of the crate's documentation, one section per module, so
that a function is where its documentation is. Most vector operations act entrywise or combine
entries with the arithmetic of [`Integer`](/verification/integers/), whose own checks are listed
on that page; the rows here record what checks the vector operation as a whole.

## Reading the tables {#reading-the-tables}

Each row is one operation, named after the module that implements it and linked to that module's
documentation. The **Functions** column lists the traits and methods the module implements; a
trait's by-value and by-reference implementations, and its `*Assign` form, count as one function,
since they compute the same thing. The remaining columns are the oracles of
[the introduction](/verification/#the-oracles) that have vector functions: Azurite, whose
`AzVector AzInt n` is mapped [here](/mapping/azurite-integer-vectors/), FLINT's `fmpz_vec.h`,
mapped [here](/mapping/flint-integer-vectors/), and the reference implementations in `test_util`.
Neither GMP nor num has a vector type, so their columns are left out.

| | meaning |
| :---: | --- |
| ✓ | The oracle computes this function and agrees with Malachite on every input tried. |
| ≈ | The oracle agrees, after an adaptation on the oracle side that is more than a change of spelling: a convention of Malachite's that the oracle does not share is applied to the oracle's output before the comparison. The mapping pages say what the adaptation is. |
| | The oracle does not check this function. |

Of the 87 rows, 13 have an Azurite check, 4 a FLINT check, and 17 a reference implementation;
59 are checked only by Malachite's own unit and property tests. Those rows are collected at the end
of the page, in [What is not yet cross-checked](#not-yet-cross-checked).

Functions that are not part of the public interface are not listed, such as the `vec_*` helpers
on coefficient slices that the polynomial types share with `IntegerVector`; they are exercised by
the same runs as the functions built on them.

## Basic {#basic}

The standard traits of the type itself, from
[`malachite_nz::integer_vector`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [named](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Named-for-IntegerVector) | `Named` |   |   |   |
| [hash](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Hash-for-IntegerVector) | `Hash` |   |   |   |
| [clone](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Clone-for-IntegerVector) | `Clone` |   |   |   |

`Named` gives the type's name as a string, and `Hash` and `Clone` are derived from the `Vec` of
entries, so none has anything to compare against.

## The vector interface {#the-vector-interface}

The methods of the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) trait,
which Malachite's vector types share, and the indexing and selection traits.

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `from_elements`, `from_owned_elements` |   |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `zero` | ✓ |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `set_zero`, `is_zero` |   |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `standard_basis_vector` | ✓ |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `standard_basis_index` |   |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `to_elements`, `into_elements`, `elements_ref` |   |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `dimension`, `push`, `set_dimension` |   |   |   |
| [extend](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/extend/index.html) | `Extend` |   |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `sort` |   |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `pivot`, `pivot_index`, `height_index` |   |   |   |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `max_bits` |   |   | ✓ |
| [vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-Vector-for-IntegerVector) | `sum_max_bits` |   |   | ✓ |
| [access](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/access/index.html) | `Index` | ✓ |   |   |
| [access](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/access/index.html) | `IndexMut` |   |   |   |
| [select_coordinates](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/select_coordinates/index.html) | `SelectCoordinates`, `SelectCoordinatesAssign` |   |   |   |

Most of these construct, read, and resize vectors and need no oracle beyond their own tests; the
zero vector, the standard basis vectors, and indexing are checked against Azurite's `Zero`
instance, `AzVector.stdBasis`, and `AzVector.get`. The two bit-size summaries, the largest number of significant bits of an entry and the total over all
entries, are compared with entry-by-entry reference loops.

## Comparison {#comparison}

From
[`malachite_nz::integer_vector::comparison`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/comparison/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [eq](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/struct.IntegerVector.html#impl-PartialEq-for-IntegerVector) | `PartialEq`, `Eq` |   |   |   |
| [partial_eq_natural_vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/comparison/partial_eq_natural_vector/index.html) | `PartialEq<NaturalVector>` and the reverse direction |   |   |   |
| [partial_eq_unsigned_vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/comparison/partial_eq_unsigned_vector/index.html) | `PartialEq<UnsignedVector<T>>` and the reverse direction |   |   |   |
| [shortlex_cmp](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/comparison/shortlex_cmp/index.html) | `Ord` and `PartialOrd` for `ShortlexIntegerVector` and `ShortlexIntegerVectorRef` |   |   | ✓ |
| [entrywise_max](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/comparison/entrywise_max/index.html) | `EntrywiseMax`, `EntrywiseMaxAssign` |   |   |   |
| [entrywise_min](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/comparison/entrywise_min/index.html) | `EntrywiseMin`, `EntrywiseMinAssign` |   |   |   |

`IntegerVector` has no order of its own; the `Shortlex` wrappers order vectors by dimension and
then lexicographically, and that order is compared with a reference that builds the comparison
from the dimensions and the entries directly.

## Arithmetic {#arithmetic}

From
[`malachite_nz::integer_vector::arithmetic`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/index.html).

### Addition, subtraction, and multiplication {#addition-subtraction-and-multiplication}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [add](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/add/index.html) | `Add`, `AddAssign` | ✓ |   |   |
| [sub](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/sub/index.html) | `Sub`, `SubAssign` | ✓ |   |   |
| [neg](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/neg/index.html) | `Neg`, `NegAssign` | ✓ |   |   |
| [sum](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/sum/index.html) | `Sum` |   |   |   |
| [scalar_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/scalar_mul/index.html) | `Mul<Integer>`, `MulAssign<Integer>`, and `Mul<IntegerVector>` for `Integer` | ✓ |   | ✓ |
| [scalar_div](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/scalar_div/index.html) | `Div<Integer>`, `DivAssign<Integer>` |   |   |   |
| [div_exact](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/div_exact/index.html) | `DivExact`, `DivExactAssign` |   |   |   |
| [add_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/add_mul/index.html) | `AddMul`, `AddMulAssign` |   |   |   |
| [sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/sub_mul/index.html) | `SubMul`, `SubMulAssign` |   |   |   |
| [add_mul_shl](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/add_mul_shl/index.html) | `AddMulShl`, `AddMulShlAssign` |   |   |   |
| [sub_mul_shl](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/sub_mul_shl/index.html) | `SubMulShl`, `SubMulShlAssign` |   |   |   |
| [entrywise_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/entrywise_mul/index.html) | `EntrywiseMul`, `EntrywiseMulAssign` |   |   |   |
| [entrywise_div_round](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/entrywise_div_round/index.html) | `EntrywiseDivRound`, `EntrywiseDivRoundAssign` |   |   |   |

Entrywise sums, differences, and negations, in every by-value, by-reference, and in-place form,
are checked against `AzVector`'s `Add`, `Sub`, and `Neg` instances, and multiplication by a scalar,
on either side, against its `SMul` instance, all of which Azurite proves to be the entrywise
operations on Mathlib's `Fin n → ℤ`; multiplication by a scalar is also compared with a loop of
`Integer` products.

### Signs and absolute values {#signs-and-absolute-values}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [entrywise_abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/entrywise_abs/index.html) | `EntrywiseAbs`, `EntrywiseAbsAssign`, `EntrywiseUnsignedAbs` |   |   |   |
| [checked_abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/checked_abs/index.html) | `CheckedAbs` |   |   | ✓ |
| [canonicalize_sign](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/canonicalize_sign/index.html) | `CanonicalizeSign`, `CanonicalizeSignAssign` |   |   |   |

### Shifts {#shifts}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [shl](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/shl/index.html) | `Shl`, `ShlAssign` |   |   |   |
| [shr](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/shr/index.html) | `Shr`, `ShrAssign` |   |   |   |
| [entrywise_shl_round](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/entrywise_shl_round/index.html) | `EntrywiseShlRound`, `EntrywiseShlRoundAssign` |   |   |   |
| [entrywise_shr_round](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/entrywise_shr_round/index.html) | `EntrywiseShrRound`, `EntrywiseShrRoundAssign` |   |   |   |

### Dot products and norms {#dot-products-and-norms}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [dot](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/dot/index.html) | `Dot` | ✓ |   | ✓ |
| [dot_reversed](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/dot_reversed/index.html) | `DotReversed` |   |   | ✓ |
| [add_dot](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/add_dot/index.html) | `AddDot` |   |   | ✓ |
| [add_dot_reversed](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/add_dot_reversed/index.html) | `AddDotReversed` |   |   | ✓ |
| [sub_dot](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/sub_dot/index.html) | `SubDot` |   |   | ✓ |
| [sub_dot_reversed](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/sub_dot_reversed/index.html) | `SubDotReversed` |   |   | ✓ |
| [dot_sign](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/dot_sign/index.html) | `DotSign` | ✓ |   |   |
| [abs_squared](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/abs_squared/index.html) | `AbsSquared` | ✓ |   | ✓ |
| [l1_norm](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/l1_norm/index.html) | `L1Norm` |   |   |   |
| [height](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/height/index.html) | `Height`, `HeightRef` |   |   |   |
| [cross](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/cross/index.html) | `Cross`, `CrossAssign` | ✓ |   | ✓ |
| [generalized_cross](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/generalized_cross/index.html) | `GeneralizedCross` |   |   | ✓ |

The dot product, the squared length, and the cross product are checked against Azurite's
`AzVector.dot`, `AzVector.normSq`, and `AzVector.cross`, which are proven equal to Mathlib's
`dotProduct` and cross product. The dot products and the fused forms that add a dot product to, or
subtract it from, an `Integer` are compared with one schoolbook reference that sums the products one at a time; the reversed forms
pair each entry with the mirror-image entry of the other vector. `DotSign` is checked against
Azurite's `AzInt.dotSign`, whose screens on bit lengths and whose final comparison are proven
correct; [the mapping page](/mapping/azurite-integer-vectors/#arithmetic) describes the algorithm.
The cross product is compared with its defining formula, and the generalized cross product with
the determinants that define its entries.

### Content and products of entries {#content-and-products-of-entries}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [content](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/content/index.html) | `Content`, `PrimitivePart`, `PrimitivePartAssign`, `ContentAndPrimitivePart` |   |   |   |
| [canonical_primitive_part](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/canonical_primitive_part/index.html) | `CanonicalPrimitivePart`, `CanonicalPrimitivePartAssign`, `ContentAndCanonicalPrimitivePart` |   |   |   |
| [content_chained](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/content_chained/index.html) | `ContentChained` |   | ✓ | ✓ |
| [element_lcm](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/element_lcm/index.html) | `ElementLcm` |   | ✓ |   |
| [element_sum](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/element_sum/index.html) | `ElementSum` |   |   |   |
| [element_product](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/arithmetic/element_product/index.html) | `ElementProduct` |   |   |   |

`content_chained`, the greatest common divisor of a vector's entries and one more `Integer`, is
checked against FLINT's `_fmpz_vec_content_chained`, and `element_lcm` against `_fmpz_vec_lcm`.

## Modular arithmetic {#modular-arithmetic}

From
[`malachite_nz::integer_vector::modular`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/modular/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [mod_op](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/modular/mod_op/index.html) | `Mod`, `Rem`, `RemAssign` |   |   |   |
| [balanced_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/modular/balanced_mod/index.html) | `BalancedMod`, `BalancedModAssign` |   | ✓ |   |
| [mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/modular/mod_power_of_2/index.html) | `ModPowerOf2`, `RemPowerOf2`, `RemPowerOf2Assign` |   |   |   |
| [balanced_mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/modular/balanced_mod_power_of_2/index.html) | `BalancedModPowerOf2`, `BalancedModPowerOf2Assign` |   |   |   |
| [multi_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/modular/multi_mod/index.html) | `multi_mod` |   |   | ✓ |
| [multi_crt](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/modular/multi_crt/index.html) | `multi_balanced_crt` |   | ✓ | ✓ |

`balanced_mod`, the entrywise residues in $$(-m/2, m/2]$$, is checked against FLINT's
`fmpz_poly_scalar_smod_fmpz` applied to the vector's entries as coefficients. `multi_balanced_crt`,
which reconstructs a vector from its residues modulo several word-sized moduli, is checked against
FLINT's `_fmpz_vec_multi_CRT_ui`, and against a reference that reconstructs each entry separately
with `Integer`'s own `multi_balanced_crt`, whose checks are on [the integers
page](/verification/integers/#chinese-remaindering); `multi_mod`, the inverse, is compared with a
reference that reduces the whole vector modulo each modulus in turn.

## Linear algebra {#linear-algebra}

From
[`malachite_nz::integer_vector::linear_algebra`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/linear_algebra/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [are_linearly_independent](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/linear_algebra/are_linearly_independent/index.html) | `AreLinearlyIndependent` |   |   |   |
| [are_affinely_independent](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/linear_algebra/are_affinely_independent/index.html) | `AreAffinelyIndependent` |   |   |   |

Both are decided by the rank of a matrix built from the vectors, whose checks are on the integer
matrix page to come.

## Logic {#logic}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [significant_bits](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/logic/significant_bits/index.html) | `SignificantBits` |   |   |   |

## Conversion {#conversion}

From
[`malachite_nz::integer_vector::conversion`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [from_natural_vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/from_natural_vector/index.html) | `From<NaturalVector>` |   |   |   |
| [natural_vector_from_integer_vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/natural_vector_from_integer_vector/index.html) | `TryFrom<IntegerVector>` and `ConvertibleFrom<&IntegerVector>` for `NaturalVector` |   |   |   |
| [from_unsigned_vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/from_unsigned_vector/index.html) | `From<UnsignedVector<T>>` |   |   |   |
| [unsigned_vector_from_integer_vector](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/unsigned_vector_from_integer_vector/index.html) | `TryFrom<IntegerVector>` and `ConvertibleFrom<&IntegerVector>` for `UnsignedVector<T>` |   |   |   |
| [from_iterator](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/from_iterator/index.html) | `FromIterator<Integer>` |   |   |   |
| [into_iterator](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/into_iterator/index.html) | `IntoIterator` |   |   |   |

### Strings {#strings}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [to_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/string/to_string/index.html) | `Display`, `Debug` | ✓ |   |   |
| [from_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/string/from_string/index.html) | `FromStr` | ✓ |   |   |
| [latex](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/string/latex/index.html) | `ToLatex` |   |   |   |
| [typst](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/conversion/string/typst/index.html) | `ToTypst` |   |   |   |

Vectors are written as `(1, -2, 3)`, with each entry written by `Integer`'s `Display`, which is
also Azurite's format. Since the output demos print only the vector, Azurite's check of `Display`
and `Debug` is that its own parser reads each printed vector and its printer reproduces it
exactly, which rejects any non-canonical spacing, sign, or leading zero. `FromStr` is checked on
random strings and on printed vectors, with an oracle-side implementation of its documented
grammar (the elements separated by `", "` inside parentheses, each read by `Integer::from_str`'s
rule, which, like Azurite's integer parser, differs from `AzInt.parse` on `"-0"` and on base
prefixes) and Azurite's integers and printer.

## Exhaustive generation {#exhaustive-generation}

From [`malachite_nz::integer_vector::exhaustive`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/exhaustive/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/exhaustive/index.html) | `exhaustive_integer_vectors`, `exhaustive_integer_vectors_from_iterator` |   |   |   |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/exhaustive/index.html) | `exhaustive_integer_vectors_with_dimension`, `exhaustive_integer_vectors_with_dimension_from_iterator` |   |   |   |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/exhaustive/index.html) | `exhaustive_ordered_unique_integer_vectors_with_dimension` |   |   |   |

## Random generation {#random-generation}

From [`malachite_nz::integer_vector::random`](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/index.html) | `random_integer_vectors`, `random_integer_vectors_from_iterator`, `striped_random_integer_vectors` |   |   |   |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/index.html) | `random_integer_vectors_with_dimension`, `striped_random_integer_vectors_with_dimension` |   |   |   |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer_vector/random/index.html) | `random_ordered_unique_integer_vectors_with_dimension`, `random_ordered_unique_integer_vectors_with_dimension_from_iterator`, `striped_random_ordered_unique_integer_vectors_with_dimension` |   |   |   |

## What is not yet cross-checked {#not-yet-cross-checked}

The rows above with no mark, and the rows with no Azurite mark, sort into two groups by what it
would take to give them a verified oracle, after which come the rows outside the scope of an
oracle. Within each group the most widely used functions come
first.

**A few lines of oracle code around Azurite functions.** These need no new Azurite function, but
the oracle would compose existing ones over the entries, so the composition itself is unverified
code: the entrywise products, quotients, remainders, balanced remainders, shifts, absolute values,
minima, and maxima (each `AzInt` operation `map`ped or `zip`ped over the vector); the fused
operations (`add_mul`, `sub_mul`, `add_mul_shl`, `sub_mul_shl`) and the fused and reversed dot
products; the sums and products of the entries, the content and primitive parts, and `element_lcm`
(folds of `AzInt` addition, multiplication, `gcd`, and `lcm`); the L1 norm and the height; the
residues modulo a power of 2; `multi_mod` and `multi_balanced_crt` (Azurite's Garner algorithm on
each entry); and the conversions to and from `NaturalVector` and `UnsignedVector`.

**Azurite would need new functions.** The generalized cross product and the tests of linear and
affine independence, which need determinants or ranks of integer matrices, and the `Shortlex`
order, which Azurite's vectors do not have; the exhaustive generators, for which `AzVector` has no
`ExhaustiveGenerator` instance.

**Outside the scope of an oracle.** `Named`, `Hash`, `Clone`, the construction and access
functions of the vector interface, `Extend`, `FromIterator`, `IntoIterator`, LaTeX and Typst
output, serde, and the random generators. These are trivial or have no independent
implementation to compare with, and are tested by their own unit and property tests, the random
generators by the distribution of large samples.
