// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;

#[test]
fn test_complexity() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.complexity(), out);
    };
    test("NaN", "NaN", 1);
    test("Infinity", "Infinity", 1);
    test("-Infinity", "-Infinity", 1);
    test("0.0", "0x0.0", 1);
    test("-0.0", "-0x0.0", 1);

    test("1.0", "0x1.0#1", 1);
    test("2.0", "0x2.0#1", 1);
    test("0.5", "0x0.8#1", 1);
    test("0.33333333333333331", "0x0.55555555555554#53", 53);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 53);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 53);
    test("3.0e120", "0x1.0E+100#1", 400);
    test("4.0e-121", "0x1.0E-100#1", 400);

    test("-1.0", "-0x1.0#1", 1);
    test("-2.0", "-0x2.0#1", 1);
    test("-0.5", "-0x0.8#1", 1);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 53);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 53);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 53);
    test("-3.0e120", "-0x1.0E+100#1", 400);
    test("-4.0e-121", "-0x1.0E-100#1", 400);
}

#[test]
fn test_significant_bits() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.significant_bits(), out);
    };
    test("NaN", "NaN", 1);
    test("Infinity", "Infinity", 1);
    test("-Infinity", "-Infinity", 1);
    test("0.0", "0x0.0", 1);
    test("-0.0", "-0x0.0", 1);

    test("1.0", "0x1.0#1", 1);
    test("2.0", "0x2.0#1", 1);
    test("0.5", "0x0.8#1", 1);
    test("0.33333333333333331", "0x0.55555555555554#53", 53);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", 53);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", 53);
    test("3.0e120", "0x1.0E+100#1", 1);
    test("4.0e-121", "0x1.0E-100#1", 1);

    test("-1.0", "-0x1.0#1", 1);
    test("-2.0", "-0x2.0#1", 1);
    test("-0.5", "-0x0.8#1", 1);
    test("-0.33333333333333331", "-0x0.55555555555554#53", 53);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", 53);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", 53);
    test("-3.0e120", "-0x1.0E+100#1", 1);
    test("-4.0e-121", "-0x1.0E-100#1", 1);
}

#[test]
fn complexity_properties() {
    float_gen().test_properties(|x| {
        let complexity = x.complexity();
        assert_ne!(complexity, 0);
        assert_eq!((-x).complexity(), complexity);
    });
}

#[test]
fn significant_bits_properties() {
    float_gen().test_properties(|x| {
        let bits = x.significant_bits();
        assert_ne!(bits, 0);
        assert_eq!((-&x).significant_bits(), bits);
        assert!(bits <= x.complexity());
    });
}
