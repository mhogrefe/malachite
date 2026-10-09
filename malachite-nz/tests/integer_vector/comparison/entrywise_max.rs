// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::max;
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{EntrywiseAbs, EntrywiseMax, EntrywiseMaxAssign};
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_pair_gen_var_1, integer_vector_triple_gen_var_1, natural_vector_pair_gen_var_1,
};

#[test]
fn test_entrywise_max() {
    let test = |s, t, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let w = IntegerVector::from_str(t).unwrap();
        // All four combinations of value and reference, and in place with both.
        let r = (&v).entrywise_max(&w);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).entrywise_max(w.clone()), r);
        assert_eq!(v.clone().entrywise_max(&w), r);
        assert_eq!(v.clone().entrywise_max(w.clone()), r);
        let mut x = v.clone();
        x.entrywise_max_assign(&w);
        assert_eq!(x, r);
        let mut x = v;
        x.entrywise_max_assign(w);
        assert_eq!(x, r);
    };
    test("()", "()", "()");
    test("(0)", "(0)", "(0)");
    test("(1, -5, 3)", "(-4, 2, 3)", "(1, 2, 3)");
    test(
        "(-18446744073709551616, -1)",
        "(-18446744073709551615, -2)",
        "(-18446744073709551615, -1)",
    );
}

#[test]
#[should_panic]
fn entrywise_max_fail() {
    let _ = IntegerVector::from_str("(1, 2)")
        .unwrap()
        .entrywise_max(IntegerVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_max_ref_ref_fail() {
    let _ = (&IntegerVector::from_str("(1, 2)").unwrap())
        .entrywise_max(&IntegerVector::from_str("(1)").unwrap());
}

#[test]
#[should_panic]
fn entrywise_max_assign_fail() {
    let mut v = IntegerVector::from_str("(1)").unwrap();
    v.entrywise_max_assign(IntegerVector::from_str("(1, 2)").unwrap());
}

#[test]
fn entrywise_max_properties() {
    integer_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let r = (&v).entrywise_max(&w);
        // The forms agree.
        assert_eq!((&v).entrywise_max(w.clone()), r);
        assert_eq!(v.clone().entrywise_max(&w), r);
        assert_eq!(v.clone().entrywise_max(w.clone()), r);
        let mut x = v.clone();
        x.entrywise_max_assign(&w);
        assert_eq!(x, r);
        let mut x = v.clone();
        x.entrywise_max_assign(w.clone());
        assert_eq!(x, r);

        // Element by element, this is the maximum, and the dimension is unchanged.
        assert_eq!(r.dimension(), v.dimension());
        for ((x, y), z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
            assert_eq!(z, max(x, y));
        }
        // It is commutative and idempotent.
        assert_eq!((&w).entrywise_max(&v), r);
        assert_eq!((&v).entrywise_max(&v), v);
        assert_eq!((&r).entrywise_max(&v), r);
        // The entrywise maximum with the negation is the entrywise absolute value.
        assert_eq!((&v).entrywise_max(-&v), (&v).entrywise_abs());
    });

    integer_vector_triple_gen_var_1().test_properties(|(u, v, w)| {
        // It is associative.
        assert_eq!(
            (&u).entrywise_max(&v).entrywise_max(&w),
            u.entrywise_max(v.entrywise_max(w))
        );
    });

    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // Non-negative vectors give the same result as NaturalVectors.
        assert_eq!(
            IntegerVector::from(v.clone()).entrywise_max(IntegerVector::from(w.clone())),
            IntegerVector::from(v.entrywise_max(w))
        );
    });
}
