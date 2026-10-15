// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Cross, CrossAssign, Dot};
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_gen_var_1, integer_vector_pair_gen_var_2, integer_vector_triple_gen_var_2,
};
use malachite_nz::test_util::integer_vector::arithmetic::cross::integer_vector_cross_naive;

fn check(v: &IntegerVector, w: &IntegerVector) -> IntegerVector {
    let r = v.cross(w);
    assert_eq!(v.cross(w.clone()), r);
    assert_eq!(v.clone().cross(w), r);
    assert_eq!(v.clone().cross(w.clone()), r);
    let mut s = v.clone();
    s.cross_assign(w);
    assert_eq!(s, r);
    let mut s = v.clone();
    s.cross_assign(w.clone());
    assert_eq!(s, r);
    r
}

#[test]
fn test_cross() {
    let test = |s, t, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let w = IntegerVector::from_str(t).unwrap();
        let r = check(&v, &w);
        assert_eq!(r.to_string(), out);
        assert_eq!(integer_vector_cross_naive(&v, &w), r);
    };
    test("(0, 0, 0)", "(1, 2, 3)", "(0, 0, 0)");
    test("(1, 2, 3)", "(0, 0, 0)", "(0, 0, 0)");
    test("(1, 0, 0)", "(0, 1, 0)", "(0, 0, 1)");
    test("(0, 1, 0)", "(0, 0, 1)", "(1, 0, 0)");
    test("(0, 0, 1)", "(1, 0, 0)", "(0, 1, 0)");
    test("(0, 1, 0)", "(1, 0, 0)", "(0, 0, -1)");
    test("(1, 2, 3)", "(4, 5, 6)", "(-3, 6, -3)");
    test("(1, 2, 3)", "(2, 4, 6)", "(0, 0, 0)");
    test("(1, 2, 3)", "(1, 2, 3)", "(0, 0, 0)");
    test("(-1, 2, -3)", "(4, -5, 6)", "(-3, -6, -3)");
    test("(3, -3, 1)", "(4, 9, 2)", "(-15, -2, 39)");
    test(
        "(18446744073709551616, -18446744073709551616, 1)",
        "(18446744073709551617, 7, -18446744073709551615)",
        "(340282366920938463444927863358058659833, 340282366920938463463374607431768211457, \
        340282366920938463610948560021444624384)",
    );
    test(
        "(1267650600228229401496703205376, 0, -1)",
        "(1, 1180591620717411303424, 3)",
        "(1180591620717411303424, -3802951800684688204490109616129, \
        1496577676626844588240573268701473812127674924007424)",
    );
}

#[test]
#[should_panic]
fn cross_fail_1() {
    IntegerVector::from_str("(1, 2)")
        .unwrap()
        .cross(IntegerVector::from_str("(1, 2)").unwrap());
}

#[test]
#[should_panic]
fn cross_fail_2() {
    IntegerVector::from_str("(1, 2, 3)")
        .unwrap()
        .cross(IntegerVector::from_str("(1, 2, 3, 4)").unwrap());
}

#[test]
#[should_panic]
fn cross_assign_fail() {
    let mut v = IntegerVector::from_str("(1, 2, 3, 4)").unwrap();
    v.cross_assign(IntegerVector::from_str("(1, 2, 3, 4)").unwrap());
}

#[test]
fn cross_properties() {
    integer_vector_pair_gen_var_2().test_properties(|(v, w)| {
        let r = check(&v, &w);
        assert_eq!(integer_vector_cross_naive(&v, &w), r);
        // Swapping the vectors negates the product.
        assert_eq!((&w).cross(&v), -&r);
        // The product is perpendicular to both vectors.
        assert_eq!((&r).dot(&v), 0u32);
        assert_eq!((&r).dot(&w), 0u32);
        // Lagrange's identity: |v x w|^2 = |v|^2 |w|^2 - (v . w)^2.
        let vw = (&v).dot(&w);
        assert_eq!((&r).dot(&r), (&v).dot(&v) * (&w).dot(&w) - &vw * &vw);
    });

    integer_vector_gen_var_1().test_properties(|v| {
        // A vector's product with itself, or with zero, is zero.
        let zero = IntegerVector::zero(3);
        assert!((&v).cross(&v).is_zero());
        assert!((&v).cross(&zero).is_zero());
        assert!((&zero).cross(&v).is_zero());
    });

    integer_vector_triple_gen_var_2().test_properties(|(u, v, w)| {
        // The product distributes over addition.
        assert_eq!((&u + &v).cross(&w), (&u).cross(&w) + (&v).cross(&w));
        // The scalar triple product is invariant under cyclic permutation.
        let triple = (&u).cross(&v).dot(&w);
        assert_eq!((&v).cross(&w).dot(&u), triple);
        assert_eq!((&w).cross(&u).dot(&v), triple);
        // The Jacobi identity: u x (v x w) + v x (w x u) + w x (u x v) = 0.
        let sum =
            (&u).cross((&v).cross(&w)) + (&v).cross((&w).cross(&u)) + (&w).cross((&u).cross(&v));
        assert!(sum.is_zero());
    });
}
