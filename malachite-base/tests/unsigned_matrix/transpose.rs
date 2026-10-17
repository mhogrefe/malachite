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

#[test]
fn test_transpose() {
    let test = |s, out| {
        let m = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let t = m.transpose();
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);

        let mut m_alt = m.clone();
        m_alt.transpose_assign();
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, t);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[]_{0,3}", "[]_{3,0}");
    test("[]_{3,0}", "[]_{0,3}");
    test("[7]", "[7]");
    test("[1, 2, 3]", "[1; 2; 3]");
    test("[1; 2; 3]", "[1, 2, 3]");
    // - square
    test("[1, 2; 3, 4]", "[1, 3; 2, 4]");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 9]", "[1, 4, 7; 2, 5, 8; 3, 6, 9]");
    // - not square
    test("[1, 2, 3; 4, 5, 6]", "[1, 4; 2, 5; 3, 6]");
    test("[1, 2; 3, 4; 5, 6]", "[1, 3, 5; 2, 4, 6]");
}

#[test]
fn transpose_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let t = m.transpose();
        assert!(t.is_valid());
        assert_eq!((t.height(), t.width()), (m.width(), m.height()));
        for i in 0..m.height() {
            for j in 0..m.width() {
                assert_eq!(
                    t.elements[usize::exact_from(j * m.height() + i)],
                    m.elements[usize::exact_from(i * m.width() + j)]
                );
            }
        }
        assert_eq!(t.transpose(), m);

        let mut m_alt = m.clone();
        m_alt.transpose_assign();
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, t);
        m_alt.transpose_assign();
        assert_eq!(m_alt, m);

        let mut sorted_m = m.elements.clone();
        sorted_m.sort_unstable();
        let mut sorted_t = t.elements;
        sorted_t.sort_unstable();
        assert_eq!(sorted_m, sorted_t);
    });
}
