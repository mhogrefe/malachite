// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{PowerOf2, PowerOf10XMinus1, PowerOf10XMinus1Assign};
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
use malachite_float::arithmetic::power_of_10_x_minus_1::{
    primitive_float_power_of_10_x_minus_1, primitive_float_power_of_10_x_minus_1_rational,
};
use malachite_float::test_util::arithmetic::power_of_10_x_minus_1::{
    rug_power_of_10_x_minus_1, rug_power_of_10_x_minus_1_prec,
    rug_power_of_10_x_minus_1_prec_round, rug_power_of_10_x_minus_1_rational_prec,
    rug_power_of_10_x_minus_1_rational_prec_round, rug_power_of_10_x_minus_1_round,
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
fn test_power_of_10_x_minus_1() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let power_of_10_x_minus_1 = x.clone().power_of_10_x_minus_1();
        assert!(power_of_10_x_minus_1.is_valid());

        assert_eq!(power_of_10_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_10_x_minus_1), out_hex);

        let power_of_10_x_minus_1_alt = (&x).power_of_10_x_minus_1();
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );

        let mut power_of_10_x_minus_1_alt = x.clone();
        power_of_10_x_minus_1_alt.power_of_10_x_minus_1_assign();
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1(
                &rug::Float::exact_from(&x)
            ))),
            ComparableFloatRef(&power_of_10_x_minus_1)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-1.0", "-0x1.0#1");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "8.0", "0x8.0#1");
    test("-1.0", "-0x1.0#1", "-1.0", "-0x1.0#1");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "9.0",
        "0x9.000000000000000000000000#100",
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "521.7352996704365",
        "0x209.bc3c996548c#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1384.4557313670107",
        "0x568.74aacf95128#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-0.9992782158409252",
        "-0x0.ffd0b278a45a50#53",
    );
    test("123.0", "0x7b.0#7", "1.0e123", "0x1.84E+102#7");
    test("-123.0", "-0x7b.0#7", "-1.0", "-0x1.00#7");
    test("0.5", "0x0.8#1", "2.0", "0x2.0#1");
    test("-0.5", "-0x0.8#1", "-0.5", "-0x0.8#1");
    test("-100.0", "-0x64.0#7", "-1.0", "-0x1.00#7");
    test("-1000.0", "-0x3e8.0#10", "-1.0", "-0x1.000#10");
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        "4.650033563667687747641662e-28",
        "0x2.4d763776aaa2b05ba95cE-23#80",
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        "-4.650033563667687747641662e-28",
        "-0x2.4d763776aaa2b05ba95cE-23#80",
    );
}

