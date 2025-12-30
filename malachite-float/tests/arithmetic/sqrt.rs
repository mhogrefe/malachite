// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::{f32, f64};
use malachite_base::num::arithmetic::traits::{PowerOf2, Sqrt, SqrtAssign, Square};
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
use malachite_float::arithmetic::sqrt::primitive_float_sqrt_rational;
use malachite_float::emulate_float_to_float_fn;
use malachite_float::test_util::arithmetic::sqrt::{
    rug_sqrt, rug_sqrt_prec, rug_sqrt_prec_round, rug_sqrt_round, sqrt_rational_prec_round_generic,
    sqrt_rational_prec_round_simple,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, test_constant, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_6, float_gen_var_7, float_gen_var_8, float_gen_var_11,
    float_gen_var_12, float_rounding_mode_pair_gen_var_24, float_rounding_mode_pair_gen_var_25,
    float_rounding_mode_pair_gen_var_26, float_rounding_mode_pair_gen_var_27,
    float_rounding_mode_pair_gen_var_28, float_rounding_mode_pair_gen_var_29,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_13, float_unsigned_rounding_mode_triple_gen_var_14,
    rational_unsigned_rounding_mode_triple_gen_var_3,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_gen, rational_pair_gen, rational_unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_sqrt() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let sqrt = x.clone().sqrt();
        assert!(sqrt.is_valid());

        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);

        let sqrt_alt = (&x).sqrt();
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));

        let mut sqrt_alt = x.clone();
        sqrt_alt.sqrt_assign();
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&sqrt)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    // - in sqrt_float_significand_ref
    // - in sqrt_float_significand_ref_helper
    // - in sqrt_float_significand_same_prec_ref
    // - 1 limb in sqrt_float_significand_same_prec_ref
    // - in sqrt_float_significand_same_prec_lt_w
    // - exp_u.odd() in sqrt_float_significand_same_prec_lt_w
    // - in limb_sqrt_approx
    // - in half_limb_sqrt_approx
    // - z <= 2 * (LIMIT - 1) in limb_sqrt_approx
    // - z > y + y in limb_sqrt_approx
    // - r0.wrapping_add(7) & (mask >> 1) <= 7 in sqrt_float_significand_same_prec_lt_w
    // - rb == 0 || (rb == 1 && sb <= r0.wrapping_mul(2)) in sqrt_float_significand_same_prec_lt_w
    // - sb == 0 in sqrt_float_significand_same_prec_lt_w
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "NaN", "NaN");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "1.0",
        "0x1.0000000000000000000000000#100",
    );
    test("-1.0", "-0x1.0000000000000000000000000#100", "NaN", "NaN");

    test("123.0", "0x7b.0#7", "11.1", "0xb.2#7");
    test("-123.0", "-0x7b.0#7", "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1.7724538509055159",
        "0x1.c5bf891b4ef6a#53",
    );
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "NaN", "NaN");

    test(
        "245015989.087862987938504839548125246171",
        "0xe9aa5b5.167e3053b9874c7a7e9ad42ec#126",
        "15652.9865868422371154837031363385510498",
        "0x3d24.fc90f48e1283b9ddd9ddcd80c651#126",
    );
    test(
        "8207.999969482421875000000208449716311754706238513360135037",
        "0x200f.fffe00000000000000fc000ffffffffe7fffffffe0000#191",
        "90.59801305482599751936121493336015116751576530601123201492",
        "0x5a.991762310f07e6c407b327fcf04aa587cf47462012c4a5#191",
    );
    test(
        "255.99998475609754677861929270691580954",
        "0xff.ffff003ff00000000003ffffffffc#122",
        "15.999999523628041245261756402527546248",
        "0xf.fffff801ff7e00ff9f10dd9f7ca998#122",
    );
    test(
        "0.9175812291850011518031983506835431746",
        "0x0.eae69a7ac5e74e0580d8afa7065e9b0#121",
        "0.9579046033843877252229074148114839997",
        "0x0.f5393c70394ab5224b7b609430e0038#121",
    );
}

#[test]
fn test_sqrt_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (sqrt, o) = x.clone().sqrt_prec(prec);
        assert!(sqrt.is_valid());

        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, o_out);

        let (sqrt_alt, o_alt) = x.sqrt_prec_ref(prec);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));
        assert_eq!(o_alt, o_out);

        let mut sqrt_alt = x.clone();
        let o_alt = sqrt_alt.sqrt_prec_assign(prec);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));
        assert_eq!(o_alt, o_out);

        let (rug_sqrt, rug_o) = rug_sqrt_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt)),
            ComparableFloatRef(&sqrt),
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

    test("123.0", "0x7b.0#7", 1, "8.0", "0x8.0#1", Less);
    test("123.0", "0x7b.0#7", 10, "11.09", "0xb.18#10", Greater);
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
        "1.771",
        "0x1.c58#10",
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
        "0.004",
        "0x0.01#1",
        159,
        "0.0625",
        "0x0.10000000000000000000000000000000000000000#159",
        Equal,
    );
    test(
        "3.977856078879580383473276114029204544685854243426e189",
        "0x3.9238e0e804fbb07c90c48c1a3f2717fe767aa541E+157#162",
        162,
        "6.307024717630001791804921737602144987170364743623e94",
        "0x7.8f1c619832e271db7d7c00000000000000000000E+78#162",
        Equal,
    );
    // - rm == Nearest && sticky1 == (1 << (sh - 1)) && !sqrt_inexact && sticky0 == 0 in
    //   sqrt_float_significands_general
    // - in even_rule
    // - sh < Limb::WIDTH in even_rule
    // - rp[0] & (1 << sh) != 0 in even_rule
    // - inexact != Less in even_rule
    test("2.2", "0x2.4#4", 1, "2.0", "0x2.0#1", Greater);
    // - rp[0] & (1 << sh) == 0 in even_rule
    // - inexact == Less in even_rule
    test("1.56", "0x1.9#5", 2, "1.0", "0x1.0#2", Less);
    // - u_size > rrsize in sqrt_float_significands_general
    // - u_size > rrsize && !odd_exp in sqrt_float_significands_general
    // - sticky0 == 0 && l != 0 in sqrt_float_significands_general
    test(
        "0.000199046277632504184666664672269768242929310652018203552191617720205649",
        "0x0.000d0b7140b8f3aea60aad60c1dc3b2ee0d83e2eba33dcfb6f874df52d78#225",
        26,
        "0.0141083761",
        "0x0.039c9b46#26",
        Less,
    );
    // - u_size <= rrsize && odd_exp && k == 0 in sqrt_float_significands_general
    test(
        "330.3297903358337046735382484655688",
        "0x14a.546d23b2f14f71a1f4c12ddaa88#115",
        45,
        "18.17497703811",
        "0x12.2ccb4b903c#45",
        Greater,
    );
    // - u_size > rrsize && odd_exp in sqrt_float_significands_general
    test(
        "4.377225292368646512419693467837359069662104001145e-12",
        "0x4.d01452b8a6a69b159aa9fe42a13c8852969ebbf2E-10#163",
        31,
        "2.092181946e-6",
        "0x0.00002319da608#31",
        Greater,
    );
    // - sticky0 != 0 || l == 0 in sqrt_float_significands_general
    test(
        "1.05073202367937779245019344912798895329031793654792205944648e-92",
        "0x5.7ab7ff36bcc0024c64f4507f522e91df687d9649e2e60984E-77#195",
        53,
        "1.0250522053434049e-46",
        "0x9.5cfc196264d98E-39#53",
        Greater,
    );
}

