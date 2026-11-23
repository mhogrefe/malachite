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
| ◦ | Reference column only: the reference implementation is compared with Malachite in the unit tests, but no longer in the property tests, because Azurite checks the function. The reference is kept for the benchmarks. |
| | The oracle does not check this function. |

Azurite's `AzFloat` has an unbounded exponent, while a `Float`'s exponent is bounded, so the
Azurite oracle applies Malachite's documented overflow and underflow rules to Azurite's result
before comparing; that adaptation is the same for every operation and is not counted as ≈. A ✓
in the Reference column can also stand for a second production algorithm rather than a test-only
reference: `mul_rational_prec_round`, `div_rational_prec_round`, and `rational_div_float_prec_round`
each dispatch between a "naive" and a "direct" algorithm by operand size, and the property tests
run both on every input and compare them with each other and with the default, so each algorithm
is tested beyond the inputs that would select it, which the Azurite oracle, seeing only the default,
cannot do. A function with no mark in any column is checked only by Malachite's own unit and
property tests;
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
| [catalans_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/catalans_constant/index.html) | `catalans_constant_prec`, `catalans_constant_prec_round` | ✓ | ✓ |  |
| [cbrt_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/cbrt_2/index.html) | `cbrt_2_prec`, `cbrt_2_prec_round` | ✓ | ✓ |  |
| [champernowne_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/champernowne_constant/index.html) | `champernowne_constant_prec`, `champernowne_constant_prec_round`, `champernowne_constant_base_prec`, `champernowne_constant_base_prec_round` |  |  | ✓ |
| [champernowne_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/champernowne_constant/index.html) | `primitive_float_champernowne_constant_base` |  |  |  |
| [copeland_erdos_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/copeland_erdos_constant/index.html) | `copeland_erdos_constant_prec`, `copeland_erdos_constant_prec_round`, `copeland_erdos_constant_base_prec`, `copeland_erdos_constant_base_prec_round` |  |  | ✓ |
| [copeland_erdos_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/copeland_erdos_constant/index.html) | `primitive_float_copeland_erdos_constant_base` |  |  |  |
| [dottie_number](https://docs.rs/malachite-float/latest/malachite_float/float/constants/dottie_number/index.html) | `dottie_number_prec`, `dottie_number_prec_round` | ✓ |  |  |
| [e](https://docs.rs/malachite-float/latest/malachite_float/float/constants/e/index.html) | `e_prec`, `e_prec_round` | ✓ | ✓ |  |
| [eulers_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/eulers_constant/index.html) | `eulers_constant_prec`, `eulers_constant_prec_round` | ✓ | ✓ |  |
| [gauss_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/gauss_constant/index.html) | `gauss_constant_prec`, `gauss_constant_prec_round` | ✓ |  |  |
| [gelfond_schneider_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/gelfond_schneider_constant/index.html) | `gelfond_schneider_constant_prec`, `gelfond_schneider_constant_prec_round` | ✓ |  |  |
| [gelfonds_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/gelfonds_constant/index.html) | `gelfonds_constant_prec`, `gelfonds_constant_prec_round` | ✓ |  |  |
| [lemniscate_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/lemniscate_constant/index.html) | `lemniscate_constant_prec`, `lemniscate_constant_prec_round` | ✓ |  | ✓ |
| [liouvilles_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/liouvilles_constant/index.html) | `liouvilles_constant_prec`, `liouvilles_constant_prec_round`, `liouvilles_constant_base_prec`, `liouvilles_constant_base_prec_round` |  |  | ✓ |
| [liouvilles_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/liouvilles_constant/index.html) | `primitive_float_liouvilles_constant_base` |  |  |  |
| [ln_10](https://docs.rs/malachite-float/latest/malachite_float/float/constants/ln_10/index.html) | `ln_10_prec`, `ln_10_prec_round` | ✓ | ✓ |  |
| [ln_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/ln_2/index.html) | `ln_2_prec`, `ln_2_prec_round` | ✓ | ✓ |  |
| [log_10_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_10_2/index.html) | `log_10_2_prec`, `log_10_2_prec_round` | ✓ | ✓ |  |
| [log_10_e](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_10_e/index.html) | `log_10_e_prec`, `log_10_e_prec_round` | ✓ |  | ✓ |
| [log_2_10](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_2_10/index.html) | `log_2_10_prec`, `log_2_10_prec_round` | ✓ | ✓ |  |
| [log_2_e](https://docs.rs/malachite-float/latest/malachite_float/float/constants/log_2_e/index.html) | `log_2_e_prec`, `log_2_e_prec_round` | ✓ |  | ✓ |
| [one_over_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/one_over_pi/index.html) | `one_over_pi_prec`, `one_over_pi_prec_round` | ✓ |  | ✓ |
| [one_over_sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/one_over_sqrt_pi/index.html) | `one_over_sqrt_pi_prec`, `one_over_sqrt_pi_prec_round` | ✓ |  | ✓ |
| [one_over_sqrt_tau](https://docs.rs/malachite-float/latest/malachite_float/float/constants/one_over_sqrt_tau/index.html) | `one_over_sqrt_tau_prec`, `one_over_sqrt_tau_prec_round` | ✓ |  | ✓ |
| [phi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/phi/index.html) | `phi_prec`, `phi_prec_round` | ✓ |  |  |
| [pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi/index.html) | `pi_prec`, `pi_prec_round` | ✓ | ✓ |  |
| [pi_over_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_2/index.html) | `pi_over_2_prec`, `pi_over_2_prec_round` | ✓ |  |  |
| [pi_over_3](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_3/index.html) | `pi_over_3_prec`, `pi_over_3_prec_round` | ✓ |  | ✓ |
| [pi_over_4](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_4/index.html) | `pi_over_4_prec`, `pi_over_4_prec_round` | ✓ |  |  |
| [pi_over_6](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_6/index.html) | `pi_over_6_prec`, `pi_over_6_prec_round` | ✓ |  |  |
| [pi_over_8](https://docs.rs/malachite-float/latest/malachite_float/float/constants/pi_over_8/index.html) | `pi_over_8_prec`, `pi_over_8_prec_round` | ✓ |  |  |
| [prime_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/prime_constant/index.html) | `prime_constant_prec`, `prime_constant_prec_round` | ✓ |  |  |
| [prouhet_thue_morse_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/prouhet_thue_morse_constant/index.html) | `prouhet_thue_morse_constant_prec`, `prouhet_thue_morse_constant_prec_round` | ✓ |  | ◦ |
| [ramanujans_constant](https://docs.rs/malachite-float/latest/malachite_float/float/constants/ramanujans_constant/index.html) | `ramanujans_constant_prec`, `ramanujans_constant_prec_round` | ✓ |  |  |
| [sqrt_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_2/index.html) | `sqrt_2_prec`, `sqrt_2_prec_round` | ✓ | ✓ |  |
| [sqrt_2_over_2](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_2_over_2/index.html) | `sqrt_2_over_2_prec`, `sqrt_2_over_2_prec_round` | ✓ | ✓ |  |
| [sqrt_3](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_3/index.html) | `sqrt_3_prec`, `sqrt_3_prec_round` | ✓ | ✓ |  |
| [sqrt_3_over_3](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_3_over_3/index.html) | `sqrt_3_over_3_prec`, `sqrt_3_over_3_prec_round` | ✓ | ✓ |  |
| [sqrt_5](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_5/index.html) | `sqrt_5_prec`, `sqrt_5_prec_round` | ✓ | ✓ |  |
| [sqrt_5_over_5](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_5_over_5/index.html) | `sqrt_5_over_5_prec`, `sqrt_5_over_5_prec_round` | ✓ | ✓ |  |
| [sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/sqrt_pi/index.html) | `sqrt_pi_prec`, `sqrt_pi_prec_round` | ✓ |  | ✓ |
| [tau](https://docs.rs/malachite-float/latest/malachite_float/float/constants/tau/index.html) | `tau_prec`, `tau_prec_round` | ✓ |  |  |
| [two_over_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/two_over_pi/index.html) | `two_over_pi_prec`, `two_over_pi_prec_round` | ✓ |  |  |
| [two_over_sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/float/constants/two_over_sqrt_pi/index.html) | `two_over_sqrt_pi_prec`, `two_over_sqrt_pi_prec_round` | ✓ |  |  |

The fifteen constants that MPFR provides directly or computes as a correctly rounded value of one of
its functions ($$\pi$$, $$e$$, Euler's and Catalan's constants, $$\ln 2$$, $$\ln 10$$, $$\log_{10}
2$$, $$\log_2 10$$, and the square and cube roots) are compared with MPFR at every precision and
rounding mode the tests generate. Twelve more are checked against references: the digit-defined
constants (Champernowne's, Copeland–Erdős, and Liouville's, in any base, and the Prouhet–Thue–Morse
constant, in the unit tests only (◦), since Azurite checks it) against a digit-by-digit expansion
bracketed by exact `Rational`s, and the constants built from others ($$1/\pi$$, $$\pi/3$$,
$$\sqrt{\pi}$$, the lemniscate constant, $$\log_2 e$$, and the like) against a computation that
brackets the inputs at a higher precision and rounds once the bracket decides the result. Every constant, those with no oracle included, is also compared with a
hard-coded value at precisions 1,000 and 10,000, and its rounding `Ordering`s are checked against
its neighbors: the `Floor` and `Ceiling` results must be adjacent, with the exact value between
them. The constants that are $$\pi$$ or $$\tau$$ times a power of 2 ($$\pi/2$$, $$\pi/4$$,
$$\pi/8$$, $$\tau$$) are exact shifts of a checked value. Nine constants are also checked
against Azurite (`AzFloat.sqrt2PrecRound` and its siblings) at every precision and rounding mode the
demos generate, up to precision 10,000: $$\sqrt2$$, $$\sqrt3$$, $$\sqrt5$$, $$\sqrt2/2$$,
$$\sqrt3/3$$, $$\sqrt5/5$$, $$\phi$$, the prime constant, and the Prouhet–Thue–Morse constant. For
$$\phi$$ and the prime constant, Azurite is the only oracle. Six logarithmic constants are checked
against Azurite the same way: $$\ln 2$$, $$\log_2 e$$, $$\ln 10$$, $$\log_{10} e$$,
$$\log_2 10$$, and $$\log_{10} 2$$, whose Azurite values are proven to be correctly rounded; for
$$\log_2 e$$ and $$\log_{10} e$$, which MPFR does not provide, Azurite is the only oracle outside
Malachite. So are $$\pi$$, $$\tau$$, $$e$$, Gauss's constant, the lemniscate constant, and $$\pi/2$$,
$$\pi/3$$, $$\pi/4$$, $$\pi/6$$, and $$\pi/8$$; for $$\tau$$, Gauss's constant, and $$\pi/2$$,
$$\pi/4$$, $$\pi/6$$, and $$\pi/8$$, Azurite is the only oracle outside Malachite. So are $$1/\pi$$,
$$2/\pi$$, $$\sqrt\pi$$, $$1/\sqrt\pi$$, $$2/\sqrt\pi$$, and $$1/\sqrt{2\pi}$$; for $$2/\pi$$ and
$$2/\sqrt\pi$$, Azurite is the only oracle outside Malachite. So are Gelfond's constant $$e^\pi$$,
Ramanujan's constant $$e^{\pi\sqrt{163}}$$, the Gelfond–Schneider constant $$2^{\sqrt2}$$, and
Euler's constant $$\gamma$$; for the first three, Azurite is the only oracle outside Malachite.
So are $$\sqrt[3]{2}$$ and the Dottie number, the solution of $$\cos x = x$$; for the Dottie
number, Azurite is the only oracle outside Malachite. Azurite's Gauss's constant, lemniscate
constant, and these four are proven to be either correctly rounded or `NaN` (if an iteration
limit is reached), and Malachite never returns `NaN` for them, so every agreement is a correctly
rounded value. Catalan's constant is checked against Azurite too, in addition to MPFR; Azurite's
value is proven to be correctly rounded, with no iteration limit.

## Arithmetic {#arithmetic}

From [`malachite_float::float::arithmetic`](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/index.html). The
rows are grouped by theme.

### Addition, subtraction, multiplication, and division {#addition-subtraction-multiplication-and-division}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [add](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add/index.html) | `Add`, `add_prec_round` | ✓ | ✓ | ◦ |
| [add](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add/index.html) | `add_rational_prec_round` | ✓ | ✓ | ◦ |
| [sub](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub/index.html) | `Sub`, `sub_prec_round` | ✓ | ✓ | ◦ |
| [sub](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub/index.html) | `sub_rational_prec_round` | ✓ | ✓ | ◦ |
| [mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul/index.html) | `Mul`, `mul_prec_round` | ✓ | ✓ | ◦ |
| [mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul/index.html) | `mul_rational_prec_round` | ✓ | ✓ | ✓ |
| [div](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/div/index.html) | `Div`, `div_prec_round` | ✓ | ✓ | ◦ |
| [div](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/div/index.html) | `div_rational_prec_round` | ✓ | ✓ | ✓ |
| [div](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/div/index.html) | `rational_div_float_prec_round` | ✓ | ✓ | ✓ |
| [neg](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/neg/index.html) | `Neg` | ✓ | ✓ |  |
| [abs](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/abs/index.html) | `Abs` | ✓ | ✓ |  |
| [abs](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/abs/index.html) | `abs_negative_zero` |  |  |  |
| [reciprocal](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal/index.html) | `Reciprocal`, `reciprocal_prec_round` | ✓ | ✓ | ✓ |
| [square](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/square/index.html) | `Square`, `square_prec_round` | ✓ | ✓ | ◦ |
| [abs_squared](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/abs_squared/index.html) | `AbsSquared` |  |  |  |
| [sum](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sum/index.html) | `Sum`, `sum_prec_round` | ✓ | ✓ |  |
| [sum](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sum/index.html) | `primitive_float_sum` | ≈ |  |  |
| [product](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/product/index.html) | `Product`, `product_prec_round` | ✓ |  |  |
| [product](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/product/index.html) | `primitive_float_product` | ≈ |  |  |
| [dot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/dot/index.html) | `dot_prec_round` |  | ✓ |  |
| [dot](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/dot/index.html) | `primitive_float_dot` |  |  |  |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/index.html) | `AddMul`, `add_mul_prec_round` | ✓ | ✓ | ✓ |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/index.html) | `add_mul_rational_prec_round` | ✓ |  | ✓ |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/index.html) | `primitive_float_add_mul` | ≈ |  |  |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/add_mul/index.html) | `primitive_float_add_mul_rational` | ≈ |  |  |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/index.html) | `SubMul`, `sub_mul_prec_round` | ✓ | ✓ | ✓ |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/index.html) | `sub_mul_rational_prec_round` | ✓ |  | ✓ |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/index.html) | `primitive_float_sub_mul` | ≈ |  |  |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sub_mul/index.html) | `primitive_float_sub_mul_rational` | ≈ |  |  |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/index.html) | `MulAddMul`, `mul_add_mul_prec_round` | ✓ | ✓ | ✓ |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/index.html) | `mul_add_mul_rational_prec_round` | ✓ |  | ✓ |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/index.html) | `primitive_float_mul_add_mul` | ≈ |  |  |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_add_mul/index.html) | `primitive_float_mul_add_mul_rational` | ≈ |  |  |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/index.html) | `MulSubMul`, `mul_sub_mul_prec_round` | ✓ | ✓ | ✓ |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/index.html) | `mul_sub_mul_rational_prec_round` | ✓ |  | ✓ |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/index.html) | `primitive_float_mul_sub_mul` | ≈ |  |  |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/mul_sub_mul/index.html) | `primitive_float_mul_sub_mul_rational` | ≈ |  |  |
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
and its siblings) and MPFR. The four operations and squaring also have references that compute the
exact result as a `Rational` and round it once; since Azurite checks these functions, the
references are compared only in the unit tests (◦) and kept for the benchmarks. The `Rational`
forms of multiplication and division are still compared with their references in the property
tests. MPFR's operations with a rational
operand are exact before their single rounding, so the `Rational` forms of the four operations are
compared with them directly; they are also checked by Azurite (`AzFloat.addRatPrecRound`,
`subRatPrecRound`, `mulRatPrecRound`, `divRatPrecRound`, `ratSubPrecRound`, and
`ratDivPrecRound`), including `Rational + Float`, `Rational - Float`, `Rational * Float`, and
`Rational / Float`. Some of the demos on extreme inputs (those for addition and subtraction, and the
`_round` and `_prec_round` forms of multiplication and division) take a fiftieth to a tenth of a
second per line, so they are checked on 300 lines in each generator mode rather than 10,000. Sums
and products, in all four forms and on extreme inputs, are checked against Azurite's
`sumPrecRound` and `productPrecRound`, and `reciprocal`, in all four forms and on extreme inputs, against
`recipPrecRound`. The `f64` versions of the sum and product, `primitive_float_sum` and
`primitive_float_product`, are checked against Azurite's `sumFloat64` and `productFloat64`, which
are proven to round the exact result once to binary64, subnormals and overflow included; they are
≈ because Azurite gives every exact zero as `0.0`, so for a zero result the oracle checks the sign
against Malachite's rule instead (a sum of zeros all of one sign has that sign, and a zero product
has the sign of the product of the signs). The fused operations `add_mul` and `sub_mul`, in all four forms, by value and by
reference, and on extreme inputs, are checked against Azurite's `addMulPrecRound` and
`subMulPrecRound`, and their `f64` versions `primitive_float_add_mul` and `primitive_float_sub_mul`
against `addMulFloat64` and `subMulFloat64`, proven to be the correctly rounded `fma` and its
subtracting form; those two are ≈ because for an exact zero the oracle checks the sign against
IEEE 754's rule rather than Azurite, which gives `+0.0`. Their `Rational` forms, `add_mul_rational`
and `sub_mul_rational` in all four forms and `primitive_float_add_mul_rational` and
`primitive_float_sub_mul_rational`, are checked the same way against `addMulRatPrecRound`,
`subMulRatPrecRound`, `addMulRatFloat64`, and `subMulRatFloat64`. The sums and differences of two products,
`mul_add_mul` and `mul_sub_mul`, in all four forms, by value and by reference, are checked against
`mulAddMulPrecRound` and `mulSubMulPrecRound`, and their `f64` versions against `mulAddMulFloat64`
and `mulSubMulFloat64`, ≈ for the same reason; their `Rational` forms likewise against
`mulAddMulRatPrecRound`, `mulSubMulRatPrecRound`, `mulAddMulRatFloat64`, and
`mulSubMulRatFloat64`. Sums, dot products, `hypot`, `positive_difference` (MPFR's
`mpfr_dim`), and the fused operations (MPFR's `mpfr_fma` and `mpfr_fmma` families) are checked
against MPFR, and the fused operations' `Rational` forms against exact references. `average`, `abs_squared`, `abs_negative_zero`, the `Rational` form of `positive_difference`, and
the other `primitive_float_*` versions in this section are covered by property tests, which compare the
primitive versions with the `Float` functions at the primitive type's precision and exponent
range.

### Shifts and powers of 2 {#shifts-and-powers-of-2}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [shl](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shl/index.html) | `Shl` | ✓ | ✓ | ◦ |
| [shr](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shr/index.html) | `Shr` | ✓ | ✓ | ◦ |
| [shl_round](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shl_round/index.html) | `ShlRound`, `shl_prec_round` |  | ✓ | ✓ |
| [shr_round](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/shr_round/index.html) | `ShrRound`, `shr_prec_round` |  | ✓ | ✓ |
| [power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/power_of_2/index.html) | `PowerOf2`, `power_of_2_prec_round` | ✓ | ✓ | ◦ |
| [is_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/is_power_of_2/index.html) | `IsPowerOf2` | ✓ |  |  |

Shifts multiply by a power of 2 and change only the exponent; they are checked by Azurite and MPFR,
and against a reference that shifts the exact `Rational` value in the unit tests (◦). `shl_round`
and `shr_round`, which also round to a new precision, are checked against MPFR and similar
references. `power_of_2` is checked by Azurite and MPFR, and against references for every exponent
type in the unit tests (◦); `is_power_of_2` is checked by Azurite.

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
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_prec_round` | ✓ | ✓ |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_then_prec_round` | ✓ | ✓ |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_ties_away` |  |  |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `round_to_integer_ties_away_then_prec_round` |  |  |  |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/round_to_integer/index.html) | `primitive_float_round_to_integer`, `primitive_float_round_to_integer_ties_away` |  |  |  |
| [fractional_part](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/fractional_part/index.html) | `fractional_part_prec_round` |  | ✓ |  |
| [fractional_part](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/fractional_part/index.html) | `integer_and_fractional_parts_prec_round` |  | ✓ |  |
| [fractional_part](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/fractional_part/index.html) | `primitive_float_fractional_part`, `primitive_float_integer_and_fractional_parts` |  |  |  |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/rem/index.html) | `Rem`, `rem_prec_round` | ✓ | ✓ |  |
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
(`round_to_integer_then`), is checked against Azurite's `roundToIntegerPrecRound` and
`roundToIntegerThenPrecRound`, and against MPFR's rounding functions, called directly; the forms
that break ties away from zero are covered by property tests. The fractional part and the split
into integer and fractional parts are checked against MPFR's `mpfr_frac` and `mpfr_modf`. The
truncated remainder is also checked against Azurite's `remPrecRound`. The
remainders are checked against MPFR's truncated and IEEE 754 remainders, and the versions that also
return the low bits of the quotient against `mpfr_fmodquo` and `mpfr_remquo`; the forms with a
`Rational` operand on either side have no MPFR counterpart and are covered by property tests.

### Roots and powers {#roots-and-powers}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `Sqrt`, `sqrt_prec_round` | ✓ | ✓ |  |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `sqrt_rational_prec_round` | ✓ |  | ✓ |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `sqrt_unsigned_prec_round` | ✓ |  |  |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/sqrt/index.html) | `primitive_float_sqrt_rational` |  |  |  |
| [reciprocal_sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal_sqrt/index.html) | `ReciprocalSqrt`, `reciprocal_sqrt_prec_round` | ✓ | ✓ |  |
| [reciprocal_sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal_sqrt/index.html) | `reciprocal_sqrt_rational_prec_round` | ✓ |  | ✓ |
| [reciprocal_sqrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/reciprocal_sqrt/index.html) | `primitive_float_reciprocal_sqrt`, `primitive_float_reciprocal_sqrt_rational` |  |  |  |
| [cbrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cbrt/index.html) | `Cbrt`, `cbrt_prec_round` | ✓ | ✓ |  |
| [cbrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cbrt/index.html) | `cbrt_rational_prec_round` | ✓ |  |  |
| [cbrt](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/cbrt/index.html) | `primitive_float_cbrt`, `primitive_float_cbrt_rational` |  |  |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `Root<i64>`, `root_s_prec_round` |  | ✓ |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `root_s_rational_prec_round` |  |  |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `Root<u64>`, `root_u_prec_round` |  | ✓ |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `root_u_rational_prec_round` |  |  |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/root/index.html) | `primitive_float_root_s`, `primitive_float_root_s_rational`, `primitive_float_root_u`, `primitive_float_root_u_rational` |  |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `Pow`, `pow_prec_round` | ≈ | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_integer_prec_round` | ≈ | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_rational_prec_round` | ≈ |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_s_prec_round` | ≈ | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `pow_u_prec_round` | ≈ | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `powr_prec_round` |  |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `rational_pow_prec_round` | ✓ |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `rational_pow_rational_prec_round` | ✓ |  |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `unsigned_pow_prec_round` | ✓ | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `unsigned_pow_rational_prec_round` | ✓ | ≈ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `unsigned_pow_unsigned_prec_round` | ✓ | ✓ |  |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/float/arithmetic/pow/index.html) | `primitive_float_pow`, `primitive_float_pow_integer`, `primitive_float_pow_rational`, `primitive_float_pow_u`, `primitive_float_rational_pow`, `primitive_float_unsigned_pow` |  |  |  |

