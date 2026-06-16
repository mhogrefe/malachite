// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    LogBase2Of1PlusX, LogBasePowerOf2Of1PlusX, LogBasePowerOf2Of1PlusXAssign, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::signed_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::arithmetic::log_base_power_of_2_1_plus_x::{
    rug_log_base_power_of_2_1_plus_x, rug_log_base_power_of_2_1_plus_x_prec,
    rug_log_base_power_of_2_1_plus_x_prec_round, rug_log_base_power_of_2_1_plus_x_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_signed_rounding_mode_triple_gen_var_9, float_signed_rounding_mode_triple_gen_var_10,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_8,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use std::panic::catch_unwind;

#[test]
fn test_log_base_power_of_2_1_plus_x() {
    let test = |s, s_hex, pow: i64, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let log = x.clone().log_base_power_of_2_1_plus_x(pow);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);

        let log_alt = (&x).log_base_power_of_2_1_plus_x(pow);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));

        let mut log_alt = x.clone();
        log_alt.log_base_power_of_2_1_plus_x_assign(pow);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));

        // log_{2^pow}(1 + x) = log_2(1 + x) / pow, with the result's signed zero ignored.
        assert_eq!(
            ComparableFloatRef(
                &Float::from(&rug_log_base_power_of_2_1_plus_x(
                    &rug::Float::exact_from(&x),
                    pow
                ))
                .abs_negative_zero()
            ),
            ComparableFloatRef(&log.abs_negative_zero_ref())
        );
    };
    test("NaN", "NaN", 1, "NaN", "NaN");
    test("NaN", "NaN", -1, "NaN", "NaN");
    test("Infinity", "Infinity", 1, "Infinity", "Infinity");
    test("Infinity", "Infinity", -1, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 1, "NaN", "NaN");
    test("-Infinity", "-Infinity", -1, "NaN", "NaN");
    test("0.0", "0x0.0", 1, "0.0", "0x0.0");
    test("0.0", "0x0.0", -1, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", 1, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", -1, "0.0", "0x0.0");
    test("-1.0", "-0x1.0#1", 1, "-Infinity", "-Infinity");
    test("-1.0", "-0x1.0#1", -1, "Infinity", "Infinity");
    test("-1.0", "-0x1.0#1", 2, "-Infinity", "-Infinity");
    // 1 + x = 2^m, exact when m / pow is representable
    test("7.0", "0x7.0#3", 2, "1.5", "0x1.8#3"); // log_4(8) = 3/2, exact
    test("-0.5", "-0x0.8#1", 2, "-0.5", "-0x0.8#1"); // log_4(0.5) = -1/2, exact
    test("3.0", "0x3.0#2", 1, "2.0", "0x2.0#2"); // log_2(4) = 2
    // 1 + x = 2^m, inexact (m / pow not representable)
    test("3.0", "0x3.0#2", 3, "0.8", "0x0.c#2"); // log_8(4) = 2/3, inexact
    test("123.0", "0x7b.0#7", 1, "6.94", "0x6.f#7");
    test("123.0", "0x7b.0#7", 2, "3.47", "0x3.78#7");
    test("123.0", "0x7b.0#7", -1, "-6.94", "-0x6.f#7");
    test("-123.0", "-0x7b.0#7", 2, "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        "1.0250928321213835",
        "0x1.066c7bdd534ad#53",
    );
}

