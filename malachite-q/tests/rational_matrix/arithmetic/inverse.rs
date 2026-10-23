// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Inverse;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::arithmetic::solve::solve_nonsingular;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_2, rational_matrix_pair_gen_var_5,
};
use std::panic::catch_unwind;

// Checks the inverse B of A: both ways of passing A agree, AB = BA = I, and, for small A, B is the
// solution of AX = I. Returns it.
fn verify_inverse(a: &RationalMatrix) -> String {
    let result = a.clone().inverse();
    assert_eq!(a.inverse(), result);
    let n = a.height();
    let identity = RationalMatrix::identity(n);
    if let Some(b) = &result {
        assert!(b.is_valid());
        assert_eq!(a * b, identity);
        assert_eq!(b * a, identity);
    }
    if n <= 4 {
        assert_eq!(solve_nonsingular(a, &identity), result);
    }
    result.map_or_else(|| "None".to_string(), |b| format!("Some({b})"))
}

#[test]
fn test_inverse() {
    let test = |a, out| {
        let a = RationalMatrix::from_str(a).unwrap();
        assert_eq!(verify_inverse(&a), out);
    };
    test("[]_{0,0}", "Some([]_{0,0})");
    test("[0]", "None");
    test("[5]", "Some([1/5])");
    test("[-2/3]", "Some([-3/2])");
    test("[1, 2; 3, 4]", "Some([-2, 1; 3/2, -1/2])");
    test("[1/2, 1/3; 1/4, 1/5]", "Some([12, -20; -15, 30])");
    test("[2, 4; 1, 2]", "None");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "Some([-2/3, -4/3, 1; -2/3, 11/3, -2; 1, -2, 1])",
    );
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "None");
    test(
        "[1, 1/2, 1/3; 1/2, 1/3, 1/4; 1/3, 1/4, 1/5]",
        "Some([9, -36, 30; -36, 192, -180; 30, -180, 180])",
    );
    test(
        "[1/2, 0, 0, 0; 0, 1/3, 0, 0; 0, 0, 1/4, 0; 0, 0, 0, 1/5]",
        "Some([2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 4, 0; 0, 0, 0, 5])",
    );
}

#[test]
fn inverse_fail() {
    let a = RationalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().inverse());
    assert_panic!((&a).inverse());
}

#[test]
fn inverse_properties() {
    rational_matrix_gen_var_2().test_properties(|a| {
        verify_inverse(&a);
    });

    rational_matrix_pair_gen_var_5().test_properties(|(a, b)| {
        let n = a.height();
        let k = b.width();
        if k < n {
            verify_inverse(&(&b * a.window(0..k, 0..n)));
        }
    });
}
