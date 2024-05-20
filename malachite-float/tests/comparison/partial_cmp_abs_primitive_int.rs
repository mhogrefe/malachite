// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Abs, UnsignedAbs};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::{signed_pair_gen, unsigned_pair_gen_var_27};
use malachite_base::{apply_fn_to_signeds, apply_fn_to_unsigneds};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_float_signed_triple_gen, float_float_unsigned_triple_gen, float_signed_pair_gen,
    float_signed_signed_triple_gen, float_unsigned_pair_gen, float_unsigned_unsigned_triple_gen,
};
use malachite_float::Float;
use rug;
use std::cmp::Ordering::{self, *};

#[test]
fn test_partial_cmp_abs_u32() {
    let test = |s, s_hex, v: u32, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&u), out.map(Ordering::reverse));
        assert_eq!(rug::Float::exact_from(&u).abs().partial_cmp(&v), out);
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 0, Some(Greater));
    test("-2.0", "-0x2.0#1", 0, Some(Greater));
    test("-0.5", "-0x0.8#1", 0, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Greater));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 1, Some(Equal));
    test("-2.0", "-0x2.0#1", 1, Some(Greater));
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
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Greater));
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
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));
}

#[test]
fn test_partial_cmp_abs_u64() {
    let test = |s, s_hex, v: u64, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&u), out.map(Ordering::reverse));
        assert_eq!(rug::Float::exact_from(&u).abs().partial_cmp(&v), out);
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 0, Some(Greater));
    test("-2.0", "-0x2.0#1", 0, Some(Greater));
    test("-0.5", "-0x0.8#1", 0, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Greater));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 1, Some(Equal));
    test("-2.0", "-0x2.0#1", 1, Some(Greater));
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
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Greater));
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
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));
}