// Exact, huge, and near-power-of-2 special cases, driven by `Float` value (so their hex need not be
// written by hand). The huge near-power-of-2 cases exercise the inner `log_base_2_1_plus_x`
// `_special` routine, and are verified against the oracle rather than by hand.
#[test]
fn test_log_base_power_of_2_1_plus_x_prec_round_special() {
    let test =
        |x: Float, pow: i64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out| {
            let (log, o) = x
                .clone()
                .log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
            assert!(log.is_valid());

            assert_eq!(log.to_string(), out);
            assert_eq!(to_hex_string(&log), out_hex);
            assert_eq!(o, o_out);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_log, rug_o) = rug_log_base_power_of_2_1_plus_x_prec_round(
                    &rug::Float::exact_from(&x),
                    pow,
                    prec,
                    rug_rm,
                );
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
                    ComparableFloatRef(&log.abs_negative_zero_ref()),
                );
                assert_eq!(rug_o, o);
            }
        };
    // exact: 1 + x = 2^m -> m / pow when representable
    test(Float::from(7), 2, 2, Exact, "1.5", "0x1.8#2", Equal); // log_4(8) = 3/2
    test(
        -Float::power_of_2(-1i64),
        2,
        1,
        Exact,
        "-0.5",
        "-0x0.8#1",
        Equal,
    ); // log_4(0.5) = -1/2
    test(Float::from(3), 1, 2, Exact, "2.0", "0x2.0#2", Equal); // log_2(4) = 2
    test(
        Float::exact_from(1023),
        5,
        10,
        Exact,
        "2.0",
        "0x2.00#10",
        Equal,
    ); // log_32(1024) = 2
    // inexact: log_8(4) = 2/3
    test(
        Float::from(3),
        3,
        10,
        Nearest,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    // huge x = 2^k: 1 + x is astronomically close to 2^k, so the inner `_special` routine fires.
    // result is k / pow rounded. Verified against the oracle.
    test(
        Float::power_of_2(1000i64),
        2,
        10,
        Nearest,
        "500.0",
        "0x1f4.0#10",
        Less,
    );
    test(
        Float::power_of_2(1000i64),
        8,
        10,
        Nearest,
        "125.0",
        "0x7d.0#10",
        Less,
    );
    test(
        Float::power_of_2(1000i64),
        -2,
        10,
        Nearest,
        "-500.0",
        "-0x1f4.0#10",
        Greater,
    );
    // 2^1000 with pow = 1000: log_{2^1000}(1 + 2^1000) is just above 1, rounds to 1.
    test(
        Float::power_of_2(1000i64),
        1000,
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Less,
    );
    // tiny x = 2^-1000: result ~ x / (pow * ln 2)
    test(
        Float::power_of_2(-1000i64),
        2,
        10,
        Nearest,
        "6.735e-302",
        "0xb.8cE-251#10",
        Greater,
    );
}

#[test]
fn test_log_base_power_of_2_1_plus_x_prec() {
    let test = |s, s_hex, pow: i64, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log, o) = x.clone().log_base_power_of_2_1_plus_x_prec(pow, prec);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_prec_ref(pow, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_power_of_2_1_plus_x_prec_assign(pow, prec);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let (rug_log, rug_o) =
            rug_log_base_power_of_2_1_plus_x_prec(&rug::Float::exact_from(&x), pow, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, 1, "Infinity", "Infinity", Equal);
    test(
        "Infinity",
        "Infinity",
        -1,
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", -2, 1, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, 10, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", -2, 10, "Infinity", "Infinity", Equal);
    test("7.0", "0x7.0#3", 2, 2, "1.5", "0x1.8#2", Equal);
    test("7.0", "0x7.0#3", 2, 10, "1.5", "0x1.800#10", Equal);
    test("3.0", "0x3.0#2", 3, 10, "0.667", "0x0.aac#10", Greater);
    test("123.0", "0x7b.0#7", 2, 10, "3.477", "0x3.7a#10", Less);
    test("-123.0", "-0x7b.0#7", 2, 10, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        2,
        10,
        "1.025",
        "0x1.068#10",
        Greater,
    );
}

