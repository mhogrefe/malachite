// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{
    rational_matrix_gen, rational_matrix_unsigned_pair_gen_var_1,
};

#[test]
fn test_is_zero_row() {
    let test = |s, i: u64, out| {
        assert_eq!(RationalMatrix::from_str(s).unwrap().is_zero_row(i), out);
    };
    test("[]_{3,0}", 0, true);
    test("[]_{3,0}", 2, true);
    test("[0]", 0, true);
    test("[-1/2]", 0, false);
    test("[1, 2, 3; 0, 0, 0]", 0, false);
    test("[1, 2, 3; 0, 0, 0]", 1, true);
    test("[0, 0; -1/2, 0; 0, 0]", 0, true);
    test("[0, 0; -1/2, 0; 0, 0]", 1, false);
    test("[0, 0; -1/2, 0; 0, 0]", 2, true);
    test("[0, 0; 0, -1/2; 0, 0]", 1, false);
}

#[test]
#[should_panic]
fn is_zero_row_fail_1() {
    let _ = RationalMatrix::from_str("[]_{0,0}").unwrap().is_zero_row(0);
}

#[test]
#[should_panic]
fn is_zero_row_fail_2() {
    let _ = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
        .unwrap()
        .is_zero_row(2);
}

#[test]
#[should_panic]
fn is_zero_row_fail_3() {
    let _ = RationalMatrix::from_str("[]_{0,4}").unwrap().is_zero_row(0);
}

#[test]
fn is_zero_row_properties() {
    rational_matrix_unsigned_pair_gen_var_1().test_properties(|(mut m, i)| {
        let zero = m.is_zero_row(i);
        assert_eq!(zero, m.row_iter(i).all(|x| *x == 0u32));
        assert_eq!(zero, m.row(i).is_zero());
        assert_eq!(m.transpose().column(i).is_zero(), zero);
        if m.is_zero() {
            assert!(zero);
        }
        for x in m.row_iter_mut(i) {
            *x = Rational::ZERO;
        }
        assert!(m.is_zero_row(i));
    });

    rational_matrix_gen().test_properties(|m| {
        assert_eq!((0..m.height()).all(|i| m.is_zero_row(i)), m.is_zero());
    });
}
