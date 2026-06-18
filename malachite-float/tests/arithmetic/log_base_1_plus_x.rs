// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, LogBaseOf1PlusX, LogBaseOf1PlusXAssign, PowerOf2,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_unsigned_pair_gen_var_4, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::arithmetic::log_base_1_plus_x::primitive_float_log_base_1_plus_x;
use malachite_float::arithmetic::log_base_power_of_2_1_plus_x::primitive_float_log_base_power_of_2_1_plus_x;
use malachite_float::test_util::arithmetic::log_base_1_plus_x::{
    rug_log_base_1_plus_x, rug_log_base_1_plus_x_prec, rug_log_base_1_plus_x_prec_round,
};
use malachite_float::test_util::common::{rug_round_try_from_rounding_mode, to_hex_string};
use malachite_float::test_util::generators::{
    float_unsigned_rounding_mode_triple_gen_var_32, float_unsigned_rounding_mode_triple_gen_var_33,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_7,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

// Cross-checks the by-value/by-reference/assigning variants, delegation when `base` is a power of
// 2, precision, and (skipping underflow and Exact) the rug oracle. Returns the computed `(Float,
// Ordering)`.
fn check(x: &Float, base: u64, prec: u64, rm: RoundingMode, extreme: bool) -> (Float, Ordering) {
    let (log, o) = x.clone().log_base_1_plus_x_prec_round(base, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_1_plus_x_prec_round_ref(base, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_1_plus_x_prec_round_assign(base, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    // When the base is a power of 2 the result must match log_base_power_of_2_1_plus_x.
    if base.is_power_of_2() {
        let (alt, o2) = x.log_base_power_of_2_1_plus_x_prec_round_ref(
            i64::from(base.trailing_zeros()),
            prec,
            rm,
        );
        assert_eq!(ComparableFloatRef(&alt), ComparableFloatRef(&log));
        assert_eq!(o2, o);
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }

    // Cross-check against the rug oracle, skipped for extreme inputs (whose ln_1p(x) / ln(base)
    // bracketing at large working precision is slow) and for underflow. The underflow skip is not
    // because rug can't reproduce the clamp -- rug's exponent range equals Float's (both are MPFR's
    // defaults, +/-(2^30 - 1)), so it underflows at the same threshold. Rather, underflow only
    // occurs for inputs with exponent near MIN_EXPONENT (x ~ 2^MIN_EXPONENT), and on such inputs
    // the oracle's exact-case fallback reconstructs a Rational from a float of exponent ~ -2^30,
    // i.e. a denominator of ~2^30 ~ 10^9 bits; that bignum arithmetic is intractable and
    // effectively hangs. Underflow correctness is covered by the dedicated underflow unit test.
    let underflowed = log == 0u32
        || ComparableFloat(log.abs_negative_zero_ref())
            == ComparableFloat(Float::min_positive_value_prec(prec).abs_negative_zero());
    if !extreme
        && !underflowed
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rug_log, rug_o) =
            rug_log_base_1_plus_x_prec_round(&rug::Float::exact_from(x), base, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    }
    (log, o)
}

fn log_base_1_plus_x_prec_round_properties_helper(
    x: &Float,
    base: u64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (log, o) = check(x, base, prec, rm, extreme);
    // x < 0 (i.e. 1 + x < 1) gives a negative log; x > 0 gives positive; x = 0 gives 0.
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
            let (alt, o_alt) = x.log_base_1_plus_x_prec_round_ref(base, prec, rm);
            assert_eq!(
                ComparableFloat(alt.abs_negative_zero()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(o_alt, Equal);
        }
    } else {
        assert_panic!(x.log_base_1_plus_x_prec_round_ref(base, prec, Exact));
    }
}

#[test]
fn log_base_1_plus_x_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, base, prec, rm)| {
            log_base_1_plus_x_prec_round_properties_helper(&x, base, prec, rm, false);
        },
    );
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, base, prec, rm)| {
            log_base_1_plus_x_prec_round_properties_helper(&x, base, prec, rm, true);
        },
    );

    // The special cases hold for every base > 1, precision, and rounding mode. (Base 4 exercises
    // the power-of-2 delegation, base 3 the general path.)
    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        for base in [3, 4] {
            let f = |x: Float| x.log_base_1_plus_x_prec_round(base, prec, rm);
            assert!(f(Float::NAN).0.is_nan());
            assert_eq!(f(Float::INFINITY), (Float::INFINITY, Equal));
            assert!(f(Float::NEGATIVE_INFINITY).0.is_nan());
            // log_b(1 + 0) = 0, with the sign of the zero preserved.
            assert_eq!(
                ComparableFloat(f(Float::ZERO).0),
                ComparableFloat(Float::ZERO)
            );
            assert_eq!(
                ComparableFloat(f(-Float::ZERO).0),
                ComparableFloat(-Float::ZERO)
            );
            // 1 + (-1) = 0, so log_b(0) = -infinity.
            assert_eq!(f(Float::NEGATIVE_ONE), (Float::NEGATIVE_INFINITY, Equal));
            // x < -1 is outside the domain.
            assert!(f(Float::from(-2)).0.is_nan());
        }
    });
}

