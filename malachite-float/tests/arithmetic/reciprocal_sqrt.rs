// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::{f32, f64};
use malachite_base::num::arithmetic::traits::{
    PowerOf2, Reciprocal, ReciprocalSqrt, ReciprocalSqrtAssign, Square,
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
use malachite_float::arithmetic::reciprocal_sqrt::{
    primitive_float_reciprocal_sqrt, primitive_float_reciprocal_sqrt_rational,
};
use malachite_float::test_util::arithmetic::reciprocal_sqrt::{
    reciprocal_sqrt_rational_prec_round_generic, rug_reciprocal_sqrt, rug_reciprocal_sqrt_prec,
    rug_reciprocal_sqrt_prec_round, rug_reciprocal_sqrt_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_30,
    float_rounding_mode_pair_gen_var_31, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_15,
    float_unsigned_rounding_mode_triple_gen_var_16,
    rational_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_gen_var_1, rational_pair_gen_var_8, rational_unsigned_pair_gen_var_7,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_reciprocal_sqrt() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let reciprocal_sqrt = x.clone().reciprocal_sqrt();
        assert!(reciprocal_sqrt.is_valid());

        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);

        let reciprocal_sqrt_alt = (&x).reciprocal_sqrt();
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );

        let mut reciprocal_sqrt_alt = x.clone();
        reciprocal_sqrt_alt.reciprocal_sqrt_assign();
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt(&rug::Float::exact_from(
                &x
            )))),
            ComparableFloatRef(&reciprocal_sqrt)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "0.0", "0x0.0");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "Infinity", "Infinity");
    test("-0.0", "-0x0.0", "Infinity", "Infinity");
    // - working_prec < limb_to_bit_count(rn) in limbs_reciprocal_sqrt
    // - in limbs_reciprocal_sqrt
    // - an <= n in limbs_reciprocal_sqrt
    // - p > 11 in limbs_reciprocal_sqrt
    // - ahn <= an in limbs_reciprocal_sqrt
    // - p == 11 in limbs_reciprocal_sqrt
    // - h << 1 <= Limb::WIDTH in limbs_reciprocal_sqrt
    // - rn == 1 in limbs_reciprocal_sqrt
    // - neg == 0 first time in limbs_reciprocal_sqrt
    // - tn == 1 in limbs_reciprocal_sqrt
    // - !a_s in limbs_reciprocal_sqrt
    // - pl != 0 in limbs_reciprocal_sqrt
    // - neg == 0 second time in limbs_reciprocal_sqrt
    // - neg == 0 && ln == 0 in limbs_reciprocal_sqrt
    // - cy != 0 in limbs_reciprocal_sqrt
    // - h << 1 > Limb::WIDTH && xn == 1 in limbs_reciprocal_sqrt
    // - rn != 1 in limbs_reciprocal_sqrt
    // - tn != 1 in limbs_reciprocal_sqrt
    // - pl == 0 in limbs_reciprocal_sqrt
    // - can't round in limbs_reciprocal_sqrt
    // - s == 0 && x.is_power_of_2() in limbs_reciprocal_sqrt
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "NaN", "NaN");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1.0",
        "0x1.0000000000000000000000000#100",
    );
    test("-1.0", "-0x1.0000000000000000000000000#100", "NaN", "NaN");

    test("123.0", "0x7b.0#7", "0.09", "0x0.170#7");
    test("-123.0", "-0x7b.0#7", "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "0.5641895835477563",
        "0x0.906eba8214db68#53",
    );
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "NaN", "NaN");
}

#[test]
fn test_reciprocal_sqrt_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal_sqrt, o) = x.clone().reciprocal_sqrt_prec(prec);
        assert!(reciprocal_sqrt.is_valid());

        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_sqrt_alt, o_alt) = x.reciprocal_sqrt_prec_ref(prec);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_sqrt_alt = x.clone();
        let o_alt = reciprocal_sqrt_alt.reciprocal_sqrt_prec_assign(prec);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );
        assert_eq!(o_alt, o_out);

        let (rug_reciprocal_sqrt, rug_o) =
            rug_reciprocal_sqrt_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
            ComparableFloatRef(&reciprocal_sqrt),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, "1.0", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);
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
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);

    test("123.0", "0x7b.0#7", 1, "0.06", "0x0.1#1", Less);
    test("123.0", "0x7b.0#7", 10, "0.0902", "0x0.1718#10", Greater);
    test("-123.0", "-0x7b.0#7", 1, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.564",
        "0x0.908#10",
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
    // - a_s in limbs_reciprocal_sqrt
    // - cy == 0 in limbs_reciprocal_sqrt
    // - neg != 0 first time in limbs_reciprocal_sqrt
    // - neg != 0 second time in limbs_reciprocal_sqrt
    // - neg != 0 && ln == 0 in limbs_reciprocal_sqrt
    // - can round in limbs_reciprocal_sqrt
    test("2.0", "0x2.0#1", 1, "0.5", "0x0.8#1", Less);
    // - neg == 0 && ln != 0 in limbs_reciprocal_sqrt
    // - h << 1 > Limb::WIDTH && xn != 1 in limbs_reciprocal_sqrt
    // - rn < xn << 1 in limbs_reciprocal_sqrt
    test(
        "21729783659306408649613509.686",
        "0x11f975eebbcb21a32ee0c5.af8#95",
        95,
        "2.14522227948652325458263602314e-13",
        "0x3.c61f4a318617d94043119898E-11#95",
        Less,
    );
    // - working_prec >= limb_to_bit_count(rn) in limbs_reciprocal_sqrt
    // - neg != 0 && ln != 0 in limbs_reciprocal_sqrt
    test(
        "513.233925441497129966680656795646506",
        "0x201.3be289a8eeba947a6a3693540ab#118",
        118,
        "0.0441410156743607347324939524894430093",
        "0x0.0b4cd35abbcd63a67e82b6f5189c42c#118",
        Less,
    );
    // - rn >= xn << 1 in limbs_reciprocal_sqrt
    test(
        "531607.999405753398566100250398911805595276409254333486007034444758646220477451",
        "0x81c97.ffd90e3247f501afdb3e5781d0c650b8de694094110713b5a698d3d38550#257",
        257,
        "0.00137152663593666837873986552665405007986703190176786715506823381645265890475571",
        "0x0.0059e2660c04516f28e8c959120460810dbcbbd967b4a8d1f95cf325678e624d8b8#257",
        Less,
    );
    // - s != 0 || !x.is_power_of_2() in limbs_reciprocal_sqrt
    // - ahn > an in limbs_reciprocal_sqrt
    test(
        "2.97703041169639178e-21",
        "0xe.0f0249448dd1dE-18#56",
        56,
        "18327716753.4591751",
        "0x4446ac391.758c80#56",
        Less,
    );
    // - an > n in limbs_reciprocal_sqrt
    test(
        "0.000199046277632504184666664672269768242929310652018203552191617720205649",
        "0x0.000d0b7140b8f3aea60aad60c1dc3b2ee0d83e2eba33dcfb6f874df52d78#225",
        26,
        "70.879879",
        "0x46.e13fc#26",
        Less,
    );
}