#[test]
fn log_base_power_of_2_1_plus_x_prec_fail() {
    assert_panic!(Float::NAN.log_base_power_of_2_1_plus_x_prec(1, 0));
    assert_panic!(Float::NAN.log_base_power_of_2_1_plus_x_prec_ref(1, 0));
    assert_panic!(Float::NAN.log_base_power_of_2_1_plus_x_prec(0, 1));
    assert_panic!(Float::NAN.log_base_power_of_2_1_plus_x_prec_ref(0, 1));
    assert_panic!({
        let mut x = Float::NAN;
        x.log_base_power_of_2_1_plus_x_prec_assign(1, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.log_base_power_of_2_1_plus_x_prec_assign(0, 1)
    });
}

#[test]
fn test_log_base_power_of_2_1_plus_x_round() {
    let test = |s, s_hex, pow: i64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log, o) = x.clone().log_base_power_of_2_1_plus_x_round(pow, rm);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_round_ref(pow, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_power_of_2_1_plus_x_round_assign(pow, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) =
                rug_log_base_power_of_2_1_plus_x_round(&rug::Float::exact_from(&x), pow, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
                ComparableFloatRef(&log.abs_negative_zero_ref()),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        -1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", -1, Floor, "-0.0", "-0x0.0", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("7.0", "0x7.0#3", 2, Exact, "1.5", "0x1.8#3", Equal);
    test("-0.5", "-0x0.8#1", 2, Exact, "-0.5", "-0x0.8#1", Equal);
    test("123.0", "0x7b.0#7", 2, Floor, "3.47", "0x3.78#7", Less);
    test("123.0", "0x7b.0#7", 2, Ceiling, "3.5", "0x3.80#7", Greater);
    test("123.0", "0x7b.0#7", 2, Down, "3.47", "0x3.78#7", Less);
    test("123.0", "0x7b.0#7", 2, Up, "3.5", "0x3.80#7", Greater);
    test("123.0", "0x7b.0#7", 2, Nearest, "3.47", "0x3.78#7", Less);
}

#[test]
fn log_base_power_of_2_1_plus_x_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.log_base_power_of_2_1_plus_x_round(0, Floor));
    assert_panic!(THREE.log_base_power_of_2_1_plus_x_round_ref(0, Floor));
    // - 1 + 3 = 4 = 2^2, but log_8(4) = 2/3 is not representable
    assert_panic!(THREE.log_base_power_of_2_1_plus_x_round(3, Exact));
    assert_panic!(THREE.log_base_power_of_2_1_plus_x_round_ref(3, Exact));
    assert_panic!({
        let mut x = THREE;
        x.log_base_power_of_2_1_plus_x_round_assign(3, Exact);
    });
    // - 1 + 123 = 124 is not a power of 2, so the base-2 logarithm is irrational
    let x = parse_hex_string("0x7b.0#7");
    assert_panic!(x.clone().log_base_power_of_2_1_plus_x_round(2, Exact));
    assert_panic!(x.log_base_power_of_2_1_plus_x_round_ref(2, Exact));
}

