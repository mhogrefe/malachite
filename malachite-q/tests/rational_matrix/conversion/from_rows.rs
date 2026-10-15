// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_matrix_gen;

fn vectors(ss: &[&str]) -> Vec<RationalVector> {
    ss.iter()
        .map(|s| RationalVector::from_str(s).unwrap())
        .collect()
}

fn rows_of(m: &RationalMatrix) -> Vec<RationalVector> {
    (0..m.height())
        .map(|i| RationalVector::from_elements(&m.elements[i * m.width()..(i + 1) * m.width()]))
        .collect()
}

#[test]
fn test_from_rows() {
    let test = |xss: &[&str], out| {
        let vs = vectors(xss);
        let m = RationalMatrix::from_rows(&vs);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        let m_alt = RationalMatrix::from_owned_rows(vs);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
    };
    test(&[], "[]_{0,0}");
    test(&["()", "()", "()"], "[]_{3,0}");
    test(&["(5)"], "[5]");
    test(&["(1, 2, 3)"], "[1, 2, 3]");
    test(&["(1)", "(2)", "(3)"], "[1; 2; 3]");
    test(&["(1, 2, 3)", "(4, 5, 6)"], "[1, 2, 3; 4, 5, 6]");
    test(&["(1, -2)", "(3, -4)", "(5, -6)"], "[1, -2; 3, -4; 5, -6]");
    test(&["(1/2, 3)", "(-1/3, 4)"], "[1/2, 3; -1/3, 4]");
}

#[test]
#[should_panic]
fn from_rows_fail() {
    RationalMatrix::from_rows(&vectors(&["(1, 2)", "(3)"]));
}

#[test]
#[should_panic]
fn from_owned_rows_fail() {
    RationalMatrix::from_owned_rows(vectors(&["()", "(3)"]));
}

#[test]
fn from_rows_properties() {
    rational_matrix_gen().test_properties(|m| {
        let vs = rows_of(&m);
        let m_alt = RationalMatrix::from_rows(&vs);
        assert!(m_alt.is_valid());
        // With no rows, the other dimension is lost.
        if vs.is_empty() {
            assert_eq!((m_alt.height(), m_alt.width()), (0, 0));
        } else {
            assert_eq!(m_alt, m);
        }
        assert_eq!(RationalMatrix::from_owned_rows(vs.clone()), m_alt);
        assert_eq!(RationalMatrix::from_columns(&vs), m_alt.transpose());
    });
}
