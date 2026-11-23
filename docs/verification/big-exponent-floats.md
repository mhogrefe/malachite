---
layout: default
title: "How Malachite Is Tested: Big-Exponent Floats"
permalink: /verification/big-exponent-floats/
theme: jekyll-theme-slate
---

# How Malachite Is Tested: Big-Exponent Floats

This page lists every public function of
[`BigExponentFloat`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/struct.BigExponentFloat.html),
the binary floating-point type of the `malachite-float` crate that has an unbounded exponent and no
stored precision, and records which independent implementations it is checked against. [The
introduction](/verification/) describes the oracles and the testing they sit in; this page is the
ledger. It follows the organization of the crate's documentation, one section per module, so that a
function is where its documentation is.

## Reading the tables {#reading-the-tables}

Each row is one operation, named after the module that implements it and linked to that module's
documentation. The **Functions** column lists the traits and methods the module implements. A
trait's by-value and by-reference implementations, its `*Assign` form, and the `_prec` and
`_prec_round` variants of an operation (which differ only in the rounding mode) count as one
function, named after its `_prec_round` member, while functions with genuinely different results are
listed separately. The columns are:

- **Azurite** — the Azurite oracle of the `oracle-test/` driver, described in [the
  introduction](/verification/). Azurite's floating-point type, like `BigExponentFloat`, has an
  unbounded exponent, so the oracle recomputes every demo line directly, at any exponent, with no
  reduction of the inputs. Each demo is run in three generator modes at ten thousand lines each
  (two for the constants, whose generators have no `special_random` mode).
  Where `BigExponentFloat` documents a special value of its own, the check applies the documented
  rule: $$x/0$$ and $$0^y$$ for a negative odd integer $$y$$ are `NaN`, since with no negative zero
  the sign of the infinite limit is undecided, and so is a logarithm to base 1.
