// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
use itertools::Itertools;
use malachite_base::strings::latex::ToLatex;
use malachite_base::test_util::generators::unsigned_vec_gen;
use std::collections::BTreeMap;
#[cfg(feature = "std")]
use std::collections::HashMap;

#[test]
fn test_map_to_latex() {
    let test = |entries: &[(u8, u8)], out: &str| {
        assert_eq!(
            entries
                .iter()
                .copied()
                .collect::<BTreeMap<_, _>>()
                .to_latex_string(),
            out
        );
        // The `HashMap` fragment agrees with the `BTreeMap` one.
        assert_eq!(
            entries
                .iter()
                .copied()
                .collect::<HashMap<_, _>>()
                .to_latex_string(),
            out
        );
    };
    test(&[], r"\left\{\right\}");
    test(&[(1, 10)], r"\left\{1 \mapsto 10\right\}");
    test(
        &[(1, 10), (2, 20), (3, 30)],
        r"\left\{1 \mapsto 10, 2 \mapsto 20, 3 \mapsto 30\right\}",
    );
    // the entries come out in ascending key order, whatever order they went in
    test(
        &[(3, 30), (1, 10), (2, 20)],
        r"\left\{1 \mapsto 10, 2 \mapsto 20, 3 \mapsto 30\right\}",
    );
    // a key and a value may be equal without the fragment losing track of which is which
    test(
        &[(1, 1), (2, 2)],
        r"\left\{1 \mapsto 1, 2 \mapsto 2\right\}",
    );
}

#[test]
fn test_map_to_latex_element_types() {
    // The keys' and values' own fragments are used, whatever they are.
    assert_eq!(
        BTreeMap::from([("hi", 1u8)]).to_latex_string(),
        r"\left\{\text{hi} \mapsto 1\right\}"
    );
    assert_eq!(
        BTreeMap::from([(1u8, None::<u8>), (2, Some(3))]).to_latex_string(),
        r"\left\{1 \mapsto \bot, 2 \mapsto \left[3\right]\right\}"
    );
    // nesting
    assert_eq!(
        BTreeMap::from([(1u8, BTreeMap::from([(2u8, 3u8)]))]).to_latex_string(),
        r"\left\{1 \mapsto \left\{2 \mapsto 3\right\}\right\}"
    );
}

#[test]
fn test_map_to_latex_is_injective() {
    // Distinct maps never share a fragment, and a map is never confused with the set of its keys or
    // with a sequence.
    let fragments = [
        BTreeMap::<u8, u8>::new().to_latex_string(),
        BTreeMap::from([(1u8, 2u8)]).to_latex_string(),
        BTreeMap::from([(2u8, 1u8)]).to_latex_string(),
        BTreeMap::from([(1u8, 2u8), (2, 1)]).to_latex_string(),
        vec![1u8, 2].to_latex_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}

#[test]
fn map_to_latex_properties() {
    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let bm = xs
            .iter()
            .map(|&x| (x, x.wrapping_mul(3)))
            .collect::<BTreeMap<_, _>>();
        let hm = xs
            .iter()
            .map(|&x| (x, x.wrapping_mul(3)))
            .collect::<HashMap<_, _>>();
        let s = bm.to_latex_string();
        // A `HashMap` never depends on its hasher: it agrees with the `BTreeMap` of the same
        // entries, and so with itself from one run to the next.
        assert_eq!(hm.to_latex_string(), s);
        assert!(s.starts_with(r"\left\{"));
        assert!(s.ends_with(r"\right\}"));
        // The fragment is the entries' own fragments, in ascending key order.
        let inner = &s[r"\left\{".len()..s.len() - r"\right\}".len()];
        assert_eq!(
            inner,
            bm.iter()
                .map(|(k, v)| format!("{} \\mapsto {}", k.to_latex(), v.to_latex()))
                .join(", ")
        );
    });
}
