// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::strings::typst::assert_typst_compiles;
use itertools::Itertools;
use malachite_base::strings::typst::ToTypst;
use malachite_base::test_util::generators::unsigned_vec_gen;
use std::collections::{BTreeMap, HashMap};

#[test]
fn test_map_to_typst() {
    let test = |entries: &[(u8, u8)], out: &str| {
        assert_eq!(
            entries
                .iter()
                .copied()
                .collect::<BTreeMap<_, _>>()
                .to_typst_string(),
            out
        );
        // The `HashMap` fragment agrees with the `BTreeMap` one.
        assert_eq!(
            entries
                .iter()
                .copied()
                .collect::<HashMap<_, _>>()
                .to_typst_string(),
            out
        );
    };
    test(&[], "{}");
    test(&[(1, 10)], "{1 |-> 10}");
    test(
        &[(1, 10), (2, 20), (3, 30)],
        "{1 |-> 10, 2 |-> 20, 3 |-> 30}",
    );
    // the entries come out in ascending key order, whatever order they went in
    test(
        &[(3, 30), (1, 10), (2, 20)],
        "{1 |-> 10, 2 |-> 20, 3 |-> 30}",
    );
    // a key and a value may be equal without the fragment losing track of which is which
    test(&[(1, 1), (2, 2)], "{1 |-> 1, 2 |-> 2}");
}

#[test]
fn test_map_to_typst_element_types() {
    // The keys' and values' own fragments are used, whatever they are.
    assert_eq!(
        BTreeMap::from([("hi", 1u8)]).to_typst_string(),
        r#"{"hi" |-> 1}"#
    );
    assert_eq!(
        BTreeMap::from([(1u8, None::<u8>), (2, Some(3))]).to_typst_string(),
        "{1 |-> bot, 2 |-> [3]}"
    );
    // nesting
    assert_eq!(
        BTreeMap::from([(1u8, BTreeMap::from([(2u8, 3u8)]))]).to_typst_string(),
        "{1 |-> {2 |-> 3}}"
    );
}

#[test]
fn test_map_to_typst_is_injective() {
    // Distinct maps never share a fragment, and a map is never confused with the set of its keys or
    // with a sequence.
    let fragments = [
        BTreeMap::<u8, u8>::new().to_typst_string(),
        BTreeMap::from([(1u8, 2u8)]).to_typst_string(),
        BTreeMap::from([(2u8, 1u8)]).to_typst_string(),
        BTreeMap::from([(1u8, 2u8), (2, 1)]).to_typst_string(),
        vec![1u8, 2].to_typst_string(),
    ];
    assert_eq!(fragments.iter().unique().count(), fragments.len());
}

#[test]
fn map_to_typst_properties() {
    let mut frags = Vec::new();
    unsigned_vec_gen::<u8>().test_properties(|xs| {
        let bm = xs
            .iter()
            .map(|&x| (x, x.wrapping_mul(3)))
            .collect::<BTreeMap<_, _>>();
        let hm = xs
            .iter()
            .map(|&x| (x, x.wrapping_mul(3)))
            .collect::<HashMap<_, _>>();
        let s = bm.to_typst_string();
        // A `HashMap` never depends on its hasher: it agrees with the `BTreeMap` of the same
        // entries, and so with itself from one run to the next.
        assert_eq!(hm.to_typst_string(), s);
        assert!(s.starts_with('{'));
        assert!(s.ends_with('}'));
        // The fragment is the entries' own fragments, in ascending key order.
        let inner = &s[1..s.len() - 1];
        assert_eq!(
            inner,
            bm.iter()
                .map(|(k, v)| format!("{} |-> {}", k.to_typst(), v.to_typst()))
                .join(", ")
        );
        frags.push(s);
    });
    assert_typst_compiles(&frags);
}