#[test]
fn reciprocal_sqrt_prec_fail() {
    assert_panic!(Float::NAN.reciprocal_sqrt_prec(0));
    assert_panic!(Float::NAN.reciprocal_sqrt_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.reciprocal_sqrt_prec_assign(0)
    });
}

#[test]
fn test_reciprocal_sqrt_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal_sqrt, o) = x.clone().reciprocal_sqrt_round(rm);
        assert!(reciprocal_sqrt.is_valid());

        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_sqrt_alt, o_alt) = x.reciprocal_sqrt_round_ref(rm);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_sqrt_alt = x.clone();
        let o_alt = reciprocal_sqrt_alt.reciprocal_sqrt_round_assign(rm);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_reciprocal_sqrt, rug_o) =
                rug_reciprocal_sqrt_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
                ComparableFloatRef(&reciprocal_sqrt),
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

    test("Infinity", "Infinity", Floor, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Ceiling, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Down, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Down, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Up, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Nearest, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", Exact, "0.0", "0x0.0", Equal);

    test("-Infinity", "-Infinity", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Exact, "NaN", "NaN", Equal);

    test("0.0", "0x0.0", Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Down, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Up, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", Exact, "Infinity", "Infinity", Equal);

    test("-0.0", "-0x0.0", Floor, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Ceiling, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Down, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Up, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Nearest, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", Exact, "Infinity", "Infinity", Equal);

    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Exact, "1.0", "0x1.0#1", Equal);

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
        "1.0",
        "0x1.0000000000000000000000000#100",
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
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

    test("123.0", "0x7b.0#7", Floor, "0.09", "0x0.170#7", Less);
    test("123.0", "0x7b.0#7", Ceiling, "0.091", "0x0.174#7", Greater);
    test("123.0", "0x7b.0#7", Down, "0.09", "0x0.170#7", Less);
    test("123.0", "0x7b.0#7", Up, "0.091", "0x0.174#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "0.09", "0x0.170#7", Less);

    test("-123.0", "-0x7b.0#7", Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Ceiling, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Down, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Up, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Nearest, "NaN", "NaN", Equal);

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "0.5641895835477563",
        "0x0.906eba8214db68#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "0.5641895835477564",
        "0x0.906eba8214db70#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "0.5641895835477563",
        "0x0.906eba8214db68#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "0.5641895835477564",
        "0x0.906eba8214db70#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "0.5641895835477563",
        "0x0.906eba8214db68#53",
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
}

#[test]
fn reciprocal_sqrt_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.reciprocal_sqrt_round(Exact));
    assert_panic!(THREE.reciprocal_sqrt_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.reciprocal_sqrt_round_assign(Exact);
    });
}

