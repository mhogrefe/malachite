// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddDot, Dot, SubDot, SubDotAssign};
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_integer_vector_integer_triple_gen_var_1;
use malachite_nz::test_util::integer_vector::arithmetic::dot_general::vec_dot_general_naive;

#[test]
fn test_sub_dot() {
    let test = |x, v, w, out| {
        let x = Integer::from_str(x).unwrap();
        let v = IntegerVector::from_str(v).unwrap();
        let w = IntegerVector::from_str(w).unwrap();
        let r = (&x).sub_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().sub_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().sub_dot(v.clone(), &w), r);
        assert_eq!(x.clone().sub_dot(&v, w.clone()), r);
        assert_eq!(x.clone().sub_dot(&v, &w), r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, &w);
        assert_eq!(y, r);
    };
    test("0", "()", "()", "0");
    test("7", "(1, -2, 3)", "(4, 5, -6)", "31");
    test("-5", "(3)", "(-4)", "7");
    test(
        "1000000000000000000000",
        "(-1000000000000000000000, 1)",
        "(1000000000000000000000, 1000000000000000000000)",
        "1000000000000000000000000000000000000000000",
    );
    test(
        "18446744073709551616",
        "(18446744073709551615, -18446744073709551615)",
        "(18446744073709551615, 18446744073709551615)",
        "18446744073709551616",
    );
}

#[test]
#[should_panic]
fn sub_dot_fail_1() {
    Integer::from_str("1").unwrap().sub_dot(
        IntegerVector::from_str("(1, 2)").unwrap(),
        IntegerVector::from_str("(1)").unwrap(),
    );
}

#[test]
fn sub_dot_properties() {
    integer_vector_integer_vector_integer_triple_gen_var_1().test_properties(|(v, w, x)| {
        let r = (&x).sub_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(x.clone().sub_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().sub_dot(v.clone(), &w), r);
        assert_eq!(x.clone().sub_dot(&v, w.clone()), r);
        assert_eq!(x.clone().sub_dot(&v, &w), r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.sub_dot_assign(&v, &w);
        assert_eq!(y, r);
        // It is the dot product subtracted from `x`.
        assert_eq!(r, &x - (&v).dot(&w));
        // Adding it back recovers `x`.
        assert_eq!((&r).add_dot(&v, &w), x);
        assert_eq!(
            vec_dot_general_naive(Some(&x), true, &v.elements, &w.elements, false),
            r
        );
    });
}
