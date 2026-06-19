// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::arithmetic::log_base_rational_float_base::*;
use malachite_float::arithmetic::log_base_rational_rational_base::*;
use malachite_float::test_util::arithmetic::log_base_rational_float_base::{
    rug_log_base_rational_float_base_prec, rug_log_base_rational_float_base_prec_round,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    rational_float_unsigned_rounding_mode_quadruple_gen_var_1,
    rational_float_unsigned_rounding_mode_quadruple_gen_var_2,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_primitive_float_pair_gen;
use std::panic::catch_unwind;

// Cross-checks the by-value and by-reference variants, precision, the rational-base logarithm (for
// base > 1), and (unless `extreme`, a clamp/special result, or a large operand) the rug oracle.
fn check(
    x: &Rational,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) -> (Float, Ordering) {
    let (log, o) =
        Float::log_base_rational_float_base_prec_round(x.clone(), base.clone(), prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = Float::log_base_rational_float_base_prec_round_ref(x, base, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    // When the base is finite and greater than 1, the result must match the independently-tested
    // rational-base, rational-argument logarithm (which requires base > 1).
    if !extreme && base.is_finite() && *base > 1u32 {
        let (alt, o_alt) = Float::log_base_rational_rational_base_prec_round_ref(
            x,
            &Rational::exact_from(base),
            prec,
            rm,
        );
        assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    // Cross-check against the rug oracle, skipping extreme/large inputs and clamp/special results
    // (where the oracle's dyadic fallback would reconstruct an intractable ~2^30-bit Rational);
    // those are covered by the special-case unit tests and the rational-base cross-check above.
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
        let (rug_log, rug_o) = rug_log_base_rational_float_base_prec_round(
            x,
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
fn test_log_base_rational_float_base_prec_round() {
    let test = |xn: i64, xd: u64, base: f64, prec: u64, rm: RoundingMode, out: &str, o_out| {
        let x = Rational::from_signeds(xn, i64::exact_from(xd));
        let base = Float::exact_from(base);
        let (log, o) = check(&x, &base, prec, rm, false);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    // Exact, integer base.
    test(9, 1, 3.0, 10, Exact, "2.0", Equal); // log_3(9) = 2
    test(8, 1, 2.0, 10, Exact, "3.0", Equal); // log_2(8) = 3
    // Exact, non-integer base.
    test(8, 1, 4.0, 10, Exact, "1.5", Equal); // log_4(8) = 3/2
    test(2, 1, 4.0, 10, Exact, "0.5", Equal); // log_4(2) = 1/2
    // Non-dyadic x (only possible with a Rational argument).
    test(1, 3, 3.0, 10, Exact, "-1.0", Equal); // log_3(1/3) = -1
    test(1, 9, 3.0, 10, Exact, "-2.0", Equal); // log_3(1/9) = -2
    // Base in (0, 1): sign-flipped.
    test(4, 1, 0.5, 10, Exact, "-2.0", Equal); // log_{1/2}(4) = -2
    test(1, 4, 0.5, 10, Exact, "2.0", Equal); // log_{1/2}(1/4) = 2
    // Irrational (cross-checked against the oracle).
    test(2, 1, 3.0, 20, Floor, "0.630929", Less); // log_3(2)
    test(2, 1, 3.0, 20, Ceiling, "0.63093", Greater);
    test(1, 3, 0.5, 20, Nearest, "1.584963", Greater); // log_{1/2}(1/3) = log_2(3)
    // x = 1: signed zero by the base.
    test(1, 1, 3.0, 10, Exact, "0.0", Equal); // base > 1 -> +0
    test(1, 1, 0.5, 10, Exact, "-0.0", Equal); // base < 1 -> -0
}

#[test]
fn test_log_base_rational_float_base_specials() {
    let p = |x: Rational, base: Float| Float::log_base_rational_float_base_prec(x, base, 10).0;
    let cf = |x: Float| ComparableFloat(x);
    let two = Float::from(2);
    let half = Float::from(0.5);
    let five = Rational::from(5);
    let h = Rational::from_signeds(1, 2);

    // base out of domain.
    assert!(p(five.clone(), Float::NAN).is_nan());
    assert!(p(five.clone(), Float::from(-2)).is_nan());
    assert!(p(five.clone(), Float::NEGATIVE_INFINITY).is_nan());
    // x out of domain.
    assert!(p(Rational::from(-3), two.clone()).is_nan());
    // base = +infinity: 0 (signed), NaN for x = 0.
    assert_eq!(cf(p(five.clone(), Float::INFINITY)), cf(Float::ZERO));
    assert_eq!(cf(p(h.clone(), Float::INFINITY)), cf(Float::NEGATIVE_ZERO));
    assert!(p(Rational::ZERO, Float::INFINITY).is_nan());
    // base = 0: sign-flipped 0, NaN for x = 0.
    assert_eq!(cf(p(five.clone(), Float::ZERO)), cf(Float::NEGATIVE_ZERO));
    assert_eq!(cf(p(h.clone(), Float::ZERO)), cf(Float::ZERO));
    assert_eq!(cf(p(five.clone(), -Float::ZERO)), cf(Float::NEGATIVE_ZERO));
    assert!(p(Rational::ZERO, Float::ZERO).is_nan());
    // base = 1.
    assert_eq!(p(five.clone(), Float::ONE), Float::INFINITY);
    assert_eq!(p(h.clone(), Float::ONE), Float::NEGATIVE_INFINITY);
    assert_eq!(p(Rational::ZERO, Float::ONE), Float::NEGATIVE_INFINITY);
    assert!(p(Rational::ONE, Float::ONE).is_nan());
    // x = 0 with a normal base.
    assert_eq!(p(Rational::ZERO, two.clone()), Float::NEGATIVE_INFINITY); // base > 1
    assert_eq!(p(Rational::ZERO, half.clone()), Float::INFINITY); // base < 1
    // x = 1.
    assert_eq!(cf(p(Rational::ONE, two.clone())), cf(Float::ZERO));
    assert_eq!(cf(p(Rational::ONE, half.clone())), cf(Float::NEGATIVE_ZERO));
}

#[test]
fn test_log_base_rational_float_base_prec() {
    let test = |xn: i64, xd: u64, base: f64, prec: u64, out: &str, o_out| {
        let x = Rational::from_signeds(xn, i64::exact_from(xd));
        let base = Float::exact_from(base);

        let (log, o) = Float::log_base_rational_float_base_prec(x.clone(), base.clone(), prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = Float::log_base_rational_float_base_prec_ref(&x, &base, prec);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) =
            rug_log_base_rational_float_base_prec(&x, &rug::Float::exact_from(&base), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    test(9, 1, 3.0, 10, "2.0", Equal);
    test(8, 1, 4.0, 10, "1.5", Equal);
    test(1, 9, 3.0, 10, "-2.0", Equal); // non-dyadic x
    test(4, 1, 0.5, 10, "-2.0", Equal); // base < 1
    test(2, 1, 3.0, 20, "0.63093", Greater); // log_3(2)
    test(1, 1, 3.0, 10, "0.0", Equal);
}

fn log_base_rational_float_base_prec_round_properties_helper(
    x: &Rational,
    base: &Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (log, o) = check(x, base, prec, rm, extreme);

    // The sign of a normal result follows whether x and base are on the same side of 1.
    if log.is_normal()
        && *x > 0u32
        && *x != 1u32
        && base.is_finite()
        && *base > 0u32
        && *base != 1u32
        && o != Equal
    {
        assert_eq!(log > 0u32, (*x > 1u32) == (*base > 1u32));
    }

    // An exact result does not depend on the rounding mode; an inexact one cannot be produced with
    // `Exact`.
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (alt, o_alt) =
                Float::log_base_rational_float_base_prec_round_ref(x, base, prec, rm);
            assert_eq!(
                ComparableFloat(alt.abs_negative_zero()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, Equal);
        }
    } else {
        assert_panic!(Float::log_base_rational_float_base_prec_round_ref(
            x, base, prec, Exact
        ));
    }
}

#[test]
fn log_base_rational_float_base_prec_round_properties() {
    rational_float_unsigned_rounding_mode_quadruple_gen_var_1().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_rational_float_base_prec_round_properties_helper(&x, &base, prec, rm, false);
        },
    );
    rational_float_unsigned_rounding_mode_quadruple_gen_var_2().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_rational_float_base_prec_round_properties_helper(&x, &base, prec, rm, true);
        },
    );

    // The special cases hold for every precision and rounding mode.
    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let f = |x: Rational, base: Float| {
            Float::log_base_rational_float_base_prec_round(x, base, prec, rm)
        };
        assert!(f(Rational::from(5), Float::NAN).0.is_nan());
        assert!(f(Rational::from(-3), Float::from(2)).0.is_nan());
        assert!(f(Rational::from(5), Float::from(-2)).0.is_nan());
        assert_eq!(
            f(Rational::ZERO, Float::from(2)),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert!(f(Rational::ONE, Float::ONE).0.is_nan());
        assert_eq!(
            ComparableFloat(f(Rational::ONE, Float::from(2)).0),
            ComparableFloat(Float::ZERO)
        );
    });
}

#[test]
fn log_base_rational_float_base_prec_properties() {
    let f = |x: Rational, base: Float, prec: u64, extreme: bool| {
        let (log, o) = Float::log_base_rational_float_base_prec(x.clone(), base.clone(), prec);
        assert!(log.is_valid());
        let (log_ref, o_ref) = Float::log_base_rational_float_base_prec_ref(&x, &base, prec);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, &base, prec, Nearest, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    rational_float_unsigned_rounding_mode_quadruple_gen_var_1()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, false));
    rational_float_unsigned_rounding_mode_quadruple_gen_var_2()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, true));
}

