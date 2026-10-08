// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::unsigned_vector::UnsignedVector;

#[test]
fn test_to_latex_string() {
    let test = |xs: &[u32], out: &str| {
        assert_eq!(UnsignedVector::from_elements(xs).to_latex_string(), out);
    };
    test(&[], "\\left(\\right)");
    test(&[0], "\\left(0\\right)");
    test(&[1, 2, 3], "\\left(1, 2, 3\\right)");
    test(&[u32::MAX], "\\left(4294967295\\right)");
}

#[test]
fn to_latex_string_properties() {
    unsigned_vector_gen().test_properties(|v| {
        // A `u64`'s fragment is its decimal digits, so the fragment is the `Display` output with
        // growing parentheses.
        let s = v.to_string();
        assert_eq!(
            v.to_latex_string(),
            format!("\\left({}\\right)", &s[1..s.len() - 1])
        );
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = UnsignedVector {
            elements: xs.clone(),
        };
        // The fragment is the elements' own fragments, comma-separated, in parentheses.
        assert_eq!(
            v.to_latex_string(),
            format!(
                "\\left({}\\right)",
                xs.iter().map(u8::to_latex_string).join(", ")
            )
        );
    });
}
