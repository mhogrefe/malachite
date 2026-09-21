// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;
use malachite_nz::test_util::generators::natural_polynomial_gen;

#[test]
fn test_from_natural_polynomial() {
    let test = |s| {
        let p = NaturalPolynomial::from_str(s).unwrap();
        let q = IntegerPolynomial::from(p);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), s);
    };
    test("0");
    test("1");
    test("5");
    test("x");
    test("x^2+3*x+2");
    test("123456789012345678901234567890*x^7+1");
}

#[test]
fn from_natural_polynomial_properties() {
    natural_polynomial_gen().test_properties(|p| {
        let q = IntegerPolynomial::from(p.clone());
        assert!(q.is_valid());
        // Nothing is lost: the degree, every coefficient, and the written form all survive.
        assert_eq!(q.degree(), p.degree());
        assert_eq!(q.to_string(), p.to_string());
        assert_eq!(
            q.coefficients_asc(),
            p.coefficients_asc()
                .iter()
                .map(|c| Integer::from(c.clone()))
                .collect::<Vec<_>>()
        );
        // The result is never negative anywhere, since it came from `Natural`s.
        assert!(q.coefficients_asc().iter().all(|c| *c >= 0u32));
        // Reading the string back as an `IntegerPolynomial` gives the same thing.
        assert_eq!(IntegerPolynomial::from_str(&p.to_string()).unwrap(), q);
        // A coefficient at any index agrees.
        for i in 0..5 {
            assert_eq!(
                *q.coefficient(i),
                Integer::exact_from(p.coefficient(i).clone())
            );
        }
    });
}
