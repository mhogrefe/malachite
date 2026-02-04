// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::{f32, f64};
use malachite_base::num::arithmetic::traits::{Ln, LnAssign};
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
use malachite_float::arithmetic::ln::primitive_float_ln;
use malachite_float::test_util::arithmetic::ln::{
    rug_ln, rug_ln_prec, rug_ln_prec_round, rug_ln_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_34,
    float_rounding_mode_pair_gen_var_35, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_19,
    float_unsigned_rounding_mode_triple_gen_var_20,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use std::panic::catch_unwind;

#[test]
fn test_ln() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let ln = x.clone().ln();
        assert!(ln.is_valid());

        assert_eq!(ln.to_string(), out);
        assert_eq!(to_hex_string(&ln), out_hex);

        let ln_alt = (&x).ln();
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));

        let mut ln_alt = x.clone();
        ln_alt.ln_assign();
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&ln)
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

    test("123.0", "0x7b.0#7", "4.81", "0x4.d#7");
    test("-123.0", "-0x7b.0#7", "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1.1447298858494002",
        "0x1.250d048e7a1bd#53",
    );
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "NaN", "NaN");

    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        "1.577721810442023610823457130564e-30",
        "0x1.ffffffffffffffffffffffffeE-25#100",
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        "-1.577721810442023610823457130566e-30",
        "-0x2.0000000000000000000000000E-25#99",
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "1.0",
        "0x1.0000000000000#53",
    );
}

#[test]
fn test_ln_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln, o) = x.clone().ln_prec(prec);
        assert!(ln.is_valid());

        assert_eq!(ln.to_string(), out);
        assert_eq!(to_hex_string(&ln), out_hex);
        assert_eq!(o, o_out);

        let (ln_alt, o_alt) = x.ln_prec_ref(prec);
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));
        assert_eq!(o_alt, o_out);

        let mut ln_alt = x.clone();
        let o_alt = ln_alt.ln_prec_assign(prec);
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));
        assert_eq!(o_alt, o_out);

        let (rug_ln, rug_o) = rug_ln_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln)),
            ComparableFloatRef(&ln),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 1, "-Infinity", "-Infinity", Equal);
    // - in ln_prec_round_normal
    // - *x == 1u32 in ln_prec_round_normal
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
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);

    test("123.0", "0x7b.0#7", 1, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 10, "4.81", "0x4.d0#10", Greater);
    test("-123.0", "-0x7b.0#7", 1, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", 10, "NaN", "NaN", Equal);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "1.145",
        "0x1.250#10",
        Less,
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
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        "1.578e-30",
        "0x2.00E-25#10",
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
        "-1.578e-30",
        "-0x2.00E-25#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "1.0",
        "0x1.000#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1000,
        "0.999999999999999946817622933941086294801645535631167330140966894773031833945843167502391\
        658696177866134372942956876650105751129080333063719848201745765425041221713102525849084801\
        372391581210765049471829553635989842946539858533293656242339571977421630183546441129939950\
        3482873700231381735818901075103963",
        "0x0.fffffffffffffc2af5533763665751db2b3c952fde86afe1810bb66ea1494e989573a8be58f802b1c430a\
        02dc8ffc80b2e393a359078dce063403a2113f8777bb9d11195ac4058bd5552c15bae9bd34651b64dd0f9acde5\
        926173f1391023dd0bccba5e4cc09fab80c32b730f5bd7df655766e551cbd2672a9afa3df95#1000",
        Greater,
    );
    // - *x != 1u32 in ln_prec_round_normal
    // - tmp1.is_normal() && tmp2.is_normal() in ln_prec_round_normal
    // - !float_can_round in ln_prec_round_normal
    // - float_can_round in ln_prec_round_normal
    test("2.0", "0x2.0#1", 1, "0.5", "0x0.8#1", Less);
    // - !tmp1.is_normal() && !tmp2.is_normal() in ln_prec_round_normal
    test(
        "0.999998",
        "0x0.ffffe#19",
        5,
        "-1.9e-6",
        "-0x0.000020#5",
        Greater,
    );
}

#[test]
fn ln_prec_fail() {
    assert_panic!(Float::NAN.ln_prec(0));
    assert_panic!(Float::NAN.ln_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.ln_prec_assign(0)
    });
}

