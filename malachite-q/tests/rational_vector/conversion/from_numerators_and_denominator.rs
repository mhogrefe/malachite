// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_from_numerators_and_denominator() {
    let test = |ns, d: u32, out| {
        let v = RationalVector::from_numerators_and_denominator(
            &IntegerVector::from_str(ns).unwrap(),
            &Natural::from(d),
        );
        assert_eq!(v.to_string(), out);
    };
    test("()", 1, "()");
    test("()", 7, "()");
    test("(0)", 5, "(0)");
    test("(3, -4, 30)", 6, "(1/2, -2/3, 5)");
    // The numerators and denominator need not be in lowest terms.
    test("(2, 4, 6)", 2, "(1, 2, 3)");
    test("(-10)", 4, "(-5/2)");
}

#[test]
#[should_panic]
fn from_numerators_and_denominator_fail() {
    RationalVector::from_numerators_and_denominator(
        &IntegerVector::from_str("(1)").unwrap(),
        &Natural::from(0u32),
    );
}

#[test]
fn from_numerators_and_denominator_properties() {
    rational_vector_gen().test_properties(|v| {
        let (ns, d) = v.to_numerators_and_denominator();
        assert_eq!(RationalVector::from_numerators_and_denominator(&ns, &d), v);
    });

    integer_vector_gen().test_properties(|ns| {
        for d in [1u32, 2, 6, 1000] {
            let d = Natural::from(d);
            let v = RationalVector::from_numerators_and_denominator(&ns, &d);
            assert_eq!(v.dimension(), ns.dimension());
            for (x, n) in v.elements.iter().zip(ns.elements.iter()) {
                assert_eq!(*x, Rational::from(n) / Rational::from(&d));
            }
        }
        // Over a denominator of 1, the elements are the numerators.
        let v = RationalVector::from_numerators_and_denominator(&ns, &Natural::from(1u32));
        assert_eq!(v.to_numerators_and_denominator(), (ns, Natural::from(1u32)));
    });
}
