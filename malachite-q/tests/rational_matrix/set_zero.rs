// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;

#[test]
fn test_set_zero() {
    let test = |s, out| {
        let mut m = RationalMatrix::from_str(s).unwrap();
        m.set_zero();
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{3,0}", "[]_{3,0}");
    test("[5]", "[0]");
    test("[1, 2; 3, 4]", "[0, 0; 0, 0]");
    test("[1, 2, 3; 4, 5, 6]", "[0, 0, 0; 0, 0, 0]");
}

#[test]
fn set_zero_properties() {
    rational_matrix_gen().test_properties(|m| {
        let mut n = m.clone();
        n.set_zero();
        assert!(n.is_valid());
        assert!(n.is_zero());
        assert_eq!(n, RationalMatrix::zero(m.height(), m.width()));
    });
}
