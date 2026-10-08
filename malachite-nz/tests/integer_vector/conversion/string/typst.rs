// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_gen;

#[test]
fn test_to_typst_string() {
    let test = |xs: &[i32], out: &str| {
        let v = IntegerVector {
            elements: xs.iter().copied().map(Integer::from).collect(),
        };
        assert_eq!(v.to_typst_string(), out);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, 2, 3], "(1, 2, 3)");
    test(&[-5], "(-5)");
    test(&[i32::MIN, 10], "(-2147483648, 10)");
}

#[test]
fn to_typst_string_properties() {
    integer_vector_gen().test_properties(|v| {
        let typst = v.to_typst_string();
        // The fragment is the elements' own fragments, comma-separated, in parentheses.
        assert_eq!(
            typst,
            format!(
                "({})",
                v.elements.iter().map(Integer::to_typst_string).join(", ")
            )
        );
        // An `Integer`'s fragment is its decimal digits, so the fragment is the `Display` output.
        assert_eq!(typst, v.to_string());
    });
}
