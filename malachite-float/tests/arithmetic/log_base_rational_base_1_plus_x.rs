// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{LogBaseOf1PlusX, LogBaseOf1PlusXAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::arithmetic::log_base_rational_base_1_plus_x::{
    rug_log_base_rational_base_1_plus_x, rug_log_base_rational_base_1_plus_x_prec,
    rug_log_base_rational_base_1_plus_x_prec_round, rug_log_base_rational_base_1_plus_x_round,
};
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::generators::{
    float_rational_rounding_mode_triple_gen_var_14, float_rational_rounding_mode_triple_gen_var_15,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_13,
    float_rational_unsigned_rounding_mode_quadruple_gen_var_14,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::panic::catch_unwind;

// Cross-checks the by-value/by-reference/assigning variants, precision, and (unless `extreme`,
// underflow, or a large base/x) the rug oracle. Returns the computed `(Float, Ordering)`.
fn check(
    x: &Float,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) -> (Float, Ordering) {
    let (log, o) = x
        .clone()
        .log_base_rational_base_1_plus_x_prec_round(base, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_rational_base_1_plus_x_prec_round_ref(base, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_rational_base_1_plus_x_prec_round_assign(base, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    // Skip the oracle for extreme inputs, large operands (its ln_1p(x)/ln(base) bracketing uses a
    // working precision that grows with the base's and x's bit lengths), and underflow (where the
    // oracle's dyadic fallback would reconstruct a Rational from a float of exponent ~ -2^30, an
    // intractable bignum -- the same skip as in the integer-base `log_base_1_plus_x` test;
    // underflow correctness is covered by reasoning and the shared ExtendedFloat clamp).
    let underflowed = log == 0u32
        || ComparableFloat(log.abs_negative_zero_ref())
            == ComparableFloat(Float::min_positive_value_prec(prec).abs_negative_zero());
    if !extreme
        && !underflowed
        && base.significant_bits() <= 128
        && x.significant_bits() <= 128
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rug_log, rug_o) = rug_log_base_rational_base_1_plus_x_prec_round(
            &rug::Float::exact_from(x),
            base,
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
fn test_log_base_rational_base_1_plus_x_prec_round() {
    let test = |n: i64, d: u64, bn: u64, bd: u64, prec: u64, rm: RoundingMode, out: &str, o_out| {
        let x = Float::exact_from(Rational::from_signeds(n, i64::exact_from(d)));
        let base = Rational::from_unsigneds(bn, bd);
        let (log, o) = check(&x, &base, prec, rm, false);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    // Exact, integer base: 1 + x a power of the base's root.
    test(8, 1, 3, 1, 10, Exact, "2.0", Equal); // log_3(1 + 8) = log_3(9) = 2
    test(8, 1, 9, 1, 10, Exact, "1.0", Equal); // log_9(1 + 8) = log_9(9) = 1
    test(3, 1, 4, 1, 10, Exact, "1.0", Equal); // log_4(1 + 3) = log_4(4) = 1
    test(1, 1, 4, 1, 10, Exact, "0.5", Equal); // log_4(1 + 1) = log_4(2) = 1/2
    // Exact, rational base, including a non-integer x (dyadic root 3/2).
    test(1, 2, 3, 2, 10, Exact, "1.0", Equal); // log_{3/2}(1 + 1/2) = log_{3/2}(3/2) = 1
    test(5, 4, 3, 2, 10, Exact, "2.0", Equal); // log_{3/2}(1 + 5/4) = log_{3/2}(9/4) = 2
    // x in (-1, 0): negative result.
    test(-1, 2, 4, 1, 10, Exact, "-0.5", Equal); // log_4(1 - 1/2) = log_4(1/2) = -1/2
    test(-3, 4, 4, 1, 10, Exact, "-1.0", Equal); // log_4(1 - 3/4) = log_4(1/4) = -1
    // Irrational (cross-checked against the oracle).
    test(1, 1, 3, 1, 20, Floor, "0.630929", Less); // log_3(2)
    test(1, 1, 3, 1, 20, Ceiling, "0.63093", Greater);
    test(1, 1, 3, 2, 20, Nearest, "1.709511", Less); // log_{3/2}(2)
    // x = 0.
    test(0, 1, 3, 1, 10, Exact, "0.0", Equal);
    test(0, 1, 7, 3, 10, Exact, "0.0", Equal);
}

#[test]
fn test_log_base_rational_base_1_plus_x_specials() {
    let three = Rational::from(3u32);
    let nearest = |x: Float| x.log_base_rational_base_1_plus_x_prec_round(&three, 10, Nearest);

    assert_eq!(nearest(Float::NAN).0.to_string(), "NaN");
    assert_eq!(nearest(Float::INFINITY), (Float::INFINITY, Equal));
    assert_eq!(nearest(Float::NEGATIVE_INFINITY).0.to_string(), "NaN");
    assert_eq!(nearest(Float::ZERO), (Float::ZERO, Equal));
    assert_eq!(nearest(-Float::ZERO), (-Float::ZERO, Equal));
    assert_eq!(
        nearest(Float::NEGATIVE_ONE),
        (Float::NEGATIVE_INFINITY, Equal)
    ); // 1 + (-1) = 0
    assert_eq!(nearest(Float::from(-2)).0.to_string(), "NaN"); // x < -1
}

#[test]
fn test_log_base_rational_base_1_plus_x_prec() {
    let test = |n: i64, d: u64, bn: u64, bd: u64, prec: u64, out: &str, o_out| {
        let x = Float::exact_from(Rational::from_signeds(n, i64::exact_from(d)));
        let base = Rational::from_unsigneds(bn, bd);

        let (log, o) = x.clone().log_base_rational_base_1_plus_x_prec(&base, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_rational_base_1_plus_x_prec_ref(&base, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.log_base_rational_base_1_plus_x_prec_assign(&base, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) =
            rug_log_base_rational_base_1_plus_x_prec(&rug::Float::exact_from(&x), &base, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    test(8, 1, 3, 1, 10, "2.0", Equal); // log_3(9) = 2
    test(3, 1, 4, 1, 10, "1.0", Equal); // log_4(4) = 1
    test(1, 1, 4, 1, 10, "0.5", Equal); // log_4(2) = 1/2
    test(1, 2, 3, 2, 10, "1.0", Equal); // log_{3/2}(3/2) = 1
    test(-3, 4, 4, 1, 10, "-1.0", Equal); // log_4(1/4) = -1
    test(1, 1, 3, 1, 20, "0.63093", Greater); // log_3(2)
    test(0, 1, 3, 1, 10, "0.0", Equal); // log_3(1) = 0
}

#[test]
fn test_log_base_rational_base_1_plus_x_round() {
    let test = |n: i64, d: u64, bn: u64, bd: u64, rm: RoundingMode, out: &str, o_out| {
        let x = Float::exact_from(Rational::from_signeds(n, i64::exact_from(d)));
        let base = Rational::from_unsigneds(bn, bd);

        let (log, o) = x.clone().log_base_rational_base_1_plus_x_round(&base, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_rational_base_1_plus_x_round_ref(&base, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) = rug_log_base_rational_base_1_plus_x_round(
                &rug::Float::exact_from(&x),
                &base,
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log)),
                ComparableFloatRef(&log),
            );
            assert_eq!(rug_o, o);
        }
    };
    // Exact results that fit in the input's precision (across all rounding modes).
    test(8, 1, 3, 1, Floor, "2.0", Equal); // log_3(9) = 2 (2.0 is representable at x = 8's precision)
    test(8, 1, 3, 1, Ceiling, "2.0", Equal);
    test(8, 1, 3, 1, Nearest, "2.0", Equal);
    test(8, 1, 3, 1, Down, "2.0", Equal);
    test(8, 1, 3, 1, Up, "2.0", Equal);
    test(8, 1, 3, 1, Exact, "2.0", Equal);
    test(3, 1, 4, 1, Exact, "1.0", Equal); // log_4(4) = 1
    test(1, 1, 4, 1, Exact, "0.5", Equal); // log_4(2) = 1/2
    test(1, 2, 3, 2, Exact, "1.0", Equal); // log_{3/2}(3/2) = 1
    // Specials (the input's precision is irrelevant).
    test(0, 1, 3, 1, Exact, "0.0", Equal); // log_3(1) = 0
    test(-2, 1, 3, 1, Nearest, "NaN", Equal); // x < -1
}

#[test]
fn test_log_base_rational_base_1_plus_x() {
    // The `LogBaseOf1PlusX<Rational>` trait: rounds to the input's precision, to nearest.
    let test = |n: i64, d: u64, bn: u64, bd: u64, out: &str| {
        let x = Float::exact_from(Rational::from_signeds(n, i64::exact_from(d)));
        let base = Rational::from_unsigneds(bn, bd);

        let log = x.clone().log_base_1_plus_x(base.clone());
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);

        let log_alt = (&x).log_base_1_plus_x(&base);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

        let mut x_alt = x.clone();
        x_alt.log_base_1_plus_x_assign(&base);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_rational_base_1_plus_x(
                &rug::Float::exact_from(&x),
                &base
            ))),
            ComparableFloatRef(&log),
        );
    };
    test(8, 1, 3, 1, "2.0"); // log_3(9) = 2
    test(3, 1, 4, 1, "1.0"); // log_4(4) = 1
    test(1, 1, 4, 1, "0.5"); // log_4(2) = 1/2
    test(-3, 4, 4, 1, "-1.0"); // log_4(1/4) = -1
}

fn log_base_rational_base_1_plus_x_prec_round_properties_helper(
    x: &Float,
    base: &Rational,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (log, o) = check(x, base, prec, rm, extreme);

    // When the base is a `u64` integer, the result must match the independently-implemented
    // integer-base `log_base_1_plus_x` (which computes log_2(1 + x) / log_2(base) for an integer
    // base, with its own exact detection and dispatch).
    if let Ok(n) = u64::try_from(base) {
        let (alt, o_alt) = x.log_base_1_plus_x_prec_round_ref(n, prec, rm);
        assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    // The sign of the result follows whether 1 + x is above or below 1: x > 0 gives a positive log,
    // -1 < x < 0 gives a negative log.
    if log.is_normal() {
        if *x > 0u32 && o > Less {
            assert!(log > 0u32);
        } else if *x < 0u32 && *x > -1i32 && o < Greater {
            assert!(log < 0u32);
        }
    }

    // An exact result does not depend on the rounding mode; an inexact one cannot be produced with
    // `Exact`.
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (alt, o_alt) = x.log_base_rational_base_1_plus_x_prec_round_ref(base, prec, rm);
            assert_eq!(
                ComparableFloat(alt.abs_negative_zero()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, Equal);
        }
    } else {
        assert_panic!(x.log_base_rational_base_1_plus_x_prec_round_ref(base, prec, Exact));
    }
}

#[test]
fn log_base_rational_base_1_plus_x_prec_round_properties() {
    float_rational_unsigned_rounding_mode_quadruple_gen_var_13().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_rational_base_1_plus_x_prec_round_properties_helper(
                &x, &base, prec, rm, false,
            );
        },
    );
    float_rational_unsigned_rounding_mode_quadruple_gen_var_14().test_properties_with_limit(
        20,
        |(x, base, prec, rm)| {
            log_base_rational_base_1_plus_x_prec_round_properties_helper(&x, &base, prec, rm, true);
        },
    );

    // The special cases hold for every base > 1, precision, and rounding mode.
    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        for base in [Rational::from(3u32), Rational::from_unsigneds(3u32, 2)] {
            let f = |x: Float| x.log_base_rational_base_1_plus_x_prec_round(&base, prec, rm);
            assert!(f(Float::NAN).0.is_nan());
            assert_eq!(f(Float::INFINITY), (Float::INFINITY, Equal));
            assert!(f(Float::NEGATIVE_INFINITY).0.is_nan());
            assert_eq!(f(Float::ZERO), (Float::ZERO, Equal));
            assert_eq!(f(-Float::ZERO), (-Float::ZERO, Equal));
            assert_eq!(f(Float::NEGATIVE_ONE), (Float::NEGATIVE_INFINITY, Equal));
            assert!(f(Float::from(-2)).0.is_nan());
        }
    });
}

