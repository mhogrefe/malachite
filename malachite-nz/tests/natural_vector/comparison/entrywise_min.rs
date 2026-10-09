// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::min;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{EntrywiseMax, EntrywiseMin, EntrywiseMinAssign};
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_pair_gen_var_1, natural_vector_triple_gen_var_1,
};

#[test]
fn test_entrywise_min() {
    let test = |s, t, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let w = NaturalVector::from_str(t).unwrap();
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
    test(
        "(18446744073709551616, 0)",
        "(1, 18446744073709551615)",
        "(1, 0)",
    );
}

#[test]
#[should_panic]
fn entrywise_min_fail() {
    let _ = NaturalVector::from_str("(1, 2)")
        .unwrap()
        .entrywise_min(NaturalVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_min_ref_ref_fail() {
    let _ = (&NaturalVector::from_str("(1, 2)").unwrap())
        .entrywise_min(&NaturalVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_min_assign_fail() {
    let mut v = NaturalVector::from_str("(1)").unwrap();
    v.entrywise_min_assign(NaturalVector::from_str("(1, 2)").unwrap());
}

#[test]
fn entrywise_min_properties() {
    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
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
        // Zero is the smallest element, so the minimum with the zero vector is the zero vector.
        assert_eq!(
            (&v).entrywise_min(NaturalVector::zero(v.dimension())),
            NaturalVector::zero(v.dimension())
        );
        // The minimum and maximum add up to the sum.
        assert_eq!(&r + (&v).entrywise_max(&w), &v + &w);
    });

    natural_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // It is associative.
        assert_eq!(
            (&u).entrywise_min(&v).entrywise_min(&w),
            u.entrywise_min(v.entrywise_min(w))
        );
    });

    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // As IntegerVectors, the result is the same.
        assert_eq!(
            IntegerVector::from((&v).entrywise_min(&w)),
            IntegerVector::from(v).entrywise_min(IntegerVector::from(w))
        );
    });
}
