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
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_matrix_gen;

fn vectors(ss: &[&str]) -> Vec<IntegerVector> {
    ss.iter()
        .map(|s| IntegerVector::from_str(s).unwrap())
        .collect()
}

fn row_slices(m: &IntegerMatrix) -> Vec<&[Integer]> {
    let width = usize::exact_from(m.width());
    if width == 0 {
        vec![&[]; usize::exact_from(m.height())]
    } else {
        m.elements_ref().chunks_exact(width).collect()
    }
}

#[test]
fn test_from_row_slices() {
    let test = |xss: &[&str], out| {
        let vs = vectors(xss);
        let rows: Vec<&[Integer]> = vs.iter().map(|v| v.elements.as_slice()).collect();
        let m = IntegerMatrix::from_row_slices(&rows);
        assert!(m.is_valid());
        assert_eq!(m.to_string(), out);
        assert_eq!(IntegerMatrix::from_rows(&vs), m);
    };
    test(&[], "[]_{0,0}");
    test(&["()", "()", "()"], "[]_{3,0}");
    test(&["(-5)"], "[-5]");
    test(&["(1)", "(-2)", "(3)"], "[1; -2; 3]");
    test(&["(1, -2, 3)", "(-4, 5, -6)"], "[1, -2, 3; -4, 5, -6]");
    test(
        &["(-1267650600228229401496703205376, 2)", "(3, 4)"],
        "[-1267650600228229401496703205376, 2; 3, 4]",
    );
}

#[test]
#[should_panic]
fn from_row_slices_fail() {
    let vs = vectors(&["(1, 2)", "(3)"]);
    let rows: Vec<&[Integer]> = vs.iter().map(|v| v.elements.as_slice()).collect();
    IntegerMatrix::from_row_slices(&rows);
}

#[test]
fn from_row_slices_properties() {
    integer_matrix_gen().test_properties(|m| {
        let rows = row_slices(&m);
        let m_alt = IntegerMatrix::from_row_slices(&rows);
        assert!(m_alt.is_valid());
        // With no rows, the other dimension is lost.
        if rows.is_empty() {
            assert_eq!((m_alt.height(), m_alt.width()), (0, 0));
        } else {
            assert_eq!(m_alt, m);
        }
        let vs: Vec<IntegerVector> = rows
            .iter()
            .map(|row| IntegerVector::from_elements(row))
            .collect();
        assert_eq!(IntegerMatrix::from_rows(&vs), m_alt);
    });
}
