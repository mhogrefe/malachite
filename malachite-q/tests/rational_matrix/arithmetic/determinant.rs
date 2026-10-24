// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Determinant, Inverse, Reciprocal};
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_2, rational_matrix_pair_gen_var_5,
};
use malachite_q::test_util::rational_matrix::arithmetic::solve::rational_matrix_low_rank;
use std::panic::catch_unwind;

// Checks the determinant of `a`: both ways of passing it agree, it is that of the transpose, it is
// zero exactly when there is no inverse, and otherwise it is the reciprocal of the inverse's
// determinant. Returns it.
fn verify_determinant(a: &RationalMatrix) -> Rational {
    let det = a.clone().determinant();
    assert!(det.is_valid());
    assert_eq!(a.determinant(), det);
    assert_eq!(a.transpose().determinant(), det);
    let inverse = a.inverse();
    assert_eq!(inverse.is_none(), det == 0u32);
    if let Some(b) = inverse {
        assert_eq!(b.determinant(), (&det).reciprocal());
    }
    det
}

#[test]
fn test_determinant() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        assert_eq!(verify_determinant(&a).to_string(), out);
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[-2/3]", "-2/3");
    test("[1, 2; 3, 4]", "-2");
    test("[1/2, 1/3; 1/4, 1/5]", "1/60");
    test("[2, 4; 1, 2]", "0");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "-3");
    test("[1, 1/2, 1/3; 1/2, 1/3, 1/4; 1/3, 1/4, 1/5]", "1/2160");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "0");
    test(
        "[1, 1/2, 1/3, 1/4; 1/2, 1/3, 1/4, 1/5; 1/3, 1/4, 1/5, 1/6; 1/4, 1/5, 1/6, 1/7]",
        "1/6048000",
    );
}

#[test]
fn determinant_fail() {
    let a = RationalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().determinant());
    assert_panic!((&a).determinant());
}

#[test]
fn determinant_properties() {
    rational_matrix_gen_var_2().test_properties(|a| {
        verify_determinant(&a);
    });

    rational_matrix_pair_gen_var_5().test_properties(|(a, b)| {
        verify_determinant(&rational_matrix_low_rank(&a, &b));
    });
}
