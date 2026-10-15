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

fn columns_of(m: &RationalMatrix) -> Vec<RationalVector> {
    (0..m.width())
        .map(|j| {
            RationalVector::from_owned_elements(
                (0..m.height())
                    .map(|i| m.elements[i * m.width() + j].clone())
                    .collect(),
            )
        })
        .collect()
}

#[test]
fn test_from_columns() {
    let test = |xss: &[&str], out| {
        let vs = vectors(xss);
        let m = RationalMatrix::from_columns(&vs);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        let m_alt = RationalMatrix::from_owned_columns(vs);
        assert!(m_alt.is_valid());
        assert_eq!(m_alt, m);
    };
    test(&[], "[]_{0,0}");
    test(&["()", "()", "()"], "[]_{0,3}");
    test(&["(5)"], "[5]");
    test(&["(1, 2, 3)"], "[1; 2; 3]");
    test(&["(1)", "(2)", "(3)"], "[1, 2, 3]");
    test(&["(1, 2)", "(3, 4)", "(5, 6)"], "[1, 3, 5; 2, 4, 6]");
    test(&["(1, -2, 3)", "(-4, 5, -6)"], "[1, -4; -2, 5; 3, -6]");
    test(&["(1/2, 3)", "(-1/3, 4)"], "[1/2, -1/3; 3, 4]");
}

#[test]
#[should_panic]
fn from_columns_fail() {
    RationalMatrix::from_columns(&vectors(&["(1, 2)", "(3)"]));
}

#[test]
#[should_panic]
fn from_owned_columns_fail() {
    RationalMatrix::from_owned_columns(vectors(&["()", "(3)"]));
}

#[test]
fn from_columns_properties() {
    rational_matrix_gen().test_properties(|m| {
        let vs = columns_of(&m);
        let m_alt = RationalMatrix::from_columns(&vs);
        assert!(m_alt.is_valid());
        // With no columns, the other dimension is lost.
        if vs.is_empty() {
            assert_eq!((m_alt.height(), m_alt.width()), (0, 0));
        } else {
            assert_eq!(m_alt, m);
        }
        assert_eq!(RationalMatrix::from_owned_columns(vs.clone()), m_alt);
        assert_eq!(RationalMatrix::from_rows(&vs), m_alt.transpose());
    });
}