#[test]
fn log_base_1_plus_x_prec_properties() {
    let f = |x: Float, base: u64, prec: u64, extreme: bool| {
        let (log, o) = x.clone().log_base_1_plus_x_prec(base, prec);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_1_plus_x_prec_ref(base, prec);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, base, prec, Nearest, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_7()
        .test_properties(|(x, base, prec, _rm)| f(x, base, prec, false));
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_8()
        .test_properties(|(x, base, prec, _rm)| f(x, base, prec, true));
}

#[test]
fn log_base_1_plus_x_round_properties() {
    let f = |x: Float, base: u64, rm: RoundingMode, extreme: bool| {
        let (log, o) = x.clone().log_base_1_plus_x_round(base, rm);
        assert!(log.is_valid());
        let (log_ref, o_ref) = x.log_base_1_plus_x_round_ref(base, rm);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = check(&x, base, x.significant_bits(), rm, extreme);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    };
    float_unsigned_rounding_mode_triple_gen_var_32()
        .test_properties(|(x, base, rm)| f(x, base, rm, false));
    float_unsigned_rounding_mode_triple_gen_var_33()
        .test_properties(|(x, base, rm)| f(x, base, rm, true));
}

#[test]
fn log_base_1_plus_x_properties() {
    let f = |x: Float, base: u64, extreme: bool| {
        let prec = x.significant_bits();
        let (expected, _) = check(&x, base, prec, Nearest, extreme);
        let log = x.clone().log_base_1_plus_x(base);
        assert!(log.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        let log_ref = (&x).log_base_1_plus_x(base);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        let mut x_alt = x.clone();
        x_alt.log_base_1_plus_x_assign(base);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    };
    float_unsigned_rounding_mode_triple_gen_var_32()
        .test_properties(|(x, base, _rm)| f(x, base, false));
    float_unsigned_rounding_mode_triple_gen_var_33()
        .test_properties(|(x, base, _rm)| f(x, base, true));
}

#[test]
fn log_base_1_plus_x_prec_round_fail() {
    // Precision must be nonzero.
    assert_panic!(Float::from(7).log_base_1_plus_x_prec_round(3, 0, Nearest));
    // Base must be greater than 1.
    assert_panic!(Float::from(7).log_base_1_plus_x_prec_round(1, 10, Nearest));
    assert_panic!(Float::from(7).log_base_1_plus_x_prec_round(0, 10, Nearest));
    // Exact is not allowed when the result is not exactly representable.
    assert_panic!(Float::from(1).log_base_1_plus_x_prec_round(3, 10, Exact));
}

// log_b(1 + x) can underflow: dividing log_2(1 + x) by log_2(base) > 1 can push the result below
// MIN_EXPONENT. rug's exponent range is wider than Float's, so the property tests can't reach
// these; they are locked in here. Behavior matches div_prec_round's per-rounding-mode clamping.
#[test]
fn log_base_1_plus_x_underflow() {
    let test_u =
        |x: Float, base: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let (log, o) = x.clone().log_base_1_plus_x_prec_round(base, 1, rm);
            assert!(log.is_valid());
            assert_eq!(log.to_string(), out);
            assert_eq!(to_hex_string(&log), out_hex);
            assert_eq!(o, o_out);
            let (log_alt, o_alt) = x.log_base_1_plus_x_prec_round_ref(base, 1, rm);
            assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
            assert_eq!(o_alt, o);
        };
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Floor,
        "too_small",
        "0x1.0E-268435456#1",
        Less,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Ceiling,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Nearest,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Down,
        "too_small",
        "0x1.0E-268435456#1",
        Less,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Up,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_u(
        Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Floor,
        "-too_small",
        "-0x2.0E-268435456#1",
        Less,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Ceiling,
        "-too_small",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Nearest,
        "-too_small",
        "-0x2.0E-268435456#1",
        Less,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Down,
        "-too_small",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        3,
        Up,
        "-too_small",
        "-0x2.0E-268435456#1",
        Less,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_u(
        -Float::power_of_2(i64::from(Float::MIN_EXPONENT)),
        100,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
}

#[test]
fn test_log_base_1_plus_x_prec_round() {
    let test =
        |n: i64, d: u64, base: u64, prec: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
            let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));
            let (log, o) = check(&x, base, prec, rm, false);
            assert_eq!(log.to_string(), out);
            assert_eq!(o, o_out);
        };
    test(8, 1, 9, 10, Floor, "1.0", Equal);
    test(8, 1, 9, 10, Ceiling, "1.0", Equal);
    test(8, 1, 9, 10, Nearest, "1.0", Equal);
    test(8, 1, 9, 10, Exact, "1.0", Equal);
    test(8, 1, 3, 10, Floor, "2.0", Equal);
    test(8, 1, 3, 10, Ceiling, "2.0", Equal);
    test(8, 1, 3, 10, Nearest, "2.0", Equal);
    test(8, 1, 3, 10, Exact, "2.0", Equal);
    test(2, 1, 9, 10, Floor, "0.5", Equal);
    test(2, 1, 9, 10, Ceiling, "0.5", Equal);
    test(2, 1, 9, 10, Nearest, "0.5", Equal);
    test(2, 1, 9, 10, Exact, "0.5", Equal);
    test(0, 1, 10, 10, Floor, "0.0", Equal);
    test(0, 1, 10, 10, Ceiling, "0.0", Equal);
    test(0, 1, 10, 10, Nearest, "0.0", Equal);
    test(0, 1, 10, 10, Exact, "0.0", Equal);
    test(1, 1, 3, 20, Floor, "0.630929", Less);
    test(1, 1, 3, 20, Ceiling, "0.63093", Greater);
    test(1, 1, 3, 20, Nearest, "0.63093", Greater);
    test(50, 1, 3, 20, Floor, "3.578899", Less);
    test(50, 1, 3, 20, Ceiling, "3.578903", Greater);
    test(50, 1, 3, 20, Nearest, "3.578903", Greater);
    test(7, 1, 5, 30, Floor, "1.292029673", Less);
    test(7, 1, 5, 30, Ceiling, "1.292029675", Greater);
    test(7, 1, 5, 30, Nearest, "1.292029675", Greater);
    test(1, 8, 3, 20, Floor, "0.1072106", Less);
    test(1, 8, 3, 20, Ceiling, "0.1072108", Greater);
    test(1, 8, 3, 20, Nearest, "0.1072108", Greater);
    test(-1, 2, 3, 20, Floor, "-0.63093", Less);
    test(-1, 2, 3, 20, Ceiling, "-0.630929", Greater);
    test(-1, 2, 3, 20, Nearest, "-0.63093", Less);
    test(-1, 1, 10, 10, Floor, "-Infinity", Equal);
    test(-1, 1, 10, 10, Ceiling, "-Infinity", Equal);
    test(-1, 1, 10, 10, Nearest, "-Infinity", Equal);
    test(-1, 1, 10, 10, Exact, "-Infinity", Equal);
    test(-3, 1, 10, 10, Floor, "NaN", Equal);
    test(-3, 1, 10, 10, Ceiling, "NaN", Equal);
    test(-3, 1, 10, 10, Nearest, "NaN", Equal);
    test(-3, 1, 10, 10, Exact, "NaN", Equal);
    test(26, 1, 3, 30, Floor, "3.0", Equal);
    test(26, 1, 3, 30, Ceiling, "3.0", Equal);
    test(26, 1, 3, 30, Nearest, "3.0", Equal);
    test(26, 1, 3, 30, Exact, "3.0", Equal);
}