The square root, reciprocal square root, cube root, and roots with a `u64` or `i64` index are
checked against MPFR (`mpfr_sqrt`, `mpfr_rec_sqrt`, `mpfr_cbrt`, `mpfr_rootn_ui`, and
`mpfr_rootn_si`), and the square root, reciprocal square root, and cube root also by Azurite.
Their `Rational` forms, and the square root of a `u64`, are checked against Azurite's
`sqrtRatPrecRound`, `rsqrtRatPrecRound`, and `cbrtRatPrecRound`, and the first two against the
generic algorithm that the default falls back to, run on every input. Powers with a `Float`,
`Integer`, `u64`, or `i64` exponent, and the powers of an unsigned base, are checked against MPFR's
`mpfr_pow` and `mpfr_pow_z`. An unsigned base raised to a `Rational` is ≈: MPFR has no such
function, so the oracle brackets the result with MPFR's `exp` and `ln` at increasing precision
until its rounding is decided. The powers of an unsigned base, with a `Float`, `Rational`, or
`u64` exponent, are also checked against Azurite's `powBasePrecRound` and `powBaseRatPrecRound`.
Powers with an `Integer`, `u64`, or `i64` exponent are also checked against Azurite's
`powIntPrecRound`, ≈ because Malachite follows MPFR's $$\mathrm{NaN}^0 = 1$$, where Azurite gives
`NaN`, and gives $$(-0)^n = -\infty$$ for a negative odd $$n$$, where Azurite, with no signed zero,
gives $$+\infty$$. Powers with a `Float` or `Rational` base and a `Float` or `Rational` exponent
are checked against Azurite's `powPrecRound`, `powRatPrecRound`, `ratPowPrecRound`, and
`ratPowRatPrecRound`; those with a `Float` base are ≈, as $$(-0)^y$$ for a negative odd integer
$$y$$ is $$-\infty$$ in Malachite and $$+\infty$$ in Azurite. `powr`, the `Rational` forms of the
roots, and the `primitive_float_*` versions are covered by property tests.

