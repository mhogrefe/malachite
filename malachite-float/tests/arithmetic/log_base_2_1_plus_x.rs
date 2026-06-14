// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{LogBase2Of1PlusX, LogBase2Of1PlusXAssign, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    rounding_mode_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::test_util::arithmetic::log_base_2_1_plus_x::{
    rug_log_base_2_1_plus_x, rug_log_base_2_1_plus_x_prec, rug_log_base_2_1_plus_x_prec_round,
    rug_log_base_2_1_plus_x_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_40,
    float_rounding_mode_pair_gen_var_41, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_25,
    float_unsigned_rounding_mode_triple_gen_var_26,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use std::panic::catch_unwind;

#[test]
fn test_log_base_2_1_plus_x() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let log_base_2_1_plus_x = x.clone().log_base_2_1_plus_x();
        assert!(log_base_2_1_plus_x.is_valid());

        assert_eq!(log_base_2_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2_1_plus_x), out_hex);

        let log_base_2_1_plus_x_alt = (&x).log_base_2_1_plus_x();
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );

        let mut log_base_2_1_plus_x_alt = x.clone();
        log_base_2_1_plus_x_alt.log_base_2_1_plus_x_assign();
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x(
                &rug::Float::exact_from(&x)
            ))),
            ComparableFloatRef(&log_base_2_1_plus_x)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "-Infinity", "-Infinity");
    test("-0.5", "-0x0.8#1", "-1.0", "-0x1.0#1");
    test("-0.8", "-0x0.c#2", "-2.0", "-0x2.0#2");
    test("3.0", "0x3.0#2", "2.0", "0x2.0#2");
    test("7.0", "0x7.0#3", "3.0", "0x3.0#3");
    test("123.0", "0x7b.0#7", "6.94", "0x6.f#7");
    test("-123.0", "-0x7b.0#7", "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "2.0501856642427669",
        "0x2.0cd8f7baa695a#53",
    );
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "NaN", "NaN");
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "1.8946361239720115",
        "0x1.e506df7e47ad8#53",
    );
}

// Exact, huge, and tiny special cases, driven by `Float` value (so their hex need not be written by
// hand).
#[test]
fn test_log_base_2_1_plus_x_prec_round_special() {
    let test =
        |x: Float, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let (log_base_2_1_plus_x, o) = x.clone().log_base_2_1_plus_x_prec_round(prec, rm);
            assert!(log_base_2_1_plus_x.is_valid());

            assert_eq!(log_base_2_1_plus_x.to_string(), out);
            assert_eq!(to_hex_string(&log_base_2_1_plus_x), out_hex);
            assert_eq!(o, o_out);

            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_log_base_2_1_plus_x, rug_o) =
                    rug_log_base_2_1_plus_x_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
                    ComparableFloatRef(&log_base_2_1_plus_x),
                );
                assert_eq!(rug_o, o);
            }
        };
    // exact: x = 2^k - 1 -> k
    test(Float::ONE, 1, Exact, "1.0", "0x1.0#1", Equal);
    test(Float::from(3), 2, Exact, "2.0", "0x2.0#2", Equal);
    test(Float::from(7), 3, Exact, "3.0", "0x3.0#3", Equal);
    test(Float::exact_from(31), 5, Exact, "5.0", "0x5.0#5", Equal);
    test(
        Float::exact_from(1023),
        10,
        Exact,
        "10.0",
        "0xa.00#10",
        Equal,
    );
    // exact negative: x = -(1 - 2^-k) -> -k
    test(
        -Float::power_of_2(-1i64),
        1,
        Exact,
        "-1.0",
        "-0x1.0#1",
        Equal,
    );
    test(
        Float::NEGATIVE_ONE.add_prec(Float::power_of_2(-2i64), 2).0,
        2,
        Exact,
        "-2.0",
        "-0x2.0#2",
        Equal,
    );
    test(
        Float::NEGATIVE_ONE.add_prec(Float::power_of_2(-3i64), 3).0,
        3,
        Exact,
        "-3.0",
        "-0x3.0#3",
        Equal,
    );
    // exact k that does not fit the target precision: 1 + x = 2^5 = 32, result 5 at prec 1, 2
    test(Float::exact_from(31), 1, Nearest, "4.0", "0x4.0#1", Less);
    test(Float::exact_from(31), 2, Nearest, "4.0", "0x4.0#2", Less);
    // huge x = 2^k: result is k rounded
    test(
        Float::power_of_2(60i64),
        60,
        Nearest,
        "60.0",
        "0x3c.00000000000000#60",
        Less,
    );
    test(
        Float::power_of_2(100i64),
        100,
        Nearest,
        "100.0",
        "0x64.000000000000000000000000#100",
        Less,
    );
    test(
        Float::power_of_2(1000i64),
        1000,
        Nearest,
        "1000.0",
        "0x3e8.000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00000000000000000000000000000000000000000000000000000000000000000000000000#1000",
        Less,
    );
    test(
        Float::power_of_2(60i64),
        4,
        Nearest,
        "60.0",
        "0x3c.0#4",
        Less,
    );
    // tiny x = 2^-1000: result ~ x / ln 2
    test(
        Float::power_of_2(-1000i64),
        10,
        Nearest,
        "1.347e-301",
        "0x1.718E-250#10",
        Greater,
    );
    // x just above -1
    test(
        Float::NEGATIVE_ONE
            .add_prec(Float::power_of_2(-100i64), 100)
            .0,
        10,
        Nearest,
        "-100.0",
        "-0x64.0#10",
        Equal,
    );
}

