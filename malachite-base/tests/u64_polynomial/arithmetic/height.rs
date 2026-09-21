// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Height;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::u64_polynomial_gen;
use malachite_base::u64_polynomial::U64Polynomial;

#[test]
fn test_height() {
    let test = |s, height, bits| {
        let p = U64Polynomial::from_str(s).unwrap();
        assert_eq!(p.to_height(), height);
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), bits);
    };
    // The zero polynomial has no coefficients, so its height is 0.
    test("0", 0, 0);
    test("1", 1, 1);
    test("x^2+3*x+2", 3, 2);
    // The height ignores the degree: a large exponent with small coefficients is still small.
    test("x^100", 1, 1);
    test("18446744073709551615*x+1", 18446744073709551615, 64);
}

#[test]
fn height_properties() {
    u64_polynomial_gen().test_properties(|p| {
        let height = p.to_height();
        // The three ways of asking agree.
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), height.significant_bits());

        // The height is a coefficient, and no coefficient exceeds it.
        assert!(p.coefficients_asc().iter().all(|&c| c <= height));
        assert_eq!(p.degree().is_none(), p.coefficients_asc().is_empty());
        if p.coefficients_asc().is_empty() {
            assert_eq!(height, 0);
        } else {
            assert!(p.coefficients_asc().contains(&height));
        }
    });
}
