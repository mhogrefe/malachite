// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen;

fn matrix(height: usize, width: usize, xs: &[u32]) -> NaturalMatrix {
    NaturalMatrix::from_owned_elements(
        height,
        width,
        xs.iter().copied().map(Natural::from).collect(),
    )
}

// The rows of a matrix with at least one row and one column, each as the strings `f` makes of its
// elements, joined by `element_separator`.
fn rows(m: &NaturalMatrix, element_separator: &str, f: impl Fn(&Natural) -> String) -> Vec<String> {
    m.elements
        .chunks(m.width())
        .map(|row| row.iter().map(&f).join(element_separator))
        .collect()
}

#[test]
fn test_to_string() {
    let test = |height, width, xs: &[u32], out| {
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
    test(1, 2, &[u32::MAX, 10], "[4294967295, 10]");
}

#[test]
fn to_string_properties() {
    natural_matrix_gen().test_properties(|m| {
        let s = m.to_string();
        assert_eq!(m.to_debug_string(), s);
        if m.height() == 0 || m.width() == 0 {
            assert_eq!(s, format!("[]_{{{},{}}}", m.height(), m.width()));
        } else {
            assert_eq!(
                s,
                format!("[{}]", rows(&m, ", ", Natural::to_string).join("; "))
            );
            // The rows of the transpose are the columns.
            assert_eq!(
                m.transpose().to_string().matches(';').count() + 1,
                m.width()
            );
        }
        assert!(s.starts_with('['));
    });

    natural_matrix_gen().test_properties(|m| {
        // A `Vec` of matrices is written with each matrix as `Display` writes it.
        let ms = vec![m.clone(), m.clone()];
        assert_eq!(ms.to_debug_string(), format!("[{m}, {m}]"));
    });
}