## Exponentials and logarithms {#exponentials-and-logarithms}

From [`malachite_float::float::exponential`](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/index.html).

### Exponentials and compound interest {#exponentials-and-compound-interest}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/exp/index.html) | `Exp`, `exp_prec_round` | ✓ | ✓ |  |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/exp/index.html) | `exp_rational_prec_round` | ✓ | ≈ |  |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/exp/index.html) | `primitive_float_exp`, `primitive_float_exp_rational` |  |  |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/exp_x_minus_1/index.html) | `ExpXMinus1`, `exp_x_minus_1_prec_round` | ≈ | ✓ |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/exp_x_minus_1/index.html) | `exp_x_minus_1_rational_prec_round` | ✓ | ≈ |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/exp_x_minus_1/index.html) | `primitive_float_exp_x_minus_1`, `primitive_float_exp_x_minus_1_rational` |  |  |  |
| [power_of_2_of_float](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_2_of_float/index.html) | `PowerOf2`, `power_of_2_of_float_prec_round` | ✓ | ✓ |  |
| [power_of_2_of_float](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_2_of_float/index.html) | `power_of_2_rational_prec_round` | ✓ | ≈ |  |
| [power_of_2_of_float](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_2_of_float/index.html) | `primitive_float_power_of_2`, `primitive_float_power_of_2_rational` |  |  |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_2_x_minus_1/index.html) | `PowerOf2XMinus1`, `power_of_2_x_minus_1_prec_round` | ≈ | ✓ |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_2_x_minus_1/index.html) | `power_of_2_x_minus_1_rational_prec_round` | ✓ | ≈ |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_2_x_minus_1/index.html) | `primitive_float_power_of_2_x_minus_1`, `primitive_float_power_of_2_x_minus_1_rational` |  |  |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_10/index.html) | `PowerOf10`, `power_of_10_of_float_prec_round` | ✓ | ✓ |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_10/index.html) | `power_of_10_rational_prec_round` | ✓ | ≈ |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_10/index.html) | `primitive_float_power_of_10`, `primitive_float_power_of_10_rational` |  |  |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_10_x_minus_1/index.html) | `PowerOf10XMinus1`, `power_of_10_x_minus_1_prec_round` | ≈ | ✓ |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_10_x_minus_1/index.html) | `power_of_10_x_minus_1_rational_prec_round` | ✓ | ≈ |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/power_of_10_x_minus_1/index.html) | `primitive_float_power_of_10_x_minus_1`, `primitive_float_power_of_10_x_minus_1_rational` |  |  |  |
| [compound](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/compound/index.html) | `Compound`, `compound_prec_round` |  | ✓ |  |
| [compound](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/compound/index.html) | `primitive_float_compound` |  |  |  |

The exponentials $$e^x$$, $$2^x$$, and $$10^x$$, with and without the $$x - 1$$ forms, are checked
against the corresponding MPFR functions, and `compound`, $$(1 + x)^n$$, against
`mpfr_compound_si`. The `Rational` forms are ≈ for the reason given under
[Trigonometric functions](#trigonometric-functions), and the `primitive_float_*` versions are
compared with the `Float` functions. $$e^x$$ and $$e^x - 1$$, and their `Rational` forms, are also
checked against Azurite's `expPrecRound`, `expM1PrecRound`, `expRatPrecRound`, and
`expM1RatPrecRound`, and $$2^x$$ and $$2^x - 1$$, and theirs, against `pow2PrecRound`,
`pow2M1PrecRound`, `pow2RatPrecRound`, and `pow2M1RatPrecRound`, and $$10^x$$ and $$10^x - 1$$,
and theirs, against `pow10PrecRound`, `pow10M1PrecRound`, `pow10RatPrecRound`, and
`pow10M1RatPrecRound`, in Malachite's exponent range; the $$x - 1$$ forms are ≈ there because Azurite has no signed zero.

### Logarithms {#logarithms}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/ln/index.html) | `Ln`, `ln_prec_round` | ✓ | ✓ |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/ln/index.html) | `ln_rational_prec_round` | ✓ |  |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/ln/index.html) | `ln_unsigned_prec_round` | ✓ |  |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/ln/index.html) | `primitive_float_ln`, `primitive_float_ln_rational` |  |  |  |
| [ln_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/ln_1_plus_x/index.html) | `Ln1PlusX`, `ln_1_plus_x_prec_round` | ≈ | ✓ |  |
| [ln_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/ln_1_plus_x/index.html) | `primitive_float_ln_1_plus_x` |  |  |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_2/index.html) | `LogBase2`, `log_base_2_prec_round` | ✓ | ✓ |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_2/index.html) | `log_base_2_rational_prec_round` | ✓ | ≈ |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_2/index.html) | `primitive_float_log_base_2`, `primitive_float_log_base_2_rational` |  |  |  |
| [log_base_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_2_1_plus_x/index.html) | `LogBase2Of1PlusX`, `log_base_2_1_plus_x_prec_round` | ≈ | ✓ |  |
| [log_base_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_2_1_plus_x/index.html) | `primitive_float_log_base_2_1_plus_x` |  |  |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_10/index.html) | `LogBase10`, `log_base_10_prec_round` | ✓ | ✓ |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_10/index.html) | `log_base_10_rational_prec_round` | ✓ | ≈ |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_10/index.html) | `primitive_float_log_base_10`, `primitive_float_log_base_10_rational` |  |  |  |
| [log_base_10_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_10_1_plus_x/index.html) | `LogBase10Of1PlusX`, `log_base_10_1_plus_x_prec_round` | ≈ | ✓ |  |
| [log_base_10_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_10_1_plus_x/index.html) | `primitive_float_log_base_10_1_plus_x` |  |  |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base/index.html) | `LogBase`, `log_base_prec_round` | ✓ | ≈ |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base/index.html) | `log_base_rational_prec_round` | ✓ | ≈ |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base/index.html) | `primitive_float_log_base`, `primitive_float_log_base_rational` |  |  |  |
| [log_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_1_plus_x/index.html) | `LogBaseOf1PlusX`, `log_base_1_plus_x_prec_round` | ≈ | ≈ |  |
| [log_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_1_plus_x/index.html) | `primitive_float_log_base_1_plus_x` |  |  |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_power_of_2/index.html) | `LogBasePowerOf2`, `log_base_power_of_2_prec_round` | ✓ | ≈ |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_power_of_2/index.html) | `log_base_power_of_2_rational_prec_round` | ✓ | ≈ |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_power_of_2/index.html) | `primitive_float_log_base_power_of_2`, `primitive_float_log_base_power_of_2_rational` |  |  |  |
| [log_base_power_of_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_power_of_2_1_plus_x/index.html) | `LogBasePowerOf2Of1PlusX`, `log_base_power_of_2_1_plus_x_prec_round` | ≈ | ≈ |  |
| [log_base_power_of_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_power_of_2_1_plus_x/index.html) | `primitive_float_log_base_power_of_2_1_plus_x` |  |  |  |
| [log_base_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_float_base/index.html) | `LogBase`, `log_base_float_base_prec_round` | ≈ | ≈ |  |
| [log_base_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_float_base/index.html) | `primitive_float_log_base_float_base` |  |  |  |
| [log_base_float_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_float_base_1_plus_x/index.html) | `LogBaseOf1PlusX`, `log_base_float_base_1_plus_x_prec_round` | ≈ | ≈ |  |
| [log_base_float_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_float_base_1_plus_x/index.html) | `primitive_float_log_base_float_base_1_plus_x` |  |  |  |
| [log_base_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_base/index.html) | `LogBase`, `log_base_rational_base_prec_round` | ✓ | ≈ |  |
| [log_base_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_base/index.html) | `primitive_float_log_base_rational_base` |  |  |  |
| [log_base_rational_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_base_1_plus_x/index.html) | `LogBaseOf1PlusX`, `log_base_rational_base_1_plus_x_prec_round` | ≈ | ≈ |  |
| [log_base_rational_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_base_1_plus_x/index.html) | `primitive_float_log_base_rational_base_1_plus_x` |  |  |  |
| [log_base_rational_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_float_base/index.html) | `log_base_rational_float_base_prec_round` | ≈ | ≈ |  |
| [log_base_rational_float_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_float_base/index.html) | `primitive_float_log_base_rational_float_base` |  |  |  |
| [log_base_rational_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_rational_base/index.html) | `log_base_rational_rational_base_prec_round` | ✓ | ≈ |  |
| [log_base_rational_rational_base](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/log_base_rational_rational_base/index.html) | `primitive_float_log_base_rational_rational_base` |  |  |  |