#[test]
fn test_ln_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln, o) = x.clone().ln_round(rm);
        assert!(ln.is_valid());

        assert_eq!(ln.to_string(), out);
        assert_eq!(to_hex_string(&ln), out_hex);
        assert_eq!(o, o_out);

        let (ln_alt, o_alt) = x.ln_round_ref(rm);
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));
        assert_eq!(o_alt, o_out);

        let mut ln_alt = x.clone();
        let o_alt = ln_alt.ln_round_assign(rm);
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_ln, rug_o) = rug_ln_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_ln)),
                ComparableFloatRef(&ln),
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

    test("123.0", "0x7b.0#7", Floor, "4.75", "0x4.c#7", Less);
    test("123.0", "0x7b.0#7", Ceiling, "4.81", "0x4.d#7", Greater);
    test("123.0", "0x7b.0#7", Down, "4.75", "0x4.c#7", Less);
    test("123.0", "0x7b.0#7", Up, "4.81", "0x4.d#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "4.81", "0x4.d#7", Greater);

    test("-123.0", "-0x7b.0#7", Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Ceiling, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Down, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Up, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Nearest, "NaN", "NaN", Equal);

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "1.1447298858493999",
        "0x1.250d048e7a1bc#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "1.1447298858494002",
        "0x1.250d048e7a1bd#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "1.1447298858493999",
        "0x1.250d048e7a1bc#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "1.1447298858494002",
        "0x1.250d048e7a1bd#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "1.1447298858494002",
        "0x1.250d048e7a1bd#53",
        Greater,
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
        "5.0e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "too_big",
        "0x6.0E+268435455#2",
        Nearest,
        "8.0e8",
        "0x3.0E+7#2",
        Greater,
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        Nearest,
        "-5.0e8",
        "-0x2.0E+7#1",
        Greater,
    );
    test(
        "too_small",
        "0x1.0E-268435456#2",
        Nearest,
        "-8.0e8",
        "-0x3.0E+7#2",
        Less,
    );

    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Floor,
        "1.577721810442023610823457130564e-30",
        "0x1.ffffffffffffffffffffffffeE-25#100",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Ceiling,
        "1.577721810442023610823457130566e-30",
        "0x2.0000000000000000000000000E-25#100",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Down,
        "1.577721810442023610823457130564e-30",
        "0x1.ffffffffffffffffffffffffeE-25#100",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Up,
        "1.577721810442023610823457130566e-30",
        "0x2.0000000000000000000000000E-25#100",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Nearest,
        "1.577721810442023610823457130564e-30",
        "0x1.ffffffffffffffffffffffffeE-25#100",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Floor,
        "-1.577721810442023610823457130571e-30",
        "-0x2.0000000000000000000000008E-25#99",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Ceiling,
        "-1.577721810442023610823457130566e-30",
        "-0x2.0000000000000000000000000E-25#99",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Down,
        "-1.577721810442023610823457130566e-30",
        "-0x2.0000000000000000000000000E-25#99",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Up,
        "-1.577721810442023610823457130571e-30",
        "-0x2.0000000000000000000000008E-25#99",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Nearest,
        "-1.577721810442023610823457130566e-30",
        "-0x2.0000000000000000000000000E-25#99",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "0.9999999999999999",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "1.0",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "0.9999999999999999",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "1.0",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "1.0",
        "0x1.0000000000000#53",
        Greater,
    );
}

#[test]
fn ln_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.ln_round(Exact));
    assert_panic!(THREE.ln_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.ln_round_assign(Exact);
    });
}

