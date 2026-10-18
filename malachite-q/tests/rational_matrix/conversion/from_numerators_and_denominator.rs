// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_matrix_gen;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;

#[test]
fn test_from_numerators_and_denominator() {
    let test = |ns, d: u32, out| {
        let a = RationalMatrix::from_numerators_and_denominator(
            &IntegerMatrix::from_str(ns).unwrap(),
            &Natural::from(d),
        );
        assert_eq!(a.to_string(), out);
    };
    test("[]_{0,0}", 1, "[]_{0,0}");
    test("[]_{3,0}", 7, "[]_{3,0}");
    test("[0]", 5, "[0]");
    test("[6, -8; 60, 3]", 12, "[1/2, -2/3; 5, 1/4]");
    test("[2, 4; 6, 8]", 2, "[1, 2; 3, 4]");
    test("[-10]", 4, "[-5/2]");
}

#[test]
#[should_panic]
fn from_numerators_and_denominator_fail() {
    RationalMatrix::from_numerators_and_denominator(
        &IntegerMatrix::from_str("[1]").unwrap(),
        &Natural::from(0u32),
    );
}

#[test]
fn from_numerators_and_denominator_properties() {
    rational_matrix_gen().test_properties(|a| {
        let (ns, d) = a.to_numerators_and_denominator();
        assert_eq!(RationalMatrix::from_numerators_and_denominator(&ns, &d), a);
    });

    integer_matrix_gen().test_properties(|ns| {
        for d in [1u32, 2, 6, 1000] {
            let d = Natural::from(d);
            let a = RationalMatrix::from_numerators_and_denominator(&ns, &d);
            assert_eq!((a.height(), a.width()), (ns.height(), ns.width()));
            for (x, n) in a.elements.iter().zip(ns.elements.iter()) {
                assert_eq!(*x, Rational::from(n) / Rational::from(&d));
            }
        }
        // Over a denominator of 1, the elements are the numerators.
        let a = RationalMatrix::from_numerators_and_denominator(&ns, &Natural::from(1u32));
        assert_eq!(a, RationalMatrix::from(ns.clone()));
        assert_eq!(a.to_numerators_and_denominator(), (ns, Natural::from(1u32)));
    });
}
