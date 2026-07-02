// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{LogBaseOf1PlusX, LogBaseOf1PlusXAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_pair_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::arithmetic::log_base_float_base_1_plus_x::*;
use malachite_float::arithmetic::log_base_rational_base_1_plus_x::*;
use malachite_float::test_util::arithmetic::log_base_float_base_1_plus_x::{
    rug_log_base_float_base_1_plus_x, rug_log_base_float_base_1_plus_x_prec,
    rug_log_base_float_base_1_plus_x_prec_round, rug_log_base_float_base_1_plus_x_round,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_37, float_float_rounding_mode_triple_gen_var_38,
    float_float_unsigned_rounding_mode_quadruple_gen_var_13,
    float_float_unsigned_rounding_mode_quadruple_gen_var_14,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

// Cross-checks the by-value/by-reference/assigning variants, precision, the rational-base 1+x
// logarithm (for base > 1), and (unless `extreme`, a clamp/special result, or a large operand) the
// rug oracle. Returns the computed `(Float, Ordering)`.
fn check(x: &Float, base: &Float, prec: u64, rm: RoundingMode, extreme: bool) -> (Float, Ordering) {
    let (log, o) = x
        .clone()
        .log_base_float_base_1_plus_x_prec_round(base, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_float_base_1_plus_x_prec_round_ref(base, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_float_base_1_plus_x_prec_round_assign(base, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    // When the base is finite and greater than 1, the result must match the independently-tested
    // rational-base 1+x logarithm (which requires base > 1).
    if !extreme && base.is_finite() && *base > 1u32 {
        let (alt, o_alt) =
            x.log_base_rational_base_1_plus_x_prec_round_ref(&Rational::exact_from(base), prec, rm);
        assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    // Cross-check against the rug oracle, skipping extreme/large inputs and clamp/special results.
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
        let (rug_log, rug_o) = rug_log_base_float_base_1_plus_x_prec_round(
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
fn test_log_base_float_base_1_plus_x_prec_round() {
    let test = |x: f64, base: f64, prec: u64, rm: RoundingMode, out: &str, o_out| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);
        let (log, o) = check(&x, &base, prec, rm, false);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    // Exact, integer base.
    test(8.0, 3.0, 10, Exact, "2.0", Equal); // log_3(1 + 8) = log_3(9) = 2
    test(7.0, 2.0, 10, Exact, "3.0", Equal); // log_2(1 + 7) = log_2(8) = 3
    // Exact, non-integer base.
    test(7.0, 4.0, 10, Exact, "1.5", Equal); // log_4(8) = 3/2
    test(1.0, 4.0, 10, Exact, "0.5", Equal); // log_4(1 + 1) = log_4(2) = 1/2
    // Exact, base in (0, 1): sign-flipped.
    test(3.0, 0.5, 10, Exact, "-2.0", Equal); // log_{1/2}(1 + 3) = log_{1/2}(4) = -2
    test(-0.75, 0.5, 10, Exact, "2.0", Equal); // log_{1/2}(1/4) = 2
    // Exact, fractional root (base 3/2): 1 + x = (3/2)^m.
    test(0.5, 1.5, 10, Exact, "1.0", Equal); // log_{3/2}(1.5) = 1
    // x in (-1, 0).
    test(-0.75, 2.0, 10, Exact, "-2.0", Equal); // log_2(1/4) = -2
    // Irrational (cross-checked against the oracle).
    test(1.0, 3.0, 20, Floor, "0.630929", Less); // log_3(2)
    test(1.0, 3.0, 20, Ceiling, "0.63093", Greater);
    test(2.0, 2.0, 20, Nearest, "1.584963", Greater); // log_2(3)
    // x = +-0: signed zero by x's sign times the base.
    test(0.0, 3.0, 10, Exact, "0.0", Equal); // +0, base > 1 -> +0
    test(0.0, 0.5, 10, Exact, "-0.0", Equal); // +0, base < 1 -> -0
}

#[test]
fn test_log_base_float_base_1_plus_x_specials() {
    let p = |x: Float, base: Float| x.log_base_float_base_1_plus_x_prec(&base, 10).0;
    let cf = |x: Float| ComparableFloat(x);
    let two = Float::from(2);
    let half = Float::from(0.5);

    // NaN / domain.
    assert!(p(Float::NAN, two.clone()).is_nan());
    assert!(p(Float::from(5), Float::NAN).is_nan());
    assert!(p(Float::from(-2), two.clone()).is_nan()); // x < -1
    assert!(p(Float::NEGATIVE_INFINITY, two.clone()).is_nan());
    assert!(p(Float::from(5), Float::from(-2)).is_nan()); // base < 0
    assert!(p(Float::from(5), Float::NEGATIVE_INFINITY).is_nan());
    // base = +infinity: 0 (signed), NaN for x = +infinity or x = -1.
    assert_eq!(cf(p(Float::from(5), Float::INFINITY)), cf(Float::ZERO));
    assert_eq!(
        cf(p(Float::from(-0.5), Float::INFINITY)),
        cf(Float::NEGATIVE_ZERO)
    );
    assert!(p(Float::INFINITY, Float::INFINITY).is_nan());
    assert!(p(Float::NEGATIVE_ONE, Float::INFINITY).is_nan());
    // base = 0: sign-flipped 0.
    assert_eq!(cf(p(Float::from(5), Float::ZERO)), cf(Float::NEGATIVE_ZERO));
    assert_eq!(cf(p(Float::from(-0.5), Float::ZERO)), cf(Float::ZERO));
    // base = 1.
    assert_eq!(p(Float::from(5), Float::ONE), Float::INFINITY);
    assert_eq!(p(Float::from(-0.5), Float::ONE), Float::NEGATIVE_INFINITY);
    assert_eq!(p(Float::INFINITY, Float::ONE), Float::INFINITY);
    assert_eq!(p(Float::NEGATIVE_ONE, Float::ONE), Float::NEGATIVE_INFINITY);
    assert!(p(Float::ZERO, Float::ONE).is_nan()); // x = +-0, base = 1 -> NaN
    // x = +infinity / -1 with a normal base.
    assert_eq!(p(Float::INFINITY, two.clone()), Float::INFINITY);
    assert_eq!(
        p(Float::NEGATIVE_ONE, two.clone()),
        Float::NEGATIVE_INFINITY
    );
    assert_eq!(p(Float::INFINITY, half.clone()), Float::NEGATIVE_INFINITY);
    assert_eq!(p(Float::NEGATIVE_ONE, half.clone()), Float::INFINITY);
    // x = +-0.
    assert_eq!(cf(p(Float::ZERO, two.clone())), cf(Float::ZERO));
    assert_eq!(cf(p(-Float::ZERO, two.clone())), cf(Float::NEGATIVE_ZERO));
    assert_eq!(cf(p(Float::ZERO, half.clone())), cf(Float::NEGATIVE_ZERO));
    assert_eq!(cf(p(-Float::ZERO, half)), cf(Float::ZERO));
}

#[test]
fn test_log_base_float_base_1_plus_x_prec() {
    let test = |x: f64, base: f64, prec: u64, out: &str, o_out| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);

        let (log, o) = x.clone().log_base_float_base_1_plus_x_prec(&base, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_float_base_1_plus_x_prec_ref(&base, prec);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.log_base_float_base_1_plus_x_prec_assign(&base, prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) = rug_log_base_float_base_1_plus_x_prec(
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
    test(8.0, 3.0, 10, "2.0", Equal);
    test(7.0, 4.0, 10, "1.5", Equal);
    test(-0.75, 0.5, 10, "2.0", Equal); // base < 1
    test(1.0, 3.0, 20, "0.63093", Greater); // log_3(2)
    test(0.0, 3.0, 10, "0.0", Equal);
}

#[test]
fn test_log_base_float_base_1_plus_x_round() {
    let test = |x: f64, base: f64, rm: RoundingMode, out: &str, o_out| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);

        let (log, o) = x.clone().log_base_float_base_1_plus_x_round(&base, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_float_base_1_plus_x_round_ref(&base, rm);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) = rug_log_base_float_base_1_plus_x_round(
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
    // log_3(1 + 8) = 2, exact at the input's precision, across all rounding modes.
    test(8.0, 3.0, Floor, "2.0", Equal);
    test(8.0, 3.0, Ceiling, "2.0", Equal);
    test(8.0, 3.0, Nearest, "2.0", Equal);
    test(8.0, 3.0, Exact, "2.0", Equal);
    test(-0.75, 0.5, Exact, "2.0", Equal); // log_{1/2}(1/4) = 2, base < 1
    test(0.0, 3.0, Exact, "0.0", Equal);
    test(-2.0, 3.0, Nearest, "NaN", Equal); // x < -1
}

#[test]
fn test_log_base_float_base_1_plus_x() {
    // The `LogBaseOf1PlusX<Float>` trait: rounds to the input's precision, to nearest.
    let test = |x: f64, base: f64, out: &str| {
        let x = Float::exact_from(x);
        let base = Float::exact_from(base);

        let log = x.clone().log_base_1_plus_x(base.clone());
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);

        let log_alt = (&x).log_base_1_plus_x(&base);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

        let mut x_alt = x.clone();
        x_alt.log_base_1_plus_x_assign(&base);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_float_base_1_plus_x(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&base)
            ))),
            ComparableFloatRef(&log),
        );
    };
    test(8.0, 3.0, "2.0"); // log_3(9) = 2
    test(1.0, 4.0, "0.5"); // log_4(2) = 1/2
    test(-0.75, 0.5, "2.0"); // log_{1/2}(1/4) = 2
}

fn log_base_float_base_1_plus_x_prec_round_properties_helper(
    x: &Float,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (log, o) = check(x, base, prec, rm, extreme);

    // The sign of a normal result follows whether 1 + x and base are on the same side of 1: x > 0
    // (so 1 + x > 1) with base > 1, or x in (-1, 0) with base in (0, 1).
    if log.is_normal()
        && x.is_finite()
        && *x > -1i32
        && *x != 0u32
        && base.is_finite()
        && *base > 0u32
        && *base != 1u32
        && o != Equal
    {
        assert_eq!(log > 0u32, (*x > 0u32) == (*base > 1u32));
    }

    // An exact result does not depend on the rounding mode; an inexact one cannot be produced with
    // `Exact`.
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (alt, o_alt) = x.log_base_float_base_1_plus_x_prec_round_ref(base, prec, rm);
            assert_eq!(
                ComparableFloat(alt.abs_negative_zero()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, Equal);
        }
    } else {
        assert_panic!(x.log_base_float_base_1_plus_x_prec_round_ref(base, prec, Exact));
    }
}

#[test]
fn log_base_float_base_1_plus_x_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_13().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_float_base_1_plus_x_prec_round_properties_helper(&x, &base, prec, rm, false);
        },
    );
    float_float_unsigned_rounding_mode_quadruple_gen_var_14().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_float_base_1_plus_x_prec_round_properties_helper(&x, &base, prec, rm, true);
        },
    );

    // The special cases hold for every precision and rounding mode.
    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let f = |x: Float, base: Float| x.log_base_float_base_1_plus_x_prec_round(&base, prec, rm);
        assert!(f(Float::NAN, Float::from(2)).0.is_nan());
        assert!(f(Float::from(-2), Float::from(2)).0.is_nan());
        assert!(f(Float::from(5), Float::from(-2)).0.is_nan());
        assert_eq!(f(Float::INFINITY, Float::from(2)), (Float::INFINITY, Equal));
        assert_eq!(
            f(Float::NEGATIVE_ONE, Float::from(2)),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert!(f(Float::ZERO, Float::ONE).0.is_nan());
        assert_eq!(
            ComparableFloat(f(Float::ZERO, Float::from(2)).0),
            ComparableFloat(Float::ZERO)
        );
    });
}