The logarithms to bases 2, $$e$$, and 10, with and without the $$1 + x$$ forms, are checked against
the corresponding MPFR functions, and the natural logarithm and its $$1 + x$$ form also against
Azurite's `lnPrecRound` and `lnP1PrecRound`, and the base-2 ones against `log2PrecRound`,
`log2RatPrecRound`, and `log2P1PrecRound`, and the base-10 ones against `log10PrecRound`,
`log10RatPrecRound`, and `log10P1PrecRound`; the logarithms to a power of 2 are checked against
Azurite's `logBasePow2PrecRound`, `logBasePow2RatPrecRound`, and `logBasePow2P1PrecRound`, and
the logarithms to a `u64` base against `logBasePrecRound`, `logBaseRatPrecRound`, and
`logBaseP1PrecRound`, and those to a `Float` base against `logBaseFloatPrecRound`,
`logBaseFloatP1PrecRound`, and `logBaseRatFloatPrecRound`, and those to a `Rational` base against
`logBaseRatBasePrecRound`, `logBaseRatBaseP1PrecRound`, and `logBaseRatRatPrecRound`. The
$$1 + x$$ forms, and the
`Float`-base forms, which give $$\log_b 1$$ the sign of $$1 / \ln b$$, are ≈ because Azurite has
no signed zero. The logarithms to other bases (a `u64`, a power of 2, a `Float`,
or a `Rational`) are ≈: MPFR has no such functions, so the oracle brackets the quotient of two MPFR
logarithms at increasing precision until its rounding is decided, and recognizes the exactly
representable results directly. The `Rational` forms are ≈ for the reason given under
[Trigonometric functions](#trigonometric-functions); `ln_rational` and `ln_unsigned` are checked against
Azurite's `lnRatPrecRound`, and the `primitive_float_*` versions are compared with the `Float` functions.

### The Lambert W, tree, and Wright omega functions {#the-lambert-w-function}

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [lambert_w0](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/lambert_w0/index.html) | `LambertW0`, `lambert_w0_prec_round` | ≈ |  | ✓ |
| [lambert_w0](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/lambert_w0/index.html) | `lambert_w0_rational_prec_round` | ✓ |  | ✓ |
| [lambert_w0](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/lambert_w0/index.html) | `primitive_float_lambert_w0`, `primitive_float_lambert_w0_rational` |  |  |  |
| [lambert_w_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/lambert_w_minus_1/index.html) | `LambertWMinus1`, `lambert_w_minus_1_prec_round` | ✓ |  | ✓ |
| [lambert_w_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/lambert_w_minus_1/index.html) | `lambert_w_minus_1_rational_prec_round` | ✓ |  | ✓ |
| [lambert_w_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/lambert_w_minus_1/index.html) | `primitive_float_lambert_w_minus_1`, `primitive_float_lambert_w_minus_1_rational` |  |  |  |
| [tree_t0](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/tree_t0/index.html) | `TreeT0`, `tree_t0_prec_round` | ≈ |  | ✓ |
| [tree_t0](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/tree_t0/index.html) | `tree_t0_rational_prec_round` | ✓ |  | ✓ |
| [tree_t0](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/tree_t0/index.html) | `primitive_float_tree_t0`, `primitive_float_tree_t0_rational` |  |  |  |
| [tree_t_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/tree_t_minus_1/index.html) | `TreeTMinus1`, `tree_t_minus_1_prec_round` | ✓ |  | ✓ |
| [tree_t_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/tree_t_minus_1/index.html) | `tree_t_minus_1_rational_prec_round` | ✓ |  | ✓ |
| [tree_t_minus_1](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/tree_t_minus_1/index.html) | `primitive_float_tree_t_minus_1`, `primitive_float_tree_t_minus_1_rational` |  |  |  |
| [wright_omega](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/wright_omega/index.html) | `WrightOmega`, `wright_omega_prec_round` | ≈ |  | ✓ |
| [wright_omega](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/wright_omega/index.html) | `wright_omega_rational_prec_round` | ✓ |  | ✓ |
| [wright_omega](https://docs.rs/malachite-float/latest/malachite_float/float/exponential/wright_omega/index.html) | `primitive_float_wright_omega`, `primitive_float_wright_omega_rational` |  |  |  |

The two real branches of the Lambert $$W$$ function, $$W_0$$ and $$W_{-1}$$, are ports of Azurite's
`lambertW0PrecRound` and `lambertWm1PrecRound` and their `Rat` forms, and are checked against
them; $$W_0$$ is ≈ because Azurite has no signed zero, so the sign of $$W_0(\pm 0.0)$$ is not
compared. MPFR has no Lambert $$W$$ function. Since the oracle and the port share an algorithm, the
property tests also apply a check that does not depend on it, marked in the Reference column: a
result $$w$$ is certified to be correctly rounded by deciding the sign of $$t e^t - x$$, with
directed rounding at increasing precision, at the neighbors $$t$$ of $$w$$ that determine the
rounding. The `primitive_float_*` versions are compared with the `Float` functions.

The two real branches $$T_0$$ and $$T_{-1}$$ of the tree function $$T(x) = -W(-x)$$ are the Lambert
$$W$$ functions of the negated argument, rounded in the opposite direction and negated, as
Azurite's `treeT0PrecRound` and `treeTm1PrecRound` and their `Rat` forms are, and are checked
against them; $$T_0$$ is ≈ for the sign of $$T_0(\pm 0.0)$$. Their mark in the Reference column
is for the same certificate, applied to $$W(-x) = -T(x)$$.

The Wright omega function $$\omega(x) = W_0(e^x)$$ is a port of Azurite's `wrightOmegaPrecRound` and
`wrightOmegaRatPrecRound`, with one path added for a very negative argument, and is checked against
them. The `Float` row is ≈ because the check stops short of the extreme exponents: for an argument
at or below about $$-2^{31}$$ Malachite returns the underflow result directly, and Azurite takes
too long there to serve as an oracle. The mark in the Reference column is for $$W_0(e^x)$$ computed from
`exp` and `lambert_w0`, both rounded down and up, at a working precision that doubles until the two
results round alike, for arguments below 256 in absolute value.

## Elliptic integrals {#elliptic-integrals}

From [`malachite_float::float::elliptic`](https://docs.rs/malachite-float/latest/malachite_float/float/elliptic/index.html).
So far the module holds only the arithmetic-geometric mean, the iteration that the elliptic
integrals are computed with; the integrals themselves will join it.

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/float/elliptic/agm/index.html) | `Agm`, `agm_prec_round` | ≈ | ✓ |  |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/float/elliptic/agm/index.html) | `agm_rational_prec_round` | ≈ |  |  |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/float/elliptic/agm/index.html) | `Agm<Rational>`, `agm_float_rational_prec_round` | ≈ |  |  |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/float/elliptic/agm/index.html) | `primitive_float_agm`, `primitive_float_agm_rational` |  |  |  |

The arithmetic-geometric mean, which the logarithms are computed from at high precision, is
checked against `mpfr_agm` and Azurite's `agmPrecRound`; the forms with two `Rational`s and with a
`Float` and a `Rational` against Azurite's `agmRatPrecRound` and `agmFloatRatPrecRound`, the latter
also against the two-`Rational` form in the property tests. All three Azurite checks are ≈: when
one argument is zero and the other negative, Malachite gives `0.0`, as MPFR does, where Azurite
gives `NaN`. The `primitive_float_*` versions are covered by property tests.

## Trigonometric functions {#trigonometric-functions}

