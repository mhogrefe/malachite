// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Ln1PlusX, Ln1PlusXAssign};
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
use malachite_float::test_util::arithmetic::ln_1_plus_x::{
    rug_ln_1_plus_x, rug_ln_1_plus_x_prec, rug_ln_1_plus_x_prec_round, rug_ln_1_plus_x_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_36,
    float_rounding_mode_pair_gen_var_37, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_21,
    float_unsigned_rounding_mode_triple_gen_var_22,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use std::panic::catch_unwind;

#[test]
fn test_ln_1_plus_x() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let ln_1_plus_x = x.clone().ln_1_plus_x();
        assert!(ln_1_plus_x.is_valid());

        assert_eq!(ln_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&ln_1_plus_x), out_hex);

        let ln_1_plus_x_alt = (&x).ln_1_plus_x();
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );

        let mut ln_1_plus_x_alt = x.clone();
        ln_1_plus_x_alt.ln_1_plus_x_assign();
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_1_plus_x(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&ln_1_plus_x)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "0.5", "0x0.8#1");
    test("-1.0", "-0x1.0#1", "-Infinity", "-Infinity");
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        "-Infinity",
        "-Infinity",
    );

    test("123.0", "0x7b.0#7", "4.81", "0x4.d#7");
    test("-123.0", "-0x7b.0#7", "NaN", "NaN");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1.4210804127942926",
        "0x1.6bcbed09f00af#53",
    );
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "NaN", "NaN");

    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#99",
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "1.3132616875182228",
        "0x1.5031eafefb049#53",
    );
}

#[test]
fn test_ln_1_plus_x_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln_1_plus_x, o) = x.clone().ln_1_plus_x_prec(prec);
        assert!(ln_1_plus_x.is_valid());

        assert_eq!(ln_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&ln_1_plus_x), out_hex);
        assert_eq!(o, o_out);

        let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_prec_ref(prec);
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut ln_1_plus_x_alt = x.clone();
        let o_alt = ln_1_plus_x_alt.ln_1_plus_x_prec_assign(prec);
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let (rug_ln_1_plus_x, rug_o) = rug_ln_1_plus_x_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_1_plus_x)),
            ComparableFloatRef(&ln_1_plus_x),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, "0.5", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 10, "0.693", "0x0.b18#10", Greater);
    test("-1.0", "-0x1.0#1", 1, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", 10, "-Infinity", "-Infinity", Equal);
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 1, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", 10, "-Infinity", "-Infinity", Equal);

    test("123.0", "0x7b.0#7", 1, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 10, "4.82", "0x4.d2#10", Greater);
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
        "1.422",
        "0x1.6c0#10",
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
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        "0.693",
        "0x0.b18#10",
        Greater,
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
        "1.312",
        "0x1.500#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1000,
        "1.313261687518222795169562508889857360621371355719130743172227259928871207082405798391370\
        508801122188235112354330056995527817378492572489096514687095608366826205003730166034200738\
        084189699894265273702584124547429220713814861424622733790660643349661294258971286655158543\
        3239575551238146919559360534948518",
        "0x1.5031eafefb048f02e5a5df3619fc5fb5c6037bf7fdebbc4435349c9666bff79b288c0ff04c9c044ad557c\
        470c2ab58e09643de97450ffa4737f29614ff4e126f945a0da5df1c44d4c4dad8421b30ae457c32150c5881e55\
        0deaac3c641fd4f05c883f2fb8b7e6591d6d8dbce071bb345ccde22283553e5e4fff2cdf406#1000",
        Less,
    );
    test("2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Less);
    test("0.999998", "0x0.ffffe#19", 5, "0.69", "0x0.b0#5", Less);
}

#[test]
fn ln_1_plus_x_prec_fail() {
    assert_panic!(Float::NAN.ln_1_plus_x_prec(0));
    assert_panic!(Float::NAN.ln_1_plus_x_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.ln_1_plus_x_prec_assign(0)
    });
}

