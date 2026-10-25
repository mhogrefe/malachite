// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_natural_pair_gen;

#[test]
fn test_mul_scalar() {
    let test = |s, c, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
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
    test("[1, 2; 3, 4]", "3", "[3, 6; 9, 12]");
    test("[1, 2, 3]", "0", "[0, 0, 0]");
    test("[1, 2, 3]", "1", "[1, 2, 3]");
    test("[1, 2, 3]", "3", "[3, 6, 9]");
    test(
        "[18446744073709551615, 1]",
        "18446744073709551617",
        "[340282366920938463463374607431768211455, 18446744073709551617]",
    );
}

#[test]
fn mul_scalar_properties() {
    natural_matrix_natural_pair_gen().test_properties(|(a, c)| {
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
        for (x, y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*y, x * &c);
        }
        // Multiplying by 0 gives the zero matrix, by 1 changes nothing, and products of scalars act
        // one after the other.
        assert_eq!(
            &a * Natural::ZERO,
            NaturalMatrix::zero(a.height(), a.width())
        );
        assert_eq!(&a * Natural::ONE, a);
        assert_eq!(&a * (&c * &c), &b * &c);
        // It commutes with transposition and distributes over matrix addition.
        assert_eq!(a.transpose() * &c, b.transpose());
        assert_eq!((&a + &a) * &c, &b + &b);
    });

    natural_matrix_natural_pair_gen().test_properties(|(a, c)| {
        // As an IntegerMatrix and an Integer, the product is the same.
        assert_eq!(
            IntegerMatrix::from(&a * &c),
            IntegerMatrix::from(a) * Integer::from(c)
        );
    });
}
