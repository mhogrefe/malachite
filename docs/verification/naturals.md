---
layout: default
title: "How Malachite Is Tested: Naturals"
permalink: /verification/naturals/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Naturals

This page lists every public function of
[`Natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html), the
unsigned integer type of the `malachite-nz` crate, and records which independent implementations it
is checked against. [The introduction](/verification/) describes the oracles and the testing they
sit in; this page is the ledger. It follows the organization of the crate's documentation, one
section per module, so that a function is where its documentation is.

## Reading the tables {#reading-the-tables}

Each row is one operation, named after the module that implements it and linked to that module's
documentation. The **Functions** column lists the traits and methods the module implements; a
trait's by-value and by-reference implementations, and its `*Assign` form, count as one function,
since they compute the same thing, while functions with genuinely different results (`div_mod`
against `div_rem`, `Add` against `Sum`) are listed separately. The remaining columns are the
oracles of [the introduction](/verification/#the-oracles):

| | meaning |
| :---: | --- |
| ✓ | The oracle computes this function and agrees with Malachite on every input tried. |
| ≈ | The oracle agrees, after an adaptation on the oracle side that is more than a change of spelling: a convention of Malachite's that the oracle does not share is applied to the oracle's output before the comparison. The mapping pages say what the adaptation is. |
| | The oracle does not check this function. |

A function with no mark in any column is checked only by Malachite's own unit and property tests.
Those rows are collected at the end of the page, in
[What is not yet cross-checked](#not-yet-cross-checked), which is the list of what the oracles,
Azurite first, should gain next.

Functions that are not part of the public interface are not listed: the `limbs_*` functions that
operate on limb slices and the other `#[doc(hidden)]` helpers are exercised by the same runs as
the functions built on them, but they are implementation details.

## Basic {#basic}

The constants and the standard traits of the type itself, from
[`malachite_nz::natural`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [constants](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html) | `Zero`, `One`, `Two`, `Min` | | | | | |
| [default](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#impl-Default-for-Natural) | `Default` | | | | | |
| [named](https://docs.rs/malachite-nz/latest/malachite_nz/natural/struct.Natural.html#impl-Named-for-Natural) | `Named` | | | | | |

The constants are the values `0`, `1`, and `2`, and `MIN` is `0`; every oracle run uses them on
the way to checking something else, but no run checks them as such. `Named` gives the type's name
as a string and has nothing to compare against.

## Comparison {#comparison}

From
[`malachite_nz::natural::comparison`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [cmp](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/cmp/index.html) | `Ord`, `PartialOrd` | ✓ | | ✓ | ✓ | |
| [cmp](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/cmp/index.html) | `cmp_normalized` | ✓ | | | | ✓ |
| [cmp](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/cmp/index.html) | `cmp_normalized_no_shift` | | | | | |
| [cmp_double](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/cmp_double/index.html) | `OrdDouble`, `PartialOrdDouble` | | | | | |
| [eq](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/eq/index.html) | `PartialEq`, `Eq` | ✓ | | ✓ | ✓ | |
| [hash](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/hash/index.html) | `Hash` | | | | | |
| [partial_cmp_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_cmp_primitive_int/index.html) | `PartialOrd<u8>`, …, `PartialOrd<isize>`, and the reverse directions | ✓ | | ✓ | ✓ | |
| [partial_eq_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_eq_primitive_int/index.html) | `PartialEq<u8>`, …, `PartialEq<isize>`, and the reverse directions | ✓ | | ✓ | ✓ | |
| [partial_cmp_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_cmp_primitive_float/index.html) | `PartialOrd<f32>`, `PartialOrd<f64>`, and the reverse directions | | | ✓ | | |
| [partial_eq_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_eq_primitive_float/index.html) | `PartialEq<f32>`, `PartialEq<f64>`, and the reverse directions | | | ✓ | | |
| [partial_cmp_abs_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_cmp_abs_primitive_int/index.html) | `PartialOrdAbs<u8>`, …, `PartialOrdAbs<isize>`, and the reverse directions | | | ✓ | | |
| [eq_abs_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/eq_abs_primitive_int/index.html) | `EqAbs<u8>`, …, `EqAbs<isize>`, and the reverse directions | | | | | |
| [partial_cmp_abs_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/partial_cmp_abs_primitive_float/index.html) | `PartialOrdAbs<f32>`, `PartialOrdAbs<f64>`, and the reverse directions | | | | | |
| [eq_abs_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/natural/comparison/eq_abs_primitive_float/index.html) | `EqAbs<f32>`, `EqAbs<f64>`, and the reverse directions | | | | | |

`cmp` is checked by Azurite's `AzNat.compare` and by GMP and num on the same inputs, and `eq` by
Azurite's structural equality and by GMP and num. `cmp_normalized`, the comparison of two values
after aligning their leading bits, is checked by Azurite's `normalizedCompare` and a reference
implementation that shifts and compares; its `_no_shift` variant, which assumes the alignment has
been done, is not. The comparisons and equalities with primitive integers, in both directions, are
checked by Azurite's `compareUInt64` and `beqUInt64` (every natural being greater than a negative
primitive) and against GMP, and the integer ones against num as well; the comparisons with floats
are checked against GMP, which converts the float exactly. The magnitude comparisons (`EqAbs`,
`PartialOrdAbs`) are the plain comparisons on a type with no sign, and only the integer
`PartialOrdAbs` is cross-checked, against GMP; the others are currently covered by property tests
alone.

## Arithmetic {#arithmetic}

From
[`malachite_nz::natural::arithmetic`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/index.html).
The module is large, so its rows are grouped by theme.

### Addition, subtraction, and multiplication {#addition-subtraction-and-multiplication}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [add](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/add/index.html) | `Add` | ✓ |  | ✓ | ✓ |  |
| [add](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/add/index.html) | `Sum` |  |  |  |  | ✓ |
| [sub](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/sub/index.html) | `Sub` | ✓ |  | ✓ | ✓ |  |
| [checked_sub](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/checked_sub/index.html) | `CheckedSub` |  |  | ✓ | ✓ |  |
| [saturating_sub](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/saturating_sub/index.html) | `SaturatingSub` | ✓ |  |  |  |  |
| [abs_diff](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/abs_diff/index.html) | `AbsDiff` |  |  |  |  |  |
| [neg](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/neg/index.html) | `Neg` | ✓ |  | ✓ | ✓ |  |
| [mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mul/index.html) | `Mul` | ✓ |  | ✓ | ✓ |  |
| [mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mul/index.html) | `Product` |  |  |  |  | ✓ |
| [square](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/square/index.html) | `Square` | ✓ |  |  |  |  |
| [abs_squared](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/abs_squared/index.html) | `AbsSquared` |  |  |  |  |  |
| [add_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/add_mul/index.html) | `AddMul` |  |  |  |  |  |
| [sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/sub_mul/index.html) | `SubMul` |  |  |  |  |  |
| [checked_sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/checked_sub_mul/index.html) | `CheckedSubMul` |  |  |  |  |  |
| [saturating_sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/saturating_sub_mul/index.html) | `SaturatingSubMul` |  |  |  |  |  |
| [mul_add_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mul_add_mul/index.html) | `MulAddMul` |  |  |  |  |  |
| [mul_sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mul_sub_mul/index.html) | `MulSubMul` |  |  |  |  |  |
| [checked_mul_sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/checked_mul_sub_mul/index.html) | `CheckedMulSubMul` |  |  |  |  |  |
| [saturating_mul_sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/saturating_mul_sub_mul/index.html) | `SaturatingMulSubMul` |  |  |  |  |  |
| [average](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/average/index.html) | `Average` |  |  |  |  |  |
| [average](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/average/index.html) | `AverageRound` |  |  |  |  |  |

The four basic operations are checked by Azurite (`AzNat.add`, `sub`, `mul`, `square`) and by GMP
and num, and `saturating_sub` by Azurite, whose subtraction truncates at zero exactly as
`saturating_sub` does; Malachite's `-` panics where the difference would be negative, so the lines
it prints are differences that `AzNat.sub` computes exactly. `Sum` and `Product` are checked against
a reference implementation that folds `+` and `*` one term at a time, and `Neg`, which turns a
`Natural` into an `Integer`, by Azurite's `AzInt.neg` and against GMP and num. The fused operations
(`add_mul`, `sub_mul`, `mul_add_mul`, and their checked and saturating forms), `abs_diff`,
`abs_squared`, and `average` are not yet cross-checked: their property tests compare them with the
unfused combinations of operations that are, which is a strong check but not an independent one.

### Shifts {#shifts}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [shl](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/shl/index.html) | `Shl<u8>`, …, `Shl<isize>` | ✓ |  | ✓ | ✓ |  |
| [shr](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/shr/index.html) | `Shr<u8>`, …, `Shr<isize>` | ✓ |  | ✓ | ✓ |  |
| [shl_round](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/shl_round/index.html) | `ShlRound<i8>`, …, `ShlRound<isize>` |  |  |  |  |  |
| [shr_round](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/shr_round/index.html) | `ShrRound<u8>`, …, `ShrRound<isize>` | ✓ |  |  |  |  |
| [mul_shr_round](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mul_shr_round/index.html) | `MulShrRound` |  |  |  |  |  |
| [round_to_multiple_of_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/round_to_multiple_of_power_of_2/index.html) | `RoundToMultipleOfPowerOf2` |  |  |  |  |  |

Shifting by a primitive integer is checked by Azurite's `shiftLeft` and `shiftRight` for every shift
type, a negative count reversing the direction, and against GMP; the unsigned shifts also against
num. `shr_round`, which shifts right and rounds the result in a given mode, is checked by Azurite's
`shiftRightRound` for every primitive shift type, a negative shift count being a left shift; the
`Exact` mode, which Azurite does not have, is checked as "the shift is exact and the `Ordering` is
`Equal`".

### Division {#division}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [div](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div/index.html) | `Div` | ✓ |  | ✓ | ✓ |  |
| [div](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div/index.html) | `CheckedDiv` |  |  |  | ✓ |  |
| [div_mod](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div_mod/index.html) | `DivMod`, `DivRem` | ✓ |  | ✓ | ✓ |  |
| [div_mod](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div_mod/index.html) | `CeilingDivNegMod` |  |  | ✓ |  |  |
| [div_mod](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div_mod/index.html) | `DivModPrecomputed` |  |  |  |  |  |
| [mod_op](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_op/index.html) | `Mod`, `Rem` | ✓ |  | ✓ | ✓ |  |
| [mod_op](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_op/index.html) | `NegMod` |  |  | ✓ |  |  |
| [div_euclidean](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div_euclidean/index.html) | `DivEuclidean` | ✓ |  |  |  |  |
| [mod_euclidean](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_euclidean/index.html) | `ModEuclidean` | ✓ |  |  |  |  |
| [div_mod_euclidean](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div_mod_euclidean/index.html) | `DivModEuclidean` | ✓ |  |  |  |  |
| [div_exact](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div_exact/index.html) | `DivExact` |  |  | ✓ |  |  |
| [div_round](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/div_round/index.html) | `DivRound` | ✓ |  | ✓ | ✓ |  |
| [divisible_by](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/divisible_by/index.html) | `DivisibleBy` | ✓ |  | ✓ | ✓ |  |
| [round_to_multiple](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/round_to_multiple/index.html) | `RoundToMultiple` |  |  |  |  |  |
| [balanced_mod](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/balanced_mod/index.html) | `BalancedMod` |  |  |  |  |  |

Truncating division is checked three ways: `/`, `div_mod`, `div_rem`, `%`, and `mod_op` (which
coincide on naturals) against Azurite's `div`, `mod`, and `divMod`, and against GMP and num. The
Euclidean forms, which also coincide with the truncating ones on naturals, and `divisible_by` are
checked by Azurite's `divMod` and `mod` as well. `div_round` is checked by Azurite's `divRound` in
every rounding mode, `Exact` being checked as an exact division, and against GMP. The forms that
round the quotient up (`ceiling_div_neg_mod`, `neg_mod`), `div_exact`, and `divisible_by` are
checked against GMP. `DivModPrecomputed`, which reuses a precomputed inverse of the divisor, is
compared with `div_mod`, and `balanced_mod`, the remainder in $$(-m/2, m/2]$$, is tested with the
integer version.

### Signs, units, and parity {#signs-units-and-parity}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [sign](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/sign/index.html) | `Sign` |  |  | ✓ |  |  |
| [parity](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/parity/index.html) | `Parity` | ✓ |  |  |  |  |
| [is_unit](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/is_unit/index.html) | `IsUnit` |  |  |  |  |  |
| [conjugate](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/conjugate/index.html) | `Conjugate` |  |  |  |  |  |
| [canonicalize_unit](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/canonicalize_unit/index.html) | `CanonicalizeUnit` |  |  |  |  |  |
| [canonical_unit_i_pow](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/canonical_unit_i_pow/index.html) | `CanonicalUnitIPow` |  |  |  |  |  |

`sign` is checked against GMP and `even`/`odd` against Azurite's `isEven`/`isOdd`. The unit
functions (`is_unit`, `conjugate`, `canonicalize_unit`, `canonical_unit_i_pow`) are the trivial
cases of functions that matter for Gaussian integers and polynomials, where they are cross-checked.

### Modular arithmetic {#modular-arithmetic}

The functions that take a modulus $$m$$ and require their inputs to be reduced modulo it.

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [mod_is_reduced](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_is_reduced/index.html) | `ModIsReduced` | ✓ |  |  |  |  |
| [mod_add](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_add/index.html) | `ModAdd` | ✓ |  |  |  |  |
| [mod_sub](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_sub/index.html) | `ModSub` | ✓ |  |  |  |  |
| [mod_neg](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_neg/index.html) | `ModNeg` | ✓ |  |  |  |  |
| [mod_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_mul/index.html) | `ModMul` | ✓ |  |  |  |  |
| [mod_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_mul/index.html) | `ModMulPrecomputed` | ✓ |  |  |  |  |
| [mod_square](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_square/index.html) | `ModSquare` | ✓ |  |  |  |  |
| [mod_square](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_square/index.html) | `ModSquarePrecomputed` | ✓ |  |  |  |  |
| [mod_pow](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_pow/index.html) | `ModPow` | ✓ |  | ✓ | ✓ | ✓ |
| [mod_pow](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_pow/index.html) | `ModPowPrecomputed` | ✓ |  |  |  |  |
| [mod_shl](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_shl/index.html) | `ModShl<u8>`, …, `ModShl<isize>` | ✓ |  |  |  |  |
| [mod_shr](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_shr/index.html) | `ModShr<i8>`, …, `ModShr<isize>` | ✓ |  |  |  |  |
| [mod_inverse](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_inverse/index.html) | `ModInverse` | ✓ |  |  |  | ✓ |
| [mod_div](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_div/index.html) | `ModDiv` | ≈ | ✓ |  |  | ✓ |
| [mod_div_list](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_div_list/index.html) | `ModDivList` |  | ✓ |  |  | ✓ |
| [mod_sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_sqrt/index.html) | `ModSqrt` |  | ≈ |  |  |  |
| [eq_mod](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/eq_mod/index.html) | `EqMod` | ✓ |  | ✓ |  |  |

Azurite checks the whole family through its residue type `AzZMod m`, whose invariant (a value in
$$[0, m)$$) is the precondition these functions assert, so every input is first checked to be
reduced: addition, subtraction, negation, multiplication, squaring, powers, shifts, inverses, and
the three `_precomputed` variants, which are compared with Azurite's plain operations since the
precomputed data affects only speed. `mod_pow` is also checked against GMP's and num's modular
exponentiation and a square-and-multiply reference, and `mod_inverse` against a reference
extended-Euclid inverse.

`mod_div` is ≈ for Azurite because the quotient is not unique when the divisor is not a unit:
Malachite documents that it returns one of the quotients, so the oracle checks the documented
existence condition ($$\gcd(y, m) \mid x$$) and that the printed quotient $$q$$ satisfies
$$qy \equiv x \pmod m$$, rather than comparing values. FLINT's `fmpz_mod_divides` makes the same
choice of quotient as Malachite and is compared exactly. `mod_sqrt` is ≈ for FLINT because for
even moduli between 50 and 600 FLINT's `fmpz_sqrtmod` calls a Jacobi symbol routine with an even
modulus, whose behavior is undefined; that documented window is skipped, and every other input,
every odd modulus included, is compared exactly.

### Arithmetic modulo a power of 2 {#arithmetic-modulo-a-power-of-2}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2/index.html) | `ModPowerOf2` | ✓ |  |  |  |  |
| [mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2/index.html) | `RemPowerOf2` | ✓ |  |  |  |  |
| [mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2/index.html) | `NegModPowerOf2` | ✓ |  |  |  |  |
| [mod_power_of_2_is_reduced](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_is_reduced/index.html) | `ModPowerOf2IsReduced` | ✓ |  |  |  |  |
| [mod_power_of_2_add](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_add/index.html) | `ModPowerOf2Add` | ✓ |  |  |  |  |
| [mod_power_of_2_sub](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_sub/index.html) | `ModPowerOf2Sub` | ✓ |  |  |  |  |
| [mod_power_of_2_neg](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_neg/index.html) | `ModPowerOf2Neg` | ✓ |  |  |  |  |
| [mod_power_of_2_mul](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_mul/index.html) | `ModPowerOf2Mul` | ✓ |  |  |  |  |
| [mod_power_of_2_square](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_square/index.html) | `ModPowerOf2Square` | ✓ |  |  |  |  |
| [mod_power_of_2_pow](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_pow/index.html) | `ModPowerOf2Pow` | ✓ |  |  |  | ✓ |
| [mod_power_of_2_inverse](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_inverse/index.html) | `ModPowerOf2Inverse` | ✓ |  |  |  |  |
| [mod_power_of_2_shl](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_shl/index.html) | `ModPowerOf2Shl<u8>`, …, `ModPowerOf2Shl<isize>` | ✓ |  |  |  |  |
| [mod_power_of_2_shr](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/mod_power_of_2_shr/index.html) | `ModPowerOf2Shr<i8>`, …, `ModPowerOf2Shr<isize>` | ✓ |  |  |  |  |
| [eq_mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/eq_mod_power_of_2/index.html) | `EqModPowerOf2` | ✓ |  | ✓ |  |  |

Azurite checks these through its type `AzZModPow2 k`, the residues modulo $$2^k$$, in the same way
as the general family, and `ModPowerOf2` and `rem_power_of_2` (which coincide on naturals) through
`AzNat.modPow2`; `neg_mod_power_of_2`, which takes any natural, is checked as the negation in
`AzZModPow2 k` of its residue. `mod_power_of_2_pow` is also checked against a square-and-multiply
reference, and `eq_mod_power_of_2` against GMP's `is_congruent_2pow`.

### Chinese remaindering {#chinese-remaindering}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [crt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/crt/index.html) | `Crt` | ✓ | ✓ |  |  | ✓ |
| [multi_crt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/multi_crt/index.html) | `Natural::multi_crt`, `MultiCrt::new`, `MultiCrt::apply` | ✓ | ✓ |  |  | ✓ |
| [multi_crt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/multi_crt/index.html) | `MultiCrt::apply_balanced` |  |  |  |  |  |
| [crt_comb](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/crt_comb/index.html) | `CrtComb::new`, `CrtComb::reduce` |  | ✓ |  |  |  |
| [crt_comb](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/crt_comb/index.html) | `CrtComb::combine` |  | ✓ |  |  |  |
| [crt_comb](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/crt_comb/index.html) | `CrtComb::combine_balanced` |  | ✓ |  |  |  |

`crt` is checked by Azurite's Garner algorithm on the two moduli (its `None` being checked as moduli
that are not coprime), FLINT's `fmpz_CRT`, and a reference that combines the two congruences with an
extended GCD; `multi_crt` by Azurite's Garner algorithm (its `None` result being checked as a
violated precondition: moduli that are not pairwise coprime, or a residue that is not reduced),
FLINT's `fmpz_multi_CRT`, and a reference that folds `crt`. `multi_crt` is built on `MultiCrt::new`
and `MultiCrt::apply`, so those are checked through it; `apply_balanced`, which returns the residue
in $$(-M/2, M/2]$$, is not yet. The `CrtComb` precomputation is checked against FLINT's
`fmpz_multi_mod_ui` and `fmpz_multi_CRT_ui` family.

### Powers, roots, and logarithms {#powers-roots-and-logarithms}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [pow](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/pow/index.html) | `Pow<u64>` | ✓ |  | ✓ | ✓ | ✓ |
| [sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/sqrt/index.html) | `FloorSqrt` | ✓ |  | ✓ | ✓ | ✓ |
| [sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/sqrt/index.html) | `CeilingSqrt` | ✓ |  |  |  | ✓ |
| [sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/sqrt/index.html) | `CheckedSqrt` | ✓ |  |  |  | ✓ |
| [sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/sqrt/index.html) | `SqrtRem` | ✓ |  | ✓ |  | ✓ |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/root/index.html) | `FloorRoot<u64>` | ✓ |  | ✓ | ✓ | ✓ |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/root/index.html) | `CeilingRoot<u64>` |  |  |  |  | ✓ |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/root/index.html) | `CheckedRoot<u64>` | ✓ |  |  |  | ✓ |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/root/index.html) | `RootRem<u64>` |  |  | ✓ |  | ✓ |
| [log_base](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/log_base/index.html) | `FloorLogBase`, `CeilingLogBase`, `CheckedLogBase` | ✓ |  |  |  | ✓ |
| [log_base](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/log_base/index.html) | `approx_ln` |  |  |  |  |  |
| [log_base_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/log_base_2/index.html) | `FloorLogBase2`, `CeilingLogBase2`, `CheckedLogBase2` | ✓ |  |  |  |  |
| [log_base_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/log_base_power_of_2/index.html) | `FloorLogBasePowerOf2`, `CheckedLogBasePowerOf2` | ✓ |  |  |  |  |
| [log_base_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/log_base_power_of_2/index.html) | `CeilingLogBasePowerOf2` | ✓ |  |  |  | ✓ |