#[test]
fn test_reciprocal_sqrt_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (reciprocal_sqrt, o) = x.clone().reciprocal_sqrt_prec_round(prec, rm);
        assert!(reciprocal_sqrt.is_valid());

        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, o_out);

        let (reciprocal_sqrt_alt, o_alt) = x.reciprocal_sqrt_prec_round_ref(prec, rm);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut reciprocal_sqrt_alt = x.clone();
        let o_alt = reciprocal_sqrt_alt.reciprocal_sqrt_prec_round_assign(prec, rm);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt),
            ComparableFloatRef(&reciprocal_sqrt_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_reciprocal_sqrt, rug_o) =
                rug_reciprocal_sqrt_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
                ComparableFloatRef(&reciprocal_sqrt),
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

    test("Infinity", "Infinity", 1, Floor, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Down, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Up, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Nearest, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Exact, "0.0", "0x0.0", Equal);

    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Exact, "NaN", "NaN", Equal);

    test("0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Down, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Up, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Exact, "Infinity", "Infinity", Equal);

    test("-0.0", "-0x0.0", 1, Floor, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Down, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Up, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Exact, "Infinity", "Infinity", Equal);

    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("1.0", "0x1.0#1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "1.0", "0x1.000#10", Equal);

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
        Down,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Up,
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
        1,
        Exact,
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
        Down,
        "1.0",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Up,
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
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Exact,
        "1.0",
        "0x1.000#10",
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

    test("123.0", "0x7b.0#7", 1, Floor, "0.06", "0x0.1#1", Less);
    test("123.0", "0x7b.0#7", 1, Ceiling, "0.1", "0x0.2#1", Greater);
    test("123.0", "0x7b.0#7", 1, Down, "0.06", "0x0.1#1", Less);
    test("123.0", "0x7b.0#7", 1, Up, "0.1", "0x0.2#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "0.06", "0x0.1#1", Less);

    test(
        "123.0",
        "0x7b.0#7",
        10,
        Floor,
        "0.0901",
        "0x0.1710#10",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "0.0902",
        "0x0.1718#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 10, Down, "0.0901", "0x0.1710#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Up,
        "0.0902",
        "0x0.1718#10",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "0.0902",
        "0x0.1718#10",
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
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Down,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "0.563",
        "0x0.904#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "0.564",
        "0x0.908#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "0.563",
        "0x0.904#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "0.564",
        "0x0.908#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "0.564",
        "0x0.908#10",
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
}

#[test]
fn test_primitive_float_reciprocal_sqrt() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_reciprocal_sqrt(x)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::INFINITY);
    test::<f32>(1.0, 1.0);
    test::<f32>(-1.0, f32::NAN);
    test::<f32>(0.5, core::f32::consts::SQRT_2);
    test::<f32>(-0.5, f32::NAN);
    test::<f32>(2.0, core::f32::consts::FRAC_1_SQRT_2);
    test::<f32>(-2.0, f32::NAN);
    test::<f32>(core::f32::consts::PI, 0.56418955);
    test::<f32>(-core::f32::consts::PI, f32::NAN);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::INFINITY);
    test::<f64>(1.0, 1.0);
    test::<f64>(-1.0, f64::NAN);
    test::<f64>(0.5, core::f64::consts::SQRT_2);
    test::<f64>(-0.5, f64::NAN);
    test::<f64>(2.0, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>(-2.0, f64::NAN);
    test::<f64>(core::f64::consts::PI, 0.5641895835477563);
    test::<f64>(-core::f64::consts::PI, f64::NAN);
}

#[test]
fn test_reciprocal_sqrt_rational_prec() {
    let test = |s, prec, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec(u.clone(), prec);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec_ref(&u, prec);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = reciprocal_sqrt_rational_prec_round_generic(&u, prec, Nearest);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square > u);
                    let mut next = reciprocal_sqrt.clone();
                    next.increment();
                    if next != 0u32 {
                        assert!(Rational::exact_from(&next).square().reciprocal() < u);
                    }
                }
                Greater => {
                    assert!(square < u);
                    let mut previous = reciprocal_sqrt.clone();
                    previous.decrement();
                    if previous != 0u32 {
                        assert!(Rational::exact_from(&previous).square().reciprocal() > u);
                    }
                }
            }
        }
    };
    test("0", 1, "Infinity", "Infinity", Equal);
    test("0", 10, "Infinity", "Infinity", Equal);
    test("0", 100, "Infinity", "Infinity", Equal);
    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0", "0x1.000#10", Equal);
    test("1", 100, "1.0", "0x1.0000000000000000000000000#100", Equal);
    test("1/2", 1, "1.0", "0x1.0#1", Less);
    test("1/2", 10, "1.414", "0x1.6a0#10", Less);
    test(
        "1/2",
        100,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );
    test("1/3", 1, "2.0", "0x2.0#1", Greater);
    test("1/3", 10, "1.732", "0x1.bb8#10", Greater);
    test(
        "1/3",
        100,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );
    test("22/7", 1, "0.5", "0x0.8#1", Less);
    test("22/7", 10, "0.564", "0x0.908#10", Greater);
    test(
        "22/7",
        100,
        "0.5640760748177662089151473616116",
        "0x0.90674a25cc60febbd6cc7515c#100",
        Greater,
    );

    let test_big = |u: Rational, prec, out, out_hex, out_o| {
        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec(u.clone(), prec);
        assert!(reciprocal_sqrt.is_valid());

        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec_ref(&u, prec);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = reciprocal_sqrt_rational_prec_round_generic(&u, prec, Nearest);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square > u);
                    let mut next = reciprocal_sqrt.clone();
                    next.increment();
                    if next != 0u32 {
                        assert!(Rational::exact_from(&next).square().reciprocal() < u);
                    }
                }
                Greater => {
                    assert!(square < u);
                    let mut previous = reciprocal_sqrt.clone();
                    previous.decrement();
                    if previous != 0 {
                        assert!(Rational::exact_from(&previous).square().reciprocal() > u);
                    }
                }
            }
        }
    };
    test_big(
        Rational::power_of_2(1000i64),
        10,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        10,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "too_big",
        "0x1.000E+134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        "too_big",
        "0x1.6a0E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
}

#[test]
fn reciprocal_sqrt_rational_prec_fail() {
    assert_panic!(Float::reciprocal_sqrt_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::reciprocal_sqrt_rational_prec(Rational::ONE, 0));
    assert_panic!(Float::reciprocal_sqrt_rational_prec(
        Rational::NEGATIVE_ONE,
        0
    ));
}

#[test]
fn reciprocal_sqrt_rational_prec_ref_fail() {
    assert_panic!(Float::reciprocal_sqrt_rational_prec_ref(&Rational::ZERO, 0));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_ref(&Rational::ONE, 0));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_ref(
        &Rational::NEGATIVE_ONE,
        0
    ));
}

