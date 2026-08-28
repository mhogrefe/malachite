// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::{IsGaussianInteger, IsInteger, IsReal};
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;

#[test]
fn test_is_gaussian_integer() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(x.is_gaussian_integer(), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", true);
    test("0.50", "0x0.8#1", false);
    test("123.0", "0x7b.0#7", true);
    test("3.1415926535897931", "0x3.243f6a8885a3#50", false);

    test("-1.0", "-0x1.0#1", true);
    test("-0.50", "-0x0.8#1", false);
    test("-123.0", "-0x7b.0#7", true);
    test("-3.1415926535897931", "-0x3.243f6a8885a3#50", false);
}

#[test]
fn is_gaussian_integer_properties() {
    float_gen().test_properties(|x| {
        assert_eq!(x.is_gaussian_integer(), x.is_integer());
        assert_eq!(x.is_integer(), x.is_gaussian_integer() && x.is_real());
    });
}