#[test]
fn sqrt_prec_fail() {
    assert_panic!(Float::NAN.sqrt_prec(0));
    assert_panic!(Float::NAN.sqrt_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.sqrt_prec_assign(0)
    });
}

#[test]
fn test_sqrt_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (sqrt, o) = x.clone().sqrt_round(rm);
        assert!(sqrt.is_valid());

        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, o_out);

        let (sqrt_alt, o_alt) = x.sqrt_round_ref(rm);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));
        assert_eq!(o_alt, o_out);

        let mut sqrt_alt = x.clone();
        let o_alt = sqrt_alt.sqrt_round_assign(rm);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt, rug_o) = rug_sqrt_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt)),
                ComparableFloatRef(&sqrt),
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

    test("0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Exact, "-0.0", "-0x0.0", Equal);

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

    test("123.0", "0x7b.0#7", Floor, "11.0", "0xb.0#7", Less);
    test("123.0", "0x7b.0#7", Ceiling, "11.1", "0xb.2#7", Greater);
    test("123.0", "0x7b.0#7", Down, "11.0", "0xb.0#7", Less);
    test("123.0", "0x7b.0#7", Up, "11.1", "0xb.2#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "11.1", "0xb.2#7", Greater);

    test("-123.0", "-0x7b.0#7", Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Ceiling, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Down, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Up, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Nearest, "NaN", "NaN", Equal);

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "1.7724538509055159",
        "0x1.c5bf891b4ef6a#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "1.7724538509055161",
        "0x1.c5bf891b4ef6b#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "1.7724538509055159",
        "0x1.c5bf891b4ef6a#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "1.7724538509055161",
        "0x1.c5bf891b4ef6b#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "1.7724538509055159",
        "0x1.c5bf891b4ef6a#53",
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

    // - exp_u.even() in sqrt_float_significand_same_prec_lt_w
    // - z <= y + y in limb_sqrt_approx
    // - r0.wrapping_add(7) & (mask >> 1) > 7 in sqrt_float_significand_same_prec_lt_w
    // - sb != 0 in sqrt_float_significand_same_prec_lt_w
    // - rm == Nearest in sqrt_float_significand_same_prec_lt_w
    // - rm == Nearest && rb == 0 in sqrt_float_significand_same_prec_lt_w
    test("2.0", "0x2.0#1", Nearest, "1.0", "0x1.0#1", Less);
    // - rm == Nearest && rb != 0 in sqrt_float_significand_same_prec_lt_w
    // - rm == Nearest && rb != 0 && r0 != 0 in sqrt_float_significand_same_prec_lt_w
    test("2.0", "0x2.0#2", Nearest, "1.5", "0x1.8#2", Greater);
    // - z > 2 * (LIMIT - 1) in limb_sqrt_approx
    test("3.5", "0x3.8#3", Nearest, "1.8", "0x1.c#3", Less);
    // - !(rb == 0 || (rb == 1 && sb <= r0.wrapping_mul(2))) in
    //   sqrt_float_significand_same_prec_lt_w
    test(
        "8.56684288039927757e26",
        "0x2.c4a1f44fb4eb72cE+22#60",
        Nearest,
        "29269169582342.57382",
        "0x1a9ec274b106.92e6#60",
        Greater,
    );
    // - in sqrt_float_significand_same_prec_w
    // - exp_u.odd() in sqrt_float_significand_same_prec_w
    // - rb == 0 || (rb == 1 && sb <= r0.wrapping_mul(2)) in sqrt_float_significand_same_prec_w
    // - sb == 0 in sqrt_float_significand_same_prec_w
    test(
        "1.0",
        "0x1.0000000000000000#64",
        Nearest,
        "1.0",
        "0x1.0000000000000000#64",
        Equal,
    );
    // - exp_u.even() in sqrt_float_significand_same_prec_w
    // - !(rb == 0 || (rb == 1 && sb <= r0.wrapping_mul(2))) in sqrt_float_significand_same_prec_w
    // - sb != 0 in sqrt_float_significand_same_prec_w
    // - rm == Nearest in sqrt_float_significand_same_prec_w
    // - rm == Nearest && rb == 0 in sqrt_float_significand_same_prec_w
    test(
        "2.0",
        "0x2.0000000000000000#64",
        Nearest,
        "1.4142135623730950488",
        "0x1.6a09e667f3bcc908#64",
        Less,
    );
    // - rm == Nearest && rb != 0 in sqrt_float_significand_same_prec_w
    // - rm == Nearest && rb != 0 && r0 != 0 in sqrt_float_significand_same_prec_w
    test(
        "1.0000000000000000002",
        "0x1.0000000000000004#64",
        Nearest,
        "1.0000000000000000001",
        "0x1.0000000000000002#64",
        Greater,
    );
    // - 2 limbs, prec != 2*w in sqrt_float_significand_same_prec_ref
    // - in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - exp_u.odd() in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - in limbs_2_sqrt_approx
    // - in limb_sqrt
    // - in limb_inverse_sqrt_approx
    // - !r.get_highest_bit() in limb_sqrt
    // - h < 16 in limb_sqrt
    // - h < 8 in limb_sqrt
    // - h < 4 in limb_sqrt
    // - h <= 1 && ((h != 1) || (l <= r.wrapping_mul(2))) in limb_sqrt
    // - h == 0 in limbs_2_sqrt_approx
    // - r0.wrapping_add(26) & (mask >> 1) <= 30 in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - SignedLimb::wrapping_from(t2) >= 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - t2 <= 1 && (t2 != 1 || t1 <= h) && (t2 != 1 || t1 != h || t0 <= l) in
    //   sqrt_float_significand_same_prec_gt_w_lt_2w
    // - sb == 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.0",
        "0x1.0000000000000000#65",
        Nearest,
        "1.0",
        "0x1.0000000000000000#65",
        Equal,
    );
    // - exp_u.even() in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - r.get_highest_bit() in limb_sqrt
    // - h >= 4 in limb_sqrt
    // - h > 1 || ((h == 1) && (l > r.wrapping_mul(2))) in limb_sqrt
    // - r0.wrapping_add(26) & (mask >> 1) > 30 in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - sb != 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest && rb != 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest && rb != 0 && r1 != 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    test(
        "2.0",
        "0x2.0000000000000000#65",
        Nearest,
        "1.41421356237309504882",
        "0x1.6a09e667f3bcc909#65",
        Greater,
    );
    // - t2 > 1 || (t2 == 1 && t1 > h) || (t2 == 1 && t1 == h && t0 > l) in
    //   sqrt_float_significand_same_prec_gt_w_lt_2w
    // - rm == Nearest && rb == 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    test(
        "1.00000000000000000005",
        "0x1.0000000000000001#65",
        Nearest,
        "1.0",
        "0x1.0000000000000000#65",
        Less,
    );
    // - h != 0 in limbs_2_sqrt_approx
    test(
        "2.00000000000000000125",
        "0x2.0000000000000017#66",
        Nearest,
        "1.41421356237309504925",
        "0x1.6a09e667f3bcc9110#66",
        Greater,
    );
    // - h >= 8 in limb_sqrt
    test(
        "1.44020837962004126031156726e28",
        "0x2.e891fdf020840728c0894E+23#85",
        Nearest,
        "120008682170084.730095556159",
        "0x6d25b2dee6e4.bae78ad8a8#85",
        Greater,
    );
    // - h >= 16 in limb_sqrt
    test(
        "14.8543972813270696412821",
        "0xe.dab9c7bd1750bad37350#83",
        Nearest,
        "3.8541402778475862892890363",
        "0x3.daa8efef8d3e9881bc6d0#83",
        Less,
    );
    // - default case in sqrt_float_significand_same_prec_ref
    // - in sqrt_float_significands_general
    // - sh != 0 || rm != Nearest in sqrt_float_significands_general
    // - u_size <= rrsize in sqrt_float_significands_general
    // - u_size <= rrsize || !odd_exp in sqrt_float_significands_general
    // - sticky0 in sqrt_float_significands_general
    // - rm == Nearest in sqrt_float_significands_general
    // - rm == Nearest && sh < Limb::WIDTH in sqrt_float_significands_general
    // - rm == Nearest && sticky1 & (1 << (sh - 1)) == 0 in sqrt_float_significands_general
    // - in truncate
    // - sh != Limb::WIDTH in truncate
    test(
        "531607.999405753398566100250398911805595276409254333486007034444758646220477451",
        "0x81c97.ffd90e3247f501afdb3e5781d0c650b8de694094110713b5a698d3d38550#257",
        Nearest,
        "729.11453106199536130370773365901724384208766502803825392984301547488034915749",
        "0x2d9.1d51e85da56f057ec9027291169bc84fb3262e9f7b56ad4fbded5d5fb28234#257",
        Less,
    );
    // - u_size <= rrsize && odd_exp in sqrt_float_significands_general
    // - u_size <= rrsize && odd_exp && k != 0 in sqrt_float_significands_general
    test(
        "8669478862124781.0453122498906357696603256",
        "0x1eccd857f6f6ed.0b999567637220ce5162#133",
        Nearest,
        "93110036.31255215575055410689733347856975",
        "0x58cbf14.50036b073e308b45123fad288c8#133",
        Less,
    );
    // - rm == Nearest && sticky1 & (1 << (sh - 1)) != 0 in sqrt_float_significands_general
    // - rm == Nearest && sticky1 == (1 << (sh - 1)) && !sqrt_inexact && sticky0 != 0 in
    //   sqrt_float_significands_general
    // - in add_one
    // - sh != Limb::WIDTH in add_one
    // - !carry in add_one
    test(
        "4.342868714761498751021923028996829056762200989107722850118926783525941596406956471866296\
        2992498076143086829355664e-20",
        "0xc.d160fab1cc524c2fe76cffa155f81b8188af00d7023e4d317d44fbce1d3d47bc1d1312f172ce15520ea1d\
        11590abE-17#372",
        Nearest,
        "2.083955065437232202468518924164364631048457537561905126352513612727408728510770037672930\
        6989930075738883830052488e-10",
        "0xe.5221ecf52b8695b0ee215b5d4b92348fce150af3190e08806ac4bb447fc0e236659b5b02162fde717b9e2\
        56afe2eE-9#372",
        Greater,
    );
    // - sh == 0 && rm == Nearest in sqrt_float_significands_general
    // - rm == Nearest && sh >= Limb::WIDTH in sqrt_float_significands_general
    // - rm == Nearest && sticky1.get_highest_bit() in sqrt_float_significands_general
    // - rm == Nearest && sticky1 == LIMB_HIGH_BIT && !sqrt_inexact && sticky0 != 0 in
    //   sqrt_float_significands_general
    // - sh == Limb::WIDTH in add_one
    test(
        "2.652265028059746721807174554033221706564e-11",
        "0x1.d29765de1f777af51db92558a3d9f542E-9#128",
        Nearest,
        "5.15001459032860092518337573994143253147e-6",
        "0x0.0000566724ecdf4dd7e9fadaf1a94e15741b0#128",
        Greater,
    );
    // - rm == Nearest && !sticky1.get_highest_bit() in sqrt_float_significands_general
    // - sh == Limb::WIDTH in truncate
    test(
        "4.131497698938604195099390193257480907988376605341754029892e72",
        "0x2.569dc44aae4efd9bf163a72fcf66facef9729212ebaf3d4cE+60#192",
        Nearest,
        "2032608594623815959008545804304448692.2928683624055859234906",
        "0x18777574b6d7a0f8c80efa20bdefcb4.4af96bc690a8425010#192",
        Less,
    );
    // - !sticky0 in sqrt_float_significands_general
    test(
        "3.9231885846166754773973683895047915100639721527900216e56",
        "0x1.00000000000000000000000000000000000000000000E+47#176",
        Nearest,
        "19807040628566084398385987584.0",
        "0x400000000000000000000000.000000000000000000000#176",
        Equal,
    );

    test(
        "too_big",
        "0x4.0E+268435455#1",
        Nearest,
        "too_big",
        "0x8.0E+134217727#1",
        Equal,
    );
    test(
        "too_big",
        "0x6.0E+268435455#2",
        Nearest,
        "too_big",
        "0x8.0E+134217727#2",
        Less,
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        Nearest,
        "too_small",
        "0x1.0E-134217728#1",
        Equal,
    );
    test(
        "too_small",
        "0x1.0E-268435456#2",
        Nearest,
        "too_small",
        "0x1.0E-134217728#2",
        Equal,
    );

    // - rm == Floor | Down in sqrt_float_significand_same_prec_lt_w
    test("2.0", "0x2.0#1", Down, "1.0", "0x1.0#1", Less);
    // - rm == Ceiling | Up in sqrt_float_significand_same_prec_lt_w
    // - rm == Ceiling | Up && r0 == 0 in sqrt_float_significand_same_prec_lt_w
    test("2.0", "0x2.0#1", Up, "2.0", "0x2.0#1", Greater);
    // - rm == Ceiling | Up && r0 != 0 in sqrt_float_significand_same_prec_lt_w
    test("2.0", "0x2.0#2", Up, "1.5", "0x1.8#2", Greater);
    // - rm == Floor | Down in sqrt_float_significand_same_prec_w
    test(
        "2.0",
        "0x2.0000000000000000#64",
        Down,
        "1.4142135623730950488",
        "0x1.6a09e667f3bcc908#64",
        Less,
    );
    // - rm == Ceiling | Up in sqrt_float_significand_same_prec_w
    // - r0 != 0 in sqrt_float_significand_same_prec_w
    test(
        "2.0",
        "0x2.0000000000000000#64",
        Up,
        "1.4142135623730950489",
        "0x1.6a09e667f3bcc90a#64",
        Greater,
    );
    // - r0 == 0 in sqrt_float_significand_same_prec_w
    test(
        "67108863.999999999996",
        "0x3ffffff.fffffffffc#64",
        Up,
        "8192.0",
        "0x2000.0000000000000#64",
        Greater,
    );
    // - rm == Floor | Down in sqrt_float_significand_same_prec_gt_w_lt_2w
    test(
        "2.0",
        "0x2.0000000000000000#65",
        Down,
        "1.41421356237309504876",
        "0x1.6a09e667f3bcc908#65",
        Less,
    );
    // - rm == Ceiling | Up in sqrt_float_significand_same_prec_gt_w_lt_2w
    // - rm == Ceiling | Up && r1 != 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    test(
        "2.0",
        "0x2.0000000000000000#65",
        Up,
        "1.41421356237309504882",
        "0x1.6a09e667f3bcc909#65",
        Greater,
    );
    // - rm == Ceiling | Up && r1 == 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    test(
        "7.00649232162408535461864791e-46",
        "0x3.fffffffffffffffffffffeE-38#89",
        Up,
        "2.646977960169688559588507815e-23",
        "0x2.0000000000000000000000E-19#89",
        Greater,
    );
    // - rm == Floor | Down in sqrt_float_significands_general
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        Down,
        "1.414213562373095048801688724209698078569",
        "0x1.6a09e667f3bcc908b2fb1366ea957d3e#129",
        Less,
    );
    // - rm == Ceiling | Up in sqrt_float_significands_general
    test(
        "2.0",
        "0x2.00000000000000000000000000000000#129",
        Up,
        "1.414213562373095048801688724209698078572",
        "0x1.6a09e667f3bcc908b2fb1366ea957d3f#129",
        Greater,
    );
    // - carry in add_one
    test(
        "0.999999999999999999999999999999999999999",
        "0x0.ffffffffffffffffffffffffffffffff8#129",
        Up,
        "1.0",
        "0x1.00000000000000000000000000000000#129",
        Greater,
    );
    // - SignedLimb::wrapping_from(t2) < 0 in sqrt_float_significand_same_prec_gt_w_lt_2w
    test(
        "3.1370587247788594938889277427222882863e58",
        "0x4.ff643687dfbb20bf81d2ef780dca280E+48#124",
        Nearest,
        "177117439140781941633284688407.1796741",
        "0x23c4c1c9f13433f0d1315ca17.2dff1f4#124",
        Less,
    );
}

