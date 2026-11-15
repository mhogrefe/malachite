// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_negate, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_rational_pair_gen, float_rational_pair_gen,
};
use malachite_q::Rational;
use std::cmp::Ordering;

#[test]
fn test_partial_cmp_abs_rational() {
    let test = |s: &str, s_hex: &str, y: &str| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let y = Rational::from_str(y).unwrap();
        let x_abs = if x.is_sign_negative() {
            big_exponent_float_negate(x.clone())
        } else {
            x.clone()
        };
        let y_abs = (&y).abs();
        let out = x_abs.partial_cmp(&y_abs);
        assert_eq!(x.partial_cmp_abs(&y), out);
        assert_eq!(y.partial_cmp_abs(&x), out.map(Ordering::reverse));
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
    test("-Infinity", "-Infinity", "-1000");
    test("NaN", "NaN", "-1");
    test("0.0", "0x0.0", "-1");
    test("-3.1415926535897931", "-0x3.243f6a8885a3", "-3");
    test("-3.1415926535897931", "-0x3.243f6a8885a3", "-4");
    test("-255.0", "-0xff.0", "-255");
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "-1",
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "-1000000000000000000000000000000",
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "-1",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "-1",
    );
    test("0.50", "0x0.8", "1/2");
    test("0.50", "0x0.8", "1/3");
    test("0.33333333333333331", "0x0.55555555555554", "1/3");
    test("-0.33333333333333331", "-0x0.55555555555554", "-1/3");
    test("3.1415926535897931", "0x3.243f6a8885a3", "22/7");
    test("3.1415926535897931", "0x3.243f6a8885a3", "355/113");
    test("3.1415926535897931", "0x3.243f6a8885a3", "333/106");
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1/1000000000000000000000000000000",
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "-1/1000000000000000000000000000000",
    );
    test(
        "6.8e-49",
        "0x1.0E-40",
        "1/1461501637330902918203684832716283019655932542976",
    );
    test(
        "6.8e-49",
        "0x1.0E-40",
        "1/1461501637330902918203684832716283019655932542975",
    );
}

#[test]
fn partial_cmp_abs_rational_properties() {
    big_exponent_float_rational_pair_gen().test_properties(|(x, y)| {
        let ord = x.partial_cmp_abs(&y);
        assert_eq!(y.partial_cmp_abs(&x).map(Ordering::reverse), ord);
        assert_eq!(
            big_exponent_float_negate(x.clone()).partial_cmp_abs(&y),
            ord
        );
        let x_abs = if x.is_sign_negative() {
            big_exponent_float_negate(x.clone())
        } else {
            x.clone()
        };
        assert_eq!(x_abs.partial_cmp(&(&y).abs()), ord);
    });

    // - agreement with `Float`
    float_rational_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            float_to_big_exponent_float(&x).partial_cmp_abs(&y),
            x.partial_cmp_abs(&y)
        );
    });
}
