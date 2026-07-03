// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ExpXMinus1, ExpXMinus1Assign, FloorLogBase2, PowerOf2,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, rounding_mode_gen, unsigned_gen_var_11,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::arithmetic::exp_x_minus_1::{
    primitive_float_exp_x_minus_1, primitive_float_exp_x_minus_1_rational,
};
use malachite_float::test_util::arithmetic::exp_x_minus_1::{
    rug_exp_x_minus_1, rug_exp_x_minus_1_prec, rug_exp_x_minus_1_prec_round,
    rug_exp_x_minus_1_rational_prec, rug_exp_x_minus_1_rational_prec_round,
    rug_exp_x_minus_1_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;

#[test]
fn test_exp_x_minus_1() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let exp_x_minus_1 = x.clone().exp_x_minus_1();
        assert!(exp_x_minus_1.is_valid());

        assert_eq!(exp_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&exp_x_minus_1), out_hex);

        let exp_x_minus_1_alt = (&x).exp_x_minus_1();
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );

        let mut exp_x_minus_1_alt = x.clone();
        exp_x_minus_1_alt.exp_x_minus_1_assign();
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_exp_x_minus_1(&rug::Float::exact_from(
                &x
            )))),
            ComparableFloatRef(&exp_x_minus_1)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-1.0", "-0x1.0#1");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "2.0", "0x2.0#1");
    test("-1.0", "-0x1.0#1", "-0.5", "-0x0.8#1");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1.718281828459045235360287471353",
        "0x1.b7e151628aed2a6abf715880a#100",
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "14.154262241479262",
        "0xe.277dbaf2293d0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "22.140692632779267",
        "0x16.24046eb09339#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-0.9567860817362277",
        "-0x0.f4efeec2533880#53",
    );
    test("123.0", "0x7b.0#7", "2.63e53", "0x2.c0E+44#7");
    test("-123.0", "-0x7b.0#7", "-1.0", "-0x1.00#7");
    test("0.5", "0x0.8#1", "0.5", "0x0.8#1");
    test("-0.5", "-0x0.8#1", "-0.5", "-0x0.8#1");
    test("-100.0", "-0x64.0#7", "-1.0", "-0x1.00#7");
    test("-1000.0", "-0x3e8.0#10", "-1.0", "-0x1.000#10");
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
    );
}

#[test]
fn test_exp_x_minus_1_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (exp_x_minus_1, o) = x.clone().exp_x_minus_1_prec(prec);
        assert!(exp_x_minus_1.is_valid());

        assert_eq!(exp_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&exp_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_prec_ref(prec);
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut exp_x_minus_1_alt = x.clone();
        let o_alt = exp_x_minus_1_alt.exp_x_minus_1_prec_assign(prec);
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let (rug_exp_x_minus_1, rug_o) = rug_exp_x_minus_1_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_exp_x_minus_1)),
            ComparableFloatRef(&exp_x_minus_1),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, "NaN", "NaN", Equal);
    test("NaN", "NaN", 53, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 10, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 53, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "-1.0", "-0x1.0#1", Equal);
    test("-Infinity", "-Infinity", 10, "-1.0", "-0x1.000#10", Equal);
    test(
        "-Infinity",
        "-Infinity",
        53,
        "-1.0",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 53, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 53, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 10, "1.719", "0x1.b80#10", Greater);
    test(
        "1.0",
        "0x1.0#1",
        53,
        "1.7182818284590453",
        "0x1.b7e151628aed3#53",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 1, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", 10, "-0.632", "-0x0.a1c#10", Greater);
    test(
        "-1.0",
        "-0x1.0#1",
        53,
        "-0.6321205588285577",
        "-0x0.a1d2a7274c4320#53",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        "1.719",
        "0x1.b80#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        53,
        "1.7182818284590453",
        "0x1.b7e151628aed3#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "14.16",
        "0xe.28#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        53,
        "14.154262241479262",
        "0xe.277dbaf2293d0#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "2.0e1",
        "0x1.0E+1#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "22.16",
        "0x16.28#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        53,
        "22.140692632779267",
        "0x16.24046eb09339#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.957",
        "-0x0.f50#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        53,
        "-0.9567860817362277",
        "-0x0.f4efeec2533880#53",
        Greater,
    );
    test("123.0", "0x7b.0#7", 1, "2.0e53", "0x2.0E+44#1", Less);
    test("123.0", "0x7b.0#7", 10, "2.619e53", "0x2.bcE+44#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        53,
        "2.6195173187490626e53",
        "0x2.bc22a5f350232E+44#53",
        Less,
    );
    test("-123.0", "-0x7b.0#7", 1, "-1.0", "-0x1.0#1", Less);
    test("-123.0", "-0x7b.0#7", 10, "-1.0", "-0x1.000#10", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        53,
        "-1.0",
        "-0x1.0000000000000#53",
        Less,
    );
    test("0.5", "0x0.8#1", 1, "0.5", "0x0.8#1", Less);
    test("0.5", "0x0.8#1", 10, "0.648", "0x0.a60#10", Less);
    test(
        "0.5",
        "0x0.8#1",
        53,
        "0.6487212707001282",
        "0x0.a61298e1e069c0#53",
        Greater,
    );
    test("-0.5", "-0x0.8#1", 1, "-0.5", "-0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", 10, "-0.3936", "-0x0.64c#10", Less);
    test(
        "-0.5",
        "-0x0.8#1",
        53,
        "-0.39346934028736658",
        "-0x0.64ba681c834fb0#53",
        Greater,
    );
    test("-100.0", "-0x64.0#7", 1, "-1.0", "-0x1.0#1", Less);
    test("-100.0", "-0x64.0#7", 10, "-1.0", "-0x1.000#10", Less);
    test(
        "-100.0",
        "-0x64.0#7",
        53,
        "-1.0",
        "-0x1.0000000000000#53",
        Less,
    );
    test("-1000.0", "-0x3e8.0#10", 1, "-1.0", "-0x1.0#1", Less);
    test("-1000.0", "-0x3e8.0#10", 10, "-1.0", "-0x1.000#10", Less);
    test(
        "-1000.0",
        "-0x3e8.0#10",
        53,
        "-1.0",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        "2.0e-28",
        "0x1.0E-23#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        "2.019e-28",
        "0x1.000E-23#10",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        53,
        "2.0194839173657902e-28",
        "0x1.0000000000000E-23#53",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        "-2.0e-28",
        "-0x1.0E-23#1",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        "-2.019e-28",
        "-0x1.000E-23#10",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        53,
        "-2.0194839173657902e-28",
        "-0x1.0000000000000E-23#53",
        Less,
    );
}