#[test]
fn test_ln_1_plus_x_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln_1_plus_x, o) = x.clone().ln_1_plus_x_round(rm);
        assert!(ln_1_plus_x.is_valid());

        assert_eq!(ln_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&ln_1_plus_x), out_hex);
        assert_eq!(o, o_out);

        let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_round_ref(rm);
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut ln_1_plus_x_alt = x.clone();
        let o_alt = ln_1_plus_x_alt.ln_1_plus_x_round_assign(rm);
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_ln_1_plus_x, rug_o) = rug_ln_1_plus_x_round(&rug::Float::exact_from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_ln_1_plus_x)),
                ComparableFloatRef(&ln_1_plus_x),
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

    test("1.0", "0x1.0#1", Floor, "0.5", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", Down, "0.5", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", Nearest, "0.5", "0x0.8#1", Less);

    test("-1.0", "-0x1.0#1", Floor, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", Down, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", Up, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", Exact, "-Infinity", "-Infinity", Equal);

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Floor,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Ceiling,
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Down,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Up,
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        Nearest,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#100",
        Less,
    );

    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("123.0", "0x7b.0#7", Floor, "4.81", "0x4.d#7", Less);
    test("123.0", "0x7b.0#7", Ceiling, "4.88", "0x4.e#7", Greater);
    test("123.0", "0x7b.0#7", Down, "4.81", "0x4.d#7", Less);
    test("123.0", "0x7b.0#7", Up, "4.88", "0x4.e#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "4.81", "0x4.d#7", Less);

    test("-123.0", "-0x7b.0#7", Floor, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Ceiling, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Down, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Up, "NaN", "NaN", Equal);
    test("-123.0", "-0x7b.0#7", Nearest, "NaN", "NaN", Equal);

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "1.4210804127942924",
        "0x1.6bcbed09f00ae#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "1.4210804127942926",
        "0x1.6bcbed09f00af#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "1.4210804127942924",
        "0x1.6bcbed09f00ae#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "1.4210804127942926",
        "0x1.6bcbed09f00af#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "1.4210804127942926",
        "0x1.6bcbed09f00af#53",
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
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "too_small",
        "0x1.0E-268435456#2",
        Nearest,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );

    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Floor,
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Ceiling,
        "0.6931471805599453094172321214596",
        "0x0.b17217f7d1cf79abc9e3b3982#100",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Down,
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Up,
        "0.6931471805599453094172321214596",
        "0x0.b17217f7d1cf79abc9e3b3982#100",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        Nearest,
        "0.693147180559945309417232121459",
        "0x0.b17217f7d1cf79abc9e3b3981#100",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Floor,
        "0.693147180559945309417232121456",
        "0x0.b17217f7d1cf79abc9e3b397e#99",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Ceiling,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#99",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Down,
        "0.693147180559945309417232121456",
        "0x0.b17217f7d1cf79abc9e3b397e#99",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Up,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#99",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        Nearest,
        "0.693147180559945309417232121458",
        "0x0.b17217f7d1cf79abc9e3b3980#99",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "1.3132616875182226",
        "0x1.5031eafefb048#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "1.3132616875182228",
        "0x1.5031eafefb049#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "1.3132616875182226",
        "0x1.5031eafefb048#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "1.3132616875182228",
        "0x1.5031eafefb049#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "1.3132616875182228",
        "0x1.5031eafefb049#53",
        Greater,
    );
}

#[test]
fn ln_1_plus_x_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.ln_1_plus_x_round(Exact));
    assert_panic!(THREE.ln_1_plus_x_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.ln_1_plus_x_round_assign(Exact);
    });
}

