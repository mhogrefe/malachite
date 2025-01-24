// Copyright © 2025 Mikhail Hogrefe
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
use malachite_float::test_util::generators::{
    float_float_signed_triple_gen, float_float_unsigned_triple_gen, float_signed_pair_gen,
    float_signed_pair_gen_var_4, float_signed_signed_triple_gen, float_unsigned_pair_gen,
    float_unsigned_pair_gen_var_5, float_unsigned_unsigned_triple_gen,
};
use malachite_float::Float;
use rug;
use std::cmp::Ordering::{self, *};

#[test]
fn test_partial_cmp_u32() {
    let test = |s, s_hex, v: u32, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(rug::Float::exact_from(&u).partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u), out.map(Ordering::reverse));
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Less));
    test("0.0", "0x0.0", 0, Some(Equal));
    test("-0.0", "-0x0.0", 0, Some(Equal));
    test("1.0", "0x1.0#1", 0, Some(Greater));
    test("2.0", "0x2.0#1", 0, Some(Greater));
    test("0.5", "0x0.8#1", 0, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 0, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 0, Some(Greater));
    test("-1.0", "-0x1.0#1", 0, Some(Less));
    test("-2.0", "-0x2.0#1", 0, Some(Less));
    test("-0.5", "-0x0.8#1", 0, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Less));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Less));
    test("0.0", "0x0.0", 1, Some(Less));
    test("-0.0", "-0x0.0", 1, Some(Less));
    test("1.0", "0x1.0#1", 1, Some(Equal));
    test("2.0", "0x2.0#1", 1, Some(Greater));
    test("0.5", "0x0.8#1", 1, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 1, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 1, Some(Less));
    test("-1.0", "-0x1.0#1", 1, Some(Less));
    test("-2.0", "-0x2.0#1", 1, Some(Less));
    test("-0.5", "-0x0.8#1", 1, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Less));
    test("0.0", "0x0.0", 100, Some(Less));
    test("-0.0", "-0x0.0", 100, Some(Less));
    test("1.0", "0x1.0#1", 100, Some(Less));
    test("2.0", "0x2.0#1", 100, Some(Less));
    test("0.5", "0x0.8#1", 100, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("3.0e120", "0x1.0E+100#1", 100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 100, Some(Less));
    test("-1.0", "-0x1.0#1", 100, Some(Less));
    test("-2.0", "-0x2.0#1", 100, Some(Less));
    test("-0.5", "-0x0.8#1", 100, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));
}

#[test]
fn test_partial_cmp_u64() {
    let test = |s, s_hex, v: u64, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(rug::Float::exact_from(&u).partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u), out.map(Ordering::reverse));
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Less));
    test("0.0", "0x0.0", 0, Some(Equal));
    test("-0.0", "-0x0.0", 0, Some(Equal));
    test("1.0", "0x1.0#1", 0, Some(Greater));
    test("2.0", "0x2.0#1", 0, Some(Greater));
    test("0.5", "0x0.8#1", 0, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 0, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 0, Some(Greater));
    test("-1.0", "-0x1.0#1", 0, Some(Less));
    test("-2.0", "-0x2.0#1", 0, Some(Less));
    test("-0.5", "-0x0.8#1", 0, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Less));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Less));
    test("0.0", "0x0.0", 1, Some(Less));
    test("-0.0", "-0x0.0", 1, Some(Less));
    test("1.0", "0x1.0#1", 1, Some(Equal));
    test("2.0", "0x2.0#1", 1, Some(Greater));
    test("0.5", "0x0.8#1", 1, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 1, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 1, Some(Less));
    test("-1.0", "-0x1.0#1", 1, Some(Less));
    test("-2.0", "-0x2.0#1", 1, Some(Less));
    test("-0.5", "-0x0.8#1", 1, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Less));
    test("0.0", "0x0.0", 100, Some(Less));
    test("-0.0", "-0x0.0", 100, Some(Less));
    test("1.0", "0x1.0#1", 100, Some(Less));
    test("2.0", "0x2.0#1", 100, Some(Less));
    test("0.5", "0x0.8#1", 100, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("3.0e120", "0x1.0E+100#1", 100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 100, Some(Less));
    test("-1.0", "-0x1.0#1", 100, Some(Less));
    test("-2.0", "-0x2.0#1", 100, Some(Less));
    test("-0.5", "-0x0.8#1", 100, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));
}

