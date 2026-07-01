// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2XMinus1, PowerOf2XMinus1Assign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    rounding_mode_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::test_util::arithmetic::power_of_2_x_minus_1::{
    rug_power_of_2_x_minus_1, rug_power_of_2_x_minus_1_prec, rug_power_of_2_x_minus_1_prec_round,
    rug_power_of_2_x_minus_1_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use std::panic::catch_unwind;

#[test]
fn test_power_of_2_x_minus_1() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let power_of_2_x_minus_1 = x.clone().power_of_2_x_minus_1();
        assert!(power_of_2_x_minus_1.is_valid());

        assert_eq!(power_of_2_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_2_x_minus_1), out_hex);

        let power_of_2_x_minus_1_alt = (&x).power_of_2_x_minus_1();
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );

        let mut power_of_2_x_minus_1_alt = x.clone();
        power_of_2_x_minus_1_alt.power_of_2_x_minus_1_assign();
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1(
                &rug::Float::exact_from(&x)
            ))),
            ComparableFloatRef(&power_of_2_x_minus_1)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-1.0", "-0x1.0#1");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "-0.5", "-0x0.8#1");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1.0",
        "0x1.0000000000000000000000000#100",
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "5.5808859910179205",
        "0x5.94b4f1be20638#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "7.824977827076287",
        "0x7.d331bf3337c18#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-0.8866852677032391",
        "-0x0.e2fdce42a16300#53",
    );
    test("123.0", "0x7b.0#7", "1.06e37", "0x8.0E+30#7");
    test("-123.0", "-0x7b.0#7", "-1.0", "-0x1.00#7");
    test("0.5", "0x0.8#1", "0.5", "0x0.8#1");
    test("-0.5", "-0x0.8#1", "-0.2", "-0x0.4#1");
    test("-100.0", "-0x64.0#7", "-1.0", "-0x1.00#7");
    test("-1000.0", "-0x3e8.0#10", "-1.0", "-0x1.000#10");
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        "1.399799583508251067722204e-28",
        "0xb.17217f7d1cf79abc9e4E-24#80",
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        "-1.399799583508251067722204e-28",
        "-0xb.17217f7d1cf79abc9e4E-24#80",
    );
}