#[test]
fn test_power_of_10_x_minus_1_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (power_of_10_x_minus_1, o) = x.clone().power_of_10_x_minus_1_prec(prec);
        assert!(power_of_10_x_minus_1.is_valid());

        assert_eq!(power_of_10_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_10_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (power_of_10_x_minus_1_alt, o_alt) = x.power_of_10_x_minus_1_prec_ref(prec);
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut power_of_10_x_minus_1_alt = x.clone();
        let o_alt = power_of_10_x_minus_1_alt.power_of_10_x_minus_1_prec_assign(prec);
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let (rug_power_of_10_x_minus_1, rug_o) =
            rug_power_of_10_x_minus_1_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1)),
            ComparableFloatRef(&power_of_10_x_minus_1),
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
    test("1.0", "0x1.0#1", 1, "8.0", "0x8.0#1", Less);
    test("1.0", "0x1.0#1", 10, "9.0", "0x9.00#10", Equal);
    test("1.0", "0x1.0#1", 53, "9.0", "0x9.0000000000000#53", Equal);
    test("-1.0", "-0x1.0#1", 1, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", 10, "-0.9", "-0x0.e68#10", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        53,
        "-0.9",
        "-0x0.e6666666666668#53",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        "9.0",
        "0x9.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        53,
        "9.0",
        "0x9.0000000000000#53",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "5.0e2",
        "0x2.0E+2#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "522.0",
        "0x20a.0#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        53,
        "521.7352996704365",
        "0x209.bc3c996548c#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "1.0e3",
        "0x4.0E+2#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "1384.0",
        "0x568.0#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        53,
        "1384.4557313670107",
        "0x568.74aacf95128#53",
        Less,
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
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        53,
        "-0.9992782158409252",
        "-0x0.ffd0b278a45a50#53",
        Greater,
    );
    test("123.0", "0x7b.0#7", 1, "1.0e123", "0x2.0E+102#1", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        "1.001e123",
        "0x1.838E+102#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        53,
        "9.9999999999999998e122",
        "0x1.83425a5f872f1E+102#53",
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
    test("0.5", "0x0.8#1", 1, "2.0", "0x2.0#1", Less);
    test("0.5", "0x0.8#1", 10, "2.164", "0x2.2a#10", Greater);
    test(
        "0.5",
        "0x0.8#1",
        53,
        "2.1622776601683795",
        "0x2.298b075b4b6a6#53",
        Greater,
    );
    test("-0.5", "-0x0.8#1", 1, "-0.5", "-0x0.8#1", Greater);
    test("-0.5", "-0x0.8#1", 10, "-0.684", "-0x0.af0#10", Greater);
    test(
        "-0.5",
        "-0x0.8#1",
        53,
        "-0.6837722339831621",
        "-0x0.af0bb276dedbc8#53",
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
        "4.0e-28",
        "0x2.0E-23#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        "4.646e-28",
        "0x2.4dE-23#10",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        53,
        "4.650033563667688e-28",
        "0x2.4d763776aaa2cE-23#53",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        "-4.0e-28",
        "-0x2.0E-23#1",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        "-4.646e-28",
        "-0x2.4dE-23#10",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        53,
        "-4.650033563667688e-28",
        "-0x2.4d763776aaa2cE-23#53",
        Less,
    );
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_prec_fail() {
    Float::NAN.power_of_10_x_minus_1_prec(0);
}

#[test]
fn test_power_of_10_x_minus_1_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (power_of_10_x_minus_1, o) = x.clone().power_of_10_x_minus_1_round(rm);
        assert!(power_of_10_x_minus_1.is_valid());

        assert_eq!(power_of_10_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_10_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (power_of_10_x_minus_1_alt, o_alt) = x.power_of_10_x_minus_1_round_ref(rm);
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut power_of_10_x_minus_1_alt = x.clone();
        let o_alt = power_of_10_x_minus_1_alt.power_of_10_x_minus_1_round_assign(rm);
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_power_of_10_x_minus_1, rug_o) =
                rug_power_of_10_x_minus_1_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1)),
                ComparableFloatRef(&power_of_10_x_minus_1),
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
    test("1.0", "0x1.0#1", Floor, "8.0", "0x8.0#1", Less);
    test("1.0", "0x1.0#1", Ceiling, "2.0e1", "0x1.0E+1#1", Greater);
    test("1.0", "0x1.0#1", Down, "8.0", "0x8.0#1", Less);
    test("1.0", "0x1.0#1", Up, "2.0e1", "0x1.0E+1#1", Greater);
    test("1.0", "0x1.0#1", Nearest, "8.0", "0x8.0#1", Less);
    test("-1.0", "-0x1.0#1", Floor, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", Down, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", Up, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Floor,
        "9.0",
        "0x9.000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Ceiling,
        "9.0",
        "0x9.000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Down,
        "9.0",
        "0x9.000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Up,
        "9.0",
        "0x9.000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Nearest,
        "9.0",
        "0x9.000000000000000000000000#100",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "521.7352996704365",
        "0x209.bc3c996548c#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "521.7352996704366",
        "0x209.bc3c996548e#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "521.7352996704365",
        "0x209.bc3c996548c#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "521.7352996704366",
        "0x209.bc3c996548e#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "521.7352996704365",
        "0x209.bc3c996548c#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "1384.4557313670107",
        "0x568.74aacf95128#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "1384.4557313670109",
        "0x568.74aacf9512c#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "1384.4557313670107",
        "0x568.74aacf95128#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "1384.4557313670109",
        "0x568.74aacf9512c#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "1384.4557313670107",
        "0x568.74aacf95128#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "-0.9992782158409254",
        "-0x0.ffd0b278a45a58#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "-0.9992782158409252",
        "-0x0.ffd0b278a45a50#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "-0.9992782158409252",
        "-0x0.ffd0b278a45a50#53",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "-0.9992782158409254",
        "-0x0.ffd0b278a45a58#53",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "-0.9992782158409252",
        "-0x0.ffd0b278a45a50#53",
        Greater,
    );
    test("123.0", "0x7b.0#7", Floor, "9.9e122", "0x1.80E+102#7", Less);
    test(
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "1.0e123",
        "0x1.84E+102#7",
        Greater,
    );
    test("123.0", "0x7b.0#7", Down, "9.9e122", "0x1.80E+102#7", Less);
    test("123.0", "0x7b.0#7", Up, "1.0e123", "0x1.84E+102#7", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        Nearest,
        "1.0e123",
        "0x1.84E+102#7",
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
    test("0.5", "0x0.8#1", Floor, "2.0", "0x2.0#1", Less);
    test("0.5", "0x0.8#1", Ceiling, "4.0", "0x4.0#1", Greater);
    test("0.5", "0x0.8#1", Down, "2.0", "0x2.0#1", Less);
    test("0.5", "0x0.8#1", Up, "4.0", "0x4.0#1", Greater);
    test("0.5", "0x0.8#1", Nearest, "2.0", "0x2.0#1", Less);
    test("-0.5", "-0x0.8#1", Floor, "-1.0", "-0x1.0#1", Less);
    test("-0.5", "-0x0.8#1", Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-0.5", "-0x0.8#1", Down, "-0.5", "-0x0.8#1", Greater);
    test("-0.5", "-0x0.8#1", Up, "-1.0", "-0x1.0#1", Less);
    test("-0.5", "-0x0.8#1", Nearest, "-0.5", "-0x0.8#1", Greater);
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
        "4.650033563667687747641655e-28",
        "0x2.4d763776aaa2b05ba958E-23#80",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Ceiling,
        "4.650033563667687747641662e-28",
        "0x2.4d763776aaa2b05ba95cE-23#80",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Down,
        "4.650033563667687747641655e-28",
        "0x2.4d763776aaa2b05ba958E-23#80",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Up,
        "4.650033563667687747641662e-28",
        "0x2.4d763776aaa2b05ba95cE-23#80",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        Nearest,
        "4.650033563667687747641662e-28",
        "0x2.4d763776aaa2b05ba95cE-23#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Floor,
        "-4.650033563667687747641662e-28",
        "-0x2.4d763776aaa2b05ba95cE-23#80",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Ceiling,
        "-4.650033563667687747641655e-28",
        "-0x2.4d763776aaa2b05ba958E-23#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Down,
        "-4.650033563667687747641655e-28",
        "-0x2.4d763776aaa2b05ba958E-23#80",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Up,
        "-4.650033563667687747641662e-28",
        "-0x2.4d763776aaa2b05ba95cE-23#80",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        Nearest,
        "-4.650033563667687747641662e-28",
        "-0x2.4d763776aaa2b05ba95cE-23#80",
        Less,
    );
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_round_fail() {
    // 10^1.5 - 1 is irrational, so it cannot be represented exactly.
    parse_hex_string("0x1.8#10").power_of_10_x_minus_1_round(Exact);
}

#[test]
fn test_power_of_10_x_minus_1_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (power_of_10_x_minus_1, o) = x.clone().power_of_10_x_minus_1_prec_round(prec, rm);
        assert!(power_of_10_x_minus_1.is_valid());

        assert_eq!(power_of_10_x_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&power_of_10_x_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (power_of_10_x_minus_1_alt, o_alt) = x.power_of_10_x_minus_1_prec_round_ref(prec, rm);
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut power_of_10_x_minus_1_alt = x.clone();
        let o_alt = power_of_10_x_minus_1_alt.power_of_10_x_minus_1_prec_round_assign(prec, rm);
        assert!(power_of_10_x_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&power_of_10_x_minus_1),
            ComparableFloatRef(&power_of_10_x_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_power_of_10_x_minus_1, rug_o) =
                rug_power_of_10_x_minus_1_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1)),
                ComparableFloatRef(&power_of_10_x_minus_1),
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
    test("1.0", "0x1.0#1", 1, Floor, "8.0", "0x8.0#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "2.0e1", "0x1.0E+1#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "8.0", "0x8.0#1", Less);
    test("1.0", "0x1.0#1", 10, Floor, "9.0", "0x9.00#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "9.0", "0x9.00#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "9.0", "0x9.00#10", Equal);
    test("-1.0", "-0x1.0#1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", 10, Floor, "-0.9", "-0x0.e68#10", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-0.899",
        "-0x0.e64#10",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 10, Nearest, "-0.9", "-0x0.e68#10", Less);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Floor,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Floor,
        "9.0",
        "0x9.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "9.0",
        "0x9.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "9.0",
        "0x9.00#10",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Floor,
        "5.0e2",
        "0x2.0E+2#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Ceiling,
        "1.0e3",
        "0x4.0E+2#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "5.0e2",
        "0x2.0E+2#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "521.0",
        "0x209.0#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "522.0",
        "0x20a.0#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "522.0",
        "0x20a.0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "1.0e3",
        "0x4.0E+2#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "2.0e3",
        "0x8.0E+2#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "1.0e3",
        "0x4.0E+2#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "1384.0",
        "0x568.0#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "1386.0",
        "0x56a.0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "1384.0",
        "0x568.0#10",
        Less,
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
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "7.0e122",
        "0x1.0E+102#1",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "1.0e123",
        "0x2.0E+102#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "1.0e123",
        "0x2.0E+102#1",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "9.99e122",
        "0x1.830E+102#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "1.001e123",
        "0x1.838E+102#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "1.001e123",
        "0x1.838E+102#10",
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
    test("0.5", "0x0.8#1", 1, Floor, "2.0", "0x2.0#1", Less);
    test("0.5", "0x0.8#1", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("0.5", "0x0.8#1", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("0.5", "0x0.8#1", 10, Floor, "2.16", "0x2.29#10", Less);
    test("0.5", "0x0.8#1", 10, Ceiling, "2.164", "0x2.2a#10", Greater);
    test("0.5", "0x0.8#1", 10, Nearest, "2.164", "0x2.2a#10", Greater);
    test("-0.5", "-0x0.8#1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-0.5", "-0x0.8#1", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-0.5", "-0x0.8#1", 1, Nearest, "-0.5", "-0x0.8#1", Greater);
    test("-0.5", "-0x0.8#1", 10, Floor, "-0.685", "-0x0.af4#10", Less);
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Ceiling,
        "-0.684",
        "-0x0.af0#10",
        Greater,
    );
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Nearest,
        "-0.684",
        "-0x0.af0#10",
        Greater,
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
        "4.0e-28",
        "0x2.0E-23#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        Ceiling,
        "8.0e-28",
        "0x4.0E-23#1",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        1,
        Nearest,
        "4.0e-28",
        "0x2.0E-23#1",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Floor,
        "4.646e-28",
        "0x2.4dE-23#10",
        Less,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Ceiling,
        "4.654e-28",
        "0x2.4eE-23#10",
        Greater,
    );
    test(
        "2.019483917365790221854025e-28",
        "0x1.00000000000000000000E-23#80",
        10,
        Nearest,
        "4.646e-28",
        "0x2.4dE-23#10",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        Floor,
        "-8.0e-28",
        "-0x4.0E-23#1",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        Ceiling,
        "-4.0e-28",
        "-0x2.0E-23#1",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        1,
        Nearest,
        "-4.0e-28",
        "-0x2.0E-23#1",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Floor,
        "-4.654e-28",
        "-0x2.4eE-23#10",
        Less,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Ceiling,
        "-4.646e-28",
        "-0x2.4dE-23#10",
        Greater,
    );
    test(
        "-2.019483917365790221854025e-28",
        "-0x1.00000000000000000000E-23#80",
        10,
        Nearest,
        "-4.646e-28",
        "-0x2.4dE-23#10",
        Greater,
    );
    test(
        "0.2",
        "0x0.4#2",
        10,
        Nearest,
        "0.778",
        "0x0.c74#10",
        Greater,
    );
    test(
        "-0.00390625",
        "-0x0.010000000000000000#64",
        1,
        Ceiling,
        "-0.008",
        "-0x0.02#1",
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
        "-1.0",
        "-0x1.00000000000000000000000000000000000000000000000000#200",
        Less,
    );
    // A nonnegative integer x makes 10^x - 1 an exact integer, so (unlike expm1) `Exact` succeeds.
    test("3.0", "0x3.0#2", 10, Exact, "999.0", "0x3e7.0#10", Equal);
    test(
        "6.0",
        "0x6.0#3",
        20,
        Exact,
        "999999.0",
        "0xf423f.0#20",
        Equal,
    );
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_prec_round_fail_1() {
    Float::NAN.power_of_10_x_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_prec_round_fail_2() {
    // 10^1.5 - 1 is irrational, so it cannot be represented exactly.
    parse_hex_string("0x1.8#10").power_of_10_x_minus_1_prec_round(10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_10_x_minus_1_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (power_of_10_x_minus_1, o) = x.clone().power_of_10_x_minus_1_prec_round(prec, rm);
    assert!(power_of_10_x_minus_1.is_valid());

    let (power_of_10_x_minus_1_alt, o_alt) = x.power_of_10_x_minus_1_prec_round_ref(prec, rm);
    assert!(power_of_10_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_10_x_minus_1_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_power_of_10_x_minus_1, rug_o) =
            rug_power_of_10_x_minus_1_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1)),
            ComparableFloatRef(&power_of_10_x_minus_1),
        );
        assert_eq!(rug_o, o);
    }

    if power_of_10_x_minus_1.is_normal() {
        assert_eq!(power_of_10_x_minus_1.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(power_of_10_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(power_of_10_x_minus_1 < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.power_of_10_x_minus_1_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(power_of_10_x_minus_1.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.power_of_10_x_minus_1_prec_round_ref(prec, Exact));
    }
}

#[test]
fn power_of_10_x_minus_1_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        power_of_10_x_minus_1_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            power_of_10_x_minus_1_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.power_of_10_x_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.power_of_10_x_minus_1_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (s, o) = Float::NEGATIVE_INFINITY.power_of_10_x_minus_1_prec_round(prec, rm);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.power_of_10_x_minus_1_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.power_of_10_x_minus_1_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_10_x_minus_1_prec_properties_helper(x: Float, prec: u64) {
    let (power_of_10_x_minus_1, o) = x.clone().power_of_10_x_minus_1_prec(prec);
    assert!(power_of_10_x_minus_1.is_valid());

    let (power_of_10_x_minus_1_alt, o_alt) = x.power_of_10_x_minus_1_prec_ref(prec);
    assert!(power_of_10_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_10_x_minus_1_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let (rug_power_of_10_x_minus_1, rug_o) =
        rug_power_of_10_x_minus_1_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1)),
        ComparableFloatRef(&power_of_10_x_minus_1),
    );
    assert_eq!(rug_o, o);

    let (power_of_10_x_minus_1_alt, o_alt) = x.power_of_10_x_minus_1_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if power_of_10_x_minus_1.is_normal() {
        assert_eq!(power_of_10_x_minus_1.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(power_of_10_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(power_of_10_x_minus_1 < 0u32);
        }
    }
}

#[test]
fn power_of_10_x_minus_1_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        power_of_10_x_minus_1_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        power_of_10_x_minus_1_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        power_of_10_x_minus_1_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.power_of_10_x_minus_1_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.power_of_10_x_minus_1_prec(prec);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.power_of_10_x_minus_1_prec(prec);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.power_of_10_x_minus_1_prec(prec),
            (Float::INFINITY, Equal)
        );
        let (s, o) = Float::NEGATIVE_INFINITY.power_of_10_x_minus_1_prec(prec);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_10_x_minus_1_round_properties_helper(x: Float, rm: RoundingMode) {
    let (power_of_10_x_minus_1, o) = x.clone().power_of_10_x_minus_1_round(rm);
    assert!(power_of_10_x_minus_1.is_valid());

    let (power_of_10_x_minus_1_alt, o_alt) = x.power_of_10_x_minus_1_round_ref(rm);
    assert!(power_of_10_x_minus_1_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_10_x_minus_1_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    assert_eq!(o_alt, o);

    let (power_of_10_x_minus_1_alt, o_alt) =
        x.power_of_10_x_minus_1_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    assert_eq!(o_alt, o);

    if x.is_finite() {
        // 10^x > 0, so 10^x - 1 > -1; rounding down can reach but not pass -1.
        assert!(power_of_10_x_minus_1 >= Float::NEGATIVE_ONE);
    }

    if power_of_10_x_minus_1.is_normal() {
        // For finite x the result has x's precision; for x = -inf the result is -1 (normal, but x
        // has no precision), so the precision check only applies when x is finite.
        if let Some(p) = x.get_prec() {
            assert_eq!(power_of_10_x_minus_1.get_prec(), Some(p));
        }
        if x > 0u32 && o > Less {
            assert!(power_of_10_x_minus_1 > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(power_of_10_x_minus_1 < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_power_of_10_x_minus_1, rug_o) =
            rug_power_of_10_x_minus_1_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1)),
            ComparableFloatRef(&power_of_10_x_minus_1),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.power_of_10_x_minus_1_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(power_of_10_x_minus_1.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.power_of_10_x_minus_1_round_ref(Exact));
    }
}

#[test]
fn power_of_10_x_minus_1_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        power_of_10_x_minus_1_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (s, o) = Float::NAN.power_of_10_x_minus_1_round(rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.power_of_10_x_minus_1_round(rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.power_of_10_x_minus_1_round(rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.power_of_10_x_minus_1_round(rm),
            (Float::INFINITY, Equal)
        );
        let (s, o) = Float::NEGATIVE_INFINITY.power_of_10_x_minus_1_round(rm);
        assert_eq!(s, Float::NEGATIVE_ONE);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_10_x_minus_1_properties_helper(x: Float) {
    let power_of_10_x_minus_1 = x.clone().power_of_10_x_minus_1();
    assert!(power_of_10_x_minus_1.is_valid());

    let power_of_10_x_minus_1_alt = (&x).power_of_10_x_minus_1();
    assert!(power_of_10_x_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );

    let mut x_alt = x.clone();
    x_alt.power_of_10_x_minus_1_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );

    let power_of_10_x_minus_1_alt = x
        .power_of_10_x_minus_1_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );
    let power_of_10_x_minus_1_alt = x.power_of_10_x_minus_1_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );

    let power_of_10_x_minus_1_alt = x.power_of_10_x_minus_1_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&power_of_10_x_minus_1_alt),
        ComparableFloatRef(&power_of_10_x_minus_1)
    );

    if x.is_finite() {
        // 10^x > 0, so 10^x - 1 > -1; rounding down can reach but not pass -1.
        assert!(power_of_10_x_minus_1 >= Float::NEGATIVE_ONE);
    }

    let rug_power_of_10_x_minus_1 = rug_power_of_10_x_minus_1(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_power_of_10_x_minus_1)),
        ComparableFloatRef(&power_of_10_x_minus_1),
    );
}

