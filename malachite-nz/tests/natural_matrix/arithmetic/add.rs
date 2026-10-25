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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_gen, natural_matrix_pair_gen_var_1, natural_matrix_triple_gen_var_1,
};

#[test]
fn test_add() {
    let test = |s, t, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = NaturalMatrix::from_str(t).unwrap();
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
    test("[1, 2; 3, 0]", "[10, 0; 5, 1]", "[11, 2; 8, 1]");
    test("[1, 2, 3]", "[10, 0, 5]", "[11, 2, 8]");
    test(
        "[1000000000000000000000, 1]",
        "[1000000000000000000000, 2]",
        "[2000000000000000000000, 3]",
    );
}

fn pair() -> (NaturalMatrix, NaturalMatrix) {
    (
        NaturalMatrix::from_str("[1, 2]").unwrap(),
        NaturalMatrix::from_str("[1]").unwrap(),
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
        &NaturalMatrix::from_str("[]_{0,0}").unwrap() + &NaturalMatrix::from_str("[0]").unwrap();
}

#[test]
#[should_panic]
fn add_shape_fail_1() {
    // The same number of elements is not enough: the shapes must match.
    let _ =
        &NaturalMatrix::from_str("[1, 2]").unwrap() + &NaturalMatrix::from_str("[1; 2]").unwrap();
}

#[test]
#[should_panic]
fn add_shape_fail_2() {
    // Matrices with no elements must have the same shape too.
    let _ = &NaturalMatrix::from_str("[]_{2,0}").unwrap()
        + &NaturalMatrix::from_str("[]_{0,2}").unwrap();
}

#[test]
fn add_properties() {
    natural_matrix_pair_gen_var_1().test_properties(|(a, b)| {
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
        for ((x, y), z) in a
            .elements_ref()
            .iter()
            .zip(b.elements_ref())
            .zip(sum.elements_ref())
        {
            assert_eq!(x + y, *z);
        }
        assert_eq!(&b + &a, sum);
        // Addition commutes with transposition.
        assert_eq!(a.transpose() + b.transpose(), sum.transpose());
    });

    natural_matrix_triple_gen_var_1().test_properties(|(c, a, b)| {
        // Addition is associative.
        assert_eq!((&c + &a) + &b, &c + (&a + &b));
    });

    natural_matrix_gen().test_properties(|a| {
        // The zero matrix of the same shape is the identity.
        let zero = NaturalMatrix::zero(a.height(), a.width());
        assert_eq!(&a + &zero, a);
        assert_eq!(&zero + &a, a);
    });

    natural_matrix_pair_gen_var_1().test_properties(|(a, b)| {
        // The sum agrees with the sum of the matrices as `IntegerMatrix`s.
        assert_eq!(
            IntegerMatrix::from(&a + &b),
            IntegerMatrix::from(a) + IntegerMatrix::from(b)
        );
    });
}
