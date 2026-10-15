// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen;

fn ints(xs: &[u32]) -> Vec<Natural> {
    xs.iter().copied().map(Natural::from).collect()
}

#[test]
fn test_from_elements_column_major() {
    let test = |height, width, xs: &[u32], out| {
        let xs = ints(xs);
        let m = NaturalMatrix::from_elements_column_major(height, width, &xs);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        let m_alt = NaturalMatrix::from_owned_elements_column_major(height, width, xs);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
    };
    test(0, 0, &[], "[]_{0,0}");
    test(0, 4, &[], "[]_{0,4}");
    test(4, 0, &[], "[]_{4,0}");
    test(1, 1, &[5], "[5]");
    test(1, 3, &[1, 2, 3], "[1, 2, 3]");
    test(3, 1, &[1, 2, 3], "[1; 2; 3]");
    test(2, 2, &[1, 2, 3, 4], "[1, 3; 2, 4]");
    test(2, 3, &[1, 2, 3, 4, 5, 6], "[1, 3, 5; 2, 4, 6]");
    test(3, 2, &[1, 2, 3, 4, 5, 6], "[1, 4; 2, 5; 3, 6]");
}

#[test]
#[should_panic]
fn from_elements_column_major_fail_1() {
    NaturalMatrix::from_elements_column_major(2, 2, &ints(&[1, 2, 3]));
}

#[test]
#[should_panic]
fn from_elements_column_major_fail_2() {
    NaturalMatrix::from_elements_column_major(usize::MAX, 2, &[]);
}

#[test]
#[should_panic]
fn from_owned_elements_column_major_fail() {
    NaturalMatrix::from_owned_elements_column_major(2, 0, ints(&[1]));
}

#[test]
fn from_elements_column_major_properties() {
    natural_matrix_gen().test_properties(|m| {
        // The column-major elements of a matrix are the row-major elements of its transpose.
        let t = m.transpose();
        let m_alt = NaturalMatrix::from_elements_column_major(m.height(), m.width(), &t.elements);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
        let m_alt =
            NaturalMatrix::from_owned_elements_column_major(m.height(), m.width(), t.elements);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);

        assert_eq!(
            NaturalMatrix::from_elements_column_major(m.width(), m.height(), &m.elements),
            m.transpose()
        );
    });
}
