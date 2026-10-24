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
    Determinant, SmithNormalForm, SmithNormalFormAndTransforms,
};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_gen, integer_matrix_gen_var_3, integer_matrix_pair_gen_var_4,
};

// Checks that `u` is unimodular.
fn check_unimodular(u: &IntegerMatrix, size: u64) {
    assert!(u.is_valid());
    assert_eq!(u.height(), size);
    assert_eq!(u.width(), size);
    assert_eq!(u.determinant().unsigned_abs_ref(), &1u32);
}

// Checks the Smith normal form S of `a` and the transforms U and V: both ways of computing them
// agree, S is the Smith normal form, U and V are unimodular, and UAV = S. Returns them.
fn verify_transforms(a: &IntegerMatrix) -> String {
    let (s, u, v) = a.smith_normal_form_and_transforms();
    assert_eq!(
        a.clone().smith_normal_form_and_transforms(),
        (s.clone(), u.clone(), v.clone())
    );
    assert_eq!(a.smith_normal_form(), s);
    check_unimodular(&u, a.height());
    check_unimodular(&v, a.width());
    assert_eq!(&u * a * &v, s);
    format!("({s}, {u}, {v})")
}

#[test]
fn test_smith_normal_form_and_transforms() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(verify_transforms(&a), out);
    };
    test("[]_{0,0}", "([]_{0,0}, []_{0,0}, []_{0,0})");
    test(
        "[]_{0,3}",
        "([]_{0,3}, []_{0,0}, [1, 0, 0; 0, 1, 0; 0, 0, 1])",
    );
    test("[]_{2,0}", "([]_{2,0}, [1, 0; 0, 1], []_{0,0})");
    test("[0, 0; 0, 0]", "([0, 0; 0, 0], [1, 0; 0, 1], [1, 0; 0, 1])");
    test("[3]", "([3], [1], [1])");
    test("[-3]", "([3], [-1], [1])");
    test(
        "[1, 2; 3, 4]",
        "([1, 0; 0, 2], [-2, 1; 3, -1], [1, 0; 0, 1])",
    );
    test(
        "[2, 4; 1, 2]",
        "([1, 0; 0, 0], [0, 1; 1, -2], [1, 2; 0, -1])",
    );
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        "([1, 0, 0; 0, 3, 0; 0, 0, 0], [0, 2, -1; 0, 7, -4; 1, -2, 1], [0, 0, 1; 2, -3, -2; -1, 2\
        , 1])",
    );
    test(
        "[2, 0; 0, 3]",
        "([1, 0; 0, 6], [1, 1; 3, 2], [-1, 3; 1, -2])",
    );
    test(
        "[6, 0, 0; 0, 4, 0; 0, 0, 0]",
        "([2, 0, 0; 0, 12, 0; 0, 0, 0], [1, -1, 0; 2, -3, 0; 0, 0, 1], [1, -2, 0; 1, -3, 0; 0, 0, \
        1])",
    );
    test(
        "[2, 4, 4; -6, 6, 12; 10, -4, -16]",
        "([2, 0, 0; 0, 6, 0; 0, 0, 12], [1, 0, 0; -1, 3, 2; 3, -4, -3], [1, -2, -2; 0, 1, 0; 0, 0\
        , 1])",
    );
    test(
        "[1, 2, 3, 4]",
        "([1, 0, 0, 0], [1], [1, -2, -3, -4; 0, 1, 0, 0; 0, 0, 1, 0; 0, 0, 0, 1])",
    );
    test(
        "[2, 1; 4, 2; 6, 3]",
        "([1, 0; 0, 0; 0, 0], [0, 2, -1; 1, 1, -1; 0, 3, -2], [0, 1; 1, -2])",
    );
    test(
        "[6, 4; 4, 6; 9, 3]",
        "([1, 0; 0, 2; 0, 0], [607, -260, -289; -34, 15, 16; -21, 9, 10], [1, -1; 0, 1])",
    );
}

#[test]
fn smith_normal_form_and_transforms_properties() {
    integer_matrix_gen().test_properties(|a| {
        verify_transforms(&a);
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        verify_transforms(&(a * b));
    });

    integer_matrix_gen_var_3().test_properties(|a| {
        verify_transforms(&a);
    });
}
