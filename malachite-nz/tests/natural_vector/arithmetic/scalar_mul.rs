// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::L1Norm;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_natural_pair_gen;

#[test]
fn test_mul_scalar() {
    let test = |s, c, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
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
    test("(1, 2, 3)", "0", "(0, 0, 0)");
    test("(1, 2, 3)", "1", "(1, 2, 3)");
    test("(1, 2, 3)", "3", "(3, 6, 9)");
    test(
        "(18446744073709551615, 1)",
        "18446744073709551617",
        "(340282366920938463463374607431768211455, 18446744073709551617)",
    );
}

#[test]
fn mul_scalar_properties() {
    natural_vector_natural_pair_gen().test_properties(|(v, c)| {
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
        assert_eq!(&v * Natural::ZERO, NaturalVector::zero(v.dimension()));
        assert_eq!(&v * Natural::ONE, v);
        assert_eq!(&v * (&c * &c), &w * &c);
        // It distributes over vector addition.
        assert_eq!((&v + &v) * &c, &w + &w);
        // The l^1 norm scales by the scalar.
        assert_eq!(w.to_l1_norm(), v.to_l1_norm() * &c);
    });

    natural_vector_natural_pair_gen().test_properties(|(v, c)| {
        // As an IntegerVector and an Integer, the product is the same.
        assert_eq!(
            IntegerVector::from(&v * &c),
            IntegerVector::from(v) * Integer::from(c)
        );
    });
}
