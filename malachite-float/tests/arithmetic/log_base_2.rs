// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{LogBase2, LogBase2Assign, PowerOf2, Reciprocal};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    rounding_mode_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::test_util::arithmetic::log_base_2::{
    rug_log_base_2, rug_log_base_2_prec, rug_log_base_2_prec_round, rug_log_base_2_rational_prec,
    rug_log_base_2_rational_prec_round, rug_log_base_2_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_38,
    float_rounding_mode_pair_gen_var_39, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_23,
    float_unsigned_rounding_mode_triple_gen_var_24,
    rational_unsigned_rounding_mode_triple_gen_var_7,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_3;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_log_base_2() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let log_base_2 = x.clone().log_base_2();
        assert!(log_base_2.is_valid());

        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);

        let log_base_2_alt = (&x).log_base_2();
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );

        let mut log_base_2_alt = x.clone();
        log_base_2_alt.log_base_2_assign();
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&log_base_2)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "-Infinity", "-Infinity");
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity");
    test("1.0", "0x1.0#1", "0.0", "0x0.0");
    test("-1.0", "-0x1.0#1", "NaN", "NaN");
    test("1.0", "0x1.0000000000000000000000000#100", "0.0", "0x0.0");
    test("-1.0", "-0x1.0000000000000000000000000#100", "NaN", "NaN");
    test("2.0", "0x2.0#1", "1.0", "0x1.0#1");
    test("0.5", "0x0.8#1", "-1.0", "-0x1.0#1");
    test("8.0", "0x8.0#1", "4.0", "0x4.0#1");
    test("123.0", "0x7b.0#7", "6.94", "0x6.f#7");
    test("-123.0", "-0x7b.0#7", "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1.6514961294723187",
        "0x1.a6c873498ddf7#53",
    );
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "NaN", "NaN");
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        "2.276171431827064627374502422303e-30",
        "0x2.e2a8eca5705fc2eefa1ffb418E-25#100",
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        "-2.276171431827064627374502422308e-30",
        "-0x2.e2a8eca5705fc2eefa1ffb420E-25#99",
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "1.4426950408889634",
        "0x1.71547652b82fe#53",
    );
    test("too_big", "0x4.0E+268435455#1", "1.0e9", "0x4.0E+7#1");
    test("too_small", "0x1.0E-268435456#1", "-1.0e9", "-0x4.0E+7#1");
}

#[test]
fn test_log_base_2_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_2, o) = x.clone().log_base_2_prec(prec);
        assert!(log_base_2.is_valid());

        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, o_out);

        let (log_base_2_alt, o_alt) = x.log_base_2_prec_ref(prec);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );
        assert_eq!(o_alt, o);

        let mut log_base_2_alt = x.clone();
        let o_alt = log_base_2_alt.log_base_2_prec_assign(prec);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );
        assert_eq!(o_alt, o);

        let (rug_log_base_2, rug_o) = rug_log_base_2_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2)),
            ComparableFloatRef(&log_base_2),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", 1, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 1, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 10, "-1.0", "-0x1.000#10", Equal);
    test("8.0", "0x8.0#1", 1, "4.0", "0x4.0#1", Greater);
    test("8.0", "0x8.0#1", 2, "3.0", "0x3.0#2", Equal);
    test("123.0", "0x7b.0#7", 1, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 10, "6.945", "0x6.f2#10", Greater);
    test("-123.0", "-0x7b.0#7", 1, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "1.652",
        "0x1.a70#10",
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
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        "2.0e-30",
        "0x2.0E-25#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        "2.277e-30",
        "0x2.e3E-25#10",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        "-2.0e-30",
        "-0x2.0E-25#1",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        "-2.277e-30",
        "-0x2.e3E-25#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "1.443",
        "0x1.718#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1000,
        "1.442695040888963330633973025111757859004911661730492706666153957426699136013312517094091\
        499770864701151125538517194667712314473298294366791203412836830111218484997584382369906615\
        056019456341283119154888560194586978546595239587905287284321020679486194205070157570202346\
        3532938165168358726582406234612796",
        "0x1.71547652b82fdbf024ffffda5e624f862a4c6d760af2e99ab795dd63b81b3d1e7d32d4382ee7c7c6f3807\
        72942cbe61dec3221a245cbf1a949ac67ebd74cd1c0131dc85caee9ab6569a5cacfe962f00ecf2eccf35545708\
        9729552aa51d0f89ca0488eaa5613b979a0f29e93eccab78a052a6776b441673a6cab92e622#1000",
        Less,
    );
}

#[test]
fn log_base_2_prec_fail() {
    assert_panic!(Float::NAN.log_base_2_prec(0));
    assert_panic!(Float::NAN.log_base_2_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.log_base_2_prec_assign(0)
    });
}

#[test]
fn test_log_base_2_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_2, o) = x.clone().log_base_2_round(rm);
        assert!(log_base_2.is_valid());

        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, o_out);

        let (log_base_2_alt, o_alt) = x.log_base_2_round_ref(rm);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );
        assert_eq!(o_alt, o);

        let mut log_base_2_alt = x.clone();
        let o_alt = log_base_2_alt.log_base_2_round_assign(rm);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_2, rug_o) = rug_log_base_2_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_2)),
                ComparableFloatRef(&log_base_2),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", Exact, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", Down, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", Up, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Ceiling, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Down, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Nearest, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Exact, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Floor, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Ceiling, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Down, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Nearest, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Exact, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Down, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Up, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Exact, "NaN", "NaN", Equal);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Down,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Up,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("2.0", "0x2.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", Exact, "1.0", "0x1.0#1", Equal);
    test("0.5", "0x0.8#1", Floor, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", Down, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", Up, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", Nearest, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", Exact, "-1.0", "-0x1.0#1", Equal);
    test("8.0", "0x8.0#1", Floor, "2.0", "0x2.0#1", Less);
    test("8.0", "0x8.0#1", Ceiling, "4.0", "0x4.0#1", Greater);
    test("8.0", "0x8.0#1", Down, "2.0", "0x2.0#1", Less);
    test("8.0", "0x8.0#1", Up, "4.0", "0x4.0#1", Greater);
    test("8.0", "0x8.0#1", Nearest, "4.0", "0x4.0#1", Greater);
    test("123.0", "0x7b.0#7", Floor, "6.94", "0x6.f#7", Less);
    test("123.0", "0x7b.0#7", Ceiling, "7.0", "0x7.0#7", Greater);
    test("123.0", "0x7b.0#7", Down, "6.94", "0x6.f#7", Less);
    test("123.0", "0x7b.0#7", Up, "7.0", "0x7.0#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "6.94", "0x6.f#7", Less);
    test("-123.0", "-0x7b.0#7", Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Ceiling, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Down, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Up, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Nearest, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "1.6514961294723187",
        "0x1.a6c873498ddf7#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "1.6514961294723189",
        "0x1.a6c873498ddf8#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "1.6514961294723187",
        "0x1.a6c873498ddf7#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "1.6514961294723189",
        "0x1.a6c873498ddf8#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "1.6514961294723187",
        "0x1.a6c873498ddf7#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        Nearest,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "too_big",
        "0x6.0E+268435455#2",
        Nearest,
        "1.0e9",
        "0x4.0E+7#2",
        Greater,
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        Nearest,
        "-1.0e9",
        "-0x4.0E+7#1",
        Equal,
    );
    test(
        "too_small",
        "0x1.0E-268435456#2",
        Nearest,
        "-1.0e9",
        "-0x4.0E+7#2",
        Equal,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Floor,
        "2.276171431827064627374502422301e-30",
        "0x2.e2a8eca5705fc2eefa1ffb414E-25#100",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Ceiling,
        "2.276171431827064627374502422303e-30",
        "0x2.e2a8eca5705fc2eefa1ffb418E-25#100",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Down,
        "2.276171431827064627374502422301e-30",
        "0x2.e2a8eca5705fc2eefa1ffb414E-25#100",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Up,
        "2.276171431827064627374502422303e-30",
        "0x2.e2a8eca5705fc2eefa1ffb418E-25#100",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Nearest,
        "2.276171431827064627374502422303e-30",
        "0x2.e2a8eca5705fc2eefa1ffb418E-25#100",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Floor,
        "-2.276171431827064627374502422308e-30",
        "-0x2.e2a8eca5705fc2eefa1ffb420E-25#99",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Ceiling,
        "-2.276171431827064627374502422303e-30",
        "-0x2.e2a8eca5705fc2eefa1ffb418E-25#99",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Down,
        "-2.276171431827064627374502422303e-30",
        "-0x2.e2a8eca5705fc2eefa1ffb418E-25#99",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Up,
        "-2.276171431827064627374502422308e-30",
        "-0x2.e2a8eca5705fc2eefa1ffb420E-25#99",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Nearest,
        "-2.276171431827064627374502422308e-30",
        "-0x2.e2a8eca5705fc2eefa1ffb420E-25#99",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "1.4426950408889632",
        "0x1.71547652b82fd#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "1.4426950408889634",
        "0x1.71547652b82fe#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "1.4426950408889632",
        "0x1.71547652b82fd#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "1.4426950408889634",
        "0x1.71547652b82fe#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "1.4426950408889634",
        "0x1.71547652b82fe#53",
        Greater,
    );
}