`pow`, `floor_sqrt`, `sqrt_rem`, `ceiling_sqrt`, `checked_sqrt`, `floor_root` (cube roots included),
and `checked_root` are checked by Azurite (the ceiling and checked square roots read off `sqrtRem`),
and the floor and remainder forms by GMP (and the floors by num). Every root function is also
checked against a reference that finds the root by binary search, and `pow` against both repeated
multiplication and square-and-multiply; `ceiling_root` is cross-checked only through that reference.
The logarithms are checked by Azurite: the base-2 and base-$$2^k$$ logarithms from `AzNat.size` and
`isPowerOfTwo`, and `floor_log_base` and its siblings by `AzRat.floorLogBaseAbs` and `cmpPowAbs` for
a base that fits in a limb (and by repeated multiplication in `AzNat` arithmetic for a larger one);
the general logarithms are also compared with two references (a naive loop of multiplications and a
search by repeated squaring). `approx_ln`, a floating-point estimate, is covered by property tests
alone.

### Powers of 2 {#powers-of-2}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/power_of_2/index.html) | `PowerOf2<u64>` | ✓ |  |  |  |  |
| [is_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/is_power_of_2/index.html) | `IsPowerOf2` | ✓ |  | ✓ |  |  |
| [next_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/next_power_of_2/index.html) | `NextPowerOf2` |  |  | ✓ |  |  |
| [divisible_by_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/divisible_by_power_of_2/index.html) | `DivisibleByPowerOf2` | ✓ |  | ✓ |  |  |

