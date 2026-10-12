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
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_pair_gen_var_1;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_pair_gen_var_1;

#[test]
fn test_dot_sign() {
    let test = |a, b, out: Ordering| {
        let v = RationalVector::from_str(a).unwrap();
        let w = RationalVector::from_str(b).unwrap();
        assert_eq!((&v).dot_sign(&w), out);
        assert_eq!((&v).dot_sign(w.clone()), out);
        assert_eq!(v.clone().dot_sign(&w), out);
        assert_eq!(v.clone().dot_sign(w.clone()), out);
        assert_eq!(v.dot(w).sign(), out);
    };
    // - no nonzero terms
    test("()", "()", Equal);
    // - two-dimensional vectors, through `cmp_products_neg`
    test("(0, 0)", "(1/2, -2/3)", Equal);
    test("(1/2, 1/3)", "(2/3, -1)", Equal);
    test("(1/2, 1/3)", "(2/3, -1/2)", Greater);
    test("(1/2, -1/3)", "(2/3, 3/2)", Less);
    // - several terms of both signs, through the numerators
    test("(1/2, 1/3, 1/6)", "(1, 1, -5)", Equal);
    test("(1/2, 1/3, 1/6)", "(1, 1, -4)", Greater);
    test("(1/2, 1/3, 1/6)", "(1, 1, -6)", Less);
    test("(1/2, 1/3, 1/6)", "(1, 1, 1)", Greater);
    // - terms of one sign only
    test("(-1/2, -1/3, 0)", "(1, 1, 7)", Less);
    // - one term of each sign, among more coordinates
    test("(1/2, 0, 1/3)", "(2/3, 5, -1)", Equal);
    // - the interval screen on sizes
    test(
        "(1/3, 1/5, 1/7, -1)",
        "(1/3, 1/5, 1/7, 1267650600228229401496703205376)",
        Less,
    );
    test(
        "(1/3, 1/5, 1/7, -1/1267650600228229401496703205376)",
        "(1/3, 1/5, 1/7, 1)",
        Greater,
    );
}

#[test]
fn dot_sign_grid() {
    // Every pair from a grid of vectors agrees with the sign of the dot product.
    let grid: Vec<RationalVector> = [
        "(0, 0, 0)",
        "(1, 0, 0)",
        "(0, -2/3, 0)",
        "(1/2, 1/3, 1/6)",
        "(1/2, -1/3, 1/6)",
        "(-5/7, 0, 7/5)",
        "(1, 2, -3)",
        "(255/256, -256/255, 65536)",
        "(18446744073709551616/3, -3, 0)",
        "(-1/18446744073709551615, 18446744073709551615, 1)",
    ]
    .iter()
    .map(|s| RationalVector::from_str(s).unwrap())
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
    RationalVector::from_str("(1/2, 2)")
        .unwrap()
        .dot_sign(RationalVector::from_str("(1/2)").unwrap());
}

#[test]
fn dot_sign_properties() {
    rational_vector_pair_gen_var_1().test_properties(|(v, w)| {
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
        xs.push(Rational::ONE);
        ys.push(-dot);
        assert_eq!(
            RationalVector::from_owned_elements(xs)
                .dot_sign(RationalVector::from_owned_elements(ys)),
            Equal
        );
    });

    integer_vector_pair_gen_var_1().test_properties(|(v, w)| {
        // It agrees with the IntegerVector implementation.
        assert_eq!(
            RationalVector::from(v.clone()).dot_sign(RationalVector::from(w.clone())),
            IntegerVector::dot_sign(v, w)
        );
    });
}