#[test]
fn power_of_10_x_minus_1_properties() {
    float_gen().test_properties(|x| {
        power_of_10_x_minus_1_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        power_of_10_x_minus_1_properties_helper(x);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_power_of_10_x_minus_1() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_power_of_10_x_minus_1(x)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, -1.0);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 9.0);
    test::<f32>(3.0, 999.0);
    test::<f32>(-2.0, -0.99);
    // Computing `x.exp2() - 1.0` with the primitive float functions gives 0.41421354;
    // `primitive_float_power_of_10_x_minus_1` is correctly rounded.
    test::<f32>(0.5, 2.1622777);
    test::<f32>(-0.5, -0.6837722);
    test::<f32>(100.0, f32::INFINITY);
    test::<f32>(200.0, f32::INFINITY);
    test::<f32>(-130.0, -1.0);
    // For small x the subtraction in `x.exp2() - 1.0` loses everything (it gives 0.0 for both of
    // these); the correctly-rounded result is close to x ln(10).
    test::<f32>(1.0e-8, 2.3025851e-8);
    test::<f32>(-1.0e-8, -2.302585e-8);
    // A subnormal result.
    test::<f32>(1.0e-38, 2.3025849e-38);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, -1.0);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 9.0);
    test::<f64>(3.0, 999.0);
    test::<f64>(-2.0, -0.99);
    // Computing `x.exp2() - 1.0` with the primitive float functions gives 0.41421356237309515;
    // `primitive_float_power_of_10_x_minus_1` is correctly rounded.
    test::<f64>(0.5, 2.1622776601683795);
    test::<f64>(-0.5, -0.6837722339831621);
    test::<f64>(1000.0, f64::INFINITY);
    test::<f64>(1100.0, f64::INFINITY);
    test::<f64>(-1080.0, -1.0);
    // For small x the subtraction in `x.exp2() - 1.0` loses everything (it gives 0.0 and
    // -1.1102230246251565e-16 respectively); the correctly-rounded result is close to x ln(10).
    test::<f64>(1.0e-16, 2.302585092994046e-16);
    test::<f64>(-1.0e-16, -2.3025850929940453e-16);
    // A subnormal result.
    test::<f64>(1.0e-308, 2.3025850929940456e-308);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_power_of_10_x_minus_1_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_power_of_10_x_minus_1(x);
    });
}

