// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::strings::ToDebugString;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_exact_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_4,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::cmp::Ordering::*;

#[test]
fn test_to_significand() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let significand = x.to_significand();
        assert_eq!(x.significand_ref(), significand.as_ref());
        assert_eq!(x.clone().into_significand(), significand);
        if Limb::WIDTH == u64::WIDTH {
            assert_eq!(significand.to_debug_string(), out);
            assert_eq!(
                rug::Float::exact_from(&x)
                    .get_significand()
                    .map(|s| Natural::exact_from(&*s)),
                significand
            );
        }
    };
    test("NaN", "NaN", "None");
    test("Infinity", "Infinity", "None");
    test("-Infinity", "-Infinity", "None");
    test("0.0", "0x0.0", "None");
    test("-0.0", "-0x0.0", "None");

    test("1.0", "0x1.0#1", "Some(9223372036854775808)");
    test("2.0", "0x2.0#1", "Some(9223372036854775808)");
    test("0.5", "0x0.8#1", "Some(9223372036854775808)");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "Some(12297829382473033728)",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "Some(13043817825332783104)",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "Some(14488038916154245120)",
    );
    test("3.0e120", "0x1.0E+100#1", "Some(9223372036854775808)");
    test("4.0e-121", "0x1.0E-100#1", "Some(9223372036854775808)");
    test(
        "2.582249878086908589655919172003011874329705792829223512830659e120",
        "0x1.00000000000000000000000000000000000000000000000000E+100#200",
        "Some(57896044618658097711785492504343953926634992332820282019728792003956564819968)",
    );
    test(
        "3.872591914849318272818030633286351847570219192048790865487763e-121",
        "0x1.00000000000000000000000000000000000000000000000000E-100#200",
        "Some(57896044618658097711785492504343953926634992332820282019728792003956564819968)",
    );
    test("too_big", "0x4.0E+268435455#1", "Some(9223372036854775808)");
    test(
        "too_small",
        "0x1.0E-268435456#1",
        "Some(9223372036854775808)",
    );

    test("-1.0", "-0x1.0#1", "Some(9223372036854775808)");
    test("-2.0", "-0x2.0#1", "Some(9223372036854775808)");
    test("-0.5", "-0x0.8#1", "Some(9223372036854775808)");
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "Some(12297829382473033728)",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "Some(13043817825332783104)",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "Some(14488038916154245120)",
    );
    test("-3.0e120", "-0x1.0E+100#1", "Some(9223372036854775808)");
    test("-4.0e-121", "-0x1.0E-100#1", "Some(9223372036854775808)");
    test(
        "-2.582249878086908589655919172003011874329705792829223512830659e120",
        "-0x1.00000000000000000000000000000000000000000000000000E+100#200",
        "Some(57896044618658097711785492504343953926634992332820282019728792003956564819968)",
    );
    test(
        "-3.872591914849318272818030633286351847570219192048790865487763e-121",
        "-0x1.00000000000000000000000000000000000000000000000000E-100#200",
        "Some(57896044618658097711785492504343953926634992332820282019728792003956564819968)",
    );
    test(
        "-too_big",
        "-0x4.0E+268435455#1",
        "Some(9223372036854775808)",
    );
    test(
        "-too_small",
        "-0x1.0E-268435456#1",
        "Some(9223372036854775808)",
    );
}

fn to_significand_properties_helper(x: Float) {
    let significand = x.to_significand();
    assert_eq!(x.significand_ref(), significand.as_ref());
    assert_eq!(x.clone().into_significand(), significand);
    if Limb::WIDTH == u64::WIDTH {
        assert_eq!(
            rug::Float::exact_from(&x)
                .get_significand()
                .map(|s| Natural::exact_from(&*s)),
            significand
        );
    }

    significand.as_ref().map_or_else(
        || {
            assert!(!x.is_normal());
        },
        |significand| {
            assert_ne!(*significand, 0u32);
            assert!(significand
                .significant_bits()
                .divisible_by_power_of_2(Limb::LOG_WIDTH));
        },
    );
    assert_eq!((-x).into_significand(), significand);
}

#[test]
fn to_significand_properties() {
    float_gen().test_properties(|x| {
        to_significand_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        to_significand_properties_helper(x);
    });
}

