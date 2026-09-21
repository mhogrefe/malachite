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
use std::collections::{BTreeSet, HashSet};

#[test]
fn test_set_to_latex() {
    let test = |xs: &[u8], out: &str| {
        assert_eq!(
            xs.iter()
                .copied()
                .collect::<BTreeSet<_>>()
                .to_latex()
                .to_string(),
            out
        );
        // The `HashSet` fragment agrees with the `BTreeSet` one.
        assert_eq!(
            xs.iter()
                .copied()
                .collect::<HashSet<_>>()
                .to_latex()
                .to_string(),
            out
        );
    };
    test(&[], r"\left\{\right\}");
    test(&[5], r"\left\{5\right\}");
    test(&[1, 2, 3], r"\left\{1, 2, 3\right\}");
    // the elements come out in ascending order, whatever order they went in
    test(&[3, 2, 1], r"\left\{1, 2, 3\right\}");
    test(&[2, 1, 2, 3, 1], r"\left\{1, 2, 3\right\}");
}

#[test]
fn test_set_to_latex_element_types() {
    // The elements' own fragments are used, whatever they are.
    assert_eq!(
        BTreeSet::from(["hi", "yo"]).to_latex().to_string(),
        r"\left\{\text{hi}, \text{yo}\right\}"
    );
    assert_eq!(
        BTreeSet::from([true, false]).to_latex().to_string(),
        r"\left\{\text{F}, \text{T}\right\}"
    );
    // nesting
    assert_eq!(
        BTreeSet::from([BTreeSet::from([1u8]), BTreeSet::from([2, 3])])
            .to_latex()
            .to_string(),
        r"\left\{\left\{1\right\}, \left\{2, 3\right\}\right\}"
    );
}

#[test]
fn test_set_to_latex_is_injective_over_nesting() {
    // The braces exist so that distinct values never share a fragment.
    let fragments = [
        BTreeSet::from([1u8, 2]).to_latex().to_string(),
        BTreeSet::from([BTreeSet::from([1u8]), BTreeSet::from([2])])
            .to_latex()
            .to_string(),
        BTreeSet::from([BTreeSet::from([1u8, 2])])
            .to_latex()
            .to_string(),
        BTreeSet::<u8>::new().to_latex().to_string(),
        BTreeSet::from([BTreeSet::<u8>::new()])
            .to_latex()
            .to_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}

#[test]
fn set_to_latex_properties() {
    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let bs = xs.iter().copied().collect::<BTreeSet<_>>();
        let hs = xs.iter().copied().collect::<HashSet<_>>();
        let s = bs.to_latex().to_string();
        // A `HashSet` never depends on its hasher: it agrees with the `BTreeSet` of the same
        // elements, and so with itself from one run to the next.
        assert_eq!(hs.to_latex().to_string(), s);
        assert!(s.starts_with(r"\left\{"));
        assert!(s.ends_with(r"\right\}"));
        // The fragment is the elements' own fragments, in ascending order, separated by commas.
        let inner = &s[r"\left\{".len()..s.len() - r"\right\}".len()];
        assert_eq!(
            inner,
            bs.iter().map(|x| x.to_latex().to_string()).join(", ")
        );
    });
}
