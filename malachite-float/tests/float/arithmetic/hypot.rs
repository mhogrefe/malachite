// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    Abs, Hypot, HypotAssign, IsPowerOf2, PowerOf2, Square,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::float::arithmetic::hypot::primitive_float_hypot;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::hypot::{
    rug_hypot, rug_hypot_prec, rug_hypot_prec_round, rug_hypot_round,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_43, float_float_rounding_mode_triple_gen_var_44,
    float_float_unsigned_rounding_mode_quadruple_gen_var_24,
    float_float_unsigned_rounding_mode_quadruple_gen_var_25, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_2, float_pair_gen, float_pair_gen_var_10,
    float_unsigned_rounding_mode_triple_gen_var_1,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::cmp::max;
use std::panic::catch_unwind;

#[test]
fn test_hypot() {
    let test = |s: &str, s_hex: &str, t: &str, t_hex: &str, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let hypot = x.clone().hypot(y.clone());
        assert!(hypot.is_valid());
        assert_eq!(hypot.to_string(), out);
        assert_eq!(to_hex_string(&hypot), out_hex);

        let hypot_alt = x.clone().hypot(&y);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        let hypot_alt = (&x).hypot(y.clone());
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        let hypot_alt = (&x).hypot(&y);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));

        let mut hypot_alt = x.clone();
        hypot_alt.hypot_assign(y.clone());
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        let mut hypot_alt = x.clone();
        hypot_alt.hypot_assign(&y);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_hypot(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&hypot),
        );
    };
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "Infinity", "Infinity");
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test("NaN", "NaN", "0.0", "0x0.0", "NaN", "NaN");
    test("NaN", "NaN", "2.0", "0x2.0#1", "NaN", "NaN");
    test("Infinity", "Infinity", "NaN", "NaN", "Infinity", "Infinity");
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
    );
    test(
        "Infinity", "Infinity", "-2.0", "-0x2.0#1", "Infinity", "Infinity",
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        "Infinity",
        "Infinity",
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-3.0", "-0x3.0#2", "3.0", "0x3.0#2");
    test("-0.0", "-0x0.0", "3.0", "0x3.0#2", "3.0", "0x3.0#2");
    test("3.0", "0x3.0#2", "4.0", "0x4.0#1", "4.0", "0x4.0#2");
    test("-3.0", "-0x3.0#2", "-4.0", "-0x4.0#1", "4.0", "0x4.0#2");
    test("5.0", "0x5.0#3", "12.0", "0xc.0#2", "12.0", "0xc.0#3");
    test("8.0", "0x8.0#1", "15.0", "0xf.0#4", "16.0", "0x10.0#4");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("1.0", "0x1.0#1", "2.0", "0x2.0#1", "2.0", "0x2.0#1");
    test("1.50", "0x1.8#5", "2.25", "0x2.4#6", "2.69", "0x2.b#6");
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        "3.1415929203539823008849557522128",
        "0x3.243f6f0243f6f0243f6f02440#100",
        "4.4437773456403535526994711182169",
        "0x4.719b64623b234114a073b2ce0#100",
    );
}

#[test]
fn test_hypot_prec() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                prec: u64,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (hypot, o) = x.clone().hypot_prec(y.clone(), prec);
        assert!(hypot.is_valid());
        assert_eq!(hypot.to_string(), out);
        assert_eq!(to_hex_string(&hypot), out_hex);
        assert_eq!(o, o_out);

        let (hypot_alt, o_alt) = x.clone().hypot_prec_val_ref(&y, prec);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let (hypot_alt, o_alt) = x.hypot_prec_ref_val(y.clone(), prec);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let (hypot_alt, o_alt) = x.hypot_prec_ref_ref(&y, prec);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);

        let mut hypot_alt = x.clone();
        let o_alt = hypot_alt.hypot_prec_assign(y.clone(), prec);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let mut hypot_alt = x.clone();
        let o_alt = hypot_alt.hypot_prec_assign_ref(&y, prec);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);

        let (rug_hypot, rug_o) = rug_hypot_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_hypot)),
            ComparableFloatRef(&hypot)
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", "2.0", "0x2.0#1", 10, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "NaN", "NaN", 10, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-3.0",
        "-0x3.0#2",
        10,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 10, "0.0", "0x0.0", Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        "5.0000",
        "0x5.00#10",
        Equal,
    );
    test(
        "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, "4.0", "0x4.0#2", Less,
    );
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        53,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        20,
        "2.2360687",
        "0x2.3c6f0#20",
        Greater,
    );
    test(
        "-99.00",
        "-0x63.0#7",
        "100.0",
        "0x64.0#5",
        30,
        "140.71602607",
        "0x8c.b74d7c#30",
        Less,
    );
}

