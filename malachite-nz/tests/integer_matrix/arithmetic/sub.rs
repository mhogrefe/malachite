// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_pair_gen_var_1, integer_matrix_triple_gen_var_1,
    natural_matrix_pair_gen_var_1,
};

#[test]
fn test_sub() {
    let test = |s, t, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let b = IntegerMatrix::from_str(t).unwrap();
        assert_eq!((a.clone() - b.clone()).to_string(), out);
        assert_eq!((a.clone() - &b).to_string(), out);
        assert_eq!((&a - b.clone()).to_string(), out);
        assert_eq!((&a - &b).to_string(), out);
        let mut x = a.clone();
        x -= b.clone();
        assert_eq!(x.to_string(), out);
        let mut x = a;
        x -= &b;
        assert_eq!(x.to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}", "[]_{0,0}");
    test("[0]", "[0]", "[0]");
    test("[]_{3,0}", "[]_{3,0}", "[]_{3,0}");
    test("[1, -2; 3, 0]", "[-1, 5; 0, 4]", "[2, -7; 3, -4]");
    test("[1, -2, 3]", "[-1, 5, 0]", "[2, -7, 3]");
    test(
        "[1000000000000000000000, 1]",
        "[1000000000000000000000, -2]",
        "[0, 3]",
    );
}

fn pair() -> (IntegerMatrix, IntegerMatrix) {
    (
        IntegerMatrix::from_str("[1, 2]").unwrap(),
        IntegerMatrix::from_str("[1]").unwrap(),
    )
}

#[test]
#[should_panic]
fn sub_fail_1() {
    let (a, b) = pair();
    let _ = a - b;
}

#[test]
#[should_panic]
fn sub_fail_2() {
    let (a, b) = pair();
    let _ = a - &b;
}

#[test]
#[should_panic]
fn sub_fail_3() {
    let (a, b) = pair();
    let _ = &a - b;
}

#[test]
#[should_panic]
fn sub_fail_4() {
    let (a, b) = pair();
    let _ = &a - &b;
}

#[test]
#[should_panic]
fn sub_assign_fail_1() {
    let (mut a, b) = pair();
    a -= b;
}

#[test]
#[should_panic]
fn sub_assign_fail_2() {
    let (mut a, b) = pair();
    a -= &b;
}

#[test]
#[should_panic]
fn sub_empty_fail() {
    // Even a matrix with no elements cannot be subtracted from a matrix of another shape.
    let _ =
        &IntegerMatrix::from_str("[]_{0,0}").unwrap() - &IntegerMatrix::from_str("[0]").unwrap();
}

#[test]
#[should_panic]
fn sub_shape_fail_1() {
    // The same number of elements is not enough: the shapes must match.
    let _ =
        &IntegerMatrix::from_str("[1, 2]").unwrap() - &IntegerMatrix::from_str("[1; 2]").unwrap();
}

#[test]
#[should_panic]
fn sub_shape_fail_2() {
    // Matrices with no elements must have the same shape too.
    let _ = &IntegerMatrix::from_str("[]_{2,0}").unwrap()
        - &IntegerMatrix::from_str("[]_{0,2}").unwrap();
}

#[test]
fn sub_properties() {
    integer_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        let diff = &a - &b;
        assert_eq!(a.clone() - b.clone(), diff);
        assert_eq!(a.clone() - &b, diff);
        assert_eq!(&a - b.clone(), diff);
        let mut x = a.clone();
        x -= b.clone();
        assert_eq!(x, diff);
        let mut x = a.clone();
        x -= &b;
        assert_eq!(x, diff);

        // The difference is taken element by element.
        assert!(diff.is_valid());
        assert_eq!((diff.height(), diff.width()), (a.height(), a.width()));
        for ((x, y), z) in a.elements.iter().zip(&b.elements).zip(&diff.elements) {
            assert_eq!(x - y, *z);
        }
        // Subtracting is adding the negation, swapping the operands negates the difference, and
        // adding back the subtrahend recovers the minuend.
        assert_eq!(&a + -&b, diff);
        assert_eq!(&b - &a, -&diff);
        assert_eq!(&diff + &b, a);
        // Subtraction commutes with transposition.
        assert_eq!(a.transpose() - b.transpose(), diff.transpose());
    });

    integer_matrix_triple_gen_var_1().test_properties(|(c, a, b)| {
        assert_eq!((&c - &a) - &b, &c - (&a + &b));
        assert_eq!(&c - (&a - &b), (&c - &a) + &b);
    });

    integer_matrix_gen().test_properties(|a| {
        let zero = IntegerMatrix::zero(a.height(), a.width());
        assert_eq!(&a - &zero, a);
        assert_eq!(&zero - &a, -&a);
        assert!((&a - &a).is_zero());
    });

    natural_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        // Subtracting a `NaturalMatrix` from its sum with another gives the other.
        assert_eq!(
            IntegerMatrix::from(&a + &b) - IntegerMatrix::from(b),
            IntegerMatrix::from(a)
        );
    });
}