From [`malachite_float::float::trigonometric`](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/index.html).

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `Sin`, `sin_prec_round` | ≈ | ✓ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `sin_pi_prec_round` | ≈ | ✓ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `sin_pi_rational_prec_round` | ≈ |  |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `sin_rational_prec_round` | ✓ | ≈ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `sin_with_period_prec_round` | ≈ | ✓ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `sin_with_period_rational_prec_round` | ≈ | ≈ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `primitive_float_sin`, `primitive_float_sin_rational`, `primitive_float_sin_with_period`, `primitive_float_sin_with_period_rational` |  | ≈ |  |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin/index.html) | `primitive_float_sin_pi`, `primitive_float_sin_pi_rational` |  |  |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `Cos`, `cos_prec_round` | ✓ | ✓ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `cos_pi_prec_round` | ✓ | ✓ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `cos_pi_rational_prec_round` | ✓ |  |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `cos_rational_prec_round` | ✓ | ≈ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `cos_with_period_prec_round` | ✓ | ✓ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `cos_with_period_rational_prec_round` | ✓ | ≈ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `primitive_float_cos`, `primitive_float_cos_rational`, `primitive_float_cos_with_period`, `primitive_float_cos_with_period_rational` |  | ≈ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cos/index.html) | `primitive_float_cos_pi`, `primitive_float_cos_pi_rational` |  |  |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `Tan`, `tan_prec_round` | ≈ | ✓ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `tan_pi_prec_round` | ≈ | ✓ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `tan_pi_rational_prec_round` | ≈ |  |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `tan_rational_prec_round` | ✓ | ≈ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `tan_with_period_prec_round` | ≈ | ✓ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `tan_with_period_rational_prec_round` | ≈ | ≈ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `primitive_float_tan`, `primitive_float_tan_rational`, `primitive_float_tan_with_period`, `primitive_float_tan_with_period_rational` |  | ≈ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/tan/index.html) | `primitive_float_tan_pi`, `primitive_float_tan_pi_rational` |  |  |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `Sec`, `sec_prec_round` | ✓ | ✓ |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `sec_pi_prec_round` | ✓ |  | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `sec_pi_rational_prec_round` | ✓ |  |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `sec_rational_prec_round` | ✓ | ≈ |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `sec_with_period_prec_round` | ✓ |  | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `sec_with_period_rational_prec_round` | ✓ |  | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `primitive_float_sec`, `primitive_float_sec_rational` |  | ≈ |  |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sec/index.html) | `primitive_float_sec_pi`, `primitive_float_sec_pi_rational`, `primitive_float_sec_with_period`, `primitive_float_sec_with_period_rational` |  |  |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `Csc`, `csc_prec_round` | ≈ | ✓ |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `csc_pi_prec_round` | ≈ |  | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `csc_pi_rational_prec_round` | ✓ |  |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `csc_rational_prec_round` | ✓ | ≈ |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `csc_with_period_prec_round` | ≈ |  | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `csc_with_period_rational_prec_round` | ✓ |  | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `primitive_float_csc`, `primitive_float_csc_rational` |  | ≈ |  |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/csc/index.html) | `primitive_float_csc_pi`, `primitive_float_csc_pi_rational`, `primitive_float_csc_with_period`, `primitive_float_csc_with_period_rational` |  |  |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `Cot`, `cot_prec_round` | ≈ | ✓ |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `cot_pi_prec_round` | ≈ |  | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `cot_pi_rational_prec_round` | ≈ |  |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `cot_rational_prec_round` | ✓ | ≈ |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `cot_with_period_prec_round` | ≈ |  | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `cot_with_period_rational_prec_round` | ≈ |  | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `primitive_float_cot`, `primitive_float_cot_rational` |  | ≈ |  |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/cot/index.html) | `primitive_float_cot_pi`, `primitive_float_cot_pi_rational`, `primitive_float_cot_with_period`, `primitive_float_cot_with_period_rational` |  |  |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin_cos/index.html) | `SinCos`, `sin_cos_prec_round` | ≈ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin_cos/index.html) | `sin_cos_pi_prec_round` | ≈ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin_cos/index.html) | `sin_cos_pi_rational_prec_round` | ≈ | ≈ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin_cos/index.html) | `sin_cos_rational_prec_round` | ✓ | ≈ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin_cos/index.html) | `sin_cos_with_period_prec_round` | ≈ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin_cos/index.html) | `sin_cos_with_period_rational_prec_round` | ≈ | ≈ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/sin_cos/index.html) | `primitive_float_sin_cos`, `primitive_float_sin_cos_pi`, `primitive_float_sin_cos_pi_rational`, `primitive_float_sin_cos_rational`, `primitive_float_sin_cos_with_period`, `primitive_float_sin_cos_with_period_rational` |  |  |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asin/index.html) | `Asin`, `asin_prec_round` | ≈ | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asin/index.html) | `asin_pi_prec_round` | ≈ | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asin/index.html) | `asin_pi_rational_prec_round` | ✓ | ≈ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asin/index.html) | `asin_rational_prec_round` | ✓ | ≈ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asin/index.html) | `asin_with_period_prec_round` | ≈ | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asin/index.html) | `asin_with_period_rational_prec_round` | ≈ | ≈ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asin/index.html) | `primitive_float_asin`, `primitive_float_asin_pi`, `primitive_float_asin_pi_rational`, `primitive_float_asin_rational`, `primitive_float_asin_with_period`, `primitive_float_asin_with_period_rational` |  |  |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acos/index.html) | `Acos`, `acos_prec_round` | ✓ | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acos/index.html) | `acos_pi_prec_round` | ✓ | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acos/index.html) | `acos_pi_rational_prec_round` | ✓ | ≈ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acos/index.html) | `acos_rational_prec_round` | ✓ | ≈ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acos/index.html) | `acos_with_period_prec_round` | ✓ | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acos/index.html) | `acos_with_period_rational_prec_round` | ✓ | ≈ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acos/index.html) | `primitive_float_acos`, `primitive_float_acos_pi`, `primitive_float_acos_pi_rational`, `primitive_float_acos_rational`, `primitive_float_acos_with_period`, `primitive_float_acos_with_period_rational` |  |  |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `Atan`, `atan_prec_round` | ≈ | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `atan_pi_prec_round` | ≈ | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `atan_pi_rational_prec_round` | ✓ | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `atan_rational_prec_round` | ✓ | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `atan_with_period_prec_round` | ≈ | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `atan_with_period_rational_prec_round` | ≈ | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `primitive_float_atan`, `primitive_float_atan_rational` |  | ≈ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan/index.html) | `primitive_float_atan_pi`, `primitive_float_atan_pi_rational`, `primitive_float_atan_with_period`, `primitive_float_atan_with_period_rational` |  |  |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan2/index.html) | `Atan2`, `atan2_prec_round` | ≈ | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan2/index.html) | `atan2_pi_prec_round` | ≈ | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan2/index.html) | `atan2_pi_rational_prec_round` | ✓ |  |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan2/index.html) | `atan2_rational_prec_round` | ✓ | ≈ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan2/index.html) | `atan2_with_period_prec_round` | ≈ | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan2/index.html) | `atan2_with_period_rational_prec_round` | ✓ |  |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/atan2/index.html) | `primitive_float_atan2`, `primitive_float_atan2_pi`, `primitive_float_atan2_pi_rational`, `primitive_float_atan2_rational`, `primitive_float_atan2_with_period`, `primitive_float_atan2_with_period_rational` |  |  |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asec/index.html) | `Asec`, `asec_prec_round` | ✓ | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asec/index.html) | `asec_pi_prec_round` | ✓ | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asec/index.html) | `asec_pi_rational_prec_round` | ✓ | ≈ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asec/index.html) | `asec_rational_prec_round` | ✓ | ≈ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asec/index.html) | `asec_with_period_prec_round` | ✓ | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asec/index.html) | `asec_with_period_rational_prec_round` | ✓ | ≈ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/asec/index.html) | `primitive_float_asec`, `primitive_float_asec_pi`, `primitive_float_asec_pi_rational`, `primitive_float_asec_rational`, `primitive_float_asec_with_period`, `primitive_float_asec_with_period_rational` |  |  |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acsc/index.html) | `Acsc`, `acsc_prec_round` | ≈ | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acsc/index.html) | `acsc_pi_prec_round` | ≈ | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acsc/index.html) | `acsc_pi_rational_prec_round` | ✓ | ≈ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acsc/index.html) | `acsc_rational_prec_round` | ✓ | ≈ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acsc/index.html) | `acsc_with_period_prec_round` | ≈ | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acsc/index.html) | `acsc_with_period_rational_prec_round` | ≈ | ≈ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acsc/index.html) | `primitive_float_acsc`, `primitive_float_acsc_pi`, `primitive_float_acsc_pi_rational`, `primitive_float_acsc_rational`, `primitive_float_acsc_with_period`, `primitive_float_acsc_with_period_rational` |  |  |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acot/index.html) | `Acot`, `acot_prec_round` | ≈ | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acot/index.html) | `acot_pi_prec_round` | ≈ | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acot/index.html) | `acot_pi_rational_prec_round` | ✓ | ≈ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acot/index.html) | `acot_rational_prec_round` | ✓ | ≈ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acot/index.html) | `acot_with_period_prec_round` | ≈ | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acot/index.html) | `acot_with_period_rational_prec_round` | ≈ | ≈ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/float/trigonometric/acot/index.html) | `primitive_float_acot`, `primitive_float_acot_pi`, `primitive_float_acot_pi_rational`, `primitive_float_acot_rational`, `primitive_float_acot_with_period`, `primitive_float_acot_with_period_rational` |  |  |  |

Each trigonometric function comes in several forms: of a `Float` in radians; of a `Rational`; of
a multiple of $$\pi$$ (`sin_pi(x)` is $$\sin \pi x$$); and with a period (`sin_with_period(x, u)`
is $$\sin 2\pi x / u$$ for an integer $$u$$), the last two each with a `Float` or a `Rational`
argument. The `Float` forms are checked against MPFR directly, through `mpfr_sin` and its siblings,
the `mpfr_sinpi` family, and the `mpfr_sinu` family, which takes the same integer period. The
`_pi` and `_with_period` forms of `sec`, `csc`, and `cot`, which MPFR lacks, are checked against
references that bracket the reciprocal of a wider-precision sine or cosine and round once both ends
agree; on the rare inputs where the bracket does not settle the rounding, those references decline
to answer and the input is passed over.