#[test]
fn test_partial_cmp_i32() {
    let test = |s, s_hex, v: i32, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(rug::Float::exact_from(&u).partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u), out.map(Ordering::reverse));
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Less));
    test("0.0", "0x0.0", 0, Some(Equal));
    test("-0.0", "-0x0.0", 0, Some(Equal));
    test("1.0", "0x1.0#1", 0, Some(Greater));
    test("2.0", "0x2.0#1", 0, Some(Greater));
    test("0.5", "0x0.8#1", 0, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 0, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 0, Some(Greater));
    test("-1.0", "-0x1.0#1", 0, Some(Less));
    test("-2.0", "-0x2.0#1", 0, Some(Less));
    test("-0.5", "-0x0.8#1", 0, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Less));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Less));
    test("0.0", "0x0.0", 1, Some(Less));
    test("-0.0", "-0x0.0", 1, Some(Less));
    test("1.0", "0x1.0#1", 1, Some(Equal));
    test("2.0", "0x2.0#1", 1, Some(Greater));
    test("0.5", "0x0.8#1", 1, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 1, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 1, Some(Less));
    test("-1.0", "-0x1.0#1", 1, Some(Less));
    test("-2.0", "-0x2.0#1", 1, Some(Less));
    test("-0.5", "-0x0.8#1", 1, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Less));
    test("0.0", "0x0.0", 100, Some(Less));
    test("-0.0", "-0x0.0", 100, Some(Less));
    test("1.0", "0x1.0#1", 100, Some(Less));
    test("2.0", "0x2.0#1", 100, Some(Less));
    test("0.5", "0x0.8#1", 100, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("3.0e120", "0x1.0E+100#1", 100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 100, Some(Less));
    test("-1.0", "-0x1.0#1", 100, Some(Less));
    test("-2.0", "-0x2.0#1", 100, Some(Less));
    test("-0.5", "-0x0.8#1", 100, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));

    test("NaN", "NaN", -1, None);
    test("Infinity", "Infinity", -1, Some(Greater));
    test("-Infinity", "-Infinity", -1, Some(Less));
    test("0.0", "0x0.0", -1, Some(Greater));
    test("-0.0", "-0x0.0", -1, Some(Greater));
    test("1.0", "0x1.0#1", -1, Some(Greater));
    test("2.0", "0x2.0#1", -1, Some(Greater));
    test("0.5", "0x0.8#1", -1, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -1,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        -1,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -1,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", -1, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", -1, Some(Greater));
    test("-1.0", "-0x1.0#1", -1, Some(Equal));
    test("-2.0", "-0x2.0#1", -1, Some(Less));
    test("-0.5", "-0x0.8#1", -1, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -1,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -1,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -1,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", -1, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", -1, Some(Greater));

    test("NaN", "NaN", -100, None);
    test("Infinity", "Infinity", -100, Some(Greater));
    test("-Infinity", "-Infinity", -100, Some(Less));
    test("0.0", "0x0.0", -100, Some(Greater));
    test("-0.0", "-0x0.0", -100, Some(Greater));
    test("1.0", "0x1.0#1", -100, Some(Greater));
    test("2.0", "0x2.0#1", -100, Some(Greater));
    test("0.5", "0x0.8#1", -100, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -100,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        -100,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -100,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", -100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", -100, Some(Greater));
    test("-1.0", "-0x1.0#1", -100, Some(Greater));
    test("-2.0", "-0x2.0#1", -100, Some(Greater));
    test("-0.5", "-0x0.8#1", -100, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -100,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -100,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -100,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", -100, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", -100, Some(Greater));
}

