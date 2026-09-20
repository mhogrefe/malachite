// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(not(feature = "std"))]
use hashbrown::HashMap;
use itertools::Itertools;
#[cfg(feature = "std")]
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

pub fn exhaustive_b_tree_maps_helper_helper<
    K: Clone + Debug + Ord,
    V: Clone + Debug + Eq,
    I: Iterator<Item = BTreeMap<K, V>>,
>(
    xss: I,
    out: &[BTreeMap<K, V>],
) {
    let xss = xss.take(20).collect_vec();
    assert_eq!(xss.as_slice(), out);
}

pub fn exhaustive_b_tree_maps_small_helper_helper<
    K: Clone + Debug + Ord,
    V: Clone + Debug + Eq,
    I: Clone + Iterator<Item = BTreeMap<K, V>>,
>(
    xss: I,
    out_len: usize,
    out: &[BTreeMap<K, V>],
) {
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.as_slice(), out);
    assert_eq!(xss.count(), out_len);
}

pub fn exhaustive_hash_maps_helper_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq,
    I: Iterator<Item = HashMap<K, V>>,
>(
    xss: I,
    out: &[HashMap<K, V>],
) {
    let xss = xss.take(20).collect_vec();
    assert_eq!(xss.as_slice(), out);
}

pub fn exhaustive_hash_maps_small_helper_helper<
    K: Clone + Debug + Eq + Hash,
    V: Clone + Debug + Eq,
    I: Clone + Iterator<Item = HashMap<K, V>>,
>(
    xss: I,
    out_len: usize,
    out: &[HashMap<K, V>],
) {
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(xss_prefix.as_slice(), out);
    assert_eq!(xss.count(), out_len);
}