#[test]
fn sqrt_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.sqrt_round(Exact));
    assert_panic!(THREE.sqrt_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.sqrt_round_assign(Exact);
    });
}

#[test]
fn test_sqrt_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (sqrt, o) = x.clone().sqrt_prec_round(prec, rm);
        assert!(sqrt.is_valid());

        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, o_out);

        let (sqrt_alt, o_alt) = x.sqrt_prec_round_ref(prec, rm);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));
        assert_eq!(o_alt, o_out);

        let mut sqrt_alt = x.clone();
        let o_alt = sqrt_alt.sqrt_prec_round_assign(prec, rm);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt), ComparableFloatRef(&sqrt_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_sqrt, rug_o) = rug_sqrt_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sqrt)),
                ComparableFloatRef(&sqrt),
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

    test("0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Exact, "-0.0", "-0x0.0", Equal);

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

    test("123.0", "0x7b.0#7", 1, Floor, "8.0", "0x8.0#1", Less);
    test(
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "2.0e1",
        "0x1.0E+1#1",
        Greater,
    );
    test("123.0", "0x7b.0#7", 1, Down, "8.0", "0x8.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Up, "2.0e1", "0x1.0E+1#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "8.0", "0x8.0#1", Less);

    test("123.0", "0x7b.0#7", 10, Floor, "11.08", "0xb.14#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "11.09",
        "0xb.18#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 10, Down, "11.08", "0xb.14#10", Less);
    test("123.0", "0x7b.0#7", 10, Up, "11.09", "0xb.18#10", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "11.09",
        "0xb.18#10",
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
        "1.771",
        "0x1.c58#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "1.773",
        "0x1.c60#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "1.771",
        "0x1.c58#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "1.773",
        "0x1.c60#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "1.771",
        "0x1.c58#10",
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
}

#[test]
fn sqrt_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).sqrt_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).sqrt_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.sqrt_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.sqrt_prec_round(1, Exact));
    assert_panic!(THREE.sqrt_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.sqrt_prec_round_assign(1, Exact)
    });
}