The six `cos` functions of a `Float` or a `Rational` are also ✓ against Azurite, which
evaluates each `Rational` form exactly, and so is `sin_rational_prec_round`. The other five `sin`
functions are ≈ only for the sign of a zero result: Malachite, following MPFR, gives an exact zero
the sign of the argument, and Azurite has no signed zero. Azurite reduces the `_pi` and
`_with_period` forms exactly, as fractions of a period, so they need no approximation of $$\pi$$
even for huge arguments. The six `sin_cos` functions, which give the sine and cosine together, are
checked against Azurite's `sinCos` functions in the same way, and are ≈ or ✓ as the `sin` function
of the same argument is. So are the six `tan` functions, against Azurite's `tan` functions;
at a pole both give the infinity with the sign of the sine. The six `sec` functions are ✓
against Azurite's `sec` functions: the secant is never zero, and at a pole both give $$+\infty$$.
The six `csc` functions are checked against Azurite's `csc` functions; those of a `Float` are ≈
because Malachite's $$\csc(-0)$$ is $$-\infty$$, the reciprocal of the sine's $$-0$$, where
Azurite, with no signed zero, gives $$+\infty$$, so the oracle negates Azurite's infinity for a
$$-0$$ argument. The `Rational` forms are ✓. The six `cot` functions are checked against Azurite's
`cot` functions with the same adaptation; all but `cot_rational_prec_round` are ≈, since the
cotangent's zeros, at odd multiples of a quarter turn, are signed in Malachite, as the sine's are.
The six `atan` functions are checked against Azurite's `atan` functions. Those of a `Float` are ≈
for the sign of a zero result, $$\arctan(-0) = -0$$, and so is `atan_with_period_rational_prec_round`,
whose result for a period of 0 is a zero with the sign of the argument; the other two `Rational`
forms are ✓. The six `atan2` functions are checked against Azurite's `atan2` functions. Malachite
follows IEEE 754 for signed zeros, $$\operatorname{atan2}(\pm 0, x) = \pm\pi$$ for $$x$$ negative
or $$-0$$, while Azurite's zero acts as $$+0$$, so for a zero $$y$$ the oracle replaces an $$x$$ of
$$-0$$ by $$-1$$, which has the same angle, and negates the result for a $$y$$ of $$-0$$; the forms
of a `Float` are ≈, and the `Rational` forms ✓. The six `asin` functions are checked against
Azurite's `asin` functions, with the marks of the `atan` functions, for the same reasons. The six `acos` functions are ✓ against
Azurite's: the arccosine is never negative, and its zero is $$+0$$ on both sides. So are the six `asec` functions, for the same
reason. The six `acsc` functions have the marks of the `asin`
functions: Malachite's $$\operatorname{arccsc}(-\infty)$$ is $$-0$$, and so is the result of
`acsc_with_period_rational_prec_round` for a negative argument and a period of 0. The six `acot` functions are
checked against Azurite's odd arccotangent: Malachite's $$\operatorname{arccot}(-0)$$ is
$$-\pi/2$$ (and the corresponding negative value for the other forms), the negative side of the
jump at zero, where Azurite, with one zero, gives the positive side, so for an argument of $$-0$$
the oracle negates Azurite's result, computed in the negated rounding mode; Malachite's
$$\operatorname{arccot}(\pm\infty)$$ is $$\pm0$$, and a period of 0 gives a zero with the sign
of the argument. The forms of a `Float` and `acot_with_period_rational_prec_round` are ≈, and the
other `Rational` forms ✓. The plain forms need $$\pi$$ to about as many bits as the argument's
exponent, so their extreme demos, whose exponents approach $$2^{28}$$, are not run against Azurite.

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

## Hyperbolic functions {#hyperbolic-functions}

