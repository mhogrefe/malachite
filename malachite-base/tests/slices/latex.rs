// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::unsigned_vec_gen;

#[test]
fn test_slice_to_latex() {
    let test = |xs: &[u8], out: &str| {
        assert_eq!(xs.to_latex().to_string(), out);
        // The `Vec` implementation agrees with the slice one.
        assert_eq!(xs.to_vec().to_latex().to_string(), out);
    };
    test(&[], r"\left[\right]");
    test(&[5], r"\left[5\right]");
    test(&[1, 2, 3], r"\left[1, 2, 3\right]");
    test(&[0, 255], r"\left[0, 255\right]");
}

#[test]
fn test_slice_to_latex_element_types() {
    // The elements' own fragments are used, whatever they are.
    assert_eq!(
        ["hi", "yo"].as_slice().to_latex().to_string(),
        r"\left[\text{hi}, \text{yo}\right]"
    );
    assert_eq!(
        [true, false].as_slice().to_latex().to_string(),
        r"\left[\text{T}, \text{F}\right]"
    );
    assert_eq!(
        ['α', 'β'].as_slice().to_latex().to_string(),
        r"\left[\alpha, \beta\right]"
    );
    assert_eq!(
        [Some(1u8), None].as_slice().to_latex().to_string(),
        r"\left[\left[1\right], \bot\right]"
    );
    // nesting
    assert_eq!(
        [vec![1u8], vec![2, 3]].as_slice().to_latex().to_string(),
        r"\left[\left[1\right], \left[2, 3\right]\right]"
    );
}

#[test]
fn test_slice_to_latex_is_injective_over_nesting() {
    // The brackets exist so that distinct values never share a fragment. Without them the first two
    // of these would both be `1, 2`.
    let fragments = [
        vec![1u8, 2].to_latex().to_string(),
        vec![vec![1u8], vec![2]].to_latex().to_string(),
        vec![vec![1u8, 2]].to_latex().to_string(),
        Vec::<u8>::new().to_latex().to_string(),
        vec![Vec::<u8>::new()].to_latex().to_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}

#[test]
fn slice_to_latex_properties() {
    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let s = xs.to_latex().to_string();
        // The `Vec` implementation agrees with the slice one.
        assert_eq!(xs.as_slice().to_latex().to_string(), s);
        assert!(s.starts_with(r"\left["));
        assert!(s.ends_with(r"\right]"));
        // The fragment is the elements' own fragments, in order, separated by commas.
        let inner = &s[r"\left[".len()..s.len() - r"\right]".len()];
        assert_eq!(
            inner,
            xs.iter().map(|x| x.to_latex().to_string()).join(", ")
        );
    });
}