#[test]
fn test_sqrt_rational_prec() {
    let test = |s, prec, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (sqrt, o) = Float::sqrt_rational_prec(u.clone(), prec);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = Float::sqrt_rational_prec_ref(&u, prec);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_generic(&u, prec, Nearest);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_simple(&u, prec, Nearest);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        if sqrt.is_normal() {
            let square = Rational::exact_from(&sqrt).square();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square < u);
                    let mut next = sqrt.clone();
                    next.increment();
                    assert!(Rational::exact_from(&next).square() > u);
                }
                Greater => {
                    assert!(square > u);
                    let mut previous = sqrt.clone();
                    previous.decrement();
                    assert!(Rational::exact_from(&previous).square() < u);
                }
            }
        }
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 100, "0.0", "0x0.0", Equal);
    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0", "0x1.000#10", Equal);
    test("1", 100, "1.0", "0x1.0000000000000000000000000#100", Equal);
    test("1/2", 1, "0.5", "0x0.8#1", Less);
    test("1/2", 10, "0.707", "0x0.b50#10", Less);
    test(
        "1/2",
        100,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );
    test("1/3", 1, "0.5", "0x0.8#1", Less);
    test("1/3", 10, "0.577", "0x0.93c#10", Less);
    test(
        "1/3",
        100,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );
    test("22/7", 1, "2.0", "0x2.0#1", Greater);
    test("22/7", 10, "1.773", "0x1.c60#10", Greater);
    test(
        "22/7",
        100,
        "1.772810520855836656590463136493",
        "0x1.c5d6e909149e8e4e59f04b68e#100",
        Greater,
    );

    let test_big = |u: Rational, prec, out, out_hex, out_o| {
        let (sqrt, o) = Float::sqrt_rational_prec(u.clone(), prec);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = Float::sqrt_rational_prec_ref(&u, prec);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_generic(&u, prec, Nearest);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_simple(&u, prec, Nearest);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        if sqrt.is_normal() {
            let square = Rational::exact_from(&sqrt).square();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square < u);
                    let mut next = sqrt.clone();
                    next.increment();
                    assert!(Rational::exact_from(&next).square() > u);
                }
                Greater => {
                    assert!(square > u);
                    let mut previous = sqrt.clone();
                    previous.decrement();
                    assert!(Rational::exact_from(&previous).square() < u);
                }
            }
        }
    };
    test_big(
        Rational::power_of_2(1000i64),
        10,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        10,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        "too_small",
        "0x1.000E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
}

