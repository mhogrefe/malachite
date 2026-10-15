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
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_pair_gen_var_2, rational_vector_triple_gen_var_2,
};
use malachite_q::test_util::rational_vector::arithmetic::cross::rational_vector_cross_naive;

fn check(v: &RationalVector, w: &RationalVector) -> RationalVector {
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
        let v = RationalVector::from_str(s).unwrap();
        let w = RationalVector::from_str(t).unwrap();
        let r = check(&v, &w);
        assert_eq!(r.to_string(), out);
        assert_eq!(rational_vector_cross_naive(&v, &w), r);
    };
    test("(0, 0, 0)", "(1, 1/2, 3)", "(0, 0, 0)");
    test("(1, 0, 0)", "(0, 1, 0)", "(0, 0, 1)");
    test("(1/2, 0, 1)", "(0, 2/3, -1)", "(-2/3, 1/2, 1/3)");
    test("(1/2, 1/3, 1/4)", "(1, 2, 3)", "(1/2, -5/4, 2/3)");
    test("(1/2, 1/3, 1/4)", "(2, 2/3, 1/2)", "(0, 1/4, -1/3)");
    test(
        "(-1/2, 3/7, -5/11)",
        "(4/3, -2/5, 9/8)",
        "(185/616, -23/528, -13/35)",
    );
    test("(1, 2, 3)", "(4, 5, 6)", "(-3, 6, -3)");
    test(
        "(1/18446744073709551616, -3/1180591620717411303424, 1)",
        "(18446744073709551616/3, 7/5, -1/36893488147419103232)",
        "(-304893000761160863263183648258864317464561/217780714829400616616559748756331655331840, \
        12554203470773361527671578846415332832204710888928069025795/\
        2041694201525630780780247644590609268736, \
        1441151880758558727/92233720368547758080)",
    );
}

#[test]
#[should_panic]
fn cross_fail_1() {
    RationalVector::from_str("(1, 2)")
        .unwrap()
        .cross(RationalVector::from_str("(1, 2)").unwrap());
}

#[test]
#[should_panic]
fn cross_fail_2() {
    RationalVector::from_str("(1, 2, 3)")
        .unwrap()
        .cross(RationalVector::from_str("(1, 2, 3, 4)").unwrap());
}

#[test]
#[should_panic]
fn cross_assign_fail() {
    let mut v = RationalVector::from_str("(1, 2, 3, 4)").unwrap();
    v.cross_assign(RationalVector::from_str("(1, 2, 3, 4)").unwrap());
}

#[test]
fn cross_properties() {
    rational_vector_pair_gen_var_2().test_properties(|(v, w)| {
        let r = check(&v, &w);
        assert_eq!(rational_vector_cross_naive(&v, &w), r);
        // Swapping the vectors negates the product.
        assert_eq!((&w).cross(&v), -&r);
        // The product is perpendicular to both vectors.
        assert_eq!((&r).dot(&v), 0u32);
        assert_eq!((&r).dot(&w), 0u32);
        // Lagrange's identity: |v x w|^2 = |v|^2 |w|^2 - (v . w)^2.
        let vw = (&v).dot(&w);
        assert_eq!((&r).dot(&r), (&v).dot(&v) * (&w).dot(&w) - &vw * &vw);
        // A vector's product with itself is zero.
        assert!((&v).cross(&v).is_zero());
    });

    rational_vector_triple_gen_var_2().test_properties(|(u, v, w)| {
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
