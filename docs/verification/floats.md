---
layout: default
title: "How Malachite Is Tested: Floats"
permalink: /verification/floats/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Floats

This page lists every public function of [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), the arbitrary-precision binary
floating-point type of the `malachite-float` crate, and records which independent implementations
it is checked against. [The introduction](/verification/) describes the oracles and the testing
they sit in; this page is the ledger. It follows the organization of the crate's documentation, one
section per module, so that a function is where its documentation is.

## Reading the tables {#reading-the-tables}

Each row is one operation, named after the module that implements it and linked to that module's
documentation. The **Functions** column lists the traits and methods the module implements. A
trait's by-value and by-reference implementations, its `*Assign` form, and the `_prec`, `_round`,
and `_prec_round` variants of an operation (which differ only in the output precision and the
rounding mode) count as one function, while functions with genuinely different results are listed
separately. The oracles differ from those of the integer pages: FLINT and num have no
arbitrary-precision binary floats, and GMP's role is taken by [MPFR](https://www.mpfr.org/), the
reference implementation of correctly rounded arbitrary-precision arithmetic, through the
[`rug`](https://crates.io/crates/rug) crate.

| | meaning |
| :---: | --- |
| ✓ | The oracle computes this function and agrees with Malachite on every input tried. |
| ≈ | The oracle agrees, after an adaptation on the oracle side that is more than a change of spelling. |
| | The oracle does not check this function. |

Azurite's `AzFloat` has an unbounded exponent, while a `Float`'s exponent is bounded, so the
Azurite oracle applies Malachite's documented overflow and underflow rules to Azurite's result
before comparing; that adaptation is the same for every operation and is not counted as ≈. A
function with no mark in any column is checked only by Malachite's own unit and property tests;
those rows are collected in [What is not yet cross-checked](#not-yet-cross-checked). The internal
`ExtendedFloat` type, which carries a wider exponent through intermediate computations, is not
listed.

## Basic {#basic}

The constants, the classification predicates, and the accessors of the type itself, from
[`malachite_float::float::basic`](https://docs.rs/malachite-float/latest/malachite_float/float/basic/index.html).

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [constants](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html) | `Zero`, `One`, `Two`, `NegativeOne`, `OneHalf`, `NegativeZero`, `NaN`, `Infinity`, `NegativeInfinity`, `Min`, `Max` |  | ✓ |  |
| [constants](https://docs.rs/malachite-float/latest/malachite_float/float/basic/constants/index.html) | `one_prec`, `two_prec` | ✓ | ✓ |  |
| [constants](https://docs.rs/malachite-float/latest/malachite_float/float/basic/constants/index.html) | `negative_one_prec`, `one_half_prec` |  | ✓ |  |
| [constants](https://docs.rs/malachite-float/latest/malachite_float/float/basic/constants/index.html) | `min_positive_value_prec` | ✓ | ✓ |  |
| [constants](https://docs.rs/malachite-float/latest/malachite_float/float/basic/constants/index.html) | `max_finite_value_with_prec` | ✓ |  |  |
| [constants](https://docs.rs/malachite-float/latest/malachite_float/float/basic/constants/index.html) | `abs_is_min_positive_value`, `abs_is_max_finite_value_with_prec` |  |  |  |
| [default](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#impl-Default-for-Float) | `Default` |  |  |  |
| [named](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#impl-Named-for-Float) | `Named` |  |  |  |
| [clone](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html#impl-Clone-for-Float) | `Clone` |  | ✓ |  |
| [classification](https://docs.rs/malachite-float/latest/malachite_float/float/basic/classification/index.html) | `is_nan`, `is_finite`, `is_infinite`, `is_zero`, `is_normal` | ✓ | ✓ |  |
| [classification](https://docs.rs/malachite-float/latest/malachite_float/float/basic/classification/index.html) | `is_positive_zero`, `is_negative_zero`, `is_sign_positive`, `is_sign_negative`, `classify` |  | ✓ |  |
| [classification](https://docs.rs/malachite-float/latest/malachite_float/float/basic/classification/index.html) | `to_non_nan`, `into_non_nan`, `to_finite`, `into_finite` |  |  |  |
| [get_and_set](https://docs.rs/malachite-float/latest/malachite_float/float/basic/get_and_set/index.html) | `get_prec`, `get_exponent` | ✓ | ✓ |  |
| [get_and_set](https://docs.rs/malachite-float/latest/malachite_float/float/basic/get_and_set/index.html) | `to_significand`, `into_significand`, `significand_ref` | ✓ | ✓ |  |
| [get_and_set](https://docs.rs/malachite-float/latest/malachite_float/float/basic/get_and_set/index.html) | `get_min_prec` |  |  |  |
| [get_and_set](https://docs.rs/malachite-float/latest/malachite_float/float/basic/get_and_set/index.html) | `set_prec`, `set_prec_round` | ✓ | ✓ |  |
| [get_and_set](https://docs.rs/malachite-float/latest/malachite_float/float/basic/get_and_set/index.html) | `from_float_prec`, `from_float_prec_round`, and their `_ref` forms |  |  |  |
| [ulp](https://docs.rs/malachite-float/latest/malachite_float/float/basic/ulp/index.html) | `ulp` | ✓ |  |  |
| [ulp](https://docs.rs/malachite-float/latest/malachite_float/float/basic/ulp/index.html) | `increment`, `decrement` |  |  |  |
| [complexity](https://docs.rs/malachite-float/latest/malachite_float/float/basic/complexity/index.html) | `complexity`, `SignificantBits` |  |  |  |
| [can_round](https://docs.rs/malachite-float/latest/malachite_float/float/basic/can_round/index.html) | `can_round` |  | ✓ |  |
| [subnormalize](https://docs.rs/malachite-float/latest/malachite_float/float/basic/subnormalize/index.html) | `subnormalize`, `subnormalize_assign`, `subnormalize_ref` |  | ✓ |  |

The special values and the constants at a given precision are checked against MPFR's, and Azurite
checks `one_prec`, `two_prec`, and the extreme values `min_positive_value_prec` and
`max_finite_value_with_prec` (built in Azurite from Malachite's documented exponent range). The
classification predicates are checked against MPFR's, and the ones that do not depend on the sign
of a zero also against Azurite's. The precision, exponent, and significand accessors, and
`set_prec` and `set_prec_round` (rounding to a new precision), are checked by Azurite and MPFR;
`ulp` by Azurite. `can_round`, which decides whether an approximation with a known error bound can
be rounded correctly to a target precision, is checked against MPFR's `mpfr_can_round`, and
`subnormalize`, which emulates an IEEE 754 format's subnormal range, against
`mpfr_subnormalize`. `increment`, `decrement`, `get_min_prec`, `complexity`, and the
`from_float_prec` constructors, which compute the same thing as `set_prec_round` on a new value,
are covered by property tests alone.

## Comparison {#comparison}

From [`malachite_float::float::comparison`](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/index.html). `Float`'s own comparison
follows IEEE 754: `NaN` is unordered and `0.0 == -0.0`. The wrapper `ComparableFloat` gives the
total order and equality used for hashing and sorting, under which `NaN` equals itself and the two
zeros differ.

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [cmp](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/cmp/index.html) | `PartialOrd` for `Float` | ✓ | ✓ |  |
| [cmp](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/cmp/index.html) | `Ord`, `PartialOrd` for `ComparableFloat` | ✓ |  |  |
| [eq](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/eq/index.html) | `PartialEq` for `Float` | ✓ | ✓ |  |
| [eq](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/eq/index.html) | `PartialEq`, `Eq` for `ComparableFloat` | ✓ |  |  |
| [hash](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/hash/index.html) | `Hash` for `ComparableFloat` |  |  |  |
| [cmp_abs](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/cmp_abs/index.html) | `PartialOrdAbs` for `Float` |  | ✓ |  |
| [cmp_abs](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/cmp_abs/index.html) | `OrdAbs`, `PartialOrdAbs` for `ComparableFloat` |  |  |  |
| [cmp_abs](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/cmp_abs/index.html) | `PartialOrdAbsDouble` |  |  |  |
| [eq_abs](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/eq_abs/index.html) | `EqAbs` for `Float` and `ComparableFloat` |  |  |  |
| [min_max](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/min_max/index.html) | `min`, `max`, and their `_prec`, `_round`, and `_prec_round` forms |  | ✓ |  |
| [min_max](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/min_max/index.html) | `min_rational`, `max_rational`, and their `_prec`, `_round`, and `_prec_round` forms |  |  |  |
| [min_max](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/min_max/index.html) | `primitive_float_min_rational`, `primitive_float_max_rational` |  |  |  |
| [partial_cmp_natural](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_natural/index.html) | `PartialOrd<Natural>` and the reverse direction |  | ✓ |  |
| [partial_cmp_integer](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_integer/index.html) | `PartialOrd<Integer>` and the reverse direction |  | ✓ |  |
| [partial_cmp_rational](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_rational/index.html) | `PartialOrd<Rational>` and the reverse direction |  | ✓ | ✓ |
| [partial_eq_natural](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_eq_natural/index.html) | `PartialEq<Natural>` and the reverse direction |  | ✓ |  |
| [partial_eq_integer](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_eq_integer/index.html) | `PartialEq<Integer>` and the reverse direction |  | ✓ |  |
| [partial_eq_rational](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_eq_rational/index.html) | `PartialEq<Rational>` and the reverse direction |  | ✓ |  |
| [partial_cmp_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_primitive_int/index.html) | `PartialOrd<u8>`, …, `PartialOrd<isize>`, and the reverse directions |  | ✓ |  |
| [partial_eq_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_eq_primitive_int/index.html) | `PartialEq<u8>`, …, `PartialEq<isize>`, and the reverse directions |  | ✓ |  |
| [partial_cmp_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_primitive_float/index.html) | `PartialOrd<f32>`, `PartialOrd<f64>`, and the reverse directions |  | ✓ |  |
| [partial_eq_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_eq_primitive_float/index.html) | `PartialEq<f32>`, `PartialEq<f64>`, and the reverse directions |  | ✓ |  |
| [partial_cmp_abs_natural](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_abs_natural/index.html) | `PartialOrdAbs<Natural>` and the reverse direction |  | ✓ |  |
| [partial_cmp_abs_integer](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_abs_integer/index.html) | `PartialOrdAbs<Integer>` and the reverse direction |  | ✓ |  |
| [partial_cmp_abs_rational](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_abs_rational/index.html) | `PartialOrdAbs<Rational>` and the reverse direction |  | ✓ |  |
| [partial_cmp_abs_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_abs_primitive_int/index.html) | `PartialOrdAbs<u8>`, …, `PartialOrdAbs<isize>`, and the reverse directions |  | ✓ |  |
| [partial_cmp_abs_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_abs_primitive_float/index.html) | `PartialOrdAbs<f32>`, `PartialOrdAbs<f64>`, and the reverse directions |  |  |  |
| [eq_abs_natural](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/eq_abs_natural/index.html) | `EqAbs<Natural>`, `EqAbs<Integer>`, `EqAbs<Rational>`, and the reverse directions |  |  |  |
| [eq_abs_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/eq_abs_primitive_int/index.html) | `EqAbs` with the primitive integers and floats, and the reverse directions |  |  |  |
| [partial_eq_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_eq_gaussian_integer/index.html) | `PartialEq<GaussianInteger>`, `PartialEq<GaussianRational>`, and the reverse directions |  |  |  |
| [partial_cmp_abs_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/float/comparison/partial_cmp_abs_gaussian_integer/index.html) | `PartialOrdAbs` and `EqAbs` with `GaussianInteger` and `GaussianRational`, and the reverse directions |  |  |  |

`Float`'s comparison and equality are checked by Azurite and MPFR; `ComparableFloat`'s total order
and equality by Azurite, which reads the sign of a zero from the printed value. Comparison and
equality with a `Natural`, an `Integer`, a `Rational`, a primitive integer, or a primitive float
are checked against MPFR (comparison with a `Rational` also against a reference that compares
exponents first and converts the `Float` to a `Rational` exactly only when they tie). Azurite checks the comparisons and equalities with a `Natural` or
an `Integer` with the `Float` on the left, but not the reverse direction, so those rows carry no
Azurite mark. `min` and `max`, which round their result to a precision, are checked against MPFR's
`mpfr_min` and `mpfr_max`; MPFR has no comparison of absolute values with a `Natural`, an
`Integer`, a `Rational`, or a primitive integer, and those are checked against MPFR's comparison of
the absolute values. The absolute-value comparisons with floats, the absolute-value equalities,
the comparisons with Gaussian integers and rationals, the `Rational` forms of `min` and `max`, and
`partial_cmp_abs_double` are covered by property tests alone.

## Constants {#constants}

From [`malachite_float::float::constants`](https://docs.rs/malachite-float/latest/malachite_float/float/constants/index.html). Each constant is computed to any
precision and rounded in any mode, and the returned `Ordering` says which way it was rounded.

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [catalans_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/catalans_constant/index.html) | `catalans_constant_prec`, `catalans_constant_prec_round` |  | ✓ |  |
| [cbrt_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/cbrt_2/index.html) | `cbrt_2_prec`, `cbrt_2_prec_round` |  | ✓ |  |
| [champernowne_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/champernowne_constant/index.html) | `champernowne_constant_prec`, `champernowne_constant_prec_round`, `champernowne_constant_base_prec`, `champernowne_constant_base_prec_round` |  |  | ✓ |
| [champernowne_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/champernowne_constant/index.html) | `primitive_float_champernowne_constant_base` |  |  |  |
| [copeland_erdos_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/copeland_erdos_constant/index.html) | `copeland_erdos_constant_prec`, `copeland_erdos_constant_prec_round`, `copeland_erdos_constant_base_prec`, `copeland_erdos_constant_base_prec_round` |  |  | ✓ |
| [copeland_erdos_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/copeland_erdos_constant/index.html) | `primitive_float_copeland_erdos_constant_base` |  |  |  |
| [dottie_number](https://docs.rs/malachite-float/latest/malachite_float/float/constants/dottie_number/index.html) | `dottie_number_prec`, `dottie_number_prec_round` |  |  |  |
| [e](https://docs.rs/malachite-float/latest/malachite_float/float/constants/e/index.html) | `e_prec`, `e_prec_round` |  | ✓ |  |
| [eulers_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/eulers_constant/index.html) | `eulers_constant_prec`, `eulers_constant_prec_round` |  | ✓ |  |
| [gauss_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/gauss_constant/index.html) | `gauss_constant_prec`, `gauss_constant_prec_round` |  |  |  |
| [gelfond_schneider_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/gelfond_schneider_constant/index.html) | `gelfond_schneider_constant_prec`, `gelfond_schneider_constant_prec_round` |  |  |  |
| [gelfonds_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/gelfonds_constant/index.html) | `gelfonds_constant_prec`, `gelfonds_constant_prec_round` |  |  |  |
| [lemniscate_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/lemniscate_constant/index.html) | `lemniscate_constant_prec`, `lemniscate_constant_prec_round` |  |  | ✓ |
| [liouvilles_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/liouvilles_constant/index.html) | `liouvilles_constant_prec`, `liouvilles_constant_prec_round`, `liouvilles_constant_base_prec`, `liouvilles_constant_base_prec_round` |  |  | ✓ |
| [liouvilles_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/liouvilles_constant/index.html) | `primitive_float_liouvilles_constant_base` |  |  |  |
| [ln_10](https://docs.rs/malachite-float/latest/malachite_float/float/constants/ln_10/index.html) | `ln_10_prec`, `ln_10_prec_round` |  | ✓ |  |
| [ln_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/ln_2/index.html) | `ln_2_prec`, `ln_2_prec_round` |  | ✓ |  |
| [log_10_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_10_2/index.html) | `log_10_2_prec`, `log_10_2_prec_round` |  | ✓ |  |
| [log_10_e](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_10_e/index.html) | `log_10_e_prec`, `log_10_e_prec_round` |  |  | ✓ |
| [log_2_10](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_2_10/index.html) | `log_2_10_prec`, `log_2_10_prec_round` |  | ✓ |  |
| [log_2_e](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_2_e/index.html) | `log_2_e_prec`, `log_2_e_prec_round` |  |  | ✓ |
| [one_over_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/one_over_pi/index.html) | `one_over_pi_prec`, `one_over_pi_prec_round` |  |  | ✓ |
| [one_over_sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/one_over_sqrt_pi/index.html) | `one_over_sqrt_pi_prec`, `one_over_sqrt_pi_prec_round` |  |  | ✓ |
| [one_over_sqrt_tau](https://docs.rs/malachite-float/latest/malachite_float/float/constants/one_over_sqrt_tau/index.html) | `one_over_sqrt_tau_prec`, `one_over_sqrt_tau_prec_round` |  |  | ✓ |
| [phi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/phi/index.html) | `phi_prec`, `phi_prec_round` | ✓ |  |  |
| [pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi/index.html) | `pi_prec`, `pi_prec_round` |  | ✓ |  |
| [pi_over_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_2/index.html) | `pi_over_2_prec`, `pi_over_2_prec_round` |  |  |  |
| [pi_over_3](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_3/index.html) | `pi_over_3_prec`, `pi_over_3_prec_round` |  |  | ✓ |
| [pi_over_4](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_4/index.html) | `pi_over_4_prec`, `pi_over_4_prec_round` |  |  |  |
| [pi_over_6](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_6/index.html) | `pi_over_6_prec`, `pi_over_6_prec_round` |  |  |  |
| [pi_over_8](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_8/index.html) | `pi_over_8_prec`, `pi_over_8_prec_round` |  |  |  |
| [prime_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/prime_constant/index.html) | `prime_constant_prec`, `prime_constant_prec_round` | ✓ |  |  |
| [prouhet_thue_morse_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/prouhet_thue_morse_constant/index.html) | `prouhet_thue_morse_constant_prec`, `prouhet_thue_morse_constant_prec_round` | ✓ |  | ✓ |
| [ramanujans_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/ramanujans_constant/index.html) | `ramanujans_constant_prec`, `ramanujans_constant_prec_round` |  |  |  |
| [sqrt_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_2/index.html) | `sqrt_2_prec`, `sqrt_2_prec_round` | ✓ | ✓ |  |
| [sqrt_2_over_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_2_over_2/index.html) | `sqrt_2_over_2_prec`, `sqrt_2_over_2_prec_round` | ✓ | ✓ |  |
| [sqrt_3](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_3/index.html) | `sqrt_3_prec`, `sqrt_3_prec_round` | ✓ | ✓ |  |
| [sqrt_3_over_3](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_3_over_3/index.html) | `sqrt_3_over_3_prec`, `sqrt_3_over_3_prec_round` | ✓ | ✓ |  |
| [sqrt_5](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_5/index.html) | `sqrt_5_prec`, `sqrt_5_prec_round` | ✓ | ✓ |  |
| [sqrt_5_over_5](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_5_over_5/index.html) | `sqrt_5_over_5_prec`, `sqrt_5_over_5_prec_round` | ✓ | ✓ |  |
| [sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_pi/index.html) | `sqrt_pi_prec`, `sqrt_pi_prec_round` |  |  | ✓ |
| [tau](https://docs.rs/malachite-float/latest/malachite_float/float/constants/tau/index.html) | `tau_prec`, `tau_prec_round` |  |  |  |
| [two_over_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/two_over_pi/index.html) | `two_over_pi_prec`, `two_over_pi_prec_round` |  |  |  |
| [two_over_sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/two_over_sqrt_pi/index.html) | `two_over_sqrt_pi_prec`, `two_over_sqrt_pi_prec_round` |  |  |  |

The fifteen constants that MPFR provides directly or computes as a correctly rounded value of one of
its functions ($$\pi$$, $$e$$, Euler's and Catalan's constants, $$\ln 2$$, $$\ln 10$$, $$\log_{10}
2$$, $$\log_2 10$$, and the square and cube roots) are compared with MPFR at every precision and
rounding mode the tests generate. Twelve more are checked against references: the digit-defined
constants (Champernowne's, Copeland–Erdős, and Liouville's, in any base, and the Prouhet–Thue–Morse
constant) against a digit-by-digit expansion bracketed by exact `Rational`s, and the constants built
from others ($$1/\pi$$, $$\pi/3$$, $$\sqrt{\pi}$$, the lemniscate constant, $$\log_2 e$$, and the
like) against a computation that brackets the inputs at a higher precision and rounds once the
bracket decides the result. Every constant, those with no oracle included, is also compared with a
hard-coded value at precisions 1,000 and 10,000, and its rounding `Ordering`s are checked against
its neighbors: the `Floor` and `Ceiling` results must be adjacent, with the exact value between
them. The constants that are $$\pi$$ or $$\tau$$ times a power of 2 ($$\pi/2$$, $$\pi/4$$,
$$\pi/8$$, $$\tau$$) are exact shifts of a checked value; $$\pi/6$$, $$2/\pi$$, $$2/\sqrt{\pi}$$,
the Dottie number, Gauss's constant, Gelfond's and the Gelfond–Schneider constants, and Ramanujan's
constant rely on those hard-coded values and their property tests. Nine constants are also checked
against Azurite (`AzFloat.sqrt2PrecRound` and its siblings) at every precision and rounding mode the
demos generate, up to precision 10,000: $$\sqrt2$$, $$\sqrt3$$, $$\sqrt5$$, $$\sqrt2/2$$,
$$\sqrt3/3$$, $$\sqrt5/5$$, $$\phi$$, the prime constant, and the Prouhet–Thue–Morse constant. For
$$\phi$$ and the prime constant, Azurite is the only oracle.

## Arithmetic {#arithmetic}

From [`malachite_float::float::arithmetic`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/index.html). The module is large, so
its rows are grouped by theme.

### Addition, subtraction, multiplication, and division {#addition-subtraction-multiplication-and-division}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [add](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add/index.html) | `Add`, `add_prec_round` | ✓ | ✓ | ✓ |
| [add](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add/index.html) | `add_rational_prec_round` | ✓ | ✓ | ✓ |
| [sub](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub/index.html) | `Sub`, `sub_prec_round` | ✓ | ✓ | ✓ |
| [sub](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub/index.html) | `sub_rational_prec_round` | ✓ | ✓ | ✓ |
| [mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul/index.html) | `Mul`, `mul_prec_round` | ✓ | ✓ | ✓ |
| [mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul/index.html) | `mul_rational_prec_round` | ✓ | ✓ | ✓ |
| [div](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/div/index.html) | `Div`, `div_prec_round` | ✓ | ✓ | ✓ |
| [div](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/div/index.html) | `div_rational_prec_round` | ✓ | ✓ | ✓ |
| [div](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/div/index.html) | `rational_div_float_prec_round` | ✓ | ✓ | ✓ |
| [neg](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/neg/index.html) | `Neg` | ✓ | ✓ |  |
| [abs](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/abs/index.html) | `Abs` | ✓ | ✓ |  |
| [abs](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/abs/index.html) | `abs_negative_zero` |  |  |  |
| [reciprocal](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal/index.html) | `Reciprocal`, `reciprocal_prec_round` |  | ✓ | ✓ |
| [square](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/square/index.html) | `Square`, `square_prec_round` | ✓ | ✓ | ✓ |
| [abs_squared](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/abs_squared/index.html) | `AbsSquared` |  |  |  |
| [sum](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sum/index.html) | `Sum`, `sum_prec_round` |  | ✓ |  |
| [sum](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sum/index.html) | `primitive_float_sum` |  |  |  |
| [product](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/product/index.html) | `Product`, `product_prec_round` |  |  |  |
| [product](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/product/index.html) | `primitive_float_product` |  |  |  |
| [dot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/dot/index.html) | `dot_prec_round` |  | ✓ |  |
| [dot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/dot/index.html) | `primitive_float_dot` |  |  |  |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/index.html) | `AddMul`, `add_mul_prec_round` |  | ✓ | ✓ |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/index.html) | `add_mul_rational_prec_round` |  |  | ✓ |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/index.html) | `primitive_float_add_mul`, `primitive_float_add_mul_rational` |  |  |  |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/index.html) | `SubMul`, `sub_mul_prec_round` |  | ✓ | ✓ |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/index.html) | `sub_mul_rational_prec_round` |  |  | ✓ |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/index.html) | `primitive_float_sub_mul`, `primitive_float_sub_mul_rational` |  |  |  |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/index.html) | `MulAddMul`, `mul_add_mul_prec_round` |  | ✓ | ✓ |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/index.html) | `mul_add_mul_rational_prec_round` |  |  | ✓ |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/index.html) | `primitive_float_mul_add_mul`, `primitive_float_mul_add_mul_rational` |  |  |  |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/index.html) | `MulSubMul`, `mul_sub_mul_prec_round` |  | ✓ | ✓ |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/index.html) | `mul_sub_mul_rational_prec_round` |  |  | ✓ |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/index.html) | `primitive_float_mul_sub_mul`, `primitive_float_mul_sub_mul_rational` |  |  |  |
| [average](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/average/index.html) | `Average`, `average_prec_round` |  |  |  |
| [positive_difference](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/positive_difference/index.html) | `positive_difference_prec_round` |  | ✓ |  |
| [positive_difference](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/positive_difference/index.html) | `positive_difference_rational_prec_round` |  |  |  |
| [positive_difference](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/positive_difference/index.html) | `rational_positive_difference_float_prec_round` |  |  |  |
| [positive_difference](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/positive_difference/index.html) | `primitive_float_positive_difference`, `primitive_float_positive_difference_rational`, `primitive_float_rational_positive_difference_float` |  |  |  |
| [hypot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/hypot/index.html) | `Hypot`, `hypot_prec_round` |  | ✓ |  |
| [hypot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/hypot/index.html) | `primitive_float_hypot` |  |  |  |

Most operations come in several input forms, each its own row: two `Float`s; a `Float` and a
`Rational`, which is used exactly rather than rounded first; and, for the `primitive_float_*`
functions, `f32` or `f64` inputs with a correctly rounded `f32` or `f64` result. The four field
operations, squaring, negation, and absolute value are checked by Azurite (`AzFloat.addPrecRound`
and its siblings) and MPFR, and the four operations and squaring also against references that
compute the exact result as a `Rational` and round it once. MPFR's operations with a rational
operand are exact before their single rounding, so the `Rational` forms of the four operations are
compared with them directly; they are also checked by Azurite (`AzFloat.addRatPrecRound`,
`subRatPrecRound`, `mulRatPrecRound`, `divRatPrecRound`, `ratSubPrecRound`, and
`ratDivPrecRound`), including `Rational + Float`, `Rational - Float`, `Rational * Float`, and
`Rational / Float`. Some of the demos on extreme inputs (those for addition and subtraction, and the
`_round` and `_prec_round` forms of multiplication and division) take a fiftieth to a tenth of a
second per line, so they are checked on 300 lines in each generator mode rather than 10,000. Sums, dot products, `hypot`, `positive_difference` (MPFR's
`mpfr_dim`), and the fused operations (MPFR's `mpfr_fma` and `mpfr_fmma` families) are checked
against MPFR, and the fused operations' `Rational` forms against exact references. `product`,
`average`, `abs_squared`, `abs_negative_zero`, the `Rational` form of `positive_difference`, and
the `primitive_float_*` versions in this section are covered by property tests, which compare the
primitive versions with the `Float` functions at the primitive type's precision and exponent
range.

### Shifts and powers of 2 {#shifts-and-powers-of-2}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [shl](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shl/index.html) | `Shl` | ✓ | ✓ | ✓ |
| [shr](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shr/index.html) | `Shr` | ✓ | ✓ | ✓ |
| [shl_round](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shl_round/index.html) | `ShlRound`, `shl_prec_round` |  | ✓ | ✓ |
| [shr_round](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shr_round/index.html) | `ShrRound`, `shr_prec_round` |  | ✓ | ✓ |
| [power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2/index.html) | `PowerOf2`, `power_of_2_prec_round` | ✓ | ✓ | ✓ |
| [is_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/is_power_of_2/index.html) | `IsPowerOf2` | ✓ |  |  |

Shifts multiply by a power of 2 and change only the exponent; they are checked by Azurite and MPFR,
and against a reference that shifts the exact `Rational` value. `shl_round` and `shr_round`, which
also round to a new precision, are checked against MPFR and similar references. `power_of_2` is
checked by Azurite, MPFR, and references for every exponent type, and `is_power_of_2` by Azurite.

### Signs and units {#signs-and-units}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [sign](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sign/index.html) | `Sign` | ✓ |  |  |
| [is_unit](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/is_unit/index.html) | `IsUnit` |  |  |  |
| [conjugate](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/conjugate/index.html) | `Conjugate` |  |  |  |
| [canonicalize_unit](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/canonicalize_unit/index.html) | `CanonicalizeUnit` |  |  |  |
| [canonical_unit_i_pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/canonical_unit_i_pow/index.html) | `CanonicalUnitIPow` |  |  |  |

`sign` is checked by Azurite. The unit functions are the trivial cases of functions that matter
for Gaussian numbers and polynomials, where they are cross-checked.

### Rounding {#rounding}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_prec_round` |  | ✓ |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_then_prec_round` |  | ✓ |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_ties_away` |  |  |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_ties_away_then_prec_round` |  |  |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `primitive_float_round_to_integer`, `primitive_float_round_to_integer_ties_away` |  |  |  |
| [fractional_part](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/fractional_part/index.html) | `fractional_part_prec_round` |  | ✓ |  |
| [fractional_part](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/fractional_part/index.html) | `integer_and_fractional_parts_prec_round` |  | ✓ |  |
| [fractional_part](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/fractional_part/index.html) | `primitive_float_fractional_part`, `primitive_float_integer_and_fractional_parts` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `Rem`, `rem_prec_round` |  | ✓ |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `ieee_remainder_prec_round` |  | ✓ |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `ieee_remainder_and_quotient_bits_prec_round` |  | ✓ |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `ieee_remainder_rational_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `ieee_remainder_rational_and_quotient_bits_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rational_ieee_remainder_float_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rational_ieee_remainder_float_and_quotient_bits_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rational_rem_float_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rational_rem_float_and_quotient_bits_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rem_and_quotient_bits_prec_round` |  | ✓ |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rem_rational_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rem_rational_and_quotient_bits_prec_round` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `rem_unsigned_prec_round` |  | ✓ |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `primitive_float_ieee_remainder`, `primitive_float_ieee_remainder_and_quotient_bits`, `primitive_float_ieee_remainder_rational`, `primitive_float_ieee_remainder_rational_and_quotient_bits`, `primitive_float_rational_ieee_remainder_float`, `primitive_float_rational_rem_float`, `primitive_float_rem`, `primitive_float_rem_and_quotient_bits`, `primitive_float_rem_rational`, `primitive_float_rem_rational_and_quotient_bits`, `primitive_float_rem_unsigned` |  |  |  |

Rounding to an integer, including the forms that round the integer again to a target precision
(`round_to_integer_then`), is checked against MPFR's rounding functions, called directly; the forms
that break ties away from zero are covered by property tests. The fractional part and the split
into integer and fractional parts are checked against MPFR's `mpfr_frac` and `mpfr_modf`. The
remainders are checked against MPFR's truncated and IEEE 754 remainders, and the versions that also
return the low bits of the quotient against `mpfr_fmodquo` and `mpfr_remquo`; the forms with a
`Rational` operand on either side have no MPFR counterpart and are covered by property tests.

### Roots and powers {#roots-and-powers}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `Sqrt`, `sqrt_prec_round` | ✓ | ✓ |  |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `sqrt_rational_prec_round` |  |  | ✓ |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `sqrt_unsigned_prec_round` |  |  |  |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `primitive_float_sqrt_rational` |  |  |  |
| [reciprocal_sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal_sqrt/index.html) | `ReciprocalSqrt`, `reciprocal_sqrt_prec_round` | ✓ | ✓ |  |
| [reciprocal_sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal_sqrt/index.html) | `reciprocal_sqrt_rational_prec_round` |  |  | ✓ |
| [reciprocal_sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal_sqrt/index.html) | `primitive_float_reciprocal_sqrt`, `primitive_float_reciprocal_sqrt_rational` |  |  |  |
| [cbrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cbrt/index.html) | `Cbrt`, `cbrt_prec_round` |  | ✓ |  |
| [cbrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cbrt/index.html) | `cbrt_rational_prec_round` |  |  |  |
| [cbrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cbrt/index.html) | `primitive_float_cbrt`, `primitive_float_cbrt_rational` |  |  |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `Root<i64>`, `root_s_prec_round` |  | ✓ |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `root_s_rational_prec_round` |  |  |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `Root<u64>`, `root_u_prec_round` |  | ✓ |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `root_u_rational_prec_round` |  |  |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `primitive_float_root_s`, `primitive_float_root_s_rational`, `primitive_float_root_u`, `primitive_float_root_u_rational` |  |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `Pow`, `pow_prec_round` |  | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_integer_prec_round` |  | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_rational_prec_round` |  |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_s_prec_round` |  | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_u_prec_round` |  | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `powr_prec_round` |  |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `rational_pow_prec_round` |  |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `rational_pow_rational_prec_round` |  |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `unsigned_pow_prec_round` |  | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `unsigned_pow_rational_prec_round` |  | ≈ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `unsigned_pow_unsigned_prec_round` |  | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `primitive_float_pow`, `primitive_float_pow_integer`, `primitive_float_pow_rational`, `primitive_float_pow_u`, `primitive_float_rational_pow`, `primitive_float_unsigned_pow` |  |  |  |
| [compound](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/compound/index.html) | `Compound`, `compound_prec_round` |  | ✓ |  |
| [compound](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/compound/index.html) | `primitive_float_compound` |  |  |  |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/agm/index.html) | `Agm`, `agm_prec_round` |  | ✓ |  |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/agm/index.html) | `agm_rational_prec_round` |  |  |  |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/agm/index.html) | `primitive_float_agm`, `primitive_float_agm_rational` |  |  |  |

The square root, reciprocal square root, cube root, and roots with a `u64` or `i64` index are
checked against MPFR (`mpfr_sqrt`, `mpfr_rec_sqrt`, `mpfr_cbrt`, `mpfr_rootn_ui`, and
`mpfr_rootn_si`), and the square root and
reciprocal square root also by Azurite; their `Rational` forms are checked against references that
use an exact root when there is one and bracket the result otherwise. Powers with a `Float`,
`Integer`, `u64`, or `i64` exponent, and the powers of an unsigned base, are checked against MPFR's
`mpfr_pow` and `mpfr_pow_z`; `compound`, $$(1 + x)^n$$, against `mpfr_compound_si`; and the
arithmetic-geometric mean against `mpfr_agm`. An unsigned base raised to a `Rational` is ≈: MPFR
has no such function, so the oracle brackets the result with MPFR's `exp` and `ln` at increasing
precision until its rounding is decided. `powr`, the powers with a `Rational` base or with a
`Float` base and a `Rational` exponent, the `Rational` forms of `agm` and the roots, and the
`primitive_float_*` versions are covered by property tests.

### Exponentials and logarithms {#exponentials-and-logarithms}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/exp/index.html) | `Exp`, `exp_prec_round` |  | ✓ |  |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/exp/index.html) | `exp_rational_prec_round` |  | ≈ |  |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/exp/index.html) | `primitive_float_exp`, `primitive_float_exp_rational` |  |  |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/exp_x_minus_1/index.html) | `ExpXMinus1`, `exp_x_minus_1_prec_round` |  | ✓ |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/exp_x_minus_1/index.html) | `exp_x_minus_1_rational_prec_round` |  | ≈ |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/exp_x_minus_1/index.html) | `primitive_float_exp_x_minus_1`, `primitive_float_exp_x_minus_1_rational` |  |  |  |
| [power_of_2_of_float](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2_of_float/index.html) | `PowerOf2`, `power_of_2_of_float_prec_round` |  | ✓ |  |
| [power_of_2_of_float](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2_of_float/index.html) | `power_of_2_rational_prec_round` |  | ≈ |  |
| [power_of_2_of_float](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2_of_float/index.html) | `primitive_float_power_of_2`, `primitive_float_power_of_2_rational` |  |  |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2_x_minus_1/index.html) | `PowerOf2XMinus1`, `power_of_2_x_minus_1_prec_round` |  | ✓ |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2_x_minus_1/index.html) | `power_of_2_x_minus_1_rational_prec_round` |  | ≈ |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2_x_minus_1/index.html) | `primitive_float_power_of_2_x_minus_1`, `primitive_float_power_of_2_x_minus_1_rational` |  |  |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_10/index.html) | `PowerOf10`, `power_of_10_of_float_prec_round` |  | ✓ |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_10/index.html) | `power_of_10_rational_prec_round` |  | ≈ |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_10/index.html) | `primitive_float_power_of_10`, `primitive_float_power_of_10_rational` |  |  |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_10_x_minus_1/index.html) | `PowerOf10XMinus1`, `power_of_10_x_minus_1_prec_round` |  | ✓ |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_10_x_minus_1/index.html) | `power_of_10_x_minus_1_rational_prec_round` |  | ≈ |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_10_x_minus_1/index.html) | `primitive_float_power_of_10_x_minus_1`, `primitive_float_power_of_10_x_minus_1_rational` |  |  |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/ln/index.html) | `Ln`, `ln_prec_round` |  | ✓ |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/ln/index.html) | `ln_rational_prec_round` |  |  |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/ln/index.html) | `ln_unsigned_prec_round` |  |  |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/ln/index.html) | `primitive_float_ln`, `primitive_float_ln_rational` |  |  |  |
| [ln_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/ln_1_plus_x/index.html) | `Ln1PlusX`, `ln_1_plus_x_prec_round` |  | ✓ |  |
| [ln_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/ln_1_plus_x/index.html) | `primitive_float_ln_1_plus_x` |  |  |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_2/index.html) | `LogBase2`, `log_base_2_prec_round` |  | ✓ |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_2/index.html) | `log_base_2_rational_prec_round` |  | ≈ |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_2/index.html) | `primitive_float_log_base_2`, `primitive_float_log_base_2_rational` |  |  |  |
| [log_base_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_2_1_plus_x/index.html) | `LogBase2Of1PlusX`, `log_base_2_1_plus_x_prec_round` |  | ✓ |  |
| [log_base_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_2_1_plus_x/index.html) | `primitive_float_log_base_2_1_plus_x` |  |  |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_10/index.html) | `LogBase10`, `log_base_10_prec_round` |  | ✓ |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_10/index.html) | `log_base_10_rational_prec_round` |  | ≈ |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_10/index.html) | `primitive_float_log_base_10`, `primitive_float_log_base_10_rational` |  |  |  |
| [log_base_10_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_10_1_plus_x/index.html) | `LogBase10Of1PlusX`, `log_base_10_1_plus_x_prec_round` |  | ✓ |  |
| [log_base_10_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_10_1_plus_x/index.html) | `primitive_float_log_base_10_1_plus_x` |  |  |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base/index.html) | `LogBase`, `log_base_prec_round` |  | ≈ |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base/index.html) | `log_base_rational_prec_round` |  | ≈ |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base/index.html) | `primitive_float_log_base`, `primitive_float_log_base_rational` |  |  |  |
| [log_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_1_plus_x/index.html) | `LogBaseOf1PlusX`, `log_base_1_plus_x_prec_round` |  | ≈ |  |
| [log_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_1_plus_x/index.html) | `primitive_float_log_base_1_plus_x` |  |  |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_power_of_2/index.html) | `LogBasePowerOf2`, `log_base_power_of_2_prec_round` |  | ≈ |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_power_of_2/index.html) | `log_base_power_of_2_rational_prec_round` |  | ≈ |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_power_of_2/index.html) | `primitive_float_log_base_power_of_2`, `primitive_float_log_base_power_of_2_rational` |  |  |  |
| [log_base_power_of_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_power_of_2_1_plus_x/index.html) | `LogBasePowerOf2Of1PlusX`, `log_base_power_of_2_1_plus_x_prec_round` |  | ≈ |  |
| [log_base_power_of_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_power_of_2_1_plus_x/index.html) | `primitive_float_log_base_power_of_2_1_plus_x` |  |  |  |
| [log_base_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_float_base/index.html) | `LogBase`, `log_base_float_base_prec_round` |  | ≈ |  |
| [log_base_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_float_base/index.html) | `primitive_float_log_base_float_base` |  |  |  |
| [log_base_float_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_float_base_1_plus_x/index.html) | `LogBaseOf1PlusX`, `log_base_float_base_1_plus_x_prec_round` |  | ≈ |  |
| [log_base_float_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_float_base_1_plus_x/index.html) | `primitive_float_log_base_float_base_1_plus_x` |  |  |  |
| [log_base_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_base/index.html) | `LogBase`, `log_base_rational_base_prec_round` |  | ≈ |  |
| [log_base_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_base/index.html) | `primitive_float_log_base_rational_base` |  |  |  |
| [log_base_rational_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_base_1_plus_x/index.html) | `LogBaseOf1PlusX`, `log_base_rational_base_1_plus_x_prec_round` |  | ≈ |  |
| [log_base_rational_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_base_1_plus_x/index.html) | `primitive_float_log_base_rational_base_1_plus_x` |  |  |  |
| [log_base_rational_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_float_base/index.html) | `log_base_rational_float_base_prec_round` |  | ≈ |  |
| [log_base_rational_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_float_base/index.html) | `primitive_float_log_base_rational_float_base` |  |  |  |
| [log_base_rational_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_rational_base/index.html) | `log_base_rational_rational_base_prec_round` |  | ≈ |  |
| [log_base_rational_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/log_base_rational_rational_base/index.html) | `primitive_float_log_base_rational_rational_base` |  |  |  |