#[test]
fn sqrt_rational_prec_fail() {
    assert_panic!(Float::sqrt_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::sqrt_rational_prec(Rational::ONE, 0));
    assert_panic!(Float::sqrt_rational_prec(Rational::NEGATIVE_ONE, 0));
}

#[test]
fn sqrt_rational_prec_ref_fail() {
    assert_panic!(Float::sqrt_rational_prec_ref(&Rational::ZERO, 0));
    assert_panic!(Float::sqrt_rational_prec_ref(&Rational::ONE, 0));
    assert_panic!(Float::sqrt_rational_prec_ref(&Rational::NEGATIVE_ONE, 0));
}

#[test]
fn test_sqrt_rational_prec_round() {
    let test = |s, prec, rm, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (sqrt, o) = Float::sqrt_rational_prec_round(u.clone(), prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = Float::sqrt_rational_prec_round_ref(&u, prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_generic(&u, prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_simple(&u, prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        if sqrt.is_normal() {
            let square = Rational::exact_from(&sqrt).square();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square < u);
                    let mut next = sqrt.clone();
                    next.increment();
                    assert!(Rational::exact_from(&next).square() > u);
                }
                Greater => {
                    assert!(square > u);
                    let mut previous = sqrt.clone();
                    previous.decrement();
                    assert!(Rational::exact_from(&previous).square() < u);
                }
            }
        }
    };
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);

    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);

    test("0", 100, Floor, "0.0", "0x0.0", Equal);
    test("0", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 100, Down, "0.0", "0x0.0", Equal);
    test("0", 100, Up, "0.0", "0x0.0", Equal);
    test("0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("0", 100, Exact, "0.0", "0x0.0", Equal);

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

    test("1/2", 1, Floor, "0.5", "0x0.8#1", Less);
    test("1/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Down, "0.5", "0x0.8#1", Less);
    test("1/2", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Nearest, "0.5", "0x0.8#1", Less);

    test("1/2", 10, Floor, "0.707", "0x0.b50#10", Less);
    test("1/2", 10, Ceiling, "0.708", "0x0.b54#10", Greater);
    test("1/2", 10, Down, "0.707", "0x0.b50#10", Less);
    test("1/2", 10, Up, "0.708", "0x0.b54#10", Greater);
    test("1/2", 10, Nearest, "0.707", "0x0.b50#10", Less);

    test(
        "1/2",
        100,
        Floor,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.7071067811865475244008443621054",
        "0x0.b504f333f9de6484597d89b38#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Down,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );
    test(
        "1/2",
        100,
        Up,
        "0.7071067811865475244008443621054",
        "0x0.b504f333f9de6484597d89b38#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.7071067811865475244008443621046",
        "0x0.b504f333f9de6484597d89b37#100",
        Less,
    );

    test("1/3", 1, Floor, "0.5", "0x0.8#1", Less);
    test("1/3", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/3", 1, Down, "0.5", "0x0.8#1", Less);
    test("1/3", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1/3", 1, Nearest, "0.5", "0x0.8#1", Less);

    test("1/3", 10, Floor, "0.577", "0x0.93c#10", Less);
    test("1/3", 10, Ceiling, "0.578", "0x0.940#10", Greater);
    test("1/3", 10, Down, "0.577", "0x0.93c#10", Less);
    test("1/3", 10, Up, "0.578", "0x0.940#10", Greater);
    test("1/3", 10, Nearest, "0.577", "0x0.93c#10", Less);

    test(
        "1/3",
        100,
        Floor,
        "0.577350269189625764509148780501",
        "0x0.93cd3a2c8198e2690c7c0f257#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "0.577350269189625764509148780501",
        "0x0.93cd3a2c8198e2690c7c0f257#100",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.577350269189625764509148780502",
        "0x0.93cd3a2c8198e2690c7c0f258#100",
        Greater,
    );

    test("22/7", 1, Floor, "1.0", "0x1.0#1", Less);
    test("22/7", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("22/7", 1, Down, "1.0", "0x1.0#1", Less);
    test("22/7", 1, Up, "2.0", "0x2.0#1", Greater);
    test("22/7", 1, Nearest, "2.0", "0x2.0#1", Greater);

    test("22/7", 10, Floor, "1.771", "0x1.c58#10", Less);
    test("22/7", 10, Ceiling, "1.773", "0x1.c60#10", Greater);
    test("22/7", 10, Down, "1.771", "0x1.c58#10", Less);
    test("22/7", 10, Up, "1.773", "0x1.c60#10", Greater);
    test("22/7", 10, Nearest, "1.773", "0x1.c60#10", Greater);

    test(
        "22/7",
        100,
        Floor,
        "1.772810520855836656590463136491",
        "0x1.c5d6e909149e8e4e59f04b68c#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "1.772810520855836656590463136493",
        "0x1.c5d6e909149e8e4e59f04b68e#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Down,
        "1.772810520855836656590463136491",
        "0x1.c5d6e909149e8e4e59f04b68c#100",
        Less,
    );
    test(
        "22/7",
        100,
        Up,
        "1.772810520855836656590463136493",
        "0x1.c5d6e909149e8e4e59f04b68e#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "1.772810520855836656590463136493",
        "0x1.c5d6e909149e8e4e59f04b68e#100",
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
        let (sqrt, o) = Float::sqrt_rational_prec_round(u.clone(), prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = Float::sqrt_rational_prec_round_ref(&u, prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_generic(&u, prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        let (sqrt, o) = sqrt_rational_prec_round_simple(&u, prec, rm);
        assert!(sqrt.is_valid());
        assert_eq!(sqrt.to_string(), out);
        assert_eq!(to_hex_string(&sqrt), out_hex);
        assert_eq!(o, out_o);

        if sqrt.is_normal() {
            let square = Rational::exact_from(&sqrt).square();
            match o {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square < u);
                    let mut next = sqrt.clone();
                    next.increment();
                    assert!(Rational::exact_from(&next).square() > u);
                }
                Greater => {
                    assert!(square > u);
                    let mut previous = sqrt.clone();
                    previous.decrement();
                    assert!(Rational::exact_from(&previous).square() < u);
                }
            }
        }
    };
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Floor,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Ceiling,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Down,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Up,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Nearest,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(1000i64),
        10,
        Exact,
        "3.273e150",
        "0x1.000E+125#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Floor,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Ceiling,
        "too_big",
        "0xb.54E+134217727#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Down,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Up,
        "too_big",
        "0xb.54E+134217727#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)),
        10,
        Nearest,
        "too_big",
        "0xb.50E+134217727#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Floor,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Ceiling,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Down,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Up,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Nearest,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1),
        10,
        Exact,
        "too_big",
        "0x8.00E+134217727#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Floor,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Ceiling,
        "too_big",
        "0x1.0E+134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Down,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Up,
        "too_big",
        "0x1.0E+134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1) * Rational::from_unsigneds(3u8, 2),
        1,
        Nearest,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Floor,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Ceiling,
        "too_big",
        "0x1.0E+134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Down,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Up,
        "too_big",
        "0x1.0E+134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(300u16, 199),
        1,
        Nearest,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Floor,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Ceiling,
        "too_big",
        "0x1.0E+134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Down,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Up,
        "too_big",
        "0x1.0E+134217728#1",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT) - 1)
            * Rational::from_unsigneds(299u16, 200),
        1,
        Nearest,
        "too_big",
        "0x8.0E+134217727#1",
        Less,
    );

    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Floor,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Ceiling,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Down,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Up,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(-1000i64),
        10,
        Exact,
        "3.055e-151",
        "0x1.000E-125#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Floor,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Ceiling,
        "too_small",
        "0x1.6a8E-134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Down,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Up,
        "too_small",
        "0x1.6a8E-134217728#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT)),
        10,
        Nearest,
        "too_small",
        "0x1.6a0E-134217728#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Floor,
        "too_small",
        "0x1.000E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Ceiling,
        "too_small",
        "0x1.000E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Down,
        "too_small",
        "0x1.000E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Up,
        "too_small",
        "0x1.000E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Nearest,
        "too_small",
        "0x1.000E-134217728#10",
        Equal,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1),
        10,
        Exact,
        "too_small",
        "0x1.000E-134217728#10",
        Equal,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Floor,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Ceiling,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Down,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Up,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2),
        10,
        Nearest,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Floor,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Ceiling,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Down,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Up,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1024u16, 1023),
        10,
        Nearest,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Floor,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Ceiling,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Down,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Up,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1001u16, 1000),
        10,
        Nearest,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );

    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Floor,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Ceiling,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Down,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Up,
        "too_small",
        "0xb.54E-134217729#10",
        Greater,
    );
    test_big(
        Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2)
            * Rational::from_unsigneds(1025u16, 1024),
        10,
        Nearest,
        "too_small",
        "0xb.50E-134217729#10",
        Less,
    );
}

