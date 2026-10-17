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
use malachite_base::strings::latex::ToLatex;
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
fn test_to_latex_string() {
    let test = |s, out: &str| {
        assert_eq!(RationalMatrix::from_str(s).unwrap().to_latex_string(), out);
    };
    test("[]_{0,0}", "\\begin{pmatrix}\\end{pmatrix}_{0\\times 0}");
    test("[]_{0,4}", "\\begin{pmatrix}\\end{pmatrix}_{0\\times 4}");
    test("[]_{4,0}", "\\begin{pmatrix}\\end{pmatrix}_{4\\times 0}");
    test("[-5]", "\\begin{pmatrix}-5\\end{pmatrix}");
    test("[1, 2, 3]", "\\begin{pmatrix}1&2&3\\end{pmatrix}");
    test("[1; 2; 3]", "\\begin{pmatrix}1\\\\2\\\\3\\end{pmatrix}");
    test("[1, 2; 3, 4]", "\\begin{pmatrix}1&2\\\\3&4\\end{pmatrix}");
    test(
        "[-2147483648, 10]",
        "\\begin{pmatrix}-2147483648&10\\end{pmatrix}",
    );
    test(
        "[1/2, -3/4]",
        "\\begin{pmatrix}\\frac{1}{2}&-\\frac{3}{4}\\end{pmatrix}",
    );
}

#[test]
fn to_latex_string_properties() {
    rational_matrix_gen().test_properties(|m| {
        let latex = m.to_latex_string();
        if m.height() == 0 || m.width() == 0 {
            assert_eq!(
                latex,
                format!(
                    "\\begin{{pmatrix}}\\end{{pmatrix}}_{{{}\\times {}}}",
                    m.height(),
                    m.width()
                )
            );
        } else {
            // The fragment is the elements' own fragments, `&`-separated, with rows separated by
            // `\\`, in a `pmatrix`.
            assert_eq!(
                latex,
                format!(
                    "\\begin{{pmatrix}}{}\\end{{pmatrix}}",
                    rows(&m, "&", Rational::to_latex_string).join("\\\\")
                )
            );
        }
    });
}
