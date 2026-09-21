// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::u64_polynomial_gen;
use malachite_base::u64_polynomial::U64Polynomial;
use malachite_nz::natural::Natural;
use malachite_nz::natural_polynomial::NaturalPolynomial;

#[test]
fn test_from_u64_polynomial() {
    let test = |s| {
        let p = U64Polynomial::from_str(s).unwrap();
        let q = NaturalPolynomial::from(p);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), s);
    };
    test("0");
    test("1");
    test("5");
    test("x");
    test("x^2+3*x+2");
    // The largest coefficient a `U64Polynomial` can hold survives.
    test("18446744073709551615*x^7+1");
}

#[test]
fn from_u64_polynomial_properties() {
    u64_polynomial_gen().test_properties(|p| {
        let q = NaturalPolynomial::from(p.clone());
        assert!(q.is_valid());
        // Nothing is lost: the degree, every coefficient, and the written form all survive.
        assert_eq!(q.degree(), p.degree());
        assert_eq!(q.to_string(), p.to_string());
        assert_eq!(
            q.coefficients_asc(),
            p.coefficients_asc()
                .iter()
                .map(|c| Natural::from(*c))
                .collect::<Vec<_>>()
        );
        // Reading the string back as a `NaturalPolynomial` gives the same thing.
        assert_eq!(NaturalPolynomial::from_str(&p.to_string()).unwrap(), q);
        // A coefficient at any index agrees, including past the degree.
        for i in 0..5 {
            assert_eq!(*q.coefficient(i), Natural::from(p.coefficient(i)));
        }
    });
}
