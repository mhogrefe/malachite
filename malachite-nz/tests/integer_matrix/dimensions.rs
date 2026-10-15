// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_height_and_width() {
    let test = |s, height, width| {
        let m = IntegerMatrix::from_str(s).unwrap();
        assert_eq!(m.height(), height);
        assert_eq!(m.width(), width);
    };
    test("[]_{0,0}", 0, 0);
    test("[]_{0,4}", 0, 4);
    test("[]_{4,0}", 4, 0);
    test("[5]", 1, 1);
    test("[1, 2, 3; 4, 5, 6]", 2, 3);
    test("[1, 2; 3, 4; 5, 6]", 3, 2);
}

#[test]
fn dimensions_properties() {
    integer_matrix_gen().test_properties(|m| {
        assert_eq!(m.height() * m.width(), m.elements.len());
        let t = m.transpose();
        assert_eq!(t.height(), m.width());
        assert_eq!(t.width(), m.height());
    });
}
