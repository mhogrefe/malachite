// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// The matrix with the elements in row `i` and column `j` kept where `i >= j`, and the others
// replaced with zero.
fn lower_triangular_part(m: &UnsignedMatrix<u64>) -> UnsignedMatrix<u64> {
    let mut n = m.clone();
    let width = usize::exact_from(m.width());
    for (k, x) in n.elements.iter_mut().enumerate() {
        let (i, j) = (k / width, k % width);
        if !(i >= j) {
            *x = 0;
        }
    }
    n
}

#[test]
fn test_is_lower_triangular() {
    let test = |s, out| {
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(s)
                .unwrap()
                .is_lower_triangular(),
            out
        );
    };
    test("[]_{0,0}", true);
    test("[]_{3,0}", true);
    test("[]_{0,2}", true);
    test("[0]", true);
    test("[5]", true);
    test("[0, 0; 0, 0]", true);
    test("[1, 2; 2, 1]", false);
    test("[1, 2; 3, 1]", false);
    test("[0, 2; 2, 0]", false);
    test("[1, 0; 0, 2]", true);
    test("[1, 2; 0, 3]", false);
    test("[1, 0; 2, 3]", true);
    test("[1, 2, 3; 0, 4, 5]", false);
    test("[1, 0, 0; 2, 3, 0]", true);
    test("[1, 0, 0; 0, 2, 0]", true);
    test("[1; 0; 0]", true);
    test("[0; 1]", true);
    test("[1, 2, 3; 2, 4, 5; 3, 5, 6]", false);
}

#[test]
fn is_lower_triangular_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let lower = m.is_lower_triangular();
        assert_eq!(m.transpose().is_upper_triangular(), lower);
        let l = lower_triangular_part(&m);
        assert!(l.is_lower_triangular());
        assert_eq!(l == m, lower);
    });
}