#[test]
fn test_reciprocal_sqrt_rational_prec_round() {
    let test = |s, prec, rm, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec_round(u.clone(), prec, rm);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(&u, prec, rm);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = reciprocal_sqrt_rational_prec_round_generic(&u, prec, rm);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square > u);
                    let mut next = reciprocal_sqrt.clone();
                    next.increment();
                    if next != 0u32 {
                        assert!(Rational::exact_from(&next).square().reciprocal() < u);
                    }
                }
                Greater => {
                    assert!(square < u);
                    let mut previous = reciprocal_sqrt.clone();
                    previous.decrement();
                    if previous != 0u32 {
                        assert!(Rational::exact_from(&previous).square().reciprocal() > u);
                    }
                }
            }
        }
    };
    test("0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 1, Down, "Infinity", "Infinity", Equal);
    test("0", 1, Up, "Infinity", "Infinity", Equal);
    test("0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0", 1, Exact, "Infinity", "Infinity", Equal);

    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Down, "Infinity", "Infinity", Equal);
    test("0", 10, Up, "Infinity", "Infinity", Equal);
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);

    test("0", 100, Floor, "Infinity", "Infinity", Equal);
    test("0", 100, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 100, Down, "Infinity", "Infinity", Equal);
    test("0", 100, Up, "Infinity", "Infinity", Equal);
    test("0", 100, Nearest, "Infinity", "Infinity", Equal);
    test("0", 100, Exact, "Infinity", "Infinity", Equal);

    test("1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1", 10, Exact, "1.0", "0x1.000#10", Equal);

    test(
        "1",
        100,
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test("1/2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Down, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Nearest, "1.0", "0x1.0#1", Less);

    test("1/2", 10, Floor, "1.414", "0x1.6a0#10", Less);
    test("1/2", 10, Ceiling, "1.416", "0x1.6a8#10", Greater);
    test("1/2", 10, Down, "1.414", "0x1.6a0#10", Less);
    test("1/2", 10, Up, "1.416", "0x1.6a8#10", Greater);
    test("1/2", 10, Nearest, "1.414", "0x1.6a0#10", Less);

    test(
        "1/2",
        100,
        Floor,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "1.414213562373095048801688724211",
        "0x1.6a09e667f3bcc908b2fb13670#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "1.414213562373095048801688724211",
        "0x1.6a09e667f3bcc908b2fb13670#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "1.414213562373095048801688724209",
        "0x1.6a09e667f3bcc908b2fb1366e#100",
        Less,
    );

    test("1/3", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/3", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/3", 1, Down, "1.0", "0x1.0#1", Less);
    test("1/3", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1/3", 1, Nearest, "2.0", "0x2.0#1", Greater);

    test("1/3", 10, Floor, "1.73", "0x1.bb0#10", Less);
    test("1/3", 10, Ceiling, "1.732", "0x1.bb8#10", Greater);
    test("1/3", 10, Down, "1.73", "0x1.bb0#10", Less);
    test("1/3", 10, Up, "1.732", "0x1.bb8#10", Greater);
    test("1/3", 10, Nearest, "1.732", "0x1.bb8#10", Greater);

    test(
        "1/3",
        100,
        Floor,
        "1.732050807568877293527446341505",
        "0x1.bb67ae8584caa73b25742d706#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "1.732050807568877293527446341505",
        "0x1.bb67ae8584caa73b25742d706#100",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "1.732050807568877293527446341506",
        "0x1.bb67ae8584caa73b25742d708#100",
        Greater,
    );

    test("22/7", 1, Floor, "0.5", "0x0.8#1", Less);
    test("22/7", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("22/7", 1, Down, "0.5", "0x0.8#1", Less);
    test("22/7", 1, Up, "1.0", "0x1.0#1", Greater);
    test("22/7", 1, Nearest, "0.5", "0x0.8#1", Less);

    test("22/7", 10, Floor, "0.563", "0x0.904#10", Less);
    test("22/7", 10, Ceiling, "0.564", "0x0.908#10", Greater);
    test("22/7", 10, Down, "0.563", "0x0.904#10", Less);
    test("22/7", 10, Up, "0.564", "0x0.908#10", Greater);
    test("22/7", 10, Nearest, "0.564", "0x0.908#10", Greater);

    test(
        "22/7",
        100,
        Floor,
        "0.564076074817766208915147361611",
        "0x0.90674a25cc60febbd6cc7515b#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "0.5640760748177662089151473616116",
        "0x0.90674a25cc60febbd6cc7515c#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Down,
        "0.564076074817766208915147361611",
        "0x0.90674a25cc60febbd6cc7515b#100",
        Less,
    );
    test(
        "22/7",
        100,
        Up,
        "0.5640760748177662089151473616116",
        "0x0.90674a25cc60febbd6cc7515c#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "0.5640760748177662089151473616116",
        "0x0.90674a25cc60febbd6cc7515c#100",
        Greater,
    );

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

    test("-1/3", 1, Floor, "NaN", "NaN", Equal);
    test("-1/3", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1/3", 1, Down, "NaN", "NaN", Equal);
    test("-1/3", 1, Up, "NaN", "NaN", Equal);
    test("-1/3", 1, Nearest, "NaN", "NaN", Equal);

    test("-1/3", 10, Floor, "NaN", "NaN", Equal);
    test("-1/3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1/3", 10, Down, "NaN", "NaN", Equal);
    test("-1/3", 10, Up, "NaN", "NaN", Equal);
    test("-1/3", 10, Nearest, "NaN", "NaN", Equal);

    test("-1/3", 100, Floor, "NaN", "NaN", Equal);
    test("-1/3", 100, Ceiling, "NaN", "NaN", Equal);
    test("-1/3", 100, Down, "NaN", "NaN", Equal);
    test("-1/3", 100, Up, "NaN", "NaN", Equal);
    test("-1/3", 100, Nearest, "NaN", "NaN", Equal);

    test("-22/7", 1, Floor, "NaN", "NaN", Equal);
    test("-22/7", 1, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 1, Down, "NaN", "NaN", Equal);
    test("-22/7", 1, Up, "NaN", "NaN", Equal);
    test("-22/7", 1, Nearest, "NaN", "NaN", Equal);

    test("-22/7", 10, Floor, "NaN", "NaN", Equal);
    test("-22/7", 10, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 10, Down, "NaN", "NaN", Equal);
    test("-22/7", 10, Up, "NaN", "NaN", Equal);
    test("-22/7", 10, Nearest, "NaN", "NaN", Equal);

    test("-22/7", 100, Floor, "NaN", "NaN", Equal);
    test("-22/7", 100, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 100, Down, "NaN", "NaN", Equal);
    test("-22/7", 100, Up, "NaN", "NaN", Equal);
    test("-22/7", 100, Nearest, "NaN", "NaN", Equal);

    let test_big = |u: Rational, prec, rm, out, out_hex, out_o| {
        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec_round(u.clone(), prec, rm);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(&u, prec, rm);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        let (reciprocal_sqrt, o) = reciprocal_sqrt_rational_prec_round_generic(&u, prec, rm);
        assert!(reciprocal_sqrt.is_valid());
        assert_eq!(reciprocal_sqrt.to_string(), out);
        assert_eq!(to_hex_string(&reciprocal_sqrt), out_hex);
        assert_eq!(o, out_o);

        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square > u);
                    let mut next = reciprocal_sqrt.clone();
                    next.increment();
                    if next != 0u32 {
                        assert!(Rational::exact_from(&next).square().reciprocal() < u);
                    }
                }
                Greater => {
                    assert!(square < u);
                    let mut previous = reciprocal_sqrt.clone();
                    previous.decrement();
                    if previous != 0u32 {
                        assert!(Rational::exact_from(&previous).square().reciprocal() > u);
                    }
                }
            }
        }
    };
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Floor,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Ceiling,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Down,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Up,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Nearest,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Exact,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Floor,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Ceiling,
        "too_small",
        "0x1.6a8E-134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Down,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Up,
        "too_small",
        "0x1.6a8E-134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Floor,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Ceiling,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Down,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Up,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Nearest,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Exact,
        "too_small",
        "0x2.00E-134217728#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Floor,
        "too_small",
        "0x1.0E-134217728#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Ceiling,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Down,
        "too_small",
        "0x1.0E-134217728#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Up,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Nearest,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Floor,
        "too_small",
        "0x1.0E-134217728#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Ceiling,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Down,
        "too_small",
        "0x1.0E-134217728#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Up,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Nearest,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Floor,
        "too_small",
        "0x1.0E-134217728#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Ceiling,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Down,
        "too_small",
        "0x1.0E-134217728#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Up,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Nearest,
        "too_small",
        "0x2.0E-134217728#1",
        Greater,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Floor,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Ceiling,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Down,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Up,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Exact,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Floor,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Ceiling,
        "too_big",
        "0xb.54E+134217727#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Down,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Up,
        "too_big",
        "0xb.54E+134217727#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Nearest,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Floor,
        "too_big",
        "0x1.000E+134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Ceiling,
        "too_big",
        "0x1.000E+134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Down,
        "too_big",
        "0x1.000E+134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Up,
        "too_big",
        "0x1.000E+134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Nearest,
        "too_big",
        "0x1.000E+134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Exact,
        "too_big",
        "0x1.000E+134217728#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Floor,
        "too_big",
        "0x1.6a0E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Ceiling,
        "too_big",
        "0x1.6a8E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Down,
        "too_big",
        "0x1.6a0E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Up,
        "too_big",
        "0x1.6a8E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Nearest,
        "too_big",
        "0x1.6a0E+134217728#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Floor,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Ceiling,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Down,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Up,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Nearest,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "too_big",
        "0x1.698E+134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "too_big",
        "0x1.6a0E+134217728#10",
        Greater,
    );
}