#[test]
fn log_base_rational_float_base_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::log_base_rational_float_base_prec_round(
        Rational::from(8),
        Float::from(3),
        0,
        Nearest
    ));
    // Exact is not allowed when the result is not exactly representable.
    assert_panic!(Float::log_base_rational_float_base_prec_round(
        Rational::from(2),
        Float::from(3),
        10,
        Exact
    ));
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_log_base_rational_float_base() {
    fn test<T: PrimitiveFloat>(x: &Rational, base: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_log_base_rational_float_base::<T>(x, base)),
            NiceFloat(out)
        );
    }
    test::<f32>(&Rational::from(8), f32::NAN, f32::NAN); // base NaN
    test::<f32>(&Rational::from(-1), 10.0, f32::NAN); // x < 0
    test::<f32>(&Rational::from(8), -2.0, f32::NAN); // base < 0
    test::<f32>(&Rational::from(8), 4.0, 1.5); // log_4(8) = 3/2
    test::<f32>(&Rational::from(4), 0.5, -2.0); // log_(1/2)(4) = -2
    test::<f32>(&Rational::from(1000), 10.0, 3.0); // log_10(1000)
    test::<f32>(&Rational::from(50), 10.0, 1.6989699602127075); // log_10(50)
    test::<f32>(&Rational::from_unsigneds(1u8, 3), 10.0, -0.47712126); // log_10(1/3)
    test::<f32>(&Rational::ZERO, 10.0, f32::NEGATIVE_INFINITY); // x = 0, base > 1
    test::<f32>(&Rational::ZERO, 0.5, f32::INFINITY); // x = 0, base in (0, 1)
    test::<f32>(&Rational::ONE, 10.0, 0.0); // log_b(1) = 0
    test::<f32>(&Rational::from(8), f32::INFINITY, 0.0); // base = inf, x > 0
    test::<f32>(&Rational::from(8), 1.0, f32::INFINITY); // base = 1, x > 1
    test::<f32>(
        &Rational::from_unsigneds(1u8, 2),
        1.0,
        f32::NEGATIVE_INFINITY,
    ); // base = 1, 0<x<1
    test::<f32>(&Rational::ONE, 1.0, f32::NAN); // base = 1, x = 1

    test::<f64>(&Rational::from(8), f64::NAN, f64::NAN); // base NaN
    test::<f64>(&Rational::from(-1), 10.0, f64::NAN); // x < 0
    test::<f64>(&Rational::from(8), -2.0, f64::NAN); // base < 0
    test::<f64>(&Rational::from(8), 4.0, 1.5); // log_4(8) = 3/2
    test::<f64>(&Rational::from(4), 0.5, -2.0); // log_(1/2)(4) = -2
    test::<f64>(&Rational::from(1000), 10.0, 3.0); // log_10(1000)
    test::<f64>(&Rational::from(50), 10.0, 1.6989700043360187); // log_10(50)
    test::<f64>(
        &Rational::from_unsigneds(1u8, 3),
        10.0,
        -0.47712125471966244,
    ); // log_10(1/3)
    test::<f64>(&Rational::ZERO, 10.0, f64::NEGATIVE_INFINITY); // x = 0, base > 1
    test::<f64>(&Rational::ZERO, 0.5, f64::INFINITY); // x = 0, base in (0, 1)
    test::<f64>(&Rational::ONE, 10.0, 0.0); // log_b(1) = 0
    test::<f64>(&Rational::from(8), f64::INFINITY, 0.0); // base = inf, x > 0
    test::<f64>(&Rational::from(8), 1.0, f64::INFINITY); // base = 1, x > 1
    test::<f64>(
        &Rational::from_unsigneds(1u8, 2),
        1.0,
        f64::NEGATIVE_INFINITY,
    ); // base = 1, 0<x<1
    test::<f64>(&Rational::ONE, 1.0, f64::NAN); // base = 1, x = 1
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_log_base_rational_float_base_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    Rational: ExactFrom<T>,
{
    rational_primitive_float_pair_gen::<T>().test_properties(|(x, base)| {
        let y = primitive_float_log_base_rational_float_base::<T>(&x, base);
        // For a finite base greater than 1, the result matches the Rational-base function.
        if base.is_finite() && base > T::ONE {
            assert_eq!(
                NiceFloat(y),
                NiceFloat(primitive_float_log_base_rational_rational_base::<T>(
                    &x,
                    &Rational::exact_from(base)
                ))
            );
        }
    });
}

#[test]
fn primitive_float_log_base_rational_float_base_properties() {
    apply_fn_to_primitive_floats!(primitive_float_log_base_rational_float_base_properties_helper);
}