#[test]
fn hypot_prec_fail() {
    assert_panic!(Float::from(3).hypot_prec(Float::from(4), 0));
    assert_panic!(Float::from(3).hypot_prec_ref_ref(&Float::from(4), 0));
}

#[test]
fn test_hypot_round() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (hypot, o) = x.clone().hypot_round(y.clone(), rm);
        assert!(hypot.is_valid());
        assert_eq!(hypot.to_string(), out);
        assert_eq!(to_hex_string(&hypot), out_hex);
        assert_eq!(o, o_out);

        let (hypot_alt, o_alt) = x.clone().hypot_round_val_ref(&y, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let (hypot_alt, o_alt) = x.hypot_round_ref_val(y.clone(), rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let (hypot_alt, o_alt) = x.hypot_round_ref_ref(&y, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);

        let mut hypot_alt = x.clone();
        let o_alt = hypot_alt.hypot_round_assign(y.clone(), rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let mut hypot_alt = x.clone();
        let o_alt = hypot_alt.hypot_round_assign_ref(&y, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_hypot, rug_o) =
                rug_hypot_round(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_hypot)),
                ComparableFloatRef(&hypot)
            );
            assert_eq!(rug_o, o);
        }
    };
    test(
        "NaN", "NaN", "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", "-3.0", "-0x3.0#2", Floor, "3.0", "0x3.0#2", Equal,
    );
    test(
        "20.0", "0x14.0#3", "21.0", "0x15.0#5", Exact, "29.0", "0x1d.0#5", Equal,
    );
    test(
        "1.0000000",
        "0x1.00000#20",
        "1.0000",
        "0x1.000#10",
        Floor,
        "1.4142132",
        "0x1.6a09e#20",
        Less,
    );
    test(
        "1.0000000",
        "0x1.00000#20",
        "1.0000",
        "0x1.000#10",
        Ceiling,
        "1.4142151",
        "0x1.6a0a0#20",
        Greater,
    );
    test(
        "1.0000000",
        "0x1.00000#20",
        "1.0000",
        "0x1.000#10",
        Down,
        "1.4142132",
        "0x1.6a09e#20",
        Less,
    );
    test(
        "1.0000000",
        "0x1.00000#20",
        "1.0000",
        "0x1.000#10",
        Up,
        "1.4142151",
        "0x1.6a0a0#20",
        Greater,
    );
    // - the NaN, infinity, and zero singular arms, in both operand orders; an infinite leg
    //   dominates a NaN, and zero legs delegate to rounding the other's absolute value
    test(
        "1.0000000",
        "0x1.00000#20",
        "1.0000",
        "0x1.000#10",
        Nearest,
        "1.4142132",
        "0x1.6a09e#20",
        Less,
    );
}

#[test]
fn hypot_round_fail() {
    assert_panic!(Float::from(1).hypot_round(Float::from(1), Exact));
    assert_panic!(Float::from(3).hypot_round(Float::from(4), Exact));
}