#[test]
fn test_ln_1_plus_x_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (ln_1_plus_x, o) = x.clone().ln_1_plus_x_prec_round(prec, rm);
        assert!(ln_1_plus_x.is_valid());

        assert_eq!(ln_1_plus_x.to_string(), out);
        assert_eq!(to_hex_string(&ln_1_plus_x), out_hex);
        assert_eq!(o, o_out);

        let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_prec_round_ref(prec, rm);
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut ln_1_plus_x_alt = x.clone();
        let o_alt = ln_1_plus_x_alt.ln_1_plus_x_prec_round_assign(prec, rm);
        assert!(ln_1_plus_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&ln_1_plus_x),
            ComparableFloatRef(&ln_1_plus_x_alt)
        );
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_ln_1_plus_x, rug_o) =
                rug_ln_1_plus_x_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_ln_1_plus_x)),
                ComparableFloatRef(&ln_1_plus_x),
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

    test("1.0", "0x1.0#1", 1, Floor, "0.5", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Down, "0.5", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "0.5", "0x0.8#1", Less);

    test("1.0", "0x1.0#1", 10, Floor, "0.692", "0x0.b14#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Down, "0.692", "0x0.b14#10", Less);
    test("1.0", "0x1.0#1", 10, Up, "0.693", "0x0.b18#10", Greater);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.693",
        "0x0.b18#10",
        Greater,
    );

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
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 1, Down, "-Infinity", "-Infinity", Equal);
    test("-1.0", "-0x1.0#1", 1, Up, "-Infinity", "-Infinity", Equal);
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
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 10, Up, "-Infinity", "-Infinity", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Floor,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Down,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Less,
    );

    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Floor,
        "0.692",
        "0x0.b14#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Down,
        "0.692",
        "0x0.b14#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Up,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "0.693",
        "0x0.b18#10",
        Greater,
    );

    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        1,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Down,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Up,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );

    test("123.0", "0x7b.0#7", 1, Floor, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Down, "4.0", "0x4.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Up, "8.0", "0x8.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "4.0", "0x4.0#1", Less);

    test("123.0", "0x7b.0#7", 10, Floor, "4.81", "0x4.d0#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "4.82",
        "0x4.d2#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 10, Down, "4.81", "0x4.d0#10", Less);
    test("123.0", "0x7b.0#7", 10, Up, "4.82", "0x4.d2#10", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Nearest,
        "4.82",
        "0x4.d2#10",
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
        "1.42",
        "0x1.6b8#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "1.422",
        "0x1.6c0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "1.42",
        "0x1.6b8#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "1.422",
        "0x1.6c0#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "1.422",
        "0x1.6c0#10",
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
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Floor,
        "0.692",
        "0x0.b14#10",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Ceiling,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Down,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Down,
        "0.692",
        "0x0.b14#10",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Up,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "1.000000000000000000000000000002",
        "0x1.0000000000000000000000002#100",
        10,
        Nearest,
        "0.693",
        "0x0.b18#10",
        Greater,
    );

    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Floor,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Floor,
        "0.692",
        "0x0.b14#10",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Ceiling,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Down,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Down,
        "0.692",
        "0x0.b14#10",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Up,
        "0.693",
        "0x0.b18#10",
        Greater,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Less,
    );
    test(
        "0.999999999999999999999999999998",
        "0x0.ffffffffffffffffffffffffe#99",
        10,
        Nearest,
        "0.693",
        "0x0.b18#10",
        Greater,
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
        "1.312",
        "0x1.500#10",
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
        "1.314",
        "0x1.508#10",
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
        "1.312",
        "0x1.500#10",
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
        "1.314",
        "0x1.508#10",
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
        "1.312",
        "0x1.500#10",
        Less,
    );

    // Coverage cases.
    // - second Ziv-loop iteration (float_can_round fails the first time)
    test("0.19", "0x0.30#3", 3, Nearest, "0.16", "0x0.28#3", Less);
    // - round_near_x halfway case, error away from zero
    test("-0.09", "-0x0.18#2", 1, Nearest, "-0.1", "-0x0.2#1", Less);
    // - round_near_x halfway case, error toward zero
    test("0.05", "0x0.0c#2", 1, Nearest, "0.03", "0x0.08#1", Less);
    // - 1 + x overflows to infinity, ln(x) used instead
    test(
        "too_big",
        "0x7.ffffffeE+268435455#30",
        16,
        Nearest,
        "7.4426e8",
        "0x2.c5c8E+7#16",
        Less,
    );
    // - round_near_x exact-copy case, rounding away from zero steps the result
    test("-0.06", "-0x0.1#1", 1, Up, "-0.1", "-0x0.2#1", Less);
    // - round_near_x exact-copy case, rounding toward zero steps the result
    test("0.03", "0x0.08#1", 1, Down, "0.02", "0x0.04#1", Less);
    // - round_near_x exact-copy case, error away from zero, no step needed
    test(
        "-0.06", "-0x0.1#1", 1, Nearest, "-0.06", "-0x0.1#1", Greater,
    );
    // - round_near_x exact-copy case, error toward zero, no step needed
    test(
        "too_small",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    // - round_near_x cannot round, falling through to the general case
    test(
        "-0.06",
        "-0x0.10#3",
        1,
        Nearest,
        "-0.06",
        "-0x0.1#1",
        Greater,
    );
}

