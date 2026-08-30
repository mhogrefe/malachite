// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{AbsSquared, Square};
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::float_gen;
use malachite_float::ComparableFloat;

#[test]
fn test_abs_squared() {
    let test = |s, s_hex, out, out_hex| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let squared = x.clone().abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);
        assert_eq!(to_hex_string(&squared), out_hex);

        let squared = (&x).abs_squared();
        assert!(squared.is_valid());
        assert_eq!(squared.to_string(), out);
        assert_eq!(to_hex_string(&squared), out_hex);
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "Infinity", "Infinity");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "0.0", "0x0.0");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("-1.0", "-0x1.0#1", "1.0", "0x1.0#1");
    test("123.0", "0x7b.0#7", "1.510e4", "0x3.b0E+3#7");
    test("-1.5", "-0x1.8#2", "2.0", "0x2.0#2");
}

#[test]
fn abs_squared_properties() {
    float_gen().test_properties(|x| {
        let abs_squared = x.clone().abs_squared();
        assert!(abs_squared.is_valid());
        assert_eq!(
            ComparableFloat((&x).abs_squared()),
            ComparableFloat(abs_squared.clone())
        );
        assert_eq!(
            ComparableFloat((&x).square()),
            ComparableFloat(abs_squared.clone())
        );
        assert_eq!(
            ComparableFloat((-&x).abs_squared()),
            ComparableFloat(abs_squared)
        );
    });
}