#[test]
fn sqrt_rational_prec_round_fail() {
    assert_panic!(Float::sqrt_rational_prec_round(Rational::ZERO, 0, Floor));
    assert_panic!(Float::sqrt_rational_prec_round(Rational::ONE, 0, Floor));
    assert_panic!(Float::sqrt_rational_prec_round(
        Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::sqrt_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::sqrt_rational_prec_round(
        Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
}

#[test]
fn sqrt_rational_prec_round_ref_fail() {
    assert_panic!(Float::sqrt_rational_prec_round_ref(
        &Rational::ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::sqrt_rational_prec_round_ref(
        &Rational::ONE,
        0,
        Floor
    ));
    assert_panic!(Float::sqrt_rational_prec_round_ref(
        &Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::sqrt_rational_prec_round_ref(
        &Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::sqrt_rational_prec_round_ref(
        &Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sqrt_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
        Rational: ExactFrom<T>,
    {
        let u = Rational::from_str(s).unwrap();

        let sqrt = primitive_float_sqrt_rational::<T>(&u);
        assert_eq!(NiceFloat(sqrt), NiceFloat(out));
        if sqrt.is_normal() {
            let square = Rational::exact_from(sqrt).square();
            match square.cmp(&u) {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square < u);
                    let mut next = sqrt;
                    next = next.next_higher();
                    assert!(Rational::exact_from(next).square() > u);
                }
                Greater => {
                    assert!(square > u);
                    let mut previous = sqrt;
                    previous = previous.next_lower();
                    assert!(Rational::exact_from(previous).square() < u);
                }
            }
        }
    }
    test::<f32>("0", 0.0);
    test::<f32>("1", 1.0);
    test::<f32>("1/2", 0.70710677);
    test::<f32>("1/3", 0.57735026);
    test::<f32>("22/7", 1.7728106);
    test::<f32>("225", 15.0);
    test::<f32>("-1", f32::NAN);
    test::<f32>("-1/2", f32::NAN);
    test::<f32>("-1/3", f32::NAN);
    test::<f32>("-22/7", f32::NAN);

    test::<f64>("0", 0.0);
    test::<f64>("1", 1.0);
    test::<f64>("1/2", f64::consts::FRAC_1_SQRT_2);
    test::<f64>("1/3", 0.5773502691896257);
    test::<f64>("22/7", 1.7728105208558367);
    test::<f64>("225", 15.0);
    test::<f64>("-1", f64::NAN);
    test::<f64>("-1/2", f64::NAN);
    test::<f64>("-1/3", f64::NAN);
    test::<f64>("-22/7", f64::NAN);

    #[allow(clippy::needless_pass_by_value)]
    fn test_big<T: PrimitiveFloat>(u: Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
        Rational: ExactFrom<T>,
    {
        let sqrt = primitive_float_sqrt_rational::<T>(&u);
        assert_eq!(NiceFloat(sqrt), NiceFloat(out));
        if sqrt.is_normal() {
            let square = Rational::exact_from(sqrt).square();
            match square.cmp(&u) {
                Equal => assert_eq!(square, u),
                Less => {
                    assert!(square < u);
                    let mut next = sqrt;
                    next = next.next_higher();
                    assert!(Rational::exact_from(next).square() > u);
                }
                Greater => {
                    assert!(square > u);
                    let mut previous = sqrt;
                    previous = previous.next_lower();
                    assert!(Rational::exact_from(previous).square() < u);
                }
            }
        }
    }
    test_big::<f32>(Rational::power_of_2(1000i64), f32::INFINITY);
    test_big::<f32>(Rational::power_of_2(-1000i64), 0.0);
    test_big::<f32>(Rational::power_of_2(-290i64), 2.2e-44);
    test_big::<f32>(Rational::power_of_2(-200i64), 7.888609e-31);

    test_big::<f64>(Rational::power_of_2(10000i64), f64::INFINITY);
    test_big::<f64>(Rational::power_of_2(1000i64), 3.273390607896142e150);
    test_big::<f64>(Rational::power_of_2(-10000i64), 0.0);
    test_big::<f64>(Rational::power_of_2(-2100i64), 8.289046e-317);
    test_big::<f64>(Rational::power_of_2(-1000i64), 3.054936363499605e-151);
}

#[allow(clippy::needless_pass_by_value)]
fn sqrt_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode, extreme: bool) {
    let (sqrt, o) = x.clone().sqrt_prec_round(prec, rm);
    assert!(sqrt.is_valid());

    let (sqrt_alt, o_alt) = x.clone().sqrt_prec_round_ref(prec, rm);
    assert!(sqrt_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.sqrt_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sqrt));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sqrt, rug_o) = rug_sqrt_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt)),
            ComparableFloatRef(&sqrt),
        );
        assert_eq!(rug_o, o);
    }

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(sqrt.is_sign_positive());
    }

    if sqrt.is_normal() {
        assert_eq!(sqrt.get_prec(), Some(prec));
        if x > 1u32 && o < Greater {
            assert!(sqrt < x);
        } else if x < 1u32 && o > Less {
            assert!(sqrt > x);
        }
    }

    if !extreme && x.is_normal() && sqrt.is_normal() {
        let square = Rational::exact_from(&sqrt).square();
        match o {
            Equal => assert_eq!(square, x),
            Less => {
                assert!(square < x);
                let mut next = sqrt.clone();
                next.increment();
                assert!(Rational::exact_from(&next).square() > x);
            }
            Greater => {
                assert!(square > x);
                let mut previous = sqrt.clone();
                previous.decrement();
                assert!(Rational::exact_from(&previous).square() < x);
            }
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.sqrt_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(sqrt.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.sqrt_prec_round_ref(prec, Exact));
    }
}

