// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::float_gen;
use malachite_float::{BigExponentFloat, Float};

#[test]
fn test_from_float() {
    let test = |s, out| {
        let f = parse_hex_string(s);
        let x = BigExponentFloat::from(f.clone());
        assert!(x.is_valid());
        assert_eq!(format!("{x:#x}"), out);
        assert_eq!(format!("{:#x}", BigExponentFloat::from(&f)), out);
    };
    test("NaN", "NaN");
    test("Infinity", "Infinity");
    test("-Infinity", "-Infinity");
    test("0x0.0", "0x0.0");
    // - negative zero loses its sign
    test("-0x0.0", "0x0.0");
    test("0x1.8#2", "0x1.8");
    test("-0x1.8#2", "-0x1.8");
    // - trailing zero limbs are dropped
    test("0x1.0000000000000000000000000#100", "0x1.0");
    test("0x4.0E+268435455#1", "0x4.0E+268435455");
    test("0x1.0E-268435456#1", "0x1.0E-268435456");
}

#[test]
fn from_float_properties() {
    float_gen().test_properties(|f| {
        let x = BigExponentFloat::from(&f);
        assert!(x.is_valid());
        assert_eq!(
            format!("{x:#x}"),
            format!("{:#x}", BigExponentFloat::from(f.clone()))
        );
        assert_eq!(x.is_nan(), f.is_nan());
        assert_eq!(x.is_zero(), f.is_zero());
        assert_eq!(x.get_prec(), f.get_min_prec());
        // - the value is kept exactly
        if !f.is_nan() {
            let g = Float::try_from(&x).unwrap();
            assert_eq!(g, f);
            assert_eq!(g.get_prec(), f.get_min_prec());
        }
    });
}