#[test]
fn reciprocal_sqrt_rational_prec_round_fail() {
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round(
        Rational::ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round(
        Rational::ONE,
        0,
        Floor
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round(
        Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round(
        Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
}

#[test]
fn reciprocal_sqrt_rational_prec_round_ref_fail() {
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round_ref(
        &Rational::ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round_ref(
        &Rational::ONE,
        0,
        Floor
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round_ref(
        &Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round_ref(
        &Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::reciprocal_sqrt_rational_prec_round_ref(
        &Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
}

#[test]
fn test_primitive_float_reciprocal_sqrt_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
        Rational: ExactFrom<T>,
    {
        let u = Rational::from_str(s).unwrap();
        let reciprocal_sqrt = primitive_float_reciprocal_sqrt_rational::<T>(&u);
        assert_eq!(NiceFloat(reciprocal_sqrt), NiceFloat(out));
        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(reciprocal_sqrt).square().reciprocal();
            match square.cmp(&u) {
                Less => {
                    let mut next = reciprocal_sqrt;
                    next = next.next_lower();
                    assert!(Rational::exact_from(next).square().reciprocal() > u);
                }
                Greater => {
                    let mut previous = reciprocal_sqrt;
                    previous = previous.next_higher();
                    assert!(Rational::exact_from(previous).square().reciprocal() < u);
                }
                _ => {}
            }
        }
    }
    test::<f32>("0", f32::INFINITY);
    test::<f32>("1", 1.0);
    test::<f32>("1/2", 1.4142135);
    test::<f32>("1/3", 1.7320508);
    test::<f32>("22/7", 0.56407607);
    test::<f32>("1/225", 15.0);
    test::<f32>("-1", f32::NAN);
    test::<f32>("-1/2", f32::NAN);
    test::<f32>("-1/3", f32::NAN);
    test::<f32>("-22/7", f32::NAN);

    test::<f64>("0", f64::INFINITY);
    test::<f64>("1", 1.0);
    test::<f64>("1/2", 1.4142135623730951);
    test::<f64>("1/3", 1.7320508075688772);
    test::<f64>("22/7", 0.5640760748177662);
    test::<f64>("1/225", 15.0);
    test::<f64>("-1", f64::NAN);
    test::<f64>("-1/2", f64::NAN);
    test::<f64>("-1/3", f64::NAN);
    test::<f64>("-22/7", f64::NAN);

    fn test_big<T: PrimitiveFloat>(u: Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
        Rational: ExactFrom<T>,
    {
        let reciprocal_sqrt = primitive_float_reciprocal_sqrt_rational::<T>(&u);
        assert_eq!(NiceFloat(reciprocal_sqrt), NiceFloat(out));
        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(reciprocal_sqrt).square().reciprocal();
            match square.cmp(&u) {
                Less => {
                    let mut next = reciprocal_sqrt;
                    next = next.next_lower();
                    assert!(Rational::exact_from(next).square().reciprocal() > u);
                }
                Greater => {
                    let mut previous = reciprocal_sqrt;
                    previous = previous.next_higher();
                    assert!(Rational::exact_from(previous).square().reciprocal() < u);
                }
                _ => {}
            }
        }
    }
    test_big::<f32>(Rational::power_of_2(1000i64), 0.0);
    test_big::<f32>(Rational::power_of_2(-1000i64), f32::INFINITY);
    test_big::<f32>(Rational::power_of_2(-290i64), f32::INFINITY);
    test_big::<f32>(Rational::power_of_2(-200i64), 1.2676506e30);

    test_big::<f64>(Rational::power_of_2(10000i64), 0.0);
    test_big::<f64>(Rational::power_of_2(1000i64), 3.054936363499605e-151);
    test_big::<f64>(Rational::power_of_2(-10000i64), f64::INFINITY);
    test_big::<f64>(Rational::power_of_2(-2100i64), f64::INFINITY);
    test_big::<f64>(Rational::power_of_2(-1000i64), 3.273390607896142e150);
}

#[test]
fn reciprocal_sqrt_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).reciprocal_sqrt_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).reciprocal_sqrt_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.reciprocal_sqrt_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.reciprocal_sqrt_prec_round(1, Exact));
    assert_panic!(THREE.reciprocal_sqrt_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.reciprocal_sqrt_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_sqrt_prec_round_properties_helper(
    x: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (reciprocal_sqrt, o) = x.clone().reciprocal_sqrt_prec_round(prec, rm);
    assert!(reciprocal_sqrt.is_valid());

    let (reciprocal_sqrt_alt, o_alt) = x.clone().reciprocal_sqrt_prec_round_ref(prec, rm);
    assert!(reciprocal_sqrt_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_sqrt_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_reciprocal_sqrt, rug_o) =
            rug_reciprocal_sqrt_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
            ComparableFloatRef(&reciprocal_sqrt),
        );
        assert_eq!(rug_o, o);
    }

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(reciprocal_sqrt.is_sign_positive());
    }

    if reciprocal_sqrt.is_normal() {
        assert_eq!(reciprocal_sqrt.get_prec(), Some(prec));
        if x > 1u32 && o < Greater {
            assert!(reciprocal_sqrt < x);
        } else if x < 1u32 && o > Less {
            assert!(reciprocal_sqrt > x);
        }
    }

    if !extreme && x.is_normal() && reciprocal_sqrt.is_normal() {
        let reciprocal_square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
        match o {
            Equal => assert_eq!(reciprocal_square, x),
            Less => {
                assert!(reciprocal_square > x);
                let mut next = reciprocal_sqrt.clone();
                next.increment();
                if next != 0u32 {
                    assert!(Rational::exact_from(&next).square().reciprocal() < x);
                }
            }
            Greater => {
                assert!(reciprocal_square < x);
                let mut previous = reciprocal_sqrt.clone();
                previous.decrement();
                if previous != 0u32 {
                    assert!(Rational::exact_from(&previous).square().reciprocal() > x);
                }
            }
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.reciprocal_sqrt_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(reciprocal_sqrt.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.reciprocal_sqrt_prec_round_ref(prec, Exact));
    }
}

#[test]
fn reciprocal_sqrt_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_15().test_properties(|(x, prec, rm)| {
        reciprocal_sqrt_prec_round_properties_helper(x, prec, rm, false);
    });

    float_unsigned_rounding_mode_triple_gen_var_16().test_properties(|(x, prec, rm)| {
        reciprocal_sqrt_prec_round_properties_helper(x, prec, rm, true);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (product, o) = Float::NAN.reciprocal_sqrt_prec_round(prec, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.reciprocal_sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.reciprocal_sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.reciprocal_sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.reciprocal_sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ONE.reciprocal_sqrt_prec_round(prec, rm),
            (Float::one_prec(prec), Equal)
        );

        let (s, o) = Float::NEGATIVE_ONE.reciprocal_sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_sqrt_prec_properties_helper(x: Float, prec: u64, extreme: bool) {
    let (reciprocal_sqrt, o) = x.clone().reciprocal_sqrt_prec(prec);
    assert!(reciprocal_sqrt.is_valid());

    let (reciprocal_sqrt_alt, o_alt) = x.reciprocal_sqrt_prec_ref(prec);
    assert!(reciprocal_sqrt_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_sqrt_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    assert_eq!(o_alt, o);

    let (rug_reciprocal_sqrt, rug_o) = rug_reciprocal_sqrt_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
        ComparableFloatRef(&reciprocal_sqrt),
    );
    assert_eq!(rug_o, o);

    let (reciprocal_sqrt_alt, o_alt) = x.reciprocal_sqrt_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    assert_eq!(o_alt, o);

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(reciprocal_sqrt.is_sign_positive());
    }

    if reciprocal_sqrt.is_normal() {
        assert_eq!(reciprocal_sqrt.get_prec(), Some(prec));
        if x > 1u32 && o < Greater {
            assert!(reciprocal_sqrt < x);
        } else if x < 1u32 && o > Less {
            assert!(reciprocal_sqrt > x);
        }
    }

    if !extreme && x.is_normal() && reciprocal_sqrt.is_normal() {
        let reciprocal_square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
        match o {
            Equal => assert_eq!(reciprocal_square, x),
            Less => {
                assert!(reciprocal_square > x);
                let mut next = reciprocal_sqrt.clone();
                next.increment();
                if next != 0u32 {
                    assert!(Rational::exact_from(&next).square().reciprocal() < x);
                }
            }
            Greater => {
                assert!(reciprocal_square < x);
                let mut previous = reciprocal_sqrt.clone();
                previous.decrement();
                if previous != 0 {
                    assert!(Rational::exact_from(&previous).square().reciprocal() > x);
                }
            }
        }
    }
}

#[test]
fn reciprocal_sqrt_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        reciprocal_sqrt_prec_properties_helper(x, prec, false);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        reciprocal_sqrt_prec_properties_helper(x, prec, false);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        reciprocal_sqrt_prec_properties_helper(x, prec, true);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (reciprocal_sqrt, o) = Float::NAN.reciprocal_sqrt_prec(prec);
        assert!(reciprocal_sqrt.is_nan());
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::ZERO.reciprocal_sqrt_prec(prec);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::INFINITY)
        );
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::NEGATIVE_ZERO.reciprocal_sqrt_prec(prec);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::INFINITY)
        );
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::INFINITY.reciprocal_sqrt_prec(prec);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::NEGATIVE_INFINITY.reciprocal_sqrt_prec(prec);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::NAN)
        );
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ONE.reciprocal_sqrt_prec(prec),
            (Float::one_prec(prec), Equal)
        );

        let (reciprocal_sqrt, o) = Float::NEGATIVE_ONE.reciprocal_sqrt_prec(prec);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::NAN)
        );
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_sqrt_round_properties_helper(x: Float, rm: RoundingMode, extreme: bool) {
    let (reciprocal_sqrt, o) = x.clone().reciprocal_sqrt_round(rm);
    assert!(reciprocal_sqrt.is_valid());

    let (reciprocal_sqrt_alt, o_alt) = x.reciprocal_sqrt_round_ref(rm);
    assert!(reciprocal_sqrt_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.reciprocal_sqrt_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    assert_eq!(o_alt, o);

    let (reciprocal_sqrt_alt, o_alt) = x.reciprocal_sqrt_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    assert_eq!(o_alt, o);

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(reciprocal_sqrt.is_sign_positive());
    }

    if reciprocal_sqrt.is_normal() {
        assert_eq!(reciprocal_sqrt.get_prec(), Some(x.get_prec().unwrap()));
        if x > 1u32 && o < Greater {
            assert!(reciprocal_sqrt < x);
        } else if x < 1u32 && o > Less {
            assert!(reciprocal_sqrt > x);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_reciprocal_sqrt, rug_o) =
            rug_reciprocal_sqrt_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
            ComparableFloatRef(&reciprocal_sqrt),
        );
        assert_eq!(rug_o, o);
    }

    if !extreme && x.is_normal() && reciprocal_sqrt.is_normal() {
        let reciprocal_square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
        match o {
            Equal => assert_eq!(reciprocal_square, x),
            Less => {
                assert!(reciprocal_square > x);
                let mut next = reciprocal_sqrt.clone();
                next.increment();
                if next != 0u32 {
                    assert!(Rational::exact_from(&next).square().reciprocal() < x);
                }
            }
            Greater => {
                assert!(reciprocal_square < x);
                let mut previous = reciprocal_sqrt.clone();
                previous.decrement();
                if previous != 0u32 {
                    assert!(Rational::exact_from(&previous).square().reciprocal() > x);
                }
            }
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.reciprocal_sqrt_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(reciprocal_sqrt.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.reciprocal_sqrt_round_ref(Exact));
    }
}