#[test]
fn test_partial_cmp_i64() {
    let test = |s, s_hex, v: i64, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp(&v), out);
        assert_eq!(rug::Float::exact_from(&u).partial_cmp(&v), out);
        assert_eq!(v.partial_cmp(&u), out.map(Ordering::reverse));
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Less));
    test("0.0", "0x0.0", 0, Some(Equal));
    test("-0.0", "-0x0.0", 0, Some(Equal));
    test("1.0", "0x1.0#1", 0, Some(Greater));
    test("2.0", "0x2.0#1", 0, Some(Greater));
    test("0.5", "0x0.8#1", 0, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 0, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 0, Some(Greater));
    test("-1.0", "-0x1.0#1", 0, Some(Less));
    test("-2.0", "-0x2.0#1", 0, Some(Less));
    test("-0.5", "-0x0.8#1", 0, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Less));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Less));
    test("0.0", "0x0.0", 1, Some(Less));
    test("-0.0", "-0x0.0", 1, Some(Less));
    test("1.0", "0x1.0#1", 1, Some(Equal));
    test("2.0", "0x2.0#1", 1, Some(Greater));
    test("0.5", "0x0.8#1", 1, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        1,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", 1, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 1, Some(Less));
    test("-1.0", "-0x1.0#1", 1, Some(Less));
    test("-2.0", "-0x2.0#1", 1, Some(Less));
    test("-0.5", "-0x0.8#1", 1, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        1,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        1,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Less));
    test("0.0", "0x0.0", 100, Some(Less));
    test("-0.0", "-0x0.0", 100, Some(Less));
    test("1.0", "0x1.0#1", 100, Some(Less));
    test("2.0", "0x2.0#1", 100, Some(Less));
    test("0.5", "0x0.8#1", 100, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("3.0e120", "0x1.0E+100#1", 100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", 100, Some(Less));
    test("-1.0", "-0x1.0#1", 100, Some(Less));
    test("-2.0", "-0x2.0#1", 100, Some(Less));
    test("-0.5", "-0x0.8#1", 100, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        100,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        100,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));

    test("NaN", "NaN", -1, None);
    test("Infinity", "Infinity", -1, Some(Greater));
    test("-Infinity", "-Infinity", -1, Some(Less));
    test("0.0", "0x0.0", -1, Some(Greater));
    test("-0.0", "-0x0.0", -1, Some(Greater));
    test("1.0", "0x1.0#1", -1, Some(Greater));
    test("2.0", "0x2.0#1", -1, Some(Greater));
    test("0.5", "0x0.8#1", -1, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -1,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        -1,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -1,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", -1, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", -1, Some(Greater));
    test("-1.0", "-0x1.0#1", -1, Some(Equal));
    test("-2.0", "-0x2.0#1", -1, Some(Less));
    test("-0.5", "-0x0.8#1", -1, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -1,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -1,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -1,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", -1, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", -1, Some(Greater));

    test("NaN", "NaN", -100, None);
    test("Infinity", "Infinity", -100, Some(Greater));
    test("-Infinity", "-Infinity", -100, Some(Less));
    test("0.0", "0x0.0", -100, Some(Greater));
    test("-0.0", "-0x0.0", -100, Some(Greater));
    test("1.0", "0x1.0#1", -100, Some(Greater));
    test("2.0", "0x2.0#1", -100, Some(Greater));
    test("0.5", "0x0.8#1", -100, Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -100,
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        -100,
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -100,
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", -100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", -100, Some(Greater));
    test("-1.0", "-0x1.0#1", -100, Some(Greater));
    test("-2.0", "-0x2.0#1", -100, Some(Greater));
    test("-0.5", "-0x0.8#1", -100, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -100,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -100,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -100,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", -100, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", -100, Some(Greater));
}