The exponentials and the logarithms to bases 2, $$e$$, and 10, with and without the $$x - 1$$ and
$$1 + x$$ forms, are checked against the corresponding MPFR functions. The logarithms to other
bases (a `u64`, a power of 2, a `Float`, or a `Rational`) are ≈: MPFR has no such functions, so the
oracle brackets the quotient of two MPFR logarithms at increasing precision until its rounding is
decided, and recognizes the exactly representable results directly. The `Rational` forms are ≈ for
the reason given under [Trigonometric functions](#trigonometric-functions); `ln_rational` and
`ln_unsigned` are covered by property tests, and the `primitive_float_*` versions are compared with
the `Float` functions.

### Trigonometric functions {#trigonometric-functions}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `Sin`, `sin_prec_round` |  | ✓ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `sin_pi_prec_round` |  | ✓ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `sin_pi_rational_prec_round` |  |  |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `sin_rational_prec_round` |  | ≈ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `sin_with_period_prec_round` |  | ✓ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `sin_with_period_rational_prec_round` |  | ≈ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `primitive_float_sin`, `primitive_float_sin_rational`, `primitive_float_sin_with_period`, `primitive_float_sin_with_period_rational` |  | ≈ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin/index.html) | `primitive_float_sin_pi`, `primitive_float_sin_pi_rational` |  |  |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `Cos`, `cos_prec_round` |  | ✓ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `cos_pi_prec_round` |  | ✓ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `cos_pi_rational_prec_round` |  |  |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `cos_rational_prec_round` |  | ≈ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `cos_with_period_prec_round` |  | ✓ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `cos_with_period_rational_prec_round` |  | ≈ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `primitive_float_cos`, `primitive_float_cos_rational`, `primitive_float_cos_with_period`, `primitive_float_cos_with_period_rational` |  | ≈ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cos/index.html) | `primitive_float_cos_pi`, `primitive_float_cos_pi_rational` |  |  |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `Tan`, `tan_prec_round` |  | ✓ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `tan_pi_prec_round` |  | ✓ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `tan_pi_rational_prec_round` |  |  |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `tan_rational_prec_round` |  | ≈ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `tan_with_period_prec_round` |  | ✓ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `tan_with_period_rational_prec_round` |  | ≈ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `primitive_float_tan`, `primitive_float_tan_rational`, `primitive_float_tan_with_period`, `primitive_float_tan_with_period_rational` |  | ≈ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tan/index.html) | `primitive_float_tan_pi`, `primitive_float_tan_pi_rational` |  |  |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `Sec`, `sec_prec_round` |  | ✓ |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `sec_pi_prec_round` |  |  | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `sec_pi_rational_prec_round` |  |  |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `sec_rational_prec_round` |  | ≈ |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `sec_with_period_prec_round` |  |  | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `sec_with_period_rational_prec_round` |  |  | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `primitive_float_sec`, `primitive_float_sec_rational` |  | ≈ |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sec/index.html) | `primitive_float_sec_pi`, `primitive_float_sec_pi_rational`, `primitive_float_sec_with_period`, `primitive_float_sec_with_period_rational` |  |  |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `Csc`, `csc_prec_round` |  | ✓ |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `csc_pi_prec_round` |  |  | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `csc_pi_rational_prec_round` |  |  |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `csc_rational_prec_round` |  | ≈ |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `csc_with_period_prec_round` |  |  | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `csc_with_period_rational_prec_round` |  |  | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `primitive_float_csc`, `primitive_float_csc_rational` |  | ≈ |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csc/index.html) | `primitive_float_csc_pi`, `primitive_float_csc_pi_rational`, `primitive_float_csc_with_period`, `primitive_float_csc_with_period_rational` |  |  |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `Cot`, `cot_prec_round` |  | ✓ |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `cot_pi_prec_round` |  |  | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `cot_pi_rational_prec_round` |  |  |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `cot_rational_prec_round` |  | ≈ |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `cot_with_period_prec_round` |  |  | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `cot_with_period_rational_prec_round` |  |  | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `primitive_float_cot`, `primitive_float_cot_rational` |  | ≈ |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cot/index.html) | `primitive_float_cot_pi`, `primitive_float_cot_pi_rational`, `primitive_float_cot_with_period`, `primitive_float_cot_with_period_rational` |  |  |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin_cos/index.html) | `SinCos`, `sin_cos_prec_round` |  | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin_cos/index.html) | `sin_cos_pi_prec_round` |  | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin_cos/index.html) | `sin_cos_pi_rational_prec_round` |  | ≈ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin_cos/index.html) | `sin_cos_rational_prec_round` |  | ≈ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin_cos/index.html) | `sin_cos_with_period_prec_round` |  | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin_cos/index.html) | `sin_cos_with_period_rational_prec_round` |  | ≈ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sin_cos/index.html) | `primitive_float_sin_cos`, `primitive_float_sin_cos_pi`, `primitive_float_sin_cos_pi_rational`, `primitive_float_sin_cos_rational`, `primitive_float_sin_cos_with_period`, `primitive_float_sin_cos_with_period_rational` |  |  |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asin/index.html) | `Asin`, `asin_prec_round` |  | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asin/index.html) | `asin_pi_prec_round` |  | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asin/index.html) | `asin_pi_rational_prec_round` |  | ≈ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asin/index.html) | `asin_rational_prec_round` |  | ≈ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asin/index.html) | `asin_with_period_prec_round` |  | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asin/index.html) | `asin_with_period_rational_prec_round` |  | ≈ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asin/index.html) | `primitive_float_asin`, `primitive_float_asin_pi`, `primitive_float_asin_pi_rational`, `primitive_float_asin_rational`, `primitive_float_asin_with_period`, `primitive_float_asin_with_period_rational` |  |  |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acos/index.html) | `Acos`, `acos_prec_round` |  | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acos/index.html) | `acos_pi_prec_round` |  | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acos/index.html) | `acos_pi_rational_prec_round` |  | ≈ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acos/index.html) | `acos_rational_prec_round` |  | ≈ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acos/index.html) | `acos_with_period_prec_round` |  | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acos/index.html) | `acos_with_period_rational_prec_round` |  | ≈ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acos/index.html) | `primitive_float_acos`, `primitive_float_acos_pi`, `primitive_float_acos_pi_rational`, `primitive_float_acos_rational`, `primitive_float_acos_with_period`, `primitive_float_acos_with_period_rational` |  |  |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `Atan`, `atan_prec_round` |  | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `atan_pi_prec_round` |  | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `atan_pi_rational_prec_round` |  | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `atan_rational_prec_round` |  | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `atan_with_period_prec_round` |  | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `atan_with_period_rational_prec_round` |  | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `primitive_float_atan`, `primitive_float_atan_rational` |  | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan/index.html) | `primitive_float_atan_pi`, `primitive_float_atan_pi_rational`, `primitive_float_atan_with_period`, `primitive_float_atan_with_period_rational` |  |  |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan2/index.html) | `Atan2`, `atan2_prec_round` |  | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan2/index.html) | `atan2_pi_prec_round` |  | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan2/index.html) | `atan2_pi_rational_prec_round` |  |  |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan2/index.html) | `atan2_rational_prec_round` |  | ≈ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan2/index.html) | `atan2_with_period_prec_round` |  | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan2/index.html) | `atan2_with_period_rational_prec_round` |  |  |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atan2/index.html) | `primitive_float_atan2`, `primitive_float_atan2_pi`, `primitive_float_atan2_pi_rational`, `primitive_float_atan2_rational`, `primitive_float_atan2_with_period`, `primitive_float_atan2_with_period_rational` |  |  |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asec/index.html) | `Asec`, `asec_prec_round` |  | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asec/index.html) | `asec_pi_prec_round` |  | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asec/index.html) | `asec_pi_rational_prec_round` |  | ≈ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asec/index.html) | `asec_rational_prec_round` |  | ≈ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asec/index.html) | `asec_with_period_prec_round` |  | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asec/index.html) | `asec_with_period_rational_prec_round` |  | ≈ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asec/index.html) | `primitive_float_asec`, `primitive_float_asec_pi`, `primitive_float_asec_pi_rational`, `primitive_float_asec_rational`, `primitive_float_asec_with_period`, `primitive_float_asec_with_period_rational` |  |  |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsc/index.html) | `Acsc`, `acsc_prec_round` |  | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsc/index.html) | `acsc_pi_prec_round` |  | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsc/index.html) | `acsc_pi_rational_prec_round` |  | ≈ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsc/index.html) | `acsc_rational_prec_round` |  | ≈ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsc/index.html) | `acsc_with_period_prec_round` |  | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsc/index.html) | `acsc_with_period_rational_prec_round` |  | ≈ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsc/index.html) | `primitive_float_acsc`, `primitive_float_acsc_pi`, `primitive_float_acsc_pi_rational`, `primitive_float_acsc_rational`, `primitive_float_acsc_with_period`, `primitive_float_acsc_with_period_rational` |  |  |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acot/index.html) | `Acot`, `acot_prec_round` |  | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acot/index.html) | `acot_pi_prec_round` |  | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acot/index.html) | `acot_pi_rational_prec_round` |  | ≈ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acot/index.html) | `acot_rational_prec_round` |  | ≈ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acot/index.html) | `acot_with_period_prec_round` |  | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acot/index.html) | `acot_with_period_rational_prec_round` |  | ≈ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acot/index.html) | `primitive_float_acot`, `primitive_float_acot_pi`, `primitive_float_acot_pi_rational`, `primitive_float_acot_rational`, `primitive_float_acot_with_period`, `primitive_float_acot_with_period_rational` |  |  |  |

Each trigonometric function comes in several forms: of a `Float` in radians; of a `Rational`; of
a multiple of $$\pi$$ (`sin_pi(x)` is $$\sin \pi x$$); and with a period (`sin_with_period(x, u)`
is $$\sin 2\pi x / u$$ for an integer $$u$$), the last two each with a `Float` or a `Rational`
argument. The `Float` forms are checked against MPFR directly, through `mpfr_sin` and its siblings,
the `mpfr_sinpi` family, and the `mpfr_sinu` family, which takes the same integer period. The
`_pi` and `_with_period` forms of `sec`, `csc`, and `cot`, which MPFR lacks, are checked against
references that bracket the reciprocal of a wider-precision sine or cosine and round once both ends
agree; on the rare inputs where the bracket does not settle the rounding, those references decline
to answer and the input is passed over.

The `Rational` forms are ≈. MPFR takes only binary floating-point arguments, so the oracle first
rounds the `Rational` to a `Float` with 128 more bits than the target precision, plus the bits of
its denominator and exponent, and applies MPFR to that. The result agrees with the exact one except
in the vanishingly rare case where the true value lies within those extra bits of a rounding
boundary, and the tests require agreement on every input. The `primitive_float_*` versions of
`sin`, `cos`, `tan`, `sec`, `csc`, `cot`, and `atan` are ≈ in a similar way: the oracle's value is
mapped into the primitive type, whose exponent range differs from MPFR's, by rounding it once at the
precision it has there (rounding first to more bits and then to the primitive type would round
twice, which goes wrong for values just short of a midpoint of the primitive type). The other
primitive forms are compared with the `Float` functions.

### Hyperbolic functions {#hyperbolic-functions}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sinh/index.html) | `Sinh`, `sinh_prec_round` |  | ✓ |  |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sinh/index.html) | `sinh_rational_prec_round` |  | ≈ |  |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sinh/index.html) | `primitive_float_sinh`, `primitive_float_sinh_rational` |  | ≈ |  |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cosh/index.html) | `Cosh`, `cosh_prec_round` |  | ✓ |  |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cosh/index.html) | `cosh_rational_prec_round` |  | ≈ |  |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cosh/index.html) | `primitive_float_cosh`, `primitive_float_cosh_rational` |  | ≈ |  |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tanh/index.html) | `Tanh`, `tanh_prec_round` |  | ✓ |  |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tanh/index.html) | `tanh_rational_prec_round` |  | ≈ |  |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/tanh/index.html) | `primitive_float_tanh`, `primitive_float_tanh_rational` |  | ≈ |  |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sech/index.html) | `Sech`, `sech_prec_round` |  | ✓ |  |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sech/index.html) | `sech_rational_prec_round` |  | ≈ |  |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sech/index.html) | `primitive_float_sech`, `primitive_float_sech_rational` |  | ≈ |  |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csch/index.html) | `Csch`, `csch_prec_round` |  | ✓ |  |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csch/index.html) | `csch_rational_prec_round` |  | ≈ |  |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/csch/index.html) | `primitive_float_csch`, `primitive_float_csch_rational` |  | ≈ |  |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/coth/index.html) | `Coth`, `coth_prec_round` |  | ✓ |  |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/coth/index.html) | `coth_rational_prec_round` |  | ≈ |  |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/coth/index.html) | `primitive_float_coth`, `primitive_float_coth_rational` |  | ≈ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sinh_cosh/index.html) | `SinhCosh`, `sinh_cosh_prec_round` |  | ✓ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sinh_cosh/index.html) | `sinh_cosh_rational_prec_round` |  | ≈ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sinh_cosh/index.html) | `primitive_float_sinh_cosh`, `primitive_float_sinh_cosh_rational` |  |  |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asinh/index.html) | `Asinh`, `asinh_prec_round` |  | ✓ |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asinh/index.html) | `primitive_float_asinh` |  | ≈ |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asinh/index.html) | `asinh_rational_prec_round` |  | ≈ |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asinh/index.html) | `primitive_float_asinh_rational` |  | ≈ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acosh/index.html) | `Acosh`, `acosh_prec_round` |  | ✓ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acosh/index.html) | `primitive_float_acosh` |  | ≈ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acosh/index.html) | `acosh_rational_prec_round` |  | ≈ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acosh/index.html) | `primitive_float_acosh_rational` |  | ≈ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atanh/index.html) | `Atanh`, `atanh_prec_round` |  | ✓ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atanh/index.html) | `primitive_float_atanh` |  | ≈ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atanh/index.html) | `atanh_rational_prec_round` |  | ≈ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/atanh/index.html) | `primitive_float_atanh_rational` |  | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asech/index.html) | `Asech`, `asech_prec_round` |  | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asech/index.html) | `primitive_float_asech` |  | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asech/index.html) | `asech_rational_prec_round` |  | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/asech/index.html) | `primitive_float_asech_rational` |  | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsch/index.html) | `Acsch`, `acsch_prec_round` |  | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsch/index.html) | `primitive_float_acsch` |  | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsch/index.html) | `acsch_rational_prec_round` |  | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acsch/index.html) | `primitive_float_acsch_rational` |  | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acoth/index.html) | `Acoth`, `acoth_prec_round` |  | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acoth/index.html) | `primitive_float_acoth` |  | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acoth/index.html) | `acoth_rational_prec_round` |  | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/acoth/index.html) | `primitive_float_acoth_rational` |  | ≈ |  |

The hyperbolic functions, their reciprocals, `sinh_cosh`, and the inverse hyperbolic sine, cosine,
and tangent of a `Float` are checked against MPFR directly. The `Rational` forms are ≈, for the
reason given under [Trigonometric functions](#trigonometric-functions). For the inverse hyperbolic
cosine of a `Rational` near 1, the oracle adds as many bits as the magnitude of the exponent of
$$x - 1$$, since a relative error $$e$$ in the input moves the result by about $$e/(2(x-1))$$; for the
inverse hyperbolic tangent of a `Rational` near $$\pm1$$ it adds as many as the magnitude of the
exponent of $$1 - |x|$$, for the same reason.

The `primitive_float_*` versions are ≈: the oracle's value is mapped into the primitive type, whose
exponent range differs from MPFR's. For `sinh`, `cosh`, `tanh`, `asinh`, and `acosh`, MPFR computes
at the primitive type's own precision and an overflow is mapped to infinity; for the others the
value is rounded once at the precision it has in the primitive type, as under
[Trigonometric functions](#trigonometric-functions), which matters for values just short of a
midpoint of the primitive type, such as $$\operatorname{acsch}(2^{150}/32767)$$ in `f32`.
`primitive_float_sinh_cosh` is compared with `primitive_float_sinh` and `primitive_float_cosh`.

MPFR has no inverse hyperbolic secant, cosecant, or cotangent, so every `asech`, `acsch`, and
`acoth` row is ≈. For `asech`, the oracle evaluates $$\operatorname{acosh}(1/x)$$ with MPFR for
$$x \geq \frac{1}{2}$$, the reciprocal being an exact `Rational` (with the extra input bits of the
inverse hyperbolic cosine's `Rational` oracle), and $$\ln(1+\sqrt{1-x^2}) - \ln x$$ with 128 extra
bits for smaller $$x$$, where $$1/x$$ could overflow. For `acsch`, it evaluates
$$\operatorname{asinh}(1/x)$$, the reciprocal again being exact, and for extreme exponents
$$\ln(1+\sqrt{1+x^2}) - \ln|x|$$ or the inverse hyperbolic sine of a reciprocal rounded with 128
extra bits. For `acoth`, it evaluates $$\operatorname{atanh}(1/x)$$, whose `Rational` oracle's extra
input bits cover the ill-conditioning near $$|x| = 1$$, or, for a huge $$x$$, the inverse hyperbolic
tangent of a reciprocal rounded with 128 extra bits. The property tests also compare each `Float`
function with the `Rational` function of the exact reciprocal (`acosh_rational_prec_round`,
`asinh_rational_prec_round`, or `atanh_rational_prec_round`), an independent Malachite computation,
and each `Rational` function with the `Float` one.

### Other functions {#other-functions}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [factorial](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/factorial/index.html) | `factorial_prec_round` |  | ✓ |  |

The factorial of an integer, rounded to a precision, is checked against MPFR's `mpfr_fac_ui`.

## Conversion {#conversion}

From [`malachite_float::float::conversion`](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/index.html).

### Integers, naturals, and rationals {#integers-naturals-and-rationals}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [from_natural](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_natural/index.html) | `TryFrom<Natural>` | ✓ |  |  |
| [from_natural](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_natural/index.html) | `ConvertibleFrom<Natural>` |  |  |  |
| [from_natural](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_natural/index.html) | `from_natural_prec`, `from_natural_prec_round` | ✓ | ✓ |  |
| [from_integer](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_integer/index.html) | `TryFrom<Integer>` | ✓ |  |  |
| [from_integer](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_integer/index.html) | `ConvertibleFrom<Integer>` |  |  |  |
| [from_integer](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_integer/index.html) | `from_integer_prec`, `from_integer_prec_round` | ✓ | ✓ |  |
| [from_rational](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_rational/index.html) | `TryFrom<Rational>`, `ConvertibleFrom<Rational>` |  |  |  |
| [from_rational](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_rational/index.html) | `from_rational_prec`, `from_rational_prec_round` | ✓ | ✓ |  |
| [natural_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/natural_from_float/index.html) | `RoundingFrom<Float>`, `TryFrom<Float>`, `ConvertibleFrom<Float>` for `Natural` |  |  |  |
| [integer_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/integer_from_float/index.html) | `RoundingFrom<Float>` for `Integer` |  | ✓ |  |
| [integer_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/integer_from_float/index.html) | `TryFrom<Float>`, `ConvertibleFrom<Float>` for `Integer` |  |  |  |
| [rational_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/rational_from_float/index.html) | `TryFrom<Float>` for `Rational` |  | ✓ |  |
| [rational_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/rational_from_float/index.html) | `ConvertibleFrom<Float>` for `Rational` |  |  |  |
| [from_bits](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_bits/index.html) | `non_dyadic_from_bits_prec`, `non_dyadic_from_bits_prec_round` |  |  | ✓ |
| [from_digits](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_digits/index.html) | `non_dyadic_from_digits_prec`, `non_dyadic_from_digits_prec_round`, `non_dyadic_from_power_of_2_digits_prec`, `non_dyadic_from_power_of_2_digits_prec_round` |  |  | ✓ |
| [is_integer](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/is_integer/index.html) | `IsInteger` |  |  |  |
| [is_real](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/is_real/index.html) | `IsReal` |  |  |  |

Conversion from a `Natural`, an `Integer`, or a `Rational` to a given precision and rounding mode
is checked by Azurite (`setPrecRound` of the exact `ofAzNat` or `ofAzInt` value, and
`ofAzRatRound`) and MPFR; the exact conversions from a `Natural` or `Integer` (`TryFrom`, which
uses the least precision that holds the value and fails only when the value is too large for the
exponent range) by Azurite. Rounding a `Float` to an `Integer` is
checked against MPFR's conversion in each rounding mode, and the exact conversion to a `Rational`
against MPFR's. The conversions to `Natural`, the fallible conversions to `Integer`, and the
`ConvertibleFrom` predicates are covered by property tests. The `non_dyadic_from_*` constructors build
a `Float` from an infinite stream of bits or digits whose value is known not to be dyadic. They are
checked against a reference that brackets the value between exact `Rational`s, on irrational digit
streams and on the expansions of non-dyadic `Rational`s in many bases; on the latter they must also
agree with `from_rational_prec_round`, which Azurite and MPFR check.

### Primitive types {#primitive-types}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [from_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_int/index.html) | `From<u8>`, …, `From<usize>` | ✓ | ✓ |  |
| [from_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_int/index.html) | `From<i8>`, …, `From<isize>` |  | ✓ |  |
| [from_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_int/index.html) | `from_unsigned_prec`, `from_unsigned_prec_round`, `from_signed_prec`, `from_signed_prec_round` |  | ✓ |  |
| [from_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_int/index.html) | `const_from_unsigned`, `const_from_signed`, `const_from_unsigned_times_power_of_2`, `const_from_signed_times_power_of_2` |  |  |  |
| [primitive_int_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_int_from_float/index.html) | `RoundingFrom<Float>`, `TryFrom<Float>`, `ConvertibleFrom<Float>` for every primitive integer |  |  |  |
| [from_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_float/index.html) | `From<f32>`, `From<f64>` |  | ✓ |  |
| [from_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_primitive_float/index.html) | `from_primitive_float_prec`, `from_primitive_float_prec_round` |  | ✓ |  |
| [primitive_float_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_float_from_float/index.html) | `RoundingFrom<Float>` for `f32`, `f64` |  | ✓ |  |
| [primitive_float_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/primitive_float_from_float/index.html) | `TryFrom<Float>`, `ConvertibleFrom<Float>` for `f32`, `f64` |  |  |  |

Conversion from every primitive integer and float type, exactly or to a given precision, is checked
against MPFR, and from the unsigned types also by Azurite. Rounding a `Float` to an `f32` or `f64`
is compared with MPFR's own conversion in every rounding mode, subnormal results included. The
conversions to the primitive integers, the exact float conversions, and the `const` constructors
are covered by property tests.

### Mantissa and exponent {#mantissa-and-exponent}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [mantissa_and_exponent](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/mantissa_and_exponent/index.html) | `RawMantissaAndExponent` |  |  |  |
| [mantissa_and_exponent](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/mantissa_and_exponent/index.html) | `IntegerMantissaAndExponent` |  |  |  |
| [mantissa_and_exponent](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/mantissa_and_exponent/index.html) | `SciMantissaAndExponent` with a `Float` mantissa |  |  |  |
| [mantissa_and_exponent](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/mantissa_and_exponent/index.html) | `SciMantissaAndExponent` with an `f32` or `f64` mantissa, `sci_mantissa_and_exponent_round` |  |  |  |

The decompositions of a `Float` into a mantissa and an exponent (the raw significand and exponent,
an odd integer mantissa, and a mantissa in $$[1, 2)$$ as a `Float` or a primitive float) are
covered by unit and property tests, including round trips through their `from_*` inverses.

### Gaussian integers and rationals {#gaussian-integers-and-rationals}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [from_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_gaussian_integer/index.html) | `TryFrom<GaussianInteger>`, `ConvertibleFrom<GaussianInteger>` |  |  |  |
| [from_gaussian_rational](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/from_gaussian_rational/index.html) | `TryFrom<GaussianRational>`, `ConvertibleFrom<GaussianRational>` |  |  |  |
| [gaussian_integer_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/gaussian_integer_from_float/index.html) | `TryFrom<Float>`, `ConvertibleFrom<Float>` for `GaussianInteger` |  |  |  |
| [gaussian_rational_from_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/gaussian_rational_from_float/index.html) | `TryFrom<Float>`, `ConvertibleFrom<Float>` for `GaussianRational` |  |  |  |
| [is_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/is_gaussian_integer/index.html) | `IsGaussianInteger` |  |  |  |

The conversions between a `Float` and the Gaussian types succeed exactly when the imaginary part is
zero and the real part converts, and they are covered by property tests.

### Strings {#strings}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [to_string](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/to_string/index.html) | `Display`, `Debug` for `Float` and `ComparableFloat` |  |  |  |
| [to_string](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/to_string/index.html) | `Binary`, `Octal`, `LowerHex`, `UpperHex`, `ToStringBase` |  |  |  |
| [from_string](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/from_string/index.html) | `FromStr`, `FromStringBase` for `Float` and `ComparableFloat` |  |  |  |
| [from_sci_string](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/from_sci_string/index.html) | `FromSciString`, `from_sci_string_prec`, `from_sci_string_prec_round`, `from_sci_string_with_options_prec` |  |  | ✓ |
| [to_sci](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/to_sci/index.html) | `ToSci` |  |  |  |
| [get_str](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/index.html) | `get_str` |  | ✓ |  |
| [get_str](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/get_str/index.html) | `get_str_digit_count` |  |  |  |
| [strtofr](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/strtofr/index.html) | `strtofr`, `set_str` |  | ✓ |  |
| [format_float](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/format_float/index.html) | `format_float_str`, `GmpFormatArg` |  | ✓ |  |
| [latex](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/latex/index.html) | `ToLatex` |  |  |  |
| [typst](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/string/typst/index.html) | `ToTypst` |  |  |  |
| [serde](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/serde/index.html) | `Serialize`, `Deserialize` |  |  |  |

The MPFR-style string functions are checked against MPFR itself: `get_str` against `mpfr_get_str`,
`strtofr` and `set_str` against `mpfr_strtofr` and `mpfr_set_str`, and the `printf`-style
`format_float_str` against `mpfr_snprintf`. Parsing scientific notation to a precision is checked
against a reference that parses the string exactly as a `Rational` and rounds it once. Malachite's
own `Display` (enough correctly rounded digits, a number fixed by the precision, to read back to the
same `Float`), `Debug`, the base and hexadecimal formatting, `FromStr`, `ToSci`, and LaTeX, Typst,
and serde output are covered by property tests, chiefly round trips: a printed value, with its
precision, must parse back to the same `Float`.

## Exhaustive generation {#exhaustive-generation}

*This section is not yet written.*

## Random generation {#random-generation}

*This section is not yet written.*

## What is not yet cross-checked {#not-yet-cross-checked}

*This section is not yet written.*