#[test]
fn reciprocal_sqrt_round_properties() {
    float_rounding_mode_pair_gen_var_30().test_properties(|(x, rm)| {
        reciprocal_sqrt_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_31().test_properties(|(x, rm)| {
        reciprocal_sqrt_round_properties_helper(x, rm, true);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (reciprocal_sqrt, o) = Float::NAN.reciprocal_sqrt_round(rm);
        assert!(reciprocal_sqrt.is_nan());
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::ZERO.reciprocal_sqrt_round(rm);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::INFINITY)
        );
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::NEGATIVE_ZERO.reciprocal_sqrt_round(rm);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::INFINITY)
        );
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::INFINITY.reciprocal_sqrt_round(rm);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::ZERO)
        );
        assert_eq!(o, Equal);

        let (reciprocal_sqrt, o) = Float::NEGATIVE_INFINITY.reciprocal_sqrt_round(rm);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::NAN)
        );
        assert_eq!(o, Equal);

        assert_eq!(Float::ONE.reciprocal_sqrt_round(rm), (Float::ONE, Equal));

        let (reciprocal_sqrt, o) = Float::NEGATIVE_ONE.reciprocal_sqrt_round(rm);
        assert_eq!(
            ComparableFloat(reciprocal_sqrt),
            ComparableFloat(Float::NAN)
        );
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn reciprocal_sqrt_properties_helper(x: Float, extreme: bool) {
    let reciprocal_sqrt = x.clone().reciprocal_sqrt();
    assert!(reciprocal_sqrt.is_valid());

    let reciprocal_sqrt_alt = (&x).reciprocal_sqrt();
    assert!(reciprocal_sqrt_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );

    let mut x_alt = x.clone();
    x_alt.reciprocal_sqrt_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );

    let reciprocal_sqrt_alt = x
        .reciprocal_sqrt_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );
    let reciprocal_sqrt_alt = x.reciprocal_sqrt_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );

    let reciprocal_sqrt_alt = x.reciprocal_sqrt_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&reciprocal_sqrt_alt),
        ComparableFloatRef(&reciprocal_sqrt)
    );

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(reciprocal_sqrt.is_sign_positive());
    }

    if x.is_normal() && reciprocal_sqrt.is_normal() {
        assert_eq!(reciprocal_sqrt.get_prec(), Some(x.get_prec().unwrap()));
    }

    if !extreme && x.is_normal() && reciprocal_sqrt.is_normal() {
        assert_eq!(reciprocal_sqrt.get_prec(), Some(x.get_prec().unwrap()));
        let reciprocal_square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
        if reciprocal_square < x {
            let mut next = reciprocal_sqrt.clone();
            next.decrement();
            assert!(Rational::exact_from(&next).square().reciprocal() > x);
        } else if reciprocal_square > x {
            let mut previous = reciprocal_sqrt.clone();
            previous.increment();
            assert!(Rational::exact_from(&previous).square().reciprocal() < x);
        }
    }

    let rug_reciprocal_sqrt = rug_reciprocal_sqrt(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
        ComparableFloatRef(&reciprocal_sqrt),
    );
}

