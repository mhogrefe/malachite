// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::rational_vector_gen;

#[test]
fn test_to_typst_string() {
    let test = |xs: &[i32], out: &str| {
        let v = RationalVector {
            elements: xs.iter().copied().map(Rational::from).collect(),
        };
        assert_eq!(v.to_typst_string(), out);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, 2, 3], "(1, 2, 3)");
    test(&[-5], "(-5)");
    test(&[i32::MIN, 10], "(-2147483648, 10)");
    let test_str = |s, out: &str| {
        assert_eq!(RationalVector::from_str(s).unwrap().to_typst_string(), out);
    };
    test_str("(22/7, -1)", "(frac(22, 7), -1)");
    test_str("(1/2, -3/4)", "(frac(1, 2), -frac(3, 4))");
}

#[test]
fn to_typst_string_properties() {
    rational_vector_gen().test_properties(|v| {
        let typst = v.to_typst_string();
        // The fragment is the elements' own fragments, comma-separated, in parentheses.
        assert_eq!(
            typst,
            format!(
                "({})",
                v.elements.iter().map(Rational::to_typst_string).join(", ")
            )
        );
    });
}