#[test]
fn ln_1_plus_x_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).ln_1_plus_x_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).ln_1_plus_x_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.ln_1_plus_x_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.ln_1_plus_x_prec_round(1, Exact));
    assert_panic!(THREE.ln_1_plus_x_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.ln_1_plus_x_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_1_plus_x_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (ln_1_plus_x, o) = x.clone().ln_1_plus_x_prec_round(prec, rm);
    assert!(ln_1_plus_x.is_valid());

    let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_prec_round_ref(prec, rm);
    assert!(ln_1_plus_x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_1_plus_x_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_1_plus_x));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_ln_1_plus_x, rug_o) =
            rug_ln_1_plus_x_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_1_plus_x)),
            ComparableFloatRef(&ln_1_plus_x),
        );
        assert_eq!(rug_o, o);
    }

    // TODO (coverage step): add ln_1_plus_x-specific property comparing the result to x; the
    // analogous ln property `ln < x` does not translate cleanly when the result precision differs
    // from the precision of x.

    if ln_1_plus_x.is_normal() {
        assert_eq!(ln_1_plus_x.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(ln_1_plus_x > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(ln_1_plus_x < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.ln_1_plus_x_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(ln_1_plus_x.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.ln_1_plus_x_prec_round_ref(prec, Exact));
    }
}

#[test]
fn ln_1_plus_x_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_21().test_properties(|(x, prec, rm)| {
        ln_1_plus_x_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_22().test_properties(|(x, prec, rm)| {
        ln_1_plus_x_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (ln_1_plus_x, o) = Float::NAN.ln_1_plus_x_prec_round(prec, rm);
        assert!(ln_1_plus_x.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.ln_1_plus_x_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (s, o) = Float::NEGATIVE_INFINITY.ln_1_plus_x_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.ln_1_plus_x_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.ln_1_plus_x_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::NEGATIVE_ONE.ln_1_plus_x_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_1_plus_x_prec_properties_helper(x: Float, prec: u64) {
    let (ln_1_plus_x, o) = x.clone().ln_1_plus_x_prec(prec);
    assert!(ln_1_plus_x.is_valid());

    let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_prec_ref(prec);
    assert!(ln_1_plus_x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_1_plus_x_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_1_plus_x));
    assert_eq!(o_alt, o);

    let (rug_ln_1_plus_x, rug_o) = rug_ln_1_plus_x_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_ln_1_plus_x)),
        ComparableFloatRef(&ln_1_plus_x),
    );
    assert_eq!(rug_o, o);

    let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );
    assert_eq!(o_alt, o);

    // TODO (coverage step): add ln_1_plus_x-specific property comparing the result to x; the
    // analogous ln property `ln < x` does not translate cleanly when the result precision differs
    // from the precision of x.

    if ln_1_plus_x.is_normal() {
        assert_eq!(ln_1_plus_x.get_prec(), Some(prec));
        if x > 0u32 && o > Less {
            assert!(ln_1_plus_x > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(ln_1_plus_x < 0u32);
        }
    }
}

#[test]
fn ln_1_plus_x_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        ln_1_plus_x_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        ln_1_plus_x_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        ln_1_plus_x_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (ln_1_plus_x, o) = Float::NAN.ln_1_plus_x_prec(prec);
        assert!(ln_1_plus_x.is_nan());
        assert_eq!(o, Equal);

        let (ln_1_plus_x, o) = Float::ZERO.ln_1_plus_x_prec(prec);
        assert_eq!(ComparableFloat(ln_1_plus_x), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (ln_1_plus_x, o) = Float::NEGATIVE_ZERO.ln_1_plus_x_prec(prec);
        assert_eq!(
            ComparableFloat(ln_1_plus_x),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.ln_1_plus_x_prec(prec),
            (Float::INFINITY, Equal)
        );
        let (ln_1_plus_x, o) = Float::NEGATIVE_INFINITY.ln_1_plus_x_prec(prec);
        assert_eq!(ComparableFloat(ln_1_plus_x), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::NEGATIVE_ONE.ln_1_plus_x_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_1_plus_x_round_properties_helper(x: Float, rm: RoundingMode) {
    let (ln_1_plus_x, o) = x.clone().ln_1_plus_x_round(rm);
    assert!(ln_1_plus_x.is_valid());

    let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_round_ref(rm);
    assert!(ln_1_plus_x_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.ln_1_plus_x_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_1_plus_x));
    assert_eq!(o_alt, o);

    let (ln_1_plus_x_alt, o_alt) = x.ln_1_plus_x_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );
    assert_eq!(o_alt, o);

    if x >= -1i32 && x.is_finite() {
        // Since the result has the same precision as x, and ln(1+x) <= x, the rounded result cannot
        // exceed x.
        assert!(ln_1_plus_x <= x);
    }

    if ln_1_plus_x.is_normal() {
        assert_eq!(ln_1_plus_x.get_prec(), Some(x.get_prec().unwrap()));
        if x > 0u32 && o > Less {
            assert!(ln_1_plus_x > 0u32);
        } else if x < 0u32 && o < Greater {
            assert!(ln_1_plus_x < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_ln_1_plus_x, rug_o) = rug_ln_1_plus_x_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_ln_1_plus_x)),
            ComparableFloatRef(&ln_1_plus_x),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.ln_1_plus_x_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(ln_1_plus_x.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.ln_1_plus_x_round_ref(Exact));
    }
}

#[test]
fn ln_1_plus_x_round_properties() {
    float_rounding_mode_pair_gen_var_36().test_properties(|(x, rm)| {
        ln_1_plus_x_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_37().test_properties(|(x, rm)| {
        ln_1_plus_x_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (ln_1_plus_x, o) = Float::NAN.ln_1_plus_x_round(rm);
        assert!(ln_1_plus_x.is_nan());
        assert_eq!(o, Equal);

        let (ln_1_plus_x, o) = Float::ZERO.ln_1_plus_x_round(rm);
        assert_eq!(ComparableFloat(ln_1_plus_x), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (ln_1_plus_x, o) = Float::NEGATIVE_ZERO.ln_1_plus_x_round(rm);
        assert_eq!(
            ComparableFloat(ln_1_plus_x),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.ln_1_plus_x_round(rm),
            (Float::INFINITY, Equal)
        );
        let (ln_1_plus_x, o) = Float::NEGATIVE_INFINITY.ln_1_plus_x_round(rm);
        assert_eq!(ComparableFloat(ln_1_plus_x), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::NEGATIVE_ONE.ln_1_plus_x_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );
    });
}

#[allow(clippy::needless_pass_by_value)]
fn ln_1_plus_x_properties_helper(x: Float) {
    let ln_1_plus_x = x.clone().ln_1_plus_x();
    assert!(ln_1_plus_x.is_valid());

    let ln_1_plus_x_alt = (&x).ln_1_plus_x();
    assert!(ln_1_plus_x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );

    let mut x_alt = x.clone();
    x_alt.ln_1_plus_x_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&ln_1_plus_x));

    let ln_1_plus_x_alt = x
        .ln_1_plus_x_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );
    let ln_1_plus_x_alt = x.ln_1_plus_x_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );

    let ln_1_plus_x_alt = x.ln_1_plus_x_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&ln_1_plus_x_alt),
        ComparableFloatRef(&ln_1_plus_x)
    );

    if x >= -1i32 && x.is_finite() {
        // Since the result has the same precision as x, and ln(1+x) <= x, the rounded result cannot
        // exceed x.
        assert!(ln_1_plus_x <= x);
    }

    let rug_ln_1_plus_x = rug_ln_1_plus_x(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_ln_1_plus_x)),
        ComparableFloatRef(&ln_1_plus_x),
    );
}

#[test]
fn ln_1_plus_x_properties() {
    float_gen().test_properties(|x| {
        ln_1_plus_x_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        ln_1_plus_x_properties_helper(x);
    });
}