#[test]
fn reciprocal_sqrt_properties() {
    float_gen().test_properties(|x| {
        reciprocal_sqrt_properties_helper(x, false);
    });

    float_gen_var_12().test_properties(|x| {
        reciprocal_sqrt_properties_helper(x, true);
    });
}

fn primitive_float_reciprocal_sqrt_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let reciprocal_sqrt = primitive_float_reciprocal_sqrt(x);
        if x.is_normal() && reciprocal_sqrt.is_normal() {
            let reciprocal_square = Rational::exact_from(reciprocal_sqrt).square().reciprocal();
            if reciprocal_square < x {
                let next = reciprocal_sqrt.next_lower();
                assert!(Rational::exact_from(next).square().reciprocal() > x);
            } else if reciprocal_square > x {
                let previous = reciprocal_sqrt.next_higher();
                assert!(Rational::exact_from(previous).square().reciprocal() < x);
            }
        }
    });
}

#[test]
fn primitive_float_reciprocal_sqrt_properties() {
    apply_fn_to_primitive_floats!(primitive_float_reciprocal_sqrt_properties_helper);
}

#[test]
fn reciprocal_sqrt_rational_prec_properties() {
    rational_unsigned_pair_gen_var_7().test_properties(|(x, prec)| {
        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec(x.clone(), prec);
        assert!(reciprocal_sqrt.is_valid());

        let (reciprocal_sqrt_alt, o_alt) = Float::reciprocal_sqrt_rational_prec_ref(&x, prec);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt_alt),
            ComparableFloatRef(&reciprocal_sqrt)
        );
        assert_eq!(o, o_alt);

        let (reciprocal_sqrt_alt, o_alt) =
            Float::reciprocal_sqrt_rational_prec_round_ref(&x, prec, Nearest);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt_alt),
            ComparableFloatRef(&reciprocal_sqrt)
        );
        assert_eq!(o, o_alt);

        let (reciprocal_sqrt_alt, o_alt) =
            reciprocal_sqrt_rational_prec_round_generic(&x, prec, Nearest);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt_alt),
            ComparableFloatRef(&reciprocal_sqrt)
        );
        assert_eq!(o, o_alt);

        if !reciprocal_sqrt.is_nan() {
            assert_eq!(
                reciprocal_sqrt.get_prec(),
                if x == 0u32 { None } else { Some(prec) }
            );
        }

        if x >= 0u32 {
            assert!(reciprocal_sqrt.is_sign_positive());
        }

        if reciprocal_sqrt.is_normal() {
            if x > 1u32 && o < Greater {
                assert!(reciprocal_sqrt < x);
            } else if x < 1u32 && o > Less {
                assert!(reciprocal_sqrt > x);
            }

            let square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
            match o {
                Equal => assert_eq!(square, x),
                Less => {
                    assert!(square > x);
                    let mut next = reciprocal_sqrt.clone();
                    next.increment();
                    if next != 0u32 {
                        assert!(Rational::exact_from(&next).square().reciprocal() < x);
                    }
                }
                Greater => {
                    assert!(square < x);
                    let mut previous = reciprocal_sqrt.clone();
                    previous.decrement();
                    if previous != 0 {
                        assert!(Rational::exact_from(&previous).square().reciprocal() > x);
                    }
                }
            }
        }
    });
}

