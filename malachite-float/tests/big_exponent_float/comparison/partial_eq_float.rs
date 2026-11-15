// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_to_float, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{big_exponent_float_pair_gen, float_pair_gen};

#[test]
fn test_partial_eq_float() {
    let test = |s: &str, s_hex: &str, y: &str| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(y);
        let out = x == float_to_big_exponent_float(&y);
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("NaN", "NaN", "NaN");
    test("0.0", "0x0.0", "0x0.0");
    test("0.0", "0x0.0", "-0x0.0");
    test("1.0", "0x1.0", "0x1.0#1");
    test("1.0", "0x1.0", "0x1.0000000000000000000000000#100");
    test("1.0", "0x1.0", "-0x1.0#1");
    test("0.50", "0x0.8", "0x1.0#1");
    test("Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-Infinity");
    test("-1.0", "-0x1.0", "-Infinity");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "0x3.243f6a8885a30#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a3",
        "0x3.243f6a8885a308d4#64",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a3",
        "-0x3.243f6a8885a20#52",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "0x4.0E+268435455#1",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "Infinity",
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "-0x4.0E+268435455#1",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "0x1.0E-268435456#1",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "0x0.0",
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "-0x0.0",
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "-0x1.0E-268435456#1",
    );
}

#[test]
fn partial_eq_float_properties() {
    // - the comparison agrees with the one between `BigExponentFloat`s
    big_exponent_float_pair_gen().test_properties(|(x, y)| {
        if let Some(f) = big_exponent_float_to_float(&y) {
            assert_eq!(x == f, x == y);
            assert_eq!(f == x, x == y);
        }
    });

    // - agreement with `Float`
    float_pair_gen().test_properties(|(x, y)| {
        assert_eq!(float_to_big_exponent_float(&x) == y, x == y);
    });
}
