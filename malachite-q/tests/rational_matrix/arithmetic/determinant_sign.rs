// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Determinant, DeterminantSign, Sign};
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_gen_var_2, rational_matrix_pair_gen_var_5,
};
use malachite_q::test_util::rational_matrix::arithmetic::solve::rational_matrix_low_rank;
use std::panic::catch_unwind;

fn verify(a: &RationalMatrix) -> Ordering {
    let sign = a.clone().determinant_sign();
    assert_eq!(a.determinant_sign(), sign);
    assert_eq!(a.determinant().sign(), sign);
    assert_eq!(a.transpose().determinant_sign(), sign);
    sign
}

#[test]
fn test_determinant_sign() {
    let test = |s, out| {
        let a = RationalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a), out);
    };
    // - dimension 0
    test("[]_{0,0}", Greater);
    // - dimension 1
    test("[0]", Equal);
    test("[-7/2]", Less);
    // - dimension 2
    test("[1/2, 2/3; 3/4, 4/5]", Less);
    test("[1/2, 1/3; 3/2, 1]", Equal);
    test("[-1/2, 1/3; 1/3, -1/4]", Greater);
    // - dimension at least 3
    test("[1/2, -2, 3; 4, 5/3, -6; -7, 8, 10]", Greater);
    test("[1/2, 1, 3/2; 4, 5, 6; 7, 8, 9]", Equal);
    test("[1/2, 1, 3/2; 4, 5, 6; 7, 8, 10]", Less);
}

#[test]
fn determinant_sign_fail() {
    let a = RationalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().determinant_sign());
    assert_panic!((&a).determinant_sign());
}

#[test]
fn determinant_sign_properties() {
    rational_matrix_gen_var_2().test_properties(|a| {
        verify(&a);
    });

    rational_matrix_pair_gen_var_5().test_properties(|(a, b)| {
        verify(&rational_matrix_low_rank(&a, &b));
    });
}