#[test]
fn test_hypot_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                t: &str,
                t_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (hypot, o) = x.clone().hypot_prec_round(y.clone(), prec, rm);
        assert!(hypot.is_valid());
        assert_eq!(hypot.to_string(), out);
        assert_eq!(to_hex_string(&hypot), out_hex);
        assert_eq!(o, o_out);

        let (hypot_alt, o_alt) = x.clone().hypot_prec_round_val_ref(&y, prec, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let (hypot_alt, o_alt) = x.hypot_prec_round_ref_val(y.clone(), prec, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let (hypot_alt, o_alt) = x.hypot_prec_round_ref_ref(&y, prec, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);

        let mut hypot_alt = x.clone();
        let o_alt = hypot_alt.hypot_prec_round_assign(y.clone(), prec, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);
        let mut hypot_alt = x.clone();
        let o_alt = hypot_alt.hypot_prec_round_assign_ref(&y, prec, rm);
        assert!(hypot_alt.is_valid());
        assert_eq!(ComparableFloatRef(&hypot), ComparableFloatRef(&hypot_alt));
        assert_eq!(o_alt, o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_hypot, rug_o) = rug_hypot_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                prec,
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_hypot)),
                ComparableFloatRef(&hypot)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        2,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-3.0", "-0x3.0#2", 2, Floor, "3.0", "0x3.0#2", Equal,
    );
    // - the Ziv path with an inexact square root: can_round breaks on the first iteration (err =
    //   2), with no operand swap
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        Floor,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    // - an operand swap (|x| < |y|); an exactly representable hypotenuse rounded to a precision too
    //   small to hold it
    test(
        "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Floor, "4.0", "0x4.0#2", Less,
    );
    // - the |x|-approximation shortcut (diff_exp > threshold), through float_round_near_x
    test(
        "1.0",
        "0x1.0#1",
        "9.3e-302",
        "0x1.0E-250#1",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    // - the shortcut with a negative x and a high-precision y
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        "4.0389678347315804437080503e-27",
        "0x1.40000000000000000000E-22#80",
        25,
        Floor,
        "1.26765060e30",
        "0x1.000000E+25#25",
        Less,
    );
    test(
        "-0.0", "-0x0.0", "-3.0", "-0x3.0#2", 2, Ceiling, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Ceiling, "6.0", "0x6.0#2", Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "9.3e-302",
        "0x1.0E-250#1",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        "4.0389678347315804437080503e-27",
        "0x1.40000000000000000000E-22#80",
        25,
        Ceiling,
        "1.26765068e30",
        "0x1.000001E+25#25",
        Greater,
    );
    test(
        "-0.0", "-0x0.0", "-3.0", "-0x3.0#2", 2, Down, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        Down,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    test(
        "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Down, "4.0", "0x4.0#2", Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "9.3e-302",
        "0x1.0E-250#1",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        "4.0389678347315804437080503e-27",
        "0x1.40000000000000000000E-22#80",
        25,
        Down,
        "1.26765060e30",
        "0x1.000000E+25#25",
        Less,
    );
    test(
        "-0.0", "-0x0.0", "-3.0", "-0x3.0#2", 2, Up, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        Up,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Up, "6.0", "0x6.0#2", Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "9.3e-302",
        "0x1.0E-250#1",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        "4.0389678347315804437080503e-27",
        "0x1.40000000000000000000E-22#80",
        25,
        Up,
        "1.26765068e30",
        "0x1.000001E+25#25",
        Greater,
    );
    test(
        "-0.0", "-0x0.0", "-3.0", "-0x3.0#2", 2, Nearest, "3.0", "0x3.0#2", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    test(
        "3.0", "0x3.0#2", "4.0", "0x4.0#1", 2, Nearest, "4.0", "0x4.0#2", Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "9.3e-302",
        "0x1.0E-250#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        "4.0389678347315804437080503e-27",
        "0x1.40000000000000000000E-22#80",
        25,
        Nearest,
        "1.26765060e30",
        "0x1.000000E+25#25",
        Less,
    );
    // - the Ziv loop breaking on an exact intermediate result, under every rounding mode, including
    //   at large in-range exponents
    test(
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        Floor,
        "5.0000",
        "0x5.00#10",
        Equal,
    );
    test(
        "20.0",
        "0x14.0#3",
        "-21.0",
        "-0x15.0#5",
        12,
        Floor,
        "29.000",
        "0x1d.00#12",
        Equal,
    );
    test(
        "9.8e150",
        "0x3.0E+125#2",
        "1.3e151",
        "0x4.0E+125#1",
        3,
        Floor,
        "1.6e151",
        "0x5.0E+125#3",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        Ceiling,
        "5.0000",
        "0x5.00#10",
        Equal,
    );
    test(
        "20.0",
        "0x14.0#3",
        "-21.0",
        "-0x15.0#5",
        12,
        Ceiling,
        "29.000",
        "0x1d.00#12",
        Equal,
    );
    test(
        "9.8e150",
        "0x3.0E+125#2",
        "1.3e151",
        "0x4.0E+125#1",
        3,
        Ceiling,
        "1.6e151",
        "0x5.0E+125#3",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "5.0000",
        "0x5.00#10",
        Equal,
    );
    test(
        "20.0",
        "0x14.0#3",
        "-21.0",
        "-0x15.0#5",
        12,
        Nearest,
        "29.000",
        "0x1d.00#12",
        Equal,
    );
    test(
        "9.8e150",
        "0x3.0E+125#2",
        "1.3e151",
        "0x4.0E+125#1",
        3,
        Nearest,
        "1.6e151",
        "0x5.0E+125#3",
        Equal,
    );
    // - Exact routed through the integer-level exact path
    test(
        "3.0",
        "0x3.0#2",
        "4.0",
        "0x4.0#1",
        10,
        Exact,
        "5.0000",
        "0x5.00#10",
        Equal,
    );
    test(
        "20.0",
        "0x14.0#3",
        "-21.0",
        "-0x15.0#5",
        12,
        Exact,
        "29.000",
        "0x1d.00#12",
        Equal,
    );
    test(
        "9.8e150",
        "0x3.0E+125#2",
        "1.3e151",
        "0x4.0E+125#1",
        3,
        Exact,
        "1.6e151",
        "0x5.0E+125#3",
        Equal,
    );
    // - the working precision is below the input precision (err = 4 in the Ziv loop)
    test(
        "3.1428571428571428571428571428571428571428571428571428571428568",
        "0x3.24924924924924924924924924924924924924924924924924#200",
        "3.1415929203539823008849557522123893805309734513274336283185852",
        "0x3.243f6f0243f6f0243f6f0243f6f0243f6f0243f6f0243f6f04#200",
        10,
        Floor,
        "4.4375",
        "0x4.70#10",
        Less,
    );
    test(
        "3.1428571428571428571428571428571428571428571428571428571428568",
        "0x3.24924924924924924924924924924924924924924924924924#200",
        "3.1415929203539823008849557522123893805309734513274336283185852",
        "0x3.243f6f0243f6f0243f6f0243f6f0243f6f0243f6f0243f6f04#200",
        10,
        Nearest,
        "4.4453",
        "0x4.72#10",
        Greater,
    );
    // - a Nearest tie in the shortcut: x is representable at prec + 1 but not at prec, and the
    //   sticky contribution of y breaks the tie away from zero, where rounding x alone would tie to
    //   even
    test(
        "1.0010",
        "0x1.004#11",
        "9.3e-302",
        "0x1.0E-250#1",
        10,
        Nearest,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.0010",
        "0x1.004#11",
        "9.3e-302",
        "0x1.0E-250#1",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    // - overflow saturation from the Ziv path under Ceiling and Up; under Nearest the result stays
    //   finite, after a second Ziv iteration
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    // - the shortcut at the very top of the exponent range: stepping away from zero overflows to
    //   Infinity under Ceiling
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Nearest,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0",
        "0x1.0#1",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0",
        "0x1.0#1",
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
}

#[test]
fn hypot_prec_round_fail() {
    assert_panic!(Float::from(3).hypot_prec_round(Float::from(4), 0, Floor));
    assert_panic!(Float::from(3).hypot_prec_round_ref_ref(&Float::from(4), 0, Floor));
    // sqrt(2) is irrational
    assert_panic!(Float::from(1).hypot_prec_round(Float::from(1), 10, Exact));
    // 5 needs 3 bits
    assert_panic!(Float::from(3).hypot_prec_round(Float::from(4), 2, Exact));
    // the shortcut regime is never exact
    assert_panic!({
        let y = Float::from(1) >> 1000u64;
        Float::from(1).hypot_prec_round(y, 10, Exact)
    });
}

// The exponent gap is just inside the general-path threshold, so the MPFR-style path would scale y
// to an exponent far below the minimum: the regime of MPFR's FIXME concerning underflow, which the
// exact integer-level path handles. The same value of x at tiny precision routes through the
// shortcut instead, giving an independent answer to compare against.
#[test]
fn test_hypot_underflow_regime_high() {
    let x_thin = Float::power_of_2(1073741821i64);
    let x_fat = Float::from_float_prec_round_ref(&x_thin, 1073741825, Floor).0;
    let y = Float::power_of_2(-1073741822i64);
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let (h_fat, o_fat) = x_fat.hypot_prec_round_ref_ref(&y, 10, rm);
        let (h_thin, o_thin) = x_thin.hypot_prec_round_ref_ref(&y, 10, rm);
        assert!(h_fat.is_valid());
        assert_eq!(ComparableFloatRef(&h_fat), ComparableFloatRef(&h_thin));
        assert_eq!(o_fat, o_thin);
    }
    assert_panic!(x_fat.hypot_prec_round_ref_ref(&y, 10, Exact));

    // With x all-ones at the very top of the exponent range, rounding away from zero pushes the
    // result past the maximum exponent, hitting the exact path's overflow arm; toward-zero modes
    // and Nearest keep the exponent in range.
    let x_top =
        Float::from_float_prec_round_ref(&Float::max_finite_value_with_prec(10), 1073741825, Floor)
            .0;
    for (rm, o_want) in
        [(Ceiling, Greater), (Up, Greater), (Floor, Less), (Down, Less), (Nearest, Less)]
    {
        let (h, o) = x_top.hypot_prec_round_ref_ref(&y, 10, rm);
        if o_want == Greater {
            assert_eq!(h, Float::INFINITY);
        } else {
            assert_eq!(
                ComparableFloat(h),
                ComparableFloat(Float::max_finite_value_with_prec(10))
            );
        }
        assert_eq!(o, o_want);
    }
}

// Verifies the result against exact `Rational` arithmetic: the ternary value must match the
// comparison of hypot^2 with x^2 + y^2, and the rounding must be correct, which is checked by
// comparing the squares of the appropriate neighbor or midpoint against x^2 + y^2. Only called with
// finite nonzero x and y and normal positive hypot.
fn verify_against_exact(x: &Float, y: &Float, prec: u64, rm: RoundingMode, h: &Float, o: Ordering) {
    let r = Rational::exact_from(x).square() + Rational::exact_from(y).square();
    let hr = Rational::exact_from(h);
    let h_squared = (&hr).square();
    match o {
        Equal => assert_eq!(h_squared, r),
        Less => assert!(h_squared < r),
        Greater => assert!(h_squared > r),
    }
    let eh = i64::from(h.get_exponent().unwrap());
    // spacing of prec-bit values just above h, and just below h (halved at a binade boundary)
    let ulp_above = Rational::power_of_2(eh - i64::exact_from(prec));
    let ulp_below = if h.is_power_of_2() {
        Rational::power_of_2(eh - 1 - i64::exact_from(prec))
    } else {
        ulp_above.clone()
    };
    match rm {
        Floor | Down => {
            assert_ne!(o, Greater);
            assert!((hr + ulp_above).square() > r);
        }
        Ceiling | Up => {
            assert_ne!(o, Less);
            assert!((hr - ulp_below).square() < r);
        }
        Nearest => match o {
            Less => assert!((hr + (ulp_above >> 1u64)).square() >= r),
            Greater => assert!((hr - (ulp_below >> 1u64)).square() <= r),
            Equal => {}
        },
        Exact => assert_eq!(o, Equal),
    }
}

const EXPONENT_GATE: i64 = 1 << 16;

fn exponent_in_gate(x: &Float) -> bool {
    x.get_exponent()
        .is_none_or(|e| i64::from(e).abs() < EXPONENT_GATE)
}

#[allow(clippy::needless_pass_by_value)]
fn hypot_prec_round_properties_helper(x: Float, y: Float, prec: u64, rm: RoundingMode) {
    let (hypot, o) = x.clone().hypot_prec_round(y.clone(), prec, rm);
    assert!(hypot.is_valid());
    let (hypot_alt, o_alt) = x.clone().hypot_prec_round_val_ref(&y, prec, rm);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let (hypot_alt, o_alt) = x.hypot_prec_round_ref_val(y.clone(), prec, rm);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let (hypot_alt, o_alt) = x.hypot_prec_round_ref_ref(&y, prec, rm);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.hypot_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.hypot_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    // symmetry
    let (hypot_alt, o_alt) = y.hypot_prec_round_ref_ref(&x, prec, rm);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    // the result is never negative, and never a negative zero
    if !hypot.is_nan() {
        assert!(!hypot.is_sign_negative());
    }

    // negating either operand changes nothing
    let (hypot_alt, o_alt) = (-&x).hypot_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let (hypot_alt, o_alt) = x.hypot_prec_round_ref_ref(&-&y, prec, rm);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    if exponent_in_gate(&x)
        && exponent_in_gate(&y)
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
        let (rug_hypot, rug_o) = rug_hypot_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rug_rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_hypot)),
            ComparableFloatRef(&hypot)
        );
        assert_eq!(rug_o, o);
    }

    if x.is_infinite() || y.is_infinite() {
        assert_eq!(hypot, Float::INFINITY);
        assert_eq!(o, Equal);
    } else if x.is_nan() || y.is_nan() {
        assert!(hypot.is_nan());
        assert_eq!(o, Equal);
    }

    if hypot.is_normal() && x.is_normal() && y.is_normal() {
        assert_eq!(hypot.get_prec(), Some(prec));
        if exponent_in_gate(&x) && exponent_in_gate(&y) && exponent_in_gate(&hypot) {
            verify_against_exact(&x, &y, prec, rm, &hypot, o);
        }
    }

    if exponent_in_gate(&x) && exponent_in_gate(&y) {
        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                let (s, oo) = x.hypot_prec_round_ref_ref(&y, prec, rm);
                assert_eq!(
                    ComparableFloat(s.abs_negative_zero_ref()),
                    ComparableFloat(hypot.abs_negative_zero_ref())
                );
                assert_eq!(oo, Equal);
            }
        } else {
            assert_panic!(x.hypot_prec_round_ref_ref(&y, prec, Exact));
        }
    }
}

