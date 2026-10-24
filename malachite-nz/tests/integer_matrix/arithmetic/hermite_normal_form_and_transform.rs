// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Determinant, HermiteNormalForm, HermiteNormalFormAndTransform,
};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::hermite_normal_form_and_transform::*;
use malachite_nz::test_util::generators::common::has_full_column_rank;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_gen_var_3, integer_matrix_pair_gen_var_4,
};

// Checks that `u` is a unimodular transform taking `a` to `h`.
fn check_transform(a: &IntegerMatrix, h: &IntegerMatrix, u: &IntegerMatrix) {
    assert!(u.is_valid());
    assert_eq!(u.height(), a.height());
    assert_eq!(u.width(), a.height());
    assert_eq!(u * a, *h);
    assert_eq!(u.determinant().unsigned_abs_ref(), &1u32);
}

// Checks the Hermite normal form H of `a` and the transform U: both ways of computing them agree, H
// is the Hermite normal form, and U is unimodular with UA = H; the naive method, which may give
// another U, gives a valid one too; and, when `a` has full column rank, the minors-based method
// gives the same pair. Returns them.
fn verify_transform(a: &IntegerMatrix) -> String {
    let (h, u) = a.hermite_normal_form_and_transform();
    assert_eq!(
        a.clone().hermite_normal_form_and_transform(),
        (h.clone(), u.clone())
    );
    assert_eq!(a.hermite_normal_form(), h);
    check_transform(a, &h, &u);
    let (h_naive, u_naive) = hnf_transform_naive(a);
    assert_eq!(h_naive, h);
    check_transform(a, &h, &u_naive);
    if has_full_column_rank(a) {
        assert_eq!(hnf_minors_transform(a), (h.clone(), u.clone()));
    } else {
        assert_eq!(u_naive, u);
    }
    format!("({h}, {u})")
}

#[test]
fn test_hermite_normal_form_and_transform() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_transform(&a), out);
    };
    test("[]_{0,0}", "([]_{0,0}, []_{0,0})");
    test("[]_{0,3}", "([]_{0,3}, []_{0,0})");
    test("[]_{2,0}", "([]_{2,0}, [1, 0; 0, 1])");
    test("[0, 0; 0, 0]", "([0, 0; 0, 0], [1, 0; 0, 1])");
    test("[3]", "([3], [1])");
    test("[-3]", "([3], [-1])");
    test("[1, 2; 3, 4]", "([1, 0; 0, 2], [-2, 1; 3, -1])");
    test("[2, 4; 1, 2]", "([1, 2; 0, 0], [0, 1; 1, -2])");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "([1, 2, 3; 0, 3, 6; 0, 0, 0], [0, 2, -1; 0, 7, -4; 1, -2, 1])",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "([1, 2, 0; 0, 3, 0; 0, 0, 1], [-2, 6, -3; -2, 11, -6; 1, -2, 1])",
    );
    test("[0, 2, 1; 0, 4, 3]", "([0, 2, 0; 0, 0, 1], [3, -1; -2, 1])");
    test("[1, 2, 3, 4]", "([1, 2, 3, 4], [1])");
    test("[1; 2; 3]", "([1; 0; 0], [1, 0, 0; -2, 1, 0; -3, 0, 1])");
    test(
        "[2, 1; 4, 2; 6, 3]",
        "([2, 1; 0, 0; 0, 0], [0, 2, -1; 1, 1, -1; 0, 3, -2])",
    );
    test(
        "[6, 4; 4, 6; 9, 3]",
        "([1, 1; 0, 2; 0, 0], [607, -260, -289; -34, 15, 16; -21, 9, 10])",
    );
    test(
        "[1, 2, 3, 4; 2, 4, 6, 8; 1, 0, 1, 0; 3, 2, 4, 4]",
        "([1, 0, 0, 0; 0, 2, 0, 4; 0, 0, 1, 0; 0, 0, 0, 0], [1, -1, -1, 1; 1, -1, -5, 2; 1, 0, 2, \
        -1; 2, -1, 0, 0])",
    );
    // - hnf_minors_transform swaps in a row of U when a diagonal element of H is zero
    test("[0, 1; 1, 0]", "([1, 0; 0, 1], [0, 1; 1, 0])");
}

#[test]
fn hermite_normal_form_and_transform_properties() {
    integer_matrix_gen().test_properties(|a| {
        verify_transform(&a);
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        verify_transform(&(a * b));
    });

    integer_matrix_gen_var_3().test_properties(|a| {
        verify_transform(&a);
    });
}
