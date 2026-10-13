// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddDot, AddDotAssign, Dot, SubDot};
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_integer_vector_integer_triple_gen_var_1;
use malachite_nz::test_util::integer_vector::arithmetic::dot_general::vec_dot_general_naive;

#[test]
fn test_add_dot() {
    let test = |x, v, w, out| {
        let x = Integer::from_str(x).unwrap();
        let v = IntegerVector::from_str(v).unwrap();
        let w = IntegerVector::from_str(w).unwrap();
        let r = (&x).add_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(r.to_string(), out);
        assert_eq!(x.clone().add_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().add_dot(v.clone(), &w), r);
        assert_eq!(x.clone().add_dot(&v, w.clone()), r);
        assert_eq!(x.clone().add_dot(&v, &w), r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, &w);
        assert_eq!(y, r);
    };
    test("0", "()", "()", "0");
    test("7", "(1, -2, 3)", "(4, 5, -6)", "-17");
    test("-5", "(3)", "(-4)", "-17");
    test(
        "1000000000000000000000",
        "(-1000000000000000000000, 1)",
        "(1000000000000000000000, 1000000000000000000000)",
        "-999999999999999999998000000000000000000000",
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
fn add_dot_fail_1() {
    Integer::from_str("1").unwrap().add_dot(
        IntegerVector::from_str("(1, 2)").unwrap(),
        IntegerVector::from_str("(1)").unwrap(),
    );
}

#[test]
fn add_dot_properties() {
    integer_vector_integer_vector_integer_triple_gen_var_1().test_properties(|(v, w, x)| {
        let r = (&x).add_dot(&v, &w);
        assert!(r.is_valid());
        assert_eq!(x.clone().add_dot(v.clone(), w.clone()), r);
        assert_eq!(x.clone().add_dot(v.clone(), &w), r);
        assert_eq!(x.clone().add_dot(&v, w.clone()), r);
        assert_eq!(x.clone().add_dot(&v, &w), r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(v.clone(), &w);
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, w.clone());
        assert_eq!(y, r);
        let mut y = x.clone();
        y.add_dot_assign(&v, &w);
        assert_eq!(y, r);
        // It is the dot product added to `x`.
        assert_eq!(r, &x + (&v).dot(&w));
        // Subtracting it back recovers `x`.
        assert_eq!((&r).sub_dot(&v, &w), x);
        assert_eq!(
            vec_dot_general_naive(Some(&x), false, &v.elements, &w.elements, false),
            r
        );
    });
}