#[test]
fn log_base_2_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.log_base_2_round(Exact));
    assert_panic!(THREE.log_base_2_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.log_base_2_round_assign(Exact);
    });
}

#[test]
fn test_log_base_2_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_2, o) = x.clone().log_base_2_prec_round(prec, rm);
        assert!(log_base_2.is_valid());

        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, o_out);

        let (log_base_2_alt, o_alt) = x.log_base_2_prec_round_ref(prec, rm);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );
        assert_eq!(o_alt, o);

        let mut log_base_2_alt = x.clone();
        let o_alt = log_base_2_alt.log_base_2_prec_round_assign(prec, rm);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_2),
            ComparableFloatRef(&log_base_2_alt)
        );
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_2, rug_o) =
                rug_log_base_2_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_2)),
                ComparableFloatRef(&log_base_2),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Down, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Up, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Exact, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 1, Down, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Up, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 1, Exact, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Down, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Up, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Exact, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Down, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Up, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Exact, "NaN", "NaN", Equal);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Down,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Up,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Down,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Up,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 1, Down, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 1, Up, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("0.5", "0x0.8#1", 1, Exact, "-1.0", "-0x1.0#1", Equal);
    test("8.0", "0x8.0#1", 1, Floor, "2.0", "0x2.0#1", Less);
    test("8.0", "0x8.0#1", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("8.0", "0x8.0#1", 1, Down, "2.0", "0x2.0#1", Less);
    test("8.0", "0x8.0#1", 1, Up, "4.0", "0x4.0#1", Greater);
    test("8.0", "0x8.0#1", 1, Nearest, "4.0", "0x4.0#1", Greater);
    test("8.0", "0x8.0#1", 2, Floor, "3.0", "0x3.0#2", Equal);
    test("8.0", "0x8.0#1", 2, Ceiling, "3.0", "0x3.0#2", Equal);
    test("8.0", "0x8.0#1", 2, Down, "3.0", "0x3.0#2", Equal);
    test("8.0", "0x8.0#1", 2, Up, "3.0", "0x3.0#2", Equal);
    test("8.0", "0x8.0#1", 2, Nearest, "3.0", "0x3.0#2", Equal);
    test("8.0", "0x8.0#1", 2, Exact, "3.0", "0x3.0#2", Equal);
    test("123.0", "0x7b.0#7", 1, Floor, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Down, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Up, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 10, Floor, "6.94", "0x6.f0#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "6.945",
        "0x6.f2#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 10, Down, "6.94", "0x6.f0#10", Less);
    test("123.0", "0x7b.0#7", 10, Up, "6.945", "0x6.f2#10", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "6.945",
        "0x6.f2#10",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", 1, Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 1, Ceiling, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 1, Down, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 1, Up, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 1, Nearest, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, Ceiling, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, Down, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, Up, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "1.65",
        "0x1.a68#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "1.652",
        "0x1.a70#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "1.65",
        "0x1.a68#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "1.652",
        "0x1.a70#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "1.652",
        "0x1.a70#10",
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
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
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
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Floor,
        "2.0e-30",
        "0x2.0E-25#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Floor,
        "2.274e-30",
        "0x2.e2E-25#10",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Ceiling,
        "3.0e-30",
        "0x4.0E-25#1",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Ceiling,
        "2.277e-30",
        "0x2.e3E-25#10",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Down,
        "2.0e-30",
        "0x2.0E-25#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Down,
        "2.274e-30",
        "0x2.e2E-25#10",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Up,
        "3.0e-30",
        "0x4.0E-25#1",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Up,
        "2.277e-30",
        "0x2.e3E-25#10",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Nearest,
        "2.0e-30",
        "0x2.0E-25#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Nearest,
        "2.277e-30",
        "0x2.e3E-25#10",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Floor,
        "-3.0e-30",
        "-0x4.0E-25#1",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Floor,
        "-2.277e-30",
        "-0x2.e3E-25#10",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Ceiling,
        "-2.0e-30",
        "-0x2.0E-25#1",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Ceiling,
        "-2.274e-30",
        "-0x2.e2E-25#10",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Down,
        "-2.0e-30",
        "-0x2.0E-25#1",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Down,
        "-2.274e-30",
        "-0x2.e2E-25#10",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Up,
        "-3.0e-30",
        "-0x4.0E-25#1",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Up,
        "-2.277e-30",
        "-0x2.e3E-25#10",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Nearest,
        "-2.0e-30",
        "-0x2.0E-25#1",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Nearest,
        "-2.277e-30",
        "-0x2.e3E-25#10",
        Less,
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
        10,
        Floor,
        "1.441",
        "0x1.710#10",
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
        Ceiling,
        "1.443",
        "0x1.718#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Down,
        "1.441",
        "0x1.710#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Up,
        "1.443",
        "0x1.718#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "1.443",
        "0x1.718#10",
        Greater,
    );
}

#[test]
fn log_base_2_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).log_base_2_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).log_base_2_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.log_base_2_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.log_base_2_prec_round(1, Exact));
    assert_panic!(THREE.log_base_2_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.log_base_2_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (log_base_2, o) = x.clone().log_base_2_prec_round(prec, rm);
    assert!(log_base_2.is_valid());

    let (log_base_2_alt, o_alt) = x.log_base_2_prec_round_ref(prec, rm);
    assert!(log_base_2_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_2_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_2));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log_base_2, rug_o) =
            rug_log_base_2_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2)),
            ComparableFloatRef(&log_base_2),
        );
        assert_eq!(rug_o, o);
    }

    if x.is_finite() && x != 0u32 {
        let (log_base_2_alt, o_alt) =
            Float::log_base_2_rational_prec_round_ref(&Rational::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&log_base_2_alt),
            ComparableFloatRef(&log_base_2)
        );
        assert_eq!(o_alt, o);
    }

    if x < 0u32 {
        assert!(log_base_2.is_nan());
    }

    // If log_base_2 is normal, then x is positive and finite, so 2^(e-1) <= x < 2^e, where e is the
    // exponent of x. Thus e - 1 <= log_2(x) < e, and o bounds the rounded result on one side.
    if log_base_2.is_normal() {
        let e = i64::from(x.get_exponent().unwrap());
        if o != Greater {
            assert!(log_base_2 < e);
        }
        if o != Less {
            assert!(log_base_2 >= e - 1);
        }
    }

    if log_base_2.is_normal() {
        assert_eq!(log_base_2.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(log_base_2 > 0u32);
        } else if x > 0u32 && x < 1u32 && o < Greater {
            assert!(log_base_2 < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_2_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log_base_2.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_2_prec_round_ref(prec, Exact));
    }
}

