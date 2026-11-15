// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_integer_pair_gen, float_integer_pair_gen,
};
use malachite_nz::integer::Integer;
use std::cmp::Ordering::{self, *};

#[test]
fn test_partial_cmp_integer() {
    let test = |s: &str, s_hex: &str, y: &str, out: Option<Ordering>| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Integer::from_str(y).unwrap();
        assert_eq!(x.partial_cmp(&y), out);
        assert_eq!(y.partial_cmp(&x), out.map(Ordering::reverse));
        assert_eq!(x == y, out == Some(Equal));
        assert_eq!(y == x, out == Some(Equal));
    };
    test("NaN", "NaN", "0", None);
    test("Infinity", "Infinity", "0", Some(Greater));
    test("-Infinity", "-Infinity", "0", Some(Less));
    test("0.0", "0x0.0", "0", Some(Equal));
    test("0.0", "0x0.0", "1", Some(Less));
    test("1.0", "0x1.0", "1", Some(Equal));
    test("0.50", "0x0.8", "1", Some(Less));
    test("1.0", "0x1.0", "0", Some(Greater));
    test("3.1415926535897931", "0x3.243f6a8885a3", "3", Some(Greater));
    test("3.1415926535897931", "0x3.243f6a8885a3", "4", Some(Less));
    test("255.0", "0xff.0", "255", Some(Equal));
    test("255.5", "0xff.8", "255", Some(Greater));
    test(
        "1.5e48",
        "0x1.0E+40",
        "1461501637330902918203684832716283019655932542976",
        Some(Equal),
    );
    test(
        "1.5e48",
        "0x1.0E+40",
        "1461501637330902918203684832716283019655932542977",
        Some(Less),
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "0",
        Some(Greater),
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1461501637330902918203684832716283019655932542976",
        Some(Greater),
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "0",
        Some(Greater),
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1",
        Some(Less),
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "0",
        Some(Less),
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "0",
        Some(Less),
    );
    test("-Infinity", "-Infinity", "-1000", Some(Less));
    test("NaN", "NaN", "-1", None);
    test("0.0", "0x0.0", "-1", Some(Greater));
    test("-3.1415926535897931", "-0x3.243f6a8885a3", "-3", Some(Less));
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a3",
        "-4",
        Some(Greater),
    );
    test("-255.0", "-0xff.0", "-255", Some(Equal));
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "-1",
        Some(Greater),
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "-1000000000000000000000000000000",
        Some(Less),
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "-1",
        Some(Greater),
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "-1",
        Some(Greater),
    );
}

#[test]
fn partial_cmp_integer_properties() {
    big_exponent_float_integer_pair_gen().test_properties(|(x, y)| {
        let ord = x.partial_cmp(&y);
        assert_eq!(y.partial_cmp(&x).map(Ordering::reverse), ord);
        assert_eq!(ord, x.partial_cmp(&BigExponentFloat::from(&y)));
    });

    // - agreement with `Float`
    float_integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            float_to_big_exponent_float(&x).partial_cmp(&y),
            x.partial_cmp(&y)
        );
    });
}
