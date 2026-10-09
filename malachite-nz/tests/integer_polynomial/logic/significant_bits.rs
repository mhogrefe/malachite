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
use malachite_base::vector::Vector;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_polynomial_gen;

#[test]
fn test_significant_bits() {
    let test = |s, out| {
        assert_eq!(
            IntegerPolynomial::from_str(s).unwrap().significant_bits(),
            out
        );
    };
    test("0", 0);
    test("-5*x^2+2*x+1", 6);
    test("-x^3", 1);
}

#[test]
fn significant_bits_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let bits = p.significant_bits();
        // It is the sum of the numbers of significant bits of the coefficients, which is what the
        // vector of the coefficients has.
        assert_eq!(
            p.coefficients_asc()
                .iter()
                .map(SignificantBits::significant_bits)
                .sum::<u64>(),
            bits
        );
        assert_eq!(
            IntegerVector::from_elements(p.coefficients_asc()).significant_bits(),
            bits
        );
        // It is at least the number of bits of the largest coefficient, and negating the polynomial
        // does not change it.
        assert!(bits >= p.height_significant_bits());
        assert_eq!((-&p).significant_bits(), bits);
    });
}
