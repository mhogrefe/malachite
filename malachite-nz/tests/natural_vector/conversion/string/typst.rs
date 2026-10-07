// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::natural_vector_gen;

#[test]
fn test_to_typst_string() {
    let test = |xs: &[u32], out: &str| {
        let v = NaturalVector {
            elements: xs.iter().copied().map(Natural::from).collect(),
        };
        assert_eq!(v.to_typst_string(), out);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, 2, 3], "(1, 2, 3)");
    test(&[u32::MAX, 10], "(4294967295, 10)");
}

#[test]
fn to_typst_string_properties() {
    natural_vector_gen().test_properties(|v| {
        let typst = v.to_typst_string();
        // The fragment is the elements' own fragments, comma-separated, in parentheses.
        assert_eq!(
            typst,
            format!(
                "({})",
                v.elements.iter().map(Natural::to_typst_string).join(", ")
            )
        );
        // A `Natural`'s fragment is its decimal digits, so the fragment is the `Display` output.
        assert_eq!(typst, v.to_string());
    });
}