#[test]
fn reciprocal_sqrt_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_4().test_properties(|(x, prec, rm)| {
        let (reciprocal_sqrt, o) = Float::reciprocal_sqrt_rational_prec_round(x.clone(), prec, rm);
        assert!(reciprocal_sqrt.is_valid());

        let (reciprocal_sqrt_alt, o_alt) =
            Float::reciprocal_sqrt_rational_prec_round_ref(&x, prec, rm);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt_alt),
            ComparableFloatRef(&reciprocal_sqrt)
        );
        assert_eq!(o, o_alt);

        if !reciprocal_sqrt.is_nan() {
            match (x >= 0, rm) {
                (_, Floor) | (true, Down) | (false, Up) => {
                    assert_ne!(o, Greater);
                }
                (_, Ceiling) | (true, Up) | (false, Down) => {
                    assert_ne!(o, Less);
                }
                (_, Exact) => assert_eq!(o, Equal),
                _ => {}
            }
        }

        let (reciprocal_sqrt_alt, o_alt) =
            reciprocal_sqrt_rational_prec_round_generic(&x, prec, rm);
        assert!(reciprocal_sqrt_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&reciprocal_sqrt_alt),
            ComparableFloatRef(&reciprocal_sqrt)
        );
        assert_eq!(o, o_alt);

        if !reciprocal_sqrt.is_nan() {
            assert_eq!(
                reciprocal_sqrt.get_prec(),
                if x == 0u32 { None } else { Some(prec) }
            );
        }

        if x >= 0u32 {
            assert!(reciprocal_sqrt.is_sign_positive());
        }

        if reciprocal_sqrt.is_normal() {
            if x > 1u32 && o < Greater {
                assert!(reciprocal_sqrt < x);
            } else if x < 1u32 && o > Less {
                assert!(reciprocal_sqrt > x);
            }
        }

        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(&reciprocal_sqrt).square().reciprocal();
            match o {
                Equal => assert_eq!(square, x),
                Less => {
                    assert!(square > x);
                    let mut next = reciprocal_sqrt.clone();
                    next.increment();
                    if next != 0 {
                        assert!(Rational::exact_from(&next).square().reciprocal() < x);
                    }
                }
                Greater => {
                    assert!(square < x);
                    let mut previous = reciprocal_sqrt.clone();
                    previous.decrement();
                    if previous != 0u32 {
                        assert!(Rational::exact_from(&previous).square().reciprocal() > x);
                    }
                }
            }
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = Float::reciprocal_sqrt_rational_prec_round_ref(&x, prec, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero_ref()),
                    ComparableFloat(reciprocal_sqrt.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(Float::reciprocal_sqrt_rational_prec_round_ref(
                &x, prec, Exact
            ));
        }
    });

    // reciprocal_sqrt(3/5) is one of the simplest cases that doesn't reduce to sqrt or
    // reciprocal_sqrt of a Float
    const X: Rational = Rational::const_from_unsigneds(3, 5);
    test_constant(
        |prec, rm| Float::reciprocal_sqrt_rational_prec_round(X, prec, rm),
        10000,
    );
}

fn primitive_float_reciprocal_sqrt_rational_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    Rational: ExactFrom<T>,
{
    rational_gen_var_1().test_properties(|x| {
        let reciprocal_sqrt = primitive_float_reciprocal_sqrt_rational::<T>(&x);
        if reciprocal_sqrt.is_normal() {
            let square = Rational::exact_from(reciprocal_sqrt).square().reciprocal();
            match square.cmp(&x) {
                Less => {
                    assert!(
                        Rational::exact_from(reciprocal_sqrt.next_higher())
                            .square()
                            .reciprocal()
                            < x
                    );
                }
                Greater => {
                    assert!(
                        Rational::exact_from(reciprocal_sqrt.next_lower())
                            .square()
                            .reciprocal()
                            > x
                    );
                }
                _ => {}
            }
        }
    });

    rational_pair_gen_var_8().test_properties(|(x, y)| {
        let reciprocal_sqrt_x = NiceFloat(primitive_float_reciprocal_sqrt_rational::<T>(&x));
        let reciprocal_sqrt_y = NiceFloat(primitive_float_reciprocal_sqrt_rational::<T>(&y));
        if !reciprocal_sqrt_x.0.is_nan() && !reciprocal_sqrt_y.0.is_nan() {
            match x.partial_cmp(&y).unwrap() {
                Equal => assert_eq!(reciprocal_sqrt_x, reciprocal_sqrt_y),
                Less => assert!(reciprocal_sqrt_x >= reciprocal_sqrt_y),
                Greater => assert!(reciprocal_sqrt_x <= reciprocal_sqrt_y),
            }
        }
    });
}

#[test]
fn primitive_float_reciprocal_sqrt_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_reciprocal_sqrt_rational_helper);
}
