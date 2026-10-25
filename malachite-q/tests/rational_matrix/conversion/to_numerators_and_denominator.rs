// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Gcd;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;
use malachite_q::test_util::rational_matrix::conversion::to_numerators_and_denominator::*;

#[test]
fn test_to_numerators_and_denominator() {
    let test = |s, numerators_out, denominator_out: u64| {
        let (ns, d) = RationalMatrix::from_str(s)
            .unwrap()
            .to_numerators_and_denominator();
        assert_eq!(ns.to_string(), numerators_out);
        assert_eq!(d, denominator_out);
        let (ns_alt, d_alt) = RationalMatrix::from_str(s)
            .unwrap()
            .into_numerators_and_denominator();
        assert_eq!(ns_alt, ns);
        assert_eq!(d_alt, d);
    };
    test("[]_{0,0}", "[]_{0,0}", 1);
    test("[]_{2,0}", "[]_{2,0}", 1);
    test("[]_{0,3}", "[]_{0,3}", 1);
    test("[0]", "[0]", 1);
    test("[5, -3; 1, 0]", "[5, -3; 1, 0]", 1);
    test("[1/2]", "[1]", 2);
    test("[-1/2]", "[-1]", 2);
    test("[1/2, -2/3; 5, 1/4]", "[6, -8; 60, 3]", 12);
    test("[1/4; 1/6]", "[3; 2]", 12);
    test("[1/6, 1/6, 1/6]", "[1, 1, 1]", 6);
    test("[0, 1/1000000000000]", "[0, 1]", 1000000000000);
}

#[test]
fn to_numerators_and_denominator_properties() {
    rational_matrix_gen().test_properties(|a| {
        let (ns, d) = a.to_numerators_and_denominator();
        assert_eq!(
            a.clone().into_numerators_and_denominator(),
            (ns.clone(), d.clone())
        );
        assert_ne!(d, 0u32);
        assert_eq!((ns.height(), ns.width()), (a.height(), a.width()));
        // The quotients are the elements.
        for (n, x) in ns.elements_ref().iter().zip(a.elements_ref().iter()) {
            assert_eq!(&Rational::from(n) / Rational::from(&d), *x);
        }
        // Every element's denominator divides the result's, and the result is in lowest terms.
        for x in a.elements_ref() {
            assert!((&d % x.denominator_ref()) == 0u32);
        }
        let mut g = d.clone();
        for n in ns.elements_ref() {
            g = g.gcd(n.unsigned_abs_ref());
        }
        assert_eq!(g, Natural::ONE);

        // The computation that avoids least common multiples agrees.
        assert_eq!(
            rational_matrix_to_numerators_and_denominator_naive(&a),
            (ns.clone(), d.clone())
        );
        // Clearing denominators commutes with transposition.
        assert_eq!(
            a.transpose().to_numerators_and_denominator(),
            (ns.transpose(), d.clone())
        );
        // Building the matrix back gives the original.
        assert_eq!(RationalMatrix::from_numerators_and_denominator(&ns, &d), a);
    });
}