`power_of_2`, `is_power_of_2`, and `divisible_by_power_of_2` are checked by Azurite, and the last
three functions by GMP.

### GCD and number theory {#gcd-and-number-theory}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [gcd](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/gcd/index.html) | `Gcd` | ✓ |  | ✓ | ✓ | ✓ |
| [gcd](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/gcd/extended_gcd/index.html) | `ExtendedGcd` | ≈ |  | ✓ |  | ✓ |
| [gcd](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/gcd/extended_gcd_partial/index.html) | `extended_gcd_partial` |  | ✓ |  |  |  |
| [lcm](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/lcm/index.html) | `Lcm` |  |  | ✓ | ✓ |  |
| [coprime_with](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/coprime_with/index.html) | `CoprimeWith` | ✓ |  |  |  |  |
| [kronecker_symbol](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/kronecker_symbol/index.html) | `JacobiSymbol` | ✓ |  | ✓ |  | ✓ |
| [kronecker_symbol](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/kronecker_symbol/index.html) | `LegendreSymbol` |  |  | ✓ |  | ✓ |
| [kronecker_symbol](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/kronecker_symbol/index.html) | `KroneckerSymbol` |  |  | ✓ |  |  |

`gcd` is checked by Azurite's binary GCD, GMP, num, and two references (Euclid's algorithm and the
binary algorithm), and `coprime_with` by Azurite. `extended_gcd` is ≈ for Azurite because a Bézout
pair is not unique and Azurite normalizes its pair differently: the oracle checks that the GCDs
agree, that Malachite's pair satisfies $$sa + tb = g$$, and that it is the pair Malachite
documents (bounded by $$|s| \le b/g$$, $$|t| \le a/g$$, with the fixed values for zeros and
divisors); GMP and the two references compare the pair exactly. `extended_gcd_partial`, the
partial extended GCD used in continued-fraction and lattice code, is checked against FLINT's
`fmpz_xgcd_partial`. `jacobi_symbol` is checked by Azurite, GMP, and a reference, and the
Legendre and Kronecker symbols by GMP.