#[test]
fn test_log_base_1_plus_x_prec() {
    // The `_prec` methods round to nearest; cross-checked against the rug oracle.
    let test = |n: i64, d: u64, base: u64, prec: u64, out: &str, o_out: Ordering| {
        let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));

        let (log, o) = x.clone().log_base_1_plus_x_prec(base, prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_1_plus_x_prec_ref(base, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_1_plus_x_prec_assign(base, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) = rug_log_base_1_plus_x_prec(&rug::Float::exact_from(&x), base, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    };
    test(8, 1, 9, 10, "1.0", Equal); // log_9(9) = 1
    test(8, 1, 3, 10, "2.0", Equal); // log_3(9) = 2
    test(2, 1, 9, 10, "0.5", Equal); // log_9(3) = 1/2
    test(26, 1, 3, 30, "3.0", Equal); // log_3(27) = 3
    test(0, 1, 10, 10, "0.0", Equal);
    test(1, 1, 3, 20, "0.63093", Greater); // log_3(2)
    test(50, 1, 3, 20, "3.578903", Greater); // log_3(51)
    test(7, 1, 5, 30, "1.292029675", Greater); // log_5(8)
    test(1, 8, 3, 20, "0.1072108", Greater); // log_3(9/8)
    test(-1, 2, 3, 20, "-0.63093", Less); // log_3(1/2)
    test(-1, 1, 10, 10, "-Infinity", Equal);
    test(-3, 1, 10, 10, "NaN", Equal);
}

#[test]
fn test_log_base_1_plus_x() {
    // The `LogBaseOf1PlusX` trait: rounds to the input's precision, to nearest.
    let test = |n: i64, d: u64, base: u64, out: &str| {
        let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));
        let log = x.clone().log_base_1_plus_x(base);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);

        let log_alt = (&x).log_base_1_plus_x(base);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

        let mut x_alt = x.clone();
        x_alt.log_base_1_plus_x_assign(base);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_1_plus_x(
                &rug::Float::exact_from(&x),
                base
            ))),
            ComparableFloatRef(&log),
        );
    };
    test(8, 1, 9, "1.0");
    test(8, 1, 3, "2.0");
    test(2, 1, 9, "0.5");
    test(26, 1, 3, "3.0");
    test(0, 1, 10, "0.0");
    test(50, 1, 3, "3.6");
    test(7, 1, 5, "1.2");
    test(1, 1, 3, "0.5");
    test(1, 8, 3, "0.1");
    test(-1, 2, 3, "-0.5");
    test(-1, 1, 10, "-Infinity");
    test(-3, 1, 10, "NaN");
}

