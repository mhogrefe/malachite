---
layout: default
title: "How Malachite Is Tested: Rationals"
permalink: /verification/rationals/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Rationals

This page lists every public function of [`Rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html), the rational number type of the
`malachite-q` crate, and records which independent implementations it is checked against.
[The introduction](/verification/) describes the oracles and the testing they sit in; this page is
the ledger. It follows the organization of the crate's documentation, one section per module, so
that a function is where its documentation is. A `Rational` is a sign and a reduced fraction of two
[`Natural`](/verification/naturals/)s, so its arithmetic rests on the `Natural` and
[`Integer`](/verification/integers/) functions, but each of its functions is listed and checked
here in its own right.

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
[`malachite_q::rational`](https://docs.rs/malachite-q/latest/malachite_q/rational/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [constants](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html) | `Zero`, `One`, `Two`, `NegativeOne`, `OneHalf` | | | | | |
| [default](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-Default-for-Rational) | `Default` | | | | | |
| [named](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-Named-for-Rational) | `Named` | | | | | |
| [hash](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-Hash-for-Rational) | `Hash` | | | | | |
| [clone](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-Clone-for-Rational) | `Clone` | | | ✓ | ✓ | |
| [significant_bits](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-SignificantBits-for-%26Rational) | `SignificantBits` | | | | | |

The constants are the values $$0$$, $$1$$, $$2$$, $$-1$$, and $$1/2$$; every oracle run uses them
on the way to checking something else, but no run checks them as such. `Named` gives the type's
name as a string, and `Hash` is derived from the sign, numerator, and denominator, so neither has
anything to compare against. `Clone`, also derived, is checked against GMP's and num's copies.
`significant_bits`, the total number of bits in the numerator and denominator, is covered by
property tests alone.

## Comparison {#comparison}

From
[`malachite_q::rational::comparison`](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/index.html).

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [cmp](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/cmp/index.html) | `Ord`, `PartialOrd` | ✓ |  | ✓ | ✓ |  |
| [eq](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#impl-PartialEq-for-Rational) | `PartialEq`, `Eq` | ✓ |  | ✓ | ✓ |  |
| [cmp_complexity](https://docs.rs/malachite-q/latest/malachite_q/rational/struct.Rational.html#method.cmp_complexity) | `cmp_complexity` |  |  |  |  |  |
| [cmp_abs](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/cmp_abs/index.html) | `OrdAbs`, `PartialOrdAbs` |  |  | ✓ |  |  |
| [cmp_abs](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/cmp_abs/index.html) | `OrdAbsDouble`, `PartialOrdAbsDouble` |  |  |  |  |  |
| [eq_abs](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/eq_abs/index.html) | `EqAbs` |  |  |  |  |  |
| [partial_cmp_natural](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_natural/index.html) | `PartialOrd<Natural>` and the reverse direction |  |  | ✓ |  |  |
| [partial_cmp_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_integer/index.html) | `PartialOrd<Integer>` and the reverse direction |  |  | ✓ |  |  |
| [partial_eq_natural](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_eq_natural/index.html) | `PartialEq<Natural>` and the reverse direction |  |  | ✓ |  |  |
| [partial_eq_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_eq_integer/index.html) | `PartialEq<Integer>` and the reverse direction |  |  | ✓ |  |  |
| [partial_cmp_primitive_int](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_primitive_int/index.html) | `PartialOrd<u8>`, …, `PartialOrd<isize>`, and the reverse directions |  |  | ✓ |  |  |
| [partial_eq_primitive_int](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_eq_primitive_int/index.html) | `PartialEq<u8>`, …, `PartialEq<isize>`, and the reverse directions |  |  | ✓ |  |  |
| [partial_cmp_primitive_float](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_primitive_float/index.html) | `PartialOrd<f32>`, `PartialOrd<f64>`, and the reverse directions |  |  | ✓ |  |  |
| [partial_eq_primitive_float](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_eq_primitive_float/index.html) | `PartialEq<f32>`, `PartialEq<f64>`, and the reverse directions |  |  | ✓ |  |  |
| [partial_cmp_abs_natural](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_abs_natural/index.html) | `PartialOrdAbs<Natural>` and the reverse direction |  |  |  |  |  |
| [partial_cmp_abs_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_abs_integer/index.html) | `PartialOrdAbs<Integer>` and the reverse direction |  |  |  |  |  |
| [eq_abs_natural](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/eq_abs_natural/index.html) | `EqAbs<Natural>` and the reverse direction |  |  |  |  |  |
| [eq_abs_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/eq_abs_integer/index.html) | `EqAbs<Integer>` and the reverse direction |  |  |  |  |  |
| [partial_cmp_abs_primitive_int](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_abs_primitive_int/index.html) | `PartialOrdAbs<u8>`, …, `PartialOrdAbs<isize>`, and the reverse directions |  |  |  |  |  |
| [eq_abs_primitive_int](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/eq_abs_primitive_int/index.html) | `EqAbs<u8>`, …, `EqAbs<isize>`, and the reverse directions |  |  |  |  |  |
| [partial_cmp_abs_primitive_float](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/partial_cmp_abs_primitive_float/index.html) | `PartialOrdAbs<f32>`, `PartialOrdAbs<f64>`, and the reverse directions |  |  |  |  |  |
| [eq_abs_primitive_float](https://docs.rs/malachite-q/latest/malachite_q/rational/comparison/eq_abs_primitive_float/index.html) | `EqAbs<f32>`, `EqAbs<f64>`, and the reverse directions |  |  |  |  |  |

The order and equality of two `Rational`s are checked by Azurite, GMP, and num. Comparison and
equality with a `Natural`, an `Integer`, a primitive integer, or a primitive float are checked by
GMP. Azurite checks the comparisons with a `Natural`, an `Integer`, or a primitive integer, and the
equalities with a `Natural` or an `Integer`, with the `Rational` on the left (`x < n`), but not the
reverse direction, so those rows carry no Azurite mark. Comparison of absolute values is checked
by GMP's `cmp_abs` for two `Rational`s; the mixed absolute-value comparisons and equalities, and
the doubled comparison `cmp_abs_double` ($$\operatorname{cmp}(|x|, 2|y|)$$), have no oracle.
`cmp_complexity`, the well-order by denominator, then absolute numerator, then sign, behind the
simplest-rational functions, is tested by unit and property tests alone.

## Arithmetic {#arithmetic}

From
[`malachite_q::rational::arithmetic`](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/index.html).
The module is large, so its rows are grouped by theme.

### Addition, subtraction, multiplication, and division {#addition-subtraction-multiplication-and-division}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [add](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/add/index.html) | `Add` | ✓ |  | ✓ | ✓ | ✓ |
| [add](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/add/index.html) | `Sum` |  |  |  |  | ✓ |
| [sub](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/sub/index.html) | `Sub` | ✓ |  | ✓ | ✓ | ✓ |
| [abs_diff](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/abs_diff/index.html) | `AbsDiff` |  |  |  |  |  |
| [neg](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/neg/index.html) | `Neg` | ✓ |  | ✓ | ✓ |  |
| [mul](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/mul/index.html) | `Mul` | ✓ |  | ✓ | ✓ | ✓ |
| [mul](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/mul/index.html) | `Product` |  |  |  |  | ✓ |
| [div](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/div/index.html) | `Div` | ✓ |  | ✓ | ✓ | ✓ |
| [div](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/div/index.html) | `CheckedDiv` |  |  |  | ✓ |  |
| [reciprocal](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/reciprocal/index.html) | `Reciprocal` | ✓ |  | ✓ | ✓ |  |
| [square](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/square/index.html) | `Square` |  |  |  |  |  |
| [abs_squared](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/abs_squared/index.html) | `AbsSquared` |  |  |  |  |  |
| [add_mul](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/add_mul/index.html) | `AddMul` |  |  |  |  | ✓ |
| [sub_mul](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/sub_mul/index.html) | `SubMul` |  |  |  |  |  |
| [mul_add_mul](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/mul_add_mul/index.html) | `MulAddMul` |  |  |  |  |  |
| [mul_sub_mul](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/mul_sub_mul/index.html) | `MulSubMul` |  |  |  |  |  |
| [average](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/average/index.html) | `Average` |  |  |  |  |  |

The four field operations, negation, and the reciprocal are checked by Azurite (`AzRat.add`,
`sub`, `mul`, `div`, `neg`, `inv`, each reducing its result to lowest terms with Azurite's GCD), by
GMP and num, and, for the four operations, by references that combine the fractions by the
schoolbook formula and reduce once at the end. `Sum` and `Product` are checked against references
that do the same over a whole sequence, and `add_mul` against a reference that multiplies and adds
in separate canonical steps. `checked_div` is checked against num. `square`, `abs_squared`,
`abs_diff`, `average`, and the other fused operations are covered by property tests that compare
them with the unfused combinations, which is a strong check but not an independent one.

### Shifts {#shifts}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [shl](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/shl/index.html) | `Shl<u8>`, …, `Shl<isize>` | ✓ |  | ✓ |  |  |
| [shr](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/shr/index.html) | `Shr<u8>`, …, `Shr<isize>` | ✓ |  | ✓ |  |  |
| [round_to_multiple_of_power_of_2](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/round_to_multiple_of_power_of_2/index.html) | `RoundToMultipleOfPowerOf2` |  |  |  |  |  |

Shifting a `Rational` multiplies or divides it by a power of 2 exactly, with no rounding, so a
negative count simply reverses the direction. Both shifts are checked by Azurite's `shiftLeft` and
`shiftRight` and against GMP for every shift type. `round_to_multiple_of_power_of_2` is covered by
property tests alone.

### Signs and units {#signs-and-units}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [sign](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/sign/index.html) | `Sign` | ✓ |  | ✓ | ✓ |  |
| [abs](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/abs/index.html) | `Abs` | ✓ |  | ✓ | ✓ |  |
| [is_unit](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/is_unit/index.html) | `IsUnit` |  |  |  |  |  |
| [conjugate](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/conjugate/index.html) | `Conjugate` |  |  |  |  |  |
| [canonicalize_unit](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/canonicalize_unit/index.html) | `CanonicalizeUnit` |  |  |  |  |  |
| [canonical_unit_i_pow](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/canonical_unit_i_pow/index.html) | `CanonicalUnitIPow` |  |  |  |  |  |

`sign` and `abs` are checked by Azurite (`AzRat.sign` and `AzRat.abs`), GMP, and num. The unit
functions (`is_unit`, true for every nonzero `Rational`; `conjugate`; `canonicalize_unit`;
`canonical_unit_i_pow`) are the trivial cases of functions that matter for Gaussian rationals and
polynomials, where they are cross-checked.

### Rounding {#rounding}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [floor](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/floor/index.html) | `Floor` | ✓ |  | ✓ | ✓ |  |
| [ceiling](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/ceiling/index.html) | `Ceiling` | ✓ |  | ✓ | ✓ |  |
| [round_to_multiple](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/round_to_multiple/index.html) | `RoundToMultiple` |  |  |  |  |  |
| [mod_op](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/mod_op/index.html) | `Mod` |  |  |  |  | ✓ |
| [mod_op](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/mod_op/index.html) | `Rem` |  |  |  | ✓ | ✓ |
| [mod_op](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/mod_op/index.html) | `CeilingMod` |  |  |  |  | ✓ |

`floor` and `ceiling`, which return an `Integer`, are checked by Azurite's `AzRat.round` in the
`Floor` and `Ceiling` modes, by GMP, and by num. The remainders of a `Rational` by a `Rational`
(`mod_op`, whose result takes the divisor's sign; `rem`, the dividend's; and `ceiling_mod`) are
checked against references that compute the floored, truncated, or ceiling quotient and subtract,
and `rem` also against num. `round_to_multiple` is covered by property tests alone; rounding to an
`Integer` in any mode is listed under [Integers and naturals](#integers-and-naturals).

### Powers, roots, and logarithms {#powers-roots-and-logarithms}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [pow](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/pow/index.html) | `Pow<u64>`, `Pow<i64>` | ✓ |  | ✓ | ✓ |  |
| [sqrt](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/sqrt/index.html) | `CheckedSqrt` |  |  |  |  |  |
| [root](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/root/index.html) | `CheckedRoot<u64>`, `CheckedRoot<i64>` |  |  |  |  |  |
| [express_as_power](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/express_as_power/index.html) | `ExpressAsPower` |  |  |  |  |  |
| [log_base_2](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/log_base_2/index.html) | `FloorLogBase2`, `CeilingLogBase2`, `floor_log_base_2_abs`, `ceiling_log_base_2_abs` | ✓ |  |  |  |  |
| [log_base_2](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/log_base_2/index.html) | `CheckedLogBase2` |  |  |  |  |  |
| [log_base](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/log_base/index.html) | `FloorLogBase`, `CeilingLogBase`, `CheckedLogBase` | ✓ |  |  |  |  |
| [log_base](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/log_base/index.html) | `approx_log` |  |  |  |  |  |
| [log_base_power_of_2](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/log_base_power_of_2/index.html) | `FloorLogBasePowerOf2`, `CeilingLogBasePowerOf2`, `CheckedLogBasePowerOf2` |  |  |  |  |  |

`pow`, with a non-negative or a negative exponent, is checked by Azurite (`AzRat.pow` and `zpow`),
GMP, and num. The base-2 logarithms are checked by Azurite's `floorLogBase2Abs` and
`ceilingLogBase2Abs`, and the logarithms to a `u64` base by `floorLogBaseAbs` (the ceiling and
checked forms derived from it by an exact power comparison in the oracle). `checked_log_base_2`,
the logarithms to a power-of-2 base, `checked_sqrt`, `checked_root`, and `express_as_power`, which
return a value only when the answer is exact, are covered by property tests alone, as is
`approx_log`, a floating-point estimate.

### Powers of 2 {#powers-of-2}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [power_of_2](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/power_of_2/index.html) | `PowerOf2<u64>`, `PowerOf2<i64>` |  |  |  |  |  |
| [is_power_of_2](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/is_power_of_2/index.html) | `IsPowerOf2` |  |  |  |  |  |
| [next_power_of_2](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/next_power_of_2/index.html) | `NextPowerOf2` |  |  |  |  |  |

The powers of 2, including negative ones ($$2^{-k}$$), `is_power_of_2`, and `next_power_of_2`
(the smallest power of 2 at least the input, possibly a fraction) are covered by property tests
alone.

### Approximation {#approximation}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [approximate](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/approximate/index.html) | `Approximate` |  |  |  |  | ✓ |
| [simplest_rational_in_interval](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/simplest_rational_in_interval/index.html) | `SimplestRationalInInterval` |  |  |  |  | ✓ |
| [denominators_in_closed_interval](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/denominators_in_closed_interval/index.html) | `DenominatorsInClosedInterval` |  |  |  |  |  |
| [farey_neighbors](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/farey_neighbors/index.html) | `Rational::farey_neighbors` |  | ✓ |  |  |  |
| [height](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/height/index.html) | `Height`, `HeightRef`, `height_significant_bits` |  | ✓ |  |  |  |
| [reconstruct](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/reconstruct/index.html) | `Rational::reconstruct`, `reconstruct_ref` |  | ✓ |  |  |  |
| [reconstruct](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/reconstruct/index.html) | `Rational::reconstruct_with_bounds`, `reconstruct_with_bounds_ref` |  | ✓ |  |  |  |

`approximate`, the closest `Rational` with a bounded denominator, is checked against a reference
that tries every denominator; `simplest_rational_in_interval` against references that search by
increasing denominator and that walk the continued fractions of the endpoints. The neighbors of a
`Rational` in the Farey sequence of a given order are checked against FLINT's
`fmpq_farey_neighbors`, the height (the larger of the numerator and denominator) and its bit count
against `fmpq_height` and `fmpq_height_bits`, and rational reconstruction from a residue, with the
default bounds and with explicit ones, against `fmpq_reconstruct_fmpz` and
`fmpq_reconstruct_fmpz_2`. `denominators_in_closed_interval` is covered by property tests alone.

### Number theory {#number-theory}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [gcd](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/gcd/index.html) | `Gcd` |  | ✓ |  |  |  |
| [extended_gcd](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/extended_gcd/index.html) | `ExtendedGcd` |  | ✓ |  |  |  |
| [dedekind_sum](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/dedekind_sum/index.html) | `Rational::dedekind_sum` |  | ✓ |  |  | ✓ |
| [harmonic_number](https://docs.rs/malachite-q/latest/malachite_q/rational/arithmetic/harmonic_number/index.html) | `Rational::harmonic_number` |  | ✓ |  |  | ✓ |

The GCD of two `Rational`s, the largest `Rational` of which both are integer multiples, is checked
against FLINT's `fmpq_gcd`, and the extended GCD, with its cofactors, against
`fmpq_gcd_cofactors`. The Dedekind sum $$s(h, k)$$ is checked against FLINT's `fmpq_dedekind_sum`
and against a reference that evaluates the defining sum term by term, and the harmonic numbers
$$H_n$$ against FLINT's `fmpq_harmonic_ui` and the sum $$\sum_{i=1}^n 1/i$$.

## Conversion {#conversion}

From
[`malachite_q::rational::conversion`](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/index.html).

### Integers and naturals {#integers-and-naturals}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [from_natural](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_natural/index.html) | `From<Natural>` | ✓ |  |  |  |  |
| [from_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_integer/index.html) | `From<Integer>` | ✓ |  |  |  |  |
| [from_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_numerator_and_denominator/index.html) | `from_naturals`, `from_naturals_ref` | ✓ |  |  |  |  |
| [from_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_numerator_and_denominator/index.html) | `from_integers`, `from_integers_ref` | ✓ |  | ✓ | ✓ |  |
| [from_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_numerator_and_denominator/index.html) | `from_sign_and_naturals`, `from_sign_and_naturals_ref` | ✓ |  |  |  |  |
| [from_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_numerator_and_denominator/index.html) | `from_unsigneds`, `from_sign_and_unsigneds` |  |  |  |  |  |
| [from_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_numerator_and_denominator/index.html) | `from_signeds` |  |  | ✓ |  |  |
| [from_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_numerator_and_denominator/index.html) | `const_from_unsigneds`, `const_from_signeds` |  |  |  |  |  |
| [to_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/to_numerator_and_denominator/index.html) | `to_numerator`, `into_numerator`, `numerator_ref` |  |  | ✓ | ✓ |  |
| [to_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/to_numerator_and_denominator/index.html) | `to_denominator`, `into_denominator`, `denominator_ref` |  |  | ✓ | ✓ |  |
| [to_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/to_numerator_and_denominator/index.html) | `to_numerator_and_denominator`, `into_numerator_and_denominator`, `numerator_and_denominator_ref` |  |  |  |  |  |
| [mutate_numerator_and_denominator](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/mutate_numerator_and_denominator/index.html) | `mutate_numerator`, `mutate_denominator`, `mutate_numerator_and_denominator` |  |  |  |  |  |
| [integer_from_rational](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html) | `RoundingFrom<Rational>` for `Integer` | ✓ |  |  |  |  |
| [integer_from_rational](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/integer_from_rational/index.html) | `TryFrom<Rational>`, `ConvertibleFrom<Rational>` for `Integer` |  |  |  |  |  |
| [natural_from_rational](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/natural_from_rational/index.html) | `RoundingFrom<Rational>`, `TryFrom<Rational>`, `ConvertibleFrom<Rational>` for `Natural` |  |  |  |  |  |
| [from_gaussian_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_gaussian_integer/index.html) | `TryFrom<GaussianInteger>`, `ConvertibleFrom<GaussianInteger>` |  |  |  |  |  |

Conversion from a `Natural` or an `Integer`, and construction from a numerator and denominator
(`from_naturals`, `from_integers`, `from_sign_and_naturals`, each reducing the fraction), are
checked by Azurite (`AzNat.toAzRat`, `AzInt.toAzRat`, `ofAzNats`, `ofAzInts`, and `ofSignAzNats`);
`from_integers` also against GMP and num, and `from_signeds` against GMP. The numerator and
denominator accessors are checked against GMP's and num's. Rounding a `Rational` to an `Integer` in
any mode is checked by Azurite's `AzRat.round`, `Exact` being checked as "the denominator is 1".
The fallible and clamping conversions to `Integer` and `Natural`, the constructors from primitive
pairs, the paired accessors, and `mutate_numerator_and_denominator`, which applies a function to
the parts and re-reduces, are covered by property tests alone, as is the conversion from a
Gaussian integer.

### Primitive types {#primitive-types}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [from_primitive_int](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_int/index.html) | `From<u8>`, …, `From<isize>` | ✓ |  | ✓ |  |  |
| [from_primitive_int](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_int/index.html) | `const_from_unsigned`, `const_from_signed` |  |  |  |  |  |
| [primitive_int_from_rational](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_int_from_rational/index.html) | `RoundingFrom<Rational>`, `TryFrom<Rational>`, `ConvertibleFrom<Rational>` for every primitive integer |  |  |  |  |  |
| [from_primitive_float](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_float/index.html) | `TryFrom<f32>`, `TryFrom<f64>` |  |  | ✓ |  |  |
| [from_primitive_float](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_primitive_float/index.html) | `ConvertibleFrom<f32>`, `ConvertibleFrom<f64>` |  |  |  |  |  |
| [from_float_simplest](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_float_simplest/index.html) | `try_from_float_simplest` |  |  |  |  |  |
| [primitive_float_from_rational](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html) | `RoundingFrom<Rational>` for `f32`, `f64` |  |  |  |  |  |
| [primitive_float_from_rational](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/primitive_float_from_rational/index.html) | `TryFrom<Rational>`, `ConvertibleFrom<Rational>` for `f32`, `f64` |  |  |  |  |  |
| [from_bool](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/from_bool/index.html) | `From<bool>` |  |  |  |  |  |
| [is_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/is_integer/index.html) | `IsInteger` |  |  |  |  |  |
| [is_real](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/is_real/index.html) | `IsReal` |  |  |  |  |  |
| [is_gaussian_integer](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/is_gaussian_integer/index.html) | `IsGaussianInteger` |  |  |  |  |  |

Conversion from every primitive integer type is checked by Azurite (`UInt64.toAzRat`,
`Int64.toAzRat`, and their narrower forms) and GMP, and the exact conversion from a finite `f32` or
`f64` by GMP. Conversion to a float is compared with GMP's `mpq_get_d` in the `Down` mode only,
since GMP's conversion truncates rather than rounding in a chosen mode; the other modes, like the
conversions to the primitive integers and `try_from_float_simplest` (the simplest `Rational` that
rounds to a given float), are covered by property tests alone.

### Mantissa and exponent {#mantissa-and-exponent}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [mantissa_and_exponent](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/mantissa_and_exponent/index.html) | `SciMantissaAndExponent<f32, i64>`, `SciMantissaAndExponent<f64, i64>` |  |  |  |  |  |
| [mantissa_and_exponent](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/mantissa_and_exponent/index.html) | `sci_mantissa_and_exponent_round`, `sci_mantissa_and_exponent_round_ref` |  |  |  |  |  |

The scientific mantissa and exponent of a `Rational`, a float mantissa in $$[1, 2)$$ and a signed
exponent, rounded in a chosen mode, are covered by property tests alone. The `Natural` versions are
checked by Azurite, and the same oracle would extend to a fraction.

### Continued fractions {#continued-fractions}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [to_continued_fraction](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/continued_fraction/to_continued_fraction/index.html) | `ContinuedFraction` |  |  |  |  |  |
| [from_continued_fraction](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/continued_fraction/from_continued_fraction/index.html) | `Rational::from_continued_fraction`, `from_continued_fraction_ref` |  |  |  |  | ✓ |
| [convergents](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/continued_fraction/convergents/index.html) | `Convergents` |  |  |  |  | ✓ |

Building a `Rational` from its continued fraction and listing its convergents are checked against
references that evaluate the continued fraction from the bottom up and that compute each convergent
from scratch. The continued fraction itself is tested by round trips through
`from_continued_fraction` and by its defining properties.

### Digits {#digits}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [digits](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/digits/digits/index.html) | `Rational::digits` |  |  |  |  |  |
| [to_digits](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/digits/to_digits/index.html) | `to_digits`, `into_digits` |  |  |  |  |  |
| [from_digits](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/digits/from_digits/index.html) | `Rational::from_digits`, `from_digits_ref` |  |  |  |  |  |
| [power_of_2_digits](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/digits/power_of_2_digits/index.html) | `Rational::power_of_2_digits` |  |  |  |  |  |
| [to_power_of_2_digits](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/digits/to_power_of_2_digits/index.html) | `to_power_of_2_digits`, `into_power_of_2_digits` |  |  |  |  |  |
| [from_power_of_2_digits](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/digits/from_power_of_2_digits/index.html) | `Rational::from_power_of_2_digits`, `from_power_of_2_digits_ref` |  |  |  |  |  |

The expansion of a `Rational` in a base, an integer part and a fractional part that eventually
repeats, and its inverse are covered by property tests alone: round trips through the
`from_*` functions, agreement of the integer part with the `Natural` digit functions (which are
checked by Azurite), and the expansion's shape, such as, in a power-of-2 base, a repeating part that
is empty exactly when the denominator is a power of 2.

### Strings {#strings}

| Operation | Functions | Azurite | FLINT | GMP | num | Reference |
| --- | --- | :---: | :---: | :---: | :---: | :---: |
| [to_string](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/to_string/index.html) | `Display`, `Debug` | ✓ |  | ✓ | ✓ |  |
| [to_string](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/to_string/index.html) | `Binary`, `Octal`, `LowerHex`, `UpperHex` |  |  | ✓ | ✓ |  |
| [to_string](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/to_string/index.html) | `ToStringBase` |  |  | ✓ |  |  |
| [from_string](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/from_string/index.html) | `FromStr` | ✓ |  | ✓ | ✓ |  |
| [from_string](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/from_string/index.html) | `FromStringBase` |  |  | ✓ |  |  |
| [from_sci_string](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/from_sci_string/index.html) | `FromSciString` | ✓ |  |  |  |  |
| [from_sci_string](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/from_sci_string/index.html) | `from_sci_string_simplest`, `from_sci_string_simplest_with_options` |  |  |  |  |  |
| [to_sci](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/to_sci/index.html) | `ToSci` | ✓ |  |  |  |  |
| [to_sci](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/to_sci/index.html) | `length_after_point_in_small_base` | ✓ |  |  |  |  |
| [format_rational](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/format_rational/index.html) | `format_rational_str`, `GmpFormatArg` |  |  | ✓ |  |  |
| [latex](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/latex/index.html) | `ToLatex` |  |  |  |  |  |
| [typst](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/string/typst/index.html) | `ToTypst` |  |  |  |  |  |
| [serde](https://docs.rs/malachite-q/latest/malachite_q/rational/conversion/serde/index.html) | `Serialize`, `Deserialize` |  |  |  |  |  |

Decimal output and input are checked by Azurite (`toString` and a parse in Malachite's grammar,
with its optional `+` signs and its rejection of a zero denominator), GMP, and num; output and
input in other bases, and the formatting traits, by GMP (and the traits also by num). Scientific
notation is checked by Azurite: `from_sci_string` in every base and with its options by
`AzRat.fromSci`, `to_sci` and `to_sci_with_options` digit for digit by `AzRat.toSci` (the options'
`Debug` text read into Azurite's `SciOptions`), `fmt_sci_valid` by `toSciExact`, and
`length_after_point_in_small_base`, the number of digits after the point when the expansion ends,
by `lengthAfterPoint`. The GMP-style `format_rational_str` is compared with GMP's own
`gmp_snprintf`. `from_sci_string_simplest`, which returns the simplest `Rational` that rounds to
the given digits, and LaTeX, Typst, and serde output are covered by property tests alone.

## Exhaustive generation {#exhaustive-generation}

*This section is not yet written.*

## Random generation {#random-generation}

*This section is not yet written.*

## What is not yet cross-checked {#not-yet-cross-checked}

*This section is not yet written.*