#[test]
fn test_log_base_power_of_2_1_plus_x_prec_round() {
    let test = |s, s_hex, pow: i64, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log, o) = x
            .clone()
            .log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        assert!(log.is_valid());

        assert_eq!(log.to_string(), out);
        assert_eq!(to_hex_string(&log), out_hex);
        assert_eq!(o, o_out);

        let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        let mut log_alt = x.clone();
        let o_alt = log_alt.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, rm);
        assert!(log_alt.is_valid());
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log, rug_o) = rug_log_base_power_of_2_1_plus_x_prec_round(
                &rug::Float::exact_from(&x),
                pow,
                prec,
                rug_rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
                ComparableFloatRef(&log.abs_negative_zero_ref()),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, 1, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        -1,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-Infinity", "-Infinity", 1, 1, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", -1, 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 2, 1, Floor, "-0.0", "-0x0.0", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("7.0", "0x7.0#3", 2, 2, Exact, "1.5", "0x1.8#2", Equal);
    test("-0.5", "-0x0.8#1", 2, 1, Exact, "-0.5", "-0x0.8#1", Equal);
    test("3.0", "0x3.0#2", 1, 2, Exact, "2.0", "0x2.0#2", Equal);
    test("4.0", "0x4.0#1", 3, 1, Floor, "0.5", "0x0.8#1", Less);
    test("4.0", "0x4.0#1", 3, 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test(
        "3.0",
        "0x3.0#2",
        3,
        10,
        Nearest,
        "0.667",
        "0x0.aac#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        2,
        10,
        Floor,
        "3.477",
        "0x3.7a#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        2,
        10,
        Ceiling,
        "3.48",
        "0x3.7b#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        -2,
        10,
        Floor,
        "-3.48",
        "-0x3.7b#10",
        Less,
    );
    test("-123.0", "-0x7b.0#7", 2, 10, Floor, "NaN", "NaN", Equal);
}

// log_{2^pow}(1 + x) can underflow: log_2(1 + x) is always representable, but dividing by pow (with
// |pow| > 1) can push the result below MIN_EXPONENT. The smallest positive Float x = 2^-2^30 has
// log_2(1 + x) ~ x / ln 2, just above 2^-2^30, and dividing by |pow| > 1 drops it below the minimum
// exponent. rug's exponent range is wider than Float's, so the property tests can't reach these
// cases; they are locked in here. Behavior matches div_prec_round's per-rounding-mode clamping.
#[test]
fn test_log_base_power_of_2_1_plus_x_prec_round_underflow() {
    let test =
        |x: Float, pow: i64, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out| {
            let (log, o) = x
                .clone()
                .log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
            assert!(log.is_valid());
            assert_eq!(log.to_string(), out);
            assert_eq!(to_hex_string(&log), out_hex);
            assert_eq!(o, o_out);

            let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm);
            assert!(log_alt.is_valid());
            assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&log_alt));
            assert_eq!(o_alt, o);
        };
    let tiny = Float::power_of_2(-1073741824i64);
    // pow = 2: log underflows; Down -> 0, Up/Nearest -> min positive Float.
    test(tiny.clone(), 2, 1, Down, "0.0", "0x0.0", Less);
    test(
        tiny.clone(),
        2,
        1,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        tiny.clone(),
        2,
        1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    // pow = 8: even deeper underflow, Nearest rounds down to 0.
    test(tiny.clone(), 8, 1, Nearest, "0.0", "0x0.0", Less);
    // negative pow: result is negative tiny, clamps to -0 or -min positive Float.
    test(tiny.clone(), -2, 1, Down, "-0.0", "-0x0.0", Greater);
    test(
        tiny.clone(),
        -2,
        1,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    // negative tiny x = -2^-2^30: log_2(1 + x) ~ x / ln 2 < 0, so the result is negative.
    let neg_tiny = -Float::power_of_2(-1073741824i64);
    test(neg_tiny.clone(), 2, 1, Down, "-0.0", "-0x0.0", Greater);
    test(
        neg_tiny,
        2,
        1,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
}

#[test]
fn log_base_power_of_2_1_plus_x_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).log_base_power_of_2_1_plus_x_prec_round(1, 0, Floor));
    assert_panic!(Float::one_prec(1).log_base_power_of_2_1_plus_x_prec_round_ref(1, 0, Floor));
    assert_panic!(Float::one_prec(1).log_base_power_of_2_1_plus_x_prec_round(0, 1, Floor));
    assert_panic!(Float::one_prec(1).log_base_power_of_2_1_plus_x_prec_round_ref(0, 1, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.log_base_power_of_2_1_plus_x_prec_round_assign(1, 0, Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.log_base_power_of_2_1_plus_x_prec_round_assign(0, 1, Floor)
    });

    // - 1 + 123 = 124 is not a power of 2, so the base-2 logarithm is irrational
    let x = parse_hex_string("0x7b.0#7");
    assert_panic!(
        x.clone()
            .log_base_power_of_2_1_plus_x_prec_round(2, 10, Exact)
    );
    assert_panic!(x.log_base_power_of_2_1_plus_x_prec_round_ref(2, 10, Exact));
    // - 1 + 3 = 4 = 2^2, but log_8(4) = 2/3 is not representable
    assert_panic!(THREE.log_base_power_of_2_1_plus_x_prec_round(3, 100, Exact));
    assert_panic!(THREE.log_base_power_of_2_1_plus_x_prec_round_ref(3, 100, Exact));
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_1_plus_x_prec_round_properties_helper(
    x: Float,
    pow: i64,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (log, o) = x
        .clone()
        .log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_power_of_2_1_plus_x_prec_round_assign(pow, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    // rug's exponent range is wider than Float's, so it doesn't reproduce underflow; the clamped
    // cases are covered by unit tests.
    let underflowed = log == 0u32
        || ComparableFloat(log.abs_negative_zero_ref())
            == ComparableFloat(Float::min_positive_value_prec(prec).abs_negative_zero());

    if !underflowed && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) = rug_log_base_power_of_2_1_plus_x_prec_round(
            &rug::Float::exact_from(&x),
            pow,
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    }

    // log_{2^1}(1 + x) == log_2(1 + x)
    if pow == 1 {
        let (log_2_alt, o_alt) = x.log_base_2_1_plus_x_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    if x < -1i32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));

        // Cross-check: pow * log_{2^pow}(1 + x) == log_2(1 + x). The exact value of the result
        // times pow must lie between log_2(1 + x)'s Floor and Ceiling brackets (computed at the
        // same precision), since both are correctly-rounded approximations of the same real number.
        // Skipped when the result underflowed (clamped to the minimum positive value), since then
        // it is no longer a faithful approximation of log_2(1 + x) / pow. Also skipped for extreme
        // inputs, whose `Float`-to-`Rational` conversions produce astronomically large numerators
        // or denominators and would make the test intolerably slow.
        if !extreme && !underflowed {
            let (l2_lo, _) = x.log_base_2_1_plus_x_prec_round_ref(prec, Floor);
            let (l2_hi, _) = x.log_base_2_1_plus_x_prec_round_ref(prec, Ceiling);
            if l2_lo.is_normal() && l2_hi.is_normal() {
                let prod = Rational::exact_from(&log) * Rational::from(pow);
                let r_lo = Rational::exact_from(&l2_lo);
                let r_hi = Rational::exact_from(&l2_hi);
                let (lo, hi) = if r_lo <= r_hi {
                    (r_lo, r_hi)
                } else {
                    (r_hi, r_lo)
                };
                // Allow a generous slack on each side, scaled by the magnitude of the brackets and
                // by |pow|, to account for the independent roundings of the two logarithms (the
                // result is rounded at its own scale, which is |pow| times smaller than log_2(1 +
                // x)).
                let scale = hi.floor_log_base_2_abs().max(lo.floor_log_base_2_abs());
                let slack = Rational::from(pow.unsigned_abs())
                    * Rational::power_of_2(scale - i64::exact_from(prec) + 2);
                assert!(prod >= &lo - &slack);
                assert!(prod <= &hi + &slack);
            }
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, Exact));
    }
}

