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
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_significant_bits() {
    let test = |s, out| {
        assert_eq!(
            UnsignedPolynomial::<u8>::from_str(s)
                .unwrap()
                .significant_bits(),
            out
        );
    };
    test("0", 0);
    test("5*x^2+2*x+1", 6);
    test("x^3", 1);
}

#[test]
fn significant_bits_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let bits = p.significant_bits();
        // It is the sum of the numbers of significant bits of the coefficients, which is what the
        // vector of the coefficients has.
        assert_eq!(
            p.coefficients_asc()
                .iter()
                .map(|&c| c.significant_bits())
                .sum::<u64>(),
            bits
        );
        assert_eq!(
            UnsignedVector::from_elements(p.coefficients_asc()).significant_bits(),
            bits
        );
        // It is at least the number of bits of the largest coefficient.
        assert!(bits >= p.height_significant_bits());
    });
}
