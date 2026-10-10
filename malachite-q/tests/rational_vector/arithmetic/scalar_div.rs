// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Abs, L1Norm, PowerOf2, Reciprocal};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_rational_pair_gen_var_1, rational_vector_unsigned_pair_gen_var_2,
};
use std::panic::catch_unwind;

#[test]
fn test_div_scalar() {
    let test = |s, c, out| {
        let v = RationalVector::from_str(s).unwrap();
        let c = Rational::from_str(c).unwrap();
        // All four combinations of value and reference, and in place with both.
        let w = &v / &c;
        assert_eq!(w.to_string(), out);
        assert_eq!(&v / c.clone(), w);
        assert_eq!(v.clone() / &c, w);
        assert_eq!(v.clone() / c.clone(), w);
        let mut x = v.clone();
        x /= &c;
        assert_eq!(x, w);
        let mut x = v;
        x /= c;
        assert_eq!(x, w);
    };
    test("()", "5", "()");
    test("(1/2, -2/3, 3)", "1", "(1/2, -2/3, 3)");
    test("(1/2, -2/3, 3)", "-1", "(-1/2, 2/3, -3)");
    // Each quotient is in lowest terms.
    test("(1/2, -2/3, 3)", "3/4", "(2/3, -8/9, 4)");
    test("(5, 6)", "15/2", "(2/3, 4/5)");
    test("(0, 4, -6)", "-2", "(0, -2, 3)");
}

#[test]
fn div_scalar_fail() {
    let v = RationalVector::from_str("(1/2, 3)").unwrap();
    assert_panic!(&v / Rational::ZERO);
    assert_panic!(&v / &Rational::ZERO);
    assert_panic!(v.clone() / Rational::ZERO);
    assert_panic!(v.clone() / &Rational::ZERO);
    assert_panic!(RationalVector::from_str("()").unwrap() / Rational::ZERO);
    assert_panic!({
        let mut w = v.clone();
        w /= Rational::ZERO;
    });
    assert_panic!({
        let mut w = v.clone();
        w /= &Rational::ZERO;
    });
}

#[test]
fn div_scalar_properties() {
    rational_vector_rational_pair_gen_var_1().test_properties(|(v, c)| {
        let w = &v / &c;
        // The forms agree.
        assert_eq!(&v / c.clone(), w);
        assert_eq!(v.clone() / &c, w);
        assert_eq!(v.clone() / c.clone(), w);
        let mut x = v.clone();
        x /= &c;
        assert_eq!(x, w);
        let mut x = v.clone();
        x /= c.clone();
        assert_eq!(x, w);

        // Element by element, this is the scalar quotient, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x / &c);
        }
        // It is multiplication by the reciprocal, and multiplying by the scalar undoes it.
        assert_eq!(&v * (&c).reciprocal(), w);
        assert_eq!(&w * &c, v);
        // Dividing by 1 changes nothing, dividing by -1 negates, and quotients by scalars act one
        // after the other.
        assert_eq!(&v / Rational::ONE, v);
        assert_eq!(&v / Rational::NEGATIVE_ONE, -&v);
        assert_eq!(&v / (&c * &c), &w / &c);
        // It distributes over vector addition.
        assert_eq!((&v + &v) / &c, &w + &w);
        // The l^1 norm is divided by the absolute value of the scalar.
        assert_eq!(w.to_l1_norm(), v.to_l1_norm() / (&c).abs());
    });

    rational_vector_unsigned_pair_gen_var_2::<u64>().test_properties(|(v, bits)| {
        // Dividing by a power of 2 is a right shift.
        assert_eq!(&v / Rational::power_of_2(bits), &v >> bits);
    });
}
