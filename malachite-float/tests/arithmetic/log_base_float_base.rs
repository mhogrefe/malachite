// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::LogBase;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::arithmetic::log_base_float_base::{
    rug_log_base_float_base, rug_log_base_float_base_prec, rug_log_base_float_base_prec_round,
    rug_log_base_float_base_round,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_35, float_float_rounding_mode_triple_gen_var_36,
    float_float_unsigned_rounding_mode_quadruple_gen_var_11,
    float_float_unsigned_rounding_mode_quadruple_gen_var_12,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

// Cross-checks the by-value/by-reference/assigning variants, precision, and (unless `extreme`, a
// clamp/special result, or a large operand) the rug oracle. Returns the computed `(Float,
// Ordering)`.
fn check(x: &Float, base: &Float, prec: u64, rm: RoundingMode, extreme: bool) -> (Float, Ordering) {
    let (log, o) = x.clone().log_base_float_base_prec_round(base, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_float_base_prec_round_ref(base, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_float_base_prec_round_assign(base, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    // When the base is finite and greater than 1, the result must match the independently-tested
    // rational-base logarithm (which requires base > 1).
    if !extreme && base.is_finite() && *base > 1u32 {
        let (alt, o_alt) =
            x.log_base_rational_base_prec_round_ref(&Rational::exact_from(base), prec, rm);
        assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    // Cross-check against the rug oracle. Skip extreme inputs and large operands (slow bracketing),
    // and clamp/special results (NaN/infinity/zero or the min/max-magnitude clamp values), where
    // the oracle's dyadic fallback would reconstruct an intractable ~2^30-bit Rational; those are
    // covered by the special-case unit tests and the rational-base cross-check above.
    let e = log.get_exponent();
    let clamp_or_special = log == 0u32
        || log.is_infinite()
        || log.is_nan()
        || e == Some(Float::MIN_EXPONENT)
        || e == Some(Float::MAX_EXPONENT);
    if !extreme
        && !clamp_or_special
        && x.significant_bits() <= 128
        && base.significant_bits() <= 128
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rug_log, rug_o) = rug_log_base_float_base_prec_round(
            &rug::Float::exact_from(x),
            &rug::Float::exact_from(base),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    }
    (log, o)
}

#[test]
fn test_log_base_float_base_prec_round() {
    let test = |x: f64, base: f64, prec: u64, rm: RoundingMode, out: &str, o_out| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);
        let (log, o) = check(&x, &base, prec, rm, false);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    // Exact, integer base.
    test(9.0, 3.0, 10, Exact, "2.0", Equal); // log_3(9) = 2
    test(8.0, 2.0, 10, Exact, "3.0", Equal); // log_2(8) = 3
    // Exact, rational base.
    test(8.0, 4.0, 10, Exact, "1.5", Equal); // log_4(8) = 3/2
    test(2.0, 4.0, 10, Exact, "0.5", Equal); // log_4(2) = 1/2
    // Base in (0, 1): sign-flipped.
    test(4.0, 0.5, 10, Exact, "-2.0", Equal); // log_{1/2}(4) = -2
    test(0.25, 0.5, 10, Exact, "2.0", Equal); // log_{1/2}(1/4) = 2
    test(8.0, 0.5, 10, Exact, "-3.0", Equal); // log_{1/2}(8) = -3
    // x in (0, 1).
    test(0.25, 2.0, 10, Exact, "-2.0", Equal); // log_2(1/4) = -2
    // Irrational (cross-checked against the oracle).
    test(2.0, 3.0, 20, Floor, "0.630929", Less); // log_3(2)
    test(2.0, 3.0, 20, Ceiling, "0.63093", Greater);
    test(3.0, 2.0, 20, Nearest, "1.584963", Greater); // log_2(3)
    // x = 1: signed zero by the base.
    test(1.0, 3.0, 10, Exact, "0.0", Equal); // base > 1 -> +0
    test(1.0, 0.5, 10, Exact, "-0.0", Equal); // base < 1 -> -0
}

#[test]
fn test_log_base_float_base_specials() {
    let p = |x: Float, base: Float| x.log_base_float_base_prec(&base, 10).0;
    let cf = |x: Float| ComparableFloat(x);
    let two = Float::from(2);
    let half = Float::from(0.5);

    // NaN inputs.
    assert!(p(Float::NAN, two.clone()).is_nan());
    assert!(p(Float::from(5), Float::NAN).is_nan());
    // x out of domain.
    assert!(p(Float::from(-3), two.clone()).is_nan());
    assert!(p(Float::NEGATIVE_INFINITY, two.clone()).is_nan());
    // base out of domain.
    assert!(p(Float::from(5), Float::from(-2)).is_nan());
    assert!(p(Float::from(5), Float::NEGATIVE_INFINITY).is_nan());
    // base = +infinity: 0 (signed), or NaN for an infinite/zero x.
    assert_eq!(cf(p(Float::from(5), Float::INFINITY)), cf(Float::ZERO));
    assert_eq!(
        cf(p(half.clone(), Float::INFINITY)),
        cf(Float::NEGATIVE_ZERO)
    );
    assert!(p(Float::INFINITY, Float::INFINITY).is_nan());
    assert!(p(Float::ZERO, Float::INFINITY).is_nan());
    // base = 0: sign-flipped 0, or NaN for an infinite/zero x.
    assert_eq!(cf(p(Float::from(5), Float::ZERO)), cf(Float::NEGATIVE_ZERO));
    assert_eq!(cf(p(half.clone(), Float::ZERO)), cf(Float::ZERO));
    assert_eq!(
        cf(p(Float::from(5), -Float::ZERO)),
        cf(Float::NEGATIVE_ZERO)
    );
    assert!(p(Float::INFINITY, Float::ZERO).is_nan());
    // base = 1.
    assert_eq!(p(Float::from(5), Float::ONE), Float::INFINITY);
    assert_eq!(p(half.clone(), Float::ONE), Float::NEGATIVE_INFINITY);
    assert_eq!(p(Float::INFINITY, Float::ONE), Float::INFINITY);
    assert_eq!(p(Float::ZERO, Float::ONE), Float::NEGATIVE_INFINITY);
    assert!(p(Float::ONE, Float::ONE).is_nan());
    // x = +infinity / 0 with a normal base.
    assert_eq!(p(Float::INFINITY, two.clone()), Float::INFINITY);
    assert_eq!(p(Float::ZERO, two.clone()), Float::NEGATIVE_INFINITY);
    assert_eq!(p(Float::INFINITY, half.clone()), Float::NEGATIVE_INFINITY);
    assert_eq!(p(Float::ZERO, half.clone()), Float::INFINITY);
    // x = 1.
    assert_eq!(cf(p(Float::ONE, two.clone())), cf(Float::ZERO));
    assert_eq!(cf(p(Float::ONE, half.clone())), cf(Float::NEGATIVE_ZERO));
    let _ = two;
}

#[test]
fn test_log_base_float_base_prec() {
    let test = |x: f64, base: f64, prec: u64, out: &str, o_out| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);

        let (log, o) = x.clone().log_base_float_base_prec(&base, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_float_base_prec_ref(&base, prec);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.log_base_float_base_prec_assign(&base, prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) = rug_log_base_float_base_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&base),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    test(9.0, 3.0, 10, "2.0", Equal);
    test(8.0, 4.0, 10, "1.5", Equal);
    test(4.0, 0.5, 10, "-2.0", Equal); // base < 1
    test(2.0, 3.0, 20, "0.63093", Greater); // log_3(2)
    test(1.0, 3.0, 10, "0.0", Equal);
}

#[test]
fn test_log_base_float_base_round() {
    let test = |x: f64, base: f64, rm: RoundingMode, out: &str, o_out| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);

        let (log, o) = x.clone().log_base_float_base_round(&base, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_float_base_round_ref(&base, rm);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) = rug_log_base_float_base_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&base),
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log)),
                ComparableFloatRef(&log),
            );
            assert_eq!(rug_o, o);
        }
    };
    // log_3(81) = 4, exact at the input's precision, across all rounding modes.
    test(81.0, 3.0, Floor, "4.0", Equal);
    test(81.0, 3.0, Ceiling, "4.0", Equal);
    test(81.0, 3.0, Nearest, "4.0", Equal);
    test(81.0, 3.0, Exact, "4.0", Equal);
    test(0.25, 0.5, Exact, "2.0", Equal); // log_{1/2}(1/4) = 2, base < 1
    test(1.0, 3.0, Exact, "0.0", Equal);
    test(-3.0, 3.0, Nearest, "NaN", Equal); // x < 0
}