#[test]
fn test_power_of_2_x_minus_1_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (power_of_2_x_minus_1, o) = x.clone().power_of_2_x_minus_1_prec(prec);
        assert!(power_of_2_x_minus_1.is_valid());

        assert_eq!(power_of_2_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_2_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (power_of_2_x_minus_1_alt, o_alt) = x.power_of_2_x_minus_1_prec_ref(prec);
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut power_of_2_x_minus_1_alt = x.clone();
        let o_alt = power_of_2_x_minus_1_alt.power_of_2_x_minus_1_prec_assign(prec);
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let (rug_power_of_2_x_minus_1, rug_o) =
            rug_power_of_2_x_minus_1_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1)),
            ComparableFloatRef(&power_of_2_x_minus_1),
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
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 53, "1.0", "0x1.0000000000000#53", Equal);
    test("-1.0", "-0x1.0#1", 1, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", 10, "-0.5", "-0x0.800#10", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        53,
        "-0.5",
        "-0x0.80000000000000#53",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        53,
        "1.0",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "5.58",
        "0x5.94#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        53,
        "5.5808859910179205",
        "0x5.94b4f1be20638#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "7.83",
        "0x7.d4#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        53,
        "7.824977827076287",
        "0x7.d331bf3337c18#53",
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
        "-0.887",
        "-0x0.e30#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        53,
        "-0.8866852677032391",
        "-0x0.e2fdce42a16300#53",
        Greater,
    );
    test("123.0", "0x7b.0#7", 1, "1.0e37", "0x8.0E+30#1", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        "1.063e37",
        "0x8.00E+30#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        53,
        "1.0633823966279327e37",
        "0x8.0000000000000E+30#53",
        Greater,
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
    test("0.5", "0x0.8#1", 1, "0.5", "0x0.8#1", Greater);
    test("0.5", "0x0.8#1", 10, "0.4141", "0x0.6a0#10", Less);
    test(
        "0.5",
        "0x0.8#1",
        53,
        "0.41421356237309503",
        "0x0.6a09e667f3bcc8#53",
        Less,
    );
    test("-0.5", "-0x0.8#1", 1, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", 10, "-0.293", "-0x0.4b0#10", Less);
    test(
        "-0.5",
        "-0x0.8#1",
        53,
        "-0.29289321881345248",
        "-0x0.4afb0ccc06219c#53",
        Less,
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
        "1.0e-28",
        "0x8.0E-24#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        "1.4e-28",
        "0xb.18E-24#10",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        53,
        "1.399799583508251e-28",
        "0xb.17217f7d1cf78E-24#53",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        "-1.0e-28",
        "-0x8.0E-24#1",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        "-1.4e-28",
        "-0xb.18E-24#10",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        53,
        "-1.399799583508251e-28",
        "-0xb.17217f7d1cf78E-24#53",
        Greater,
    );
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_prec_fail() {
    Float::NAN.power_of_2_x_minus_1_prec(0);
}

#[test]
fn test_power_of_2_x_minus_1_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (power_of_2_x_minus_1, o) = x.clone().power_of_2_x_minus_1_round(rm);
        assert!(power_of_2_x_minus_1.is_valid());

        assert_eq!(power_of_2_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_2_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (power_of_2_x_minus_1_alt, o_alt) = x.power_of_2_x_minus_1_round_ref(rm);
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut power_of_2_x_minus_1_alt = x.clone();
        let o_alt = power_of_2_x_minus_1_alt.power_of_2_x_minus_1_round_assign(rm);
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_power_of_2_x_minus_1, rug_o) =
                rug_power_of_2_x_minus_1_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1)),
                ComparableFloatRef(&power_of_2_x_minus_1),
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
    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("-1.0", "-0x1.0#1", Floor, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", Down, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", Up, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "-0.5", "-0x0.8#1", Equal);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "5.5808859910179196",
        "0x5.94b4f1be20634#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "5.5808859910179205",
        "0x5.94b4f1be20638#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "5.5808859910179196",
        "0x5.94b4f1be20634#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "5.5808859910179205",
        "0x5.94b4f1be20638#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "5.5808859910179205",
        "0x5.94b4f1be20638#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "7.824977827076286",
        "0x7.d331bf3337c14#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "7.824977827076287",
        "0x7.d331bf3337c18#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "7.824977827076286",
        "0x7.d331bf3337c14#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "7.824977827076287",
        "0x7.d331bf3337c18#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "7.824977827076287",
        "0x7.d331bf3337c18#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-0.8866852677032392",
        "-0x0.e2fdce42a16308#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-0.8866852677032391",
        "-0x0.e2fdce42a16300#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-0.8866852677032391",
        "-0x0.e2fdce42a16300#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-0.8866852677032392",
        "-0x0.e2fdce42a16308#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-0.8866852677032391",
        "-0x0.e2fdce42a16300#53",
        Greater,
    );
    test("123.0", "0x7b.0#7", Floor, "1.055e37", "0x7.fE+30#7", Less);
    test(
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "1.06e37",
        "0x8.0E+30#7",
        Greater,
    );
    test("123.0", "0x7b.0#7", Down, "1.055e37", "0x7.fE+30#7", Less);
    test("123.0", "0x7b.0#7", Up, "1.06e37", "0x8.0E+30#7", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        Nearest,
        "1.06e37",
        "0x8.0E+30#7",
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
    test("0.5", "0x0.8#1", Floor, "0.2", "0x0.4#1", Less);
    test("0.5", "0x0.8#1", Ceiling, "0.5", "0x0.8#1", Greater);
    test("0.5", "0x0.8#1", Down, "0.2", "0x0.4#1", Less);
    test("0.5", "0x0.8#1", Up, "0.5", "0x0.8#1", Greater);
    test("0.5", "0x0.8#1", Nearest, "0.5", "0x0.8#1", Greater);
    test("-0.5", "-0x0.8#1", Floor, "-0.5", "-0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", Ceiling, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", Down, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", Up, "-0.5", "-0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", Nearest, "-0.2", "-0x0.4#1", Greater);
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
        "1.399799583508251067722202e-28",
        "0xb.17217f7d1cf79abc9e3E-24#80",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Ceiling,
        "1.399799583508251067722204e-28",
        "0xb.17217f7d1cf79abc9e4E-24#80",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Down,
        "1.399799583508251067722202e-28",
        "0xb.17217f7d1cf79abc9e3E-24#80",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Up,
        "1.399799583508251067722204e-28",
        "0xb.17217f7d1cf79abc9e4E-24#80",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Nearest,
        "1.399799583508251067722204e-28",
        "0xb.17217f7d1cf79abc9e4E-24#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Floor,
        "-1.399799583508251067722204e-28",
        "-0xb.17217f7d1cf79abc9e4E-24#80",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Ceiling,
        "-1.399799583508251067722202e-28",
        "-0xb.17217f7d1cf79abc9e3E-24#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Down,
        "-1.399799583508251067722202e-28",
        "-0xb.17217f7d1cf79abc9e3E-24#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Up,
        "-1.399799583508251067722204e-28",
        "-0xb.17217f7d1cf79abc9e4E-24#80",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Nearest,
        "-1.399799583508251067722204e-28",
        "-0xb.17217f7d1cf79abc9e4E-24#80",
        Less,
    );
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_round_fail() {
    // 2^1.5 - 1 is irrational, so it cannot be represented exactly.
    parse_hex_string("0x1.8#10").power_of_2_x_minus_1_round(Exact);
}