#[test]
fn test_log_base_2_1_plus_x_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_2_1_plus_x, o) = x.clone().log_base_2_1_plus_x_prec(prec);
        assert!(log_base_2_1_plus_x.is_valid());

        assert_eq!(log_base_2_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2_1_plus_x), out_hex);
        assert_eq!(o, o_out);

        let (log_base_2_1_plus_x_alt, o_alt) = x.log_base_2_1_plus_x_prec_ref(prec);
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut log_base_2_1_plus_x_alt = x.clone();
        let o_alt = log_base_2_1_plus_x_alt.log_base_2_1_plus_x_prec_assign(prec);
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let (rug_log_base_2_1_plus_x, rug_o) =
            rug_log_base_2_1_plus_x_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
            ComparableFloatRef(&log_base_2_1_plus_x),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 1, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", 10, "-Infinity", "-Infinity", Equal);
    test("123.0", "0x7b.0#7", 1, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 10, "6.953", "0x6.f4#10", Less);
    test("-123.0", "-0x7b.0#7", 1, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "2.051",
        "0x2.0d#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "1.895",
        "0x1.e50#10",
        Less,
    );
    test("2.0", "0x2.0#1", 1, "2.0", "0x2.0#1", Greater);
    test("0.999998", "0x0.ffffe#19", 5, "1.0", "0x1.0#5", Greater);
}

#[test]
fn log_base_2_1_plus_x_prec_fail() {
    assert_panic!(Float::NAN.log_base_2_1_plus_x_prec(0));
    assert_panic!(Float::NAN.log_base_2_1_plus_x_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.log_base_2_1_plus_x_prec_assign(0)
    });
}