#[test]
fn log_base_rational_base_1_plus_x_prec_properties() {
    let f = |x: Float, base: Rational, prec: u64, extreme: bool| {
        let (log, o) = x.clone().log_base_rational_base_1_plus_x_prec(&base, prec);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_rational_base_1_plus_x_prec_ref(&base, prec);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, &base, prec, Nearest, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_rational_unsigned_rounding_mode_quadruple_gen_var_13()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, false));
    float_rational_unsigned_rounding_mode_quadruple_gen_var_14()
        .test_properties_with_limit(20, |(x, base, prec, _rm)| f(x, base, prec, true));
}

#[test]
fn log_base_rational_base_1_plus_x_round_properties() {
    let f = |x: Float, base: Rational, rm: RoundingMode, extreme: bool| {
        let (log, o) = x.clone().log_base_rational_base_1_plus_x_round(&base, rm);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_rational_base_1_plus_x_round_ref(&base, rm);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, &base, x.significant_bits(), rm, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_rational_rounding_mode_triple_gen_var_14()
        .test_properties_with_limit(20, |(x, base, rm)| f(x, base, rm, false));
    float_rational_rounding_mode_triple_gen_var_15()
        .test_properties_with_limit(20, |(x, base, rm)| f(x, base, rm, true));
}

#[test]
fn log_base_rational_base_1_plus_x_properties() {
    let f = |x: Float, base: Rational, extreme: bool| {
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
    float_rational_rounding_mode_triple_gen_var_14()
        .test_properties_with_limit(20, |(x, base, _rm)| f(x, base, false));
    float_rational_rounding_mode_triple_gen_var_15()
        .test_properties_with_limit(20, |(x, base, _rm)| f(x, base, true));
}

#[test]
fn log_base_rational_base_1_plus_x_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::from(8).log_base_rational_base_1_plus_x_prec_round(
        &Rational::from(3u32),
        0,
        Nearest
    ));
    // Base must be greater than 1.
    assert_panic!(Float::from(8).log_base_rational_base_1_plus_x_prec_round(
        &Rational::ONE,
        10,
        Nearest
    ));
    assert_panic!(Float::from(8).log_base_rational_base_1_plus_x_prec_round(
        &Rational::from_unsigneds(1u32, 2),
        10,
        Nearest
    ));
    // Exact is not allowed when the result is not exactly representable.
    assert_panic!(Float::from(1).log_base_rational_base_1_plus_x_prec_round(
        &Rational::from(3u32),
        10,
        Exact
    ));
}
