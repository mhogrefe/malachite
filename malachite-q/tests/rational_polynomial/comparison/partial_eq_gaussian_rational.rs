// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_q::gaussian_rational::GaussianRational;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::{
    gaussian_rational_gen, rational_gen, rational_polynomial_gaussian_rational_pair_gen,
};

#[test]
fn test_partial_eq_gaussian_rational() {
    let test = |s, t, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        let g = GaussianRational::from_str(t).unwrap();
        assert_eq!(p == g, out);
        assert_eq!(g == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "i/2", false);
    test("1", "1", true);
    test("1/2", "1/2", true);
    test("1/2", "-1/2", false);
    test("1/2", "1/2+i/2", false);
    test("-22/7", "-22/7", true);
    test("-22/7", "-22/7-i", false);
    // No polynomial of positive degree equals a GaussianRational, whatever its constant term.
    test("x", "0", false);
    test("1/2*x+1/2", "1/2", false);
}

#[test]
fn partial_eq_gaussian_rational_properties() {
    rational_polynomial_gaussian_rational_pair_gen().test_properties(|(p, g)| {
        let eq = p == g;
        assert_eq!(g == p, eq);
        // Only a GaussianRational with no imaginary part can equal a polynomial, and then exactly
        // when its real part does.
        assert_eq!(eq, g.imaginary == 0u32 && p == g.real);
    });

    gaussian_rational_gen().test_properties(|g| {
        assert_eq!(RationalPolynomial::ZERO == g, g == 0u32);
        assert_eq!(RationalPolynomial::one() == g, g == 1u32);
        assert_eq!(RationalPolynomial::negative_one() == g, g == -1i32);
        assert!(RationalPolynomial::x() != g);
    });

    rational_gen().test_properties(|r| {
        // A real GaussianRational compares with a polynomial exactly as its real part does.
        let p = RationalPolynomial::from(r.clone());
        let g = GaussianRational::from(r);
        assert!(p == g);
        assert!(g == p);
    });
}