#[test]
fn test_log_base_2_1_plus_x_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_2_1_plus_x, o) = x.clone().log_base_2_1_plus_x_round(rm);
        assert!(log_base_2_1_plus_x.is_valid());

        assert_eq!(log_base_2_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2_1_plus_x), out_hex);
        assert_eq!(o, o_out);

        let (log_base_2_1_plus_x_alt, o_alt) = x.log_base_2_1_plus_x_round_ref(rm);
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut log_base_2_1_plus_x_alt = x.clone();
        let o_alt = log_base_2_1_plus_x_alt.log_base_2_1_plus_x_round_assign(rm);
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_2_1_plus_x, rug_o) =
                rug_log_base_2_1_plus_x_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
                ComparableFloatRef(&log_base_2_1_plus_x),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal);

    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Exact, "1.0", "0x1.0#1", Equal);

    test("-1.0", "-0x1.0#1", Floor, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", Exact, "-Infinity", "-Infinity", Equal);

    test("123.0", "0x7b.0#7", Floor, "6.94", "0x6.f#7", Less);
    test("123.0", "0x7b.0#7", Ceiling, "7.0", "0x7.0#7", Greater);
    test("123.0", "0x7b.0#7", Down, "6.94", "0x6.f#7", Less);
    test("123.0", "0x7b.0#7", Up, "7.0", "0x7.0#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "6.94", "0x6.f#7", Less);

    test("-123.0", "-0x7b.0#7", Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Nearest, "NaN", "NaN", Equal);

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "2.0501856642427669",
        "0x2.0cd8f7baa695a#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "2.0501856642427674",
        "0x2.0cd8f7baa695c#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "2.0501856642427669",
        "0x2.0cd8f7baa695a#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "2.0501856642427674",
        "0x2.0cd8f7baa695c#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "2.0501856642427669",
        "0x2.0cd8f7baa695a#53",
        Less,
    );

    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "1.8946361239720113",
        "0x1.e506df7e47ad7#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "1.8946361239720115",
        "0x1.e506df7e47ad8#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "1.8946361239720115",
        "0x1.e506df7e47ad8#53",
        Greater,
    );
}

#[test]
fn log_base_2_1_plus_x_round_fail() {
    // log2(1 + 0.5) = log2(1.5) is irrational, so Exact rounding panics.
    let x = parse_hex_string("0x0.8#1");
    assert_panic!(x.clone().log_base_2_1_plus_x_round(Exact));
    assert_panic!(x.log_base_2_1_plus_x_round_ref(Exact));
    assert_panic!({
        let mut x = parse_hex_string("0x0.8#1");
        x.log_base_2_1_plus_x_round_assign(Exact);
    });
}

#[test]
fn test_log_base_2_1_plus_x_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_2_1_plus_x, o) = x.clone().log_base_2_1_plus_x_prec_round(prec, rm);
        assert!(log_base_2_1_plus_x.is_valid());

        assert_eq!(log_base_2_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2_1_plus_x), out_hex);
        assert_eq!(o, o_out);

        let (log_base_2_1_plus_x_alt, o_alt) = x.log_base_2_1_plus_x_prec_round_ref(prec, rm);
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut log_base_2_1_plus_x_alt = x.clone();
        let o_alt = log_base_2_1_plus_x_alt.log_base_2_1_plus_x_prec_round_assign(prec, rm);
        assert!(log_base_2_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2_1_plus_x),
            ComparableFloatRef(&log_base_2_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_2_1_plus_x, rug_o) =
                rug_log_base_2_1_plus_x_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
                ComparableFloatRef(&log_base_2_1_plus_x),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);

    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.0", "0x1.000#10", Equal);

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
        Nearest,
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
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("123.0", "0x7b.0#7", 1, Floor, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 10, Floor, "6.953", "0x6.f4#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "6.96",
        "0x6.f6#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 10, Nearest, "6.953", "0x6.f4#10", Less);

    test("-123.0", "-0x7b.0#7", 1, Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, Nearest, "NaN", "NaN", Equal);

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "2.047",
        "0x2.0c#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "2.051",
        "0x2.0d#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "2.051",
        "0x2.0d#10",
        Greater,
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "1.895",
        "0x1.e50#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "1.896",
        "0x1.e58#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "1.895",
        "0x1.e50#10",
        Less,
    );
}