#[test]
fn log_base_2_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_23().test_properties(|(x, prec, rm)| {
        log_base_2_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_24().test_properties(|(x, prec, rm)| {
        log_base_2_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (log_base_2, o) = Float::NAN.log_base_2_prec_round(prec, rm);
        assert!(log_base_2.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.log_base_2_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (log_base_2, o) = Float::NEGATIVE_INFINITY.log_base_2_prec_round(prec, rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.log_base_2_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::NEGATIVE_ZERO.log_base_2_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (log_base_2, o) = Float::ONE.log_base_2_prec_round(prec, rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::NEGATIVE_ONE.log_base_2_prec_round(prec, rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_prec_properties_helper(x: Float, prec: u64) {
    let (log_base_2, o) = x.clone().log_base_2_prec(prec);
    assert!(log_base_2.is_valid());

    let (log_base_2_alt, o_alt) = x.log_base_2_prec_ref(prec);
    assert!(log_base_2_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_2_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_2));
    assert_eq!(o_alt, o);

    let (rug_log_base_2, rug_o) = rug_log_base_2_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log_base_2)),
        ComparableFloatRef(&log_base_2),
    );
    assert_eq!(rug_o, o);

    let (log_base_2_alt, o_alt) = x.log_base_2_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    assert_eq!(o_alt, o);

    if x.is_finite() && x != 0u32 {
        let (log_base_2_alt, o_alt) =
            Float::log_base_2_rational_prec_ref(&Rational::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&log_base_2_alt),
            ComparableFloatRef(&log_base_2)
        );
        assert_eq!(o_alt, o);
    }

    if x < 0u32 {
        assert!(log_base_2.is_nan());
    }

    // If log_base_2 is normal, then x is positive and finite, so 2^(e-1) <= x < 2^e, where e is the
    // exponent of x. Thus e - 1 <= log_2(x) < e, and o bounds the rounded result on one side.
    if log_base_2.is_normal() {
        let e = i64::from(x.get_exponent().unwrap());
        if o != Greater {
            assert!(log_base_2 < e);
        }
        if o != Less {
            assert!(log_base_2 >= e - 1);
        }
    }

    if log_base_2.is_normal() {
        assert_eq!(log_base_2.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(log_base_2 > 0u32);
        } else if x > 0u32 && x < 1u32 && o < Greater {
            assert!(log_base_2 < 0u32);
        }
    }
}

#[test]
fn log_base_2_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        log_base_2_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        log_base_2_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        log_base_2_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (log_base_2, o) = Float::NAN.log_base_2_prec(prec);
        assert!(log_base_2.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.log_base_2_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::NEGATIVE_ZERO.log_base_2_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::INFINITY.log_base_2_prec(prec),
            (Float::INFINITY, Equal)
        );

        let (log_base_2, o) = Float::NEGATIVE_INFINITY.log_base_2_prec(prec);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::ONE.log_base_2_prec(prec);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::NEGATIVE_ONE.log_base_2_prec(prec);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_round_properties_helper(x: Float, rm: RoundingMode) {
    let (log_base_2, o) = x.clone().log_base_2_round(rm);
    assert!(log_base_2.is_valid());

    let (log_base_2_alt, o_alt) = x.log_base_2_round_ref(rm);
    assert!(log_base_2_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_2_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_2));
    assert_eq!(o_alt, o);

    let (log_base_2_alt, o_alt) = x.log_base_2_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    assert_eq!(o_alt, o);

    if x < 0u32 {
        assert!(log_base_2.is_nan());
    }

    // If log_base_2 is normal, then x is positive and finite, so 2^(e-1) <= x < 2^e, where e is the
    // exponent of x. Thus e - 1 <= log_2(x) < e, and o bounds the rounded result on one side.
    if log_base_2.is_normal() {
        let e = i64::from(x.get_exponent().unwrap());
        if o != Greater {
            assert!(log_base_2 < e);
        }
        if o != Less {
            assert!(log_base_2 >= e - 1);
        }
    }

    if log_base_2.is_normal() {
        assert_eq!(log_base_2.get_prec(), Some(x.get_prec().unwrap()));
        if x > 1u32 && o > Less {
            assert!(log_base_2 > 0u32);
        } else if x > 0u32 && x < 1u32 && o < Greater {
            assert!(log_base_2 < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log_base_2, rug_o) = rug_log_base_2_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2)),
            ComparableFloatRef(&log_base_2),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_2_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log_base_2.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_2_round_ref(Exact));
    }
}