#[test]
fn test_get_exponent() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let exponent = x.get_exponent();
        assert_eq!(exponent, out);
        assert_eq!(rug::Float::exact_from(&x).get_exp(), out);
    };
    test("NaN", "NaN", None);
    test("Infinity", "Infinity", None);
    test("-Infinity", "-Infinity", None);
    test("0.0", "0x0.0", None);
    test("-0.0", "-0x0.0", None);

    test("1.0", "0x1.0#1", Some(1));
    test("2.0", "0x2.0#1", Some(2));
    test("0.5", "0x0.8#1", Some(0));
    test("0.33333333333333331", "0x0.55555555555554#53", Some(-1));
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", Some(1));
    test("3.1415926535897931", "0x3.243f6a8885a30#53", Some(2));
    test("3.0e120", "0x1.0E+100#1", Some(401));
    test("4.0e-121", "0x1.0E-100#1", Some(-399));
    test(
        "2.582249878086908589655919172003011874329705792829223512830659e120",
        "0x1.00000000000000000000000000000000000000000000000000E+100#200",
        Some(401),
    );
    test(
        "3.872591914849318272818030633286351847570219192048790865487763e-121",
        "0x1.00000000000000000000000000000000000000000000000000E-100#200",
        Some(-399),
    );
    test("too_big", "0x4.0E+268435455#1", Some(1073741823));
    test("too_small", "0x1.0E-268435456#1", Some(-1073741823));

    test("-1.0", "-0x1.0#1", Some(1));
    test("-2.0", "-0x2.0#1", Some(2));
    test("-0.5", "-0x0.8#1", Some(0));
    test("-0.33333333333333331", "-0x0.55555555555554#53", Some(-1));
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", Some(1));
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", Some(2));
    test("-3.0e120", "-0x1.0E+100#1", Some(401));
    test("-4.0e-121", "-0x1.0E-100#1", Some(-399));
    test(
        "-2.582249878086908589655919172003011874329705792829223512830659e120",
        "-0x1.00000000000000000000000000000000000000000000000000E+100#200",
        Some(401),
    );
    test(
        "-3.872591914849318272818030633286351847570219192048790865487763e-121",
        "-0x1.00000000000000000000000000000000000000000000000000E-100#200",
        Some(-399),
    );
    test("-too_big", "-0x4.0E+268435455#1", Some(1073741823));
    test("-too_small", "-0x1.0E-268435456#1", Some(-1073741823));
}

fn get_exponent_properties_helper(x: Float) {
    let exponent = x.get_exponent();
    assert_eq!(rug::Float::exact_from(&x).get_exp(), exponent);
    if let Some(exponent) = exponent {
        assert_eq!(x.sci_exponent() + 1, exponent);
        assert!(exponent <= Float::MAX_EXPONENT);
        assert!(exponent >= Float::MIN_EXPONENT);
    } else {
        assert!(!x.is_normal());
    }
}

#[test]
fn get_exponent_properties() {
    float_gen().test_properties(|x| {
        get_exponent_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        get_exponent_properties_helper(x);
    });
}

#[test]
fn test_get_prec() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let exponent = x.get_prec();
        assert_eq!(exponent, out);
        if x.is_normal() {
            assert_eq!(Some(u64::from(rug::Float::exact_from(&x).prec())), out);
        }
    };
    test("NaN", "NaN", None);
    test("Infinity", "Infinity", None);
    test("-Infinity", "-Infinity", None);
    test("0.0", "0x0.0", None);
    test("-0.0", "-0x0.0", None);

    test("1.0", "0x1.0#1", Some(1));
    test("2.0", "0x2.0#1", Some(1));
    test("0.5", "0x0.8#1", Some(1));
    test("0.33333333333333331", "0x0.55555555555554#53", Some(53));
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", Some(53));
    test("3.1415926535897931", "0x3.243f6a8885a30#53", Some(53));
    test("3.0e120", "0x1.0E+100#1", Some(1));
    test("4.0e-121", "0x1.0E-100#1", Some(1));
    test(
        "2.582249878086908589655919172003011874329705792829223512830659e120",
        "0x1.00000000000000000000000000000000000000000000000000E+100#200",
        Some(200),
    );
    test(
        "3.872591914849318272818030633286351847570219192048790865487763e-121",
        "0x1.00000000000000000000000000000000000000000000000000E-100#200",
        Some(200),
    );
    test("too_big", "0x4.0E+268435455#1", Some(1));
    test("too_small", "0x1.0E-268435456#1", Some(1));

    test("-1.0", "-0x1.0#1", Some(1));
    test("-2.0", "-0x2.0#1", Some(1));
    test("-0.5", "-0x0.8#1", Some(1));
    test("-0.33333333333333331", "-0x0.55555555555554#53", Some(53));
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", Some(53));
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", Some(53));
    test("-3.0e120", "-0x1.0E+100#1", Some(1));
    test("-4.0e-121", "-0x1.0E-100#1", Some(1));
    test(
        "-2.582249878086908589655919172003011874329705792829223512830659e120",
        "-0x1.00000000000000000000000000000000000000000000000000E+100#200",
        Some(200),
    );
    test(
        "-3.872591914849318272818030633286351847570219192048790865487763e-121",
        "-0x1.00000000000000000000000000000000000000000000000000E-100#200",
        Some(200),
    );
    test("-too_big", "-0x4.0E+268435455#1", Some(1));
    test("-too_small", "-0x1.0E-268435456#1", Some(1));
}

