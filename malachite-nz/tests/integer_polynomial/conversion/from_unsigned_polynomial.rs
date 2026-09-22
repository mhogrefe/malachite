// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::test_util::generators::unsigned_polynomial_gen;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural_polynomial::NaturalPolynomial;

#[test]
fn test_from_unsigned_polynomial() {
    let test = |s| {
        let p = UnsignedPolynomial::<u64>::from_str(s).unwrap();
        let q = IntegerPolynomial::from(p);
        assert!(q.is_valid());
        assert_eq!(q.to_string(), s);
    };
    test("0");
    test("1");
    test("5");
    test("x");
    test("x^2+3*x+2");
    test("18446744073709551615*x^7+1");
}

#[test]
fn from_unsigned_polynomial_properties() {
    unsigned_polynomial_gen().test_properties(|p| {
        let q = IntegerPolynomial::from(p.clone());
        assert!(q.is_valid());
        // Nothing is lost: the degree, every coefficient, and the written form all survive.
        assert_eq!(q.degree(), p.degree());
        assert_eq!(q.to_string(), p.to_string());
        assert_eq!(
            q.coefficients_asc(),
            p.coefficients_asc()
                .iter()
                .map(|c| Integer::from(*c))
                .collect::<Vec<_>>()
        );
        // The result is never negative anywhere, since it came from `u64`s.
        assert!(q.coefficients_asc().iter().all(|c| *c >= 0u32));
        // Reading the string back as an `IntegerPolynomial` gives the same thing.
        assert_eq!(IntegerPolynomial::from_str(&p.to_string()).unwrap(), q);
        // Going through `NaturalPolynomial` gives the same thing as going directly.
        assert_eq!(
            IntegerPolynomial::from(NaturalPolynomial::from(p.clone())),
            q
        );
        for i in 0..5 {
            assert_eq!(*q.coefficient(i), Integer::from(p.coefficient(i)));
        }
    });
}