#[test]
fn log_base_2_round_properties() {
    float_rounding_mode_pair_gen_var_38().test_properties(|(x, rm)| {
        log_base_2_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_39().test_properties(|(x, rm)| {
        log_base_2_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (log_base_2, o) = Float::NAN.log_base_2_round(rm);
        assert!(log_base_2.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.log_base_2_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::NEGATIVE_ZERO.log_base_2_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::INFINITY.log_base_2_round(rm),
            (Float::INFINITY, Equal)
        );

        let (log_base_2, o) = Float::NEGATIVE_INFINITY.log_base_2_round(rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::ONE.log_base_2_round(rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::NEGATIVE_ONE.log_base_2_round(rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_properties_helper(x: Float) {
    let log_base_2 = x.clone().log_base_2();
    assert!(log_base_2.is_valid());

    let log_base_2_alt = (&x).log_base_2();
    assert!(log_base_2_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );

    let mut x_alt = x.clone();
    x_alt.log_base_2_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_2));

    let log_base_2_alt = x.log_base_2_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    let log_base_2_alt = x.log_base_2_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );

    let (log_base_2_alt, o) = x.log_base_2_round_ref(Nearest);
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );

    if x < 0u32 {
        assert!(log_base_2.is_nan());
    }

    // If log_base_2 is normal, then x is positive and finite, so 2^(e-1) <= x < 2^e, where e is the
    // exponent of x. Thus e - 1 <= log_2(x) < e, and o bounds the rounded result on one side.
    if log_base_2.is_normal() {
        let e = i64::from(x.get_exponent().unwrap());
        if o != Greater {
            assert!(log_base_2 < e);
        }
        if o != Less {
            assert!(log_base_2 >= e - 1);
        }
    }

    let rug_log_base_2 = rug_log_base_2(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log_base_2)),
        ComparableFloatRef(&log_base_2),
    );
}

#[test]
fn log_base_2_properties() {
    float_gen().test_properties(|x| {
        log_base_2_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        log_base_2_properties_helper(x);
    });
}

#[test]
fn log_base_2_rational_prec_fail() {
    assert_panic!(Float::log_base_2_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::log_base_2_rational_prec(Rational::ONE, 0));
    assert_panic!(Float::log_base_2_rational_prec(Rational::NEGATIVE_ONE, 0));
}

#[test]
fn log_base_2_rational_prec_ref_fail() {
    assert_panic!(Float::log_base_2_rational_prec_ref(&Rational::ZERO, 0));
    assert_panic!(Float::log_base_2_rational_prec_ref(&Rational::ONE, 0));
    assert_panic!(Float::log_base_2_rational_prec_ref(
        &Rational::NEGATIVE_ONE,
        0
    ));
}

#[test]
fn log_base_2_rational_prec_round_fail() {
    assert_panic!(Float::log_base_2_rational_prec_round(
        Rational::ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::log_base_2_rational_prec_round(
        Rational::ONE,
        0,
        Floor
    ));
    assert_panic!(Float::log_base_2_rational_prec_round(
        Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
    // - 123 is not a power of 2, so its base-2 logarithm is irrational
    assert_panic!(Float::log_base_2_rational_prec_round(
        Rational::from(123u32),
        1,
        Exact
    ));
    // - 22/7 is not a power of 2, so its base-2 logarithm is irrational
    assert_panic!(Float::log_base_2_rational_prec_round(
        Rational::from_unsigneds(22u8, 7),
        100,
        Exact
    ));
    // - log_2(2^5) = 5, which needs 3 bits and is not representable with precision 1
    assert_panic!(Float::log_base_2_rational_prec_round(
        Rational::power_of_2(5i64),
        1,
        Exact
    ));
}

#[test]
fn log_base_2_rational_prec_round_ref_fail() {
    assert_panic!(Float::log_base_2_rational_prec_round_ref(
        &Rational::ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::log_base_2_rational_prec_round_ref(
        &Rational::ONE,
        0,
        Floor
    ));
    assert_panic!(Float::log_base_2_rational_prec_round_ref(
        &Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
    // - 123 is not a power of 2, so its base-2 logarithm is irrational
    assert_panic!(Float::log_base_2_rational_prec_round_ref(
        &Rational::from(123u32),
        1,
        Exact
    ));
    // - 22/7 is not a power of 2, so its base-2 logarithm is irrational
    assert_panic!(Float::log_base_2_rational_prec_round_ref(
        &Rational::from_unsigneds(22u8, 7),
        100,
        Exact
    ));
    // - log_2(2^5) = 5, which needs 3 bits and is not representable with precision 1
    assert_panic!(Float::log_base_2_rational_prec_round_ref(
        &Rational::power_of_2(5i64),
        1,
        Exact
    ));
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (log_base_2, o) = Float::log_base_2_rational_prec_round(x.clone(), prec, rm);
    assert!(log_base_2.is_valid());

    let (log_base_2_alt, o_alt) = Float::log_base_2_rational_prec_round_ref(&x, prec, rm);
    assert!(log_base_2_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log_base_2, rug_o) = rug_log_base_2_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2)),
            ComparableFloatRef(&log_base_2),
        );
        assert_eq!(rug_o, o);
    }

    if x != 0u32 {
        let (log_base_2_alt, o_alt) =
            Float::log_base_2_rational_prec_round((&x).reciprocal(), prec, -rm);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&(-log_base_2_alt).abs_negative_zero()),
            ComparableFloatRef(&log_base_2.abs_negative_zero_ref())
        );
        assert_eq!(o_alt.reverse(), o);
    }

    if log_base_2.is_normal() {
        assert_eq!(log_base_2.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(log_base_2 > 0u32);
        } else if x < 1u32 && o < Greater {
            assert!(log_base_2 < 0u32);
        }

        // If log_base_2 is normal, then x is positive, so e <= log_2(x) < e + 1, where e is the
        // floor of log_2(x), and o bounds the rounded result on one side.
        let e = x.floor_log_base_2_abs();
        if o != Greater {
            assert!(log_base_2 < e + 1);
        }
        if o != Less {
            assert!(log_base_2 >= e);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::log_base_2_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log_base_2.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::log_base_2_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn log_base_2_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_7().test_properties(|(x, prec, rm)| {
        log_base_2_rational_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    rational_unsigned_rounding_mode_triple_gen_var_7().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            log_base_2_rational_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        assert_eq!(
            Float::log_base_2_rational_prec_round(Rational::ZERO, prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (log_base_2, o) = Float::log_base_2_rational_prec_round(Rational::ONE, prec, rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_2, o) =
            Float::log_base_2_rational_prec_round(Rational::NEGATIVE_ONE, prec, rm);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::log_base_2_rational_prec_round(Rational::TWO, prec, rm);
        assert_eq!(log_base_2, 1);
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::log_base_2_rational_prec_round(Rational::ONE_HALF, prec, rm);
        assert_eq!(log_base_2, -1);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_2_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (log_base_2, o) = Float::log_base_2_rational_prec(x.clone(), prec);
    assert!(log_base_2.is_valid());

    let (log_base_2_alt, o_alt) = Float::log_base_2_rational_prec_ref(&x, prec);
    assert!(log_base_2_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    assert_eq!(o_alt, o);

    let (log_base_2_alt, o_alt) = Float::log_base_2_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&log_base_2_alt),
        ComparableFloatRef(&log_base_2)
    );
    assert_eq!(o_alt, o);

    let (rug_log_base_2, rug_o) = rug_log_base_2_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log_base_2)),
        ComparableFloatRef(&log_base_2),
    );
    assert_eq!(rug_o, o);

    if x != 0u32 {
        let (log_base_2_alt, o_alt) = Float::log_base_2_rational_prec((&x).reciprocal(), prec);
        assert!(log_base_2_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&(-log_base_2_alt).abs_negative_zero()),
            ComparableFloatRef(&log_base_2.abs_negative_zero_ref())
        );
        assert_eq!(o_alt.reverse(), o);
    }

    if log_base_2.is_normal() {
        assert_eq!(log_base_2.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(log_base_2 > 0u32);
        } else if x < 1u32 && o < Greater {
            assert!(log_base_2 < 0u32);
        }

        // If log_base_2 is normal, then x is positive, so e <= log_2(x) < e + 1, where e is the
        // floor of log_2(x), and o bounds the rounded result on one side.
        let e = x.floor_log_base_2_abs();
        if o != Greater {
            assert!(log_base_2 < e + 1);
        }
        if o != Less {
            assert!(log_base_2 >= e);
        }
    }
}

#[test]
fn log_base_2_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        log_base_2_rational_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    rational_unsigned_pair_gen_var_3().test_properties_with_config(&config, |(x, prec)| {
        log_base_2_rational_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        assert_eq!(
            Float::log_base_2_rational_prec(Rational::ZERO, prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (log_base_2, o) = Float::log_base_2_rational_prec(Rational::ONE, prec);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::log_base_2_rational_prec(Rational::NEGATIVE_ONE, prec);
        assert_eq!(ComparableFloat(log_base_2), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::log_base_2_rational_prec(Rational::TWO, prec);
        assert_eq!(log_base_2, 1);
        assert_eq!(o, Equal);

        let (log_base_2, o) = Float::log_base_2_rational_prec(Rational::ONE_HALF, prec);
        assert_eq!(log_base_2, -1);
        assert_eq!(o, Equal);
    });
}

#[test]
fn test_log_base_2_rational_prec() {
    let test = |s, prec, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (log_base_2, o) = Float::log_base_2_rational_prec(u.clone(), prec);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        let (log_base_2, o) = Float::log_base_2_rational_prec_ref(&u, prec);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        let (rug_log_base_2, rug_o) = rug_log_base_2_rational_prec(&u, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2)),
            ComparableFloatRef(&log_base_2),
        );
        assert_eq!(rug_o, o);
    };
    test("0", 1, "-Infinity", "-Infinity", Equal);
    test("0", 10, "-Infinity", "-Infinity", Equal);
    test("0", 100, "-Infinity", "-Infinity", Equal);
    test("1", 1, "0.0", "0x0.0", Equal);
    test("1", 10, "0.0", "0x0.0", Equal);
    test("1", 100, "0.0", "0x0.0", Equal);
    test("-1", 1, "NaN", "NaN", Equal);
    test("-1", 10, "NaN", "NaN", Equal);
    test("-1", 100, "NaN", "NaN", Equal);
    test("-1/2", 1, "NaN", "NaN", Equal);
    test("-1/2", 10, "NaN", "NaN", Equal);
    test("-1/2", 100, "NaN", "NaN", Equal);
    test("-22/7", 1, "NaN", "NaN", Equal);
    test("-22/7", 10, "NaN", "NaN", Equal);
    test("-22/7", 100, "NaN", "NaN", Equal);
    test("2", 1, "1.0", "0x1.0#1", Equal);
    test("2", 10, "1.0", "0x1.000#10", Equal);
    test("2", 100, "1.0", "0x1.0000000000000000000000000#100", Equal);
    test("1/2", 1, "-1.0", "-0x1.0#1", Equal);
    test("1/2", 10, "-1.0", "-0x1.000#10", Equal);
    test(
        "1/2",
        100,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test("8", 1, "4.0", "0x4.0#1", Greater);
    test("8", 10, "3.0", "0x3.00#10", Equal);
    test("8", 100, "3.0", "0x3.0000000000000000000000000#100", Equal);
    test("1/8", 1, "-4.0", "-0x4.0#1", Less);
    test("1/8", 10, "-3.0", "-0x3.00#10", Equal);
    test(
        "1/8",
        100,
        "-3.0",
        "-0x3.0000000000000000000000000#100",
        Equal,
    );
    test("1024", 1, "8.0", "0x8.0#1", Less);
    test("1024", 10, "10.0", "0xa.00#10", Equal);
    test(
        "1024",
        100,
        "10.0",
        "0xa.000000000000000000000000#100",
        Equal,
    );
    test("3/5", 1, "-0.5", "-0x0.8#1", Greater);
    test("3/5", 10, "-0.737", "-0x0.bcc#10", Less);
    test(
        "3/5",
        100,
        "-0.7369655941662061664165804855416",
        "-0x0.bca9c6f53897a45984a4c9f33#100",
        Greater,
    );
    test("1/3", 1, "-2.0", "-0x2.0#1", Less);
    test("1/3", 10, "-1.586", "-0x1.960#10", Less);
    test(
        "1/3",
        100,
        "-1.584962500721156181453738943947",
        "-0x1.95c01a39fbd6879fa00b120a0#100",
        Greater,
    );
    test("22/7", 1, "2.0", "0x2.0#1", Greater);
    test("22/7", 10, "1.652", "0x1.a70#10", Greater);
    test(
        "22/7",
        100,
        "1.652076696579693148757393729494",
        "0x1.a6ee7f964b22a0c5d02f629d4#100",
        Less,
    );
    test("355/113", 1, "2.0", "0x2.0#1", Greater);
    test("355/113", 10, "1.652", "0x1.a70#10", Greater);
    test(
        "355/113",
        100,
        "1.651496251976856700958521105235",
        "0x1.a6c87557b50969b3fc2c87d7e#100",
        Greater,
    );

    let test_big = |u: Rational, prec, out: &str, out_hex: &str, out_o| {
        let (log_base_2, o) = Float::log_base_2_rational_prec(u.clone(), prec);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        let (log_base_2, o) = Float::log_base_2_rational_prec_ref(&u, prec);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        let (rug_log_base_2, rug_o) = rug_log_base_2_rational_prec(&u, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_2)),
            ComparableFloatRef(&log_base_2),
        );
        assert_eq!(rug_o, o);
    };
    test_big(
        Rational::power_of_2(1000i64),
        1,
        "1.0e3",
        "0x4.0E+2#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        "1000.0",
        "0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        "-1.0e3",
        "-0x4.0E+2#1",
        Less,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        "-1000.0",
        "-0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        "-1.0e9",
        "-0x4.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        "-1.074e9",
        "-0x4.00E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "-1.074e9",
        "-0x4.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        1,
        "9.0e-302",
        "0x1.0E-250#1",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        10,
        "1.347e-301",
        "0x1.718E-250#10",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        100,
        "1.346414794256683307023036700644e-301",
        "0x1.71547652b82fe1777d0ffda0eE-250#100",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        1,
        "-9.0e-302",
        "-0x1.0E-250#1",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        10,
        "-1.347e-301",
        "-0x1.718E-250#10",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        100,
        "-1.346414794256683307023036700644e-301",
        "-0x1.71547652b82fe1777d0ffda0eE-250#100",
        Less,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        1,
        "2.0e9",
        "0x8.0E+7#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        10,
        "2.147e9",
        "0x8.00E+7#10",
        Equal,
    );
}

#[test]
fn test_log_base_2_rational_prec_round() {
    let test = |s, prec, rm, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (log_base_2, o) = Float::log_base_2_rational_prec_round(u.clone(), prec, rm);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        let (log_base_2, o) = Float::log_base_2_rational_prec_round_ref(&u, prec, rm);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_2, rug_o) = rug_log_base_2_rational_prec_round(&u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_2)),
                ComparableFloatRef(&log_base_2),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test("0", 1, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0", 1, Down, "-Infinity", "-Infinity", Equal);
    test("0", 1, Up, "-Infinity", "-Infinity", Equal);
    test("0", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", 1, Exact, "-Infinity", "-Infinity", Equal);

    test("0", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("0", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0", 10, Down, "-Infinity", "-Infinity", Equal);
    test("0", 10, Up, "-Infinity", "-Infinity", Equal);
    test("0", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", 10, Exact, "-Infinity", "-Infinity", Equal);

    test("0", 100, Floor, "-Infinity", "-Infinity", Equal);
    test("0", 100, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0", 100, Down, "-Infinity", "-Infinity", Equal);
    test("0", 100, Up, "-Infinity", "-Infinity", Equal);
    test("0", 100, Nearest, "-Infinity", "-Infinity", Equal);
    test("0", 100, Exact, "-Infinity", "-Infinity", Equal);

    test("1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 1, Down, "0.0", "0x0.0", Equal);
    test("1", 1, Up, "0.0", "0x0.0", Equal);
    test("1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", 1, Exact, "0.0", "0x0.0", Equal);

    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 10, Down, "0.0", "0x0.0", Equal);
    test("1", 10, Up, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);

    test("1", 100, Floor, "0.0", "0x0.0", Equal);
    test("1", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 100, Down, "0.0", "0x0.0", Equal);
    test("1", 100, Up, "0.0", "0x0.0", Equal);
    test("1", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", 100, Exact, "0.0", "0x0.0", Equal);

    test("-1", 1, Floor, "NaN", "NaN", Equal);
    test("-1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1", 1, Down, "NaN", "NaN", Equal);
    test("-1", 1, Up, "NaN", "NaN", Equal);
    test("-1", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", 1, Exact, "NaN", "NaN", Equal);

    test("-1", 10, Floor, "NaN", "NaN", Equal);
    test("-1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", 10, Down, "NaN", "NaN", Equal);
    test("-1", 10, Up, "NaN", "NaN", Equal);
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", 10, Exact, "NaN", "NaN", Equal);

    test("-1", 100, Floor, "NaN", "NaN", Equal);
    test("-1", 100, Ceiling, "NaN", "NaN", Equal);
    test("-1", 100, Down, "NaN", "NaN", Equal);
    test("-1", 100, Up, "NaN", "NaN", Equal);
    test("-1", 100, Nearest, "NaN", "NaN", Equal);
    test("-1", 100, Exact, "NaN", "NaN", Equal);

    test("-1/2", 1, Floor, "NaN", "NaN", Equal);
    test("-1/2", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 1, Down, "NaN", "NaN", Equal);
    test("-1/2", 1, Up, "NaN", "NaN", Equal);
    test("-1/2", 1, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 1, Exact, "NaN", "NaN", Equal);

    test("-1/2", 10, Floor, "NaN", "NaN", Equal);
    test("-1/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 10, Down, "NaN", "NaN", Equal);
    test("-1/2", 10, Up, "NaN", "NaN", Equal);
    test("-1/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 10, Exact, "NaN", "NaN", Equal);

    test("-1/2", 100, Floor, "NaN", "NaN", Equal);
    test("-1/2", 100, Ceiling, "NaN", "NaN", Equal);
    test("-1/2", 100, Down, "NaN", "NaN", Equal);
    test("-1/2", 100, Up, "NaN", "NaN", Equal);
    test("-1/2", 100, Nearest, "NaN", "NaN", Equal);
    test("-1/2", 100, Exact, "NaN", "NaN", Equal);

    test("-22/7", 1, Floor, "NaN", "NaN", Equal);
    test("-22/7", 1, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 1, Down, "NaN", "NaN", Equal);
    test("-22/7", 1, Up, "NaN", "NaN", Equal);
    test("-22/7", 1, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 1, Exact, "NaN", "NaN", Equal);

    test("-22/7", 10, Floor, "NaN", "NaN", Equal);
    test("-22/7", 10, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 10, Down, "NaN", "NaN", Equal);
    test("-22/7", 10, Up, "NaN", "NaN", Equal);
    test("-22/7", 10, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 10, Exact, "NaN", "NaN", Equal);

    test("-22/7", 100, Floor, "NaN", "NaN", Equal);
    test("-22/7", 100, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 100, Down, "NaN", "NaN", Equal);
    test("-22/7", 100, Up, "NaN", "NaN", Equal);
    test("-22/7", 100, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 100, Exact, "NaN", "NaN", Equal);

    test("2", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("2", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("2", 1, Down, "1.0", "0x1.0#1", Equal);
    test("2", 1, Up, "1.0", "0x1.0#1", Equal);
    test("2", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("2", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("2", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("2", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("2", 10, Down, "1.0", "0x1.000#10", Equal);
    test("2", 10, Up, "1.0", "0x1.000#10", Equal);
    test("2", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("2", 10, Exact, "1.0", "0x1.000#10", Equal);

    test(
        "2",
        100,
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2",
        100,
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test("1/2", 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test("1/2", 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("1/2", 1, Down, "-1.0", "-0x1.0#1", Equal);
    test("1/2", 1, Up, "-1.0", "-0x1.0#1", Equal);
    test("1/2", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("1/2", 1, Exact, "-1.0", "-0x1.0#1", Equal);

    test("1/2", 10, Floor, "-1.0", "-0x1.000#10", Equal);
    test("1/2", 10, Ceiling, "-1.0", "-0x1.000#10", Equal);
    test("1/2", 10, Down, "-1.0", "-0x1.000#10", Equal);
    test("1/2", 10, Up, "-1.0", "-0x1.000#10", Equal);
    test("1/2", 10, Nearest, "-1.0", "-0x1.000#10", Equal);
    test("1/2", 10, Exact, "-1.0", "-0x1.000#10", Equal);

    test(
        "1/2",
        100,
        Floor,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Down,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Up,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Nearest,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Exact,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );

    test("8", 1, Floor, "2.0", "0x2.0#1", Less);
    test("8", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("8", 1, Down, "2.0", "0x2.0#1", Less);
    test("8", 1, Up, "4.0", "0x4.0#1", Greater);
    test("8", 1, Nearest, "4.0", "0x4.0#1", Greater);

    test("8", 10, Floor, "3.0", "0x3.00#10", Equal);
    test("8", 10, Ceiling, "3.0", "0x3.00#10", Equal);
    test("8", 10, Down, "3.0", "0x3.00#10", Equal);
    test("8", 10, Up, "3.0", "0x3.00#10", Equal);
    test("8", 10, Nearest, "3.0", "0x3.00#10", Equal);
    test("8", 10, Exact, "3.0", "0x3.00#10", Equal);

    test(
        "8",
        100,
        Floor,
        "3.0",
        "0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "8",
        100,
        Ceiling,
        "3.0",
        "0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "8",
        100,
        Down,
        "3.0",
        "0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "8",
        100,
        Up,
        "3.0",
        "0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "8",
        100,
        Nearest,
        "3.0",
        "0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "8",
        100,
        Exact,
        "3.0",
        "0x3.0000000000000000000000000#100",
        Equal,
    );

    test("1/8", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("1/8", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("1/8", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("1/8", 1, Up, "-4.0", "-0x4.0#1", Less);
    test("1/8", 1, Nearest, "-4.0", "-0x4.0#1", Less);

    test("1/8", 10, Floor, "-3.0", "-0x3.00#10", Equal);
    test("1/8", 10, Ceiling, "-3.0", "-0x3.00#10", Equal);
    test("1/8", 10, Down, "-3.0", "-0x3.00#10", Equal);
    test("1/8", 10, Up, "-3.0", "-0x3.00#10", Equal);
    test("1/8", 10, Nearest, "-3.0", "-0x3.00#10", Equal);
    test("1/8", 10, Exact, "-3.0", "-0x3.00#10", Equal);

    test(
        "1/8",
        100,
        Floor,
        "-3.0",
        "-0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/8",
        100,
        Ceiling,
        "-3.0",
        "-0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/8",
        100,
        Down,
        "-3.0",
        "-0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/8",
        100,
        Up,
        "-3.0",
        "-0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/8",
        100,
        Nearest,
        "-3.0",
        "-0x3.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1/8",
        100,
        Exact,
        "-3.0",
        "-0x3.0000000000000000000000000#100",
        Equal,
    );

    test("1024", 1, Floor, "8.0", "0x8.0#1", Less);
    test("1024", 1, Ceiling, "2.0e1", "0x1.0E+1#1", Greater);
    test("1024", 1, Down, "8.0", "0x8.0#1", Less);
    test("1024", 1, Up, "2.0e1", "0x1.0E+1#1", Greater);
    test("1024", 1, Nearest, "8.0", "0x8.0#1", Less);

    test("1024", 10, Floor, "10.0", "0xa.00#10", Equal);
    test("1024", 10, Ceiling, "10.0", "0xa.00#10", Equal);
    test("1024", 10, Down, "10.0", "0xa.00#10", Equal);
    test("1024", 10, Up, "10.0", "0xa.00#10", Equal);
    test("1024", 10, Nearest, "10.0", "0xa.00#10", Equal);
    test("1024", 10, Exact, "10.0", "0xa.00#10", Equal);

    test(
        "1024",
        100,
        Floor,
        "10.0",
        "0xa.000000000000000000000000#100",
        Equal,
    );
    test(
        "1024",
        100,
        Ceiling,
        "10.0",
        "0xa.000000000000000000000000#100",
        Equal,
    );
    test(
        "1024",
        100,
        Down,
        "10.0",
        "0xa.000000000000000000000000#100",
        Equal,
    );
    test(
        "1024",
        100,
        Up,
        "10.0",
        "0xa.000000000000000000000000#100",
        Equal,
    );
    test(
        "1024",
        100,
        Nearest,
        "10.0",
        "0xa.000000000000000000000000#100",
        Equal,
    );
    test(
        "1024",
        100,
        Exact,
        "10.0",
        "0xa.000000000000000000000000#100",
        Equal,
    );

    test("3/5", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("3/5", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("3/5", 1, Down, "-0.5", "-0x0.8#1", Greater);
    test("3/5", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("3/5", 1, Nearest, "-0.5", "-0x0.8#1", Greater);

    test("3/5", 10, Floor, "-0.737", "-0x0.bcc#10", Less);
    test("3/5", 10, Ceiling, "-0.736", "-0x0.bc8#10", Greater);
    test("3/5", 10, Down, "-0.736", "-0x0.bc8#10", Greater);
    test("3/5", 10, Up, "-0.737", "-0x0.bcc#10", Less);
    test("3/5", 10, Nearest, "-0.737", "-0x0.bcc#10", Less);

    test(
        "3/5",
        100,
        Floor,
        "-0.7369655941662061664165804855424",
        "-0x0.bca9c6f53897a45984a4c9f34#100",
        Less,
    );
    test(
        "3/5",
        100,
        Ceiling,
        "-0.7369655941662061664165804855416",
        "-0x0.bca9c6f53897a45984a4c9f33#100",
        Greater,
    );
    test(
        "3/5",
        100,
        Down,
        "-0.7369655941662061664165804855416",
        "-0x0.bca9c6f53897a45984a4c9f33#100",
        Greater,
    );
    test(
        "3/5",
        100,
        Up,
        "-0.7369655941662061664165804855424",
        "-0x0.bca9c6f53897a45984a4c9f34#100",
        Less,
    );
    test(
        "3/5",
        100,
        Nearest,
        "-0.7369655941662061664165804855416",
        "-0x0.bca9c6f53897a45984a4c9f33#100",
        Greater,
    );

    test("1/3", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("1/3", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("1/3", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("1/3", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("1/3", 1, Nearest, "-2.0", "-0x2.0#1", Less);

    test("1/3", 10, Floor, "-1.586", "-0x1.960#10", Less);
    test("1/3", 10, Ceiling, "-1.584", "-0x1.958#10", Greater);
    test("1/3", 10, Down, "-1.584", "-0x1.958#10", Greater);
    test("1/3", 10, Up, "-1.586", "-0x1.960#10", Less);
    test("1/3", 10, Nearest, "-1.586", "-0x1.960#10", Less);

    test(
        "1/3",
        100,
        Floor,
        "-1.584962500721156181453738943949",
        "-0x1.95c01a39fbd6879fa00b120a2#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "-1.584962500721156181453738943947",
        "-0x1.95c01a39fbd6879fa00b120a0#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "-1.584962500721156181453738943947",
        "-0x1.95c01a39fbd6879fa00b120a0#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Up,
        "-1.584962500721156181453738943949",
        "-0x1.95c01a39fbd6879fa00b120a2#100",
        Less,
    );
    test(
        "1/3",
        100,
        Nearest,
        "-1.584962500721156181453738943947",
        "-0x1.95c01a39fbd6879fa00b120a0#100",
        Greater,
    );

    test("22/7", 1, Floor, "1.0", "0x1.0#1", Less);
    test("22/7", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("22/7", 1, Down, "1.0", "0x1.0#1", Less);
    test("22/7", 1, Up, "2.0", "0x2.0#1", Greater);
    test("22/7", 1, Nearest, "2.0", "0x2.0#1", Greater);

    test("22/7", 10, Floor, "1.65", "0x1.a68#10", Less);
    test("22/7", 10, Ceiling, "1.652", "0x1.a70#10", Greater);
    test("22/7", 10, Down, "1.65", "0x1.a68#10", Less);
    test("22/7", 10, Up, "1.652", "0x1.a70#10", Greater);
    test("22/7", 10, Nearest, "1.652", "0x1.a70#10", Greater);

    test(
        "22/7",
        100,
        Floor,
        "1.652076696579693148757393729494",
        "0x1.a6ee7f964b22a0c5d02f629d4#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "1.652076696579693148757393729495",
        "0x1.a6ee7f964b22a0c5d02f629d6#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Down,
        "1.652076696579693148757393729494",
        "0x1.a6ee7f964b22a0c5d02f629d4#100",
        Less,
    );
    test(
        "22/7",
        100,
        Up,
        "1.652076696579693148757393729495",
        "0x1.a6ee7f964b22a0c5d02f629d6#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "1.652076696579693148757393729494",
        "0x1.a6ee7f964b22a0c5d02f629d4#100",
        Less,
    );

    test("355/113", 1, Floor, "1.0", "0x1.0#1", Less);
    test("355/113", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("355/113", 1, Down, "1.0", "0x1.0#1", Less);
    test("355/113", 1, Up, "2.0", "0x2.0#1", Greater);
    test("355/113", 1, Nearest, "2.0", "0x2.0#1", Greater);

    test("355/113", 10, Floor, "1.65", "0x1.a68#10", Less);
    test("355/113", 10, Ceiling, "1.652", "0x1.a70#10", Greater);
    test("355/113", 10, Down, "1.65", "0x1.a68#10", Less);
    test("355/113", 10, Up, "1.652", "0x1.a70#10", Greater);
    test("355/113", 10, Nearest, "1.652", "0x1.a70#10", Greater);

    test(
        "355/113",
        100,
        Floor,
        "1.651496251976856700958521105234",
        "0x1.a6c87557b50969b3fc2c87d7c#100",
        Less,
    );
    test(
        "355/113",
        100,
        Ceiling,
        "1.651496251976856700958521105235",
        "0x1.a6c87557b50969b3fc2c87d7e#100",
        Greater,
    );
    test(
        "355/113",
        100,
        Down,
        "1.651496251976856700958521105234",
        "0x1.a6c87557b50969b3fc2c87d7c#100",
        Less,
    );
    test(
        "355/113",
        100,
        Up,
        "1.651496251976856700958521105235",
        "0x1.a6c87557b50969b3fc2c87d7e#100",
        Greater,
    );
    test(
        "355/113",
        100,
        Nearest,
        "1.651496251976856700958521105235",
        "0x1.a6c87557b50969b3fc2c87d7e#100",
        Greater,
    );

    let test_big = |u: Rational, prec, rm, out: &str, out_hex: &str, out_o| {
        let (log_base_2, o) = Float::log_base_2_rational_prec_round(u.clone(), prec, rm);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        let (log_base_2, o) = Float::log_base_2_rational_prec_round_ref(&u, prec, rm);
        assert!(log_base_2.is_valid());
        assert_eq!(log_base_2.to_string(), out);
        assert_eq!(to_hex_string(&log_base_2), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_2, rug_o) = rug_log_base_2_rational_prec_round(&u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_2)),
                ComparableFloatRef(&log_base_2),
            );
            assert_eq!(rug_o, o);
        }
    };
    test_big(
        Rational::power_of_2(1000i64),
        1,
        Floor,
        "5.0e2",
        "0x2.0E+2#1",
        Less,
    );
    test_big(
        Rational::power_of_2(1000i64),
        1,
        Ceiling,
        "1.0e3",
        "0x4.0E+2#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(1000i64),
        1,
        Down,
        "5.0e2",
        "0x2.0E+2#1",
        Less,
    );
    test_big(
        Rational::power_of_2(1000i64),
        1,
        Up,
        "1.0e3",
        "0x4.0E+2#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(1000i64),
        1,
        Nearest,
        "1.0e3",
        "0x4.0E+2#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(1000i64),
        10,
        Floor,
        "1000.0",
        "0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Ceiling,
        "1000.0",
        "0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Down,
        "1000.0",
        "0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Up,
        "1000.0",
        "0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Nearest,
        "1000.0",
        "0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Exact,
        "1000.0",
        "0x3e8.0#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Floor,
        "-1.0e3",
        "-0x4.0E+2#1",
        Less,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Ceiling,
        "-5.0e2",
        "-0x2.0E+2#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Down,
        "-5.0e2",
        "-0x2.0E+2#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Up,
        "-1.0e3",
        "-0x4.0E+2#1",
        Less,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        1,
        Nearest,
        "-1.0e3",
        "-0x4.0E+2#1",
        Less,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Floor,
        "-1000.0",
        "-0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Ceiling,
        "-1000.0",
        "-0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Down,
        "-1000.0",
        "-0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Up,
        "-1000.0",
        "-0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "-1000.0",
        "-0x3e8.0#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Exact,
        "-1000.0",
        "-0x3e8.0#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Floor,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Ceiling,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Down,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Up,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Nearest,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Floor,
        "1.073e9",
        "0x3.ffE+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Ceiling,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Down,
        "1.073e9",
        "0x3.ffE+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Up,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Floor,
        "1.073e9",
        "0x3.ffE+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Ceiling,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Down,
        "1.073e9",
        "0x3.ffE+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Up,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Nearest,
        "1.074e9",
        "0x4.00E+7#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Floor,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Ceiling,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Down,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Up,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Nearest,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Floor,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Ceiling,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Down,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Up,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Nearest,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Floor,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Ceiling,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Down,
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Up,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Nearest,
        "1.0e9",
        "0x4.0E+7#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Floor,
        "-1.0e9",
        "-0x4.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Ceiling,
        "-5.0e8",
        "-0x2.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Down,
        "-5.0e8",
        "-0x2.0E+7#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Up,
        "-1.0e9",
        "-0x4.0E+7#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        1,
        Nearest,
        "-1.0e9",
        "-0x4.0E+7#1",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Floor,
        "-1.074e9",
        "-0x4.00E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Ceiling,
        "-1.073e9",
        "-0x3.ffE+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Down,
        "-1.073e9",
        "-0x3.ffE+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Up,
        "-1.074e9",
        "-0x4.00E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Nearest,
        "-1.074e9",
        "-0x4.00E+7#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Floor,
        "-1.074e9",
        "-0x4.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Ceiling,
        "-1.074e9",
        "-0x4.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Down,
        "-1.074e9",
        "-0x4.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Up,
        "-1.074e9",
        "-0x4.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Nearest,
        "-1.074e9",
        "-0x4.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Exact,
        "-1.074e9",
        "-0x4.00E+7#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Floor,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Ceiling,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Down,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Up,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Nearest,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Floor,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Ceiling,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Down,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Up,
        "-1.076e9",
        "-0x4.02E+7#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Nearest,
        "-1.074e9",
        "-0x4.00E+7#10",
        Greater,
    );

    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        1,
        Floor,
        "9.0e-302",
        "0x1.0E-250#1",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        1,
        Ceiling,
        "2.0e-301",
        "0x2.0E-250#1",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        1,
        Down,
        "9.0e-302",
        "0x1.0E-250#1",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        1,
        Up,
        "2.0e-301",
        "0x2.0E-250#1",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        1,
        Nearest,
        "9.0e-302",
        "0x1.0E-250#1",
        Less,
    );

    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        10,
        Floor,
        "1.345e-301",
        "0x1.710E-250#10",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        10,
        Ceiling,
        "1.347e-301",
        "0x1.718E-250#10",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        10,
        Down,
        "1.345e-301",
        "0x1.710E-250#10",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        10,
        Up,
        "1.347e-301",
        "0x1.718E-250#10",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "1.347e-301",
        "0x1.718E-250#10",
        Greater,
    );

    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        100,
        Floor,
        "1.346414794256683307023036700642e-301",
        "0x1.71547652b82fe1777d0ffda0cE-250#100",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        100,
        Ceiling,
        "1.346414794256683307023036700644e-301",
        "0x1.71547652b82fe1777d0ffda0eE-250#100",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        100,
        Down,
        "1.346414794256683307023036700642e-301",
        "0x1.71547652b82fe1777d0ffda0cE-250#100",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        100,
        Up,
        "1.346414794256683307023036700644e-301",
        "0x1.71547652b82fe1777d0ffda0eE-250#100",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-1000i64),
        100,
        Nearest,
        "1.346414794256683307023036700644e-301",
        "0x1.71547652b82fe1777d0ffda0eE-250#100",
        Greater,
    );

    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        1,
        Floor,
        "-2.0e-301",
        "-0x2.0E-250#1",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        1,
        Ceiling,
        "-9.0e-302",
        "-0x1.0E-250#1",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        1,
        Down,
        "-9.0e-302",
        "-0x1.0E-250#1",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        1,
        Up,
        "-2.0e-301",
        "-0x2.0E-250#1",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        1,
        Nearest,
        "-9.0e-302",
        "-0x1.0E-250#1",
        Greater,
    );

    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        10,
        Floor,
        "-1.347e-301",
        "-0x1.718E-250#10",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        10,
        Ceiling,
        "-1.345e-301",
        "-0x1.710E-250#10",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        10,
        Down,
        "-1.345e-301",
        "-0x1.710E-250#10",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        10,
        Up,
        "-1.347e-301",
        "-0x1.718E-250#10",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "-1.347e-301",
        "-0x1.718E-250#10",
        Less,
    );

    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        100,
        Floor,
        "-1.346414794256683307023036700644e-301",
        "-0x1.71547652b82fe1777d0ffda0eE-250#100",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        100,
        Ceiling,
        "-1.346414794256683307023036700642e-301",
        "-0x1.71547652b82fe1777d0ffda0cE-250#100",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        100,
        Down,
        "-1.346414794256683307023036700642e-301",
        "-0x1.71547652b82fe1777d0ffda0cE-250#100",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        100,
        Up,
        "-1.346414794256683307023036700644e-301",
        "-0x1.71547652b82fe1777d0ffda0eE-250#100",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-1000i64),
        100,
        Nearest,
        "-1.346414794256683307023036700644e-301",
        "-0x1.71547652b82fe1777d0ffda0eE-250#100",
        Less,
    );

    test_big(
        Rational::power_of_2(1i64 << 31),
        1,
        Floor,
        "2.0e9",
        "0x8.0E+7#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        1,
        Ceiling,
        "2.0e9",
        "0x8.0E+7#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        1,
        Down,
        "2.0e9",
        "0x8.0E+7#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        1,
        Up,
        "2.0e9",
        "0x8.0E+7#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        1,
        Nearest,
        "2.0e9",
        "0x8.0E+7#1",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        1,
        Exact,
        "2.0e9",
        "0x8.0E+7#1",
        Equal,
    );

    test_big(
        Rational::power_of_2(1i64 << 31),
        10,
        Floor,
        "2.147e9",
        "0x8.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        10,
        Ceiling,
        "2.147e9",
        "0x8.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        10,
        Down,
        "2.147e9",
        "0x8.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        10,
        Up,
        "2.147e9",
        "0x8.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        10,
        Nearest,
        "2.147e9",
        "0x8.00E+7#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1i64 << 31),
        10,
        Exact,
        "2.147e9",
        "0x8.00E+7#10",
        Equal,
    );
    // Inputs extremely close to a power of 2, where the general Ziv loop would otherwise need a
    // precision proportional to the distance (previously diverged / crashed).
    test_big(
        Rational::power_of_2(100000i64) + Rational::ONE,
        1,
        Floor,
        "7.0e4",
        "0x1.0E+4#1",
        Less,
    );
    test_big(
        Rational::power_of_2(100000i64) + Rational::ONE,
        5,
        Floor,
        "9.8e4",
        "0x1.8E+4#5",
        Less,
    );
    test_big(
        Rational::power_of_2(100000i64) + Rational::ONE,
        1,
        Ceiling,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(100000i64) + Rational::ONE,
        5,
        Ceiling,
        "1.0e5",
        "0x1.9E+4#5",
        Greater,
    );
    test_big(
        Rational::power_of_2(100000i64) + Rational::ONE,
        1,
        Nearest,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(100000i64) + Rational::ONE,
        5,
        Nearest,
        "9.8e4",
        "0x1.8E+4#5",
        Less,
    );
    test_big(
        Rational::power_of_2(100000i64) - Rational::power_of_2(-5i64),
        1,
        Floor,
        "7.0e4",
        "0x1.0E+4#1",
        Less,
    );
    test_big(
        Rational::power_of_2(100000i64) - Rational::power_of_2(-5i64),
        5,
        Floor,
        "9.8e4",
        "0x1.8E+4#5",
        Less,
    );
    test_big(
        Rational::power_of_2(100000i64) - Rational::power_of_2(-5i64),
        1,
        Ceiling,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(100000i64) - Rational::power_of_2(-5i64),
        5,
        Ceiling,
        "1.0e5",
        "0x1.9E+4#5",
        Greater,
    );
    test_big(
        Rational::power_of_2(100000i64) - Rational::power_of_2(-5i64),
        1,
        Nearest,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(100000i64) - Rational::power_of_2(-5i64),
        5,
        Nearest,
        "9.8e4",
        "0x1.8E+4#5",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-100000i64),
        1,
        Floor,
        "too_small",
        "0x1.0E-25000#1",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-100000i64),
        5,
        Floor,
        "too_small",
        "0x1.7E-25000#5",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-100000i64),
        1,
        Ceiling,
        "too_small",
        "0x2.0E-25000#1",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-100000i64),
        5,
        Ceiling,
        "too_small",
        "0x1.8E-25000#5",
        Greater,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-100000i64),
        1,
        Nearest,
        "too_small",
        "0x1.0E-25000#1",
        Less,
    );
    test_big(
        Rational::ONE + Rational::power_of_2(-100000i64),
        5,
        Nearest,
        "too_small",
        "0x1.7E-25000#5",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-100000i64),
        1,
        Floor,
        "-too_small",
        "-0x2.0E-25000#1",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-100000i64),
        5,
        Floor,
        "-too_small",
        "-0x1.8E-25000#5",
        Less,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-100000i64),
        1,
        Ceiling,
        "-too_small",
        "-0x1.0E-25000#1",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-100000i64),
        5,
        Ceiling,
        "-too_small",
        "-0x1.7E-25000#5",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-100000i64),
        1,
        Nearest,
        "-too_small",
        "-0x1.0E-25000#1",
        Greater,
    );
    test_big(
        Rational::ONE - Rational::power_of_2(-100000i64),
        5,
        Nearest,
        "-too_small",
        "-0x1.7E-25000#5",
        Greater,
    );
    test_big(
        Rational::from(4) + Rational::power_of_2(-200i64),
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test_big(
        Rational::from(4) + Rational::power_of_2(-200i64),
        5,
        Floor,
        "2.0",
        "0x2.0#5",
        Less,
    );
    test_big(
        Rational::from(4) + Rational::power_of_2(-200i64),
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test_big(
        Rational::from(4) + Rational::power_of_2(-200i64),
        5,
        Ceiling,
        "2.1",
        "0x2.2#5",
        Greater,
    );
    test_big(
        Rational::from(4) + Rational::power_of_2(-200i64),
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test_big(
        Rational::from(4) + Rational::power_of_2(-200i64),
        5,
        Nearest,
        "2.0",
        "0x2.0#5",
        Less,
    );
    test_big(
        Rational::from_signeds(1, 4) - Rational::power_of_2(-200i64),
        1,
        Floor,
        "-4.0",
        "-0x4.0#1",
        Less,
    );
    test_big(
        Rational::from_signeds(1, 4) - Rational::power_of_2(-200i64),
        5,
        Floor,
        "-2.1",
        "-0x2.2#5",
        Less,
    );
    test_big(
        Rational::from_signeds(1, 4) - Rational::power_of_2(-200i64),
        1,
        Ceiling,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test_big(
        Rational::from_signeds(1, 4) - Rational::power_of_2(-200i64),
        5,
        Ceiling,
        "-2.0",
        "-0x2.0#5",
        Greater,
    );
    test_big(
        Rational::from_signeds(1, 4) - Rational::power_of_2(-200i64),
        1,
        Nearest,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test_big(
        Rational::from_signeds(1, 4) - Rational::power_of_2(-200i64),
        5,
        Nearest,
        "-2.0",
        "-0x2.0#5",
        Greater,
    );
}
