// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Dot, ElementSum, EntrywiseMul, EntrywiseMulAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_pair_gen_var_1, rational_vector_triple_gen_var_1,
};

#[test]
fn test_entrywise_mul() {
    let test = |s, t, out| {
        let v = RationalVector::from_str(s).unwrap();
        let w = RationalVector::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&v).entrywise_mul(&w);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).entrywise_mul(w.clone()), r);
        assert_eq!(v.clone().entrywise_mul(&w), r);
        assert_eq!(v.clone().entrywise_mul(w.clone()), r);
        let mut x = v.clone();
        x.entrywise_mul_assign(&w);
        assert_eq!(x, r);
        let mut x = v;
        x.entrywise_mul_assign(w);
        assert_eq!(x, r);
    };
    test("()", "()", "()");
    test("(0)", "(-5/2)", "(0)");
    test("(1/2, -2, 3)", "(4, 1/3, -1/6)", "(2, -2/3, -1/2)");
    test(
        "(-18446744073709551616/3, 3/7)",
        "(3/18446744073709551615, -7/3)",
        "(-18446744073709551616/18446744073709551615, -1)",
    );
}

#[test]
#[should_panic]
fn entrywise_mul_fail() {
    let _ = RationalVector::from_str("(1, 2)")
        .unwrap()
        .entrywise_mul(RationalVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_mul_ref_ref_fail() {
    let _ = (&RationalVector::from_str("(1, 2)").unwrap())
        .entrywise_mul(&RationalVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_mul_assign_fail() {
    let mut v = RationalVector::from_str("(1)").unwrap();
    v.entrywise_mul_assign(RationalVector::from_str("(1, 2)").unwrap());
}

#[test]
fn entrywise_mul_properties() {
    rational_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let r = (&v).entrywise_mul(&w);
        // The forms agree.
        assert_eq!((&v).entrywise_mul(w.clone()), r);
        assert_eq!(v.clone().entrywise_mul(&w), r);
        assert_eq!(v.clone().entrywise_mul(w.clone()), r);
        let mut x = v.clone();
        x.entrywise_mul_assign(&w);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.entrywise_mul_assign(w.clone());
        assert_eq!(x, r);

        // Element by element, this is the product, and the dimension is unchanged.
        assert_eq!(r.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(*z, x * y);
        }
        // It is commutative.
        assert_eq!((&w).entrywise_mul(&v), r);
        // The sum of its elements is the dot product.
        assert_eq!(r.to_element_sum(), (&v).dot(&w));
        // The all-ones vector is the identity, and the zero vector absorbs everything.
        let ones = RationalVector {
            elements: vec![Rational::ONE; v.elements.len()],
        };
        assert_eq!((&v).entrywise_mul(&ones), v);
        let zero = RationalVector::zero(v.dimension());
        assert_eq!((&v).entrywise_mul(&zero), zero);
        // Negating one vector negates the product.
        assert_eq!((-&v).entrywise_mul(&w), -&r);
    });

    rational_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // It is associative, and it distributes over addition.
        assert_eq!(
            (&u).entrywise_mul(&v).entrywise_mul(&w),
            (&u).entrywise_mul((&v).entrywise_mul(&w))
        );
        assert_eq!(
            (&u).entrywise_mul(&v + &w),
            (&u).entrywise_mul(&v) + u.entrywise_mul(w)
        );
    });
}
