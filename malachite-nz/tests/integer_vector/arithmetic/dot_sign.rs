// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Dot, DotSign, Sign};
use malachite_base::num::basic::traits::One;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_pair_gen_var_1, natural_vector_pair_gen_var_1,
};

#[test]
fn test_dot_sign() {
    let test = |a, b, out: Ordering| {
        let v = IntegerVector::from_str(a).unwrap();
        let w = IntegerVector::from_str(b).unwrap();
        assert_eq!((&v).dot_sign(&w), out);
        assert_eq!((&v).dot_sign(w.clone()), out);
        assert_eq!(v.clone().dot_sign(&w), out);
        assert_eq!(v.clone().dot_sign(w.clone()), out);
        assert_eq!(v.dot(w).sign(), out);
    };
    // - no nonzero terms
    test("()", "()", Equal);
    test("(0, 0, 0)", "(1, -2, 3)", Equal);
    test("(1, 0, 3)", "(0, 5, 0)", Equal);
    // - terms of one sign only
    test("(1, 0, 3)", "(0, 5, 7)", Greater);
    test("(-1, 0, 3)", "(0, 5, -7)", Less);
    // - two-dimensional vectors, through `cmp_products_neg`
    test("(3, 7)", "(7, -3)", Equal);
    test("(3, 7)", "(8, -3)", Greater);
    test("(3, -7)", "(7, 4)", Less);
    // - one term of each sign, among more coordinates
    test("(3, 7, 0)", "(8, -3, 5)", Greater);
    test("(3, 0, 7)", "(7, 5, -3)", Equal);
    // - several terms, decided by the largest term against the rest
    test("(1, 1, 1)", "(1, 1, -2)", Equal);
    test("(1, 1, 1)", "(1, 1, -3)", Less);
    test("(1, 1, 1)", "(1, 1, -1)", Greater);
    test(
        "(1000000000000, 2, -3, 5)",
        "(1000000000000, -7, 11, -13)",
        Greater,
    );
    test(
        "(1000000000000, 2, -3, 5)",
        "(-1000000000000, -7, 11, -13)",
        Less,
    );
    test(
        "(18446744073709551616, -1, -1)",
        "(18446744073709551616, 170141183460469231731687303715884105728, \
        170141183460469231731687303715884105728)",
        Equal,
    );
    // - a largest term at least twice as wide as the others, weighed against their dot product
    test(
        "(15, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)",
        "(15, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1)",
        Greater,
    );
    // - the interval screen on bit lengths
    test(
        "(1, 1, 1, -1)",
        "(1, 1, 1, 1267650600228229401496703205376)",
        Less,
    );
    test(
        "(-1, -1, -1, 1)",
        "(1, 1, 1, 1267650600228229401496703205376)",
        Greater,
    );
    test("(3, 5, 7, -11, 13)", "(5, 7, 11, 1000000000000, 17)", Less);
}

#[test]
fn dot_sign_grid() {
    // Every pair from a grid of vectors agrees with the sign of the dot product.
    let grid: Vec<IntegerVector> = [
        "(0, 0, 0)",
        "(1, 0, 0)",
        "(0, -2, 0)",
        "(1, 1, 1)",
        "(1, -1, 1)",
        "(-5, 0, 7)",
        "(1, 2, -3)",
        "(255, -256, 65536)",
        "(18446744073709551616, -3, 0)",
        "(-1, 18446744073709551615, 1)",
    ]
    .iter()
    .map(|s| IntegerVector::from_str(s).unwrap())
    .collect();
    for v in &grid {
        for w in &grid {
            assert_eq!(v.dot_sign(w), v.dot(w).sign());
        }
    }
}

#[test]
#[should_panic]
fn dot_sign_fail() {
    IntegerVector::from_str("(1, 2)")
        .unwrap()
        .dot_sign(IntegerVector::from_str("(1)").unwrap());
}

#[test]
fn dot_sign_properties() {
    integer_vector_pair_gen_var_1().test_properties(|(v, w)| {
        let sign = (&v).dot_sign(&w);
        assert_eq!((&v).dot_sign(w.clone()), sign);
        assert_eq!(v.clone().dot_sign(&w), sign);
        assert_eq!(v.clone().dot_sign(w.clone()), sign);
        // It is the sign of the dot product, it is symmetric, and negating a vector reverses it.
        let dot = (&v).dot(&w);
        assert_eq!(dot.sign(), sign);
        assert_eq!((&w).dot_sign(&v), sign);
        assert_eq!((-&v).dot_sign(&w), sign.reverse());
        // Appending the negative of the dot product as a term makes the dot product zero, which
        // only the exact computation can decide.
        let mut xs = v.into_elements();
        let mut ys = w.into_elements();
        xs.push(Integer::ONE);
        ys.push(-dot);
        assert_eq!(
            IntegerVector::from_owned_elements(xs).dot_sign(IntegerVector::from_owned_elements(ys)),
            Equal
        );
    });

    natural_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // It agrees with the NaturalVector implementation.
        assert_eq!(
            IntegerVector::from(v.clone()).dot_sign(IntegerVector::from(w.clone())),
            v.dot_sign(w)
        );
    });
}
