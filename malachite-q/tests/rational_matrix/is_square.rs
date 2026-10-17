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
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::{rational_matrix_gen, rational_matrix_gen_var_1};

#[test]
fn test_is_square() {
    let test = |s, out| {
        assert_eq!(RationalMatrix::from_str(s).unwrap().is_square(), out);
    };
    test("[]_{0,0}", true);
    test("[]_{0,3}", false);
    test("[]_{3,0}", false);
    test("[5]", true);
    test("[1, 2; 3, 4]", true);
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", true);
    test("[1, 2, 3; 4, 5, 6]", false);
    test("[1; 2; 3]", false);
}

#[test]
fn is_square_properties() {
    rational_matrix_gen().test_properties(|m| {
        let square = m.is_square();
        assert_eq!(square, m.height() == m.width());
        assert_eq!(m.transpose().is_square(), square);
        // A square matrix's element count is a perfect square.
        if square {
            assert_eq!(u64::exact_from(m.elements.len()), m.height() * m.height());
        }
    });

    rational_matrix_gen_var_1().test_properties(|m| {
        assert!(m.is_square());
        assert!(m.transpose().is_square());
    });
}
