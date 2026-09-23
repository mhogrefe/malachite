// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, HeightRef};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::Polynomial;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::test_util::generators::integer_polynomial_gen;

#[test]
fn test_height() {
    let test = |s, height: u32, bits| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        assert_eq!(p.to_height(), height);
        assert_eq!(*p.height_ref(), height);
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), bits);
    };
    // The zero polynomial has no coefficients, so its height is 0.
    test("0", 0, 0);
    test("1", 1, 1);
    test("-1", 1, 1);
    // The height is a magnitude, so a negative coefficient can be the largest.
    test("x^2-3*x+2", 3, 2);
    test("-5*x+2", 5, 3);
    test("-x^100", 1, 1);
}

#[test]
fn height_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let height = p.to_height();
        // The four ways of asking agree.
        assert_eq!(*p.height_ref(), height);
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), height.significant_bits());

        // The height is the magnitude of a coefficient, and no coefficient's magnitude exceeds it.
        let magnitudes: Vec<_> = p
            .coefficients_asc()
            .iter()
            .map(|c| c.unsigned_abs_ref().clone())
            .collect();
        assert!(magnitudes.iter().all(|m| *m <= height));
        if p.degree().is_none() {
            assert_eq!(height, 0);
        } else {
            assert!(magnitudes.contains(&height));
        }

        // Negating every coefficient leaves the height alone, since it is built from magnitudes.
        let negated = IntegerPolynomial::from_coefficients_asc(
            p.coefficients_asc().iter().map(|c| -c).collect(),
        );
        assert_eq!(negated.to_height(), height);
    });
}