#[test]
fn log_base_power_of_2_1_plus_x_prec_round_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, pow, prec, rm)| {
            log_base_power_of_2_1_plus_x_prec_round_properties_helper(x, pow, prec, rm, false);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7().test_properties_with_config(
        &config,
        |(x, pow, prec, rm)| {
            log_base_power_of_2_1_plus_x_prec_round_properties_helper(x, pow, prec, rm, false);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, pow, prec, rm)| {
            log_base_power_of_2_1_plus_x_prec_round_properties_helper(x, pow, prec, rm, true);
        },
    );

    signed_rounding_mode_pair_gen_var_3::<i64>().test_properties(|(pow, rm)| {
        let prec = 10;
        let (log, o) = Float::NAN.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::INFINITY.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        if pow > 0 {
            assert_eq!(log, Float::INFINITY);
        } else {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) =
            Float::NEGATIVE_INFINITY.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::ZERO.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        if pow > 0 {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::ZERO));
        } else {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::NEGATIVE_ZERO));
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ZERO.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        if pow > 0 {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::NEGATIVE_ZERO));
        } else {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::ZERO));
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ONE.log_base_power_of_2_1_plus_x_prec_round(pow, prec, rm);
        if pow > 0 {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        } else {
            assert_eq!(log, Float::INFINITY);
        }
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_1_plus_x_prec_properties_helper(x: Float, pow: i64, prec: u64) {
    let (log, o) = x.clone().log_base_power_of_2_1_plus_x_prec(pow, prec);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_prec_ref(pow, prec);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_power_of_2_1_plus_x_prec_assign(pow, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    // rug's exponent range is wider than Float's, so it doesn't reproduce underflow; the clamped
    // cases are covered by unit tests.
    let underflowed = log == 0u32
        || ComparableFloat(log.abs_negative_zero_ref())
            == ComparableFloat(Float::min_positive_value_prec(prec).abs_negative_zero());

    if !underflowed {
        let (rug_log, rug_o) =
            rug_log_base_power_of_2_1_plus_x_prec(&rug::Float::exact_from(&x), pow, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    }

    let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, prec, Nearest);
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if pow == 1 {
        let (log_2_alt, o_alt) = x.log_base_2_1_plus_x_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    if x < -1i32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
    }
}

#[test]
fn log_base_power_of_2_1_plus_x_prec_properties() {
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7().test_properties(
        |(x, pow, prec, _)| {
            log_base_power_of_2_1_plus_x_prec_properties_helper(x, pow, prec);
        },
    );

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_signed_unsigned_rounding_mode_quadruple_gen_var_7().test_properties_with_config(
        &config,
        |(x, pow, prec, _)| {
            log_base_power_of_2_1_plus_x_prec_properties_helper(x, pow, prec);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, pow, prec, _)| {
            log_base_power_of_2_1_plus_x_prec_properties_helper(x, pow, prec);
        },
    );
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_1_plus_x_round_properties_helper(x: Float, pow: i64, rm: RoundingMode) {
    let (log, o) = x.clone().log_base_power_of_2_1_plus_x_round(pow, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = x.log_base_power_of_2_1_plus_x_round_ref(pow, rm);
    assert!(log_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_power_of_2_1_plus_x_round_assign(pow, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    let (log_alt, o_alt) =
        x.log_base_power_of_2_1_plus_x_prec_round_ref(pow, x.significant_bits(), rm);
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    if pow == 1 {
        let (log_2_alt, o_alt) = x.log_base_2_1_plus_x_round_ref(rm);
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    }

    if x < -1i32 {
        assert!(log.is_nan());
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(x.get_prec().unwrap()));
    }

    // rug's exponent range is wider than Float's, so it doesn't reproduce underflow; the clamped
    // cases are covered by unit tests.
    let prec = x.significant_bits();
    let underflowed = log == 0u32
        || ComparableFloat(log.abs_negative_zero_ref())
            == ComparableFloat(Float::min_positive_value_prec(prec).abs_negative_zero());

    if !underflowed && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) =
            rug_log_base_power_of_2_1_plus_x_round(&rug::Float::exact_from(&x), pow, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_power_of_2_1_plus_x_round_ref(pow, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_power_of_2_1_plus_x_round_ref(pow, Exact));
    }
}

#[test]
fn log_base_power_of_2_1_plus_x_round_properties() {
    float_signed_rounding_mode_triple_gen_var_9().test_properties(|(x, pow, rm)| {
        log_base_power_of_2_1_plus_x_round_properties_helper(x, pow, rm);
    });

    float_signed_rounding_mode_triple_gen_var_10().test_properties(|(x, pow, rm)| {
        log_base_power_of_2_1_plus_x_round_properties_helper(x, pow, rm);
    });

    signed_rounding_mode_pair_gen_var_3::<i64>().test_properties(|(pow, rm)| {
        let (log, o) = Float::NAN.log_base_power_of_2_1_plus_x_round(pow, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::INFINITY.log_base_power_of_2_1_plus_x_round(pow, rm);
        if pow > 0 {
            assert_eq!(log, Float::INFINITY);
        } else {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_INFINITY.log_base_power_of_2_1_plus_x_round(pow, rm);
        assert!(log.is_nan());
        assert_eq!(o, Equal);

        let (log, o) = Float::ZERO.log_base_power_of_2_1_plus_x_round(pow, rm);
        if pow > 0 {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::ZERO));
        } else {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::NEGATIVE_ZERO));
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ZERO.log_base_power_of_2_1_plus_x_round(pow, rm);
        if pow > 0 {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::NEGATIVE_ZERO));
        } else {
            assert_eq!(ComparableFloat(log), ComparableFloat(Float::ZERO));
        }
        assert_eq!(o, Equal);

        let (log, o) = Float::NEGATIVE_ONE.log_base_power_of_2_1_plus_x_round(pow, rm);
        if pow > 0 {
            assert_eq!(log, Float::NEGATIVE_INFINITY);
        } else {
            assert_eq!(log, Float::INFINITY);
        }
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_power_of_2_1_plus_x_properties_helper(x: Float, pow: i64) {
    let log = x.clone().log_base_power_of_2_1_plus_x(pow);
    assert!(log.is_valid());

    let log_alt = (&x).log_base_power_of_2_1_plus_x(pow);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

    let mut x_alt = x.clone();
    x_alt.log_base_power_of_2_1_plus_x_assign(pow);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log));

    let log_alt = x
        .log_base_power_of_2_1_plus_x_prec_round_ref(pow, x.significant_bits(), Nearest)
        .0;
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    let log_alt = x
        .log_base_power_of_2_1_plus_x_prec_ref(pow, x.significant_bits())
        .0;
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    let log_alt = x.log_base_power_of_2_1_plus_x_round_ref(pow, Nearest).0;
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));

    if pow == 1 {
        let log_2_alt = (&x).log_base_2_1_plus_x();
        assert_eq!(ComparableFloatRef(&log_2_alt), ComparableFloatRef(&log));
    }

    if x < -1i32 {
        assert!(log.is_nan());
    }

    // rug's exponent range is wider than Float's, so it doesn't reproduce underflow; the clamped
    // cases are covered by unit tests.
    let prec = x.significant_bits();
    let underflowed = log == 0u32
        || ComparableFloat(log.abs_negative_zero_ref())
            == ComparableFloat(Float::min_positive_value_prec(prec).abs_negative_zero());

    if !underflowed {
        let rug_log = rug_log_base_power_of_2_1_plus_x(&rug::Float::exact_from(&x), pow);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref()),
        );
    }
}

#[test]
fn log_base_power_of_2_1_plus_x_properties() {
    float_signed_rounding_mode_triple_gen_var_9().test_properties(|(x, pow, _)| {
        log_base_power_of_2_1_plus_x_properties_helper(x, pow);
    });

    float_signed_rounding_mode_triple_gen_var_10().test_properties(|(x, pow, _)| {
        log_base_power_of_2_1_plus_x_properties_helper(x, pow);
    });
}