From [`malachite_float::float::hyperbolic`](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/index.html).

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sinh/index.html) | `Sinh`, `sinh_prec_round` | ≈ | ✓ |  |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sinh/index.html) | `sinh_rational_prec_round` | ✓ | ≈ |  |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sinh/index.html) | `primitive_float_sinh`, `primitive_float_sinh_rational` |  | ≈ |  |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/cosh/index.html) | `Cosh`, `cosh_prec_round` | ✓ | ✓ |  |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/cosh/index.html) | `cosh_rational_prec_round` | ✓ | ≈ |  |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/cosh/index.html) | `primitive_float_cosh`, `primitive_float_cosh_rational` |  | ≈ |  |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/tanh/index.html) | `Tanh`, `tanh_prec_round` | ≈ | ✓ |  |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/tanh/index.html) | `tanh_rational_prec_round` | ✓ | ≈ |  |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/tanh/index.html) | `primitive_float_tanh`, `primitive_float_tanh_rational` |  | ≈ |  |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sech/index.html) | `Sech`, `sech_prec_round` | ✓ | ✓ |  |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sech/index.html) | `sech_rational_prec_round` | ✓ | ≈ |  |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sech/index.html) | `primitive_float_sech`, `primitive_float_sech_rational` |  | ≈ |  |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/csch/index.html) | `Csch`, `csch_prec_round` | ≈ | ✓ |  |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/csch/index.html) | `csch_rational_prec_round` | ≈ | ≈ |  |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/csch/index.html) | `primitive_float_csch`, `primitive_float_csch_rational` |  | ≈ |  |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/coth/index.html) | `Coth`, `coth_prec_round` | ≈ | ✓ |  |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/coth/index.html) | `coth_rational_prec_round` | ✓ | ≈ |  |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/coth/index.html) | `primitive_float_coth`, `primitive_float_coth_rational` |  | ≈ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sinh_cosh/index.html) | `SinhCosh`, `sinh_cosh_prec_round` | ≈ | ✓ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sinh_cosh/index.html) | `sinh_cosh_rational_prec_round` | ✓ | ≈ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/sinh_cosh/index.html) | `primitive_float_sinh_cosh`, `primitive_float_sinh_cosh_rational` |  |  |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asinh/index.html) | `Asinh`, `asinh_prec_round` | ≈ | ✓ |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asinh/index.html) | `primitive_float_asinh` |  | ≈ |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asinh/index.html) | `asinh_rational_prec_round` | ✓ | ≈ |  |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asinh/index.html) | `primitive_float_asinh_rational` |  | ≈ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acosh/index.html) | `Acosh`, `acosh_prec_round` | ✓ | ✓ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acosh/index.html) | `primitive_float_acosh` |  | ≈ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acosh/index.html) | `acosh_rational_prec_round` | ✓ | ≈ |  |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acosh/index.html) | `primitive_float_acosh_rational` |  | ≈ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/atanh/index.html) | `Atanh`, `atanh_prec_round` | ≈ | ✓ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/atanh/index.html) | `primitive_float_atanh` |  | ≈ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/atanh/index.html) | `atanh_rational_prec_round` | ✓ | ≈ |  |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/atanh/index.html) | `primitive_float_atanh_rational` |  | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asech/index.html) | `Asech`, `asech_prec_round` | ✓ | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asech/index.html) | `primitive_float_asech` |  | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asech/index.html) | `asech_rational_prec_round` | ✓ | ≈ |  |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/asech/index.html) | `primitive_float_asech_rational` |  | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acsch/index.html) | `Acsch`, `acsch_prec_round` | ≈ | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acsch/index.html) | `primitive_float_acsch` |  | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acsch/index.html) | `acsch_rational_prec_round` | ✓ | ≈ |  |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acsch/index.html) | `primitive_float_acsch_rational` |  | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acoth/index.html) | `Acoth`, `acoth_prec_round` | ≈ | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acoth/index.html) | `primitive_float_acoth` |  | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acoth/index.html) | `acoth_rational_prec_round` | ✓ | ≈ |  |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/acoth/index.html) | `primitive_float_acoth_rational` |  | ≈ |  |
| [gudermannian](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/gudermannian/index.html) | `Gudermannian`, `gudermannian_prec_round` | ≈ |  | ✓ |
| [gudermannian](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/gudermannian/index.html) | `gudermannian_rational_prec_round` | ✓ |  | ✓ |
| [gudermannian](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/gudermannian/index.html) | `primitive_float_gudermannian`, `primitive_float_gudermannian_rational` |  |  |  |
| [inverse_gudermannian](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/inverse_gudermannian/index.html) | `InverseGudermannian`, `inverse_gudermannian_prec_round` | ≈ |  | ✓ |
| [inverse_gudermannian](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/inverse_gudermannian/index.html) | `inverse_gudermannian_rational_prec_round` | ✓ |  | ✓ |
| [inverse_gudermannian](https://docs.rs/malachite-float/latest/malachite_float/float/hyperbolic/inverse_gudermannian/index.html) | `primitive_float_inverse_gudermannian`, `primitive_float_inverse_gudermannian_rational` |  |  |  |

The hyperbolic functions, their reciprocals, `sinh_cosh`, and the inverse hyperbolic sine, cosine,
and tangent of a `Float` are checked against MPFR directly. The `Rational` forms are ≈, for the
reason given under [Trigonometric functions](#trigonometric-functions). For the inverse hyperbolic
cosine of a `Rational` near 1, the oracle adds as many bits as the magnitude of the exponent of
$$x - 1$$, since a relative error $$e$$ in the input moves the result by about $$e/(2(x-1))$$; for the
inverse hyperbolic tangent of a `Rational` near $$\pm1$$ it adds as many as the magnitude of the
exponent of $$1 - |x|$$, for the same reason. The twelve hyperbolic and inverse hyperbolic functions
and `sinh_cosh`, of a `Float` and of a `Rational`, are also checked against Azurite's, which is the
only direct oracle for `asech`, `acsch`, and `acoth`. Against Azurite, the `Float` forms of the odd
functions are ≈: Malachite, following MPFR, gives $$\sinh(-0.0) = \tanh(-0.0) =
\operatorname{asinh}(-0.0) = \operatorname{atanh}(-0.0) = -0.0$$ and $$\operatorname{csch}(-\infty)
= \operatorname{acsch}(-\infty) = \operatorname{acoth}(-\infty) = -0.0$$, where Azurite's zero has
no sign, and $$\operatorname{csch}(-0.0) = \coth(-0.0) = \operatorname{acsch}(-0.0) = -\infty$$,
where Azurite's single zero gives $$+\infty$$, so for an argument of $$-0.0$$ the oracle negates an
infinite Azurite result. `csch_rational_prec_round` is ≈ too, since the hyperbolic cosecant of a
large negative `Rational` underflows to $$-0.0$$; the `cosh`, `sech`, `acosh`, and `asech` forms and
the other `Rational` forms are ✓. The extreme demos of `sinh`, `cosh`, `sech`, and `csch`, whose
exponents approach $$2^{28}$$, are not run against Azurite: where a `Float` overflows or underflows,
Azurite, with an unbounded exponent, computes the result, which takes $$\log_2 e$$ to about as many
bits as the argument's exponent.

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

The Gudermannian function is a port of Azurite's `gdPrecRound` and `gdRatPrecRound`, and is checked
against them; it is ≈ because Azurite has no signed zero, so the sign of
$$\operatorname{gd}(\pm 0.0)$$ is not compared. MPFR has no Gudermannian function. The mark in the
Reference column is for a computation that shares none of the port's paths: the property tests
evaluate $$\arctan(\sinh x)$$ with the hyperbolic sine and the arctangent rounded down and up, at
a working precision that doubles until the two results round alike, for arguments with exponents
between $$-1000$$ and $$8$$ (the time it takes grows with $$|x|$$). The `primitive_float_*` versions
are compared with the `Float` functions.

The inverse Gudermannian function is likewise a port of Azurite's `gdInvPrecRound` and
`gdInvRatPrecRound`, checked against them, and ≈ for the sign of
$$\operatorname{gd}^{-1}(\pm 0.0)$$. Its reference is $$\operatorname{atanh}(\sin x)$$, with the
sine and the inverse hyperbolic tangent rounded down and up at a working precision that doubles
until the two results round alike.

## The gamma function {#the-gamma-function}

From [`malachite_float::float::gamma`](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/index.html).

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/gamma/index.html) | `Gamma`, `gamma_prec_round` | ≈ | ✓ |  |
| [gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/gamma/index.html) | `gamma_rational_prec_round` | ≈ | ≈ |  |
| [gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/gamma/index.html) | `primitive_float_gamma`, `primitive_float_gamma_rational` |  |  |  |
| [reciprocal_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/reciprocal_gamma/index.html) | `ReciprocalGamma`, `reciprocal_gamma_prec_round` | ≈ |  | ✓ |
| [reciprocal_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/reciprocal_gamma/index.html) | `reciprocal_gamma_rational_prec_round` | ✓ |  | ✓ |
| [reciprocal_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/reciprocal_gamma/index.html) | `primitive_float_reciprocal_gamma`, `primitive_float_reciprocal_gamma_rational` |  |  |  |
| [ln_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/ln_gamma/index.html) | `LnGamma`, `ln_gamma_prec_round` | ✓ | ≈ |  |
| [ln_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/ln_gamma/index.html) | `ln_gamma_rational_prec_round` | ✓ | ≈ |  |
| [ln_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/ln_gamma/index.html) | `primitive_float_ln_gamma`, `primitive_float_ln_gamma_rational` |  |  |  |
| [ln_abs_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/ln_abs_gamma/index.html) | `LnAbsGamma`, `ln_abs_gamma_prec_round` | ≈ | ✓ |  |
| [ln_abs_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/ln_abs_gamma/index.html) | `ln_abs_gamma_rational_prec_round` | ✓ | ≈ |  |
| [ln_abs_gamma](https://docs.rs/malachite-float/latest/malachite_float/float/gamma/ln_abs_gamma/index.html) | `primitive_float_ln_abs_gamma`, `primitive_float_ln_abs_gamma_rational` |  |  |  |

The gamma function and its reciprocal are ports of Azurite's `gammaPrecRound`,
`reciprocalGammaPrecRound`, and their `Rat` forms, and are checked against them. The `Float`
forms are ≈ because Azurite has no signed zero: Malachite, following MPFR, gives $$\Gamma(-0.0)
= -\infty$$, which the oracle obtains by negating Azurite's $$+\infty$$, and the sign of
$$1/\Gamma(\pm 0.0) = \pm 0.0$$ is not compared. `gamma_rational_prec_round` is ≈ too: the gamma
function of a large negative argument that is not an integer underflows, to $$-0.0$$ when its
value is negative, and the sign of that zero is not compared either.
`reciprocal_gamma_rational_prec_round` is ✓. Azurite's functions have an iteration limit and
return `NaN` when it is reached; that never happened in these runs. The demos with extreme
exponents are not run against Azurite, which computes the value of the gamma function even where
it is far outside the range of a `Float`, while Malachite decides overflow and underflow from
the size of the argument.

The logarithm of the gamma function and the logarithm of its absolute value are ports of Azurite's
`lnGammaPrecRound`, `lnAbsGammaPrecRound`, and their `Rat` forms, and are checked against them.
`ln_abs_gamma_prec_round` is ≈ because Azurite has no signed zero: Malachite, following MPFR, gives
the sign `false` at $$-0.0$$, which the oracle expects there. The demo with extreme exponents is run
against Azurite for `ln_gamma` but not for `ln_abs_gamma`: Azurite's sine has absolute accuracy, so
its `lnAbsGamma` takes about $$|e|$$ bits of working precision for a negative argument near $$-2^e$$
with $$e$$ far below 0.
In the property tests `ln_abs_gamma_prec_round` is compared with MPFR's `mpfr_lgamma`, value, sign,
and ternary, for every argument and in every rounding mode, and `ln_gamma_prec_round` with
`mpfr_lngamma` for the arguments that are not negative; that one is ≈ because `mpfr_lngamma` is
defined for some negative arguments, and at $$-\infty$$, where Malachite gives `NaN`. The `Rational`
forms are ≈ against MPFR as the gamma function's is: the rational is first rounded to a `Float` with
256 extra bits, and the comparison is made for arguments below $$2^{16}$$ that are not within
$$2^{-64}$$ of a zero or, if negative, of an integer.

The gamma function of a `Float` is also checked against MPFR's `mpfr_gamma` in the property tests,
for arguments below $$2^{16}$$ in absolute value (MPFR is slow beyond that), in every rounding mode.
The `Rational` form is ≈ against MPFR: the rational is first rounded to a `Float` with 256 extra
bits, and the comparison is made for arguments below $$2^{16}$$ that are not within $$2^{-64}$$ of
an integer, where that rounding cannot change the result unless the value is extraordinarily close
to a rounding boundary. MPFR has no reciprocal gamma function; the mark in the Reference column is
for the reciprocal of the gamma function rounded down and up, at a working precision that doubles
until the two results round alike, for arguments with exponents between $$-1000$$ and $$20$$. That
reference shares the enclosure of $$\ln\Gamma$$ with the function under test, but not the
tiny-argument path or the rounding. The property tests also compare each `Float` function with the
`Rational` function of the same value, and the `primitive_float_*` versions are compared with the
`Float` functions.

## Number theory {#number-theory}

From [`malachite_float::float::number_theory`](https://docs.rs/malachite-float/latest/malachite_float/float/number_theory/index.html).

| Operation | Functions | Azurite | MPFR | Reference |
| --- | --- | :---: | :---: | :---: |
| [factorial](https://docs.rs/malachite-float/latest/malachite_float/float/number_theory/factorial/index.html) | `factorial_prec_round` |  | ✓ |  |

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
| [is_integer](https://docs.rs/malachite-float/latest/malachite_float/float/conversion/is_integer/index.html) | `IsInteger` | ✓ |  |  |
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

From [`malachite_float::float::exhaustive`](https://docs.rs/malachite-float/latest/malachite_float/float/exhaustive/index.html).

The exhaustive generators (`exhaustive_floats`, `exhaustive_finite_floats`, the positive, negative,
nonzero, non-negative, and non-positive finite versions, `exhaustive_floats_with_precision` and
`exhaustive_positive_floats_with_precision`, and the positive generators with a given scientific
exponent, with or without a given precision) have no oracle: MPFR has no generators, and Azurite has
exhaustive generators of integers, rationals, and their vectors and polynomials, but none of floats.
Their tests pin the first values that each generator produces, in decimal and in hexadecimal, so
that the precision of each value is pinned as well, and check that every value is valid.

## Random generation {#random-generation}

From [`malachite_float::float::random`](https://docs.rs/malachite-float/latest/malachite_float/float/random/index.html).

The random generators (`random_floats` and `random_finite_floats`, the positive, negative, nonzero,
non-negative, and non-positive finite versions, `random_positive_floats_with_precision`, the
`striped_` versions of all of these, `uniform_random_non_negative_floats_less_than_one` and
`uniform_random_non_negative_floats_at_most_one`, `exponential_random_floats`, and
`normal_random_floats`) have no independent oracle in the sense of this page: what they produce
depends on Malachite's own seeded generator, so another library cannot reproduce it. They are tested
instead by pinning the first values for fixed seeds and by comparing the distribution of a large
sample (its most common values, its median, and its mean, standard deviation, skewness, and excess
kurtosis) with the values the distribution should have, which catches a biased or truncated
generator.

## What is not yet cross-checked {#not-yet-cross-checked}

These 132 rows have no mark in any column, grouped by the section they are in:

- **Basic.** `constants` (`abs_is_min_positive_value`, `abs_is_max_finite_value_with_prec`),
  `default` (`Default`), `named` (`Named`), `classification` (`to_non_nan`, `into_non_nan`,
  `to_finite`, `into_finite`), `get_and_set` (`get_min_prec`; `from_float_prec`,
  `from_float_prec_round`, and their `_ref` forms), `ulp` (`increment`, `decrement`), `complexity`
  (`complexity`, `SignificantBits`).

- **Comparison.** `hash` (`Hash` for `ComparableFloat`), `cmp_abs` (`OrdAbs`, `PartialOrdAbs` for
  `ComparableFloat`; `PartialOrdAbsDouble`), `eq_abs` (`EqAbs` for `Float` and `ComparableFloat`),
  `min_max` (`min_rational`, `max_rational`, and their `_prec`, `_round`, and `_prec_round` forms;
  `primitive_float_min_rational`, `primitive_float_max_rational`), `partial_cmp_abs_primitive_float`
  (`PartialOrdAbs<f32>`, `PartialOrdAbs<f64>`, and the reverse directions), `eq_abs_natural`
  (`EqAbs<Natural>`, `EqAbs<Integer>`, `EqAbs<Rational>`, and the reverse directions),
  `eq_abs_primitive_int` (`EqAbs` with the primitive integers and floats, and the reverse
  directions), `partial_eq_gaussian_integer` (`PartialEq<GaussianInteger>`,
  `PartialEq<GaussianRational>`, and the reverse directions), `partial_cmp_abs_gaussian_integer`
  (`PartialOrdAbs` and `EqAbs` with `GaussianInteger` and `GaussianRational`, and the reverse
  directions).

- **Constants.** `champernowne_constant` (`primitive_float_champernowne_constant_base`),
  `copeland_erdos_constant` (`primitive_float_copeland_erdos_constant_base`), `liouvilles_constant`
  (`primitive_float_liouvilles_constant_base`).

- **Arithmetic.** `abs` (`abs_negative_zero`), `abs_squared` (`AbsSquared`), `dot`
  (`primitive_float_dot`), `average` (`Average`, `average_prec_round`), `positive_difference`
  (`positive_difference_rational_prec_round`; `rational_positive_difference_float_prec_round`;
  `primitive_float_positive_difference`, `primitive_float_positive_difference_rational`,
  `primitive_float_rational_positive_difference_float`), `hypot` (`primitive_float_hypot`),
  `is_unit` (`IsUnit`), `conjugate` (`Conjugate`), `canonicalize_unit` (`CanonicalizeUnit`),
  `canonical_unit_i_pow` (`CanonicalUnitIPow`), `round_to_integer` (`round_to_integer_ties_away`;
  `round_to_integer_ties_away_then_prec_round`; `primitive_float_round_to_integer`,
  `primitive_float_round_to_integer_ties_away`), `fractional_part`
  (`primitive_float_fractional_part`, `primitive_float_integer_and_fractional_parts`), `rem`
  (`ieee_remainder_rational_prec_round`; `ieee_remainder_rational_and_quotient_bits_prec_round`;
  `rational_ieee_remainder_float_prec_round`;
  `rational_ieee_remainder_float_and_quotient_bits_prec_round`; `rational_rem_float_prec_round`;
  `rational_rem_float_and_quotient_bits_prec_round`; `rem_rational_prec_round`;
  `rem_rational_and_quotient_bits_prec_round`; `primitive_float_ieee_remainder`,
  `primitive_float_ieee_remainder_and_quotient_bits`, `primitive_float_ieee_remainder_rational`,
  `primitive_float_ieee_remainder_rational_and_quotient_bits`,
  `primitive_float_rational_ieee_remainder_float`, `primitive_float_rational_rem_float`,
  `primitive_float_rem`, `primitive_float_rem_and_quotient_bits`, `primitive_float_rem_rational`,
  `primitive_float_rem_rational_and_quotient_bits`, `primitive_float_rem_unsigned`), `sqrt`
  (`primitive_float_sqrt_rational`), `reciprocal_sqrt` (`primitive_float_reciprocal_sqrt`,
  `primitive_float_reciprocal_sqrt_rational`), `cbrt` (`primitive_float_cbrt`,
  `primitive_float_cbrt_rational`), `root` (`root_s_rational_prec_round`;
  `root_u_rational_prec_round`; `primitive_float_root_s`, `primitive_float_root_s_rational`,
  `primitive_float_root_u`, `primitive_float_root_u_rational`), `pow` (`powr_prec_round`;
  `primitive_float_pow`, `primitive_float_pow_integer`, `primitive_float_pow_rational`,
  `primitive_float_pow_u`, `primitive_float_rational_pow`, `primitive_float_unsigned_pow`).

- **Exponentials and logarithms.** `exp` (`primitive_float_exp`, `primitive_float_exp_rational`),
  `exp_x_minus_1` (`primitive_float_exp_x_minus_1`, `primitive_float_exp_x_minus_1_rational`),
  `power_of_2_of_float` (`primitive_float_power_of_2`, `primitive_float_power_of_2_rational`),
  `power_of_2_x_minus_1` (`primitive_float_power_of_2_x_minus_1`,
  `primitive_float_power_of_2_x_minus_1_rational`), `power_of_10` (`primitive_float_power_of_10`,
  `primitive_float_power_of_10_rational`), `power_of_10_x_minus_1`
  (`primitive_float_power_of_10_x_minus_1`, `primitive_float_power_of_10_x_minus_1_rational`),
  `compound` (`primitive_float_compound`), `ln` (`primitive_float_ln`,
  `primitive_float_ln_rational`), `ln_1_plus_x` (`primitive_float_ln_1_plus_x`), `log_base_2`
  (`primitive_float_log_base_2`, `primitive_float_log_base_2_rational`), `log_base_2_1_plus_x`
  (`primitive_float_log_base_2_1_plus_x`), `log_base_10` (`primitive_float_log_base_10`,
  `primitive_float_log_base_10_rational`), `log_base_10_1_plus_x`
  (`primitive_float_log_base_10_1_plus_x`), `log_base` (`primitive_float_log_base`,
  `primitive_float_log_base_rational`), `log_base_1_plus_x` (`primitive_float_log_base_1_plus_x`),
  `log_base_power_of_2` (`primitive_float_log_base_power_of_2`,
  `primitive_float_log_base_power_of_2_rational`), `log_base_power_of_2_1_plus_x`
  (`primitive_float_log_base_power_of_2_1_plus_x`), `log_base_float_base`
  (`primitive_float_log_base_float_base`), `log_base_float_base_1_plus_x`
  (`primitive_float_log_base_float_base_1_plus_x`), `log_base_rational_base`
  (`primitive_float_log_base_rational_base`), `log_base_rational_base_1_plus_x`
  (`primitive_float_log_base_rational_base_1_plus_x`), `log_base_rational_float_base`
  (`primitive_float_log_base_rational_float_base`), `log_base_rational_rational_base`
  (`primitive_float_log_base_rational_rational_base`), `lambert_w0` (`primitive_float_lambert_w0`,
  `primitive_float_lambert_w0_rational`), `lambert_w_minus_1` (`primitive_float_lambert_w_minus_1`,
  `primitive_float_lambert_w_minus_1_rational`), `tree_t0` (`primitive_float_tree_t0`,
  `primitive_float_tree_t0_rational`), `tree_t_minus_1` (`primitive_float_tree_t_minus_1`,
  `primitive_float_tree_t_minus_1_rational`), `wright_omega` (`primitive_float_wright_omega`,
  `primitive_float_wright_omega_rational`).

- **Elliptic integrals.** `agm` (`primitive_float_agm`, `primitive_float_agm_rational`).

- **Trigonometric functions.** `sin` (`primitive_float_sin_pi`, `primitive_float_sin_pi_rational`),
  `cos` (`primitive_float_cos_pi`, `primitive_float_cos_pi_rational`), `tan`
  (`primitive_float_tan_pi`, `primitive_float_tan_pi_rational`), `sec` (`primitive_float_sec_pi`,
  `primitive_float_sec_pi_rational`, `primitive_float_sec_with_period`,
  `primitive_float_sec_with_period_rational`), `csc` (`primitive_float_csc_pi`,
  `primitive_float_csc_pi_rational`, `primitive_float_csc_with_period`,
  `primitive_float_csc_with_period_rational`), `cot` (`primitive_float_cot_pi`,
  `primitive_float_cot_pi_rational`, `primitive_float_cot_with_period`,
  `primitive_float_cot_with_period_rational`), `sin_cos` (`primitive_float_sin_cos`,
  `primitive_float_sin_cos_pi`, `primitive_float_sin_cos_pi_rational`,
  `primitive_float_sin_cos_rational`, `primitive_float_sin_cos_with_period`,
  `primitive_float_sin_cos_with_period_rational`), `asin` (`primitive_float_asin`,
  `primitive_float_asin_pi`, `primitive_float_asin_pi_rational`, `primitive_float_asin_rational`,
  `primitive_float_asin_with_period`, `primitive_float_asin_with_period_rational`), `acos`
  (`primitive_float_acos`, `primitive_float_acos_pi`, `primitive_float_acos_pi_rational`,
  `primitive_float_acos_rational`, `primitive_float_acos_with_period`,
  `primitive_float_acos_with_period_rational`), `atan` (`primitive_float_atan_pi`,
  `primitive_float_atan_pi_rational`, `primitive_float_atan_with_period`,
  `primitive_float_atan_with_period_rational`), `atan2` (`primitive_float_atan2`,
  `primitive_float_atan2_pi`, `primitive_float_atan2_pi_rational`, `primitive_float_atan2_rational`,
  `primitive_float_atan2_with_period`, `primitive_float_atan2_with_period_rational`), `asec`
  (`primitive_float_asec`, `primitive_float_asec_pi`, `primitive_float_asec_pi_rational`,
  `primitive_float_asec_rational`, `primitive_float_asec_with_period`,
  `primitive_float_asec_with_period_rational`), `acsc` (`primitive_float_acsc`,
  `primitive_float_acsc_pi`, `primitive_float_acsc_pi_rational`, `primitive_float_acsc_rational`,
  `primitive_float_acsc_with_period`, `primitive_float_acsc_with_period_rational`), `acot`
  (`primitive_float_acot`, `primitive_float_acot_pi`, `primitive_float_acot_pi_rational`,
  `primitive_float_acot_rational`, `primitive_float_acot_with_period`,
  `primitive_float_acot_with_period_rational`).

- **Hyperbolic functions.** `sinh_cosh` (`primitive_float_sinh_cosh`,
  `primitive_float_sinh_cosh_rational`), `gudermannian` (`primitive_float_gudermannian`,
  `primitive_float_gudermannian_rational`), `inverse_gudermannian`
  (`primitive_float_inverse_gudermannian`, `primitive_float_inverse_gudermannian_rational`).

- **The gamma function.** `gamma` (`primitive_float_gamma`, `primitive_float_gamma_rational`),
  `reciprocal_gamma` (`primitive_float_reciprocal_gamma`,
  `primitive_float_reciprocal_gamma_rational`), `ln_gamma` (`primitive_float_ln_gamma`,
  `primitive_float_ln_gamma_rational`), `ln_abs_gamma` (`primitive_float_ln_abs_gamma`,
  `primitive_float_ln_abs_gamma_rational`).

- **Conversion.** `from_natural` (`ConvertibleFrom<Natural>`), `from_integer`
  (`ConvertibleFrom<Integer>`), `from_rational` (`TryFrom<Rational>`, `ConvertibleFrom<Rational>`),
  `natural_from_float` (`RoundingFrom<Float>`, `TryFrom<Float>`, `ConvertibleFrom<Float>` for
  `Natural`), `integer_from_float` (`TryFrom<Float>`, `ConvertibleFrom<Float>` for `Integer`),
  `rational_from_float` (`ConvertibleFrom<Float>` for `Rational`), `is_real` (`IsReal`),
  `from_primitive_int` (`const_from_unsigned`, `const_from_signed`,
  `const_from_unsigned_times_power_of_2`, `const_from_signed_times_power_of_2`),
  `primitive_int_from_float` (`RoundingFrom<Float>`, `TryFrom<Float>`, `ConvertibleFrom<Float>` for
  every primitive integer), `primitive_float_from_float` (`TryFrom<Float>`, `ConvertibleFrom<Float>`
  for `f32`, `f64`), `mantissa_and_exponent` (`RawMantissaAndExponent`;
  `IntegerMantissaAndExponent`; `SciMantissaAndExponent` with a `Float` mantissa;
  `SciMantissaAndExponent` with an `f32` or `f64` mantissa, `sci_mantissa_and_exponent_round`),
  `from_gaussian_integer` (`TryFrom<GaussianInteger>`, `ConvertibleFrom<GaussianInteger>`),
  `from_gaussian_rational` (`TryFrom<GaussianRational>`, `ConvertibleFrom<GaussianRational>`),
  `gaussian_integer_from_float` (`TryFrom<Float>`, `ConvertibleFrom<Float>` for `GaussianInteger`),
  `gaussian_rational_from_float` (`TryFrom<Float>`, `ConvertibleFrom<Float>` for
  `GaussianRational`), `is_gaussian_integer` (`IsGaussianInteger`), `to_string` (`Display`, `Debug`
  for `Float` and `ComparableFloat`; `Binary`, `Octal`, `LowerHex`, `UpperHex`, `ToStringBase`),
  `from_string` (`FromStr`, `FromStringBase` for `Float` and `ComparableFloat`), `to_sci` (`ToSci`),
  `get_str` (`get_str_digit_count`), `latex` (`ToLatex`), `typst` (`ToTypst`), `serde` (`Serialize`,
  `Deserialize`).

Each section above says how its unmarked rows are tested, and almost all of them are covered by
property tests. The `primitive_float_*` functions are the `Float` functions computed at the
precision of `f32` or `f64` and converted into that type, so they rest on the checks of the `Float`
functions, and in most sections property tests compare the two. The unmarked forms with a `Rational`
operand (`min_rational` and `max_rational`, the remainders and `positive_difference` with a
`Rational` on either side, and the `Rational` roots), `powr`, and the forms of rounding to an
integer that break ties away from zero are covered by property tests; so are the absolute-value
comparisons and equalities, the comparisons with Gaussian numbers, the conversions to `Natural`, the
other fallible conversions and the `ConvertibleFrom` predicates, and the mantissa-and-exponent
decompositions, the last with round trips through their inverses. The string, LaTeX, Typst, and
serde output is checked chiefly by round trips. The rest, such as the constants and accessors of the
Basic section, `Default`, `Named`, `Hash`, and the unit functions, are fixed values or simple
operations, covered by unit and property tests.
