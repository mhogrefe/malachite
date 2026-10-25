// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_matrix_gen;
use malachite_base::unsigned_matrix::UnsignedMatrix;

fn matrix(height: u64, width: u64, xs: &[u64]) -> UnsignedMatrix<u64> {
    UnsignedMatrix::from_owned_elements(height, width, xs.to_vec())
}

// The rows of a matrix with at least one row and one column, each as the strings `f` makes of its
// elements, joined by `element_separator`.
fn rows(
    m: &UnsignedMatrix<u64>,
    element_separator: &str,
    f: impl Fn(&u64) -> String,
) -> Vec<String> {
    m.elements_ref()
        .chunks(usize::exact_from(m.width()))
        .map(|row| row.iter().map(&f).join(element_separator))
        .collect()
}

#[test]
fn test_to_string() {
    let test = |height, width, xs: &[u64], out| {
        let m = matrix(height, width, xs);
        assert_eq!(m.to_string(), out);
        assert_eq!(m.to_debug_string(), out);
    };
    test(0, 0, &[], "[]_{0,0}");
    test(0, 4, &[], "[]_{0,4}");
    test(4, 0, &[], "[]_{4,0}");
    test(1, 0, &[], "[]_{1,0}");
    test(0, 1, &[], "[]_{0,1}");
    test(1, 1, &[0], "[0]");
    test(1, 1, &[5], "[5]");
    test(1, 3, &[1, 2, 3], "[1, 2, 3]");
    test(3, 1, &[1, 2, 3], "[1; 2; 3]");
    test(2, 2, &[1, 2, 3, 4], "[1, 2; 3, 4]");
    test(2, 3, &[1, 20, 3, 40, 5, 60], "[1, 20, 3; 40, 5, 60]");
    test(1, 2, &[u64::MAX, 10], "[18446744073709551615, 10]");
}

#[test]
fn to_string_properties() {
    unsigned_matrix_gen().test_properties(|m| {
        let s = m.to_string();
        assert_eq!(m.to_debug_string(), s);
        if m.height() == 0 || m.width() == 0 {
            assert_eq!(s, format!("[]_{{{},{}}}", m.height(), m.width()));
        } else {
            assert_eq!(
                s,
                format!("[{}]", rows(&m, ", ", u64::to_string).join("; "))
            );
            // The rows of the transpose are the columns.
            assert_eq!(
                u64::exact_from(m.transpose().to_string().matches(';').count() + 1),
                m.width()
            );
        }
        assert!(s.starts_with('['));
    });

    unsigned_matrix_gen().test_properties(|m| {
        // A `Vec` of matrices is written with each matrix as `Display` writes it.
        let ms = vec![m.clone(), m.clone()];
        assert_eq!(ms.to_debug_string(), format!("[{m}, {m}]"));
    });
}
