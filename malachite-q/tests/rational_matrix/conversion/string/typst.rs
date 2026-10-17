// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::strings::typst::ToTypst;
use malachite_q::Rational;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::test_util::generators::rational_matrix_gen;

// The rows of a matrix with at least one row and one column, each as the strings `f` makes of its
// elements, joined by `element_separator`.
fn rows(
    m: &RationalMatrix,
    element_separator: &str,
    f: impl Fn(&Rational) -> String,
) -> Vec<String> {
    m.elements
        .chunks(usize::exact_from(m.width()))
        .map(|row| row.iter().map(&f).join(element_separator))
        .collect()
}

#[test]
fn test_to_typst_string() {
    let test = |s, out: &str| {
        assert_eq!(RationalMatrix::from_str(s).unwrap().to_typst_string(), out);
    };
    test("[]_{0,0}", "mat()_(0 times 0)");
    test("[]_{0,4}", "mat()_(0 times 4)");
    test("[]_{4,0}", "mat()_(4 times 0)");
    test("[-5]", "mat(-5)");
    test("[1, 2, 3]", "mat(1, 2, 3)");
    test("[1; 2; 3]", "mat(1; 2; 3)");
    test("[1, 2; 3, 4]", "mat(1, 2; 3, 4)");
    test("[-2147483648, 10]", "mat(-2147483648, 10)");
    test("[1/2, -3/4]", "mat(frac(1, 2), -frac(3, 4))");
}

#[test]
fn to_typst_string_properties() {
    rational_matrix_gen().test_properties(|m| {
        let typst = m.to_typst_string();
        if m.height() == 0 || m.width() == 0 {
            assert_eq!(typst, format!("mat()_({} times {})", m.height(), m.width()));
        } else {
            assert_eq!(
                typst,
                format!(
                    "mat({})",
                    rows(&m, ", ", Rational::to_typst_string).join("; ")
                )
            );
        }
    });
}
