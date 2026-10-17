// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen;

#[test]
fn test_is_skew_symmetric() {
    let test = |s, out| {
        assert_eq!(NaturalMatrix::from_str(s).unwrap().is_skew_symmetric(), out);
    };
    test("[]_{0,0}", true);
    test("[]_{3,0}", false);
    test("[]_{0,2}", false);
    test("[0]", true);
    test("[5]", false);
    test("[0, 0; 0, 0]", true);
    test("[1, 2; 2, 1]", false);
    test("[1, 2; 3, 1]", false);
    test("[0, 2; 2, 0]", false);
    test("[1, 0; 0, 2]", false);
    test("[1, 2; 0, 3]", false);
    test("[1, 0; 2, 3]", false);
    test("[1, 2, 3; 0, 4, 5]", false);
    test("[1, 0, 0; 2, 3, 0]", false);
    test("[1, 0, 0; 0, 2, 0]", false);
    test("[1; 0; 0]", false);
    test("[0; 1]", false);
    test("[1, 2, 3; 2, 4, 5; 3, 5, 6]", false);
}

#[test]
fn is_skew_symmetric_properties() {
    natural_matrix_gen().test_properties(|m| {
        let skew = m.is_skew_symmetric();
        // No element can be negative, so only the square zero matrices are skew-symmetric.
        assert_eq!(skew, m.is_square() && m.is_zero());
        assert_eq!(m.transpose().is_skew_symmetric(), skew);
        if skew {
            assert!(m.is_symmetric());
        }
    });
}
