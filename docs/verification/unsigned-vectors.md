---
layout: default
title: "How Malachite Is Tested: Unsigned Vectors"
permalink: /verification/unsigned-vectors/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Unsigned Vectors

This page lists every public function of
[`UnsignedVector<T>`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html), the vector of primitive unsigned integers of the `malachite-base`
crate, and records which independent implementations it is checked against.
[The introduction](/verification/) describes the oracles and the testing they sit in; this page is
the ledger. It follows the organization of the crate's documentation, one section per module, so
that a function is where its documentation is. `UnsignedVector<T>` is generic over the entry type
`T`, which is one of `u8`, `u16`, `u32`, `u64`, `u128`, and `usize`, and its property tests run
each function at every one of these widths. Most of its functions are arithmetic modulo a number
$$m$$ or a power of 2 $$2^k$$ that fits in `T`, on vectors whose entries are already reduced; they
are what Malachite uses for vectors over $$\mathbb{Z}/m\mathbb{Z}$$ when $$m$$ is word-sized.
The conversions to and from [`NaturalVector`](/verification/natural-vectors/) are listed on that
page.

## Reading the tables {#reading-the-tables}

Each row is one operation, named after the module that implements it and linked to that module's
documentation. The **Functions** column lists the traits and methods the module implements; a
trait's by-value and by-reference implementations, and its `*Assign` form, count as one function,
since they compute the same thing. The remaining columns are the oracles of
[the introduction](/verification/#the-oracles) that have vector functions: Azurite, whose
`AzVector (AzZMod m) n` and `AzVector (AzZModPow2 k) n` are mapped
[here](/mapping/azurite-vectors-mod/) and [here](/mapping/azurite-vectors-mod-power-of-2/),
FLINT's `nmod_vec.h`, mapped [here](/mapping/flint-word-sized-integer-vectors-mod-n/), and the
reference implementations in `test_util`. Neither GMP nor num has a vector type, so their columns
are left out.

| | meaning |
| :---: | --- |
| ✓ | The oracle computes this function and agrees with Malachite on every input tried. |
| ≈ | The oracle agrees, after an adaptation on the oracle side that is more than a change of spelling: a convention of Malachite's that the oracle does not share is applied to the oracle's output before the comparison. The notes say what the adaptation is. |
| | The oracle does not check this function. |

Of the 99 rows, 20 have an Azurite check, 5 a FLINT check, and 11 a reference implementation;
69 are checked only by Malachite's own unit and property tests. Those rows are collected at the end
of the page, in [What is not yet cross-checked](#not-yet-cross-checked).

Functions that are not part of the public interface are not listed, such as the slice kernels
that the polynomial and matrix types share with `UnsignedVector`, and the alternative algorithms
that the tests compare with each other; they are exercised by the same runs as the functions built
on them.

## Basic {#basic}

The standard traits of the type itself, from
[`malachite_base::unsigned_vector`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [named](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Named-for-UnsignedVector%3Cu8%3E) | `Named` |   |   |   |
| [hash](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Hash-for-UnsignedVector%3CT%3E) | `Hash` |   |   |   |
| [clone](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Clone-for-UnsignedVector%3CT%3E) | `Clone` |   |   |   |

`Named` gives the type's name as a string, such as `UnsignedVector<u8>`, and `Hash` and `Clone`
are derived from the `Vec` of entries, so none has anything to compare against.

## The vector interface {#the-vector-interface}

The methods of the
[`Vector`](https://docs.rs/malachite-base/latest/malachite_base/vector/trait.Vector.html) trait,
which Malachite's vector types share, and the indexing and selection traits.

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `from_elements`, `from_owned_elements` |   |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `zero` | ✓ |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `set_zero`, `is_zero` |   |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `standard_basis_vector` | ✓ |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `standard_basis_index` |   |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `to_elements`, `into_elements`, `elements_ref` |   |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `dimension`, `push`, `set_dimension` |   |   |   |
| [extend](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/extend/index.html) | `Extend` |   |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `sort` |   |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `pivot`, `pivot_index`, `height_index` |   |   |   |
| [vector](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-Vector-for-UnsignedVector%3CT%3E) | `max_bits`, `sum_max_bits` |   |   |   |
| [access](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/access/index.html) | `Index` | ✓ |   |   |
| [access](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/access/index.html) | `IndexMut` | ✓ |   |   |
| [select_coordinates](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/select_coordinates/index.html) | `SelectCoordinates`, `SelectCoordinatesAssign` |   |   |   |

Most of these construct, read, and resize vectors and need no oracle beyond their own tests. The
zero vector, the standard basis vectors, indexing, and `IndexMut` are checked against Azurite's
`Zero` instance, `AzVector.stdBasis`, `AzVector.get`, and the `AzZModPow2 64` sum, with each `u64`
read as a residue modulo $$2^{64}$$, so that an entry too large for a `u64` is rejected and adding 1
to the largest one wraps around to 0, as the demo's `wrapping_add` does. The demos are of
`UnsignedVector<u64>`.

## Comparison {#comparison}

From [`malachite_base::unsigned_vector::comparison`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/comparison/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [eq](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/struct.UnsignedVector.html#impl-PartialEq-for-UnsignedVector%3CT%3E) | `PartialEq`, `Eq` |   |   |   |
| [shortlex_cmp](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/comparison/shortlex_cmp/index.html) | `Ord` and `PartialOrd` for `ShortlexUnsignedVector` and `ShortlexUnsignedVectorRef` |   |   | ✓ |
| [entrywise_max](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/comparison/entrywise_max/index.html) | `EntrywiseMax`, `EntrywiseMaxAssign` |   |   |   |
| [entrywise_min](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/comparison/entrywise_min/index.html) | `EntrywiseMin`, `EntrywiseMinAssign` |   |   |   |

`UnsignedVector` has no order of its own; the `Shortlex` wrappers order vectors by dimension and
then lexicographically, and that order is compared with a reference that builds the comparison
from the dimensions and the entries directly. Equality with a `NaturalVector` is on [the natural
vector page](/verification/natural-vectors/#comparison).

## Arithmetic {#arithmetic}

From [`malachite_base::unsigned_vector::arithmetic`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/index.html). The arithmetic
that is not modular is limited to operations whose results always fit in the entries: division,
right shifts, the height, and the content. Sums, products, and dot products are among the modular
operations below.

### Division, shifts, and height {#division-shifts-and-height}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [scalar_div](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/scalar_div/index.html) | `Div<T>`, `DivAssign<T>` |   |   |   |
| [div_exact](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/div_exact/index.html) | `DivExact`, `DivExactAssign` |   |   |   |
| [entrywise_div_round](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/entrywise_div_round/index.html) | `EntrywiseDivRound`, `EntrywiseDivRoundAssign` |   |   |   |
| [shr](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/shr/index.html) | `Shr`, `ShrAssign` |   |   |   |
| [entrywise_shr_round](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/entrywise_shr_round/index.html) | `EntrywiseShrRound`, `EntrywiseShrRoundAssign` |   |   |   |
| [height](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/height/index.html) | `Height` |   |   |   |

### Content {#content}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [content](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/content/index.html) | `Content`, `PrimitivePart`, `PrimitivePartAssign`, `ContentAndPrimitivePart` |   |   |   |
| [canonical_primitive_part](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/canonical_primitive_part/index.html) | `CanonicalPrimitivePart`, `CanonicalPrimitivePartAssign`, `ContentAndCanonicalPrimitivePart` |   |   |   |
| [content_chained](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/arithmetic/content_chained/index.html) | `ContentChained` |   | ✓ |   |

`content_chained`, the greatest common divisor of a vector's entries and one more entry, is
checked against FLINT's `_fmpz_vec_content_chained`.

## Modular arithmetic {#modular-arithmetic}

From [`malachite_base::unsigned_vector::modular`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/index.html). Every function but the
reductions requires its vectors to be reduced already, modulo $$m$$ or $$2^k$$, and panics
otherwise.

### Reduction {#reduction}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [mod_op](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_op/index.html) | `Mod`, `ModAssign`, `Rem`, `RemAssign` |   |   |   |
| [mod_power_of_2](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2/index.html) | `ModPowerOf2`, `ModPowerOf2Assign` |   |   |   |
| [mod_is_reduced](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_is_reduced/index.html) | `ModIsReduced` |   |   |   |
| [mod_power_of_2_is_reduced](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_is_reduced/index.html) | `ModPowerOf2IsReduced` |   |   |   |

### Modulo $$m$$ {#modulo-m}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [mod_add](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_add/index.html) | `ModAdd`, `ModAddAssign` | ✓ |   |   |
| [mod_sub](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_sub/index.html) | `ModSub`, `ModSubAssign` | ✓ |   |   |
| [mod_neg](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_neg/index.html) | `ModNeg`, `ModNegAssign` | ✓ |   |   |
| [mod_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_mul/index.html) | `ModMul`, `ModMulAssign` | ✓ |   |   |
| [mod_div](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_div/index.html) | `ModDiv` |   | ✓ | ✓ |
| [mod_shl](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_shl/index.html) | `ModShl`, `ModShlAssign` |   |   |   |
| [mod_add_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_add_mul/index.html) | `ModAddMul`, `ModAddMulAssign` |   |   |   |
| [mod_sub_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_sub_mul/index.html) | `ModSubMul`, `ModSubMulAssign` |   |   |   |
| [mod_add_mul_shl](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_add_mul_shl/index.html) | `ModAddMulShl`, `ModAddMulShlAssign` |   |   |   |
| [mod_sub_mul_shl](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_sub_mul_shl/index.html) | `ModSubMulShl`, `ModSubMulShlAssign` |   |   |   |
| [mod_entrywise_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_entrywise_mul/index.html) | `ModEntrywiseMul`, `ModEntrywiseMulAssign` |   |   |   |
| [mod_entrywise_inverse](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_entrywise_inverse/index.html) | `ModEntrywiseInverse` |   | ✓ | ✓ |
| [mod_sum](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_sum/index.html) | `ModSum` |   |   |   |
| [mod_element_sum](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_element_sum/index.html) | `ModElementSum` |   |   |   |
| [mod_element_product](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_element_product/index.html) | `ModElementProduct` |   |   |   |
| [mod_dot](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_dot/index.html) | `ModDot` | ✓ |   |   |
| [mod_dot_reversed](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_dot_reversed/index.html) | `ModDotReversed` |   |   |   |
| [mod_add_dot](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_add_dot/index.html) | `ModAddDot`, `ModAddDotAssign` |   |   |   |
| [mod_add_dot_reversed](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_add_dot_reversed/index.html) | `ModAddDotReversed`, `ModAddDotReversedAssign` |   |   |   |
| [mod_sub_dot](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_sub_dot/index.html) | `ModSubDot`, `ModSubDotAssign` |   |   |   |
| [mod_sub_dot_reversed](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_sub_dot_reversed/index.html) | `ModSubDotReversed`, `ModSubDotReversedAssign` |   |   |   |
| [mod_abs_squared](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_abs_squared/index.html) | `ModAbsSquared` | ✓ |   |   |
| [mod_cross](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_cross/index.html) | `ModCross`, `ModCrossAssign` | ✓ |   | ✓ |
| [mod_generalized_cross](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_generalized_cross/index.html) | `ModGeneralizedCross` |   |   | ✓ |

Azurite checks the entrywise sum, difference, and negation, the multiplication by a scalar, the
dot product, the squared length, and the cross product against `AzVector (AzZMod m) n`, whose
operations are proven to be those of `Fin n → AzZMod m`. The oracle requires every printed input
to be reduced modulo $$m$$, as Malachite's documentation does, so an unreduced input that Malachite
accepted would be a disagreement. The demos of the sum, difference, negation, dot product, and
squared length are of `UnsignedVector<u64>`; those of the scalar multiplication and the cross
product are of every width.

The entrywise inverse is checked against FLINT's `_nmod_vec_invert`. That function assumes every
entry is invertible, while Malachite returns `None` if one is not, so the oracle first decides,
with FLINT's `fmpz_gcd`, whether every entry is coprime to $$m$$, and compares the inverses only when
it is. Division by a scalar is checked against `_fmpz_mod_vec_scalar_div_fmpz_mod` when the
divisor is invertible and against `fmpz_mod_divides`, entry by entry, when it is not; both are FLINT's
own answers. Both are also compared with references that work entry by entry. The cross product is
compared with a reference that sums over the Levi-Civita symbol, and the generalized cross product
with the determinants that define its entries.

### Modulo $$2^k$$ {#modulo-a-power-of-2}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [mod_power_of_2_add](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_add/index.html) | `ModPowerOf2Add`, `ModPowerOf2AddAssign` | ✓ |   |   |
| [mod_power_of_2_sub](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_sub/index.html) | `ModPowerOf2Sub`, `ModPowerOf2SubAssign` | ✓ |   |   |
| [mod_power_of_2_neg](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_neg/index.html) | `ModPowerOf2Neg`, `ModPowerOf2NegAssign` | ✓ |   |   |
| [mod_power_of_2_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_mul/index.html) | `ModPowerOf2Mul`, `ModPowerOf2MulAssign` | ✓ |   |   |
| [mod_power_of_2_div](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_div/index.html) | `ModPowerOf2Div` |   | ≈ | ✓ |
| [mod_power_of_2_shl](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_shl/index.html) | `ModPowerOf2Shl`, `ModPowerOf2ShlAssign` |   |   |   |
| [mod_power_of_2_add_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_add_mul/index.html) | `ModPowerOf2AddMul`, `ModPowerOf2AddMulAssign` |   |   |   |
| [mod_power_of_2_sub_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_sub_mul/index.html) | `ModPowerOf2SubMul`, `ModPowerOf2SubMulAssign` |   |   |   |
| [mod_power_of_2_add_mul_shl](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_add_mul_shl/index.html) | `ModPowerOf2AddMulShl`, `ModPowerOf2AddMulShlAssign` |   |   |   |
| [mod_power_of_2_sub_mul_shl](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_sub_mul_shl/index.html) | `ModPowerOf2SubMulShl`, `ModPowerOf2SubMulShlAssign` |   |   |   |
| [mod_power_of_2_entrywise_mul](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_entrywise_mul/index.html) | `ModPowerOf2EntrywiseMul`, `ModPowerOf2EntrywiseMulAssign` |   |   |   |
| [mod_power_of_2_entrywise_inverse](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_entrywise_inverse/index.html) | `ModPowerOf2EntrywiseInverse` |   | ✓ | ✓ |
| [mod_power_of_2_sum](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_sum/index.html) | `ModPowerOf2Sum` |   |   |   |
| [mod_power_of_2_element_sum](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_element_sum/index.html) | `ModPowerOf2ElementSum` |   |   |   |
| [mod_power_of_2_element_product](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_element_product/index.html) | `ModPowerOf2ElementProduct` |   |   |   |
| [mod_power_of_2_dot](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_dot/index.html) | `ModPowerOf2Dot` | ✓ |   |   |
| [mod_power_of_2_dot_reversed](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_dot_reversed/index.html) | `ModPowerOf2DotReversed` |   |   |   |
| [mod_power_of_2_add_dot](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_add_dot/index.html) | `ModPowerOf2AddDot`, `ModPowerOf2AddDotAssign` |   |   |   |
| [mod_power_of_2_add_dot_reversed](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_add_dot_reversed/index.html) | `ModPowerOf2AddDotReversed`, `ModPowerOf2AddDotReversedAssign` |   |   |   |
| [mod_power_of_2_sub_dot](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_sub_dot/index.html) | `ModPowerOf2SubDot`, `ModPowerOf2SubDotAssign` |   |   |   |
| [mod_power_of_2_sub_dot_reversed](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_sub_dot_reversed/index.html) | `ModPowerOf2SubDotReversed`, `ModPowerOf2SubDotReversedAssign` |   |   |   |
| [mod_power_of_2_abs_squared](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_abs_squared/index.html) | `ModPowerOf2AbsSquared` | ✓ |   |   |
| [mod_power_of_2_cross](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_cross/index.html) | `ModPowerOf2Cross`, `ModPowerOf2CrossAssign` | ✓ |   | ✓ |
| [mod_power_of_2_generalized_cross](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/modular/mod_power_of_2_generalized_cross/index.html) | `ModPowerOf2GeneralizedCross` |   |   | ✓ |

The same seven operations as modulo $$m$$ are checked against `AzVector (AzZModPow2 k) n`, with
the inputs again required to be reduced, and over the same widths.

The entrywise inverse is checked against `_nmod_vec_invert` with $$m = 2^k$$, in the same way as
modulo $$m$$. Division by an even scalar has many quotients modulo $$2^k$$ when it has any, and
Malachite returns the smallest; FLINT's `fmpz_mod_divides` need not, so the oracle uses it only to
decide whether each quotient exists and checks Malachite's quotient $$w$$ directly, that $$wc$$ is
congruent to the entry and that $$w < 2^{k - s}$$, where $$s$$ is the number of trailing zeros of
$$c$$. That is an adaptation to Malachite's convention, so the FLINT mark is ≈.

## Linear algebra {#linear-algebra}

From [`malachite_base::unsigned_vector::linear_algebra`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/linear_algebra/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [mod_are_linearly_independent](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/linear_algebra/mod_are_linearly_independent/index.html) | `ModAreLinearlyIndependent` |   |   | ✓ |
| [mod_power_of_2_are_linearly_independent](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/linear_algebra/mod_power_of_2_are_linearly_independent/index.html) | `ModPowerOf2AreLinearlyIndependent` |   |   |   |
| [mod_are_affinely_independent](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/linear_algebra/mod_are_affinely_independent/index.html) | `ModAreAffinelyIndependent` |   |   | ✓ |
| [mod_power_of_2_are_affinely_independent](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/linear_algebra/mod_power_of_2_are_affinely_independent/index.html) | `ModPowerOf2AreAffinelyIndependent` |   |   |   |

Linear independence modulo $$m$$ is compared, on small inputs, with a reference that tries every
combination of coefficients, and affine independence with the same reference applied to the
points' homogeneous coordinates. The tests modulo $$2^k$$ compare the alternative algorithms for
each with one another and with the tests modulo $$m$$ at $$m = 2^k$$; the brute-force reference
appears only in their unit tests.

## Logic {#logic}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [significant_bits](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/logic/significant_bits/index.html) | `SignificantBits` |   |   |   |

## Conversion {#conversion}

From [`malachite_base::unsigned_vector::conversion`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/conversion/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [from_iterator](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/conversion/from_iterator/index.html) | `FromIterator<T>` |   |   |   |
| [into_iterator](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/conversion/into_iterator/index.html) | `IntoIterator` |   |   |   |

### Strings {#strings}

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [to_string](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/conversion/string/to_string/index.html) | `Display`, `Debug` | ✓ |   |   |
| [from_string](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/conversion/string/from_string/index.html) | `FromStr` | ✓ |   |   |
| [latex](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/conversion/string/latex/index.html) | `ToLatex` |   |   |   |
| [typst](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/conversion/string/typst/index.html) | `ToTypst` |   |   |   |

Vectors are written as `(1, 2, 3)`, with each entry written by `T`'s `Display`, which is also
Azurite's format. Since the output demos print only the vector, Azurite's check of `Display` and
`Debug` is that its own parser reads each printed vector and its printer reproduces it exactly,
which rejects any non-canonical spacing or leading zero, and any entry of $$2^{64}$$ or more.
`FromStr` is checked on random strings and on printed vectors, with an oracle-side implementation
of its documented grammar (the entries separated by `", "` inside parentheses, each read by
`u64::from_str`'s rule: a single optional `+` and decimal digits, of a value below $$2^{64}$$) and
Azurite's printer. The demos' strings have no entry of $$2^{64}$$ or more, so the oracle's
rejection of one has been tried only on hand-written lines. The demos are of `UnsignedVector<u64>`.

## Exhaustive generation {#exhaustive-generation}

From [`malachite_base::unsigned_vector::exhaustive`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/exhaustive/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [exhaustive](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/exhaustive/index.html) | `exhaustive_unsigned_vectors`, `exhaustive_unsigned_vectors_from_iterator` |   |   |   |
| [exhaustive](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/exhaustive/index.html) | `exhaustive_unsigned_vectors_with_dimension`, `exhaustive_unsigned_vectors_with_dimension_from_iterator` |   |   |   |
| [exhaustive](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/exhaustive/index.html) | `exhaustive_ordered_unique_unsigned_vectors_with_dimension` |   |   |   |

## Random generation {#random-generation}

From [`malachite_base::unsigned_vector::random`](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/random/index.html).

| Operation | Functions | Azurite | FLINT | Reference |
| --- | --- | :---: | :---: | :---: |
| [random](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/random/index.html) | `random_unsigned_vectors`, `random_unsigned_vectors_from_iterator`, `striped_random_unsigned_vectors` |   |   |   |
| [random](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/random/index.html) | `random_unsigned_vectors_with_dimension`, `random_unsigned_vectors_with_dimension_from_iterator`, `striped_random_unsigned_vectors_with_dimension` |   |   |   |
| [random](https://docs.rs/malachite-base/latest/malachite_base/unsigned_vector/random/index.html) | `random_ordered_unique_unsigned_vectors_with_dimension`, `random_ordered_unique_unsigned_vectors_with_dimension_from_iterator`, `striped_random_ordered_unique_unsigned_vectors_with_dimension` |   |   |   |

## What is not yet cross-checked {#not-yet-cross-checked}

The rows above with no mark, and the rows with no Azurite mark, sort into two groups by what it
would take to give them a verified oracle, after which come the rows outside the scope of an
oracle. Within each group the most widely used functions come first.

**A few lines of oracle code around Azurite functions.** These need no new Azurite function, but
the oracle would compose existing ones over the entries, so the composition itself is unverified
code: the entrywise quotients, shifts, minima, and maxima, and the entrywise products modulo
$$m$$ and $$2^k$$ (each `AzNat`, `AzZMod`, or `AzZModPow2` operation `map`ped or `zip`ped over the
vector); the fused operations and the fused and reversed dot products; the sums of vectors and the
sums and products of the entries modulo $$m$$ and $$2^k$$; the content and primitive parts and the
height; the modular shifts; the reductions and reducedness tests; and the entrywise inverses and
the division by a scalar modulo $$m$$ and $$2^k$$.

**Azurite would need new functions.** The generalized cross products and the tests of linear and
affine independence, which need determinants, ranks, or Howell forms of matrices; the `Shortlex`
order, which Azurite's vectors do not have; and the exhaustive generators, for which `AzVector` has
no `ExhaustiveGenerator` instance.

**Outside the scope of an oracle.** `Named`, `Hash`, `Clone`, the construction and access
functions of the vector interface, `Extend`, `FromIterator`, `IntoIterator`, LaTeX and Typst
output, serde, and the random generators. These are trivial or have no independent
implementation to compare with, and are tested by their own unit and property tests, the random
generators by the distribution of large samples.
