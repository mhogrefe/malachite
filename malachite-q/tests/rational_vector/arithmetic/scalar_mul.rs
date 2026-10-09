// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Abs, L1Norm};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_integer_pair_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_rational_pair_gen;

#[test]
fn test_mul_scalar() {
    let test = |s, c, out| {
        let v = RationalVector::from_str(s).unwrap();
        let c = Rational::from_str(c).unwrap();
        // All four combinations of value and reference, on either side, and in place with both.
        let w = &v * &c;
        assert_eq!(w.to_string(), out);
        assert_eq!(&v * c.clone(), w);
        assert_eq!(v.clone() * &c, w);
        assert_eq!(v.clone() * c.clone(), w);
        assert_eq!(&c * &v, w);
        assert_eq!(&c * v.clone(), w);
        assert_eq!(c.clone() * &v, w);
        assert_eq!(c.clone() * v.clone(), w);
        let mut x = v.clone();
        x *= &c;
        assert_eq!(x, w);
        let mut x = v;
        x *= c;
        assert_eq!(x, w);
    };
    test("()", "5", "()");
    test("(1/2, -2/3, 3)", "0", "(0, 0, 0)");
    test("(1/2, -2/3, 3)", "1", "(1/2, -2/3, 3)");
    test("(1/2, -2/3, 3)", "-1", "(-1/2, 2/3, -3)");
    // Each product is in lowest terms.
    test("(1/2, -2/3, 3)", "3/4", "(3/8, -1/2, 9/4)");
    test("(2/3, 4/5)", "15/2", "(5, 6)");
}

#[test]
fn mul_scalar_properties() {
    rational_vector_rational_pair_gen().test_properties(|(v, c)| {
        let w = &v * &c;
        // The forms agree.
        assert_eq!(&v * c.clone(), w);
        assert_eq!(v.clone() * &c, w);
        assert_eq!(v.clone() * c.clone(), w);
        assert_eq!(&c * &v, w);
        assert_eq!(&c * v.clone(), w);
        assert_eq!(c.clone() * &v, w);
        assert_eq!(c.clone() * v.clone(), w);
        let mut x = v.clone();
        x *= &c;
        assert_eq!(x, w);
        let mut x = v.clone();
        x *= c.clone();
        assert_eq!(x, w);

        // Element by element, this is the scalar product, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x * &c);
        }
        // Multiplying by 0 gives the zero vector, by 1 changes nothing, and products of scalars act
        // one after the other.
        assert_eq!(&v * Rational::ZERO, RationalVector::zero(v.dimension()));
        assert_eq!(&v * Rational::ONE, v);
        // Multiplying by -1 negates.
        assert_eq!(&v * Rational::NEGATIVE_ONE, -&v);
        assert_eq!(&v * (&c * &c), &w * &c);
        // It distributes over vector addition.
        assert_eq!((&v + &v) * &c, &w + &w);
        // The l^1 norm scales by the absolute value of the scalar.
        assert_eq!(w.to_l1_norm(), v.to_l1_norm() * (&c).abs());
    });

    integer_vector_integer_pair_gen().test_properties(|(v, c)| {
        // An integer vector times an integer gives the same product as rationals.
        assert_eq!(
            RationalVector::from(&v * &c),
            RationalVector::from(v) * Rational::from(c)
        );
    });
}