#[allow(clippy::needless_pass_by_value)]
fn get_prec_properties_helper(x: Float) {
    x.get_prec().map_or_else(
        || {
            assert!(!x.is_normal());
        },
        |precision| {
            assert_eq!(u64::from(rug::Float::exact_from(&x).prec()), precision);
        },
    );
}

#[test]
fn get_prec_properties() {
    float_gen().test_properties(|x| {
        get_prec_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        get_prec_properties_helper(x);
    });
}

#[test]
fn test_get_min_prec() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let exponent = x.get_min_prec();
        assert_eq!(exponent, out);
    };
    test("NaN", "NaN", None);
    test("Infinity", "Infinity", None);
    test("-Infinity", "-Infinity", None);
    test("0.0", "0x0.0", None);
    test("-0.0", "-0x0.0", None);

    test("1.0", "0x1.0#1", Some(1));
    test("2.0", "0x2.0#1", Some(1));
    test("0.5", "0x0.8#1", Some(1));
    test("0.33333333333333331", "0x0.55555555555554#53", Some(53));
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", Some(53));
    test("3.1415926535897931", "0x3.243f6a8885a30#53", Some(50));
    test("3.0e120", "0x1.0E+100#1", Some(1));
    test("4.0e-121", "0x1.0E-100#1", Some(1));
    test(
        "2.582249878086908589655919172003011874329705792829223512830659e120",
        "0x1.00000000000000000000000000000000000000000000000000E+100#200",
        Some(1),
    );
    test(
        "3.872591914849318272818030633286351847570219192048790865487763e-121",
        "0x1.00000000000000000000000000000000000000000000000000E-100#200",
        Some(1),
    );
    test("too_big", "0x4.0E+268435455#1", Some(1));
    test("too_small", "0x1.0E-268435456#1", Some(1));

    test("-1.0", "-0x1.0#1", Some(1));
    test("-2.0", "-0x2.0#1", Some(1));
    test("-0.5", "-0x0.8#1", Some(1));
    test("-0.33333333333333331", "-0x0.55555555555554#53", Some(53));
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", Some(53));
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", Some(50));
    test("-3.0e120", "-0x1.0E+100#1", Some(1));
    test("-4.0e-121", "-0x1.0E-100#1", Some(1));
    test(
        "-2.582249878086908589655919172003011874329705792829223512830659e120",
        "-0x1.00000000000000000000000000000000000000000000000000E+100#200",
        Some(1),
    );
    test(
        "-3.872591914849318272818030633286351847570219192048790865487763e-121",
        "-0x1.00000000000000000000000000000000000000000000000000E-100#200",
        Some(1),
    );
    test("-too_big", "-0x4.0E+268435455#1", Some(1));
    test("-too_small", "-0x1.0E-268435456#1", Some(1));
}

#[allow(clippy::needless_pass_by_value)]
fn get_min_prec_properties_helper(x: Float) {
    x.get_min_prec().map_or_else(
        || {
            assert!(!x.is_normal());
        },
        |min_prec| {
            assert!(min_prec <= x.get_prec().unwrap());
        },
    );
}

#[test]
fn get_min_prec_properties() {
    float_gen().test_properties(|x| {
        get_min_prec_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        get_min_prec_properties_helper(x);
    });
}

