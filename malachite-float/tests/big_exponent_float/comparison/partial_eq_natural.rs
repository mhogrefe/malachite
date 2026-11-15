// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_natural_pair_gen, float_natural_pair_gen,
};
use malachite_nz::natural::Natural;
use std::cmp::Ordering::*;

#[test]
fn test_partial_eq_natural() {
    let test = |s: &str, s_hex: &str, y: &str| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Natural::from_str(y).unwrap();
        let out = x.partial_cmp(&y) == Some(Equal);
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("NaN", "NaN", "0");
    test("Infinity", "Infinity", "0");
    test("-Infinity", "-Infinity", "0");
    test("0.0", "0x0.0", "0");
    test("0.0", "0x0.0", "1");
    test("1.0", "0x1.0", "1");
    test("0.50", "0x0.8", "1");
    test("1.0", "0x1.0", "0");
    test("3.1415926535897931", "0x3.243f6a8885a3", "3");
    test("3.1415926535897931", "0x3.243f6a8885a3", "4");
    test("255.0", "0xff.0", "255");
    test("255.5", "0xff.8", "255");
    test(
        "1.5e48",
        "0x1.0E+40",
        "1461501637330902918203684832716283019655932542976",
    );
    test(
        "1.5e48",
        "0x1.0E+40",
        "1461501637330902918203684832716283019655932542977",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "0",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "1461501637330902918203684832716283019655932542976",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "0",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1",
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "0",
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "0",
    );
}

#[test]
fn partial_eq_natural_properties() {
    big_exponent_float_natural_pair_gen().test_properties(|(x, y)| {
        let e = x == y;
        assert_eq!(y == x, e);
        assert_eq!(x.partial_cmp(&y) == Some(Equal), e);
    });

    // - agreement with `Float`
    float_natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(float_to_big_exponent_float(&x) == y, x == y);
    });
}