#[test]
fn log_base_2_1_plus_x_prec_round_fail() {
    assert_panic!(Float::one_prec(1).log_base_2_1_plus_x_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).log_base_2_1_plus_x_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.log_base_2_1_plus_x_prec_round_assign(0, Floor)
    });

    // log2(1 + 0.5) = log2(1.5) is irrational, so Exact rounding panics.
    let x = parse_hex_string("0x0.8#1");
    assert_panic!(x.clone().log_base_2_1_plus_x_prec_round(1, Exact));
    assert_panic!(x.log_base_2_1_plus_x_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = parse_hex_string("0x0.8#1");
        x.log_base_2_1_plus_x_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_1_plus_x_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (log_base_2_1_plus_x, o) = x.clone().log_base_2_1_plus_x_prec_round(prec, rm);
    assert!(log_base_2_1_plus_x.is_valid());

    let (log_base_2_1_plus_x_alt, o_alt) = x.log_base_2_1_plus_x_prec_round_ref(prec, rm);
    assert!(log_base_2_1_plus_x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_2_1_plus_x_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log_base_2_1_plus_x, rug_o) =
            rug_log_base_2_1_plus_x_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
            ComparableFloatRef(&log_base_2_1_plus_x),
        );
        assert_eq!(rug_o, o);
    }

    if log_base_2_1_plus_x.is_normal() {
        assert_eq!(log_base_2_1_plus_x.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(log_base_2_1_plus_x > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(log_base_2_1_plus_x < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_2_1_plus_x_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log_base_2_1_plus_x.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_2_1_plus_x_prec_round_ref(prec, Exact));
    }
}

#[test]
fn log_base_2_1_plus_x_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_25().test_properties(|(x, prec, rm)| {
        log_base_2_1_plus_x_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_26().test_properties(|(x, prec, rm)| {
        log_base_2_1_plus_x_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (log_base_2_1_plus_x, o) = Float::NAN.log_base_2_1_plus_x_prec_round(prec, rm);
        assert!(log_base_2_1_plus_x.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.log_base_2_1_plus_x_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (s, o) = Float::NEGATIVE_INFINITY.log_base_2_1_plus_x_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.log_base_2_1_plus_x_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.log_base_2_1_plus_x_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::NEGATIVE_ONE.log_base_2_1_plus_x_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_1_plus_x_prec_properties_helper(x: Float, prec: u64) {
    let (log_base_2_1_plus_x, o) = x.clone().log_base_2_1_plus_x_prec(prec);
    assert!(log_base_2_1_plus_x.is_valid());

    let (log_base_2_1_plus_x_alt, o_alt) = x.log_base_2_1_plus_x_prec_ref(prec);
    assert!(log_base_2_1_plus_x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_2_1_plus_x_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    assert_eq!(o_alt, o);

    let (rug_log_base_2_1_plus_x, rug_o) =
        rug_log_base_2_1_plus_x_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
        ComparableFloatRef(&log_base_2_1_plus_x),
    );
    assert_eq!(rug_o, o);

    let (log_base_2_1_plus_x_alt, o_alt) = x.log_base_2_1_plus_x_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    assert_eq!(o_alt, o);

    if log_base_2_1_plus_x.is_normal() {
        assert_eq!(log_base_2_1_plus_x.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(log_base_2_1_plus_x > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(log_base_2_1_plus_x < 0u32);
        }
    }
}

#[test]
fn log_base_2_1_plus_x_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        log_base_2_1_plus_x_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        log_base_2_1_plus_x_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        log_base_2_1_plus_x_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (log_base_2_1_plus_x, o) = Float::NAN.log_base_2_1_plus_x_prec(prec);
        assert!(log_base_2_1_plus_x.is_nan());
        assert_eq!(o, Equal);

        let (log_base_2_1_plus_x, o) = Float::ZERO.log_base_2_1_plus_x_prec(prec);
        assert_eq!(
            ComparableFloat(log_base_2_1_plus_x),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(o, Equal);

        let (log_base_2_1_plus_x, o) = Float::NEGATIVE_ZERO.log_base_2_1_plus_x_prec(prec);
        assert_eq!(
            ComparableFloat(log_base_2_1_plus_x),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.log_base_2_1_plus_x_prec(prec),
            (Float::INFINITY, Equal)
        );
        let (log_base_2_1_plus_x, o) = Float::NEGATIVE_INFINITY.log_base_2_1_plus_x_prec(prec);
        assert_eq!(
            ComparableFloat(log_base_2_1_plus_x),
            ComparableFloat(Float::NAN)
        );
        assert_eq!(o, Equal);

        assert_eq!(
            Float::NEGATIVE_ONE.log_base_2_1_plus_x_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_1_plus_x_round_properties_helper(x: Float, rm: RoundingMode) {
    let (log_base_2_1_plus_x, o) = x.clone().log_base_2_1_plus_x_round(rm);
    assert!(log_base_2_1_plus_x.is_valid());

    let (log_base_2_1_plus_x_alt, o_alt) = x.log_base_2_1_plus_x_round_ref(rm);
    assert!(log_base_2_1_plus_x_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_2_1_plus_x_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    assert_eq!(o_alt, o);

    let (log_base_2_1_plus_x_alt, o_alt) =
        x.log_base_2_1_plus_x_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    assert_eq!(o_alt, o);

    if log_base_2_1_plus_x.is_normal() {
        assert_eq!(log_base_2_1_plus_x.get_prec(), Some(x.get_prec().unwrap()));
        if x > 0u32 && o > Less {
            assert!(log_base_2_1_plus_x > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(log_base_2_1_plus_x < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log_base_2_1_plus_x, rug_o) =
            rug_log_base_2_1_plus_x_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
            ComparableFloatRef(&log_base_2_1_plus_x),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_2_1_plus_x_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log_base_2_1_plus_x.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_2_1_plus_x_round_ref(Exact));
    }
}

#[test]
fn log_base_2_1_plus_x_round_properties() {
    float_rounding_mode_pair_gen_var_40().test_properties(|(x, rm)| {
        log_base_2_1_plus_x_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_41().test_properties(|(x, rm)| {
        log_base_2_1_plus_x_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (log_base_2_1_plus_x, o) = Float::NAN.log_base_2_1_plus_x_round(rm);
        assert!(log_base_2_1_plus_x.is_nan());
        assert_eq!(o, Equal);

        let (log_base_2_1_plus_x, o) = Float::ZERO.log_base_2_1_plus_x_round(rm);
        assert_eq!(
            ComparableFloat(log_base_2_1_plus_x),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(o, Equal);

        let (log_base_2_1_plus_x, o) = Float::NEGATIVE_ZERO.log_base_2_1_plus_x_round(rm);
        assert_eq!(
            ComparableFloat(log_base_2_1_plus_x),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.log_base_2_1_plus_x_round(rm),
            (Float::INFINITY, Equal)
        );
        let (log_base_2_1_plus_x, o) = Float::NEGATIVE_INFINITY.log_base_2_1_plus_x_round(rm);
        assert_eq!(
            ComparableFloat(log_base_2_1_plus_x),
            ComparableFloat(Float::NAN)
        );
        assert_eq!(o, Equal);

        assert_eq!(
            Float::NEGATIVE_ONE.log_base_2_1_plus_x_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_1_plus_x_properties_helper(x: Float) {
    let log_base_2_1_plus_x = x.clone().log_base_2_1_plus_x();
    assert!(log_base_2_1_plus_x.is_valid());

    let log_base_2_1_plus_x_alt = (&x).log_base_2_1_plus_x();
    assert!(log_base_2_1_plus_x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );

    let mut x_alt = x.clone();
    x_alt.log_base_2_1_plus_x_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );

    let log_base_2_1_plus_x_alt = x
        .log_base_2_1_plus_x_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );
    let log_base_2_1_plus_x_alt = x.log_base_2_1_plus_x_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );

    let log_base_2_1_plus_x_alt = x.log_base_2_1_plus_x_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&log_base_2_1_plus_x_alt),
        ComparableFloatRef(&log_base_2_1_plus_x)
    );

    let rug_log_base_2_1_plus_x = rug_log_base_2_1_plus_x(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log_base_2_1_plus_x)),
        ComparableFloatRef(&log_base_2_1_plus_x),
    );
}

#[test]
fn log_base_2_1_plus_x_properties() {
    float_gen().test_properties(|x| {
        log_base_2_1_plus_x_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        log_base_2_1_plus_x_properties_helper(x);
    });
}