- **FLINT** — the FLINT oracle of the `oracle-test/` driver, which reads the output of Malachite's
  demos and recomputes every line. Most functions are checked against FLINT's `arf_t`, the type
  `BigExponentFloat` is modeled on; roots, powers, remainders, and rounding to integers are checked
  against [MPFR](https://www.mpfr.org/) inside the same oracle, with exponents beyond MPFR's range
  reduced exactly first; the operations that mix a `BigExponentFloat` with a `Rational` against
  exact FLINT rational arithmetic; and the exponential and logarithmic functions against FLINT's
  `arb_t` ball arithmetic, whose exponents are unbounded too. Every demo is run in three generator
  modes at ten thousand lines each, and the generators reach exponents far beyond $$2^{64}$$.
- **Float** — agreement with Malachite's
  [`Float`](https://docs.rs/malachite-float/latest/malachite_float/float/struct.Float.html), in the
  property tests, on every input whose values lie in `Float`'s exponent range. `Float` is itself
  checked against MPFR on [its own page](/verification/floats/). This check is not fully independent
  where `BigExponentFloat` reuses `Float`'s code: the field operations, square and other roots,
  powers, `hypot`, sums, the operations mixed with a `Rational`, the exponential functions, the AGM,
  the trigonometric functions, and the conversions that pass through a `Float` call `Float`'s
  kernels or functions, so for them the check covers `BigExponentFloat`'s own handling of exponents
  and special values, and the FLINT column is the independent check of the arithmetic. The constants
  are `Float`'s, converted exactly, so their independent check is `Float`'s comparison with MPFR.
- **Reference** — a second computation inside the property tests: either a generic algorithm that
  works on `Natural`s and `Integer`s directly (`add_prec_round_generic` and its siblings), or the
  exact result computed with `Rational` arithmetic and then rounded, on inputs whose exponents are
  small enough for that.

| | meaning |
| :---: | --- |
| ✓ | The check computes this function and agrees with Malachite on every input tried. |
| | The check does not cover this function. |

A function with no mark in any column is checked only by Malachite's own unit and property tests;
those rows are collected in [What is not yet cross-checked](#not-yet-cross-checked).

## Basic {#basic}

The constants, the classification predicates, and the accessors of the type itself, from
[`malachite_float::big_exponent_float::basic`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/basic/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [can_round](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/basic/can_round/index.html) | `can_round` |  |  | ✓ |  |
| [classification](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/basic/classification/index.html) | `classify`, `into_finite`, `into_non_nan`, `is_finite`, `is_infinite`, `is_nan`, `is_normal`, `is_sign_negative`, `is_sign_positive`, `is_zero`, `to_finite`, `to_non_nan` | ✓ | ✓ | ✓ |  |
| [complexity](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/basic/complexity/index.html) | `SignificantBits`, `complexity` |  | ✓ | ✓ |  |
| [constants](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/basic/constants/index.html) | `Default`, `Infinity`, `Max`, `Min`, `NaN`, `NegativeInfinity`, `NegativeOne`, `One`, `OneHalf`, `Two`, `Zero` |  |  |  |  |
| [get_and_set](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/basic/get_and_set/index.html) | `get_exponent`, `get_prec`, `into_significand`, `round_to_prec_round`, `significand`, `to_significand` | ✓ | ✓ | ✓ |  |

## Comparison {#comparison}

Equality, ordering, and their absolute-value forms, with every other numeric type; minima and
maxima, from
[`malachite_float::big_exponent_float::comparison`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [cmp](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/cmp/index.html) | `PartialOrd` | ✓ | ✓ | ✓ | ✓ |
| [cmp_abs](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/cmp_abs/index.html) | `PartialOrdAbs`, `PartialOrdAbsDouble` |  | ✓ | ✓ |  |
| [eq](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq/index.html) | `PartialEq` | ✓ | ✓ | ✓ |  |
| [eq_abs](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs/index.html) | `EqAbs` |  | ✓ | ✓ |  |
| [eq_abs_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_float/index.html) | `EqAbs`, `EqAbs for Float` |  | ✓ | ✓ |  |
| [eq_abs_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_gaussian_integer/index.html) | `EqAbs`, `EqAbs for GaussianInteger` |  | ✓ | ✓ |  |
| [eq_abs_gaussian_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_gaussian_rational/index.html) | `EqAbs`, `EqAbs for GaussianRational` |  | ✓ | ✓ |  |
| [eq_abs_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_integer/index.html) | `EqAbs`, `EqAbs for Integer` |  | ✓ | ✓ |  |
| [eq_abs_natural](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_natural/index.html) | `EqAbs`, `EqAbs for Natural` |  | ✓ | ✓ |  |
| [eq_abs_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_primitive_float/index.html) | `EqAbs`, `EqAbs for primitive` |  | ✓ | ✓ |  |
| [eq_abs_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_primitive_int/index.html) | `EqAbs`, `EqAbs for primitive` |  | ✓ | ✓ |  |
| [eq_abs_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/eq_abs_rational/index.html) | `EqAbs`, `EqAbs for Rational` |  | ✓ | ✓ |  |
| [min_max](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/min_max/index.html) | `min`, `max`, `min_prec_round`, `max_prec_round` |  | ✓ | ✓ |  |
| [min_max](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/min_max/index.html) | `min_rational_prec_round`, `max_rational_prec_round` |  | ✓ | ✓ |  |
| [partial_cmp_abs_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_float/index.html) | `PartialOrdAbs`, `PartialOrdAbs for Float` |  | ✓ | ✓ |  |
| [partial_cmp_abs_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_gaussian_integer/index.html) | `PartialOrdAbs`, `PartialOrdAbs for GaussianInteger` |  | ✓ | ✓ |  |
| [partial_cmp_abs_gaussian_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_gaussian_rational/index.html) | `PartialOrdAbs`, `PartialOrdAbs for GaussianRational` |  | ✓ | ✓ |  |
| [partial_cmp_abs_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_integer/index.html) | `PartialOrdAbs`, `PartialOrdAbs for Integer` |  | ✓ | ✓ |  |
| [partial_cmp_abs_natural](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_natural/index.html) | `PartialOrdAbs`, `PartialOrdAbs for Natural` |  | ✓ | ✓ |  |
| [partial_cmp_abs_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_primitive_float/index.html) | `PartialOrdAbs`, `PartialOrdAbs for primitive` |  | ✓ | ✓ |  |
| [partial_cmp_abs_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_primitive_int/index.html) | `PartialOrdAbs`, `PartialOrdAbs for primitive` |  | ✓ | ✓ |  |
| [partial_cmp_abs_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_abs_rational/index.html) | `PartialOrdAbs`, `PartialOrdAbs for Rational` |  | ✓ | ✓ |  |
| [partial_cmp_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_float/index.html) | `PartialOrd`, `PartialOrd for Float` |  | ✓ | ✓ |  |
| [partial_cmp_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_integer/index.html) | `PartialOrd`, `PartialOrd for Integer` | ✓ | ✓ | ✓ |  |
| [partial_cmp_natural](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_natural/index.html) | `PartialOrd`, `PartialOrd for Natural` | ✓ | ✓ | ✓ |  |
| [partial_cmp_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_primitive_float/index.html) | `PartialOrd`, `PartialOrd for primitive` |  | ✓ | ✓ |  |
| [partial_cmp_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_primitive_int/index.html) | `PartialOrd`, `PartialOrd for primitive` |  | ✓ | ✓ |  |
| [partial_cmp_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_cmp_rational/index.html) | `PartialOrd`, `PartialOrd for Rational` |  | ✓ | ✓ |  |
| [partial_eq_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_float/index.html) | `PartialEq`, `PartialEq for Float` |  | ✓ | ✓ |  |
| [partial_eq_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_gaussian_integer/index.html) | `PartialEq`, `PartialEq for GaussianInteger` |  | ✓ | ✓ |  |
| [partial_eq_gaussian_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_gaussian_rational/index.html) | `PartialEq`, `PartialEq for GaussianRational` |  | ✓ | ✓ |  |
| [partial_eq_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_integer/index.html) | `PartialEq`, `PartialEq for Integer` | ✓ | ✓ | ✓ |  |
| [partial_eq_natural](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_natural/index.html) | `PartialEq`, `PartialEq for Natural` | ✓ | ✓ | ✓ |  |
| [partial_eq_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_primitive_float/index.html) | `PartialEq`, `PartialEq for primitive` |  | ✓ | ✓ |  |
| [partial_eq_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_primitive_int/index.html) | `PartialEq`, `PartialEq for primitive` |  | ✓ | ✓ |  |
| [partial_eq_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/comparison/partial_eq_rational/index.html) | `PartialEq`, `PartialEq for Rational` |  | ✓ | ✓ |  |

## Constants {#constants}

Approximations of mathematical constants, such as $$\pi$$, $$e$$, and $$\ln 2$$, with a given
precision and rounding mode, from
[`malachite_float::big_exponent_float::constants`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [catalans_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/catalans_constant/index.html) | `catalans_constant_prec_round` | ✓ |  | ✓ |  |
| [cbrt_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/cbrt_2/index.html) | `cbrt_2_prec_round` | ✓ |  | ✓ |  |
| [champernowne_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/champernowne_constant/index.html) | `champernowne_constant_base_prec_round`, `champernowne_constant_prec_round` |  |  | ✓ |  |
| [copeland_erdos_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/copeland_erdos_constant/index.html) | `copeland_erdos_constant_base_prec_round`, `copeland_erdos_constant_prec_round` |  |  | ✓ |  |
| [dottie_number](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/dottie_number/index.html) | `dottie_number_prec_round` | ✓ |  | ✓ |  |
| [e](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/e/index.html) | `e_prec_round` | ✓ |  | ✓ |  |
| [eulers_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/eulers_constant/index.html) | `eulers_constant_prec_round` | ✓ |  | ✓ |  |
| [gauss_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/gauss_constant/index.html) | `gauss_constant_prec_round` | ✓ |  | ✓ |  |
| [gelfond_schneider_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/gelfond_schneider_constant/index.html) | `gelfond_schneider_constant_prec_round` | ✓ |  | ✓ |  |
| [gelfonds_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/gelfonds_constant/index.html) | `gelfonds_constant_prec_round` | ✓ |  | ✓ |  |
| [lemniscate_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/lemniscate_constant/index.html) | `lemniscate_constant_prec_round` | ✓ |  | ✓ |  |
| [liouvilles_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/liouvilles_constant/index.html) | `liouvilles_constant_base_prec_round`, `liouvilles_constant_prec_round` |  |  | ✓ |  |
| [ln_10](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/ln_10/index.html) | `ln_10_prec_round` | ✓ |  | ✓ |  |
| [ln_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/ln_2/index.html) | `ln_2_prec_round` | ✓ |  | ✓ |  |
| [log_10_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/log_10_2/index.html) | `log_10_2_prec_round` | ✓ |  | ✓ |  |
| [log_10_e](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/log_10_e/index.html) | `log_10_e_prec_round` | ✓ |  | ✓ |  |
| [log_2_10](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/log_2_10/index.html) | `log_2_10_prec_round` | ✓ |  | ✓ |  |
| [log_2_e](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/log_2_e/index.html) | `log_2_e_prec_round` | ✓ |  | ✓ |  |
| [one_over_pi](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/one_over_pi/index.html) | `one_over_pi_prec_round` | ✓ |  | ✓ |  |
| [one_over_sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/one_over_sqrt_pi/index.html) | `one_over_sqrt_pi_prec_round` | ✓ |  | ✓ |  |
| [one_over_sqrt_tau](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/one_over_sqrt_tau/index.html) | `one_over_sqrt_tau_prec_round` | ✓ |  | ✓ |  |
| [phi](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/phi/index.html) | `phi_prec_round` | ✓ |  | ✓ |  |
| [pi](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/pi/index.html) | `pi_prec_round` | ✓ |  | ✓ |  |
| [pi_over_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/pi_over_2/index.html) | `pi_over_2_prec_round` | ✓ |  | ✓ |  |
| [pi_over_3](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/pi_over_3/index.html) | `pi_over_3_prec_round` | ✓ |  | ✓ |  |
| [pi_over_4](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/pi_over_4/index.html) | `pi_over_4_prec_round` | ✓ |  | ✓ |  |
| [pi_over_6](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/pi_over_6/index.html) | `pi_over_6_prec_round` | ✓ |  | ✓ |  |
| [pi_over_8](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/pi_over_8/index.html) | `pi_over_8_prec_round` | ✓ |  | ✓ |  |
| [prime_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/prime_constant/index.html) | `prime_constant_prec_round` | ✓ |  | ✓ |  |
| [prouhet_thue_morse_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/prouhet_thue_morse_constant/index.html) | `prouhet_thue_morse_constant_prec_round` | ✓ |  | ✓ |  |
| [ramanujans_constant](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/ramanujans_constant/index.html) | `ramanujans_constant_prec_round` | ✓ |  | ✓ |  |
| [sqrt_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/sqrt_2/index.html) | `sqrt_2_prec_round` | ✓ |  | ✓ |  |
| [sqrt_2_over_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/sqrt_2_over_2/index.html) | `sqrt_2_over_2_prec_round` | ✓ |  | ✓ |  |
| [sqrt_3](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/sqrt_3/index.html) | `sqrt_3_prec_round` | ✓ |  | ✓ |  |
| [sqrt_3_over_3](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/sqrt_3_over_3/index.html) | `sqrt_3_over_3_prec_round` | ✓ |  | ✓ |  |
| [sqrt_5](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/sqrt_5/index.html) | `sqrt_5_prec_round` | ✓ |  | ✓ |  |
| [sqrt_5_over_5](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/sqrt_5_over_5/index.html) | `sqrt_5_over_5_prec_round` | ✓ |  | ✓ |  |
| [sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/sqrt_pi/index.html) | `sqrt_pi_prec_round` | ✓ |  | ✓ |  |
| [tau](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/tau/index.html) | `tau_prec_round` | ✓ |  | ✓ |  |
| [two_over_pi](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/two_over_pi/index.html) | `two_over_pi_prec_round` | ✓ |  | ✓ |  |
| [two_over_sqrt_pi](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/constants/two_over_sqrt_pi/index.html) | `two_over_sqrt_pi_prec_round` | ✓ |  | ✓ |  |

## Arithmetic {#arithmetic}

The arithmetic operations, from the field operations to remainders, roots, powers, and rounding to
integers, from
[`malachite_float::big_exponent_float::arithmetic`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [abs](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/abs/index.html) | `Abs`, `AbsAssign` | ✓ | ✓ | ✓ | ✓ |
| [abs_squared](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/abs_squared/index.html) | `AbsSquared`, `AbsSquaredAssign` |  | ✓ |  |  |
| [add](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/add/index.html) | `Add`, `AddAssign`, `add_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [add_mul](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/add_mul/index.html) | `AddMul`, `AddMulAssign`, `add_mul_prec_round`, `add_mul_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [add_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/add_rational/index.html) | `add_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [average](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/average/index.html) | `Average`, `AverageAssign`, `average_prec_round` |  | ✓ | ✓ | ✓ |
| [canonical_unit_i_pow](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/canonical_unit_i_pow/index.html) | `CanonicalUnitIPow` |  | ✓ | ✓ |  |
| [canonicalize_gaussian_unit](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/canonicalize_gaussian_unit/index.html) | `CanonicalizeGaussianUnit`, `CanonicalizeGaussianUnitAssign` |  | ✓ |  |  |
| [canonicalize_unit](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/canonicalize_unit/index.html) | `CanonicalizeUnit`, `CanonicalizeUnitAssign` |  | ✓ |  |  |
| [cbrt](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/cbrt/index.html) | `cbrt_prec_round`, `cbrt_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [conjugate](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/conjugate/index.html) | `Conjugate`, `ConjugateAssign` |  | ✓ |  |  |
| [div](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/div/index.html) | `div_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [div_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/div_rational/index.html) | `div_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [dot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/dot/index.html) | `dot`, `dot_prec_round` |  | ✓ | ✓ | ✓ |
| [fractional_part](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/fractional_part/index.html) | `fractional_part`, `fractional_part_prec_round`, `integer_and_fractional_parts`, `integer_and_fractional_parts_prec_round` |  | ✓ | ✓ | ✓ |
| [hypot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/hypot/index.html) | `hypot_prec_round` |  | ✓ | ✓ | ✓ |
| [is_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/is_power_of_2/index.html) | `IsPowerOf2` | ✓ | ✓ | ✓ |  |
| [is_unit](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/is_unit/index.html) | `IsUnit` |  | ✓ | ✓ |  |
| [mul](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/mul/index.html) | `Mul`, `MulAssign`, `mul_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [mul_add_mul](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/mul_add_mul/index.html) | `MulAddMul`, `MulAddMulAssign`, `mul_add_mul_prec_round`, `mul_add_mul_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [mul_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/mul_rational/index.html) | `mul_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [mul_sub_mul](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/mul_sub_mul/index.html) | `MulSubMul`, `MulSubMulAssign`, `mul_sub_mul_prec_round`, `mul_sub_mul_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [neg](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/neg/index.html) | `Neg`, `NegAssign` | ✓ | ✓ | ✓ | ✓ |
| [positive_difference](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/positive_difference/index.html) | `positive_difference`, `positive_difference_prec_round` |  | ✓ | ✓ |  |
| [positive_difference](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/positive_difference/index.html) | `positive_difference_rational_prec_round`, `rational_positive_difference_big_exponent_float_prec_round` |  | ✓ | ✓ | ✓ |
| [pow](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/pow/index.html) | `Pow`, `PowAssign`, `pow_integer_prec_round`, `pow_prec_round`, `pow_rational_prec_round`, `pow_s_prec_round`, `pow_u_prec_round`, `powr_prec_round`, `rational_pow_prec_round`, `rational_pow_rational_prec_round`, `unsigned_pow_prec_round`, `unsigned_pow_rational_prec_round`, `unsigned_pow_unsigned_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [power_of_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/power_of_2/index.html) | `PowerOf2` | ✓ | ✓ | ✓ |  |
| [product](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/product/index.html) | `Product`, `product_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [rational_div_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/rational_div_big_exponent_float/index.html) | `rational_div_big_exponent_float_prec_round` |  | ✓ | ✓ | ✓ |
| [reciprocal](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/reciprocal/index.html) | `reciprocal_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [reciprocal_sqrt](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/reciprocal_sqrt/index.html) | `reciprocal_sqrt_prec_round`, `reciprocal_sqrt_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/rem/index.html) | `Rem`, `rem_prec_round`, `ieee_remainder`, `ieee_remainder_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/rem/index.html) | `rem_and_quotient_bits`, `rem_and_quotient_bits_prec_round`, `ieee_remainder_and_quotient_bits`, `ieee_remainder_and_quotient_bits_prec_round` |  | ✓ | ✓ | ✓ |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/rem/index.html) | `rem_rational_prec_round`, `ieee_remainder_rational_prec_round`, `rem_rational_and_quotient_bits_prec_round`, `ieee_remainder_rational_and_quotient_bits_prec_round` |  | ✓ | ✓ | ✓ |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/rem/index.html) | `rational_rem_big_exponent_float_prec_round`, `rational_ieee_remainder_big_exponent_float_prec_round`, `rational_rem_big_exponent_float_and_quotient_bits_prec_round`, `rational_ieee_remainder_big_exponent_float_and_quotient_bits_prec_round` |  | ✓ | ✓ | ✓ |
| [rem](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/rem/index.html) | `rem_unsigned`, `rem_unsigned_prec_round` |  | ✓ |  |  |
| [root](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/root/index.html) | `root_s_prec_round`, `root_s_rational_prec_round`, `root_u_prec_round`, `root_u_rational_prec_round` |  | ✓ | ✓ | ✓ |
| [round_to_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/round_to_integer/index.html) | `round_to_integer`, `round_to_integer_prec_round`, `round_to_integer_round`, `round_to_integer_then_prec_round`, `round_to_integer_ties_away`, `round_to_integer_ties_away_prec`, `round_to_integer_ties_away_then_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [shl](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/shl/index.html) | `Shl`, `ShlAssign` | ✓ | ✓ | ✓ | ✓ |
| [shr](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/shr/index.html) | `Shr`, `ShrAssign` | ✓ | ✓ | ✓ | ✓ |
| [sign](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/sign/index.html) | `Sign` | ✓ | ✓ | ✓ |  |
| [sqrt](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/sqrt/index.html) | `sqrt_prec_round`, `sqrt_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [square](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/square/index.html) | `Square`, `SquareAssign`, `square_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sub](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/sub/index.html) | `Sub`, `SubAssign`, `sub_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sub_mul](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/sub_mul/index.html) | `SubMul`, `SubMulAssign`, `sub_mul_prec_round`, `sub_mul_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sub_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/sub_rational/index.html) | `sub_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sum](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/arithmetic/sum/index.html) | `Sum`, `sum_prec_round` | ✓ | ✓ | ✓ | ✓ |

## Conversion {#conversion}

Conversions to and from the other numeric types, mantissa-and-exponent forms, and strings, from
[`malachite_float::big_exponent_float::conversion`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [big_exponent_float_from_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/big_exponent_float_from_float/index.html) | `From` |  |  | ✓ |  |
| [float_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/float_from_big_exponent_float/index.html) | `ConvertibleFrom for Float`, `TryFrom for Float`, `from_big_exponent_float_prec_round` |  |  | ✓ |  |
| [from_bits](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_bits/index.html) | `non_dyadic_from_bits_prec_round` |  |  | ✓ |  |
| [from_digits](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_digits/index.html) | `non_dyadic_from_digits_prec_round`, `non_dyadic_from_power_of_2_digits_prec_round` |  |  | ✓ |  |
| [from_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_gaussian_integer/index.html) | `ConvertibleFrom`, `TryFrom` |  |  |  |  |
| [from_gaussian_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_gaussian_rational/index.html) | `ConvertibleFrom`, `TryFrom` |  |  |  |  |
| [from_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_integer/index.html) | `From`, `from_integer_prec_round` | ✓ | ✓ | ✓ |  |
| [from_natural](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_natural/index.html) | `From`, `from_natural_prec_round` | ✓ | ✓ | ✓ |  |
| [from_primitive_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_primitive_float/index.html) | `From`, `from_primitive_float_prec_round` |  | ✓ | ✓ |  |
| [from_primitive_int](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_primitive_int/index.html) | `From`, `const_from_signed`, `const_from_signed_times_power_of_2`, `const_from_unsigned`, `const_from_unsigned_times_power_of_2`, `from_signed_prec_round`, `from_unsigned_prec_round` | ✓ | ✓ | ✓ |  |
| [from_rational](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/from_rational/index.html) | `ConvertibleFrom`, `TryFrom`, `from_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [gaussian_integer_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/gaussian_integer_from_big_exponent_float/index.html) | `ConvertibleFrom for GaussianInteger`, `TryFrom for GaussianInteger` |  | ✓ |  |  |
| [gaussian_rational_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/gaussian_rational_from_big_exponent_float/index.html) | `ConvertibleFrom for GaussianRational`, `TryFrom for GaussianRational` |  | ✓ |  |  |
| [integer_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/integer_from_big_exponent_float/index.html) | `ConvertibleFrom for Integer`, `RoundingFrom for Integer`, `TryFrom for Integer` |  | ✓ | ✓ |  |
| [is_gaussian_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/is_gaussian_integer/index.html) | `IsGaussianInteger` |  | ✓ | ✓ |  |
| [is_integer](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/is_integer/index.html) | `IsInteger` | ✓ | ✓ | ✓ |  |
| [is_real](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/is_real/index.html) | `IsReal` |  | ✓ | ✓ |  |
| [mantissa_and_exponent](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/mantissa_and_exponent/index.html) | `IntegerMantissaAndExponent`, `RawMantissaAndExponent`, `SciMantissaAndExponent`, `sci_mantissa_and_exponent_round` |  | ✓ | ✓ |  |
| [natural_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/natural_from_big_exponent_float/index.html) | `ConvertibleFrom for Natural`, `RoundingFrom for Natural`, `TryFrom for Natural` |  | ✓ | ✓ |  |
| [primitive_float_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/primitive_float_from_big_exponent_float/index.html) | `ConvertibleFrom for primitive`, `RoundingFrom for primitive`, `TryFrom for primitive` |  | ✓ | ✓ |  |
| [primitive_int_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/primitive_int_from_big_exponent_float/index.html) | `ConvertibleFrom for primitive`, `RoundingFrom for primitive`, `TryFrom for primitive` |  | ✓ | ✓ |  |
| [rational_from_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/rational_from_big_exponent_float/index.html) | `ConvertibleFrom for Rational`, `TryFrom for Rational` |  | ✓ | ✓ |  |
| [serde](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/serde/index.html) | `Serialize`, `Deserialize` |  |  |  |  |
| [string/from_sci_string](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/string/from_sci_string/index.html) | `FromSciString`, `from_sci_string_prec_round`, `from_sci_string_with_options_prec` |  | ✓ | ✓ |  |
| [string/from_string](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/string/from_string/index.html) | `FromStr`, `FromStringBase` |  | ✓ | ✓ |  |
| [string/latex](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/string/latex/index.html) | `ToLatex` |  |  | ✓ |  |
| [string/to_sci](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/string/to_sci/index.html) | `ToSci` |  | ✓ | ✓ |  |
| [string/to_string](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/string/to_string/index.html) | `Binary`, `Debug`, `Display`, `LowerHex`, `Octal`, `ToStringBase`, `UpperHex` |  | ✓ | ✓ |  |
| [string/typst](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/conversion/string/typst/index.html) | `ToTypst` |  |  | ✓ |  |

## Exponential {#exponential}

The exponential functions $$e^x$$, $$2^x$$, and $$10^x$$ and their counterparts minus 1, the
logarithms in bases $$e$$, 2, 10, and $$2^k$$, in a `u64`, `BigExponentFloat`, or `Rational` base,
and their $$1+x$$ forms, of `BigExponentFloat`s, `Rational`s, and (for $$\ln$$) `u64`s, and the
compound function $$(1+x)^n$$, the two real branches $$W_0$$ and $$W_{-1}$$ of the Lambert
$$W$$ function, the two real branches $$T_0$$ and $$T_{-1}$$ of the tree function
$$T(x) = -W(-x)$$, and the Wright omega function $$\omega(x) = W_0(e^x)$$, from
[`malachite_float::big_exponent_float::exponential`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [compound](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/compound/index.html) | `compound_prec_round` |  | ✓ | ✓ |  |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/exp/index.html) | `exp_prec_round` | ✓ | ✓ | ✓ |  |
| [exp](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/exp/index.html) | `exp_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/exp_x_minus_1/index.html) | `exp_x_minus_1_prec_round` | ✓ | ✓ | ✓ |  |
| [exp_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/exp_x_minus_1/index.html) | `exp_x_minus_1_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [lambert_w0](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/lambert_w0/index.html) | `lambert_w0_prec_round` | ✓ |  |  | ✓ |
| [lambert_w0](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/lambert_w0/index.html) | `lambert_w0_rational_prec_round` | ✓ |  | ✓ | ✓ |
| [lambert_w_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/lambert_w_minus_1/index.html) | `lambert_w_minus_1_prec_round` | ✓ |  |  | ✓ |
| [lambert_w_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/lambert_w_minus_1/index.html) | `lambert_w_minus_1_rational_prec_round` | ✓ |  | ✓ | ✓ |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/ln/index.html) | `ln_prec_round` | ✓ | ✓ | ✓ |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/ln/index.html) | `ln_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [ln](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/ln/index.html) | `ln_unsigned_prec_round` | ✓ | ✓ | ✓ |  |
| [ln_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/ln_1_plus_x/index.html) | `ln_1_plus_x_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base/index.html) | `log_base_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base/index.html) | `log_base_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_10/index.html) | `log_base_10_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_10](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_10/index.html) | `log_base_10_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_10_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_10_1_plus_x/index.html) | `log_base_10_1_plus_x_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_1_plus_x/index.html) | `log_base_1_plus_x_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_2/index.html) | `log_base_2_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_2/index.html) | `log_base_2_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_2_1_plus_x/index.html) | `log_base_2_1_plus_x_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_big_exponent_float_base](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_big_exponent_float_base/index.html) | `log_base_big_exponent_float_base_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_big_exponent_float_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_big_exponent_float_base_1_plus_x/index.html) | `log_base_big_exponent_float_base_1_plus_x_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_power_of_2/index.html) | `log_base_power_of_2_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_power_of_2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_power_of_2/index.html) | `log_base_power_of_2_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_power_of_2_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_power_of_2_1_plus_x/index.html) | `log_base_power_of_2_1_plus_x_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_rational_base](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_rational_base/index.html) | `log_base_rational_base_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_rational_base_1_plus_x](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_rational_base_1_plus_x/index.html) | `log_base_rational_base_1_plus_x_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_rational_big_exponent_float_base](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_rational_big_exponent_float_base/index.html) | `log_base_rational_big_exponent_float_base_prec_round` | ✓ | ✓ | ✓ |  |
| [log_base_rational_rational_base](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/log_base_rational_rational_base/index.html) | `log_base_rational_rational_base_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_10/index.html) | `power_of_10_of_big_exponent_float_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_10](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_10/index.html) | `power_of_10_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_10_x_minus_1/index.html) | `power_of_10_x_minus_1_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_10_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_10_x_minus_1/index.html) | `power_of_10_x_minus_1_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_2_of_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_2_of_big_exponent_float/index.html) | `power_of_2_of_big_exponent_float_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_2_of_big_exponent_float](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_2_of_big_exponent_float/index.html) | `power_of_2_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_2_x_minus_1/index.html) | `power_of_2_x_minus_1_prec_round` | ✓ | ✓ | ✓ |  |
| [power_of_2_x_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/power_of_2_x_minus_1/index.html) | `power_of_2_x_minus_1_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [tree_t0](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/tree_t0/index.html) | `tree_t0_prec_round` | ✓ |  |  | ✓ |
| [tree_t0](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/tree_t0/index.html) | `tree_t0_rational_prec_round` | ✓ |  | ✓ | ✓ |
| [tree_t_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/tree_t_minus_1/index.html) | `tree_t_minus_1_prec_round` | ✓ |  |  | ✓ |
| [tree_t_minus_1](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/tree_t_minus_1/index.html) | `tree_t_minus_1_rational_prec_round` | ✓ |  | ✓ | ✓ |
| [wright_omega](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/wright_omega/index.html) | `wright_omega_prec_round` | ✓ |  | ✓ | ✓ |
| [wright_omega](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exponential/wright_omega/index.html) | `wright_omega_rational_prec_round` | ✓ |  | ✓ | ✓ |

