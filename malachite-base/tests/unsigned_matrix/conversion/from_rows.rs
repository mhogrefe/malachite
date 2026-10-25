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
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

fn vectors(ss: &[&str]) -> Vec<UnsignedVector<u64>> {
    ss.iter()
        .map(|s| UnsignedVector::<u64>::from_str(s).unwrap())
        .collect()
}

fn rows_of(m: &UnsignedMatrix<u64>) -> Vec<UnsignedVector<u64>> {
    (0..m.height())
        .map(|i| {
            UnsignedVector::from_elements(
                &m.elements_ref()
                    [usize::exact_from(i * m.width())..usize::exact_from((i + 1) * m.width())],
            )
        })
        .collect()
}

#[test]
fn test_from_rows() {
    let test = |xss: &[&str], out| {
        let vs = vectors(xss);
        let m = UnsignedMatrix::from_rows(&vs);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        let m_alt = UnsignedMatrix::from_owned_rows(vs);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
    };
    test(&[], "[]_{0,0}");
    test(&["()", "()", "()"], "[]_{3,0}");
    test(&["(5)"], "[5]");
    test(&["(1, 2, 3)"], "[1, 2, 3]");
    test(&["(1)", "(2)", "(3)"], "[1; 2; 3]");
    test(&["(1, 2, 3)", "(4, 5, 6)"], "[1, 2, 3; 4, 5, 6]");
    test(&["(1, 20)", "(3, 40)", "(5, 60)"], "[1, 20; 3, 40; 5, 60]");
}

#[test]
#[should_panic]
fn from_rows_fail() {
    UnsignedMatrix::from_rows(&vectors(&["(1, 2)", "(3)"]));
}

#[test]
#[should_panic]
fn from_owned_rows_fail() {
    UnsignedMatrix::from_owned_rows(vectors(&["()", "(3)"]));
}

#[test]
fn from_rows_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let vs = rows_of(&m);
        let m_alt = UnsignedMatrix::from_rows(&vs);
        assert!(m_alt.is_valid());
        // With no rows, the other dimension is lost.
        if vs.is_empty() {
            assert_eq!((m_alt.height(), m_alt.width()), (0, 0));
        } else {
            assert_eq!(m_alt, m);
        }
        assert_eq!(UnsignedMatrix::from_owned_rows(vs.clone()), m_alt);
        assert_eq!(UnsignedMatrix::from_columns(&vs), m_alt.transpose());
    });
}