#[test]
fn test_ln_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln, o) = x.clone().ln_prec_round(prec, rm);
        assert!(ln.is_valid());

        assert_eq!(ln.to_string(), out);
        assert_eq!(to_hex_string(&ln), out_hex);
        assert_eq!(o, o_out);

        let (ln_alt, o_alt) = x.ln_prec_round_ref(prec, rm);
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));
        assert_eq!(o_alt, o_out);

        let mut ln_alt = x.clone();
        let o_alt = ln_alt.ln_prec_round_assign(prec, rm);
        assert!(ln_alt.is_valid());
        assert_eq!(ComparableFloatRef(&ln), ComparableFloatRef(&ln_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_ln, rug_o) = rug_ln_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_ln)),
                ComparableFloatRef(&ln),
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

    test("123.0", "0x7b.0#7", 1, Floor, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Down, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Up, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "4.0", "0x4.0#1", Less);

    test("123.0", "0x7b.0#7", 10, Floor, "4.805", "0x4.ce#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "4.81",
        "0x4.d0#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 10, Down, "4.805", "0x4.ce#10", Less);
    test("123.0", "0x7b.0#7", 10, Up, "4.81", "0x4.d0#10", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "4.81",
        "0x4.d0#10",
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
        "1.0",
        "0x1.0#1",
        Less,
    );

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "1.145",
        "0x1.250#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "1.146",
        "0x1.258#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "1.145",
        "0x1.250#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "1.146",
        "0x1.258#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "1.145",
        "0x1.250#10",
        Less,
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
        "8.0e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Floor,
        "1.576e-30",
        "0x1.ff8E-25#10",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Ceiling,
        "2.0e-30",
        "0x2.0E-25#1",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Ceiling,
        "1.578e-30",
        "0x2.00E-25#10",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Down,
        "8.0e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Down,
        "1.576e-30",
        "0x1.ff8E-25#10",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Up,
        "2.0e-30",
        "0x2.0E-25#1",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Up,
        "1.578e-30",
        "0x2.00E-25#10",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Nearest,
        "2.0e-30",
        "0x2.0E-25#1",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Nearest,
        "1.578e-30",
        "0x2.00E-25#10",
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
        "-1.581e-30",
        "-0x2.01E-25#10",
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
        "-1.578e-30",
        "-0x2.00E-25#10",
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
        "-1.578e-30",
        "-0x2.00E-25#10",
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
        "-1.581e-30",
        "-0x2.01E-25#10",
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
        "-1.578e-30",
        "-0x2.00E-25#10",
        Greater,
    );

    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Floor,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "0.999",
        "0x0.ffc#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "1.0",
        "0x1.000#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Down,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Down,
        "0.999",
        "0x0.ffc#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Up,
        "1.0",
        "0x1.000#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Greater,
    );
}

#[test]
fn ln_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).ln_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).ln_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.ln_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.ln_prec_round(1, Exact));
    assert_panic!(THREE.ln_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.ln_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (ln, o) = x.clone().ln_prec_round(prec, rm);
    assert!(ln.is_valid());

    let (ln_alt, o_alt) = x.clone().ln_prec_round_ref(prec, rm);
    assert!(ln_alt.is_valid());
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_ln, rug_o) = rug_ln_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln)),
            ComparableFloatRef(&ln),
        );
        assert_eq!(rug_o, o);
    }

    if x >= 0u32 && x.is_finite() {
        assert!(ln < x);
    }

    if ln.is_normal() {
        assert_eq!(ln.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(ln > 0u32);
        } else if x < 1u32 && o < Greater {
            assert!(ln < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.ln_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(ln.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.ln_prec_round_ref(prec, Exact));
    }
}

#[test]
fn ln_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_19().test_properties(|(x, prec, rm)| {
        ln_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_20().test_properties(|(x, prec, rm)| {
        ln_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (ln, o) = Float::NAN.ln_prec_round(prec, rm);
        assert!(ln.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.ln_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (s, o) = Float::NEGATIVE_INFINITY.ln_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.ln_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::NEGATIVE_ZERO.ln_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(Float::ONE.ln_prec_round(prec, rm), (Float::ZERO, Equal));

        let (s, o) = Float::NEGATIVE_ONE.ln_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_prec_properties_helper(x: Float, prec: u64) {
    let (ln, o) = x.clone().ln_prec(prec);
    assert!(ln.is_valid());

    let (ln_alt, o_alt) = x.ln_prec_ref(prec);
    assert!(ln_alt.is_valid());
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln));
    assert_eq!(o_alt, o);

    let (rug_ln, rug_o) = rug_ln_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_ln)),
        ComparableFloatRef(&ln),
    );
    assert_eq!(rug_o, o);

    let (ln_alt, o_alt) = x.ln_prec_round_ref(prec, Nearest);
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));
    assert_eq!(o_alt, o);

    if x >= 0u32 && x.is_finite() {
        assert!(ln < x);
    }

    if ln.is_normal() {
        assert_eq!(ln.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(ln > 0u32);
        } else if x < 1u32 && o < Greater {
            assert!(ln < 0u32);
        }
    }
}

