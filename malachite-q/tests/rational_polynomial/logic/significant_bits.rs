// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_significant_bits() {
    let test = |s, out| {
        assert_eq!(
            RationalPolynomial::from_str(s).unwrap().significant_bits(),
            out
        );
    };
    test("0", 0);
    test("1/3*x^2+1/2", 7);
    test("-x^3", 5);
    // - The coefficient 1/2 is stored as 2/4, and is reduced before being counted.
    test("1/2*x+1/4", 7);
}

#[test]
fn significant_bits_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let bits = p.significant_bits();
        // It is the sum of the numbers of significant bits of the coefficients, which is what the
        // vector of the coefficients has.
        let coefficients = p.to_coefficients_asc();
        assert_eq!(
            coefficients
                .iter()
                .map(SignificantBits::significant_bits)
                .sum::<u64>(),
            bits
        );
        assert_eq!(
            RationalVector::from_owned_elements(coefficients).significant_bits(),
            bits
        );
        // Negating the polynomial does not change it.
        assert_eq!((-&p).significant_bits(), bits);
    });

    integer_polynomial_gen().test_properties(|p| {
        // As rationals, the coefficients of an integer polynomial each gain a 1-bit denominator.
        assert_eq!(
            RationalPolynomial::from(p.clone()).significant_bits(),
            p.significant_bits() + p.len()
        );
    });
}
