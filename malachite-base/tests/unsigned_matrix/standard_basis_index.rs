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
use malachite_base::vector::Vector;

#[test]
fn test_standard_basis_index() {
    let test = |s, out| {
        assert_eq!(
            UnsignedMatrix::<u64>::from_str(s)
                .unwrap()
                .standard_basis_index(),
            out
        );
    };
    test("[]_{0,0}", None);
    test("[]_{3,0}", None);
    test("[0]", None);
    test("[1]", Some((0, 0)));
    test("[0, 0, 1; 0, 0, 0]", Some((0, 2)));
    test("[0, 0, 0; 1, 0, 0]", Some((1, 0)));
    test("[0; 0; 1]", Some((2, 0)));
    test("[1, 0; 0, 1]", None);
    test("[0, 2; 0, 0]", None);
    test("[0, 1; 1, 0]", None);
    test("[0, 0; 0, 0]", None);
}

#[test]
fn standard_basis_index_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let index = m.standard_basis_index();
        if let Some((i, j)) = index {
            assert_eq!(
                m,
                UnsignedMatrix::<u64>::standard_basis_matrix(m.height(), m.width(), i, j)
            );
            // The row holding the 1 is a standard basis vector, with its 1 in the same column.
            assert_eq!(m.row(i).standard_basis_index(), Some(j));
            assert!(!m.is_zero());
        }
        assert_eq!(
            m.transpose().standard_basis_index(),
            index.map(|(i, j)| (j, i))
        );
    });
}