#[test]
fn test_power_of_2_x_minus_1_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (power_of_2_x_minus_1, o) = x.clone().power_of_2_x_minus_1_prec_round(prec, rm);
        assert!(power_of_2_x_minus_1.is_valid());

        assert_eq!(power_of_2_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_2_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (power_of_2_x_minus_1_alt, o_alt) = x.power_of_2_x_minus_1_prec_round_ref(prec, rm);
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut power_of_2_x_minus_1_alt = x.clone();
        let o_alt = power_of_2_x_minus_1_alt.power_of_2_x_minus_1_prec_round_assign(prec, rm);
        assert!(power_of_2_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_2_x_minus_1),
            ComparableFloatRef(&power_of_2_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_power_of_2_x_minus_1, rug_o) =
                rug_power_of_2_x_minus_1_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1)),
                ComparableFloatRef(&power_of_2_x_minus_1),
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
    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 1, Floor, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", 1, Nearest, "-0.5", "-0x0.8#1", Equal);
    test("-1.0", "-0x1.0#1", 10, Floor, "-0.5", "-0x0.800#10", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-0.5",
        "-0x0.800#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.5",
        "-0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Floor,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Floor,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Ceiling,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "5.58",
        "0x5.94#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "5.586",
        "0x5.96#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "5.58",
        "0x5.94#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "7.82",
        "0x7.d2#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "7.83",
        "0x7.d4#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "7.83",
        "0x7.d4#10",
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
        "-0.887",
        "-0x0.e30#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.886",
        "-0x0.e2c#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.887",
        "-0x0.e30#10",
        Less,
    );
    test("123.0", "0x7b.0#7", 1, Floor, "5.0e36", "0x4.0E+30#1", Less);
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "1.0e37",
        "0x8.0E+30#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "1.0e37",
        "0x8.0E+30#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "1.062e37",
        "0x7.feE+30#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "1.063e37",
        "0x8.00E+30#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "1.063e37",
        "0x8.00E+30#10",
        Greater,
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
    test("0.5", "0x0.8#1", 1, Floor, "0.2", "0x0.4#1", Less);
    test("0.5", "0x0.8#1", 1, Ceiling, "0.5", "0x0.8#1", Greater);
    test("0.5", "0x0.8#1", 1, Nearest, "0.5", "0x0.8#1", Greater);
    test("0.5", "0x0.8#1", 10, Floor, "0.4141", "0x0.6a0#10", Less);
    test(
        "0.5",
        "0x0.8#1",
        10,
        Ceiling,
        "0.4146",
        "0x0.6a2#10",
        Greater,
    );
    test("0.5", "0x0.8#1", 10, Nearest, "0.4141", "0x0.6a0#10", Less);
    test("-0.5", "-0x0.8#1", 1, Floor, "-0.5", "-0x0.8#1", Less);
    test("-0.5", "-0x0.8#1", 1, Ceiling, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", 1, Nearest, "-0.2", "-0x0.4#1", Greater);
    test("-0.5", "-0x0.8#1", 10, Floor, "-0.293", "-0x0.4b0#10", Less);
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Ceiling,
        "-0.2925",
        "-0x0.4ae#10",
        Greater,
    );
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Nearest,
        "-0.293",
        "-0x0.4b0#10",
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
        "1.0e-28",
        "0x8.0E-24#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        Ceiling,
        "2.0e-28",
        "0x1.0E-23#1",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        Nearest,
        "1.0e-28",
        "0x8.0E-24#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Floor,
        "1.398e-28",
        "0xb.14E-24#10",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Ceiling,
        "1.4e-28",
        "0xb.18E-24#10",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Nearest,
        "1.4e-28",
        "0xb.18E-24#10",
        Greater,
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
        "-1.0e-28",
        "-0x8.0E-24#1",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Floor,
        "-1.4e-28",
        "-0xb.18E-24#10",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Ceiling,
        "-1.398e-28",
        "-0xb.14E-24#10",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Nearest,
        "-1.4e-28",
        "-0xb.18E-24#10",
        Less,
    );
    test(
        "0.2",
        "0x0.4#2",
        10,
        Nearest,
        "0.1892",
        "0x0.307#10",
        Greater,
    );
    test(
        "-0.00390625",
        "-0x0.010000000000000000#64",
        1,
        Ceiling,
        "-0.002",
        "-0x0.008#1",
        Greater,
    );
    test(
        "-18.75",
        "-0x12.c00000000000000#64",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-1.0e9",
        "-0x4.0E+7#1",
        7,
        Nearest,
        "-1.0",
        "-0x1.00#7",
        Less,
    );
    test(
        "1.0e9",
        "0x4.0E+7#1",
        7,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-100.0",
        "-0x64.0#7",
        200,
        Nearest,
        "-0.9999999999999999999999999999992111390947789881945882714347172",
        "-0x0.fffffffffffffffffffffffff0000000000000000000000000#200",
        Equal,
    );
    // Integer x makes 2^x - 1 an exact integer, so (unlike expm1) `Exact` succeeds.
    test("3.0", "0x3.0#2", 10, Exact, "7.0", "0x7.00#10", Equal);
    test("6.0", "0x6.0#3", 20, Exact, "63.0", "0x3f.0000#20", Equal);
    test(
        "-100.0",
        "-0x64.0#7",
        200,
        Exact,
        "-0.9999999999999999999999999999992111390947789881945882714347172",
        "-0x0.fffffffffffffffffffffffff0000000000000000000000000#200",
        Equal,
    );
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_prec_round_fail_1() {
    Float::NAN.power_of_2_x_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_x_minus_1_prec_round_fail_2() {
    // 2^1.5 - 1 is irrational, so it cannot be represented exactly.
    parse_hex_string("0x1.8#10").power_of_2_x_minus_1_prec_round(10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_x_minus_1_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (power_of_2_x_minus_1, o) = x.clone().power_of_2_x_minus_1_prec_round(prec, rm);
    assert!(power_of_2_x_minus_1.is_valid());

    let (power_of_2_x_minus_1_alt, o_alt) = x.power_of_2_x_minus_1_prec_round_ref(prec, rm);
    assert!(power_of_2_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_2_x_minus_1_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_power_of_2_x_minus_1, rug_o) =
            rug_power_of_2_x_minus_1_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1)),
            ComparableFloatRef(&power_of_2_x_minus_1),
        );
        assert_eq!(rug_o, o);
    }

    if power_of_2_x_minus_1.is_normal() {
        assert_eq!(power_of_2_x_minus_1.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(power_of_2_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(power_of_2_x_minus_1 < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.power_of_2_x_minus_1_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(power_of_2_x_minus_1.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.power_of_2_x_minus_1_prec_round_ref(prec, Exact));
    }
}

#[test]
fn power_of_2_x_minus_1_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        power_of_2_x_minus_1_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            power_of_2_x_minus_1_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.power_of_2_x_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.power_of_2_x_minus_1_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (s, o) = Float::NEGATIVE_INFINITY.power_of_2_x_minus_1_prec_round(prec, rm);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.power_of_2_x_minus_1_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.power_of_2_x_minus_1_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_x_minus_1_prec_properties_helper(x: Float, prec: u64) {
    let (power_of_2_x_minus_1, o) = x.clone().power_of_2_x_minus_1_prec(prec);
    assert!(power_of_2_x_minus_1.is_valid());

    let (power_of_2_x_minus_1_alt, o_alt) = x.power_of_2_x_minus_1_prec_ref(prec);
    assert!(power_of_2_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_2_x_minus_1_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let (rug_power_of_2_x_minus_1, rug_o) =
        rug_power_of_2_x_minus_1_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1)),
        ComparableFloatRef(&power_of_2_x_minus_1),
    );
    assert_eq!(rug_o, o);

    let (power_of_2_x_minus_1_alt, o_alt) = x.power_of_2_x_minus_1_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if power_of_2_x_minus_1.is_normal() {
        assert_eq!(power_of_2_x_minus_1.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(power_of_2_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(power_of_2_x_minus_1 < 0u32);
        }
    }
}

#[test]
fn power_of_2_x_minus_1_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        power_of_2_x_minus_1_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        power_of_2_x_minus_1_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        power_of_2_x_minus_1_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.power_of_2_x_minus_1_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.power_of_2_x_minus_1_prec(prec);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.power_of_2_x_minus_1_prec(prec);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.power_of_2_x_minus_1_prec(prec),
            (Float::INFINITY, Equal)
        );
        let (s, o) = Float::NEGATIVE_INFINITY.power_of_2_x_minus_1_prec(prec);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_x_minus_1_round_properties_helper(x: Float, rm: RoundingMode) {
    let (power_of_2_x_minus_1, o) = x.clone().power_of_2_x_minus_1_round(rm);
    assert!(power_of_2_x_minus_1.is_valid());

    let (power_of_2_x_minus_1_alt, o_alt) = x.power_of_2_x_minus_1_round_ref(rm);
    assert!(power_of_2_x_minus_1_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_2_x_minus_1_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let (power_of_2_x_minus_1_alt, o_alt) =
        x.power_of_2_x_minus_1_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if x.is_finite() {
        // 2^x > 0, so 2^x - 1 > -1; rounding down can reach but not pass -1.
        assert!(power_of_2_x_minus_1 >= Float::NEGATIVE_ONE);
    }

    if power_of_2_x_minus_1.is_normal() {
        // For finite x the result has x's precision; for x = -inf the result is -1 (normal, but x
        // has no precision), so the precision check only applies when x is finite.
        if let Some(p) = x.get_prec() {
            assert_eq!(power_of_2_x_minus_1.get_prec(), Some(p));
        }
        if x > 0u32 && o > Less {
            assert!(power_of_2_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(power_of_2_x_minus_1 < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_power_of_2_x_minus_1, rug_o) =
            rug_power_of_2_x_minus_1_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1)),
            ComparableFloatRef(&power_of_2_x_minus_1),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.power_of_2_x_minus_1_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(power_of_2_x_minus_1.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.power_of_2_x_minus_1_round_ref(Exact));
    }
}

#[test]
fn power_of_2_x_minus_1_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        power_of_2_x_minus_1_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (s, o) = Float::NAN.power_of_2_x_minus_1_round(rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.power_of_2_x_minus_1_round(rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.power_of_2_x_minus_1_round(rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.power_of_2_x_minus_1_round(rm),
            (Float::INFINITY, Equal)
        );
        let (s, o) = Float::NEGATIVE_INFINITY.power_of_2_x_minus_1_round(rm);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_x_minus_1_properties_helper(x: Float) {
    let power_of_2_x_minus_1 = x.clone().power_of_2_x_minus_1();
    assert!(power_of_2_x_minus_1.is_valid());

    let power_of_2_x_minus_1_alt = (&x).power_of_2_x_minus_1();
    assert!(power_of_2_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );

    let mut x_alt = x.clone();
    x_alt.power_of_2_x_minus_1_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );

    let power_of_2_x_minus_1_alt = x
        .power_of_2_x_minus_1_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );
    let power_of_2_x_minus_1_alt = x.power_of_2_x_minus_1_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );

    let power_of_2_x_minus_1_alt = x.power_of_2_x_minus_1_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&power_of_2_x_minus_1_alt),
        ComparableFloatRef(&power_of_2_x_minus_1)
    );

    if x.is_finite() {
        // 2^x > 0, so 2^x - 1 > -1; rounding down can reach but not pass -1.
        assert!(power_of_2_x_minus_1 >= Float::NEGATIVE_ONE);
    }

    let rug_power_of_2_x_minus_1 = rug_power_of_2_x_minus_1(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_power_of_2_x_minus_1)),
        ComparableFloatRef(&power_of_2_x_minus_1),
    );
}

#[test]
fn power_of_2_x_minus_1_properties() {
    float_gen().test_properties(|x| {
        power_of_2_x_minus_1_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        power_of_2_x_minus_1_properties_helper(x);
    });
}
