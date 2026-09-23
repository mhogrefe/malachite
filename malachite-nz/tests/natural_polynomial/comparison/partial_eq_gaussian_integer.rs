// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_nz::gaussian_integer::GaussianInteger;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::{
    gaussian_integer_gen, integer_gen, natural_polynomial_gaussian_integer_pair_gen,
};

#[test]
fn test_partial_eq_gaussian_integer() {
    let test = |s, t, out| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let g = GaussianInteger::from_str(t).unwrap();
        assert_eq!(p == g, out);
        assert_eq!(g == p, out);
    };
    // The zero polynomial equals 0 and nothing else.
    test("0", "0", true);
    test("0", "1", false);
    test("0", "i", false);
    test("1", "1", true);
    test("1", "-1", false);
    test("1", "1+i", false);
    test("123", "123", true);
    test("123", "-123", false);
    test("123", "123-i", false);
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        true,
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000+i",
        false,
    );
    // No polynomial of positive degree equals a GaussianInteger, whatever its constant term.
    test("x", "0", false);
    test("x+1", "1", false);
    test("2*x^2+3", "3", false);
}

#[test]
fn partial_eq_gaussian_integer_properties() {
    natural_polynomial_gaussian_integer_pair_gen().test_properties(|(p, g)| {
        let eq = p == g;
        assert_eq!(g == p, eq);
        // Only a GaussianInteger with no imaginary part and a non-negative real part can equal a
        // polynomial, and then exactly when the Natural with its value does.
        assert_eq!(
            eq,
            g.imaginary == 0u32 && g.real >= 0u32 && p == Natural::exact_from(&g.real)
        );
    });

    gaussian_integer_gen().test_properties(|g| {
        assert_eq!(NaturalPolynomial::ZERO == g, g == 0u32);
        assert_eq!(NaturalPolynomial::one() == g, g == 1u32);
        assert!(NaturalPolynomial::x() != g);
    });

    integer_gen().test_properties(|i| {
        // A real GaussianInteger compares with a polynomial exactly as its real part does.
        let g = GaussianInteger::from(i.clone());
        if let Ok(n) = Natural::try_from(&i) {
            let p = NaturalPolynomial::from(n);
            assert!(p == g);
            assert!(g == p);
        }
        assert_eq!(NaturalPolynomial::ZERO == g, NaturalPolynomial::ZERO == i);
    });
}