### Combinatorial functions {#combinatorial-functions}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [factorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/factorial/index.html) | `Factorial` |  |  | ✓ |  | ✓ |
| [factorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/factorial/index.html) | `DoubleFactorial` |  |  | ✓ |  | ✓ |
| [factorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/factorial/index.html) | `Multifactorial` |  |  | ✓ |  | ✓ |
| [factorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/factorial/index.html) | `Subfactorial` |  |  |  |  | ✓ |
| [falling_factorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/falling_factorial/index.html) | `FallingFactorial` |  |  |  |  | ✓ |
| [rising_factorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/rising_factorial/index.html) | `RisingFactorial` |  | ✓ |  |  |  |
| [binomial_coefficient](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/binomial_coefficient/index.html) | `BinomialCoefficient` |  |  | ✓ |  | ✓ |
| [fibonacci](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/fibonacci/index.html) | `Fibonacci` (`fibonacci`, `fibonacci_pair`) |  |  | ✓ |  | ✓ |
| [fibonacci](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/fibonacci/index.html) | `LucasNumber` (`lucas_number`, `lucas_number_pair`) |  |  | ✓ |  | ✓ |
| [primorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/primorial/index.html) | `Primorial::primorial` |  |  | ✓ |  | ✓ |
| [primorial](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/primorial/index.html) | `Primorial::product_of_first_n_primes` |  |  |  |  | ✓ |
| [bell_number](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/bell_number/index.html) | `BellNumber` |  | ✓ |  |  |  |
| [bell_number](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/bell_number/index.html) | `bell_numbers_prefix` |  | ✓ |  |  |  |
| [landau_function](https://docs.rs/malachite-nz/latest/malachite_nz/natural/arithmetic/landau_function/index.html) | `landau_function_prefix` |  | ✓ |  |  |  |

Almost every combinatorial function is compared with a naive reference (the defining product, sum,
or recurrence), and most with GMP. `rising_factorial`, `bell_number` (with its prefix table), and
`landau_function_prefix` are checked against FLINT. None has an Azurite counterpart yet, so this
whole section is open ground for it.

## Conversion {#conversion}

From
[`malachite_nz::natural::conversion`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/index.html).

### Primitive types {#primitive-types}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [from_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) | `From<u8>`, …, `From<usize>` | ✓ |  | ✓ | ✓ |  |
| [from_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) | `SaturatingFrom<i8>`, …, `SaturatingFrom<isize>` | ✓ |  |  |  |  |
| [from_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) | `TryFrom<i8>`, …, `TryFrom<isize>`, `ConvertibleFrom` |  |  |  |  |  |
| [from_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_int/index.html) | `const_from` |  |  |  |  |  |
| [primitive_int_from_natural](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html) | `WrappingFrom<Natural>` for every primitive integer | ✓ |  | ✓ |  |  |
| [primitive_int_from_natural](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html) | `TryFrom<Natural>` for every primitive integer |  |  | ✓ |  |  |
| [primitive_int_from_natural](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_int_from_natural/index.html) | `SaturatingFrom`, `OverflowingFrom`, `ConvertibleFrom` for every primitive integer |  |  |  |  |  |
| [from_bool](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_bool/index.html) | `From<bool>` |  |  |  |  |  |
| [from_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_float/index.html) | `RoundingFrom<f32>`, `RoundingFrom<f64>` | ✓ |  |  |  |  |
| [from_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_primitive_float/index.html) | `TryFrom<f32>`, `TryFrom<f64>`, `ConvertibleFrom` | ✓ |  |  |  |  |
| [primitive_float_from_natural](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_float_from_natural/index.html) | `RoundingFrom<Natural>` for `f32`, `f64` | ✓ |  |  |  |  |
| [primitive_float_from_natural](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/primitive_float_from_natural/index.html) | `TryFrom<Natural>`, `ConvertibleFrom<Natural>` for `f32`, `f64` | ✓ |  |  |  |  |
| [mantissa_and_exponent](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/mantissa_and_exponent/index.html) | `IntegerMantissaAndExponent` |  |  |  |  |  |
| [mantissa_and_exponent](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/mantissa_and_exponent/index.html) | `SciMantissaAndExponent`, `sci_mantissa_and_exponent_round`, `from_sci_mantissa_and_exponent_round` | ✓ |  |  |  |  |
| [is_integer](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/is_integer/index.html) | `IsInteger` |  |  |  |  |  |
| [is_real](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/is_real/index.html) | `IsReal` |  |  |  |  |  |
| [is_gaussian_integer](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/is_gaussian_integer/index.html) | `IsGaussianInteger` |  |  |  |  |  |
| [clone](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/clone/index.html) | `Clone` |  |  | ✓ | ✓ |  |

Conversion from the unsigned primitives is checked by Azurite (`UInt64.toAzNat`), GMP, and num, and
the clamping conversion from the signed ones by Azurite. Toward the primitives, `wrapping_from`
(the value modulo $$2^w$$, reinterpreted for signed types) is checked by Azurite's `toUInt64` and
`toInt64` families and by GMP, and `try_from` by GMP.

The conversions to and from `f32` and `f64`, and the scientific mantissa-and-exponent
decompositions, are checked by Azurite. A `Natural` becomes an exact `AzFloat` and is rounded to
`f64` by `AzFloat.toFloat64`, whose IEEE 754 rounding is proven, or to the 24 bits of an `f32` by
`setPrecRound`; a rounded value above the largest finite float overflows as IEEE 754 specifies. In
the other direction a float's exact rational value is rounded by `AzRat.round`. Malachite prints
floats as the shortest decimals that read back to them, and the oracle reads each decimal back as
an exact rational and rounds it to the float format, subnormals included, before comparing values.

### Limbs and digits {#limbs-and-digits}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [from_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_limbs/index.html) | `from_limbs_asc`, `from_owned_limbs_asc` | ✓ |  |  |  |  |
| [from_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/from_limbs/index.html) | `from_limbs_desc`, `from_owned_limbs_desc` | ✓ |  |  |  |  |
| [to_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/to_limbs/index.html) | `limbs` | ✓ |  |  |  |  |
| [to_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/to_limbs/index.html) | `to_limbs_asc`, `into_limbs_asc`, `as_limbs_asc`, `to_limbs_desc`, `into_limbs_desc` | ✓ |  |  |  |  |
| [limb_count](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/limb_count/index.html) | `limb_count` | ✓ |  |  |  |  |
| [general_digits](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/digits/general_digits/index.html) | `Digits::to_digits_asc`, `Digits::to_digits_desc` | ✓ |  |  |  | ✓ |
| [general_digits](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/digits/general_digits/index.html) | `Digits::from_digits_asc`, `Digits::from_digits_desc` | ✓ |  |  |  | ✓ |
| [power_of_2_digits](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/digits/power_of_2_digits/index.html) | `PowerOf2Digits` | ✓ |  |  |  | ✓ |
| [power_of_2_digit_iterable](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/digits/power_of_2_digit_iterable/index.html) | `PowerOf2DigitIterable` |  |  |  |  |  |

The limb-level interface is checked by Azurite's `ofLimbs` and `limbs` in both orders and every form
(`to_limbs`, `into_limbs`, `as_limbs_asc`, the reversed iterator, the `from_owned_limbs`
constructors) and `limb_count` by the number of `AzNat` limbs. Base-$$b$$ digits, in both directions
and both orders, are checked by Azurite (`limbDigits` and `ofLimbDigits` for primitive digits, and
division and Horner's rule in `AzNat` arithmetic for `Natural` digits) and by references that
convert one digit at a time; the power-of-2 digits by Azurite's `limbDigitsPow2` and
`ofLimbDigitsPow2` and by references that extract and assemble bit fields.

### Strings {#strings}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [to_string](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/to_string/index.html) | `Display`, `Debug` | ✓ |  | ✓ | ✓ | ✓ |
| [to_string](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/to_string/index.html) | `Binary`, `Octal`, `LowerHex`, `UpperHex` | ✓ |  | ✓ | ✓ | ✓ |
| [to_string](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/to_string/index.html) | `ToStringBase` | ✓ |  | ✓ |  | ✓ |
| [from_string](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/from_string/index.html) | `FromStr` | ✓ |  | ✓ | ✓ |  |
| [from_string](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/from_string/index.html) | `FromStringBase` | ✓ |  | ✓ | ✓ |  |
| [from_sci_string](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/from_sci_string/index.html) | `FromSciString` | ✓ |  |  |  |  |
| [to_sci](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/to_sci/index.html) | `ToSci` | ✓ |  |  |  |  |
| [format_natural](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/format_natural/index.html) | `format_natural_str`, `format_gmp_str`, `GmpFormatArg` |  |  | ✓ |  |  |
| [latex](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/latex/index.html) | `ToLatex` |  |  |  |  |  |
| [typst](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/string/typst/index.html) | `ToTypst` |  |  |  |  |  |
| [serde](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/serde/index.html) | `Serialize`, `Deserialize` |  |  |  |  |  |
| [pyo3](https://docs.rs/malachite-nz/latest/malachite_nz/natural/conversion/pyo3/index.html) | `FromPyObject`, `IntoPyObject` |  |  |  |  |  |

Decimal and hexadecimal output and input are checked against GMP and num, and `to_string_base`
(lower and upper case), `from_str`, and `from_string_base` by Azurite, whose digits for bases above
36 follow Malachite's documented rule. The formatting traits are checked by Azurite's
`toStringBaseWith` through the demos that format with a width (`format!("{:#032x}", n)` and so on),
where the oracle applies Rust's zero padding after the base prefix; the unpadded demos print only
the output, which has nothing to be checked against. Scientific notation is checked by Azurite's
`AzRat.fromSci` and `AzRat.toSci` on integers: `from_sci_string` as the exact rational rounded to an
integer in the requested mode (`None` under `Exact` when that is not possible, and for a negative
result), `to_sci` and `to_sci_with_options` digit for digit, and `fmt_sci_valid` as `toSciExact`.
The GMP-style `format_natural_str` is compared with GMP's own `gmp_snprintf`, called directly.
LaTeX, Typst, serde, and the Python bindings are not cross-checked.

## Logic {#logic}

From
[`malachite_nz::natural::logic`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [and](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/and/index.html) | `BitAnd` |  |  | ✓ | ✓ | ✓ |
| [or](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/or/index.html) | `BitOr` |  |  | ✓ | ✓ | ✓ |
| [xor](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/xor/index.html) | `BitXor` |  |  | ✓ | ✓ | ✓ |
| [not](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/not/index.html) | `Not` |  |  | ✓ |  |  |
| [bit_access](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_access/index.html) | `BitAccess::get_bit` | ✓ |  | ✓ | ✓ |  |
| [bit_access](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_access/index.html) | `BitAccess::set_bit` | ✓ |  | ✓ | ✓ |  |
| [bit_access](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_access/index.html) | `BitAccess::clear_bit` | ✓ |  | ✓ |  |  |
| [bit_access](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_access/index.html) | `BitAccess::assign_bit`, `flip_bit` |  |  | ✓ |  |  |
| [bit_block_access](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_block_access/index.html) | `BitBlockAccess::get_bits` | ✓ |  |  |  | ✓ |
| [bit_block_access](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_block_access/index.html) | `BitBlockAccess::assign_bits` |  |  |  |  | ✓ |
| [bit_convertible](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_convertible/index.html) | `BitConvertible` |  |  |  |  | ✓ |
| [bit_iterable](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_iterable/index.html) | `BitIterable` |  |  |  |  |  |
| [bit_scan](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/bit_scan/index.html) | `BitScan` |  |  | ✓ |  | ✓ |
| [count_ones](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/count_ones/index.html) | `CountOnes` |  |  |  |  | ✓ |
| [hamming_distance](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/hamming_distance/index.html) | `HammingDistance` |  |  | ✓ |  | ✓ |
| [significant_bits](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/significant_bits/index.html) | `SignificantBits` | ✓ |  | ✓ | ✓ |  |
| [trailing_zeros](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/trailing_zeros/index.html) | `trailing_zeros` | ✓ |  |  |  | ✓ |
| [low_mask](https://docs.rs/malachite-nz/latest/malachite_nz/natural/logic/low_mask/index.html) | `LowMask` | ✓ |  |  |  |  |

The bitwise operations and the single-bit accessors are checked against GMP, and most against num
and references that work limb by limb or bit by bit. Azurite checks `get_bit`, `set_bit`,
`clear_bit`, and `get_bits` (`testBit`, `setBit`, `clearBit`, `getBits`), and `significant_bits`,
`trailing_zeros`, and `low_mask` (`size`, `trailingZeros`, `lowMask`). `AzNat` has no `and`, `or`,
`xor`, or `not` yet, so those rows, and the bit scans and population counts built on them, are
where Azurite would add the most here.

## Factorization {#factorization}

From [`malachite_nz::natural::factorization`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/factorization/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [is_square](https://docs.rs/malachite-nz/latest/malachite_nz/natural/factorization/is_square/index.html) | `IsSquare` | ✓ |  | ✓ |  |  |
| [is_power](https://docs.rs/malachite-nz/latest/malachite_nz/natural/factorization/is_power/index.html) | `IsPower` |  |  | ✓ |  |  |
| [is_power](https://docs.rs/malachite-nz/latest/malachite_nz/natural/factorization/is_power/index.html) | `ExpressAsPower` |  |  |  |  |  |
| [remove_power](https://docs.rs/malachite-nz/latest/malachite_nz/natural/factorization/remove_power/index.html) | `RemovePower` |  |  | ✓ |  |  |
| [primes](https://docs.rs/malachite-nz/latest/malachite_nz/natural/factorization/primes/index.html) | `primes_less_than`, `primes_less_than_or_equal_to` | ✓ |  |  |  |  |
| [primes](https://docs.rs/malachite-nz/latest/malachite_nz/natural/factorization/primes/index.html) | `primes` |  |  |  |  |  |

`is_square` is checked by Azurite's `isSquare` and GMP's `is_perfect_square`, and `is_power` and
`remove_power` by GMP. `primes_less_than` and `primes_less_than_or_equal_to` are checked by Azurite
in a strong form: the oracle decides every number in the range with `isPrime`, whose result is
backed by a Miller–Rabin filter and a machine-checked APR-CL certificate, and requires the printed
list to be exactly the primes it finds. The unbounded `primes` iterator is tested by comparing its
prefixes with `primes_less_than`, so it inherits that check indirectly. `express_as_power`, which
returns a base and exponent rather than a yes or no, is covered by property tests alone.
`Natural` has no primality test of its own yet; Azurite's proven `isPrime` will be its first
oracle when it does.

## Exhaustive generation {#exhaustive-generation}

From [`malachite_nz::natural::exhaustive`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_naturals.html) | `exhaustive_naturals` | ✓ |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_positive_naturals.html) | `exhaustive_positive_naturals` | ✓ |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_range.html) | `exhaustive_natural_range` | ✓ |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_inclusive_range.html) | `exhaustive_natural_inclusive_range` | ✓ |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/natural/exhaustive/fn.exhaustive_natural_range_to_infinity.html) | `exhaustive_natural_range_to_infinity` | ✓ |  |  |  |  |

The exhaustive generators are checked against Azurite's `ExhaustiveGenerator` instances
(`naturalsGen`, `positiveNaturalsGen`, `azNatRangeGen`, `azNatRangeInclusiveGen`,
`azNatRangeToInfinityGen`), each of which carries a proof that every value occurs exactly once: the
demos print the whole-type sequences element by element and the first twenty values of each range,
and the oracle requires Azurite's generator to produce the same values in the same positions, and a
range with fewer than twenty values to end where Malachite's does. Malachite's own tests compare
prefixes with fixed expected values and check that ranges contain exactly what they should.

## Random generation {#random-generation}

From [`malachite_nz::natural::random`](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html) | `random_naturals`, `random_positive_naturals`, `random_naturals_less_than`, `random_naturals_less_than_power_of_2` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html) | `striped_random_naturals`, `striped_random_positive_naturals`, `striped_random_naturals_less_than_power_of_2` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html) | `uniform_random_natural_range`, `uniform_random_natural_inclusive_range`, `random_natural_range`, `random_natural_inclusive_range`, `random_natural_range_to_infinity` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html) | `striped_random_natural_range`, `striped_random_natural_inclusive_range`, `striped_random_natural_range_to_infinity` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/natural/random/index.html) | `get_random_natural_with_bits`, `get_random_natural_with_up_to_bits`, `get_random_natural_less_than`, and their striped forms |  |  |  |  |  |

Random generators have no independent oracle in the sense of this page: what they produce depends
on Malachite's own seeded generator, so another library cannot reproduce it. They are tested
instead by pinning the first values for fixed seeds and by comparing the distribution of a large
sample (its most common values, mean, standard deviation, skewness, and excess kurtosis) with the
values the distribution should have, which catches a biased or truncated generator.

## What is not yet cross-checked {#not-yet-cross-checked}

The rows above with no mark, and the rows with no Azurite mark, sort into two groups by what it
would take to give them a verified oracle. Within each group the most widely used functions come
first.

**A few lines of oracle code around Azurite functions.** These need no new Azurite function, but
the oracle would compose existing ones, so the composition itself is unverified code: `checked_sub`
and `saturating_sub_mul` (a comparison and a subtraction), `abs_diff`, `average` and
`average_round`, the fused operations (`add_mul`, `sub_mul`, `mul_add_mul`, `mul_sub_mul`, and
their checked and saturating forms), `shl_round`, `mul_shr_round`, `round_to_multiple` and
`round_to_multiple_of_power_of_2` (a rounded division or shift and a multiplication),
`ceiling_root` and `root_rem` (`rootInt` and a power), `next_power_of_2`, `lcm` (a GCD and a
division), `div_exact` and `checked_div`, `ceiling_div_neg_mod` and `neg_mod`, `assign_bit` and
`flip_bit` (from `testBit`, `setBit`, and `clearBit`), the primitive conversions that clamp, check,
or flag (`TryFrom`, `SaturatingFrom`, `OverflowingFrom`, `ConvertibleFrom`), `IntegerMantissaAndExponent`
(a count of trailing zeros and a shift), and `Clone`.

**Azurite would need new functions.** These are the operations whose absence from Azurite is the
real gap, in rough order of how much of Malachite they would cover:

1. The bitwise operations `and`, `or`, `xor`, and `not`, and with them the population and Hamming
   counts, the bit scans, bit iteration, conversion to and from bit vectors, and writing a block of
   bits (`assign_bits`).
2. The combinatorial functions: factorials (single, double, multi-, and sub-), the falling and
   rising factorials, binomial coefficients, Fibonacci and Lucas numbers, the primorial and the
   product of the first $$n$$ primes, Bell numbers, and the Landau function. Every one of them is
   currently checked against a reference implementation and most against GMP, but none against a
   proven implementation.
3. The Kronecker symbol (Azurite has the Jacobi symbol) and with it the Legendre symbol, the
   modular square root, `mod_div_list`, and `MultiCrt::apply_balanced`.
4. Perfect-power detection and decomposition (`is_power`, `express_as_power`) and `remove_power`.
5. A primality test on `Natural`, the one place where Azurite is ahead of Malachite: its `isPrime`
   is already proven, and is the oracle for `primes_less_than`.

**Outside the scope of an oracle.** The constants, `Default`, `Named`, `Hash`, the unit functions
(`is_unit`, `conjugate`, `canonicalize_unit`, `canonical_unit_i_pow`), `IsInteger`, `IsReal`,
`IsGaussianInteger`, `From<bool>`, `const_from`, LaTeX and Typst output, serde, the Python bindings,
and the random generators. Most are trivial on a type with no sign and no fractional part; the
rest have no independent implementation to compare with, and are tested by their own unit and
property tests, the random generators by the distribution of large samples.