#[test]
fn primitive_float_power_of_10_x_minus_1_properties() {
    apply_fn_to_primitive_floats!(primitive_float_power_of_10_x_minus_1_properties_helper);
}

#[test]
fn test_power_of_10_x_minus_1_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::power_of_10_x_minus_1_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 53, "0.0", "0x0.0", Equal);
    // - integer x: the result 10^x - 1 is exact whenever prec suffices (nonnegative x) or rounds
    test("1", 1, "8.0", "0x8.0#1", Less);
    test("1", 10, "9.0", "0x9.00#10", Equal);
    test("1", 53, "9.0", "0x9.0000000000000#53", Equal);
    test("3", 1, "1.0e3", "0x4.0E+2#1", Greater);
    test("3", 10, "999.0", "0x3e7.0#10", Equal);
    test("3", 53, "999.0", "0x3e7.00000000000#53", Equal);
    test("-1", 1, "-1.0", "-0x1.0#1", Less);
    test("-1", 10, "-0.9", "-0x0.e68#10", Less);
    test("-1", 53, "-0.9", "-0x0.e6666666666668#53", Less);
    test("-2", 1, "-1.0", "-0x1.0#1", Less);
    test("-2", 10, "-0.99", "-0x0.fd8#10", Less);
    test("-2", 53, "-0.99", "-0x0.fd70a3d70a3d70#53", Greater);
    // - non-integer x: the result is irrational, computed by the bracketing squeeze
    test("1/2", 1, "2.0", "0x2.0#1", Less);
    test("1/2", 10, "2.164", "0x2.2a#10", Greater);
    test(
        "1/2",
        53,
        "2.1622776601683795",
        "0x2.298b075b4b6a6#53",
        Greater,
    );
    test("-1/2", 1, "-0.5", "-0x0.8#1", Greater);
    test("-1/2", 10, "-0.684", "-0x0.af0#10", Greater);
    test(
        "-1/2",
        53,
        "-0.6837722339831621",
        "-0x0.af0bb276dedbc8#53",
        Less,
    );
    test("1/3", 1, "1.0", "0x1.0#1", Less);
    test("1/3", 10, "1.154", "0x1.278#10", Less);
    test(
        "1/3",
        53,
        "1.1544346900318838",
        "0x1.278908270e09e#53",
        Greater,
    );
    test("-1/3", 1, "-0.5", "-0x0.8#1", Greater);
    test("-1/3", 10, "-0.536", "-0x0.894#10", Less);
    test(
        "-1/3",
        53,
        "-0.5358411166387221",
        "-0x0.892ce227d0c038#53",
        Less,
    );
    test("22/7", 1, "1.0e3", "0x4.0E+2#1", Less);
    test("22/7", 10, "1388.0", "0x56c.0#10", Less);
    test(
        "22/7",
        53,
        "1388.4954943731377",
        "0x56c.7ed8b81ffa8#53",
        Greater,
    );
    test("-22/7", 1, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 10, "-0.999", "-0x0.ffc#10", Greater);
    test(
        "-22/7",
        53,
        "-0.9992803143269988",
        "-0x0.ffd0d5ad923100#53",
        Greater,
    );
    test("100", 1, "9.0e99", "0x1.0E+83#1", Less);
    test("100", 10, "9.996e99", "0x1.248E+83#10", Less);
    test("100", 53, "1.0e100", "0x1.249ad2594c37dE+83#53", Greater);
    test("-100", 1, "-1.0", "-0x1.0#1", Less);
    test("-100", 10, "-1.0", "-0x1.000#10", Less);
    test("-100", 53, "-1.0", "-0x1.0000000000000#53", Less);
    // - the exact 2^100 - 1 needs exactly 100 bits
    test(
        "100",
        100,
        "1.0e100",
        "0x1.249ad2594c37ceb0b2784c4ceE+83#100",
        Less,
    );
    // - the exact -1 + 2^-100 needs 101 bits: at prec 99 the deep-negative rounding shortcut
    //   resolves it from -1, and at prec 100 and 101 the exact rational is materialized (prec 100
    //   sits exactly on the midpoint between -1 and its toward-zero neighbor)
    test(
        "-100",
        99,
        "-1.0",
        "-0x1.0000000000000000000000000#99",
        Less,
    );
    test(
        "-100",
        100,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "-100",
        101,
        "-1.0",
        "-0x1.0000000000000000000000000#101",
        Less,
    );

    let test_big = |x: Rational, prec, out: &str, out_hex: &str, out_o| {
        let (f, o) = Float::power_of_10_x_minus_1_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    // - integer x > MAX_EXPONENT: 2^x - 1 overflows
    test_big(
        Rational::power_of_2(31i64),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(31i64),
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - integer x = MAX_EXPONENT: 2^x - 1 is representable, but needs at least MAX_EXPONENT bits;
    //   at smaller precisions it rounds exactly like an overflow
    test_big(
        Rational::from(Float::MAX_EXPONENT),
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - integer x < -MAX_EXPONENT: 2^x - 1 rounds from -1
    test_big(-Rational::power_of_2(31i64), 1, "-1.0", "-0x1.0#1", Less);
    test_big(
        -Rational::power_of_2(31i64),
        10,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    // - integer x = MIN_EXPONENT - 1: 2^x is the smallest positive Float, and the exact difference
    //   2^x - 1 is rounded directly
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    // - integer x = MIN_EXPONENT - 2: 2^x is below the smallest positive Float, and the result is
    //   resolved by rounding from -1
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    // - non-integer |x| < 2^MIN_EXPONENT: 2^x - 1 ~ x ln(2) is smaller than the smallest positive
    //   Float, resolved by the ln(2)-bracketing helper; for |x| in the smallest binade the result
    //   magnitude is in (min_positive / 2, min_positive), so `Nearest` rounds away from zero, while
    //   for smaller |x| it rounds to zero
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        "too_small",
        "0x2.0E-268435456#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "too_small",
        "0x2.4dE-268435456#10",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        "-too_small",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "-too_small",
        "-0x2.4dE-268435456#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 10),
        1,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 10),
        1,
        "-0.0",
        "-0x0.0",
        Greater,
    );
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_fail() {
    Float::power_of_10_x_minus_1_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_ref_fail() {
    Float::power_of_10_x_minus_1_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_power_of_10_x_minus_1_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, rm);
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
    // - integer x: the result is exact whenever prec suffices, for every rounding mode (including
    //   `Exact`)
    test("1", 1, Floor, "8.0", "0x8.0#1", Less);
    test("1", 1, Ceiling, "2.0e1", "0x1.0E+1#1", Greater);
    test("1", 1, Nearest, "8.0", "0x8.0#1", Less);
    test("1", 10, Floor, "9.0", "0x9.00#10", Equal);
    test("1", 10, Ceiling, "9.0", "0x9.00#10", Equal);
    test("1", 10, Nearest, "9.0", "0x9.00#10", Equal);
    test("1", 53, Floor, "9.0", "0x9.0000000000000#53", Equal);
    test("1", 53, Ceiling, "9.0", "0x9.0000000000000#53", Equal);
    test("1", 53, Nearest, "9.0", "0x9.0000000000000#53", Equal);
    test("3", 1, Floor, "5.0e2", "0x2.0E+2#1", Less);
    test("3", 1, Ceiling, "1.0e3", "0x4.0E+2#1", Greater);
    test("3", 1, Nearest, "1.0e3", "0x4.0E+2#1", Greater);
    test("3", 10, Floor, "999.0", "0x3e7.0#10", Equal);
    test("3", 10, Ceiling, "999.0", "0x3e7.0#10", Equal);
    test("3", 10, Nearest, "999.0", "0x3e7.0#10", Equal);
    test("3", 10, Exact, "999.0", "0x3e7.0#10", Equal);
    test("3", 53, Floor, "999.0", "0x3e7.00000000000#53", Equal);
    test("3", 53, Ceiling, "999.0", "0x3e7.00000000000#53", Equal);
    test("3", 53, Nearest, "999.0", "0x3e7.00000000000#53", Equal);
    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-1", 10, Floor, "-0.9", "-0x0.e68#10", Less);
    test("-1", 10, Ceiling, "-0.899", "-0x0.e64#10", Greater);
    test("-1", 10, Nearest, "-0.9", "-0x0.e68#10", Less);
    test("-1", 53, Floor, "-0.9", "-0x0.e6666666666668#53", Less);
    test(
        "-1",
        53,
        Ceiling,
        "-0.8999999999999999",
        "-0x0.e6666666666660#53",
        Greater,
    );
    test("-1", 53, Nearest, "-0.9", "-0x0.e6666666666668#53", Less);
    test("-2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-2", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-2", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-2", 10, Floor, "-0.99", "-0x0.fd8#10", Less);
    test("-2", 10, Ceiling, "-0.989", "-0x0.fd4#10", Greater);
    test("-2", 10, Nearest, "-0.99", "-0x0.fd8#10", Less);
    test(
        "-2",
        53,
        Floor,
        "-0.9900000000000001",
        "-0x0.fd70a3d70a3d78#53",
        Less,
    );
    test(
        "-2",
        53,
        Ceiling,
        "-0.99",
        "-0x0.fd70a3d70a3d70#53",
        Greater,
    );
    test(
        "-2",
        53,
        Nearest,
        "-0.99",
        "-0x0.fd70a3d70a3d70#53",
        Greater,
    );
    // - non-integer x: the result is irrational
    test("1/2", 1, Floor, "2.0", "0x2.0#1", Less);
    test("1/2", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("1/2", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("1/2", 10, Floor, "2.16", "0x2.29#10", Less);
    test("1/2", 10, Ceiling, "2.164", "0x2.2a#10", Greater);
    test("1/2", 10, Nearest, "2.164", "0x2.2a#10", Greater);
    test(
        "1/2",
        53,
        Floor,
        "2.1622776601683791",
        "0x2.298b075b4b6a4#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "2.1622776601683795",
        "0x2.298b075b4b6a6#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "2.1622776601683795",
        "0x2.298b075b4b6a6#53",
        Greater,
    );
    test("-1/2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1/2", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1/2", 1, Nearest, "-0.5", "-0x0.8#1", Greater);
    test("-1/2", 10, Floor, "-0.685", "-0x0.af4#10", Less);
    test("-1/2", 10, Ceiling, "-0.684", "-0x0.af0#10", Greater);
    test("-1/2", 10, Nearest, "-0.684", "-0x0.af0#10", Greater);
    test(
        "-1/2",
        53,
        Floor,
        "-0.6837722339831621",
        "-0x0.af0bb276dedbc8#53",
        Less,
    );
    test(
        "-1/2",
        53,
        Ceiling,
        "-0.683772233983162",
        "-0x0.af0bb276dedbc0#53",
        Greater,
    );
    test(
        "-1/2",
        53,
        Nearest,
        "-0.6837722339831621",
        "-0x0.af0bb276dedbc8#53",
        Less,
    );
    test("1/3", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/3", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/3", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/3", 10, Floor, "1.154", "0x1.278#10", Less);
    test("1/3", 10, Ceiling, "1.156", "0x1.280#10", Greater);
    test("1/3", 10, Nearest, "1.154", "0x1.278#10", Less);
    test(
        "1/3",
        53,
        Floor,
        "1.1544346900318836",
        "0x1.278908270e09d#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "1.1544346900318838",
        "0x1.278908270e09e#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "1.1544346900318838",
        "0x1.278908270e09e#53",
        Greater,
    );
    test("-1/3", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1/3", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-1/3", 1, Nearest, "-0.5", "-0x0.8#1", Greater);
    test("-1/3", 10, Floor, "-0.536", "-0x0.894#10", Less);
    test("-1/3", 10, Ceiling, "-0.535", "-0x0.890#10", Greater);
    test("-1/3", 10, Nearest, "-0.536", "-0x0.894#10", Less);
    test(
        "-1/3",
        53,
        Floor,
        "-0.5358411166387221",
        "-0x0.892ce227d0c038#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-0.535841116638722",
        "-0x0.892ce227d0c030#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-0.5358411166387221",
        "-0x0.892ce227d0c038#53",
        Less,
    );
    test("22/7", 1, Floor, "1.0e3", "0x4.0E+2#1", Less);
    test("22/7", 1, Ceiling, "2.0e3", "0x8.0E+2#1", Greater);
    test("22/7", 1, Nearest, "1.0e3", "0x4.0E+2#1", Less);
    test("22/7", 10, Floor, "1388.0", "0x56c.0#10", Less);
    test("22/7", 10, Ceiling, "1390.0", "0x56e.0#10", Greater);
    test("22/7", 10, Nearest, "1388.0", "0x56c.0#10", Less);
    test(
        "22/7",
        53,
        Floor,
        "1388.4954943731375",
        "0x56c.7ed8b81ffa4#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "1388.4954943731377",
        "0x56c.7ed8b81ffa8#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "1388.4954943731377",
        "0x56c.7ed8b81ffa8#53",
        Greater,
    );
    test("-22/7", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-22/7", 10, Floor, "-1.0", "-0x1.000#10", Less);
    test("-22/7", 10, Ceiling, "-0.999", "-0x0.ffc#10", Greater);
    test("-22/7", 10, Nearest, "-0.999", "-0x0.ffc#10", Greater);
    test(
        "-22/7",
        53,
        Floor,
        "-0.9992803143269989",
        "-0x0.ffd0d5ad923108#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "-0.9992803143269988",
        "-0x0.ffd0d5ad923100#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "-0.9992803143269988",
        "-0x0.ffd0d5ad923100#53",
        Greater,
    );
    test("100", 1, Floor, "9.0e99", "0x1.0E+83#1", Less);
    test("100", 1, Ceiling, "2.0e100", "0x2.0E+83#1", Greater);
    test("100", 1, Nearest, "9.0e99", "0x1.0E+83#1", Less);
    test("100", 10, Floor, "9.996e99", "0x1.248E+83#10", Less);
    test("100", 10, Ceiling, "1.001e100", "0x1.250E+83#10", Greater);
    test("100", 10, Nearest, "9.996e99", "0x1.248E+83#10", Less);
    test(
        "100",
        53,
        Floor,
        "9.999999999999998e99",
        "0x1.249ad2594c37cE+83#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "1.0e100",
        "0x1.249ad2594c37dE+83#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "1.0e100",
        "0x1.249ad2594c37dE+83#53",
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
    // - the exact -1 + 2^-100 needs 101 bits

    let test_big = |x: Rational, prec, rm, out: &str, out_hex: &str, out_o| {
        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    // - integer x > MAX_EXPONENT: 2^x - 1 overflows. Directed-down rounding returns the largest
    //   finite Float; the other modes return +inf
    test_big(
        Rational::power_of_2(31i64),
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(31i64),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(31i64),
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(31i64),
        10,
        Floor,
        "too_big",
        "0x7.feE+268435455#10",
        Less,
    );
    // - integer x = MAX_EXPONENT: 2^x - 1 is representable but needs at least MAX_EXPONENT bits; at
    //   smaller precisions it rounds exactly like an overflow
    test_big(
        Rational::from(Float::MAX_EXPONENT),
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::from(Float::MAX_EXPONENT),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::from(Float::MAX_EXPONENT),
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - integer x < -MAX_EXPONENT: the result is resolved by rounding from -1
    test_big(
        -Rational::power_of_2(31i64),
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(31i64),
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(31i64),
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(31i64),
        10,
        Ceiling,
        "-0.999",
        "-0x0.ffc#10",
        Greater,
    );
    // - integer x = MIN_EXPONENT - 1: 2^x is the smallest positive Float, and the exact difference
    //   2^x - 1 is rounded directly
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    // - integer x = MIN_EXPONENT - 2: 2^x is below the smallest positive Float, and the result is
    //   resolved by rounding from -1
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test_big(
        Rational::from(i64::from(Float::MIN_EXPONENT) - 2),
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    // - non-integer x > 2^(MAX_EXPONENT - 1): 2^x - 1 overflows
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 3),
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 3),
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 3),
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    // - non-integer x < -2^(MAX_EXPONENT - 1): the result is resolved by rounding from -1
    test_big(
        -(Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 3)),
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test_big(
        -(Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 3)),
        1,
        Ceiling,
        "-0.5",
        "-0x0.8#1",
        Greater,
    );
    test_big(
        -(Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 3)),
        10,
        Nearest,
        "-1.0",
        "-0x1.000#10",
        Less,
    );
    // - non-integer |x| < 2^MIN_EXPONENT: 2^x - 1 ~ x ln(2) underflows, via the ln(2)-bracketing
    //   helper. For x > 0 the toward-zero modes give 0; for x < 0 they give -0
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Floor,
        "too_small",
        "0x2.0E-268435456#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Ceiling,
        "too_small",
        "0x4.0E-268435456#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Nearest,
        "too_small",
        "0x2.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Floor,
        "-too_small",
        "-0x4.0E-268435456#1",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Ceiling,
        "-too_small",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        1,
        Nearest,
        "-too_small",
        "-0x2.0E-268435456#1",
        Greater,
    );
    // - |x ln(2)| below half the smallest positive Float: `Nearest` rounds to zero
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 10),
        1,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        -Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 10),
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );

    // - huge-negative non-integer x with a precision so large that `float_round_near_x` cannot
    //   resolve the rounding (its error bound MAX_EXPONENT <= prec + 1), exercising the manual -1 /
    //   -1+ulp fallback. The result is a ~128 MB Float, so it is checked by value rather than by
    //   string.
    let prec = u64::exact_from(Float::MAX_EXPONENT);
    let x =
        -(Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 3));
    let neg_one = -Float::one_prec(prec);
    // -1 + ulp, i.e. the neighbor of -1 toward zero.
    let neg_one_plus_ulp = neg_one
        .clone()
        .add_prec_round(Float::power_of_2(-i64::exact_from(prec)), prec, Exact)
        .0;
    // Toward -inf or away from zero rounds to -1.
    for rm in [Floor, Up, Nearest] {
        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&neg_one));
        assert_eq!(o, Less);
    }
    // Toward zero (Ceiling for a negative value, or Down) rounds to -1 + ulp.
    for rm in [Ceiling, Down] {
        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(
            ComparableFloatRef(&f),
            ComparableFloatRef(&neg_one_plus_ulp)
        );
        assert_eq!(o, Greater);
    }
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_round_fail_1() {
    Float::power_of_10_x_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_round_fail_2() {
    Float::power_of_10_x_minus_1_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_round_fail_3() {
    // 10^3 - 1 = 999 is not representable with 1 bit
    Float::power_of_10_x_minus_1_rational_prec_round(Rational::from(3u32), 1, Exact);
}

#[test]
#[should_panic]
fn power_of_10_x_minus_1_rational_prec_round_ref_fail() {
    Float::power_of_10_x_minus_1_rational_prec_round_ref(
        &Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_10_x_minus_1_rational_prec_round_properties_helper(
    x: Rational,
    prec: u64,
    rm: RoundingMode,
) {
    let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(f.is_valid());

    let (f_alt, o_alt) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, rm);
    assert!(f_alt.is_valid());
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    // 10^x - 1 has the same sign as x, and is never NaN.
    if x > 0u32 {
        assert!(f >= 0u32);
    } else if x < 0u32 {
        assert!(f <= 0u32);
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_f, rug_o) = rug_power_of_10_x_minus_1_rational_prec_round(&x, prec, rrm);
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
            let (s, oo) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(f.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::power_of_10_x_minus_1_rational_prec_round_ref(
            &x, prec, Exact
        ));
    }
}

#[test]
fn power_of_10_x_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        power_of_10_x_minus_1_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (f, o) = Float::power_of_10_x_minus_1_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(f), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_10_x_minus_1_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (f, o) = Float::power_of_10_x_minus_1_rational_prec(x.clone(), prec);
    assert!(f.is_valid());

    let (f_alt, o_alt) = Float::power_of_10_x_minus_1_rational_prec_ref(&x, prec);
    assert!(f_alt.is_valid());
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    let (f_alt, o_alt) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    if x > 0u32 {
        assert!(f >= 0u32);
    } else if x < 0u32 {
        assert!(f <= 0u32);
    }

    let (rug_f, rug_o) = rug_power_of_10_x_minus_1_rational_prec(&x, prec);
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
fn power_of_10_x_minus_1_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        power_of_10_x_minus_1_rational_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (f, o) = Float::power_of_10_x_minus_1_rational_prec(Rational::ZERO, prec);
        assert_eq!(ComparableFloat(f), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_power_of_10_x_minus_1_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_power_of_10_x_minus_1_rational(&u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f32>("1", 9.0);
    test::<f32>("3", 999.0);
    test::<f32>("1/2", 2.1622777);
    test::<f32>("1/3", 1.1544347);
    test::<f32>("22/7", 1388.4955);
    test::<f32>("200", f32::INFINITY);
    test::<f32>("1/1000000", 0.0000023025877);
    test::<f32>("-1", -0.9);
    test::<f32>("-3", -0.999);
    test::<f32>("-1/2", -0.6837722);
    test::<f32>("-1/3", -0.5358411);
    test::<f32>("-22/7", -0.99928033);
    test::<f32>("-200", -1.0);
    test::<f32>("-1/1000000", -0.0000023025825);

    test::<f64>("0", 0.0);
    test::<f64>("1", 9.0);
    test::<f64>("3", 999.0);
    test::<f64>("1/2", 2.1622776601683795);
    test::<f64>("1/3", 1.1544346900318838);
    test::<f64>("22/7", 1388.4954943731377);
    test::<f64>("1100", f64::INFINITY);
    test::<f64>("1/1000000", 2.3025877439451358e-6);
    test::<f64>("-1", -0.9);
    test::<f64>("-3", -0.999);
    test::<f64>("-1/2", -0.6837722339831621);
    test::<f64>("-1/3", -0.5358411166387221);
    test::<f64>("-22/7", -0.9992803143269988);
    test::<f64>("-1100", -1.0);
    test::<f64>("-1/1000000", -2.302582442047025e-6);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_power_of_10_x_minus_1_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let y = primitive_float_power_of_10_x_minus_1_rational::<T>(&x);
        // 10^x - 1 has the same sign as x, and is never NaN.
        assert!(!y.is_nan());
        if x > 0u32 {
            assert!(y >= T::ZERO);
        } else if x < 0u32 {
            assert!(y <= T::ZERO);
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // 10^x - 1 of a finite, nonzero primitive float, taken through the `Rational` path, matches
        // the direct primitive-float function. Zero is excluded: a `Rational` has no signed zero,
        // so the `Rational` path returns +0 for both signs whereas the direct path preserves it
        // (10^-0.0 - 1 = -0.0).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_power_of_10_x_minus_1_rational::<T>(
                    &Rational::exact_from(x)
                )),
                NiceFloat(primitive_float_power_of_10_x_minus_1(x))
            );
        }
    });
}

