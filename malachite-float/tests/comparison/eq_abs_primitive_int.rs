// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::test_util::generators::{
    signed_gen, signed_pair_gen, unsigned_gen, unsigned_pair_gen_var_27,
};
use malachite_base::{apply_fn_to_signeds, apply_fn_to_unsigneds};
use malachite_float::Float;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_signed_pair_gen, float_signed_pair_gen_var_4, float_unsigned_pair_gen,
    float_unsigned_pair_gen_var_5,
};

#[test]
fn test_eq_abs_u32() {
    let test = |s, s_hex, v: u32, eq: bool| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!((&u).abs() == v, eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("NaN", "NaN", 0, false);
    test("Infinity", "Infinity", 0, false);
    test("-Infinity", "-Infinity", 0, false);
    test("0.0", "0x0.0", 0, true);
    test("-0.0", "-0x0.0", 0, true);
    test("1.0", "0x1.0#1", 0, false);
    test("2.0", "0x2.0#1", 0, false);
    test("0.5", "0x0.8#1", 0, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 0, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 0, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 0, false);
    test("3.0e120", "0x1.0E+100#1", 0, false);
    test("4.0e-121", "0x1.0E-100#1", 0, false);
    test("-1.0", "-0x1.0#1", 0, false);
    test("-2.0", "-0x2.0#1", 0, false);
    test("-0.5", "-0x0.8#1", 0, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 0, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 0, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 0, false);
    test("-3.0e120", "-0x1.0E+100#1", 0, false);
    test("-4.0e-121", "-0x1.0E-100#1", 0, false);

    test("NaN", "NaN", 1, false);
    test("Infinity", "Infinity", 1, false);
    test("-Infinity", "-Infinity", 1, false);
    test("0.0", "0x0.0", 1, false);
    test("-0.0", "-0x0.0", 1, false);
    test("1.0", "0x1.0#1", 1, true);
    test("2.0", "0x2.0#1", 1, false);
    test("0.5", "0x0.8#1", 1, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 1, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 1, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 1, false);
    test("3.0e120", "0x1.0E+100#1", 1, false);
    test("4.0e-121", "0x1.0E-100#1", 1, false);
    test("-1.0", "-0x1.0#1", 1, true);
    test("-2.0", "-0x2.0#1", 1, false);
    test("-0.5", "-0x0.8#1", 1, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 1, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 1, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 1, false);
    test("-3.0e120", "-0x1.0E+100#1", 1, false);
    test("-4.0e-121", "-0x1.0E-100#1", 1, false);

    test("NaN", "NaN", 100, false);
    test("Infinity", "Infinity", 100, false);
    test("-Infinity", "-Infinity", 100, false);
    test("0.0", "0x0.0", 100, false);
    test("-0.0", "-0x0.0", 100, false);
    test("1.0", "0x1.0#1", 100, false);
    test("2.0", "0x2.0#1", 100, false);
    test("0.5", "0x0.8#1", 100, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 100, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 100, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 100, false);
    test("3.0e120", "0x1.0E+100#1", 100, false);
    test("4.0e-121", "0x1.0E-100#1", 100, false);
    test("-1.0", "-0x1.0#1", 100, false);
    test("-2.0", "-0x2.0#1", 100, false);
    test("-0.5", "-0x0.8#1", 100, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 100, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 100, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 100, false);
    test("-3.0e120", "-0x1.0E+100#1", 100, false);
    test("-4.0e-121", "-0x1.0E-100#1", 100, false);
}

#[test]
fn test_eq_abs_u64() {
    let test = |s, s_hex, v: u64, eq: bool| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!((&u).abs() == v, eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("NaN", "NaN", 0, false);
    test("Infinity", "Infinity", 0, false);
    test("-Infinity", "-Infinity", 0, false);
    test("0.0", "0x0.0", 0, true);
    test("-0.0", "-0x0.0", 0, true);
    test("1.0", "0x1.0#1", 0, false);
    test("2.0", "0x2.0#1", 0, false);
    test("0.5", "0x0.8#1", 0, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 0, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 0, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 0, false);
    test("3.0e120", "0x1.0E+100#1", 0, false);
    test("4.0e-121", "0x1.0E-100#1", 0, false);
    test("-1.0", "-0x1.0#1", 0, false);
    test("-2.0", "-0x2.0#1", 0, false);
    test("-0.5", "-0x0.8#1", 0, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 0, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 0, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 0, false);
    test("-3.0e120", "-0x1.0E+100#1", 0, false);
    test("-4.0e-121", "-0x1.0E-100#1", 0, false);

    test("NaN", "NaN", 1, false);
    test("Infinity", "Infinity", 1, false);
    test("-Infinity", "-Infinity", 1, false);
    test("0.0", "0x0.0", 1, false);
    test("-0.0", "-0x0.0", 1, false);
    test("1.0", "0x1.0#1", 1, true);
    test("2.0", "0x2.0#1", 1, false);
    test("0.5", "0x0.8#1", 1, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 1, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 1, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 1, false);
    test("3.0e120", "0x1.0E+100#1", 1, false);
    test("4.0e-121", "0x1.0E-100#1", 1, false);
    test("-1.0", "-0x1.0#1", 1, true);
    test("-2.0", "-0x2.0#1", 1, false);
    test("-0.5", "-0x0.8#1", 1, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 1, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 1, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 1, false);
    test("-3.0e120", "-0x1.0E+100#1", 1, false);
    test("-4.0e-121", "-0x1.0E-100#1", 1, false);

    test("NaN", "NaN", 100, false);
    test("Infinity", "Infinity", 100, false);
    test("-Infinity", "-Infinity", 100, false);
    test("0.0", "0x0.0", 100, false);
    test("-0.0", "-0x0.0", 100, false);
    test("1.0", "0x1.0#1", 100, false);
    test("2.0", "0x2.0#1", 100, false);
    test("0.5", "0x0.8#1", 100, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 100, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 100, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 100, false);
    test("3.0e120", "0x1.0E+100#1", 100, false);
    test("4.0e-121", "0x1.0E-100#1", 100, false);
    test("-1.0", "-0x1.0#1", 100, false);
    test("-2.0", "-0x2.0#1", 100, false);
    test("-0.5", "-0x0.8#1", 100, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 100, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 100, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 100, false);
    test("-3.0e120", "-0x1.0E+100#1", 100, false);
    test("-4.0e-121", "-0x1.0E-100#1", 100, false);
}

#[test]
fn test_eq_abs_i32() {
    let test = |s, s_hex, v: i32, eq: bool| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!((&u).abs() == v.unsigned_abs(), eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("NaN", "NaN", 0, false);
    test("Infinity", "Infinity", 0, false);
    test("-Infinity", "-Infinity", 0, false);
    test("0.0", "0x0.0", 0, true);
    test("-0.0", "-0x0.0", 0, true);
    test("1.0", "0x1.0#1", 0, false);
    test("2.0", "0x2.0#1", 0, false);
    test("0.5", "0x0.8#1", 0, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 0, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 0, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 0, false);
    test("3.0e120", "0x1.0E+100#1", 0, false);
    test("4.0e-121", "0x1.0E-100#1", 0, false);
    test("-1.0", "-0x1.0#1", 0, false);
    test("-2.0", "-0x2.0#1", 0, false);
    test("-0.5", "-0x0.8#1", 0, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 0, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 0, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 0, false);
    test("-3.0e120", "-0x1.0E+100#1", 0, false);
    test("-4.0e-121", "-0x1.0E-100#1", 0, false);

    test("NaN", "NaN", 1, false);
    test("Infinity", "Infinity", 1, false);
    test("-Infinity", "-Infinity", 1, false);
    test("0.0", "0x0.0", 1, false);
    test("-0.0", "-0x0.0", 1, false);
    test("1.0", "0x1.0#1", 1, true);
    test("2.0", "0x2.0#1", 1, false);
    test("0.5", "0x0.8#1", 1, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 1, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 1, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 1, false);
    test("3.0e120", "0x1.0E+100#1", 1, false);
    test("4.0e-121", "0x1.0E-100#1", 1, false);
    test("-1.0", "-0x1.0#1", 1, true);
    test("-2.0", "-0x2.0#1", 1, false);
    test("-0.5", "-0x0.8#1", 1, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 1, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 1, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 1, false);
    test("-3.0e120", "-0x1.0E+100#1", 1, false);
    test("-4.0e-121", "-0x1.0E-100#1", 1, false);

    test("NaN", "NaN", 100, false);
    test("Infinity", "Infinity", 100, false);
    test("-Infinity", "-Infinity", 100, false);
    test("0.0", "0x0.0", 100, false);
    test("-0.0", "-0x0.0", 100, false);
    test("1.0", "0x1.0#1", 100, false);
    test("2.0", "0x2.0#1", 100, false);
    test("0.5", "0x0.8#1", 100, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 100, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 100, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 100, false);
    test("3.0e120", "0x1.0E+100#1", 100, false);
    test("4.0e-121", "0x1.0E-100#1", 100, false);
    test("-1.0", "-0x1.0#1", 100, false);
    test("-2.0", "-0x2.0#1", 100, false);
    test("-0.5", "-0x0.8#1", 100, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 100, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 100, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 100, false);
    test("-3.0e120", "-0x1.0E+100#1", 100, false);
    test("-4.0e-121", "-0x1.0E-100#1", 100, false);

    test("NaN", "NaN", -1, false);
    test("Infinity", "Infinity", -1, false);
    test("-Infinity", "-Infinity", -1, false);
    test("0.0", "0x0.0", -1, false);
    test("-0.0", "-0x0.0", -1, false);
    test("1.0", "0x1.0#1", -1, true);
    test("2.0", "0x2.0#1", -1, false);
    test("0.5", "0x0.8#1", -1, false);
    test("0.33333333333333331", "0x0.55555555555554#53", -1, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", -1, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", -1, false);
    test("3.0e120", "0x1.0E+100#1", -1, false);
    test("4.0e-121", "0x1.0E-100#1", -1, false);
    test("-1.0", "-0x1.0#1", -1, true);
    test("-2.0", "-0x2.0#1", -1, false);
    test("-0.5", "-0x0.8#1", -1, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", -1, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", -1, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", -1, false);
    test("-3.0e120", "-0x1.0E+100#1", -1, false);
    test("-4.0e-121", "-0x1.0E-100#1", -1, false);

    test("NaN", "NaN", -100, false);
    test("Infinity", "Infinity", -100, false);
    test("-Infinity", "-Infinity", -100, false);
    test("0.0", "0x0.0", -100, false);
    test("-0.0", "-0x0.0", -100, false);
    test("1.0", "0x1.0#1", -100, false);
    test("2.0", "0x2.0#1", -100, false);
    test("0.5", "0x0.8#1", -100, false);
    test("0.33333333333333331", "0x0.55555555555554#53", -100, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", -100, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", -100, false);
    test("3.0e120", "0x1.0E+100#1", -100, false);
    test("4.0e-121", "0x1.0E-100#1", -100, false);
    test("-1.0", "-0x1.0#1", -100, false);
    test("-2.0", "-0x2.0#1", -100, false);
    test("-0.5", "-0x0.8#1", -100, false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -100,
        false,
    );
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", -100, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", -100, false);
    test("-3.0e120", "-0x1.0E+100#1", -100, false);
    test("-4.0e-121", "-0x1.0E-100#1", -100, false);
}

#[test]
fn test_eq_abs_i64() {
    let test = |s, s_hex, v: i64, eq: bool| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.eq_abs(&v), eq);
        assert_eq!((&u).abs() == v.unsigned_abs(), eq);
        assert_eq!(u.ne_abs(&v), !eq);
        assert_eq!(v.eq_abs(&u), eq);
    };
    test("NaN", "NaN", 0, false);
    test("Infinity", "Infinity", 0, false);
    test("-Infinity", "-Infinity", 0, false);
    test("0.0", "0x0.0", 0, true);
    test("-0.0", "-0x0.0", 0, true);
    test("1.0", "0x1.0#1", 0, false);
    test("2.0", "0x2.0#1", 0, false);
    test("0.5", "0x0.8#1", 0, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 0, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 0, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 0, false);
    test("3.0e120", "0x1.0E+100#1", 0, false);
    test("4.0e-121", "0x1.0E-100#1", 0, false);
    test("-1.0", "-0x1.0#1", 0, false);
    test("-2.0", "-0x2.0#1", 0, false);
    test("-0.5", "-0x0.8#1", 0, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 0, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 0, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 0, false);
    test("-3.0e120", "-0x1.0E+100#1", 0, false);
    test("-4.0e-121", "-0x1.0E-100#1", 0, false);

    test("NaN", "NaN", 1, false);
    test("Infinity", "Infinity", 1, false);
    test("-Infinity", "-Infinity", 1, false);
    test("0.0", "0x0.0", 1, false);
    test("-0.0", "-0x0.0", 1, false);
    test("1.0", "0x1.0#1", 1, true);
    test("2.0", "0x2.0#1", 1, false);
    test("0.5", "0x0.8#1", 1, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 1, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 1, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 1, false);
    test("3.0e120", "0x1.0E+100#1", 1, false);
    test("4.0e-121", "0x1.0E-100#1", 1, false);
    test("-1.0", "-0x1.0#1", 1, true);
    test("-2.0", "-0x2.0#1", 1, false);
    test("-0.5", "-0x0.8#1", 1, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 1, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 1, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 1, false);
    test("-3.0e120", "-0x1.0E+100#1", 1, false);
    test("-4.0e-121", "-0x1.0E-100#1", 1, false);

    test("NaN", "NaN", 100, false);
    test("Infinity", "Infinity", 100, false);
    test("-Infinity", "-Infinity", 100, false);
    test("0.0", "0x0.0", 100, false);
    test("-0.0", "-0x0.0", 100, false);
    test("1.0", "0x1.0#1", 100, false);
    test("2.0", "0x2.0#1", 100, false);
    test("0.5", "0x0.8#1", 100, false);
    test("0.33333333333333331", "0x0.55555555555554#53", 100, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 100, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 100, false);
    test("3.0e120", "0x1.0E+100#1", 100, false);
    test("4.0e-121", "0x1.0E-100#1", 100, false);
    test("-1.0", "-0x1.0#1", 100, false);
    test("-2.0", "-0x2.0#1", 100, false);
    test("-0.5", "-0x0.8#1", 100, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 100, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 100, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 100, false);
    test("-3.0e120", "-0x1.0E+100#1", 100, false);
    test("-4.0e-121", "-0x1.0E-100#1", 100, false);

    test("NaN", "NaN", -1, false);
    test("Infinity", "Infinity", -1, false);
    test("-Infinity", "-Infinity", -1, false);
    test("0.0", "0x0.0", -1, false);
    test("-0.0", "-0x0.0", -1, false);
    test("1.0", "0x1.0#1", -1, true);
    test("2.0", "0x2.0#1", -1, false);
    test("0.5", "0x0.8#1", -1, false);
    test("0.33333333333333331", "0x0.55555555555554#53", -1, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", -1, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", -1, false);
    test("3.0e120", "0x1.0E+100#1", -1, false);
    test("4.0e-121", "0x1.0E-100#1", -1, false);
    test("-1.0", "-0x1.0#1", -1, true);
    test("-2.0", "-0x2.0#1", -1, false);
    test("-0.5", "-0x0.8#1", -1, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", -1, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", -1, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", -1, false);
    test("-3.0e120", "-0x1.0E+100#1", -1, false);
    test("-4.0e-121", "-0x1.0E-100#1", -1, false);

    test("NaN", "NaN", -100, false);
    test("Infinity", "Infinity", -100, false);
    test("-Infinity", "-Infinity", -100, false);
    test("0.0", "0x0.0", -100, false);
    test("-0.0", "-0x0.0", -100, false);
    test("1.0", "0x1.0#1", -100, false);
    test("2.0", "0x2.0#1", -100, false);
    test("0.5", "0x0.8#1", -100, false);
    test("0.33333333333333331", "0x0.55555555555554#53", -100, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", -100, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", -100, false);
    test("3.0e120", "0x1.0E+100#1", -100, false);
    test("4.0e-121", "0x1.0E-100#1", -100, false);
    test("-1.0", "-0x1.0#1", -100, false);
    test("-2.0", "-0x2.0#1", -100, false);
    test("-0.5", "-0x0.8#1", -100, false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -100,
        false,
    );
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", -100, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", -100, false);
    test("-3.0e120", "-0x1.0E+100#1", -100, false);
    test("-4.0e-121", "-0x1.0E-100#1", -100, false);
}

#[allow(clippy::needless_pass_by_value, clippy::cmp_owned, clippy::op_ref)]
fn eq_abs_primitive_int_properties_helper_unsigned_helper<T: EqAbs<Float> + PrimitiveUnsigned>(
    n: Float,
    u: T,
) where
    Float: From<T> + PartialEq<T> + EqAbs<T>,
{
    let eq = n.eq_abs(&u);
    assert_eq!(EqAbs::<Float>::eq_abs(&n, &Float::from(u)), eq);
    assert_eq!((&n).abs() == u, eq);
    assert_eq!(n.ne_abs(&u), !eq);

    assert_eq!(u.eq_abs(&n), eq);
    assert_eq!(EqAbs::<Float>::eq_abs(&Float::from(u), &n), eq);
}

#[allow(clippy::cmp_owned, clippy::op_ref)]
fn eq_abs_primitive_int_properties_helper_unsigned<T: EqAbs<Float> + PrimitiveUnsigned>()
where
    Float: From<T> + PartialEq<T> + EqAbs<T>,
{
    float_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        eq_abs_primitive_int_properties_helper_unsigned_helper(n, u);
    });

    float_unsigned_pair_gen_var_5::<T>().test_properties(|(n, u)| {
        eq_abs_primitive_int_properties_helper_unsigned_helper(n, u);
    });

    unsigned_gen::<T>().test_properties(|x| {
        assert!(x.ne_abs(&Float::NAN));
        assert!(x.ne_abs(&Float::INFINITY));
        assert!(x.ne_abs(&Float::NEGATIVE_INFINITY));
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x).eq_abs(&y), x == y);
        assert_eq!(x.eq_abs(&Float::from(y)), x == y);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn eq_abs_primitive_int_properties_helper_signed_helper<T: EqAbs<Float> + PrimitiveSigned>(
    n: Float,
    i: T,
) where
    Float: From<T> + PartialEq<<T as UnsignedAbs>::Output> + EqAbs<T>,
{
    let eq = n.eq_abs(&i);
    assert_eq!(EqAbs::<Float>::eq_abs(&n, &Float::from(i)), eq);
    assert_eq!((&n).abs() == i.unsigned_abs(), eq);
    assert_eq!(n.ne_abs(&i), !eq);

    assert_eq!(i.eq_abs(&n), eq);
    assert_eq!(EqAbs::<Float>::eq_abs(&Float::from(i), &n), eq);
}

fn eq_abs_primitive_int_properties_helper_signed<T: EqAbs<Float> + PrimitiveSigned>()
where
    Float: From<T> + PartialEq<<T as UnsignedAbs>::Output> + EqAbs<T>,
{
    float_signed_pair_gen::<T>().test_properties(|(n, i)| {
        eq_abs_primitive_int_properties_helper_signed_helper(n, i);
    });

    float_signed_pair_gen_var_4::<T>().test_properties(|(n, i)| {
        eq_abs_primitive_int_properties_helper_signed_helper(n, i);
    });

    signed_gen::<T>().test_properties(|x| {
        assert!(x.ne_abs(&Float::NAN));
        assert!(x.ne_abs(&Float::INFINITY));
        assert!(x.ne_abs(&Float::NEGATIVE_INFINITY));
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x).eq_abs(&y), x.eq_abs(&y));
        assert_eq!(x.eq_abs(&Float::from(y)), x.eq_abs(&y));
    });
}

#[test]
fn eq_abs_primitive_int_properties() {
    apply_fn_to_unsigneds!(eq_abs_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(eq_abs_primitive_int_properties_helper_signed);
}