#[test]
fn test_log_base_float_base() {
    // The `LogBase<Float>` trait: rounds to the input's precision, to nearest.
    let test = |x: f64, base: f64, out: &str| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);

        let log = x.clone().log_base(base.clone());
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);

        let log_alt = (&x).log_base(&base);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_float_base(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&base)
            ))),
            ComparableFloatRef(&log),
        );
    };
    test(81.0, 3.0, "4.0"); // log_3(81) = 4
    test(2.0, 4.0, "0.5"); // log_4(2) = 1/2
    test(0.25, 0.5, "2.0"); // log_{1/2}(1/4) = 2
}

fn log_base_float_base_prec_round_properties_helper(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (log, o) = check(x, base, prec, rm, extreme);

    // The sign of a normal result follows whether x and base are on the same side of 1.
    if log.is_normal()
        && x.is_finite()
        && *x > 0u32
        && base.is_finite()
        && *base > 0u32
        && *base != 1u32
    {
        let x_big = *x > 1u32;
        let base_big = *base > 1u32;
        if *x != 1u32 && o != Equal {
            // log is positive iff x and base are on the same side of 1.
            assert_eq!(log > 0u32, x_big == base_big);
        }
    }

    // An exact result does not depend on the rounding mode; an inexact one cannot be produced with
    // `Exact`.
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (alt, o_alt) = x.log_base_float_base_prec_round_ref(base, prec, rm);
            assert_eq!(
                ComparableFloat(alt.abs_negative_zero()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, Equal);
        }
    } else {
        assert_panic!(x.log_base_float_base_prec_round_ref(base, prec, Exact));
    }
}

