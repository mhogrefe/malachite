// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::{unsigned_vec_gen, unsigned_vector_gen};
use malachite_base::vector::Vector;

#[test]
fn test_to_typst_string() {
    let test = |xs: &[i32], out: &str| {
        assert_eq!(Vector::from_elements(xs).to_typst_string(), out);
    };
    test(&[], "()");
    test(&[0], "(0)");
    test(&[1, -2, 3], "(1, -2, 3)");
}

#[test]
fn to_typst_string_properties() {
    unsigned_vector_gen().test_properties(|v| {
        // A `u64`'s fragment is its decimal digits, so the fragment is the `Display` output.
        assert_eq!(v.to_typst_string(), v.to_string());
    });

    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let v = Vector {
            elements: xs.clone(),
        };
        // The fragment is the elements' own fragments, comma-separated, in parentheses.
        assert_eq!(
            v.to_typst_string(),
            format!("({})", xs.iter().map(u8::to_typst_string).join(", "))
        );
    });
}
