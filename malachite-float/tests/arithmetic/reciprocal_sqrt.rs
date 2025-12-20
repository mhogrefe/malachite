// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{ReciprocalSqrt, ReciprocalSqrtAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::primitive_float_gen;
use malachite_base::test_util::generators::{
    rounding_mode_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::arithmetic::reciprocal_sqrt::{f32_reciprocal_sqrt, f64_reciprocal_sqrt};
use malachite_float::test_util::arithmetic::reciprocal_sqrt::{
    rug_reciprocal_sqrt, rug_reciprocal_sqrt_prec, rug_reciprocal_sqrt_prec_round,
    rug_reciprocal_sqrt_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_30,
    float_rounding_mode_pair_gen_var_31, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_15,
    float_unsigned_rounding_mode_triple_gen_var_16,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use std::panic::catch_unwind;

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
fn test_f32_reciprocal_sqrt() {
    fn test(x: f32, out: f32) {
        assert_eq!(NiceFloat(f32_reciprocal_sqrt(x)), NiceFloat(out));
    }
    test(f32::NAN, f32::NAN);
    test(f32::INFINITY, 0.0);
    test(f32::NEGATIVE_INFINITY, f32::NAN);
    test(0.0, f32::INFINITY);
    test(-0.0, f32::INFINITY);
    test(1.0, 1.0);
    test(-1.0, f32::NAN);
    test(0.5, core::f32::consts::SQRT_2);
    test(-0.5, f32::NAN);
    test(2.0, core::f32::consts::FRAC_1_SQRT_2);
    test(-2.0, f32::NAN);
    test(core::f32::consts::PI, 0.56418955);
    test(-core::f32::consts::PI, f32::NAN);
}

#[test]
fn test_f64_reciprocal_sqrt() {
    fn test(x: f64, out: f64) {
        assert_eq!(NiceFloat(f64_reciprocal_sqrt(x)), NiceFloat(out));
    }
    test(f64::NAN, f64::NAN);
    test(f64::INFINITY, 0.0);
    test(f64::NEGATIVE_INFINITY, f64::NAN);
    test(0.0, f64::INFINITY);
    test(-0.0, f64::INFINITY);
    test(1.0, 1.0);
    test(-1.0, f64::NAN);
    test(0.5, core::f64::consts::SQRT_2);
    test(-0.5, f64::NAN);
    test(2.0, core::f64::consts::FRAC_1_SQRT_2);
    test(-2.0, f64::NAN);
    test(core::f64::consts::PI, 0.5641895835477563);
    test(-core::f64::consts::PI, f64::NAN);
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
fn reciprocal_sqrt_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
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
        reciprocal_sqrt_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_16().test_properties(|(x, prec, rm)| {
        reciprocal_sqrt_prec_round_properties_helper(x, prec, rm);
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
fn reciprocal_sqrt_prec_properties_helper(x: Float, prec: u64) {
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
}

#[test]
fn reciprocal_sqrt_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        reciprocal_sqrt_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        reciprocal_sqrt_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        reciprocal_sqrt_prec_properties_helper(x, prec);
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
fn reciprocal_sqrt_round_properties_helper(x: Float, rm: RoundingMode) {
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
        reciprocal_sqrt_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_31().test_properties(|(x, rm)| {
        reciprocal_sqrt_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_30().test_properties(|(x, rm)| {
        reciprocal_sqrt_round_properties_helper(x, rm);
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
fn reciprocal_sqrt_properties_helper(x: Float) {
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

    let rug_reciprocal_sqrt = rug_reciprocal_sqrt(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_reciprocal_sqrt)),
        ComparableFloatRef(&reciprocal_sqrt),
    );
}

#[test]
fn reciprocal_sqrt_properties() {
    float_gen().test_properties(|x| {
        reciprocal_sqrt_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        reciprocal_sqrt_properties_helper(x);
    });
}

#[test]
fn f32_reciprocal_sqrt_properties() {
    primitive_float_gen().test_properties(|x| {
        f32_reciprocal_sqrt(x);
    });
}

#[test]
fn f64_reciprocal_sqrt_properties() {
    primitive_float_gen().test_properties(|x| {
        f64_reciprocal_sqrt(x);
    });
}
