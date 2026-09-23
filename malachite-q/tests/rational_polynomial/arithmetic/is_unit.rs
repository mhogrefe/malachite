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
use malachite_nz::test_util::generators::integer_polynomial_gen;
use malachite_q::rational_polynomial::RationalPolynomial;
use malachite_q::test_util::generators::rational_polynomial_gen;

#[test]
fn test_is_unit() {
    let test = |s, out| {
        let p = RationalPolynomial::from_str(s).unwrap();
        assert_eq!(p.is_unit(), out);
    };
    test("0", false);
    test("1", true);
    test("-1", true);
    test("2", true);
    test("1/2", true);
    test("-22/7", true);
    test("x", false);
    test("1/2*x", false);
    test("x-1", false);
}

#[test]
fn is_unit_properties() {
    rational_polynomial_gen().test_properties(|p| {
        let is_unit = p.is_unit();
        // Every nonzero constant is a unit, and nothing else is.
        assert_eq!(is_unit, p.degree() == Some(0));
        assert_eq!(is_unit, p.coefficient(0) != 0u32 && p.degree() == Some(0));
    });

    integer_polynomial_gen().test_properties(|p| {
        // Over the rationals, more integer polynomials are units than over the integers.
        let is_unit = RationalPolynomial::from(p.clone()).is_unit();
        assert_eq!(is_unit, p.degree() == Some(0));
        if p.is_unit() {
            assert!(is_unit);
        }
    });
}
