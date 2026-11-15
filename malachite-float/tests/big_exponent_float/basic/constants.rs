// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{NegativeOne, One, OneHalf, Two};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::parse_big_exponent_float_hex;

#[test]
fn test_constants() {
    let test = |s: &str, s_hex: &str, out: &str, out_hex: &str| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(format!("{x:#x}"), out_hex);
    };
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("1.0", "0x1.0", "1.0", "0x1.0");
    test("2.0", "0x2.0", "2.0", "0x2.0");
    test("-1.0", "-0x1.0", "-1.0", "-0x1.0");
    test("0.50", "0x0.8", "0.50", "0x0.8");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "-Infinity", "-Infinity");
    test("NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "NaN", "NaN");
    test("-Infinity", "-Infinity", "-Infinity", "-Infinity");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    for x in [
        BigExponentFloat::ONE,
        BigExponentFloat::TWO,
        BigExponentFloat::NEGATIVE_ONE,
        BigExponentFloat::ONE_HALF,
    ] {
        assert_eq!(x.get_prec(), Some(1));
    }
}
