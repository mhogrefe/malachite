// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_base::{apply_fn_to_signeds, apply_fn_to_unsigneds};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{float_signed_pair_gen, float_unsigned_pair_gen};
use malachite_float::Float;
use rug;

#[test]
fn test_partial_eq_u32() {
    let test = |s, s_hex, v: u32, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        let ru = rug::Float::exact_from(&u);
        assert_eq!(u == v, out);
        assert_eq!(ru == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == ru, out);
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
    test("-1.0", "-0x1.0#1", 1, false);
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
fn test_partial_eq_u64() {
    let test = |s, s_hex, v: u64, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        let ru = rug::Float::exact_from(&u);
        assert_eq!(u == v, out);
        assert_eq!(ru == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == ru, out);
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
    test("-1.0", "-0x1.0#1", 1, false);
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
fn test_partial_eq_i32() {
    let test = |s, s_hex, v: i32, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        let ru = rug::Float::exact_from(&u);
        assert_eq!(u == v, out);
        assert_eq!(ru == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == ru, out);
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
    test("-1.0", "-0x1.0#1", 1, false);
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
    test("1.0", "0x1.0#1", -1, false);
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
fn test_partial_eq_i64() {
    let test = |s, s_hex, v: i64, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        let ru = rug::Float::exact_from(&u);
        assert_eq!(u == v, out);
        assert_eq!(ru == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == ru, out);
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
    test("-1.0", "-0x1.0#1", 1, false);
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
    test("1.0", "0x1.0#1", -1, false);
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

#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_primitive_int_properties_helper_unsigned<
    T: PartialEq<Float> + PartialEq<rug::Float> + PrimitiveUnsigned,
>()
where
    Float: From<T> + PartialEq<T>,
    rug::Float: PartialEq<T>,
{
    float_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let eq = n == u;
        assert_eq!(rug::Float::exact_from(&n) == u, eq);
        assert_eq!(&n == &Float::from(u), eq);

        assert_eq!(u == n, eq);
        assert_eq!(u == rug::Float::exact_from(&n), eq);
        assert_eq!(&Float::from(u) == &n, eq);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x) == y, x == y);
        assert_eq!(x == Float::from(y), x == y);
    });
}

// Extra refs necessary for type inference
#[allow(clippy::cmp_owned, clippy::op_ref)]
fn partial_eq_primitive_int_properties_helper_signed<
    T: PartialEq<Float> + PartialEq<rug::Float> + PrimitiveSigned,
>()
where
    Float: From<T> + PartialEq<T>,
    rug::Float: PartialEq<T>,
{
    float_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let eq = n == i;
        assert_eq!(rug::Float::exact_from(&n) == i, eq);
        assert_eq!(&n == &Float::from(i), eq);

        assert_eq!(i == n, eq);
        assert_eq!(i == rug::Float::exact_from(&n), eq);
        assert_eq!(&Float::from(i) == &n, eq);
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x) == y, x == y);
        assert_eq!(x == Float::from(y), x == y);
    });
}

#[test]
fn partial_eq_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_eq_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_eq_primitive_int_properties_helper_signed);
}