The Lambert $$W$$ functions are ports of Azurite's `lambertW0PrecRound` and `lambertWm1PrecRound`
and their `Rat` forms, so the Azurite column compares two implementations of one algorithm. Their
mark in the Reference column is for a check that does not depend on that algorithm: the property
tests certify that a result $$w$$ is correctly rounded by deciding the sign of $$t e^t - x$$, with
directed rounding at increasing precision, at the neighbors $$t$$ of $$w$$ that determine the
rounding. The tree functions are the Lambert $$W$$ functions of the negated argument, rounded in
the opposite direction and negated, as Azurite's `treeT0PrecRound` and `treeTm1PrecRound` are, and
the same certificate is applied to $$W(-x) = -T(x)$$. The Wright omega function is a port of
`wrightOmegaPrecRound` and `wrightOmegaRatPrecRound`, with one path added for a very negative
argument; its reference is $$W_0(e^x)$$, computed from `exp` and `lambert_w0` rounded down and up.
Its tests keep the exponent of the argument at most $$2^{10}$$, as those of `exp` do.

## Hyperbolic {#hyperbolic}

The hyperbolic sine, cosine, tangent, cotangent, secant, and cosecant, the hyperbolic sine and
cosine together, their inverses, and the Gudermannian function and its inverse, of
`BigExponentFloat`s and
`Rational`s, from
[`malachite_float::big_exponent_float::hyperbolic`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/acosh/index.html) | `acosh_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acosh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/acosh/index.html) | `acosh_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/acoth/index.html) | `acoth_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acoth](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/acoth/index.html) | `acoth_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/acsch/index.html) | `acsch_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [acsch](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/acsch/index.html) | `acsch_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/asech/index.html) | `asech_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asech](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/asech/index.html) | `asech_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/asinh/index.html) | `asinh_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asinh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/asinh/index.html) | `asinh_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/atanh/index.html) | `atanh_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atanh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/atanh/index.html) | `atanh_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/cosh/index.html) | `cosh_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cosh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/cosh/index.html) | `cosh_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/coth/index.html) | `coth_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [coth](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/coth/index.html) | `coth_rational_prec_round` | ≈ | ✓ | ✓ |  |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/csch/index.html) | `csch_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [csch](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/csch/index.html) | `csch_rational_prec_round` | ≈ | ✓ | ✓ |  |
| [gudermannian](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/gudermannian/index.html) | `gudermannian_prec_round` | ✓ |  | ✓ | ✓ |
| [gudermannian](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/gudermannian/index.html) | `gudermannian_rational_prec_round` | ✓ |  | ✓ | ✓ |
| [inverse_gudermannian](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/inverse_gudermannian/index.html) | `inverse_gudermannian_prec_round` | ✓ |  | ✓ | ✓ |
| [inverse_gudermannian](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/inverse_gudermannian/index.html) | `inverse_gudermannian_rational_prec_round` | ✓ |  | ✓ | ✓ |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/sech/index.html) | `sech_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sech](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/sech/index.html) | `sech_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/sinh/index.html) | `sinh_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sinh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/sinh/index.html) | `sinh_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/sinh_cosh/index.html) | `sinh_cosh_prec_round` | ✓ | ✓ | ✓ |  |
| [sinh_cosh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/sinh_cosh/index.html) | `sinh_cosh_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/tanh/index.html) | `tanh_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [tanh](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/hyperbolic/tanh/index.html) | `tanh_rational_prec_round` | ✓ | ✓ | ✓ |  |