#[test]
fn hypot_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_24().test_properties(
        |(x, y, prec, rm)| {
            hypot_prec_round_properties_helper(x, y, prec, rm);
        },
    );

    float_float_unsigned_rounding_mode_quadruple_gen_var_25().test_properties(
        |(x, y, prec, rm)| {
            hypot_prec_round_properties_helper(x, y, prec, rm);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        // an infinite leg gives +Infinity, even against NaN
        let (hypot, o) = x.hypot_prec_round_ref_ref(&Float::INFINITY, prec, rm);
        assert_eq!(hypot, Float::INFINITY);
        assert_eq!(o, Equal);
        let (hypot, o) = Float::INFINITY.hypot_prec_round_ref_ref(&x, prec, rm);
        assert_eq!(hypot, Float::INFINITY);
        assert_eq!(o, Equal);
        let (hypot, o) = x.hypot_prec_round_ref_ref(&Float::NEGATIVE_INFINITY, prec, rm);
        assert_eq!(hypot, Float::INFINITY);
        assert_eq!(o, Equal);

        if !x.is_infinite() {
            let (hypot, o) = x.hypot_prec_round_ref_ref(&Float::NAN, prec, rm);
            assert!(hypot.is_nan());
            assert_eq!(o, Equal);
            let (hypot, o) = Float::NAN.hypot_prec_round_ref_ref(&x, prec, rm);
            assert!(hypot.is_nan());
            assert_eq!(o, Equal);
        }
    });

    // a zero leg means rounding the absolute value of the other
    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if x.is_nan() || x.is_infinite() {
            return;
        }
        let (abs, o_abs) = Float::from_float_prec_round_ref(&(&x).abs(), prec, rm);
        for zero in [Float::ZERO, Float::NEGATIVE_ZERO] {
            let (hypot, o) = x.hypot_prec_round_ref_ref(&zero, prec, rm);
            assert_eq!(ComparableFloat(hypot), ComparableFloat(abs.clone()));
            assert_eq!(o, o_abs);
            let (hypot, o) = zero.hypot_prec_round_ref_ref(&x, prec, rm);
            assert_eq!(ComparableFloat(hypot), ComparableFloat(abs.clone()));
            assert_eq!(o, o_abs);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn hypot_prec_properties_helper(x: Float, y: Float, prec: u64) {
    let (hypot, o) = x.clone().hypot_prec(y.clone(), prec);
    assert!(hypot.is_valid());
    let (hypot_alt, o_alt) = x.clone().hypot_prec_val_ref(&y, prec);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let (hypot_alt, o_alt) = x.hypot_prec_ref_val(y.clone(), prec);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let (hypot_alt, o_alt) = x.hypot_prec_ref_ref(&y, prec);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.hypot_prec_assign(y.clone(), prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.hypot_prec_assign_ref(&y, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    let (hypot_alt, o_alt) = x.hypot_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
}

#[test]
fn hypot_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        hypot_prec_properties_helper(x, y, prec);
    });

    float_float_unsigned_triple_gen_var_2().test_properties(|(x, y, prec)| {
        hypot_prec_properties_helper(x, y, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn hypot_round_properties_helper(x: Float, y: Float, rm: RoundingMode) {
    let (hypot, o) = x.clone().hypot_round(y.clone(), rm);
    assert!(hypot.is_valid());
    let (hypot_alt, o_alt) = x.clone().hypot_round_val_ref(&y, rm);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let (hypot_alt, o_alt) = x.hypot_round_ref_val(y.clone(), rm);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let (hypot_alt, o_alt) = x.hypot_round_ref_ref(&y, rm);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.hypot_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
    let mut x_alt = x.clone();
    let o_alt = x_alt.hypot_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);

    let prec = max(x.significant_bits(), y.significant_bits());
    let (hypot_alt, o_alt) = x.hypot_prec_round_ref_ref(&y, prec, rm);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    assert_eq!(o_alt, o);
}

#[test]
fn hypot_round_properties() {
    float_float_rounding_mode_triple_gen_var_43().test_properties(|(x, y, rm)| {
        hypot_round_properties_helper(x, y, rm);
    });

    float_float_rounding_mode_triple_gen_var_44().test_properties(|(x, y, rm)| {
        hypot_round_properties_helper(x, y, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn hypot_properties_helper(x: Float, y: Float) {
    let hypot = x.clone().hypot(y.clone());
    assert!(hypot.is_valid());
    let hypot_alt = x.clone().hypot(&y);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    let hypot_alt = (&x).hypot(y.clone());
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    let hypot_alt = (&x).hypot(&y);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));

    let mut hypot_alt = x.clone();
    hypot_alt.hypot_assign(y.clone());
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    let mut hypot_alt = x.clone();
    hypot_alt.hypot_assign(&y);
    assert!(hypot_alt.is_valid());
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));

    let prec = max(x.significant_bits(), y.significant_bits());
    let (hypot_alt, _) = x.hypot_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    let (hypot_alt, _) = x.clone().hypot_prec(y.clone(), prec);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));
    let (hypot_alt, _) = x.clone().hypot_round(y.clone(), Nearest);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));

    // symmetry
    let hypot_alt = (&y).hypot(&x);
    assert_eq!(ComparableFloatRef(&hypot_alt), ComparableFloatRef(&hypot));

    if exponent_in_gate(&x) && exponent_in_gate(&y) {
        let rug_hypot = rug_hypot(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_hypot)),
            ComparableFloatRef(&hypot)
        );
    }
}

