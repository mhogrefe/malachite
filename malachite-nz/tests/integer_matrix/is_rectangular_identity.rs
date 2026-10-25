// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::integer_matrix_gen;

#[test]
fn test_is_rectangular_identity() {
    let test = |s, out| {
        assert_eq!(
            IntegerMatrix::from_str(s)
                .unwrap()
                .is_rectangular_identity(),
            out
        );
    };
    test("[]_{0,0}", true);
    test("[]_{0,3}", true);
    test("[]_{3,0}", true);
    test("[1]", true);
    test("[1, 0, 0]", true);
    test("[1; 0; 0]", true);
    test("[1, 0, 0; 0, 1, 0]", true);
    test("[1, 0; 0, 1; 0, 0]", true);
    test("[0]", false);
    test("[2]", false);
    test("[1, 1]", false);
    test("[0, 1, 0]", false);
    test("[1, 0; 0, 1; 0, 1]", false);
    test("[1, 0, 0; 0, 2, 0]", false);
    test("[-1, 0, 0]", false);
}

#[test]
fn is_rectangular_identity_properties() {
    integer_matrix_gen().test_properties(|m| {
        let rectangular_identity = m.is_rectangular_identity();
        // It is equality with the rectangular identity matrix of the same shape.
        assert_eq!(
            rectangular_identity,
            m == IntegerMatrix::rectangular_identity(m.height(), m.width())
        );
        assert_eq!(
            m.transpose().is_rectangular_identity(),
            rectangular_identity
        );
        assert_eq!(m.is_identity(), m.is_square() && rectangular_identity);
        if m.elements_ref().is_empty() {
            assert!(rectangular_identity);
        }
    });
}
