// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Permanent;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen_var_4;
use malachite_q::test_util::rational_matrix::arithmetic::permanent::rational_matrix_permanent_naive;
use std::panic::catch_unwind;

fn verify(a: &RationalMatrix) -> String {
    let p = a.clone().permanent();
    assert_eq!(a.permanent(), p);
    assert_eq!(a.transpose().permanent(), p);
    if a.height() <= 8 {
        assert_eq!(rational_matrix_permanent_naive(a), p);
    }
    p.to_string()
}

#[test]
fn test_permanent() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a), out);
    };
    // - dimension 0
    test("[]_{0,0}", "1");
    // - dimension 1
    test("[-7/2]", "-7/2");
    // - dimension 2
    test("[1/2, 2/3; 3/4, 4/5]", "9/10");
    // - dimension at least 3
    test("[1/2, -2, 3; 4, 5/3, -6; -7, 8, 10]", "-356/3");
    test(
        "[1/2, 1/3, 1/4, 1/5; 1/3, 1/4, 1/5, 1/6; 1/4, 1/5, 1/6, 1/7; 1/5, 1/6, 1/7, 1/8]",
        "2296639/47040000",
    );
}

#[test]
fn permanent_fail() {
    let a = RationalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().permanent());
    assert_panic!((&a).permanent());
}

#[test]
fn permanent_properties() {
    rational_matrix_gen_var_4().test_properties(|a| {
        verify(&a);
    });
}