#[allow(clippy::needless_pass_by_value)]
fn partial_cmp_primitive_int_properties_helper_unsigned_helper<
    T: PartialOrd<Float> + PartialOrd<rug::Float> + PrimitiveUnsigned,
>(
    n: Float,
    u: T,
) where
    Float: From<T> + PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    let cmp = n.partial_cmp(&u);
    assert_eq!(rug::Float::exact_from(&n).partial_cmp(&u), cmp);
    assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &Float::from(u)), cmp);

    let cmp_rev = cmp.map(Ordering::reverse);
    assert_eq!(u.partial_cmp(&n), cmp_rev);
    assert_eq!(u.partial_cmp(&rug::Float::exact_from(&n)), cmp_rev);
    assert_eq!(
        PartialOrd::<Float>::partial_cmp(&Float::from(u), &n),
        cmp_rev
    );
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_unsigned<
    T: PartialOrd<Float> + PartialOrd<rug::Float> + PrimitiveUnsigned,
>()
where
    Float: From<T> + PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    float_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        partial_cmp_primitive_int_properties_helper_unsigned_helper(n, u);
    });

    float_unsigned_pair_gen_var_5::<T>().test_properties(|(n, u)| {
        partial_cmp_primitive_int_properties_helper_unsigned_helper(n, u);
    });

    float_float_unsigned_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n < u && u < m {
            assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &m), Some(Less));
        } else if n > u && u > m {
            assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &m), Some(Greater));
        }
    });

    float_unsigned_unsigned_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u < n && n < v {
            assert!(u < v);
        } else if u > n && n > v {
            assert!(u > v);
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Float::from(y)), Some(x.cmp(&y)));
    });
}

#[allow(clippy::needless_pass_by_value)]
fn partial_cmp_primitive_int_properties_helper_signed_helper<
    T: PartialOrd<Float> + PartialOrd<rug::Float> + PrimitiveSigned,
>(
    n: Float,
    i: T,
) where
    Float: From<T> + PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    let cmp = n.partial_cmp(&i);
    assert_eq!(rug::Float::exact_from(&n).partial_cmp(&i), cmp);
    assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &Float::from(i)), cmp);

    let cmp_rev = cmp.map(Ordering::reverse);
    assert_eq!(i.partial_cmp(&n), cmp_rev);
    assert_eq!(i.partial_cmp(&rug::Float::exact_from(&n)), cmp_rev);
    assert_eq!(
        PartialOrd::<Float>::partial_cmp(&Float::from(i), &n),
        cmp_rev
    );
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_primitive_int_properties_helper_signed<
    T: PartialOrd<Float> + PartialOrd<rug::Float> + PrimitiveSigned,
>()
where
    Float: From<T> + PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    float_signed_pair_gen::<T>().test_properties(|(n, i)| {
        partial_cmp_primitive_int_properties_helper_signed_helper(n, i);
    });

    float_signed_pair_gen_var_4::<T>().test_properties(|(n, i)| {
        partial_cmp_primitive_int_properties_helper_signed_helper(n, i);
    });

    float_float_signed_triple_gen::<T>().test_properties(|(n, m, i)| {
        if n < i && i < m {
            assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &m), Some(Less));
        } else if n > i && i > m {
            assert_eq!(PartialOrd::<Float>::partial_cmp(&n, &m), Some(Greater));
        }
    });

    float_signed_signed_triple_gen::<T>().test_properties(|(n, i, j)| {
        if i < n && n < j {
            assert!(i < j);
        } else if i > n && n > j {
            assert!(i > j);
        }
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x).partial_cmp(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp(&Float::from(y)), Some(x.cmp(&y)));
    });
}

#[test]
fn partial_cmp_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_primitive_int_properties_helper_signed);
}