#[test]
fn log_base_float_base_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_11().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_float_base_prec_round_properties_helper(&x, &base, prec, rm, false);
        },
    );
    float_float_unsigned_rounding_mode_quadruple_gen_var_12().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_float_base_prec_round_properties_helper(&x, &base, prec, rm, true);
        },
    );

    // The special cases hold for every precision and rounding mode.
    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let f = |x: Float, base: Float| x.log_base_float_base_prec_round(&base, prec, rm);
        assert!(f(Float::NAN, Float::from(2)).0.is_nan());
        assert!(f(Float::from(2), Float::NAN).0.is_nan());
        assert!(f(Float::from(-2), Float::from(2)).0.is_nan());
        assert!(f(Float::from(2), Float::from(-2)).0.is_nan());
        assert_eq!(f(Float::INFINITY, Float::from(2)), (Float::INFINITY, Equal));
        assert_eq!(
            f(Float::ZERO, Float::from(2)),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert!(f(Float::ONE, Float::ONE).0.is_nan());
        assert_eq!(
            ComparableFloat(f(Float::ONE, Float::from(2)).0),
            ComparableFloat(Float::ZERO)
        );
    });
}

#[test]
fn log_base_float_base_prec_properties() {
    let f = |x: Float, base: Float, prec: u64, extreme: bool| {
        let (log, o) = x.clone().log_base_float_base_prec(&base, prec);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_float_base_prec_ref(&base, prec);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, &base, prec, Nearest, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_float_unsigned_rounding_mode_quadruple_gen_var_11()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, false));
    float_float_unsigned_rounding_mode_quadruple_gen_var_12()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, true));
}

#[test]
fn log_base_float_base_round_properties() {
    let f = |x: Float, base: Float, rm: RoundingMode, extreme: bool| {
        let (log, o) = x.clone().log_base_float_base_round(&base, rm);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_float_base_round_ref(&base, rm);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, &base, x.significant_bits(), rm, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_float_rounding_mode_triple_gen_var_35()
        .test_properties_with_limit(20, |(x, base, rm)| f(x, base, rm, false));
    float_float_rounding_mode_triple_gen_var_36()
        .test_properties_with_limit(20, |(x, base, rm)| f(x, base, rm, true));
}

#[test]
fn log_base_float_base_properties() {
    let f = |x: Float, base: Float, extreme: bool| {
        let (expected, _) = check(&x, &base, x.significant_bits(), Nearest, extreme);
        let log = x.clone().log_base(base.clone());
        assert!(log.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        let log_ref = (&x).log_base(&base);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
    };
    float_float_rounding_mode_triple_gen_var_35()
        .test_properties_with_limit(20, |(x, base, _rm)| f(x, base, false));
    float_float_rounding_mode_triple_gen_var_36()
        .test_properties_with_limit(20, |(x, base, _rm)| f(x, base, true));
}

#[test]
fn log_base_float_base_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::from(8).log_base_float_base_prec_round(&Float::from(3), 0, Nearest));
    // Exact is not allowed when the result is not exactly representable.
    assert_panic!(Float::from(2).log_base_float_base_prec_round(&Float::from(3), 10, Exact));
}