#[test]
fn test_log_base_1_plus_x_round() {
    let test = |n: i64, d: u64, base: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
        let x = Float::exact_from(malachite_q::Rational::from_signeds(n, i64::exact_from(d)));
        // log_base_1_plus_x_round uses the input's precision; `check` cross-checks against the
        // oracle.
        let (log, o) = check(&x, base, x.significant_bits().max(1), rm, false);
        let (log2, o2) = x.clone().log_base_1_plus_x_round(base, rm);
        assert_eq!(ComparableFloatRef(&log2), ComparableFloatRef(&log));
        assert_eq!(o2, o);
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
    };
    test(8, 1, 9, Floor, "1.0", Equal);
    test(8, 1, 9, Ceiling, "1.0", Equal);
    test(8, 1, 9, Nearest, "1.0", Equal);
    test(8, 1, 9, Down, "1.0", Equal);
    test(8, 1, 9, Up, "1.0", Equal);
    test(8, 1, 3, Floor, "2.0", Equal);
    test(8, 1, 3, Ceiling, "2.0", Equal);
    test(8, 1, 3, Nearest, "2.0", Equal);
    test(8, 1, 3, Down, "2.0", Equal);
    test(8, 1, 3, Up, "2.0", Equal);
    test(2, 1, 9, Floor, "0.5", Equal);
    test(2, 1, 9, Ceiling, "0.5", Equal);
    test(2, 1, 9, Nearest, "0.5", Equal);
    test(2, 1, 9, Down, "0.5", Equal);
    test(2, 1, 9, Up, "0.5", Equal);
    test(26, 1, 3, Floor, "3.0", Equal);
    test(26, 1, 3, Ceiling, "3.0", Equal);
    test(26, 1, 3, Nearest, "3.0", Equal);
    test(26, 1, 3, Down, "3.0", Equal);
    test(26, 1, 3, Up, "3.0", Equal);
    test(0, 1, 10, Floor, "0.0", Equal);
    test(0, 1, 10, Ceiling, "0.0", Equal);
    test(0, 1, 10, Nearest, "0.0", Equal);
    test(0, 1, 10, Down, "0.0", Equal);
    test(0, 1, 10, Up, "0.0", Equal);
    test(50, 1, 3, Floor, "3.5", Less);
    test(50, 1, 3, Ceiling, "3.6", Greater);
    test(50, 1, 3, Nearest, "3.6", Greater);
    test(50, 1, 3, Down, "3.5", Less);
    test(50, 1, 3, Up, "3.6", Greater);
    test(7, 1, 5, Floor, "1.2", Less);
    test(7, 1, 5, Ceiling, "1.5", Greater);
    test(7, 1, 5, Nearest, "1.2", Less);
    test(7, 1, 5, Down, "1.2", Less);
    test(7, 1, 5, Up, "1.5", Greater);
    test(1, 1, 3, Floor, "0.5", Less);
    test(1, 1, 3, Ceiling, "1.0", Greater);
    test(1, 1, 3, Nearest, "0.5", Less);
    test(1, 1, 3, Down, "0.5", Less);
    test(1, 1, 3, Up, "1.0", Greater);
    test(1, 8, 3, Floor, "0.06", Less);
    test(1, 8, 3, Ceiling, "0.1", Greater);
    test(1, 8, 3, Nearest, "0.1", Greater);
    test(1, 8, 3, Down, "0.06", Less);
    test(1, 8, 3, Up, "0.1", Greater);
    test(-1, 2, 3, Floor, "-1.0", Less);
    test(-1, 2, 3, Ceiling, "-0.5", Greater);
    test(-1, 2, 3, Nearest, "-0.5", Greater);
    test(-1, 2, 3, Down, "-0.5", Greater);
    test(-1, 2, 3, Up, "-1.0", Less);
    test(-1, 1, 10, Floor, "-Infinity", Equal);
    test(-1, 1, 10, Ceiling, "-Infinity", Equal);
    test(-1, 1, 10, Nearest, "-Infinity", Equal);
    test(-1, 1, 10, Down, "-Infinity", Equal);
    test(-1, 1, 10, Up, "-Infinity", Equal);
    test(-3, 1, 10, Floor, "NaN", Equal);
    test(-3, 1, 10, Ceiling, "NaN", Equal);
    test(-3, 1, 10, Nearest, "NaN", Equal);
    test(-3, 1, 10, Down, "NaN", Equal);
    test(-3, 1, 10, Up, "NaN", Equal);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_log_base_1_plus_x() {
    fn test<T: PrimitiveFloat>(x: T, base: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_log_base_1_plus_x(x, base)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 10, f32::NAN);
    test::<f32>(f32::INFINITY, 10, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 10, f32::NAN);
    test::<f32>(0.0, 10, 0.0);
    test::<f32>(-0.0, 10, -0.0);
    test::<f32>(-1.0, 10, f32::NEGATIVE_INFINITY);
    test::<f32>(-2.0, 10, f32::NAN);
    test::<f32>(999.0, 10, 3.0); // log_10(1000)
    test::<f32>(8.0, 9, 1.0); // log_9(9)
    test::<f32>(80.0, 9, 2.0); // log_9(81)
    test::<f32>(7.0, 2, 3.0); // log_2(8), power-of-2 base
    test::<f32>(1.0, 3, 0.63092977); // log_3(2)
    test::<f32>(49.0, 10, 1.6989699602127075); // log_10(50)
    test::<f32>(9.0, 3, 2.095903158187866); // log_3(10)

    test::<f64>(f64::NAN, 10, f64::NAN);
    test::<f64>(f64::INFINITY, 10, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, 10, f64::NAN);
    test::<f64>(0.0, 10, 0.0);
    test::<f64>(-0.0, 10, -0.0);
    test::<f64>(-1.0, 10, f64::NEGATIVE_INFINITY);
    test::<f64>(-2.0, 10, f64::NAN);
    test::<f64>(999.0, 10, 3.0); // log_10(1000)
    test::<f64>(8.0, 9, 1.0); // log_9(9)
    test::<f64>(80.0, 9, 2.0); // log_9(81)
    test::<f64>(7.0, 2, 3.0); // log_2(8), power-of-2 base
    test::<f64>(1.0, 3, 0.6309297535714574); // log_3(2)
    test::<f64>(49.0, 10, 1.6989700043360187); // log_10(50)
    test::<f64>(9.0, 3, 2.0959032742893844); // log_3(10)
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_log_base_1_plus_x_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_4::<T, u64>().test_properties(|(x, base)| {
        if base < 2 {
            return;
        }
        let y = primitive_float_log_base_1_plus_x(x, base);
        // When the base is a power of 2, log_b(1 + x) agrees with log_base_power_of_2_1_plus_x.
        if base.is_power_of_2() {
            assert_eq!(
                NiceFloat(y),
                NiceFloat(primitive_float_log_base_power_of_2_1_plus_x(
                    x,
                    i64::exact_from(base.trailing_zeros())
                ))
            );
        }
    });
}

#[test]
fn primitive_float_log_base_1_plus_x_properties() {
    apply_fn_to_primitive_floats!(primitive_float_log_base_1_plus_x_properties_helper);
}
