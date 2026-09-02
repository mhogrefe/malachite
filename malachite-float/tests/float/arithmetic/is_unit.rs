// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::IsUnit;
use malachite_float::Float;
use malachite_float::test_util::generators::float_gen;
use std::str::FromStr;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        let x = Float::from_str(s).unwrap();
        assert_eq!(x.is_unit(), out);
    };
    test("0.0", false);
    test("-0.0", false);
    test("1.0", true);
    test("-1.0", true);
    test("1.5", true);
    test("-123.0", true);
    test("NaN", false);
    test("Infinity", false);
    test("-Infinity", false);
}

#[test]
fn is_unit_properties() {
    float_gen().test_properties(|x| {
        let is_unit = x.is_unit();
        assert_eq!(is_unit, x.is_finite() && x != 0u32);
    });
}
