// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;

#[test]
fn test_from_integer_matrix() {
    let test = |s| {
        let m = RationalMatrix::from(IntegerMatrix::from_str(s).unwrap());
        assert!(m.is_valid());
        assert_eq!(m.to_string(), s);
    };
    test("[]_{0,0}");
    test("[]_{0,4}");
    test("[]_{4,0}");
    test("[0]");
    test("[1, 2, 3; 4, 5, 6]");
    test("[1; 2; 3]");
    test("[123456789012345678901234567890, 0; 1, 18446744073709551616]");
    test("[1, -2; -3, 4]");
    test("[-123456789012345678901234567890; 0]");
}

#[test]
fn from_integer_matrix_properties() {
    integer_matrix_gen().test_properties(|m| {
        let n = RationalMatrix::from(m.clone());
        assert!(n.is_valid());
        // Nothing is lost: the shape, every element, and the written form all survive.
        assert_eq!((n.height(), n.width()), (m.height(), m.width()));
        assert_eq!(n.to_string(), m.to_string());
        assert_eq!(
            n.elements,
            m.elements
                .iter()
                .map(|x| Rational::from(x.clone()))
                .collect::<Vec<_>>()
        );
        // Every element is an integer.
        assert!(n.elements.iter().all(|x| *x.denominator_ref() == 1u32));
        // Reading the string back as a `RationalMatrix` gives the same thing.
        assert_eq!(RationalMatrix::from_str(&m.to_string()).unwrap(), n);
        // Converting commutes with transposing.
        assert_eq!(RationalMatrix::from(m.transpose()), n.transpose());
    });
}
