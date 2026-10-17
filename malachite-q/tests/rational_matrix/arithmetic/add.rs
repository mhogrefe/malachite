// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_1;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_gen, rational_matrix_pair_gen_var_1, rational_matrix_triple_gen_var_1,
};

#[test]
fn test_add() {
    let test = |s, t, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        let b = RationalMatrix::from_str(t).unwrap();
        assert_eq!((a.clone() + b.clone()).to_string(), out);
        assert_eq!((a.clone() + &b).to_string(), out);
        assert_eq!((&a + b.clone()).to_string(), out);
        assert_eq!((&a + &b).to_string(), out);
        let mut x = a.clone();
        x += b.clone();
        assert_eq!(x.to_string(), out);
        let mut x = a;
        x += &b;
        assert_eq!(x.to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[0]", "[0]", "[0]");
    test("[]_{3,0}", "[]_{3,0}", "[]_{3,0}");
    test("[1/2, -2; 3, 0]", "[1/3, 5; -3, 1]", "[5/6, 3; 0, 1]");
    test("[1/2, -2, 3]", "[1/3, 5, -3]", "[5/6, 3, 0]");
    test(
        "[1/1000000000000, 1]",
        "[-1/1000000000000, 1/2]",
        "[0, 3/2]",
    );
}

fn pair() -> (RationalMatrix, RationalMatrix) {
    (
        RationalMatrix::from_str("[1, 2]").unwrap(),
        RationalMatrix::from_str("[1]").unwrap(),
    )
}

#[test]
#[should_panic]
fn add_fail_1() {
    let (a, b) = pair();
    let _ = a + b;
}

#[test]
#[should_panic]
fn add_fail_2() {
    let (a, b) = pair();
    let _ = a + &b;
}

#[test]
#[should_panic]
fn add_fail_3() {
    let (a, b) = pair();
    let _ = &a + b;
}

#[test]
#[should_panic]
fn add_fail_4() {
    let (a, b) = pair();
    let _ = &a + &b;
}

#[test]
#[should_panic]
fn add_assign_fail_1() {
    let (mut a, b) = pair();
    a += b;
}

#[test]
#[should_panic]
fn add_assign_fail_2() {
    let (mut a, b) = pair();
    a += &b;
}

#[test]
#[should_panic]
fn add_empty_fail() {
    // Even a matrix with no elements cannot be added to a matrix of another shape.
    let _ =
        &RationalMatrix::from_str("[]_{0,0}").unwrap() + &RationalMatrix::from_str("[0]").unwrap();
}

#[test]
#[should_panic]
fn add_shape_fail_1() {
    // The same number of elements is not enough: the shapes must match.
    let _ =
        &RationalMatrix::from_str("[1, 2]").unwrap() + &RationalMatrix::from_str("[1; 2]").unwrap();
}

#[test]
#[should_panic]
fn add_shape_fail_2() {
    // Matrices with no elements must have the same shape too.
    let _ = &RationalMatrix::from_str("[]_{2,0}").unwrap()
        + &RationalMatrix::from_str("[]_{0,2}").unwrap();
}

#[test]
fn add_properties() {
    rational_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        let sum = &a + &b;
        assert_eq!(a.clone() + b.clone(), sum);
        assert_eq!(a.clone() + &b, sum);
        assert_eq!(&a + b.clone(), sum);
        let mut x = a.clone();
        x += b.clone();
        assert_eq!(x, sum);
        let mut x = a.clone();
        x += &b;
        assert_eq!(x, sum);

        // The sum is taken element by element, and addition is commutative.
        assert!(sum.is_valid());
        assert_eq!((sum.height(), sum.width()), (a.height(), a.width()));
        for ((x, y), z) in a.elements.iter().zip(&b.elements).zip(&sum.elements) {
            assert_eq!(x + y, *z);
        }
        assert_eq!(&b + &a, sum);
        // Addition commutes with transposition.
        assert_eq!(a.transpose() + b.transpose(), sum.transpose());
    });

    rational_matrix_triple_gen_var_1().test_properties(|(c, a, b)| {
        // Addition is associative.
        assert_eq!((&c + &a) + &b, &c + (&a + &b));
    });

    rational_matrix_gen().test_properties(|a| {
        // The zero matrix of the same shape is the identity.
        let zero = RationalMatrix::zero(a.height(), a.width());
        assert_eq!(&a + &zero, a);
        assert_eq!(&zero + &a, a);
        // Adding the negation gives zero.
        assert!((&a + -&a).is_zero());
    });

    integer_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        // The sum agrees with the sum of the matrices as `IntegerMatrix`s.
        assert_eq!(
            RationalMatrix::from(a.clone()) + RationalMatrix::from(b.clone()),
            RationalMatrix::from(a + b)
        );
    });
}