#[test]
fn sqrt_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_13().test_properties(|(x, prec, rm)| {
        sqrt_prec_round_properties_helper(x, prec, rm, false);
    });

    float_unsigned_rounding_mode_triple_gen_var_14().test_properties(|(x, prec, rm)| {
        sqrt_prec_round_properties_helper(x, prec, rm, true);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (product, o) = Float::NAN.sqrt_prec_round(prec, rm);
        assert!(product.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.sqrt_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (s, o) = Float::NEGATIVE_INFINITY.sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ONE.sqrt_prec_round(prec, rm),
            (Float::one_prec(prec), Equal)
        );

        let (s, o) = Float::NEGATIVE_ONE.sqrt_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sqrt_prec_properties_helper(x: Float, prec: u64, extreme: bool) {
    let (sqrt, o) = x.clone().sqrt_prec(prec);
    assert!(sqrt.is_valid());

    let (sqrt_alt, o_alt) = x.sqrt_prec_ref(prec);
    assert!(sqrt_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.sqrt_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sqrt));
    assert_eq!(o_alt, o);

    let (rug_sqrt, rug_o) = rug_sqrt_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sqrt)),
        ComparableFloatRef(&sqrt),
    );
    assert_eq!(rug_o, o);

    let (sqrt_alt, o_alt) = x.sqrt_prec_round_ref(prec, Nearest);
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
    assert_eq!(o_alt, o);

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(sqrt.is_sign_positive());
    }

    if sqrt.is_normal() {
        assert_eq!(sqrt.get_prec(), Some(prec));
        if x > 1u32 && o < Greater {
            assert!(sqrt < x);
        } else if x < 1u32 && o > Less {
            assert!(sqrt > x);
        }
    }

    if !extreme && x.is_normal() && sqrt.is_normal() {
        let square = Rational::exact_from(&sqrt).square();
        match o {
            Equal => assert_eq!(square, x),
            Less => {
                assert!(square < x);
                let mut next = sqrt.clone();
                next.increment();
                assert!(Rational::exact_from(&next).square() > x);
            }
            Greater => {
                assert!(square > x);
                let mut previous = sqrt.clone();
                previous.decrement();
                assert!(Rational::exact_from(&previous).square() < x);
            }
        }
    }
}

#[test]
fn sqrt_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        sqrt_prec_properties_helper(x, prec, false);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        sqrt_prec_properties_helper(x, prec, false);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        sqrt_prec_properties_helper(x, prec, true);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (sqrt, o) = Float::NAN.sqrt_prec(prec);
        assert!(sqrt.is_nan());
        assert_eq!(o, Equal);

        let (sqrt, o) = Float::ZERO.sqrt_prec(prec);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (sqrt, o) = Float::NEGATIVE_ZERO.sqrt_prec(prec);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(Float::INFINITY.sqrt_prec(prec), (Float::INFINITY, Equal));

        let (sqrt, o) = Float::NEGATIVE_INFINITY.sqrt_prec(prec);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(Float::ONE.sqrt_prec(prec), (Float::one_prec(prec), Equal));

        let (sqrt, o) = Float::NEGATIVE_ONE.sqrt_prec(prec);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sqrt_round_properties_helper(x: Float, rm: RoundingMode, extreme: bool) {
    let (sqrt, o) = x.clone().sqrt_round(rm);
    assert!(sqrt.is_valid());

    let (sqrt_alt, o_alt) = x.sqrt_round_ref(rm);
    assert!(sqrt_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));

    let mut x_alt = x.clone();
    let o_alt = x_alt.sqrt_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sqrt));
    assert_eq!(o_alt, o);

    let (sqrt_alt, o_alt) = x.sqrt_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
    assert_eq!(o_alt, o);

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(sqrt.is_sign_positive());
    }

    if sqrt.is_normal() {
        assert_eq!(sqrt.get_prec(), Some(x.get_prec().unwrap()));
        if x > 1u32 && o < Greater {
            assert!(sqrt < x);
        } else if x < 1u32 && o > Less {
            assert!(sqrt > x);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_sqrt, rug_o) = rug_sqrt_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sqrt)),
            ComparableFloatRef(&sqrt),
        );
        assert_eq!(rug_o, o);
    }

    if !extreme && x.is_normal() && sqrt.is_normal() {
        let square = Rational::exact_from(&sqrt).square();
        match o {
            Equal => assert_eq!(square, x),
            Less => {
                assert!(square < x);
                let mut next = sqrt.clone();
                next.increment();
                assert!(Rational::exact_from(&next).square() > x);
            }
            Greater => {
                assert!(square > x);
                let mut previous = sqrt.clone();
                previous.decrement();
                assert!(Rational::exact_from(&previous).square() < x);
            }
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.sqrt_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(sqrt.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.sqrt_round_ref(Exact));
    }
}