#[test]
fn hypot_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        hypot_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        hypot_properties_helper(x, y);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_hypot() {
    fn test<T: PrimitiveFloat>(x: T, y: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_hypot(x, y)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN, f32::NAN);
    test::<f32>(f32::NAN, f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN, f32::INFINITY);
    test::<f32>(f32::NAN, 0.0, f32::NAN);
    test::<f32>(0.0, -0.0, 0.0);
    test::<f32>(-0.0, -3.0, 3.0);
    test::<f32>(3.0, 4.0, 5.0);
    test::<f32>(-3.0, -4.0, 5.0);
    test::<f32>(1.0, 1.0, core::f32::consts::SQRT_2);
    test::<f32>(core::f32::consts::PI, core::f32::consts::E, 4.1543546);
    test::<f32>(3.4028235e38, 3.4028235e38, f32::INFINITY);
    test::<f32>(1.0e-45, 1.0e-45, 1.0e-45);
    test::<f64>(f64::NAN, f64::INFINITY, f64::INFINITY);
    test::<f64>(3.0, 4.0, 5.0);
    test::<f64>(1.0, 1.0, core::f64::consts::SQRT_2);
    test::<f64>(
        core::f64::consts::PI,
        core::f64::consts::E,
        4.154354402313313,
    );
    test::<f64>(
        1.7976931348623157e308,
        1.7976931348623157e308,
        f64::INFINITY,
    );
    test::<f64>(5.0e-324, 5.0e-324, 5.0e-324);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_hypot_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        let hypot = primitive_float_hypot(x, y);
        assert_eq!(NiceFloat(hypot), NiceFloat(primitive_float_hypot(y, x)));
        if !hypot.is_nan() {
            assert!(!hypot.is_sign_negative());
        }
    });
}

#[test]
fn primitive_float_hypot_properties() {
    apply_fn_to_primitive_floats!(primitive_float_hypot_properties_helper);
}
