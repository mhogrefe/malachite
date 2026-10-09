// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::min;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{EntrywiseMin, EntrywiseMinAssign};
use malachite_base::test_util::generators::{
    unsigned_vector_pair_gen_var_1, unsigned_vector_triple_gen_var_1,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_entrywise_min() {
    let test = |s, t, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = UnsignedVector::<u8>::from_str(t).unwrap();
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
    test("(1, 5, 3)", "(4, 2, 3)", "(1, 2, 3)");
    test("(255, 0)", "(0, 255)", "(0, 0)");
}

#[test]
#[should_panic]
fn entrywise_min_fail() {
    let _ = UnsignedVector::<u8>::from_str("(1, 2)")
        .unwrap()
        .entrywise_min(UnsignedVector::<u8>::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_min_ref_ref_fail() {
    let _ = (&UnsignedVector::<u8>::from_str("(1, 2)").unwrap())
        .entrywise_min(&UnsignedVector::<u8>::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_min_assign_fail() {
    let mut v = UnsignedVector::<u8>::from_str("(1)").unwrap();
    v.entrywise_min_assign(UnsignedVector::<u8>::from_str("(1, 2)").unwrap());
}

#[test]
fn entrywise_min_properties() {
    unsigned_vector_pair_gen_var_1::<u64>().test_properties(|(v, w)| {
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
        for ((&x, &y), &z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(z, min(x, y));
        }
        // It is commutative and idempotent.
        assert_eq!((&w).entrywise_min(&v), r);
        assert_eq!((&v).entrywise_min(&v), v);
        assert_eq!((&r).entrywise_min(&v), r);
        // Zero is the smallest element, so the minimum with the zero vector is the zero vector.
        assert_eq!(
            (&v).entrywise_min(UnsignedVector::<u64>::zero(v.dimension())),
            UnsignedVector::<u64>::zero(v.dimension())
        );
    });

    unsigned_vector_triple_gen_var_1::<u64>().test_properties(|(u, v, w)| {
        // It is associative.
        assert_eq!(
            (&u).entrywise_min(&v).entrywise_min(&w),
            u.entrywise_min(v.entrywise_min(w))
        );
    });

    unsigned_vector_pair_gen_var_1::<u8>().test_properties(|(v, w)| {
        // A narrower element type works the same way.
        let r = (&v).entrywise_min(&w);
        for ((&x, &y), &z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(z, min(x, y));
        }
    });
}