#[test]
fn sqrt_round_properties() {
    float_rounding_mode_pair_gen_var_24().test_properties(|(x, rm)| {
        sqrt_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_25().test_properties(|(x, rm)| {
        sqrt_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_26().test_properties(|(x, rm)| {
        sqrt_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_27().test_properties(|(x, rm)| {
        sqrt_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_28().test_properties(|(x, rm)| {
        sqrt_round_properties_helper(x, rm, false);
    });

    float_rounding_mode_pair_gen_var_29().test_properties(|(x, rm)| {
        sqrt_round_properties_helper(x, rm, true);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (sqrt, o) = Float::NAN.sqrt_round(rm);
        assert!(sqrt.is_nan());
        assert_eq!(o, Equal);

        let (sqrt, o) = Float::ZERO.sqrt_round(rm);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (sqrt, o) = Float::NEGATIVE_ZERO.sqrt_round(rm);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(Float::INFINITY.sqrt_round(rm), (Float::INFINITY, Equal));

        let (sqrt, o) = Float::NEGATIVE_INFINITY.sqrt_round(rm);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(Float::ONE.sqrt_round(rm), (Float::ONE, Equal));

        let (sqrt, o) = Float::NEGATIVE_ONE.sqrt_round(rm);
        assert_eq!(ComparableFloat(sqrt), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sqrt_properties_helper_1(x: Float, extreme: bool) {
    let sqrt = x.clone().sqrt();
    assert!(sqrt.is_valid());

    let sqrt_alt = (&x).sqrt();
    assert!(sqrt_alt.is_valid());
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));

    let mut x_alt = x.clone();
    x_alt.sqrt_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&sqrt));

    let sqrt_alt = x.sqrt_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
    let sqrt_alt = x.sqrt_prec_ref(x.significant_bits()).0;
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));

    let sqrt_alt = x.sqrt_round_ref(Nearest).0;
    assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));

    if x >= 0u32 && !x.is_negative_zero() {
        assert!(sqrt.is_sign_positive());
    }

    if !extreme && x.is_normal() && sqrt.is_normal() {
        assert_eq!(sqrt.get_prec(), Some(x.get_prec().unwrap()));
        let square = Rational::exact_from(&sqrt).square();
        if square < x {
            let mut next = sqrt.clone();
            next.increment();
            assert!(Rational::exact_from(&next).square() > x);
        } else if square > x {
            let mut previous = sqrt.clone();
            previous.decrement();
            assert!(Rational::exact_from(&previous).square() < x);
        }
    }

    let rug_sqrt = rug_sqrt(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_sqrt)),
        ComparableFloatRef(&sqrt),
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn sqrt_properties_helper_2<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let sqrt_1 = x.sqrt();
        let sqrt_2 = emulate_float_to_float_fn(Float::sqrt_prec, x);
        assert_eq!(NiceFloat(sqrt_1), NiceFloat(sqrt_2));
    });
}

#[test]
fn sqrt_properties() {
    float_gen().test_properties(|x| {
        sqrt_properties_helper_1(x, false);
    });

    float_gen_var_6().test_properties(|x| {
        sqrt_properties_helper_1(x, false);
    });

    float_gen_var_7().test_properties(|x| {
        sqrt_properties_helper_1(x, false);
    });

    float_gen_var_8().test_properties(|x| {
        sqrt_properties_helper_1(x, false);
    });

    float_gen_var_11().test_properties(|x| {
        sqrt_properties_helper_1(x, false);
    });

    float_gen_var_12().test_properties(|x| {
        sqrt_properties_helper_1(x, true);
    });

    apply_fn_to_primitive_floats!(sqrt_properties_helper_2);
}

#[test]
fn sqrt_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (sqrt, o) = Float::sqrt_rational_prec(x.clone(), prec);
        assert!(sqrt.is_valid());

        let (sqrt_alt, o_alt) = Float::sqrt_rational_prec_ref(&x, prec);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
        assert_eq!(o, o_alt);

        let (sqrt_alt, o_alt) = Float::sqrt_rational_prec_round_ref(&x, prec, Nearest);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
        assert_eq!(o, o_alt);

        let (sqrt_alt, o_alt) = sqrt_rational_prec_round_generic(&x, prec, Nearest);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
        assert_eq!(o, o_alt);

        let (sqrt_alt, o_alt) = sqrt_rational_prec_round_simple(&x, prec, Nearest);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
        assert_eq!(o, o_alt);

        if !sqrt.is_nan() {
            assert_eq!(sqrt.get_prec(), if x == 0u32 { None } else { Some(prec) });
        }

        if x >= 0u32 {
            assert!(sqrt.is_sign_positive());
        }

        if sqrt.is_normal() {
            if x > 1u32 && o < Greater {
                assert!(sqrt < x);
            } else if x < 1u32 && o > Less {
                assert!(sqrt > x);
            }

            let square = Rational::exact_from(&sqrt).square();
            match o {
                Equal => assert_eq!(square, x),
                Less => {
                    assert!(square < x);
                    let mut next = sqrt.clone();
                    next.increment();
                    assert!(Rational::exact_from(&next).square() > x);
                }
                Greater => {
                    assert!(square > x);
                    let mut previous = sqrt.clone();
                    previous.decrement();
                    assert!(Rational::exact_from(&previous).square() < x);
                }
            }
        }
    });
}

#[test]
fn sqrt_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_3().test_properties(|(x, prec, rm)| {
        let (sqrt, o) = Float::sqrt_rational_prec_round(x.clone(), prec, rm);
        assert!(sqrt.is_valid());

        let (sqrt_alt, o_alt) = Float::sqrt_rational_prec_round_ref(&x, prec, rm);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
        assert_eq!(o, o_alt);

        if !sqrt.is_nan() {
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

        let (sqrt_alt, o_alt) = sqrt_rational_prec_round_generic(&x, prec, rm);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
        assert_eq!(o, o_alt);

        let (sqrt_alt, o_alt) = sqrt_rational_prec_round_simple(&x, prec, rm);
        assert!(sqrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&sqrt_alt), ComparableFloatRef(&sqrt));
        assert_eq!(o, o_alt);

        if !sqrt.is_nan() {
            assert_eq!(sqrt.get_prec(), if x == 0u32 { None } else { Some(prec) });
        }

        if x >= 0u32 {
            assert!(sqrt.is_sign_positive());
        }

        if sqrt.is_normal() {
            if x > 1u32 && o < Greater {
                assert!(sqrt < x);
            } else if x < 1u32 && o > Less {
                assert!(sqrt > x);
            }
        }

        if sqrt.is_normal() {
            let square = Rational::exact_from(&sqrt).square();
            match o {
                Equal => assert_eq!(square, x),
                Less => {
                    assert!(square < x);
                    let mut next = sqrt.clone();
                    next.increment();
                    assert!(Rational::exact_from(&next).square() > x);
                }
                Greater => {
                    assert!(square > x);
                    let mut previous = sqrt.clone();
                    previous.decrement();
                    assert!(Rational::exact_from(&previous).square() < x);
                }
            }
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = Float::sqrt_rational_prec_round_ref(&x, prec, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero_ref()),
                    ComparableFloat(sqrt.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(Float::sqrt_rational_prec_round_ref(&x, prec, Exact));
        }
    });

    // sqrt(3/5) is one of the simplest cases that doesn't reduce to sqrt or reciprocal_sqrt of a
    // Float
    const X: Rational = Rational::const_from_unsigneds(3, 5);
    test_constant(
        |prec, rm| Float::sqrt_rational_prec_round(X, prec, rm),
        10000,
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sqrt_rational_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    Rational: ExactFrom<T>,
{
    rational_gen().test_properties(|x| {
        let sqrt = primitive_float_sqrt_rational::<T>(&x);
        if sqrt.is_normal() {
            let square = Rational::exact_from(sqrt).square();
            match square.cmp(&x) {
                Less => {
                    assert!(Rational::exact_from(sqrt.next_higher()).square() > x);
                }
                Greater => {
                    assert!(Rational::exact_from(sqrt.next_lower()).square() < x);
                }
                _ => {}
            }
        }
    });

    rational_pair_gen().test_properties(|(x, y)| {
        let sqrt_x = NiceFloat(primitive_float_sqrt_rational::<T>(&x));
        let sqrt_y = NiceFloat(primitive_float_sqrt_rational::<T>(&y));
        if !sqrt_x.0.is_nan() && !sqrt_y.0.is_nan() {
            match x.partial_cmp(&y).unwrap() {
                Equal => assert_eq!(sqrt_x, sqrt_y),
                Less => assert!(sqrt_x <= sqrt_y),
                Greater => assert!(sqrt_x >= sqrt_y),
            }
        }
    });
}

#[test]
fn primitive_float_sqrt_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sqrt_rational_helper);
}
