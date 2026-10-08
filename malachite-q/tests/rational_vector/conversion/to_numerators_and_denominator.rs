// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::Gcd;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::rational_vector::conversion::from_numerators_and_denominator::*;
use malachite_q::rational_vector::conversion::to_numerators_and_denominator::*;
use malachite_q::test_util::generators::rational_vector_gen;
use malachite_q::test_util::rational_vector::conversion::to_numerators_and_denominator::*;

#[test]
fn test_to_numerators_and_denominator() {
    let test = |s, numerators_out, denominator_out: u64| {
        let (ns, d) = to_numerators_and_denominator(&RationalVector::from_str(s).unwrap());
        assert_eq!(ns.to_string(), numerators_out);
        assert_eq!(d, denominator_out);
    };
    test("()", "()", 1);
    test("(0)", "(0)", 1);
    test("(5, -3)", "(5, -3)", 1);
    test("(1/2)", "(1)", 2);
    test("(-1/2)", "(-1)", 2);
    test("(1/2, -2/3, 5)", "(3, -4, 30)", 6);
    // The least common multiple, not the product.
    test("(1/4, 1/6)", "(3, 2)", 12);
    test("(1/6, 1/6, 1/6)", "(1, 1, 1)", 6);
    test("(0, 1/1000000000000)", "(0, 1)", 1000000000000);
}

#[test]
fn to_numerators_and_denominator_properties() {
    rational_vector_gen().test_properties(|v| {
        let (ns, d) = to_numerators_and_denominator(&v);
        assert_ne!(d, 0u32);
        assert_eq!(ns.dimension(), v.dimension());
        // The quotients are the elements.
        for (n, x) in ns.elements.iter().zip(v.elements.iter()) {
            assert_eq!(&Rational::from(n) / Rational::from(&d), *x);
        }
        // Every element's denominator divides the result's, and the result is in lowest terms.
        for x in &v.elements {
            assert!((&d % x.denominator_ref()) == 0u32);
        }
        let mut g = d.clone();
        for n in &ns.elements {
            g = g.gcd(n.unsigned_abs_ref());
        }
        assert_eq!(g, Natural::ONE);

        // The computation that avoids least common multiples agrees.
        assert_eq!(
            rational_vector_to_numerators_and_denominator_naive(&v),
            (ns.clone(), d.clone())
        );

        // Building the vector back gives the original.
        assert_eq!(from_numerators_and_denominator(&ns, &d), v);
    });
}
