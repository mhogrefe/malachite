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
fn test_is_empty() {
    let test = |s, out| {
        assert_eq!(NaturalMatrix::from_str(s).unwrap().is_empty(), out);
    };
    test("[]_{0,0}", true);
    test("[]_{0,3}", true);
    test("[]_{3,0}", true);
    test("[0]", false);
    test("[1]", false);
    test("[0, 0; 0, 0]", false);
    test("[1, 2, 3; 4, 5, 6]", false);
}

#[test]
fn is_empty_properties() {
    natural_matrix_gen().test_properties(|m| {
        let empty = m.is_empty();
        assert_eq!(empty, m.elements.is_empty());
        assert_eq!(empty, m.height() == 0 || m.width() == 0);
        assert_eq!(m.transpose().is_empty(), empty);
        if empty {
            assert!(m.is_zero());
        }
    });
}
