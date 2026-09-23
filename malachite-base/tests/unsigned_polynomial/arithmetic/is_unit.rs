// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::IsUnit;
use malachite_base::polynomial::Polynomial;
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        assert_eq!(p.is_unit(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("18446744073709551615", false);
    test("x", false);
    test("x+1", false);
}

#[test]
fn is_unit_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let is_unit = p.is_unit();
        assert_eq!(is_unit, p == 1u64);
        if is_unit {
            assert_eq!(p.degree(), Some(0));
        }
    });
}
