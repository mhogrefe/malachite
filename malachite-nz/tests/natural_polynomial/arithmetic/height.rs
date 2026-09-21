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
use malachite_base::test_util::generators::u64_polynomial_gen;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_height() {
    let test = |s, height: u32, bits| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        assert_eq!(p.to_height(), height);
        assert_eq!(*p.height_ref(), height);
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), bits);
    };
    // The zero polynomial has no coefficients, so its height is 0.
    test("0", 0, 0);
    test("1", 1, 1);
    test("x^2+3*x+2", 3, 2);
    // The height ignores the degree.
    test("x^100", 1, 1);
}

#[test]
fn test_height_big() {
    let p = NaturalPolynomial::from_str("123456789012345678901234567890*x+1").unwrap();
    assert_eq!(p.to_height().to_string(), "123456789012345678901234567890");
    assert_eq!(p.height_significant_bits(), 97);
}

#[test]
fn height_properties() {
    natural_polynomial_gen().test_properties(|p| {
        let height = p.to_height();
        // The four ways of asking agree.
        assert_eq!(*p.height_ref(), height);
        assert_eq!(p.clone().into_height(), height);
        assert_eq!(p.height_significant_bits(), height.significant_bits());

        // The height is a coefficient, and no coefficient exceeds it.
        assert!(p.coefficients_asc().iter().all(|c| *c <= height));
        if p.degree().is_none() {
            assert_eq!(height, 0);
        } else {
            assert!(p.coefficients_asc().contains(&height));
        }

        // Widening the coefficients to `Integer`s leaves the height alone, since none of them was
        // negative to begin with.
        assert_eq!(IntegerPolynomial::from(p.clone()).to_height(), height);
    });

    u64_polynomial_gen().test_properties(|p| {
        // The `u64` and `Natural` polynomials agree on heights, as they must, the conversion
        // changing no coefficient's value.
        assert_eq!(
            NaturalPolynomial::from(p.clone()).to_height(),
            Natural::from(p.to_height())
        );
    });
}
