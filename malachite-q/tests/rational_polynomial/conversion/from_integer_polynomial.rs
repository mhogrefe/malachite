// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::IsInteger;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::Rational;
use malachite_q::rational_polynomial::RationalPolynomial;

#[test]
fn test_from_integer_polynomial() {
    let test = |s| {
        let p = IntegerPolynomial::from_str(s).unwrap();
        let q = RationalPolynomial::from(p);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), s);
    };
    test("0");
    test("1");
    test("5");
    test("-5");
    test("x");
    test("x^2-3*x+2");
    test("-123456789012345678901234567890*x^7+1");
}

#[test]
fn from_integer_polynomial_properties() {
    integer_polynomial_gen().test_properties(|p| {
        let q = RationalPolynomial::from(p.clone());
        assert!(q.is_valid());
        // Nothing is lost: the degree, every coefficient, and the written form all survive.
        assert_eq!(q.degree(), p.degree());
        assert_eq!(q.to_string(), p.to_string());
        assert_eq!(
            q.to_coefficients_asc(),
            p.coefficients_asc()
                .iter()
                .map(|c| Rational::from(c.clone()))
                .collect::<Vec<_>>()
        );
        // The polynomial given is the numerator itself, and the denominator is 1.
        assert_eq!(*q.numerator_ref(), p);
        assert_eq!(*q.denominator_ref(), Natural::ONE);
        // Every coefficient is an integer, so no coefficient has a denominator of its own.
        assert!(q.to_coefficients_asc().iter().all(IsInteger::is_integer));
        // Reading the string back as a `RationalPolynomial` gives the same thing.
        assert_eq!(RationalPolynomial::from_str(&p.to_string()).unwrap(), q);
        for i in 0..5 {
            assert_eq!(q.coefficient(i), Rational::from(p.coefficient(i).clone()));
        }
    });
}