#[test]
fn ln_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        ln_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        ln_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        ln_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (ln, o) = Float::NAN.ln_prec(prec);
        assert!(ln.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(Float::ZERO.ln_prec(prec), (Float::NEGATIVE_INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_ZERO.ln_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert_eq!(Float::INFINITY.ln_prec(prec), (Float::INFINITY, Equal));
        let (ln, o) = Float::NEGATIVE_INFINITY.ln_prec(prec);
        assert_eq!(ComparableFloat(ln), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(Float::ONE.ln_prec(prec), (Float::ZERO, Equal));

        let (ln, o) = Float::NEGATIVE_ONE.ln_prec(prec);
        assert_eq!(ComparableFloat(ln), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_round_properties_helper(x: Float, rm: RoundingMode) {
    let (ln, o) = x.clone().ln_round(rm);
    assert!(ln.is_valid());

    let (ln_alt, o_alt) = x.ln_round_ref(rm);
    assert!(ln_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln));
    assert_eq!(o_alt, o);

    let (ln_alt, o_alt) = x.ln_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));
    assert_eq!(o_alt, o);

    if x >= 0u32 && x.is_finite() {
        assert!(ln < x);
    }

    if ln.is_normal() {
        assert_eq!(ln.get_prec(), Some(x.get_prec().unwrap()));
        if x > 1u32 && o > Less {
            assert!(ln > 0u32);
        } else if x < 1u32 && o < Greater {
            assert!(ln < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_ln, rug_o) = rug_ln_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln)),
            ComparableFloatRef(&ln),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.ln_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(ln.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.ln_round_ref(Exact));
    }
}

#[test]
fn ln_round_properties() {
    float_rounding_mode_pair_gen_var_34().test_properties(|(x, rm)| {
        ln_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_35().test_properties(|(x, rm)| {
        ln_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (ln, o) = Float::NAN.ln_round(rm);
        assert!(ln.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(Float::ZERO.ln_round(rm), (Float::NEGATIVE_INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_ZERO.ln_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
        assert_eq!(Float::INFINITY.ln_round(rm), (Float::INFINITY, Equal));
        let (ln, o) = Float::NEGATIVE_INFINITY.ln_round(rm);
        assert_eq!(ComparableFloat(ln), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(Float::ONE.ln_round(rm), (Float::ZERO, Equal));

        let (ln, o) = Float::NEGATIVE_ONE.ln_round(rm);
        assert_eq!(ComparableFloat(ln), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_properties_helper(x: Float) {
    let ln = x.clone().ln();
    assert!(ln.is_valid());

    let ln_alt = (&x).ln();
    assert!(ln_alt.is_valid());
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));

    let mut x_alt = x.clone();
    x_alt.ln_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln));

    let ln_alt = x.ln_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));
    let ln_alt = x.ln_prec_ref(x.significant_bits()).0;
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));

    let ln_alt = x.ln_round_ref(Nearest).0;
    assert_eq!(ComparableFloatRef(&ln_alt), ComparableFloatRef(&ln));

    if x >= 0u32 && x.is_finite() {
        assert!(ln < x);
    }

    let rug_ln = rug_ln(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_ln)),
        ComparableFloatRef(&ln),
    );
}

#[test]
fn ln_properties() {
    float_gen().test_properties(|x| {
        ln_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        ln_properties_helper(x);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_ln() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_ln(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(1.0, 0.0);
    test::<f32>(-1.0, f32::NAN);
    test::<f32>(0.5, -0.6931472);
    test::<f32>(-0.5, f32::NAN);
    test::<f32>(2.0, 0.6931472);
    test::<f32>(-2.0, f32::NAN);
    test::<f32>(core::f32::consts::PI, 1.14473);
    test::<f32>(-core::f32::consts::PI, f32::NAN);
    test::<f32>(core::f32::consts::E, 0.99999994);
    test::<f32>(-core::f32::consts::E, f32::NAN);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(1.0, 0.0);
    test::<f64>(-1.0, f64::NAN);
    test::<f64>(0.5, -0.6931471805599453);
    test::<f64>(-0.5, f64::NAN);
    test::<f64>(2.0, 0.6931471805599453);
    test::<f64>(-2.0, f64::NAN);
    test::<f64>(core::f64::consts::PI, 1.1447298858494002);
    test::<f64>(-core::f64::consts::PI, f64::NAN);
    test::<f64>(core::f64::consts::E, 1.0);
    test::<f64>(-core::f64::consts::E, f64::NAN);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_ln_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_ln(x);
    });
}

#[test]
fn primitive_float_ln_properties() {
    apply_fn_to_primitive_floats!(primitive_float_ln_properties_helper);
}