#[test]
#[should_panic]
fn exp_x_minus_1_prec_fail() {
    Float::NAN.exp_x_minus_1_prec(0);
}

#[test]
fn test_exp_x_minus_1_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (exp_x_minus_1, o) = x.clone().exp_x_minus_1_round(rm);
        assert!(exp_x_minus_1.is_valid());

        assert_eq!(exp_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&exp_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_round_ref(rm);
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut exp_x_minus_1_alt = x.clone();
        let o_alt = exp_x_minus_1_alt.exp_x_minus_1_round_assign(rm);
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_exp_x_minus_1, rug_o) =
                rug_exp_x_minus_1_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_exp_x_minus_1)),
                ComparableFloatRef(&exp_x_minus_1),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", Down, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", Up, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", Floor, "-1.0", "-0x1.0#1", Equal);
    test("-Infinity", "-Infinity", Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("-Infinity", "-Infinity", Down, "-1.0", "-0x1.0#1", Equal);
    test("-Infinity", "-Infinity", Up, "-1.0", "-0x1.0#1", Equal);
    test("-Infinity", "-Infinity", Nearest, "-1.0", "-0x1.0#1", Equal);
    test("0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", Up, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", Nearest, "2.0", "0x2.0#1", Greater);
    test("-1.0", "-0x1.0#1", Floor, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", Down, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", Up, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", Nearest, "-0.5", "-0x0.8#1", Greater);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Floor,
        "1.718281828459045235360287471351",
        "0x1.b7e151628aed2a6abf7158808#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Ceiling,
        "1.718281828459045235360287471353",
        "0x1.b7e151628aed2a6abf715880a#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Down,
        "1.718281828459045235360287471351",
        "0x1.b7e151628aed2a6abf7158808#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Up,
        "1.718281828459045235360287471353",
        "0x1.b7e151628aed2a6abf715880a#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Nearest,
        "1.718281828459045235360287471353",
        "0x1.b7e151628aed2a6abf715880a#100",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "14.154262241479261",
        "0xe.277dbaf2293c8#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "14.154262241479262",
        "0xe.277dbaf2293d0#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "14.154262241479261",
        "0xe.277dbaf2293c8#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "14.154262241479262",
        "0xe.277dbaf2293d0#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "14.154262241479262",
        "0xe.277dbaf2293d0#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "22.140692632779263",
        "0x16.24046eb09338#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "22.140692632779267",
        "0x16.24046eb09339#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "22.140692632779263",
        "0x16.24046eb09338#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "22.140692632779267",
        "0x16.24046eb09339#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "22.140692632779267",
        "0x16.24046eb09339#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-0.9567860817362278",
        "-0x0.f4efeec2533888#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-0.9567860817362277",
        "-0x0.f4efeec2533880#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-0.9567860817362277",
        "-0x0.f4efeec2533880#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-0.9567860817362278",
        "-0x0.f4efeec2533888#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-0.9567860817362277",
        "-0x0.f4efeec2533880#53",
        Greater,
    );
    test("123.0", "0x7b.0#7", Floor, "2.6e53", "0x2.b8E+44#7", Less);
    test(
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "2.63e53",
        "0x2.c0E+44#7",
        Greater,
    );
    test("123.0", "0x7b.0#7", Down, "2.6e53", "0x2.b8E+44#7", Less);
    test("123.0", "0x7b.0#7", Up, "2.63e53", "0x2.c0E+44#7", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        Nearest,
        "2.63e53",
        "0x2.c0E+44#7",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", Floor, "-1.0", "-0x1.00#7", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        Ceiling,
        "-0.99",
        "-0x0.fe#7",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", Down, "-0.99", "-0x0.fe#7", Greater);
    test("-123.0", "-0x7b.0#7", Up, "-1.0", "-0x1.00#7", Less);
    test("-123.0", "-0x7b.0#7", Nearest, "-1.0", "-0x1.00#7", Less);
    test("0.5", "0x0.8#1", Floor, "0.5", "0x0.8#1", Less);
    test("0.5", "0x0.8#1", Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.5", "0x0.8#1", Down, "0.5", "0x0.8#1", Less);
    test("0.5", "0x0.8#1", Up, "1.0", "0x1.0#1", Greater);
    test("0.5", "0x0.8#1", Nearest, "0.5", "0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", Floor, "-0.5", "-0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", Ceiling, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", Down, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", Up, "-0.5", "-0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", Nearest, "-0.5", "-0x0.8#1", Less);
    test("-100.0", "-0x64.0#7", Floor, "-1.0", "-0x1.00#7", Less);
    test(
        "-100.0",
        "-0x64.0#7",
        Ceiling,
        "-0.99",
        "-0x0.fe#7",
        Greater,
    );
    test("-100.0", "-0x64.0#7", Down, "-0.99", "-0x0.fe#7", Greater);
    test("-100.0", "-0x64.0#7", Up, "-1.0", "-0x1.00#7", Less);
    test("-100.0", "-0x64.0#7", Nearest, "-1.0", "-0x1.00#7", Less);
    test("-1000.0", "-0x3e8.0#10", Floor, "-1.0", "-0x1.000#10", Less);
    test(
        "-1000.0",
        "-0x3e8.0#10",
        Ceiling,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-1000.0",
        "-0x3e8.0#10",
        Down,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test("-1000.0", "-0x3e8.0#10", Up, "-1.0", "-0x1.000#10", Less);
    test(
        "-1000.0",
        "-0x3e8.0#10",
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Floor,
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Ceiling,
        "2.019483917365790221854028e-28",
        "0x1.00000000000000000002E-23#80",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Down,
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Up,
        "2.019483917365790221854028e-28",
        "0x1.00000000000000000002E-23#80",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Nearest,
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Floor,
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Ceiling,
        "-2.019483917365790221854023e-28",
        "-0xf.fffffffffffffffffffE-24#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Down,
        "-2.019483917365790221854023e-28",
        "-0xf.fffffffffffffffffffE-24#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Up,
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Nearest,
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Less,
    );
}

#[test]
#[should_panic]
fn exp_x_minus_1_round_fail() {
    Float::from_unsigned_prec(3u8, 10)
        .0
        .exp_x_minus_1_round(Exact);
}

#[test]
fn test_exp_x_minus_1_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (exp_x_minus_1, o) = x.clone().exp_x_minus_1_prec_round(prec, rm);
        assert!(exp_x_minus_1.is_valid());

        assert_eq!(exp_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&exp_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_prec_round_ref(prec, rm);
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut exp_x_minus_1_alt = x.clone();
        let o_alt = exp_x_minus_1_alt.exp_x_minus_1_prec_round_assign(prec, rm);
        assert!(exp_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&exp_x_minus_1),
            ComparableFloatRef(&exp_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_exp_x_minus_1, rug_o) =
                rug_exp_x_minus_1_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_exp_x_minus_1)),
                ComparableFloatRef(&exp_x_minus_1),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "-1.0",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Ceiling,
        "-1.0",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Equal,
    );
    test("0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "1.717", "0x1.b78#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.719",
        "0x1.b80#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "1.719",
        "0x1.b80#10",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", 1, Nearest, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", 10, Floor, "-0.633", "-0x0.a20#10", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-0.632",
        "-0x0.a1c#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.632",
        "-0x0.a1c#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Floor,
        "1.717",
        "0x1.b78#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "1.719",
        "0x1.b80#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "1.719",
        "0x1.b80#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Floor,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Ceiling,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "14.14",
        "0xe.24#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "14.16",
        "0xe.28#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "14.16",
        "0xe.28#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "2.0e1",
        "0x1.0E+1#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "3.0e1",
        "0x2.0E+1#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "2.0e1",
        "0x1.0E+1#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "22.12",
        "0x16.20#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "22.16",
        "0x16.28#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "22.16",
        "0x16.28#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-0.957",
        "-0x0.f50#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.956",
        "-0x0.f4c#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.957",
        "-0x0.f50#10",
        Less,
    );
    test("123.0", "0x7b.0#7", 1, Floor, "2.0e53", "0x2.0E+44#1", Less);
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "4.0e53",
        "0x4.0E+44#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "2.0e53",
        "0x2.0E+44#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "2.619e53",
        "0x2.bcE+44#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "2.623e53",
        "0x2.bdE+44#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "2.619e53",
        "0x2.bcE+44#10",
        Less,
    );
    test("-123.0", "-0x7b.0#7", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test("-123.0", "-0x7b.0#7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Floor,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Ceiling,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test("0.5", "0x0.8#1", 1, Floor, "0.5", "0x0.8#1", Less);
    test("0.5", "0x0.8#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.5", "0x0.8#1", 1, Nearest, "0.5", "0x0.8#1", Less);
    test("0.5", "0x0.8#1", 10, Floor, "0.648", "0x0.a60#10", Less);
    test(
        "0.5",
        "0x0.8#1",
        10,
        Ceiling,
        "0.649",
        "0x0.a64#10",
        Greater,
    );
    test("0.5", "0x0.8#1", 10, Nearest, "0.648", "0x0.a60#10", Less);
    test("-0.5", "-0x0.8#1", 1, Floor, "-0.5", "-0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", 1, Ceiling, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", 1, Nearest, "-0.5", "-0x0.8#1", Less);
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Floor,
        "-0.3936",
        "-0x0.64c#10",
        Less,
    );
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Ceiling,
        "-0.3931",
        "-0x0.64a#10",
        Greater,
    );
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Nearest,
        "-0.3936",
        "-0x0.64c#10",
        Less,
    );
    test("-100.0", "-0x64.0#7", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test(
        "-100.0",
        "-0x64.0#7",
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test("-100.0", "-0x64.0#7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "-100.0",
        "-0x64.0#7",
        10,
        Floor,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test(
        "-100.0",
        "-0x64.0#7",
        10,
        Ceiling,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-100.0",
        "-0x64.0#7",
        10,
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test("-1000.0", "-0x3e8.0#10", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test(
        "-1000.0",
        "-0x3e8.0#10",
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test(
        "-1000.0",
        "-0x3e8.0#10",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-1000.0",
        "-0x3e8.0#10",
        10,
        Floor,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test(
        "-1000.0",
        "-0x3e8.0#10",
        10,
        Ceiling,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-1000.0",
        "-0x3e8.0#10",
        10,
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        Floor,
        "2.0e-28",
        "0x1.0E-23#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        Ceiling,
        "4.0e-28",
        "0x2.0E-23#1",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        Nearest,
        "2.0e-28",
        "0x1.0E-23#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Floor,
        "2.019e-28",
        "0x1.000E-23#10",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Ceiling,
        "2.023e-28",
        "0x1.008E-23#10",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Nearest,
        "2.019e-28",
        "0x1.000E-23#10",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        Floor,
        "-2.0e-28",
        "-0x1.0E-23#1",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        Ceiling,
        "-1.0e-28",
        "-0x8.0E-24#1",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        Nearest,
        "-2.0e-28",
        "-0x1.0E-23#1",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Floor,
        "-2.019e-28",
        "-0x1.000E-23#10",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Ceiling,
        "-2.018e-28",
        "-0xf.fcE-24#10",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Nearest,
        "-2.019e-28",
        "-0x1.000E-23#10",
        Less,
    );

    // Branch-coverage cases (Step 4). Each exercises a path the cases above miss.
    // - ex < 0 but err <= prec + 1, so the small-input fast path is skipped; reaches the general
    //   case with ex < 0 (extra working precision) and a cancelling subtraction.
    test(
        "0.2",
        "0x0.4#2",
        10,
        Nearest,
        "0.2842",
        "0x0.48c#10",
        Greater,
    );
    // - small-input fast path is eligible (err > prec + 1) but `float_round_near_x` cannot round,
    //   so it falls through to the general case.
    test(
        "-0.00390625",
        "-0x0.010000000000000000#64",
        1,
        Ceiling,
        "-0.002",
        "-0x0.008#1",
        Greater,
    );
    // - general-case Ziv loop needs a retry (the first working precision cannot round).
    test(
        "-18.75",
        "-0x12.c00000000000000#64",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    // - x <= -32 with |x| so large that `err` clamps to `MAX_EXPONENT`; rounds -1 directly.
    test(
        "-1.0e9",
        "-0x4.0E+7#1",
        7,
        Nearest,
        "-1.0",
        "-0x1.00#7",
        Less,
    );
    // - exp(x) overflows.
    test(
        "1.0e9",
        "0x4.0E+7#1",
        7,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - x <= -32 but prec so large the large-negative fast path cannot round, so it falls through
    //   to the general case.
    test(
        "-100.0",
        "-0x64.0#7",
        200,
        Nearest,
        "-0.9999999999999999999999999999999999999999999627992402397916407",
        "-0x0.ffffffffffffffffffffffffffffffffffff2b9f0758ea851b#200",
        Less,
    );

    // - deeply negative x: e^x lies below the smallest positive Float (x < ln(2) * (MIN_EXPONENT -
    //   1) ~ -744261117.95), but prec is so large that the bits of e^x land within the output's
    //   prec-bit window, so the result must be computed for real via 2^(x/ln(2)) - 1 (this used to
    //   panic). Here y = x/ln(2) ~ -1073741826.95, and with prec = MAX_EXPONENT + 65 the result -1
    //   + 2^y is about 2^61 ulps above -1. The result is a ~128 MB Float, so instead of comparing
    //   strings, check that 1 + f (an exact `Rational`, approximately 2^y) is positive and lies in
    //   the right binade; the pre-fix answers (-1 or -1 + ulp) give 0 or the wrong binade. Also
    //   check that the Floor and Ceiling results differ by exactly one ulp and that Nearest matches
    //   one of them.
    let prec = u64::exact_from(Float::MAX_EXPONENT) + 65;
    let x = Float::exact_from(-744261120i64);
    let mut r_floor = None;
    let mut r_ceiling = None;
    let mut r_nearest = None;
    for rm in [Floor, Ceiling, Nearest] {
        let (f, o) = x.exp_x_minus_1_prec_round_ref(prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.get_prec(), Some(prec));
        let r = Rational::exact_from(&f) + Rational::ONE;
        assert!(r > 0u32);
        assert_eq!(r.floor_log_base_2(), -1073741827);
        match rm {
            Floor => {
                assert_eq!(o, Less);
                r_floor = Some(f);
            }
            Ceiling => {
                assert_eq!(o, Greater);
                r_ceiling = Some(f);
            }
            _ => r_nearest = Some(f),
        }
    }
    let (r_floor, r_ceiling, r_nearest) =
        (r_floor.unwrap(), r_ceiling.unwrap(), r_nearest.unwrap());
    assert_eq!(
        Rational::exact_from(&r_ceiling) - Rational::exact_from(&r_floor),
        Rational::power_of_2(-i64::exact_from(prec))
    );
    assert!(
        ComparableFloatRef(&r_nearest) == ComparableFloatRef(&r_floor)
            || ComparableFloatRef(&r_nearest) == ComparableFloatRef(&r_ceiling)
    );
}

#[test]
#[should_panic]
fn exp_x_minus_1_prec_round_fail_1() {
    Float::NAN.exp_x_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn exp_x_minus_1_prec_round_fail_2() {
    Float::from_unsigned_prec(3u8, 10)
        .0
        .exp_x_minus_1_prec_round(10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn exp_x_minus_1_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (exp_x_minus_1, o) = x.clone().exp_x_minus_1_prec_round(prec, rm);
    assert!(exp_x_minus_1.is_valid());

    let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_prec_round_ref(prec, rm);
    assert!(exp_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.exp_x_minus_1_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_exp_x_minus_1, rug_o) =
            rug_exp_x_minus_1_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_exp_x_minus_1)),
            ComparableFloatRef(&exp_x_minus_1),
        );
        assert_eq!(rug_o, o);
    }

    if exp_x_minus_1.is_normal() {
        assert_eq!(exp_x_minus_1.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(exp_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(exp_x_minus_1 < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.exp_x_minus_1_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(exp_x_minus_1.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.exp_x_minus_1_prec_round_ref(prec, Exact));
    }
}

#[test]
fn exp_x_minus_1_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        exp_x_minus_1_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            exp_x_minus_1_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.exp_x_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.exp_x_minus_1_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (s, o) = Float::NEGATIVE_INFINITY.exp_x_minus_1_prec_round(prec, rm);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.exp_x_minus_1_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.exp_x_minus_1_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn exp_x_minus_1_prec_properties_helper(x: Float, prec: u64) {
    let (exp_x_minus_1, o) = x.clone().exp_x_minus_1_prec(prec);
    assert!(exp_x_minus_1.is_valid());

    let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_prec_ref(prec);
    assert!(exp_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.exp_x_minus_1_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let (rug_exp_x_minus_1, rug_o) = rug_exp_x_minus_1_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_exp_x_minus_1)),
        ComparableFloatRef(&exp_x_minus_1),
    );
    assert_eq!(rug_o, o);

    let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if exp_x_minus_1.is_normal() {
        assert_eq!(exp_x_minus_1.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(exp_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(exp_x_minus_1 < 0u32);
        }
    }
}

#[test]
fn exp_x_minus_1_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        exp_x_minus_1_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        exp_x_minus_1_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        exp_x_minus_1_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.exp_x_minus_1_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.exp_x_minus_1_prec(prec);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.exp_x_minus_1_prec(prec);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.exp_x_minus_1_prec(prec),
            (Float::INFINITY, Equal)
        );
        let (s, o) = Float::NEGATIVE_INFINITY.exp_x_minus_1_prec(prec);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn exp_x_minus_1_round_properties_helper(x: Float, rm: RoundingMode) {
    let (exp_x_minus_1, o) = x.clone().exp_x_minus_1_round(rm);
    assert!(exp_x_minus_1.is_valid());

    let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_round_ref(rm);
    assert!(exp_x_minus_1_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.exp_x_minus_1_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let (exp_x_minus_1_alt, o_alt) = x.exp_x_minus_1_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if x.is_finite() {
        // Since the result has the same precision as x, and exp(x) - 1 >= x, the rounded result
        // cannot be less than x.
        assert!(exp_x_minus_1 >= x);
    }

    if exp_x_minus_1.is_normal() {
        // For finite x the result has x's precision; for x = -inf the result is -1 (normal, but x
        // has no precision), so the precision check only applies when x is finite.
        if let Some(p) = x.get_prec() {
            assert_eq!(exp_x_minus_1.get_prec(), Some(p));
        }
        if x > 0u32 && o > Less {
            assert!(exp_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(exp_x_minus_1 < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_exp_x_minus_1, rug_o) = rug_exp_x_minus_1_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_exp_x_minus_1)),
            ComparableFloatRef(&exp_x_minus_1),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.exp_x_minus_1_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(exp_x_minus_1.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.exp_x_minus_1_round_ref(Exact));
    }
}

#[test]
fn exp_x_minus_1_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        exp_x_minus_1_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (s, o) = Float::NAN.exp_x_minus_1_round(rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.exp_x_minus_1_round(rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.exp_x_minus_1_round(rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.exp_x_minus_1_round(rm),
            (Float::INFINITY, Equal)
        );
        let (s, o) = Float::NEGATIVE_INFINITY.exp_x_minus_1_round(rm);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn exp_x_minus_1_properties_helper(x: Float) {
    let exp_x_minus_1 = x.clone().exp_x_minus_1();
    assert!(exp_x_minus_1.is_valid());

    let exp_x_minus_1_alt = (&x).exp_x_minus_1();
    assert!(exp_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );

    let mut x_alt = x.clone();
    x_alt.exp_x_minus_1_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );

    let exp_x_minus_1_alt = x
        .exp_x_minus_1_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );
    let exp_x_minus_1_alt = x.exp_x_minus_1_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );

    let exp_x_minus_1_alt = x.exp_x_minus_1_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&exp_x_minus_1_alt),
        ComparableFloatRef(&exp_x_minus_1)
    );

    if x.is_finite() {
        // Since the result has the same precision as x, and exp(x) - 1 >= x, the rounded result
        // cannot be less than x.
        assert!(exp_x_minus_1 >= x);
    }

    let rug_exp_x_minus_1 = rug_exp_x_minus_1(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_exp_x_minus_1)),
        ComparableFloatRef(&exp_x_minus_1),
    );
}

#[test]
fn exp_x_minus_1_properties() {
    float_gen().test_properties(|x| {
        exp_x_minus_1_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        exp_x_minus_1_properties_helper(x);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_exp_x_minus_1() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_exp_x_minus_1(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, -1.0);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 1.7182819);
    test::<f32>(7.0, 1095.6332);
    test::<f32>(100.0, f32::INFINITY);
    test::<f32>(-0.5, -0.39346933);
    // Points where the standard library's `exp_m1` rounds differently (it gives 0.0004886587 and
    // -0.00048817202 respectively); `primitive_float_exp_x_minus_1` is correctly rounded.
    test::<f32>(0.0004885394, 0.0004886588);
    test::<f32>(-0.0004882912, -0.000488172);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, -1.0);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 1.7182818284590453);
    test::<f64>(7.0, 1095.6331584284585);
    test::<f64>(100.0, 2.6881171418161356e43);
    test::<f64>(-0.5, -0.3934693402873666);
    // A point where the standard library's `exp_m1` rounds differently (it gives
    // 0.0004884004786945544); `primitive_float_exp_x_minus_1` is correctly rounded.
    test::<f64>(0.0004882812500000812, 0.0004884004786945543);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_exp_x_minus_1_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_exp_x_minus_1(x);
    });
}

#[test]
fn primitive_float_exp_x_minus_1_properties() {
    apply_fn_to_primitive_floats!(primitive_float_exp_x_minus_1_properties_helper);
}

#[test]
fn test_exp_x_minus_1_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::exp_x_minus_1_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::exp_x_minus_1_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 53, "0.0", "0x0.0", Equal);
    test("1", 1, "2.0", "0x2.0#1", Greater);
    test("1", 10, "1.719", "0x1.b80#10", Greater);
    test(
        "1",
        53,
        "1.7182818284590453",
        "0x1.b7e151628aed3#53",
        Greater,
    );
    test("-1", 1, "-0.5", "-0x0.8#1", Greater);
    test("-1", 10, "-0.632", "-0x0.a1c#10", Greater);
    test(
        "-1",
        53,
        "-0.6321205588285577",
        "-0x0.a1d2a7274c4320#53",
        Greater,
    );
    test("1/2", 1, "0.5", "0x0.8#1", Less);
    test("1/2", 10, "0.648", "0x0.a60#10", Less);
    test(
        "1/2",
        53,
        "0.6487212707001282",
        "0x0.a61298e1e069c0#53",
        Greater,
    );
    test("-1/2", 1, "-0.5", "-0x0.8#1", Less);
    test("-1/2", 10, "-0.3936", "-0x0.64c#10", Less);
    test(
        "-1/2",
        53,
        "-0.39346934028736658",
        "-0x0.64ba681c834fb0#53",
        Greater,
    );
    test("1/3", 1, "0.5", "0x0.8#1", Greater);
    test("1/3", 10, "0.3955", "0x0.654#10", Less);
    test(
        "1/3",
        53,
        "0.39561242508608951",
        "0x0.6546db1ba2d130#53",
        Less,
    );
    test("-1/3", 1, "-0.2", "-0x0.4#1", Greater);
    test("-1/3", 10, "-0.2837", "-0x0.48a#10", Less);
    test(
        "-1/3",
        53,
        "-0.28346868942621073",
        "-0x0.4891676e868ad8#53",
        Greater,
    );
    test("22/7", 1, "2.0e1", "0x1.0E+1#1", Less);
    test("22/7", 10, "22.16", "0x16.28#10", Less);
    test(
        "22/7",
        53,
        "22.169972298262479",
        "0x16.2b834df64368#53",
        Less,
    );
    test("-22/7", 1, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 10, "-0.957", "-0x0.f50#10", Less);
    test(
        "-22/7",
        53,
        "-0.9568406907385474",
        "-0x0.f4f382f23440a0#53",
        Greater,
    );
    test("2", 1, "8.0", "0x8.0#1", Greater);
    test("2", 10, "6.39", "0x6.64#10", Greater);
    test(
        "2",
        53,
        "6.3890560989306504",
        "0x6.63992e35376b8#53",
        Greater,
    );
    test("-2", 1, "-1.0", "-0x1.0#1", Less);
    test("-2", 10, "-0.864", "-0x0.dd4#10", Greater);
    test(
        "-2",
        53,
        "-0.8646647167633873",
        "-0x0.dd5aaab880fc68#53",
        Greater,
    );
    test("100", 1, "2.0e43", "0x1.0E+36#1", Less);
    test("100", 10, "2.687e43", "0x1.348E+36#10", Less);
    test(
        "100",
        53,
        "2.6881171418161356e43",
        "0x1.3494a9b171bf5E+36#53",
        Greater,
    );
    test("-100", 1, "-1.0", "-0x1.0#1", Less);
    test("-100", 10, "-1.0", "-0x1.000#10", Less);
    test("-100", 53, "-1.0", "-0x1.0000000000000#53", Less);

    let test_big = |x: Rational, prec, out: &str, out_hex: &str, out_o| {
        let (f, o) = Float::exp_x_minus_1_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::exp_x_minus_1_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    // |x| too large to be a finite Float: expm1 overflows to +inf (x > 0) or tends to -1 (x < 0).
    test_big(
        Rational::from(1_000_000_000),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::from(1_000_000_000),
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(-Rational::from(1_000_000_000), 1, "-1.0", "-0x1.0#1", Less);
    test_big(
        -Rational::from(1_000_000_000),
        10,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test_big(
        Rational::power_of_2(1000i64),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(-Rational::power_of_2(1000i64), 1, "-1.0", "-0x1.0#1", Less);
    test_big(
        -Rational::power_of_2(1000i64),
        10,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    // |x| < 2^MIN_EXPONENT: expm1(x) ~ x is too small to be a normal Float and underflows. These
    // route through the Taylor-series helper (the squeeze cannot bracket such an x). They also
    // exercise the half-GCD `m_lens[1][0] == 0` path in malachite-nz, which no other test reaches.
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        "too_small",
        "0x1.0E-268435456#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "too_small",
        "0x1.000E-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "-too_small",
        "-0x1.000E-268435456#10",
        Less,
    );
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_fail() {
    Float::exp_x_minus_1_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_ref_fail() {
    Float::exp_x_minus_1_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_exp_x_minus_1_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::exp_x_minus_1_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 53, Floor, "0.0", "0x0.0", Equal);
    test("0", 53, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 53, Nearest, "0.0", "0x0.0", Equal);
    test("1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1", 10, Floor, "1.717", "0x1.b78#10", Less);
    test("1", 10, Ceiling, "1.719", "0x1.b80#10", Greater);
    test("1", 10, Nearest, "1.719", "0x1.b80#10", Greater);
    test(
        "1",
        53,
        Floor,
        "1.7182818284590451",
        "0x1.b7e151628aed2#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "1.7182818284590453",
        "0x1.b7e151628aed3#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "1.7182818284590453",
        "0x1.b7e151628aed3#53",
        Greater,
    );
    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1", 1, Nearest, "-0.5", "-0x0.8#1", Greater);
    test("-1", 10, Floor, "-0.633", "-0x0.a20#10", Less);
    test("-1", 10, Ceiling, "-0.632", "-0x0.a1c#10", Greater);
    test("-1", 10, Nearest, "-0.632", "-0x0.a1c#10", Greater);
    test(
        "-1",
        53,
        Floor,
        "-0.6321205588285578",
        "-0x0.a1d2a7274c4328#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "-0.6321205588285577",
        "-0x0.a1d2a7274c4320#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "-0.6321205588285577",
        "-0x0.a1d2a7274c4320#53",
        Greater,
    );
    test("1/2", 1, Floor, "0.5", "0x0.8#1", Less);
    test("1/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Nearest, "0.5", "0x0.8#1", Less);
    test("1/2", 10, Floor, "0.648", "0x0.a60#10", Less);
    test("1/2", 10, Ceiling, "0.649", "0x0.a64#10", Greater);
    test("1/2", 10, Nearest, "0.648", "0x0.a60#10", Less);
    test(
        "1/2",
        53,
        Floor,
        "0.6487212707001281",
        "0x0.a61298e1e069b8#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "0.6487212707001282",
        "0x0.a61298e1e069c0#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "0.6487212707001282",
        "0x0.a61298e1e069c0#53",
        Greater,
    );
    test("-1/2", 1, Floor, "-0.5", "-0x0.8#1", Less);
    test("-1/2", 1, Ceiling, "-0.2", "-0x0.4#1", Greater);
    test("-1/2", 1, Nearest, "-0.5", "-0x0.8#1", Less);
    test("-1/2", 10, Floor, "-0.3936", "-0x0.64c#10", Less);
    test("-1/2", 10, Ceiling, "-0.3931", "-0x0.64a#10", Greater);
    test("-1/2", 10, Nearest, "-0.3936", "-0x0.64c#10", Less);
    test(
        "-1/2",
        53,
        Floor,
        "-0.39346934028736663",
        "-0x0.64ba681c834fb4#53",
        Less,
    );
    test(
        "-1/2",
        53,
        Ceiling,
        "-0.39346934028736658",
        "-0x0.64ba681c834fb0#53",
        Greater,
    );
    test(
        "-1/2",
        53,
        Nearest,
        "-0.39346934028736658",
        "-0x0.64ba681c834fb0#53",
        Greater,
    );
    test("1/3", 1, Floor, "0.2", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.5", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.5", "0x0.8#1", Greater);
    test("1/3", 10, Floor, "0.3955", "0x0.654#10", Less);
    test("1/3", 10, Ceiling, "0.396", "0x0.656#10", Greater);
    test("1/3", 10, Nearest, "0.3955", "0x0.654#10", Less);
    test(
        "1/3",
        53,
        Floor,
        "0.39561242508608951",
        "0x0.6546db1ba2d130#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "0.39561242508608957",
        "0x0.6546db1ba2d134#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "0.39561242508608951",
        "0x0.6546db1ba2d130#53",
        Less,
    );
    test("-1/3", 1, Floor, "-0.5", "-0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "-0.2", "-0x0.4#1", Greater);
    test("-1/3", 1, Nearest, "-0.2", "-0x0.4#1", Greater);
    test("-1/3", 10, Floor, "-0.2837", "-0x0.48a#10", Less);
    test("-1/3", 10, Ceiling, "-0.2832", "-0x0.488#10", Greater);
    test("-1/3", 10, Nearest, "-0.2837", "-0x0.48a#10", Less);
    test(
        "-1/3",
        53,
        Floor,
        "-0.28346868942621078",
        "-0x0.4891676e868adc#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-0.28346868942621073",
        "-0x0.4891676e868ad8#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-0.28346868942621073",
        "-0x0.4891676e868ad8#53",
        Greater,
    );
    test("22/7", 1, Floor, "2.0e1", "0x1.0E+1#1", Less);
    test("22/7", 1, Ceiling, "3.0e1", "0x2.0E+1#1", Greater);
    test("22/7", 1, Nearest, "2.0e1", "0x1.0E+1#1", Less);
    test("22/7", 10, Floor, "22.16", "0x16.28#10", Less);
    test("22/7", 10, Ceiling, "22.19", "0x16.30#10", Greater);
    test("22/7", 10, Nearest, "22.16", "0x16.28#10", Less);
    test(
        "22/7",
        53,
        Floor,
        "22.169972298262479",
        "0x16.2b834df64368#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "22.169972298262483",
        "0x16.2b834df64369#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "22.169972298262479",
        "0x16.2b834df64368#53",
        Less,
    );
    test("-22/7", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 10, Floor, "-0.957", "-0x0.f50#10", Less);
    test("-22/7", 10, Ceiling, "-0.956", "-0x0.f4c#10", Greater);
    test("-22/7", 10, Nearest, "-0.957", "-0x0.f50#10", Less);
    test(
        "-22/7",
        53,
        Floor,
        "-0.9568406907385475",
        "-0x0.f4f382f23440a8#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "-0.9568406907385474",
        "-0x0.f4f382f23440a0#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "-0.9568406907385474",
        "-0x0.f4f382f23440a0#53",
        Greater,
    );
    test("2", 1, Floor, "4.0", "0x4.0#1", Less);
    test("2", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("2", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test("2", 10, Floor, "6.383", "0x6.62#10", Less);
    test("2", 10, Ceiling, "6.39", "0x6.64#10", Greater);
    test("2", 10, Nearest, "6.39", "0x6.64#10", Greater);
    test(
        "2",
        53,
        Floor,
        "6.3890560989306495",
        "0x6.63992e35376b4#53",
        Less,
    );
    test(
        "2",
        53,
        Ceiling,
        "6.3890560989306504",
        "0x6.63992e35376b8#53",
        Greater,
    );
    test(
        "2",
        53,
        Nearest,
        "6.3890560989306504",
        "0x6.63992e35376b8#53",
        Greater,
    );
    test("-2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-2", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-2", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-2", 10, Floor, "-0.865", "-0x0.dd8#10", Less);
    test("-2", 10, Ceiling, "-0.864", "-0x0.dd4#10", Greater);
    test("-2", 10, Nearest, "-0.864", "-0x0.dd4#10", Greater);
    test(
        "-2",
        53,
        Floor,
        "-0.8646647167633874",
        "-0x0.dd5aaab880fc70#53",
        Less,
    );
    test(
        "-2",
        53,
        Ceiling,
        "-0.8646647167633873",
        "-0x0.dd5aaab880fc68#53",
        Greater,
    );
    test(
        "-2",
        53,
        Nearest,
        "-0.8646647167633873",
        "-0x0.dd5aaab880fc68#53",
        Greater,
    );
    test("100", 1, Floor, "2.0e43", "0x1.0E+36#1", Less);
    test("100", 1, Ceiling, "4.0e43", "0x2.0E+36#1", Greater);
    test("100", 1, Nearest, "2.0e43", "0x1.0E+36#1", Less);
    test("100", 10, Floor, "2.687e43", "0x1.348E+36#10", Less);
    test("100", 10, Ceiling, "2.692e43", "0x1.350E+36#10", Greater);
    test("100", 10, Nearest, "2.687e43", "0x1.348E+36#10", Less);
    test(
        "100",
        53,
        Floor,
        "2.6881171418161351e43",
        "0x1.3494a9b171bf4E+36#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "2.6881171418161356e43",
        "0x1.3494a9b171bf5E+36#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "2.6881171418161356e43",
        "0x1.3494a9b171bf5E+36#53",
        Greater,
    );
    test("-100", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-100", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-100", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-100", 10, Floor, "-1.0", "-0x1.000#10", Less);
    test("-100", 10, Ceiling, "-0.999", "-0x0.ffc#10", Greater);
    test("-100", 10, Nearest, "-1.0", "-0x1.000#10", Less);
    test("-100", 53, Floor, "-1.0", "-0x1.0000000000000#53", Less);
    test(
        "-100",
        53,
        Ceiling,
        "-0.9999999999999999",
        "-0x0.fffffffffffff8#53",
        Greater,
    );
    test("-100", 53, Nearest, "-1.0", "-0x1.0000000000000#53", Less);
    // - general-squeeze Ziv retry: the bounds disagree at working precision `prec + 10`, so the
    //   loop raises the precision at least once before converging.
    test("262144/63", 2, Nearest, "1.0e1807", "0x8.0E+1500#2", Less);

    let test_big = |x: Rational, prec, rm, out: &str, out_hex: &str, out_o| {
        let (f, o) = Float::exp_x_minus_1_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    // |x| too large to be a finite Float, x > 0: expm1 overflows. Directed-down rounding returns
    // the largest finite Float; the other modes return +inf.
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // |x| too large to be a finite Float, x < 0: expm1 tends to -1. The result is -1 (toward -inf
    // or away from zero) or its toward-zero neighbor (toward +inf or toward zero).
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Down,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    // |x| < 2^MIN_EXPONENT: expm1(x) ~ x underflows, via the Taylor-series helper. For x > 0 the
    // toward-zero modes give the min subnormal; for x < 0 they give -0. (Also exercises the
    // half-GCD `m_lens[1][0] == 0` path in malachite-nz.)
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Floor,
        "too_small",
        "0x1.0E-268435456#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Ceiling,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );

    // - huge-negative x with a precision so large that `float_round_near_x` cannot resolve the
    //   rounding (its error bound MAX_EXPONENT <= prec + 1), exercising the manual -1 / -1+ulp
    //   fallback. The result is a ~128 MB Float, so it is checked by value rather than by string.
    let prec = u64::exact_from(Float::MAX_EXPONENT);
    let x = -Rational::power_of_2(i64::from(Float::MAX_EXPONENT));
    let neg_one = -Float::one_prec(prec);
    // -1 + ulp, i.e. the neighbor of -1 toward zero.
    let neg_one_plus_ulp = neg_one
        .clone()
        .add_prec_round(Float::power_of_2(-i64::exact_from(prec)), prec, Exact)
        .0;
    // Toward -inf or away from zero rounds to -1.
    for rm in [Floor, Up, Nearest] {
        let (f, o) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&neg_one));
        assert_eq!(o, Less);
    }
    // Toward zero (Ceiling for a negative value, or Down) rounds to -1 + ulp.
    for rm in [Ceiling, Down] {
        let (f, o) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(
            ComparableFloatRef(&f),
            ComparableFloatRef(&neg_one_plus_ulp)
        );
        assert_eq!(o, Greater);
    }

    // - deeply negative x with the bits of e^x inside the output window: the squeeze's Float
    //   bracket ends route through the Float function's deep-negative helper (this used to panic).
    //   See the corresponding case in `test_exp_x_minus_1_prec_round` for the numbers; here just
    //   check that 1 + f lands in the right binade.
    let prec = u64::exact_from(Float::MAX_EXPONENT) + 65;
    let x = Rational::from(-744261120i64);
    let (f, o) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, Nearest);
    assert!(f.is_valid());
    assert_eq!(f.get_prec(), Some(prec));
    assert_ne!(o, Equal);
    let r = Rational::exact_from(&f) + Rational::ONE;
    assert!(r > 0u32);
    assert_eq!(r.floor_log_base_2(), -1073741827);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_round_fail_1() {
    Float::exp_x_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_round_fail_2() {
    Float::exp_x_minus_1_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn exp_x_minus_1_rational_prec_round_ref_fail() {
    Float::exp_x_minus_1_rational_prec_round_ref(&Rational::ONE, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn exp_x_minus_1_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (f, o) = Float::exp_x_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(f.is_valid());

    let (f_alt, o_alt) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, rm);
    assert!(f_alt.is_valid());
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    // expm1 has the same sign as x, and is never NaN.
    if x > 0u32 {
        assert!(f >= 0u32);
    } else if x < 0u32 {
        assert!(f <= 0u32);
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_f, rug_o) = rug_exp_x_minus_1_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_f)),
            ComparableFloatRef(&f)
        );
        assert_eq!(rug_o, o);
    }

    if f.is_normal() {
        assert_eq!(f.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(f.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::exp_x_minus_1_rational_prec_round_ref(
            &x, prec, Exact
        ));
    }
}

#[test]
fn exp_x_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        exp_x_minus_1_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (f, o) = Float::exp_x_minus_1_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(f), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn exp_x_minus_1_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (f, o) = Float::exp_x_minus_1_rational_prec(x.clone(), prec);
    assert!(f.is_valid());

    let (f_alt, o_alt) = Float::exp_x_minus_1_rational_prec_ref(&x, prec);
    assert!(f_alt.is_valid());
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    let (f_alt, o_alt) = Float::exp_x_minus_1_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    if x > 0u32 {
        assert!(f >= 0u32);
    } else if x < 0u32 {
        assert!(f <= 0u32);
    }

    let (rug_f, rug_o) = rug_exp_x_minus_1_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_f)),
        ComparableFloatRef(&f)
    );
    assert_eq!(rug_o, o);

    if f.is_normal() {
        assert_eq!(f.get_prec(), Some(prec));
    }
}

#[test]
fn exp_x_minus_1_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        exp_x_minus_1_rational_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (f, o) = Float::exp_x_minus_1_rational_prec(Rational::ZERO, prec);
        assert_eq!(ComparableFloat(f), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_exp_x_minus_1_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_exp_x_minus_1_rational(&u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f32>("1", 1.7182819);
    test::<f32>("1/2", 0.6487213);
    test::<f32>("1/3", 0.39561242);
    test::<f32>("22/7", 22.169971);
    test::<f32>("1000000", f32::INFINITY);
    test::<f32>("1/1000000", 0.0000010000005);
    test::<f32>("-1", -0.63212055);
    test::<f32>("-1/2", -0.39346933);
    test::<f32>("-1/3", -0.2834687);
    test::<f32>("-22/7", -0.9568407);
    test::<f32>("-1000000", -1.0);

    test::<f64>("0", 0.0);
    test::<f64>("1", 1.7182818284590453);
    test::<f64>("1/2", 0.6487212707001282);
    test::<f64>("1/3", 0.3956124250860895);
    test::<f64>("22/7", 22.16997229826248);
    test::<f64>("1000000", f64::INFINITY);
    test::<f64>("1/1000000", 1.0000005000001667e-6);
    test::<f64>("-1", -0.6321205588285577);
    test::<f64>("-1/2", -0.3934693402873666);
    test::<f64>("-1/3", -0.28346868942621073);
    test::<f64>("-22/7", -0.9568406907385474);
    test::<f64>("-1000000", -1.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_exp_x_minus_1_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let y = primitive_float_exp_x_minus_1_rational::<T>(&x);
        // expm1 has the same sign as x, and is never NaN.
        assert!(!y.is_nan());
        if x > 0u32 {
            assert!(y >= T::ZERO);
        } else if x < 0u32 {
            assert!(y <= T::ZERO);
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // expm1 of a finite, nonzero primitive float, taken through the `Rational` path, matches
        // the direct primitive-float expm1. Zero is excluded: a `Rational` has no signed zero, so
        // the `Rational` path returns +0 for both signs whereas the direct path preserves it
        // (expm1(-0.0) = -0.0).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_exp_x_minus_1_rational::<T>(
                    &Rational::exact_from(x)
                )),
                NiceFloat(primitive_float_exp_x_minus_1(x))
            );
        }
    });
}

#[test]
fn primitive_float_exp_x_minus_1_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_exp_x_minus_1_rational_properties_helper);
}