#[test]
fn primitive_float_power_of_10_x_minus_1_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_power_of_10_x_minus_1_rational_properties_helper);
}

// Regression test: for x = MAX_EXPONENT the intermediate 2^x overflows at any working precision,
// unlike base 2, 10^MAX_EXPONENT always overflows, so this exercises the deep-negative regime
// instead: at prec = MAX_EXPONENT, x = -340000000 has 10^x below the smallest positive Float (x *
// log2(10) < MIN_EXPONENT) without triggering the huge-negative shortcut (|x| < 2 + (prec - 1) /
// 3), so the Float path materializes -1 + 10^x over Rationals while the Rational integer path
// rounds from -1 by a different mechanism; the two must agree. (~128 MB results.)
#[test]
fn test_power_of_10_x_minus_1_deep_negative_regression() {
    let prec = u64::exact_from(Float::MAX_EXPONENT);
    let x = Float::from(-340000000i32);
    let q = Rational::from(-340000000i32);
    for rm in exhaustive_rounding_modes() {
        if rm == Exact {
            continue; // 10^x - 1 is inexact here (10^x != 0)
        }
        let (pf, of) = x.power_of_10_x_minus_1_prec_round_ref(prec, rm);
        assert!(pf.is_valid());
        let (pr, or) = Float::power_of_10_x_minus_1_rational_prec_round_ref(&q, prec, rm);
        assert_eq!(ComparableFloat(pf), ComparableFloat(pr), "rm = {rm:?}");
        assert_eq!(of, or);
    }
}
