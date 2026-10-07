// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_to_latex_string() {
    let test = |xs: &[u32], out: &str| {
        let v = NaturalVector {
            elements: xs.iter().copied().map(Natural::from).collect(),
        };
        assert_eq!(v.to_latex_string(), out);
    };
    test(&[], "\\left(\\right)");
    test(&[0], "\\left(0\\right)");
    test(&[1, 2, 3], "\\left(1, 2, 3\\right)");
    test(&[u32::MAX, 10], "\\left(4294967295, 10\\right)");
}

#[test]
fn to_latex_string_properties() {
    natural_vector_gen().test_properties(|v| {
        let latex = v.to_latex_string();
        // The fragment is the elements' own fragments, comma-separated, in parentheses.
        assert_eq!(
            latex,
            format!(
                "\\left({}\\right)",
                v.elements.iter().map(Natural::to_latex_string).join(", ")
            )
        );
        // A `Natural`'s fragment is its decimal digits, so the fragment is the `Display` output
        // with growing parentheses.
        let s = v.to_string();
        assert_eq!(latex, format!("\\left({}\\right)", &s[1..s.len() - 1]));
    });
}
