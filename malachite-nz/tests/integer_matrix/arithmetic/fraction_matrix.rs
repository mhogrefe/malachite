// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd, Lcm};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::arithmetic::fraction_matrix::*;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_matrix_natural_pair_gen;

// The fraction n/d in lowest terms, for d > 0.
fn fraction(n: &Integer, d: &Natural) -> Fraction {
    let g = n.unsigned_abs_ref().gcd(d);
    Fraction {
        sign: *n >= 0u32,
        numerator: n.unsigned_abs_ref().div_exact(&g),
        denominator: d.div_exact(&g),
    }
}

#[test]
fn test_fractions_to_numerators_and_denominator() {
    let test = |xs: &[(i32, u32)], height, width, out_numerators, out_den| {
        let fractions: Vec<Fraction> = xs
            .iter()
            .map(|&(n, d)| fraction(&Integer::from(n), &Natural::from(d)))
            .collect();
        let (numerators, den) = fractions_to_numerators_and_denominator(&fractions, height, width);
        assert_eq!(numerators.to_string(), out_numerators);
        assert_eq!(den.to_string(), out_den);
    };
    test(&[], 0, 0, "[]_{0,0}", "1");
    test(&[], 2, 0, "[]_{2,0}", "1");
    test(&[(1, 2), (-1, 3), (0, 1)], 1, 3, "[3, -2, 0]", "6");
    test(&[(2, 4), (5, 1)], 2, 1, "[1; 10]", "2");
    test(&[(7, 1), (-8, 1)], 1, 2, "[7, -8]", "1");
}

#[test]
fn fractions_to_numerators_and_denominator_properties() {
    integer_matrix_natural_pair_gen().test_properties(|(a, d)| {
        let d = d + Natural::ONE;
        let fractions: Vec<Fraction> = a.elements.iter().map(|n| fraction(n, &d)).collect();
        let height = usize::exact_from(a.height());
        let width = usize::exact_from(a.width());
        let (numerators, den) = fractions_to_numerators_and_denominator(&fractions, height, width);
        assert!(numerators.is_valid());
        assert_eq!(numerators.height(), a.height());
        assert_eq!(numerators.width(), a.width());
        // The denominator is the least common multiple of the denominators, and each numerator over
        // it is the fraction.
        let lcm = fractions
            .iter()
            .fold(Natural::ONE, |l, f| l.lcm(&f.denominator));
        assert_eq!(den, lcm);
        for (n, f) in numerators.elements.iter().zip(&fractions) {
            let original = Integer::from_sign_and_abs_ref(f.sign, &f.numerator);
            assert_eq!(
                n * Integer::from(&f.denominator),
                original * Integer::from(&den)
            );
        }
    });
}
