// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::min;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    EntrywiseAbs, EntrywiseMax, EntrywiseMin, EntrywiseMinAssign,
};
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_pair_gen_var_1;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_pair_gen_var_1, rational_vector_triple_gen_var_1,
};

#[test]
fn test_entrywise_min() {
    let test = |s, t, out| {
        let v = RationalVector::from_str(s).unwrap();
        let w = RationalVector::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&v).entrywise_min(&w);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).entrywise_min(w.clone()), r);
        assert_eq!(v.clone().entrywise_min(&w), r);
        assert_eq!(v.clone().entrywise_min(w.clone()), r);
        let mut x = v.clone();
        x.entrywise_min_assign(&w);
        assert_eq!(x, r);
        let mut x = v;
        x.entrywise_min_assign(w);
        assert_eq!(x, r);
    };
    test("()", "()", "()");
    test("(0)", "(0)", "(0)");
    test("(1/2, -5, 3)", "(1/3, 2, 3)", "(1/3, -5, 3)");
    test("(-1/2, -1/3)", "(-2/3, -1/4)", "(-2/3, -1/3)");
}

#[test]
#[should_panic]
fn entrywise_min_fail() {
    let _ = RationalVector::from_str("(1, 2)")
        .unwrap()
        .entrywise_min(RationalVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_min_ref_ref_fail() {
    let _ = (&RationalVector::from_str("(1, 2)").unwrap())
        .entrywise_min(&RationalVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_min_assign_fail() {
    let mut v = RationalVector::from_str("(1)").unwrap();
    v.entrywise_min_assign(RationalVector::from_str("(1, 2)").unwrap());
}

#[test]
fn entrywise_min_properties() {
    rational_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let r = (&v).entrywise_min(&w);
        // The forms agree.
        assert_eq!((&v).entrywise_min(w.clone()), r);
        assert_eq!(v.clone().entrywise_min(&w), r);
        assert_eq!(v.clone().entrywise_min(w.clone()), r);
        let mut x = v.clone();
        x.entrywise_min_assign(&w);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.entrywise_min_assign(w.clone());
        assert_eq!(x, r);

        // Element by element, this is the minimum, and the dimension is unchanged.
        assert_eq!(r.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(z, min(x, y));
        }
        // It is commutative and idempotent.
        assert_eq!((&w).entrywise_min(&v), r);
        assert_eq!((&v).entrywise_min(&v), v);
        assert_eq!((&r).entrywise_min(&v), r);
        // The entrywise minimum with the negation is the negated entrywise absolute value, and the
        // minimum and maximum add up to the sum.
        assert_eq!((&v).entrywise_min(-&v), -(&v).entrywise_abs());
        assert_eq!(&r + (&v).entrywise_max(&w), &v + &w);
    });

    rational_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // It is associative.
        assert_eq!(
            (&u).entrywise_min(&v).entrywise_min(&w),
            u.entrywise_min(v.entrywise_min(w))
        );
    });

    integer_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // Integer vectors give the same result as RationalVectors.
        assert_eq!(
            RationalVector::from(v.clone()).entrywise_min(RationalVector::from(w.clone())),
            RationalVector::from(v.entrywise_min(w))
        );
    });
}
