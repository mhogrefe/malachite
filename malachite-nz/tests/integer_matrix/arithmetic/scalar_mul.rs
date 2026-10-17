// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_integer_pair_gen;

#[test]
fn test_mul_scalar() {
    let test = |s, c, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let c = Integer::from_str(c).unwrap();
        // All four combinations of value and reference, on either side, and in place with both.
        let b = &a * &c;
        assert_eq!(b.to_string(), out);
        assert_eq!(&a * c.clone(), b);
        assert_eq!(a.clone() * &c, b);
        assert_eq!(a.clone() * c.clone(), b);
        assert_eq!(&c * &a, b);
        assert_eq!(&c * a.clone(), b);
        assert_eq!(c.clone() * &a, b);
        assert_eq!(c.clone() * a.clone(), b);
        let mut x = a.clone();
        x *= &c;
        assert_eq!(x, b);
        let mut x = a;
        x *= c;
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "5", "[]_{0,0}");
    test("[]_{3,0}", "5", "[]_{3,0}");
    test("[1, -2; 3, 0]", "-3", "[-3, 6; -9, 0]");
    test("[1, -2, 3]", "0", "[0, 0, 0]");
    test("[1, -2, 3]", "1", "[1, -2, 3]");
    test("[1, -2, 3]", "-1", "[-1, 2, -3]");
    test("[1, -2, 3]", "-3", "[-3, 6, -9]");
}

#[test]
fn mul_scalar_properties() {
    integer_matrix_integer_pair_gen().test_properties(|(a, c)| {
        let b = &a * &c;
        // The forms agree.
        assert_eq!(&a * c.clone(), b);
        assert_eq!(a.clone() * &c, b);
        assert_eq!(a.clone() * c.clone(), b);
        assert_eq!(&c * &a, b);
        assert_eq!(&c * a.clone(), b);
        assert_eq!(c.clone() * &a, b);
        assert_eq!(c.clone() * a.clone(), b);
        let mut x = a.clone();
        x *= &c;
        assert_eq!(x, b);
        let mut x = a.clone();
        x *= c.clone();
        assert_eq!(x, b);

        // Element by element, this is the scalar product, and the shape is unchanged.
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (x, y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(*y, x * &c);
        }
        // Multiplying by 0 gives the zero matrix, by 1 changes nothing, and products of scalars act
        // one after the other.
        assert_eq!(
            &a * Integer::ZERO,
            IntegerMatrix::zero(a.height(), a.width())
        );
        assert_eq!(&a * Integer::ONE, a);
        // Multiplying by -1 negates.
        assert_eq!(&a * Integer::NEGATIVE_ONE, -&a);
        assert_eq!(&a * (&c * &c), &b * &c);
        // It commutes with transposition and distributes over matrix addition.
        assert_eq!(a.transpose() * &c, b.transpose());
        assert_eq!((&a + &a) * &c, &b + &b);
    });
}
