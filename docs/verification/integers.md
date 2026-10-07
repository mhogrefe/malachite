---
layout: default
title: "How Malachite Is Tested: Integers"
permalink: /verification/integers/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Integers

This page lists every public function of
[`Integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html), the
signed integer type of the `malachite-nz` crate, and records which independent implementations it
is checked against. [The introduction](/verification/) describes the oracles and the testing they
sit in; this page is the ledger. It follows the organization of the crate's documentation, one
section per module, so that a function is where its documentation is. An `Integer` is a sign and a
[`Natural`](/verification/naturals/) absolute value, and many of its functions reduce to the
`Natural` ones, but each is listed and checked here in its own right.

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
[`malachite_nz::integer`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [constants](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html) | `Zero`, `One`, `Two`, `NegativeOne` | | | | | |
| [default](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-Default-for-Integer) | `Default` | | | | | |
| [named](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-Named-for-Integer) | `Named` | | | | | |
| [hash](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-Hash-for-Integer) | `Hash` | | | | | |
| [clone](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-Clone-for-Integer) | `Clone` | | | ✓ | ✓ | |

The constants are the values `0`, `1`, `2`, and `-1`; every oracle run uses them on the way to
checking something else, but no run checks them as such. `Named` gives the type's name as a string
and `Hash` is derived from the sign and absolute value, so neither has anything to compare against.
`Clone`, also derived, is checked against GMP's and num's copies.
Unlike `Natural`, `Integer` has no `Min` or `Max`.

## Comparison {#comparison}

From
[`malachite_nz::integer::comparison`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [cmp](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/cmp/index.html) | `Ord`, `PartialOrd` | ✓ |  | ✓ | ✓ |  |
| [eq](https://docs.rs/malachite-nz/latest/malachite_nz/integer/struct.Integer.html#impl-PartialEq-for-Integer) | `PartialEq`, `Eq` | ✓ |  | ✓ | ✓ |  |
| [cmp_abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/cmp_abs/index.html) | `OrdAbs`, `PartialOrdAbs` |  |  | ✓ |  |  |
| [cmp_abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/cmp_abs/index.html) | `OrdAbsDouble`, `PartialOrdAbsDouble` |  |  |  |  |  |
| [eq_abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/eq_abs/index.html) | `EqAbs` |  |  |  |  |  |
| [partial_cmp_natural](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_natural/index.html) | `PartialOrd<Natural>` and the reverse direction |  |  | ✓ |  |  |
| [partial_eq_natural](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_eq_natural/index.html) | `PartialEq<Natural>` and the reverse direction |  |  | ✓ |  |  |
| [cmp_abs_natural](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/cmp_abs_natural/index.html) | `PartialOrdAbs<Natural>` and the reverse direction |  |  | ✓ |  |  |
| [eq_abs_natural](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/eq_abs_natural/index.html) | `EqAbs<Natural>` and the reverse direction |  |  |  |  |  |
| [partial_cmp_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_primitive_int/index.html) | `PartialOrd<u8>`, …, `PartialOrd<isize>`, and the reverse directions |  |  | ✓ | ✓ |  |
| [partial_eq_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_eq_primitive_int/index.html) | `PartialEq<u8>`, …, `PartialEq<isize>`, and the reverse directions |  |  | ✓ | ✓ |  |
| [cmp_abs_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/cmp_abs_primitive_int/index.html) | `PartialOrdAbs<u8>`, …, `PartialOrdAbs<isize>`, and the reverse directions |  |  |  |  |  |
| [eq_abs_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/eq_abs_primitive_int/index.html) | `EqAbs<u8>`, …, `EqAbs<isize>`, and the reverse directions |  |  |  |  |  |
| [partial_cmp_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_cmp_primitive_float/index.html) | `PartialOrd<f32>`, `PartialOrd<f64>`, and the reverse directions |  |  | ✓ |  |  |
| [partial_eq_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/partial_eq_primitive_float/index.html) | `PartialEq<f32>`, `PartialEq<f64>`, and the reverse directions |  |  | ✓ |  |  |
| [cmp_abs_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/cmp_abs_primitive_float/index.html) | `PartialOrdAbs<f32>`, `PartialOrdAbs<f64>`, and the reverse directions |  |  |  |  |  |
| [eq_abs_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/integer/comparison/eq_abs_primitive_float/index.html) | `EqAbs<f32>`, `EqAbs<f64>`, and the reverse directions |  |  |  |  |  |

The order and equality of two `Integer`s are checked by Azurite, GMP, and num. Comparison with a
`Natural` and with the primitive integers is checked by GMP, and with the unsigned primitives also
by num. Azurite checks these mixed comparisons with the `Integer` on the left (`x < n`, `x == 5`),
but not the reverse direction, whose demos exist but are not yet run, so those rows carry no
Azurite mark. Comparison of absolute values is checked by GMP's `cmp_abs` for two `Integer`s and
for an `Integer` and a `Natural`; the absolute-value comparisons with primitives, the
absolute-value equalities, and the doubled comparison `cmp_abs_double`
($$\operatorname{cmp}(|x|, 2|y|)$$) have no oracle. Comparison with `f32` and `f64` is checked
by GMP.

## Arithmetic {#arithmetic}

From
[`malachite_nz::integer::arithmetic`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/index.html).
The module is large, so its rows are grouped by theme. `Integer` has no modular arithmetic family
and no logarithms; those are `Natural` functions.

### Addition, subtraction, and multiplication {#addition-subtraction-and-multiplication}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [add](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/add/index.html) | `Add` | ✓ |  | ✓ | ✓ |  |
| [add](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/add/index.html) | `Sum` |  |  |  |  | ✓ |
| [sub](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/sub/index.html) | `Sub` | ✓ |  | ✓ | ✓ |  |
| [abs_diff](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/abs_diff/index.html) | `AbsDiff` |  |  |  |  |  |
| [neg](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/neg/index.html) | `Neg` | ✓ |  | ✓ | ✓ |  |
| [mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mul/index.html) | `Mul` | ✓ |  | ✓ | ✓ |  |
| [mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mul/index.html) | `Product` |  |  |  |  | ✓ |
| [square](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/square/index.html) | `Square` |  |  |  |  |  |
| [abs_squared](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/abs_squared/index.html) | `AbsSquared` |  |  |  |  |  |
| [add_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/add_mul/index.html) | `AddMul` |  |  |  |  |  |
| [sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/sub_mul/index.html) | `SubMul` |  |  |  |  |  |
| [mul_add_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mul_add_mul/index.html) | `MulAddMul` |  |  |  |  |  |
| [mul_sub_mul](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mul_sub_mul/index.html) | `MulSubMul` |  |  |  |  |  |
| [average](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/average/index.html) | `Average` |  |  |  |  |  |
| [average](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/average/index.html) | `AverageRound` |  |  |  |  |  |

Addition, subtraction, multiplication, and negation are checked by Azurite (`AzInt.add`, `sub`,
`mul`, `neg`) and by GMP and num. `Sum` and `Product` are checked against a reference
implementation that folds `+` and `*` one term at a time. `AzInt` has no squaring function,
so `square` is not yet cross-checked, though the `Natural` one is checked by Azurite. The fused operations (`add_mul`,
`sub_mul`, `mul_add_mul`, `mul_sub_mul`), `abs_diff`, `abs_squared`, and `average` are not yet
cross-checked: their property tests compare them with the unfused combinations of operations that
are, which is a strong check but not an independent one.

### Shifts {#shifts}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [shl](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/shl/index.html) | `Shl<u8>`, …, `Shl<isize>` | ✓ |  | ✓ | ✓ |  |
| [shr](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/shr/index.html) | `Shr<u8>`, …, `Shr<isize>` | ✓ |  | ✓ | ✓ |  |
| [shl_round](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/shl_round/index.html) | `ShlRound<i8>`, …, `ShlRound<isize>` |  |  |  |  |  |
| [shr_round](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/shr_round/index.html) | `ShrRound<u8>`, …, `ShrRound<isize>` | ✓ |  |  |  |  |
| [mul_shr_round](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mul_shr_round/index.html) | `MulShrRound` |  |  |  |  |  |
| [round_to_multiple_of_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/round_to_multiple_of_power_of_2/index.html) | `RoundToMultipleOfPowerOf2` |  |  |  |  |  |

Shifting by a primitive integer is checked by Azurite's `shiftLeft` and `shiftRight` for every shift
type, a negative count reversing the direction, and against GMP; the shifts by unsigned counts also
against num. A right shift of a negative `Integer` rounds toward $$-\infty$$, as an arithmetic
shift does, and Azurite's `shiftRight` has the same convention. `shr_round` is checked by Azurite's
`shiftRightRound` in every rounding mode, `Exact` being checked as "the shift is exact and the
`Ordering` is `Equal`".

### Division {#division}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [div](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div/index.html) | `Div` |  |  | ✓ | ✓ |  |
| [div](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div/index.html) | `CheckedDiv` |  |  |  | ✓ |  |
| [div_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_mod/index.html) | `DivMod` | ✓ |  | ✓ | ✓ |  |
| [div_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_mod/index.html) | `DivRem` |  |  | ✓ | ✓ |  |
| [div_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_mod/index.html) | `CeilingDivMod` |  |  | ✓ |  |  |
| [div_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_mod/index.html) | `DivModPrecomputed` |  |  |  |  |  |
| [mod_op](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_op/index.html) | `Mod` | ✓ |  | ✓ | ✓ |  |
| [mod_op](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_op/index.html) | `Rem` |  |  | ✓ | ✓ |  |
| [mod_op](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_op/index.html) | `CeilingMod` |  |  | ✓ |  |  |
| [div_euclidean](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_euclidean/index.html) | `DivEuclidean` | ✓ |  |  |  |  |
| [mod_euclidean](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_euclidean/index.html) | `ModEuclidean` | ✓ |  |  |  |  |
| [div_mod_euclidean](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_mod_euclidean/index.html) | `DivModEuclidean` | ✓ |  |  |  |  |
| [div_exact](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_exact/index.html) | `DivExact` | ✓ |  | ✓ |  |  |
| [div_round](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/div_round/index.html) | `DivRound` | ✓ |  | ✓ | ✓ |  |
| [divisible_by](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/divisible_by/index.html) | `DivisibleBy` |  |  | ✓ | ✓ |  |
| [eq_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/eq_mod/index.html) | `EqMod` |  |  | ✓ |  |  |
| [round_to_multiple](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/round_to_multiple/index.html) | `RoundToMultiple` |  |  |  |  |  |
| [balanced_mod](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/balanced_mod/index.html) | `BalancedMod` |  |  |  |  |  |

On `Integer`s the division conventions differ, so each gets its own row: `/` and `div_rem`
truncate the quotient toward zero, `div_mod` and `mod_op` round it toward $$-\infty$$ (the
remainder taking the divisor's sign), `ceiling_div_mod` and `ceiling_mod` round it toward
$$+\infty$$, and the Euclidean forms keep the remainder non-negative. GMP has all four families and
checks every one; num checks the truncating and floor forms. Azurite's `/` is Euclidean, and it also has
floor division (`fdivMod`), against which the floor forms (`div_mod`, `mod_op`) are checked; the
truncating and ceiling forms are not yet run against it. `div_exact`,
`div_round` (every rounding mode, `Exact` checked as an exact division), and the Euclidean forms
are checked by Azurite, and `div_exact`, `div_round`, `divisible_by`, and `eq_mod` by GMP.
`DivModPrecomputed`, which reuses a precomputed inverse of the divisor, is compared with `div_mod`;
`round_to_multiple` and `balanced_mod`, the remainder in $$(-m/2, m/2]$$, are covered by property
tests alone.

### Signs, units, and parity {#signs-units-and-parity}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [sign](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/sign/index.html) | `Sign` | ✓ |  | ✓ | ✓ |  |
| [abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/abs/index.html) | `Abs` |  |  | ✓ | ✓ |  |
| [abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/abs/index.html) | `UnsignedAbs`, `unsigned_abs_ref` | ✓ |  |  |  |  |
| [abs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/abs/index.html) | `mutate_unsigned_abs` |  |  |  |  |  |
| [parity](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/parity/index.html) | `Parity` | ✓ |  |  |  |  |
| [is_unit](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/is_unit/index.html) | `IsUnit` |  |  |  |  |  |
| [conjugate](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/conjugate/index.html) | `Conjugate` |  |  |  |  |  |
| [canonicalize_unit](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/canonicalize_unit/index.html) | `CanonicalizeUnit` |  |  |  |  |  |
| [canonical_unit_i_pow](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/canonical_unit_i_pow/index.html) | `CanonicalUnitIPow` |  |  |  |  |  |

`sign` is checked by Azurite, GMP, and num, and `abs` by GMP and num. `unsigned_abs`, which returns
the absolute value as a `Natural`, is checked by Azurite's `natAbs`; `even` and `odd` by Azurite's
`isEven` and `isOdd`. `mutate_unsigned_abs`, which applies a function to the absolute value in
place, has no counterpart to compare with. The unit functions (`is_unit`, `conjugate`,
`canonicalize_unit`, `canonical_unit_i_pow`) are the trivial cases of functions that matter for
Gaussian integers and polynomials, where they are cross-checked.

### Arithmetic modulo a power of 2 {#arithmetic-modulo-a-power-of-2}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_power_of_2/index.html) | `ModPowerOf2` | ✓ |  |  |  |  |
| [mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_power_of_2/index.html) | `RemPowerOf2` |  |  |  |  |  |
| [mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/mod_power_of_2/index.html) | `CeilingModPowerOf2` |  |  |  |  |  |
| [eq_mod_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/eq_mod_power_of_2/index.html) | `EqModPowerOf2` |  |  | ✓ |  |  |

`mod_power_of_2`, the residue of an `Integer` in $$[0, 2^k)$$, is checked by Azurite's
`AzZModPow2.ofAzInt`. The truncating and ceiling forms (`rem_power_of_2`, whose result takes the
sign of the input, and `ceiling_mod_power_of_2`, the non-positive residue) are not yet run against
it, and `eq_mod_power_of_2` is checked against GMP's `is_congruent_2pow`.

### Chinese remaindering {#chinese-remaindering}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [crt](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/crt/index.html) | `BalancedCrt` |  | ✓ |  |  | ✓ |
| [multi_crt](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/multi_crt/index.html) | `Integer::multi_balanced_crt` |  | ✓ |  |  |  |

The balanced Chinese remainder functions return the solution in $$(-M/2, M/2]$$ rather than
$$[0, M)$$. `balanced_crt` is checked against FLINT's `fmpz_CRT` with its sign flag set and a
reference that takes the solution in $$[0, M)$$ and shifts it into the balanced range, and
`multi_balanced_crt` against FLINT's `fmpz_multi_CRT` with the same flag.

### Powers and roots {#powers-and-roots}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [pow](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/pow/index.html) | `Pow<u64>` | ✓ |  | ✓ | ✓ |  |
| [sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/sqrt/index.html) | `FloorSqrt` |  |  | ✓ | ✓ |  |
| [sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/sqrt/index.html) | `CeilingSqrt` |  |  |  |  |  |
| [sqrt](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/sqrt/index.html) | `CheckedSqrt` |  |  |  |  |  |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/root/index.html) | `FloorRoot<u64>` |  |  | ✓ | ✓ |  |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/root/index.html) | `CeilingRoot<u64>` |  |  | ✓ | ✓ |  |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/root/index.html) | `CheckedRoot<u64>` |  |  |  |  |  |
| [root](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/root/index.html) | `RootRem<u64>` |  |  | ✓ |  |  |

`pow` is checked by Azurite, GMP, and num. Square roots are defined only for non-negative
`Integer`s, and `floor_sqrt` is checked against GMP and num; odd roots of negative numbers are
defined, and GMP's and num's roots round toward zero, so they check `floor_root` on non-negative
inputs and `ceiling_root` on negative ones, where those functions round toward zero too.
`root_rem` is checked against GMP. `ceiling_sqrt`, `checked_sqrt`, and `checked_root` are not yet
cross-checked for `Integer`, though their `Natural` counterparts are checked by Azurite.

### Powers of 2 {#powers-of-2}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/power_of_2/index.html) | `PowerOf2<u64>` | ✓ |  |  |  |  |
| [is_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/is_power_of_2/index.html) | `IsPowerOf2` | ✓ |  | ✓ |  |  |
| [divisible_by_power_of_2](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/divisible_by_power_of_2/index.html) | `DivisibleByPowerOf2` |  |  | ✓ |  |  |

`power_of_2` and `is_power_of_2` are checked by Azurite, and `is_power_of_2` and
`divisible_by_power_of_2` by GMP.

### GCD and number theory {#gcd-and-number-theory}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [gcd](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/gcd/index.html) | `Gcd` | ✓ |  |  |  |  |
| [extended_gcd](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/extended_gcd/index.html) | `ExtendedGcd` |  |  | ✓ | ✓ |  |
| [kronecker_symbol](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/kronecker_symbol/index.html) | `JacobiSymbol` |  |  | ✓ |  |  |
| [kronecker_symbol](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/kronecker_symbol/index.html) | `LegendreSymbol` |  |  | ✓ |  |  |
| [kronecker_symbol](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/kronecker_symbol/index.html) | `KroneckerSymbol` |  |  | ✓ |  |  |

`gcd`, which returns a `Natural`, is checked by Azurite's `NormalizedGcd` instance on `AzInt`. `extended_gcd` is checked
against GMP and num, which compare the Bézout pair exactly; Azurite's extended GCD is run on the
`Natural` version only (as ≈, for the normalization described on [the naturals
page](/verification/naturals/#gcd-and-number-theory)). The Jacobi, Legendre, and Kronecker symbols
are checked against GMP.

### Combinatorial functions {#combinatorial-functions}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [binomial_coefficient](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/binomial_coefficient/index.html) | `BinomialCoefficient` |  |  | ✓ |  |  |
| [falling_factorial](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/falling_factorial/index.html) | `FallingFactorial` |  |  |  |  | ✓ |
| [rising_factorial](https://docs.rs/malachite-nz/latest/malachite_nz/integer/arithmetic/rising_factorial/index.html) | `RisingFactorial` |  | ✓ |  |  |  |

The binomial coefficient, which takes a negative first argument through the identity
$$\binom{-n}{k} = (-1)^k \binom{n+k-1}{k}$$, is checked against GMP; `falling_factorial` against the defining product;
and `rising_factorial` against FLINT's `fmpz_rfac`.

## Conversion {#conversion}

From
[`malachite_nz::integer::conversion`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/index.html).

### Naturals {#naturals}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [from_natural](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_natural/index.html) | `From<Natural>` | ✓ |  |  |  |  |
| [from_natural](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_natural/index.html) | `from_sign_and_abs`, `from_sign_and_abs_ref` | ✓ |  |  |  |  |
| [natural_from_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/natural_from_integer/index.html) | `TryFrom<Integer>` for `Natural` |  |  |  |  |  |
| [natural_from_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/natural_from_integer/index.html) | `SaturatingFrom<Integer>`, `ConvertibleFrom<Integer>` for `Natural` |  |  |  |  |  |

Conversion from a `Natural` is checked by Azurite's `AzNat.toAzInt`, and
`from_sign_and_abs` by `AzInt.mkNorm`, which forces the sign positive when the magnitude is zero,
as Malachite does. The conversions back to `Natural`, which reject or clamp a negative input, are
not yet cross-checked; the absolute value as a `Natural` (`unsigned_abs`) is listed under
[Signs, units, and parity](#signs-units-and-parity).

### Primitive types {#primitive-types}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [from_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html) | `From<u8>`, …, `From<isize>` | ✓ |  | ✓ | ✓ |  |
| [from_primitive_int](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_int/index.html) | `const_from_unsigned`, `const_from_signed` |  |  |  |  |  |
| [primitive_int_from_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) | `WrappingFrom<Integer>` for every primitive integer | ✓ |  | ✓ |  |  |
| [primitive_int_from_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) | `TryFrom<Integer>` for every primitive integer |  |  | ✓ |  |  |
| [primitive_int_from_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_int_from_integer/index.html) | `SaturatingFrom`, `OverflowingFrom`, `ConvertibleFrom` for every primitive integer |  |  |  |  |  |
| [from_bool](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_bool/index.html) | `From<bool>` |  |  |  |  |  |
| [from_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_float/index.html) | `RoundingFrom<f32>`, `RoundingFrom<f64>` |  |  |  |  |  |
| [from_primitive_float](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_primitive_float/index.html) | `TryFrom<f32>`, `TryFrom<f64>`, `ConvertibleFrom` |  |  |  |  |  |
| [primitive_float_from_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_float_from_integer/index.html) | `RoundingFrom<Integer>` for `f32`, `f64` |  |  |  |  |  |
| [primitive_float_from_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/primitive_float_from_integer/index.html) | `TryFrom<Integer>`, `ConvertibleFrom<Integer>` for `f32`, `f64` |  |  |  |  |  |
| [is_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/is_integer/index.html) | `IsInteger` |  |  |  |  |  |
| [is_real](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/is_real/index.html) | `IsReal` |  |  |  |  |  |
| [is_gaussian_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/is_gaussian_integer/index.html) | `IsGaussianInteger` |  |  |  |  |  |

Conversion from every primitive integer type is checked by Azurite (`UInt64.toAzInt`,
`Int64.toAzInt`, and their narrower forms), GMP, and num. Toward the primitives, `wrapping_from`
(the value modulo $$2^w$$, two's complement for a negative `Integer`, reinterpreted for signed
types) is checked by Azurite's `toUInt64` and `toInt64` families and by GMP, and `try_from` by GMP.
The float conversions are not yet cross-checked for `Integer`; the `Natural` ones are checked by
Azurite through `AzFloat`, and the same oracle would extend to a sign.

### Two's-complement limbs {#twos-complement-limbs}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [from_twos_complement_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_twos_complement_limbs/index.html) | `from_twos_complement_limbs_asc`, `from_owned_twos_complement_limbs_asc` |  |  |  |  |  |
| [from_twos_complement_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/from_twos_complement_limbs/index.html) | `from_twos_complement_limbs_desc`, `from_owned_twos_complement_limbs_desc` |  |  |  |  |  |
| [to_twos_complement_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/to_twos_complement_limbs/index.html) | `to_twos_complement_limbs_asc`, `into_twos_complement_limbs_asc`, `to_twos_complement_limbs_desc`, `into_twos_complement_limbs_desc` |  |  |  |  |  |
| [to_twos_complement_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/to_twos_complement_limbs/index.html) | `twos_complement_limbs`, `TwosComplementLimbIterator::get_limb` |  |  |  |  |  |
| [to_twos_complement_limbs](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/to_twos_complement_limbs/index.html) | `twos_complement_limb_count` |  |  |  |  |  |

An `Integer`'s limbs are its two's-complement representation, sign-extended as far as needed. No
oracle checks this interface yet: Azurite has no two's-complement view of an `AzInt`, and the
property tests check the conversions against each other, against the `Natural` limbs for
non-negative values, and by round trips.

### Strings {#strings}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [to_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/to_string/index.html) | `Display`, `Debug` | ✓ |  | ✓ | ✓ |  |
| [to_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/to_string/index.html) | `Binary`, `Octal`, `LowerHex`, `UpperHex` |  |  | ✓ | ✓ |  |
| [to_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/to_string/index.html) | `ToStringBase` |  |  |  |  |  |
| [from_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/from_string/index.html) | `FromStr` | ✓ |  | ✓ | ✓ |  |
| [from_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/from_string/index.html) | `FromStringBase` | ✓ |  | ✓ | ✓ |  |
| [from_sci_string](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/from_sci_string/index.html) | `FromSciString` |  |  |  |  |  |
| [to_sci](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/to_sci/index.html) | `ToSci` |  |  |  |  |  |
| [format_integer](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/format_integer/index.html) | `format_integer_str`, `GmpFormatArg` |  |  | ✓ |  |  |
| [latex](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/latex/index.html) | `ToLatex` |  |  |  |  |  |
| [typst](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/string/typst/index.html) | `ToTypst` |  |  |  |  |  |
| [serde](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/serde/index.html) | `Serialize`, `Deserialize` |  |  |  |  |  |
| [pyo3](https://docs.rs/malachite-nz/latest/malachite_nz/integer/conversion/pyo3/index.html) | `FromPyObject`, `IntoPyObject` |  |  |  |  |  |

Decimal output is checked by Azurite's `toString`, GMP, and num. Input in every base is checked by
GMP and num, and by Azurite through a sign around the `Natural` digit rule (an optional `-`, then
the `AzNat` parse in Malachite's rules, digits above 36 included), since `AzInt.parse` differs from
`from_str` on `"-0"` and on base prefixes; the binary, octal, and hexadecimal formatting traits by GMP and num, padding included.
`to_string_base`, scientific notation (`from_sci_string`, `to_sci`), and the formatting traits'
Azurite check through padded demos, all of which are checked by Azurite for `Natural`, are not yet
run for `Integer`. The GMP-style `format_integer_str` is compared with GMP's own `gmp_snprintf`,
called directly. LaTeX, Typst, serde, and the Python bindings are not cross-checked.

## Logic {#logic}

From
[`malachite_nz::integer::logic`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/index.html). The bitwise operations act on the
two's-complement representation, a negative `Integer` behaving as if it had infinitely many leading
1 bits.

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [and](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/and/index.html) | `BitAnd` |  |  | ✓ |  | ✓ |
| [or](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/or/index.html) | `BitOr` |  |  | ✓ |  | ✓ |
| [xor](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/xor/index.html) | `BitXor` |  |  | ✓ |  | ✓ |
| [not](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/not/index.html) | `Not` |  |  | ✓ |  |  |
| [bit_access](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_access/index.html) | `BitAccess::get_bit` |  |  | ✓ |  |  |
| [bit_access](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_access/index.html) | `BitAccess::set_bit`, `clear_bit` |  |  | ✓ |  |  |
| [bit_access](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_access/index.html) | `BitAccess::assign_bit`, `flip_bit` |  |  | ✓ |  |  |
| [bit_block_access](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_block_access/index.html) | `BitBlockAccess::get_bits` |  |  |  |  | ✓ |
| [bit_block_access](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_block_access/index.html) | `BitBlockAccess::assign_bits` |  |  |  |  | ✓ |
| [bit_convertible](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_convertible/index.html) | `BitConvertible` |  |  |  |  | ✓ |
| [bit_iterable](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_iterable/index.html) | `BitIterable` |  |  |  |  |  |
| [bit_scan](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/bit_scan/index.html) | `BitScan` |  |  | ✓ |  | ✓ |
| [checked_count_ones](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/checked_count_ones/index.html) | `checked_count_ones` |  |  |  |  | ✓ |
| [checked_count_zeros](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/checked_count_zeros/index.html) | `checked_count_zeros` |  |  |  |  | ✓ |
| [checked_hamming_distance](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/checked_hamming_distance/index.html) | `CheckedHammingDistance` |  |  | ✓ |  | ✓ |
| [significant_bits](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/significant_bits/index.html) | `SignificantBits` | ✓ |  | ✓ | ✓ |  |
| [trailing_zeros](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/trailing_zeros/index.html) | `trailing_zeros` | ✓ |  |  |  | ✓ |
| [low_mask](https://docs.rs/malachite-nz/latest/malachite_nz/integer/logic/low_mask/index.html) | `LowMask` | ✓ |  |  |  |  |

`and`, `or`, `xor`, `not`, the single-bit accessors, the bit scans, and the Hamming distance are
checked against GMP, whose bit functions use the same two's-complement convention, and most of them against references that
work limb by limb or bit by bit. The population counts, which return `None` where the count is
infinite (`checked_count_ones` of a negative number, `checked_count_zeros` of a non-negative one),
and the bit-block and bit-vector conversions are checked against references alone. Azurite checks
`significant_bits`, `trailing_zeros`, and `low_mask` (`AzInt.size`, `trailingZeros`, and
`lowMask`). `AzInt` has no two's-complement bit operations at all, so this
section is the largest gap between Azurite and Malachite's `Integer`.

## Factorization {#factorization}

From [`malachite_nz::integer::factorization`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/factorization/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [is_power](https://docs.rs/malachite-nz/latest/malachite_nz/integer/factorization/is_power/index.html) | `IsPower` |  |  | ✓ |  |  |
| [is_power](https://docs.rs/malachite-nz/latest/malachite_nz/integer/factorization/is_power/index.html) | `ExpressAsPower` |  |  |  |  |  |
| [remove_power](https://docs.rs/malachite-nz/latest/malachite_nz/integer/factorization/remove_power/index.html) | `RemovePower` |  |  | ✓ |  |  |

`is_power`, which for a negative `Integer` asks for an odd power, and `remove_power` are checked
against GMP. `express_as_power`, which returns a base and exponent rather than a yes or no, is
covered by property tests alone. `Integer` has no square test or prime functions; those belong to
`Natural`.

## Exhaustive generation {#exhaustive-generation}

From [`malachite_nz::integer::exhaustive`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html) | `exhaustive_integers` |  |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html) | `exhaustive_natural_integers`, `exhaustive_positive_integers`, `exhaustive_negative_integers` |  |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html) | `exhaustive_nonzero_integers` |  |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html) | `integer_increasing_range`, `integer_increasing_inclusive_range` |  |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html) | `integer_increasing_range_to_infinity`, `integer_decreasing_range_to_negative_infinity` |  |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html) | `exhaustive_integer_range`, `exhaustive_integer_inclusive_range` |  |  |  |  |  |
| [exhaustive](https://docs.rs/malachite-nz/latest/malachite_nz/integer/exhaustive/index.html) | `exhaustive_integer_range_to_infinity`, `exhaustive_integer_range_to_negative_infinity` |  |  |  |  |  |

No exhaustive generator of `Integer` is cross-checked yet, but most of them are close: Azurite's
`ExhaustiveGenerator` instances for `AzInt` (`integersGen`, the non-negative, positive, and
negative generators, the increasing and decreasing ranges, and `azIntRangeGen` and
`azIntRangeInclusiveGen`) produce the same sequences, each with a proof that every value occurs
exactly once, and only the demos and the oracle modes that compare them are missing.
`exhaustive_nonzero_integers` and the two half-bounded ranges in order of increasing absolute value
(`exhaustive_integer_range_to_infinity` and `exhaustive_integer_range_to_negative_infinity`) have
no Azurite counterpart. Malachite's own tests compare prefixes with fixed
expected values and check that ranges contain exactly what they should.

## Random generation {#random-generation}

From [`malachite_nz::integer::random`](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html) | `random_integers`, `random_natural_integers`, `random_positive_integers`, `random_negative_integers`, `random_nonzero_integers` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html) | `striped_random_integers`, `striped_random_natural_integers`, `striped_random_positive_integers`, `striped_random_negative_integers`, `striped_random_nonzero_integers` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html) | `uniform_random_integer_range`, `uniform_random_integer_inclusive_range`, `random_integer_range`, `random_integer_inclusive_range`, `random_integer_range_to_infinity`, `random_integer_range_to_negative_infinity` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html) | `striped_random_integer_range`, `striped_random_integer_inclusive_range`, `striped_random_integer_range_to_infinity`, `striped_random_integer_range_to_negative_infinity` |  |  |  |  |  |
| [random](https://docs.rs/malachite-nz/latest/malachite_nz/integer/random/index.html) | `get_uniform_random_integer_from_range`, `get_random_integer_from_range_to_infinity`, and the other `get_*` functions |  |  |  |  |  |

Random generators have no independent oracle in the sense of this page: what they produce depends
on Malachite's own seeded generator, so another library cannot reproduce it. They are tested
instead by pinning the first values for fixed seeds and by comparing the distribution of a large
sample (its most common values, mean, standard deviation, skewness, and excess kurtosis) with the
values the distribution should have, which catches a biased or truncated generator.

## What is not yet cross-checked {#not-yet-cross-checked}

The rows above with no mark, and the rows with no Azurite mark, sort into three groups by what it
would take to give them a verified oracle. Within each group the most widely used functions come
first.

**Azurite already computes these; the oracle only needs a mode for them.** The comparisons with a
`Natural` and with the primitive integers in the reverse direction (`5 < x`), from the same
`compareAzNat`, `compareUInt64`, and `compareInt64`; the truncating quotient `/` and
`checked_div`, from `AzInt.divRound` in the `Down` mode; the exhaustive generators listed above;
and the functions whose `Natural` versions are already checked, through oracle code that would
only add a sign: the conversions to and from `f32` and `f64` (through `AzFloat.ofAzInt`,
`AzFloat.toFloat64`, and `AzRat.round`), `to_string_base`, the formatting traits with a width,
`from_sci_string`, and `to_sci`.

**A few lines of oracle code around Azurite functions.** These need no new Azurite function, but
the oracle would compose existing ones, so the composition itself is unverified code: `abs` (the
magnitude as an `AzInt`), `square`, `abs_squared`, `abs_diff`, `average` and `average_round`,
the fused operations (`add_mul`, `sub_mul`, `mul_add_mul`, `mul_sub_mul`), `shl_round`,
`mul_shr_round`, `round_to_multiple` and `round_to_multiple_of_power_of_2`; the remainders and
ceiling forms of division (`div_rem`, `rem`, `ceiling_div_mod`, `ceiling_mod`), `divisible_by`,
`eq_mod`, `balanced_mod`, and `DivModPrecomputed`; `rem_power_of_2`, `ceiling_mod_power_of_2`,
`eq_mod_power_of_2`, and `divisible_by_power_of_2` (from `AzZModPow2.ofAzInt` and
`trailingZeros`); the roots (`AzNat.sqrtRem` and `rootInt` on the magnitude, the sign restored for
odd roots); `extended_gcd` (Azurite's `egcd` on the magnitudes, with the signs of the Bézout
coefficients adjusted, and ≈ for the reason given on [the naturals
page](/verification/naturals/#gcd-and-number-theory)); the Jacobi and Legendre symbols (`jacobi`
of the residue); `balanced_crt` and `multi_balanced_crt` (Azurite's Garner algorithm, then a shift
into the balanced range); the conversions to `Natural` and to the primitives that reject, clamp, or
flag; the absolute-value comparisons (`natAbs` and a comparison); the two's-complement limbs (the
low limb of each floor shift, `toUInt64 (z >>> 64i)`); the single-bit functions and `get_bits`
(floor shifts, parity, `ofAzInt` modulo a power of 2, and adding or subtracting a power of 2); and
`Clone`.

**Azurite would need new functions.** These are the operations whose absence from Azurite is the
real gap, in rough order of how much of Malachite they would cover:

1. The two's-complement bitwise operations on `AzInt` (`and`, `or`, `xor`, and `not`), and with them
   the population counts, the Hamming distance, the bit scans, bit iteration, conversion to and
   from bit vectors, and `assign_bits`. As with `Natural`, this is the largest gap.
2. The combinatorial functions: the binomial coefficient with a negative argument, and the falling
   and rising factorials. All are checked against GMP, FLINT, or a reference, but none against a
   proven implementation.
3. The Kronecker symbol, which extends the Jacobi symbol to even and negative denominators.
4. Perfect-power detection and decomposition (`is_power`, `express_as_power`) and `remove_power`.
5. The exhaustive generators without a counterpart: `exhaustive_nonzero_integers` and the
   half-bounded ranges in order of increasing absolute value.

**Outside the scope of an oracle.** The constants, `Default`, `Named`, `Hash`, `mutate_unsigned_abs`,
the unit functions (`is_unit`, `conjugate`, `canonicalize_unit`, `canonical_unit_i_pow`),
`IsInteger`, `IsReal`, `IsGaussianInteger`, `From<bool>`, `const_from_unsigned` and
`const_from_signed`, the GMP-style `format_integer_str`, LaTeX and Typst output, serde, the Python
bindings, and the random generators. Most are trivial or are checked by other means; the rest have
no independent implementation to compare with, and are tested by their own unit and property tests,
the random generators by the distribution of large samples.
