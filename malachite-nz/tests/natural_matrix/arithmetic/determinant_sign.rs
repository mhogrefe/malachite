// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Determinant, DeterminantSign, Sign};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen_var_1;
use std::panic::catch_unwind;

fn verify(a: &NaturalMatrix) -> Ordering {
    let sign = a.clone().determinant_sign();
    assert_eq!(a.determinant_sign(), sign);
    assert_eq!(a.determinant().sign(), sign);
    assert_eq!(IntegerMatrix::from(a).determinant_sign(), sign);
    sign
}

#[test]
fn test_determinant_sign() {
    let test = |s, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a), out);
    };
    // - dimension other than 2
    test("[]_{0,0}", Greater);
    test("[0]", Equal);
    test("[7]", Greater);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", Less);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", Equal);
    test("[2, 1, 1; 1, 2, 1; 1, 1, 2]", Greater);
    // - dimension 2
    test("[1, 2; 3, 4]", Less);
    test("[4, 3; 2, 1]", Less);
    test("[2, 4; 1, 2]", Equal);
    test("[5, 4; 3, 3]", Greater);
}

#[test]
fn determinant_sign_fail() {
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().determinant_sign());
    assert_panic!((&a).determinant_sign());
}

#[test]
fn determinant_sign_properties() {
    natural_matrix_gen_var_1().test_properties(|a| {
        verify(&a);
    });
}