#[test]
fn test_partial_cmp_abs_i32() {
    let test = |s, s_hex, v: i32, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&u), out.map(Ordering::reverse));
        assert_eq!(
            rug::Float::exact_from(&u)
                .abs()
                .partial_cmp(&v.unsigned_abs()),
            out
        );
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 0, Some(Greater));
    test("-2.0", "-0x2.0#1", 0, Some(Greater));
    test("-0.5", "-0x0.8#1", 0, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Greater));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 1, Some(Equal));
    test("-2.0", "-0x2.0#1", 1, Some(Greater));
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
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Greater));
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
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));

    test("NaN", "NaN", -1, None);
    test("Infinity", "Infinity", -1, Some(Greater));
    test("-Infinity", "-Infinity", -1, Some(Greater));
    test("0.0", "0x0.0", -1, Some(Less));
    test("-0.0", "-0x0.0", -1, Some(Less));
    test("1.0", "0x1.0#1", -1, Some(Equal));
    test("2.0", "0x2.0#1", -1, Some(Greater));
    test("0.5", "0x0.8#1", -1, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -1,
        Some(Less),
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
    test("4.0e-121", "0x1.0E-100#1", -1, Some(Less));
    test("-1.0", "-0x1.0#1", -1, Some(Equal));
    test("-2.0", "-0x2.0#1", -1, Some(Greater));
    test("-0.5", "-0x0.8#1", -1, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -1,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -1,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -1,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", -1, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", -1, Some(Less));

    test("NaN", "NaN", -100, None);
    test("Infinity", "Infinity", -100, Some(Greater));
    test("-Infinity", "-Infinity", -100, Some(Greater));
    test("0.0", "0x0.0", -100, Some(Less));
    test("-0.0", "-0x0.0", -100, Some(Less));
    test("1.0", "0x1.0#1", -100, Some(Less));
    test("2.0", "0x2.0#1", -100, Some(Less));
    test("0.5", "0x0.8#1", -100, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -100,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        -100,
        Some(Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -100,
        Some(Less),
    );
    test("3.0e120", "0x1.0E+100#1", -100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", -100, Some(Less));
    test("-1.0", "-0x1.0#1", -100, Some(Less));
    test("-2.0", "-0x2.0#1", -100, Some(Less));
    test("-0.5", "-0x0.8#1", -100, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -100,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -100,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -100,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", -100, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", -100, Some(Less));
}

#[test]
fn test_partial_cmp_abs_i64() {
    let test = |s, s_hex, v: i64, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);

        assert_eq!(u.partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&u), out.map(Ordering::reverse));
        assert_eq!(
            rug::Float::exact_from(&u)
                .abs()
                .partial_cmp(&v.unsigned_abs()),
            out
        );
    };
    test("NaN", "NaN", 0, None);
    test("Infinity", "Infinity", 0, Some(Greater));
    test("-Infinity", "-Infinity", 0, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 0, Some(Greater));
    test("-2.0", "-0x2.0#1", 0, Some(Greater));
    test("-0.5", "-0x0.8#1", 0, Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        0,
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        0,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 0, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 0, Some(Greater));

    test("NaN", "NaN", 1, None);
    test("Infinity", "Infinity", 1, Some(Greater));
    test("-Infinity", "-Infinity", 1, Some(Greater));
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
    test("-1.0", "-0x1.0#1", 1, Some(Equal));
    test("-2.0", "-0x2.0#1", 1, Some(Greater));
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
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", 1, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 1, Some(Less));

    test("NaN", "NaN", 100, None);
    test("Infinity", "Infinity", 100, Some(Greater));
    test("-Infinity", "-Infinity", 100, Some(Greater));
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
    test("-3.0e120", "-0x1.0E+100#1", 100, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", 100, Some(Less));

    test("NaN", "NaN", -1, None);
    test("Infinity", "Infinity", -1, Some(Greater));
    test("-Infinity", "-Infinity", -1, Some(Greater));
    test("0.0", "0x0.0", -1, Some(Less));
    test("-0.0", "-0x0.0", -1, Some(Less));
    test("1.0", "0x1.0#1", -1, Some(Equal));
    test("2.0", "0x2.0#1", -1, Some(Greater));
    test("0.5", "0x0.8#1", -1, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -1,
        Some(Less),
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
    test("4.0e-121", "0x1.0E-100#1", -1, Some(Less));
    test("-1.0", "-0x1.0#1", -1, Some(Equal));
    test("-2.0", "-0x2.0#1", -1, Some(Greater));
    test("-0.5", "-0x0.8#1", -1, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -1,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -1,
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -1,
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", -1, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", -1, Some(Less));

    test("NaN", "NaN", -100, None);
    test("Infinity", "Infinity", -100, Some(Greater));
    test("-Infinity", "-Infinity", -100, Some(Greater));
    test("0.0", "0x0.0", -100, Some(Less));
    test("-0.0", "-0x0.0", -100, Some(Less));
    test("1.0", "0x1.0#1", -100, Some(Less));
    test("2.0", "0x2.0#1", -100, Some(Less));
    test("0.5", "0x0.8#1", -100, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        -100,
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        -100,
        Some(Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -100,
        Some(Less),
    );
    test("3.0e120", "0x1.0E+100#1", -100, Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", -100, Some(Less));
    test("-1.0", "-0x1.0#1", -100, Some(Less));
    test("-2.0", "-0x2.0#1", -100, Some(Less));
    test("-0.5", "-0x0.8#1", -100, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        -100,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        -100,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -100,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", -100, Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", -100, Some(Less));
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_abs_primitive_int_properties_helper_unsigned<
    T: PartialOrdAbs<Float> + PrimitiveUnsigned,
>()
where
    Float: From<T> + PartialOrdAbs<T> + PartialOrd<T>,
    rug::Float: PartialOrd<T>,
{
    float_unsigned_pair_gen::<T>().test_properties(|(n, u)| {
        let cmp = n.partial_cmp_abs(&u);
        assert_eq!(
            PartialOrdAbs::<Float>::partial_cmp_abs(&n, &Float::from(u)),
            cmp
        );
        assert_eq!((&n).abs().partial_cmp(&u), cmp);
        assert_eq!(rug::Float::exact_from(&n).abs().partial_cmp(&u), cmp);

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(u.partial_cmp_abs(&n), cmp_rev);
        assert_eq!(
            PartialOrdAbs::<Float>::partial_cmp_abs(&Float::from(u), &n),
            cmp_rev
        );
    });

    float_float_unsigned_triple_gen::<T>().test_properties(|(n, m, u)| {
        if n.lt_abs(&u) && u.lt_abs(&m) {
            assert_eq!(PartialOrdAbs::<Float>::partial_cmp_abs(&n, &m), Some(Less));
        } else if n.gt_abs(&u) && u.gt_abs(&m) {
            assert_eq!(
                PartialOrdAbs::<Float>::partial_cmp_abs(&n, &m),
                Some(Greater)
            );
        }
    });

    float_unsigned_unsigned_triple_gen::<T>().test_properties(|(n, u, v)| {
        if u.lt_abs(&n) && n.lt_abs(&v) {
            assert!(u < v);
        } else if u.gt_abs(&n) && n.gt_abs(&v) {
            assert!(u > v);
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x).partial_cmp_abs(&y), x.partial_cmp_abs(&y));
        assert_eq!(x.partial_cmp_abs(&Float::from(y)), x.partial_cmp_abs(&y));
    });
}

#[allow(clippy::trait_duplication_in_bounds)]
fn partial_cmp_abs_primitive_int_properties_helper_signed<
    T: PartialOrdAbs<Float> + PrimitiveSigned,
>()
where
    Float: From<T> + PartialOrdAbs<T> + PartialOrd<<T as UnsignedAbs>::Output>,
    rug::Float: PartialOrd<<T as UnsignedAbs>::Output>,
{
    float_signed_pair_gen::<T>().test_properties(|(n, i)| {
        let cmp = n.partial_cmp_abs(&i);
        assert_eq!(
            PartialOrdAbs::<Float>::partial_cmp_abs(&n, &Float::from(i)),
            cmp
        );
        assert_eq!((&n).abs().partial_cmp(&i.unsigned_abs()), cmp);
        assert_eq!(
            rug::Float::exact_from(&n)
                .abs()
                .partial_cmp(&i.unsigned_abs()),
            cmp
        );

        let cmp_rev = cmp.map(Ordering::reverse);
        assert_eq!(i.partial_cmp_abs(&n), cmp_rev);
        assert_eq!(
            PartialOrdAbs::<Float>::partial_cmp_abs(&Float::from(i), &n),
            cmp_rev
        );
    });

    float_float_signed_triple_gen::<T>().test_properties(|(n, m, i)| {
        if n.lt_abs(&i) && i.lt_abs(&m) {
            assert_eq!(PartialOrdAbs::<Float>::partial_cmp_abs(&n, &m), Some(Less));
        } else if n.gt_abs(&i) && i.gt_abs(&m) {
            assert_eq!(
                PartialOrdAbs::<Float>::partial_cmp_abs(&n, &m),
                Some(Greater)
            );
        }
    });

    float_signed_signed_triple_gen::<T>().test_properties(|(n, i, j)| {
        if i.lt_abs(&n) && n.lt_abs(&j) {
            assert!(i.lt_abs(&j));
        } else if i.gt_abs(&n) && n.gt_abs(&j) {
            assert!(i.gt_abs(&j));
        }
    });

    signed_pair_gen::<T>().test_properties(|(x, y)| {
        assert_eq!(Float::from(x).partial_cmp_abs(&y), x.partial_cmp_abs(&y));
        assert_eq!(x.partial_cmp_abs(&Float::from(y)), x.partial_cmp_abs(&y));
    });
}

#[test]
fn partial_cmp_abs_primitive_int_properties() {
    apply_fn_to_unsigneds!(partial_cmp_abs_primitive_int_properties_helper_unsigned);
    apply_fn_to_signeds!(partial_cmp_abs_primitive_int_properties_helper_signed);
}