#[test]
fn log_base_float_base_1_plus_x_prec_properties() {
    let f = |x: Float, base: Float, prec: u64, extreme: bool| {
        let (log, o) = x.clone().log_base_float_base_1_plus_x_prec(&base, prec);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_float_base_1_plus_x_prec_ref(&base, prec);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, &base, prec, Nearest, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_float_unsigned_rounding_mode_quadruple_gen_var_13()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, false));
    float_float_unsigned_rounding_mode_quadruple_gen_var_14()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, true));
}

#[test]
fn log_base_float_base_1_plus_x_round_properties() {
    let f = |x: Float, base: Float, rm: RoundingMode, extreme: bool| {
        let (log, o) = x.clone().log_base_float_base_1_plus_x_round(&base, rm);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_float_base_1_plus_x_round_ref(&base, rm);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, &base, x.significant_bits(), rm, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_float_rounding_mode_triple_gen_var_37()
        .test_properties_with_limit(20, |(x, base, rm)| f(x, base, rm, false));
    float_float_rounding_mode_triple_gen_var_38()
        .test_properties_with_limit(20, |(x, base, rm)| f(x, base, rm, true));
}

#[test]
fn log_base_float_base_1_plus_x_properties() {
    let f = |x: Float, base: Float, extreme: bool| {
        let (expected, _) = check(&x, &base, x.significant_bits(), Nearest, extreme);
        let log = x.clone().log_base_1_plus_x(base.clone());
        assert!(log.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        let log_ref = (&x).log_base_1_plus_x(&base);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        let mut x_alt = x.clone();
        x_alt.log_base_1_plus_x_assign(&base);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    };
    float_float_rounding_mode_triple_gen_var_37()
        .test_properties_with_limit(20, |(x, base, _rm)| f(x, base, false));
    float_float_rounding_mode_triple_gen_var_38()
        .test_properties_with_limit(20, |(x, base, _rm)| f(x, base, true));
}

#[test]
fn log_base_float_base_1_plus_x_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::from(8).log_base_float_base_1_plus_x_prec_round(
        &Float::from(3),
        0,
        Nearest
    ));
    // Exact is not allowed when the result is not exactly representable.
    assert_panic!(Float::from(1).log_base_float_base_1_plus_x_prec_round(
        &Float::from(3),
        10,
        Exact
    ));
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_log_base_float_base_1_plus_x() {
    fn test<T: PrimitiveFloat>(x: T, base: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_log_base_float_base_1_plus_x(x, base)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 10.0, f32::NAN); // x NaN
    test::<f32>(3.0, f32::NAN, f32::NAN); // base NaN
    test::<f32>(-2.0, 10.0, f32::NAN); // x < -1
    test::<f32>(3.0, -2.0, f32::NAN); // base < 0
    test::<f32>(3.0, 4.0, 1.0); // log_4(1 + 3) = log_4(4)
    test::<f32>(1.0, 4.0, 0.5); // log_4(1 + 1) = log_4(2)
    test::<f32>(3.0, 0.5, -2.0); // log_(1/2)(1 + 3) = log_(1/2)(4)
    test::<f32>(8.0, 9.0, 1.0); // log_9(1 + 8) = log_9(9)
    test::<f32>(1.0, 3.0, 0.63092977); // log_3(2)
    test::<f32>(1.0, 1.5, 1.7095113); // log_(3/2)(2)
    test::<f32>(49.0, 10.0, 1.6989699602127075); // log_10(50)
    test::<f32>(0.0, 10.0, 0.0); // x = +0
    test::<f32>(-0.0, 10.0, -0.0); // x = -0
    test::<f32>(-1.0, 10.0, f32::NEGATIVE_INFINITY); // x = -1, base > 1
    test::<f32>(-1.0, 0.5, f32::INFINITY); // x = -1, base in (0, 1)
    test::<f32>(f32::INFINITY, 10.0, f32::INFINITY); // x = inf, base > 1
    test::<f32>(f32::INFINITY, 0.5, f32::NEGATIVE_INFINITY); // x = inf, base in (0, 1)
    test::<f32>(3.0, f32::INFINITY, 0.0); // base = inf
    test::<f32>(3.0, 1.0, f32::INFINITY); // base = 1, x > 0
    test::<f32>(-0.5, 1.0, f32::NEGATIVE_INFINITY); // base = 1, -1 <= x < 0
    test::<f32>(0.0, 1.0, f32::NAN); // base = 1, x = 0

    test::<f64>(f64::NAN, 10.0, f64::NAN); // x NaN
    test::<f64>(3.0, f64::NAN, f64::NAN); // base NaN
    test::<f64>(-2.0, 10.0, f64::NAN); // x < -1
    test::<f64>(3.0, -2.0, f64::NAN); // base < 0
    test::<f64>(3.0, 4.0, 1.0); // log_4(1 + 3) = log_4(4)
    test::<f64>(1.0, 4.0, 0.5); // log_4(1 + 1) = log_4(2)
    test::<f64>(3.0, 0.5, -2.0); // log_(1/2)(1 + 3) = log_(1/2)(4)
    test::<f64>(8.0, 9.0, 1.0); // log_9(1 + 8) = log_9(9)
    test::<f64>(1.0, 3.0, 0.6309297535714574); // log_3(2)
    test::<f64>(1.0, 1.5, 1.7095112913514547); // log_(3/2)(2)
    test::<f64>(49.0, 10.0, 1.6989700043360187); // log_10(50)
    test::<f64>(0.0, 10.0, 0.0); // x = +0
    test::<f64>(-0.0, 10.0, -0.0); // x = -0
    test::<f64>(-1.0, 10.0, f64::NEGATIVE_INFINITY); // x = -1, base > 1
    test::<f64>(-1.0, 0.5, f64::INFINITY); // x = -1, base in (0, 1)
    test::<f64>(f64::INFINITY, 10.0, f64::INFINITY); // x = inf, base > 1
    test::<f64>(f64::INFINITY, 0.5, f64::NEGATIVE_INFINITY); // x = inf, base in (0, 1)
    test::<f64>(3.0, f64::INFINITY, 0.0); // base = inf
    test::<f64>(3.0, 1.0, f64::INFINITY); // base = 1, x > 0
    test::<f64>(-0.5, 1.0, f64::NEGATIVE_INFINITY); // base = 1, -1 <= x < 0
    test::<f64>(0.0, 1.0, f64::NAN); // base = 1, x = 0
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_log_base_float_base_1_plus_x_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    Rational: ExactFrom<T>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, base)| {
        let y = primitive_float_log_base_float_base_1_plus_x(x, base);
        // For a finite base greater than 1, the result matches the Rational-base function.
        if base.is_finite() && base > T::ONE {
            assert_eq!(
                NiceFloat(y),
                NiceFloat(primitive_float_log_base_rational_base_1_plus_x(
                    x,
                    &Rational::exact_from(base)
                ))
            );
        }
    });
}

#[test]
fn primitive_float_log_base_float_base_1_plus_x_properties() {
    apply_fn_to_primitive_floats!(primitive_float_log_base_float_base_1_plus_x_properties_helper);
}

// The exactness detection must not be skipped for large bases or extreme exponents: an
// exactly-representable result would leave the Ziv loop unable to terminate.
#[test]
fn log_base_float_base_1_plus_x_exact_extreme() {
    use malachite_base::num::arithmetic::traits::{Pow, PowerOf2};
    use malachite_nz::natural::Natural;
    // 1 + 2^70 = root, base = root^2 (a 142-bit Float): log_base(1 + x) = 1/2, exact.
    let x = Float::power_of_2(70i64);
    let base = Float::exact_from((Natural::power_of_2(70u64) + Natural::ONE).pow(2));
    let (v, o) = Float::log_base_float_base_1_plus_x_prec_round_ref(&x, &base, 1, Nearest);
    assert_eq!(o, Equal);
    assert_eq!(
        ComparableFloat(v),
        ComparableFloat(Float::power_of_2(-1i64))
    );
}