The Gudermannian function is a port of Azurite's `gdPrecRound` and `gdRatPrecRound`. Its mark in
the Reference column is for a computation that shares none of the port's paths:
$$\arctan(\sinh x)$$ with the hyperbolic sine and the arctangent rounded down and up, at a working
precision that doubles until the two results round alike, for arguments that are not too large or
too small. The inverse Gudermannian function is a port of `gdInvPrecRound` and `gdInvRatPrecRound`,
with the reference $$\operatorname{atanh}(\sin x)$$ computed in the same way.

The `csch`, `coth`, and `acsch` functions are ≈ against Azurite for their pole at 0, which Azurite gives as
$$+\infty$$ and a `BigExponentFloat` as `NaN`; the oracle reads an infinite Azurite result as `NaN`,
as for the cosecant and cotangent under [Trigonometric](#trigonometric).

## Gamma {#gamma}

The gamma function, its reciprocal, and the logarithms of the gamma function and of its absolute
value, of `BigExponentFloat`s and `Rational`s, from
[`malachite_float::big_exponent_float::gamma`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/gamma/index.html) | `gamma_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/gamma/index.html) | `gamma_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [reciprocal_gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/reciprocal_gamma/index.html) | `reciprocal_gamma_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [reciprocal_gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/reciprocal_gamma/index.html) | `reciprocal_gamma_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [ln_gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/ln_gamma/index.html) | `ln_gamma_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [ln_gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/ln_gamma/index.html) | `ln_gamma_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [ln_abs_gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/ln_abs_gamma/index.html) | `ln_abs_gamma_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [ln_abs_gamma](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/gamma/ln_abs_gamma/index.html) | `ln_abs_gamma_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |

The gamma function and its reciprocal are ports of Azurite's `gammaPrecRound`,
`reciprocalGammaPrecRound`, and their `Rat` forms. The gamma function is ≈ against Azurite for its
pole at 0, which Azurite gives as $$+\infty$$ (the limit from the right) and a `BigExponentFloat` as
`NaN`, as at the negative integers; the oracle gives `NaN` for a zero argument. The Float column is
the independent check of the gamma function, since `Float`'s is compared with MPFR on [its own
page](/verification/floats/#the-gamma-function), although `Float`'s function is a wrapper around
this one. The marks in the Reference column are for two things. The gamma function encloses the
gamma function of a positive number with single values and error bounds; the property tests also run
a second version of it that, like Azurite's, carries enclosures through every operation, compare the
results of the two for both functions, and check that the first version's enclosure contains the
second's at more than twice the working precision. And the reciprocal is compared with the
reciprocal of the gamma function rounded down and up, at a working precision that doubles until the
two results round alike, for arguments that are not too large or too small. Both functions are
also checked against FLINT's `arb_gamma`, `arb_rgamma`, and `arb_gamma_fmpq`, rounded by bracketing
the result between neighbors with one more bit, and exactly at the positive integers. Near 0,
where $$\Gamma(x)$$ is within about $$0.58$$ of $$1/x$$, the oracle compares $$\Gamma(x) - w$$ in
the form $$(1 - wx)/x - \gamma$$ plus a bound of $$2|x|$$, and $$1/\Gamma(x) - w$$ as
$$(x - w) + \gamma x^2$$ plus a bound of $$2|x|^3$$, so that arguments with very negative exponents
need no more working precision than others.

The logarithm of the gamma function and the logarithm of its absolute value are ports of Azurite's
`lnGammaPrecRound`, `lnAbsGammaPrecRound`, and their `Rat` forms, and agree with them: a
`BigExponentFloat` has no negative zero for the sign to differ at. `ln_abs_gamma_prec_round` is ≈
because only its unit tests' rows are run against Azurite, not its demos: Azurite's sine has
absolute accuracy, so its `lnAbsGamma` takes about $$|e|$$ bits of working precision for a negative
argument near $$-2^e$$ with $$e$$ far below 0, and the generators of `BigExponentFloat`s produce
exponents in the billions. (FLINT checks those demos, and Azurite the `Float` function on the
arguments of `Float`'s generators.) The Float column stands for
the comparison of `Float`'s functions, which are wrappers around these, with MPFR's `mpfr_lngamma`
and `mpfr_lgamma` on [their own page](/verification/floats/#the-gamma-function). The Reference
column is for the second version of the bracket of $$\ln\Gamma$$, which carries enclosures through
every operation: the property tests compare the results of the two versions for both functions.
FLINT's check uses `arb_lgamma` for a positive argument and, for a negative one, the reflection
formula $$\ln|\Gamma(x)| = \ln(\pi/|\sin \pi x|) - \ln\Gamma(1-x)$$ with `arb_sin_pi` or
`arb_sin_pi_fmpq`, which reduce their arguments exactly; it rounds by the same bracketing as for the
gamma function, and takes the values 0 at 1 and 2 and the sign of $$\Gamma(x)$$ exactly. The unit
tests' rows with the argument $$-2^{-2^{40}}$$ are checked against FLINT only.

## Elliptic {#elliptic}

The arithmetic-geometric mean of two `BigExponentFloat`s, of two `Rational`s, and of a
`BigExponentFloat` with a `Rational`, from
[`malachite_float::big_exponent_float::elliptic`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/elliptic/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/elliptic/agm/index.html) | `agm_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/elliptic/agm/index.html) | `agm_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [agm](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/elliptic/agm/index.html) | `agm_big_exponent_float_rational_prec_round` | ✓ | ✓ | ✓ |  |

## Trigonometric {#trigonometric}

The sine, cosine, tangent, cotangent, secant, and cosecant, the sine and cosine together, their
inverses, and atan2, of `BigExponentFloat`s and `Rational`s, in radians, in $$u$$ths of a turn, and
in half-turns, from
[`malachite_float::big_exponent_float::trigonometric`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/index.html).

| Operation | Functions | Azurite | FLINT | Float | Reference |
| --- | --- | :---: | :---: | :---: | :---: |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acos/index.html) | `acos_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acos/index.html) | `acos_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acos/index.html) | `acos_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acos/index.html) | `acos_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acos/index.html) | `acos_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acos/index.html) | `acos_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acot/index.html) | `acot_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acot/index.html) | `acot_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acot/index.html) | `acot_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acot/index.html) | `acot_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acot/index.html) | `acot_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acot/index.html) | `acot_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acsc/index.html) | `acsc_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acsc/index.html) | `acsc_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acsc/index.html) | `acsc_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acsc/index.html) | `acsc_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acsc/index.html) | `acsc_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [acsc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/acsc/index.html) | `acsc_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asec/index.html) | `asec_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asec/index.html) | `asec_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asec/index.html) | `asec_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asec/index.html) | `asec_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asec/index.html) | `asec_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asec/index.html) | `asec_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asin/index.html) | `asin_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asin/index.html) | `asin_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asin/index.html) | `asin_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asin/index.html) | `asin_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asin/index.html) | `asin_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [asin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/asin/index.html) | `asin_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan/index.html) | `atan_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan/index.html) | `atan_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan/index.html) | `atan_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan/index.html) | `atan_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan/index.html) | `atan_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan/index.html) | `atan_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan2/index.html) | `atan2_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan2/index.html) | `atan2_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan2/index.html) | `atan2_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan2/index.html) | `atan2_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan2/index.html) | `atan2_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [atan2](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/atan2/index.html) | `atan2_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cos/index.html) | `cos_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cos/index.html) | `cos_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cos/index.html) | `cos_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cos/index.html) | `cos_with_period_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cos/index.html) | `cos_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cos/index.html) | `cos_pi_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cot/index.html) | `cot_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cot/index.html) | `cot_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cot/index.html) | `cot_with_period_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cot/index.html) | `cot_with_period_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cot/index.html) | `cot_pi_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [cot](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/cot/index.html) | `cot_pi_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/csc/index.html) | `csc_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/csc/index.html) | `csc_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/csc/index.html) | `csc_with_period_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/csc/index.html) | `csc_with_period_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/csc/index.html) | `csc_pi_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [csc](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/csc/index.html) | `csc_pi_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sec/index.html) | `sec_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sec/index.html) | `sec_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sec/index.html) | `sec_with_period_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sec/index.html) | `sec_with_period_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sec/index.html) | `sec_pi_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [sec](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sec/index.html) | `sec_pi_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin/index.html) | `sin_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin/index.html) | `sin_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin/index.html) | `sin_with_period_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin/index.html) | `sin_with_period_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin/index.html) | `sin_pi_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sin](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin/index.html) | `sin_pi_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin_cos/index.html) | `sin_cos_prec_round` | ✓ | ✓ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin_cos/index.html) | `sin_cos_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin_cos/index.html) | `sin_cos_with_period_prec_round` | ✓ | ✓ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin_cos/index.html) | `sin_cos_with_period_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin_cos/index.html) | `sin_cos_pi_prec_round` | ✓ | ✓ | ✓ |  |
| [sin_cos](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/sin_cos/index.html) | `sin_cos_pi_rational_prec_round` | ✓ | ✓ | ✓ |  |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/tan/index.html) | `tan_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/tan/index.html) | `tan_rational_prec_round` | ✓ | ✓ | ✓ | ✓ |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/tan/index.html) | `tan_with_period_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/tan/index.html) | `tan_with_period_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/tan/index.html) | `tan_pi_prec_round` | ≈ | ✓ | ✓ | ✓ |
| [tan](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/trigonometric/tan/index.html) | `tan_pi_rational_prec_round` | ≈ | ✓ | ✓ | ✓ |

The `tan` and `sec` functions with a period and in half-turns, and all six `csc` and `cot`
functions (the cosecant and cotangent have a pole at 0), are ≈ against Azurite for their poles. Azurite gives a function at a pole
an infinity, as IEEE 754 does (the tangent takes the sign of the sine, the secant is $$+\infty$$,
and the cosecant and cotangent of 0 are $$+\infty$$), while a `BigExponentFloat`, which has no negative zero
to tell the two one-sided limits apart, gives `NaN` there, as it does for $$1/0$$. Azurite's
exponent is unbounded, so these functions of a finite argument are infinite only at a pole, and the
oracle reads an infinite Azurite result as `NaN`.

## Generators {#generators}

The exhaustive and random generators, from
[`malachite_float::big_exponent_float::exhaustive`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/exhaustive/index.html)
and
[`malachite_float::big_exponent_float::random`](https://docs.rs/malachite-float/latest/malachite_float/big_exponent_float/random/index.html),
have no oracle, since they define sequences rather than compute values. Their tests pin the first
values that each generator produces and, for the random generators, the most common values.

## What is not yet cross-checked {#not-yet-cross-checked}

These rows have no mark in any column:

- `constants`: `Default`, `Infinity`, `Max`, `Min`, `NaN`, `NegativeInfinity`, `NegativeOne`, `One`, `OneHalf`, `Two`, `Zero`
- `from_gaussian_integer`: `ConvertibleFrom`, `TryFrom`
- `from_gaussian_rational`: `ConvertibleFrom`, `TryFrom`
- `serde`: `Serialize`, `Deserialize`

The constants are fixed values; the Gaussian conversions reduce to the real conversions, which are
checked, plus a test of the imaginary part; and serialization round-trips through the hexadecimal
strings, which are checked. Each could be given an oracle check of its own.