#[test]
fn test_set_prec_round() {
    let test = |s, s_hex, prec, rm, out, out_hex, o| {
        let mut x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let old_x = x.clone();
        assert_eq!(x.set_prec_round(prec, rm), o);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let (x_alt, o_alt) = Float::from_float_prec_round(old_x.clone(), prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = Float::from_float_prec_round_ref(&old_x, prec, rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        if rm != Exact {
            let mut rug_x = rug::Float::exact_from(&old_x);
            assert_eq!(
                rug_x.set_prec_round(
                    u32::exact_from(prec),
                    rug_round_exact_from_rounding_mode(rm)
                ),
                o
            );
            assert_eq!(ComparableFloat(x), ComparableFloat(Float::from(&rug_x)));
        }
    };
    test("NaN", "NaN", 100, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 100, Exact, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 100, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 100, Exact, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        100,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        100,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", 100, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 100, Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 100, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 100, Exact, "-0.0", "-0x0.0", Equal);

    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Floor,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Ceiling,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Down,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Up,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Nearest,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Exact,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "3.141",
        "0x3.24#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "3.145",
        "0x3.25#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "3.141",
        "0x3.24#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "3.145",
        "0x3.25#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "3.141",
        "0x3.24#10",
        Less,
    );

    test(
        "too_big",
        "0x6.0E+268435455#2",
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "too_big",
        "0x6.0E+268435455#2",
        1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "too_big",
        "0x6.0E+268435455#2",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "too_big",
        "0x6.0E+268435455#2",
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    // It doesn't really make sense that rounding with `Nearest` could overflow, but this matches
    // the behavior of MPFR.
    test(
        "too_big",
        "0x6.0E+268435455#2",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "too_small",
        "0x1.8E-268435456#2",
        1,
        Floor,
        "too_small",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "too_small",
        "0x1.8E-268435456#2",
        1,
        Down,
        "too_small",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "too_small",
        "0x1.8E-268435456#2",
        1,
        Ceiling,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );
    test(
        "too_small",
        "0x1.8E-268435456#2",
        1,
        Up,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );
    test(
        "too_small",
        "0x1.8E-268435456#2",
        1,
        Nearest,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Floor,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Ceiling,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Down,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Up,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Nearest,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Exact,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Floor,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Floor,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Down,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Up,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Nearest,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Exact,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "-3.145",
        "-0x3.25#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "-3.145",
        "-0x3.25#10",
        Less,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    test(
        "-too_big",
        "-0x6.0E+268435455#2",
        1,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test(
        "-too_big",
        "-0x6.0E+268435455#2",
        1,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test(
        "-too_big",
        "-0x6.0E+268435455#2",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-too_big",
        "-0x6.0E+268435455#2",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    // It doesn't really make sense that rounding with `Nearest` could overflow, but this matches
    // the behavior of MPFR.
    test(
        "-too_big",
        "-0x6.0E+268435455#2",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-too_small",
        "-0x1.8E-268435456#2",
        1,
        Ceiling,
        "-too_small",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-too_small",
        "-0x1.8E-268435456#2",
        1,
        Down,
        "-too_small",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-too_small",
        "-0x1.8E-268435456#2",
        1,
        Floor,
        "-too_small",
        "-0x2.0E-268435456#1",
        Less,
    );
    test(
        "-too_small",
        "-0x1.8E-268435456#2",
        1,
        Up,
        "-too_small",
        "-0x2.0E-268435456#1",
        Less,
    );
    test(
        "-too_small",
        "-0x1.8E-268435456#2",
        1,
        Nearest,
        "-too_small",
        "-0x2.0E-268435456#1",
        Less,
    );
}

#[test]
#[should_panic]
fn set_prec_round_fail() {
    let mut x = Float::from(std::f64::consts::PI);
    x.set_prec_round(10, Exact);
}

fn set_prec_round_properties_helper(mut x: Float, prec: u64, rm: RoundingMode) {
    let old_x = x.clone();
    let o = x.set_prec_round(prec, rm);
    assert!(x.is_valid());
    let final_x = x.clone();
    if x.is_normal() {
        assert_eq!(x.get_prec(), Some(prec));
        assert_eq!(x.partial_cmp(&old_x), Some(o));
    } else {
        assert!(o != if x > 0 { Less } else { Greater });
    }

    let (x_alt, o_alt) = Float::from_float_prec_round(old_x.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = Float::from_float_prec_round_ref(&old_x, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    if rm == Exact {
        assert_eq!(o, Equal);
    } else {
        let mut rug_x = rug::Float::exact_from(&old_x);
        assert_eq!(
            rug_x.set_prec_round(
                u32::exact_from(prec),
                rug_round_exact_from_rounding_mode(rm)
            ),
            o
        );
        assert_eq!(
            ComparableFloatRef(&x),
            ComparableFloatRef(&Float::from(&rug_x))
        );
    }

    if o == Equal {
        if let Some(old_precision) = old_x.get_prec() {
            assert_eq!(x.set_prec_round(old_precision, Exact), Equal);
            assert_eq!(ComparableFloatRef(&x), ComparableFloatRef(&old_x));
        }
    }

    let mut x = -old_x;
    x.set_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(x), ComparableFloat(-final_x));
}

#[test]
fn set_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        set_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_4().test_properties(|(x, prec, rm)| {
        set_prec_round_properties_helper(x, prec, rm);
    });
}

#[test]
fn test_set_prec() {
    let test = |s, s_hex, prec, out, out_hex, o| {
        let mut x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let old_x = x.clone();
        assert_eq!(x.set_prec(prec), o);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);

        let (x_alt, o_alt) = Float::from_float_prec(old_x.clone(), prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let (x_alt, o_alt) = Float::from_float_prec_ref(&old_x, prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
        assert_eq!(o_alt, o);

        let mut rug_x = rug::Float::exact_from(&old_x);
        rug_x.set_prec(u32::exact_from(prec));
        assert_eq!(ComparableFloat(x), ComparableFloat(Float::from(&rug_x)));
    };
    test("NaN", "NaN", 100, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 100, "Infinity", "Infinity", Equal);
    test(
        "-Infinity",
        "-Infinity",
        100,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", 100, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 100, "-0.0", "-0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        "3.141592653589793115997963468544",
        "0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        10,
        "0.3335",
        "0x0.556#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        10,
        "1.414",
        "0x1.6a0#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "3.141",
        "0x3.24#10",
        Less,
    );
    // It doesn't really make sense that rounding with `Nearest` could overflow, but this matches
    // the behavior of MPFR.
    test(
        "too_big",
        "0x6.0E+268435455#2",
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "too_small",
        "0x1.8E-268435456#2",
        1,
        "too_small",
        "0x2.0E-268435456#1",
        Greater,
    );

    test(
        "-1.0",
        "-0x1.0#1",
        100,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        "-3.141592653589793115997963468544",
        "-0x3.243f6a8885a30000000000000#100",
        Equal,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        10,
        "-0.3335",
        "-0x0.556#10",
        Less,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        10,
        "-1.414",
        "-0x1.6a0#10",
        Greater,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-3.141",
        "-0x3.24#10",
        Greater,
    );
    // It doesn't really make sense that rounding with `Nearest` could overflow, but this matches
    // the behavior of MPFR.
    test(
        "-too_big",
        "-0x6.0E+268435455#2",
        1,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-too_small",
        "-0x1.8E-268435456#2",
        1,
        "-too_small",
        "-0x2.0E-268435456#1",
        Less,
    );
}

fn set_prec_properties_helper(mut x: Float, prec: u64) {
    let old_x = x.clone();
    let o = x.set_prec(prec);
    let final_x = x.clone();
    assert!(x.is_valid());
    if x.is_normal() {
        assert_eq!(x.get_prec(), Some(prec));
        assert_eq!(x.partial_cmp(&old_x), Some(o));
    } else {
        assert!(o != if x > 0 { Less } else { Greater });
    }

    let (x_alt, o_alt) = Float::from_float_prec(old_x.clone(), prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let (x_alt, o_alt) = Float::from_float_prec_ref(&old_x, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&x));
    assert_eq!(o_alt, o);

    let mut rug_x = rug::Float::exact_from(&old_x);
    rug_x.set_prec(u32::exact_from(prec));
    assert_eq!(
        ComparableFloatRef(&x),
        ComparableFloatRef(&Float::from(&rug_x))
    );

    if o == Equal {
        if let Some(old_precision) = old_x.get_prec() {
            assert_eq!(x.set_prec_round(old_precision, Exact), Equal);
            assert_eq!(ComparableFloatRef(&x), ComparableFloatRef(&old_x));
        }
    }

    let mut x = -old_x;
    x.set_prec(prec);
    assert_eq!(ComparableFloat(x), ComparableFloat(-final_x));
}

#[test]
fn set_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        set_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        set_prec_properties_helper(x, prec);
    });
}
