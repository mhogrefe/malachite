// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_is_identity() {
    let test = |s, out| {
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(s).unwrap().is_identity(),
            out
        );
    };
    test("[]_{0,0}", true);
    test("[1]", true);
    test("[1, 0; 0, 1]", true);
    test("[1, 0, 0; 0, 1, 0; 0, 0, 1]", true);
    test("[]_{3,0}", false);
    test("[0]", false);
    test("[2]", false);
    test("[1, 0]", false);
    test("[1, 1; 0, 1]", false);
    test("[1, 0; 0, 2]", false);
    test("[0, 1; 1, 0]", false);
}

#[test]
fn is_identity_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let identity = m.is_identity();
        // It is equality with the identity matrix of the same size.
        assert_eq!(
            identity,
            m.is_square() && m == UnsignedMatrix::<u64>::identity(m.height())
        );
        assert_eq!(m.transpose().is_identity(), identity);
        if identity {
            assert_eq!(m.is_zero(), m.height() == 0);
        }
    });
}
