// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_to_latex_string() {
    let test = |xs: &[i32], out: &str| {
        let v = RationalVector {
            elements: xs.iter().copied().map(Rational::from).collect(),
        };
        assert_eq!(v.to_latex_string(), out);
    };
    test(&[], "\\left(\\right)");
    test(&[0], "\\left(0\\right)");
    test(&[1, 2, 3], "\\left(1, 2, 3\\right)");
    test(&[-5], "\\left(-5\\right)");
    test(&[i32::MIN, 10], "\\left(-2147483648, 10\\right)");
    let test_str = |s, out: &str| {
        assert_eq!(RationalVector::from_str(s).unwrap().to_latex_string(), out);
    };
    test_str("(22/7, -1)", "\\left(\\frac{22}{7}, -1\\right)");
    test_str("(1/2, -3/4)", "\\left(\\frac{1}{2}, -\\frac{3}{4}\\right)");
}

#[test]
fn to_latex_string_properties() {
    rational_vector_gen().test_properties(|v| {
        let latex = v.to_latex_string();
        // The fragment is the elements' own fragments, comma-separated, in parentheses.
        assert_eq!(
            latex,
            format!(
                "\\left({}\\right)",
                v.elements.iter().map(Rational::to_latex_string).join(", ")
            )
        );
    });
}
