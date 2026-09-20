// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::maps::exhaustive::*;
use malachite_base::test_util::maps::exhaustive::{
    exhaustive_b_tree_maps_helper_helper, exhaustive_b_tree_maps_small_helper_helper,
};
use std::collections::{BTreeMap, HashSet};
use std::fmt::Debug;

fn exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range_helper<
    I: Clone + Iterator,
    J: Clone + Iterator,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys: I,
    values: J,
    out: &[BTreeMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Ord,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_b_tree_maps_helper_helper(
        exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range(
            size_a,
            size_b,
            unique_value_count_a,
            unique_value_count_b,
            keys,
            values,
        ),
        out,
    );
}

fn exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range_small_helper<
    I: Clone + Iterator,
    J: Clone + Iterator,
>(
    size_a: u64,
    size_b: u64,
    unique_value_count_a: u64,
    unique_value_count_b: u64,
    keys: I,
    values: J,
    out_len: usize,
    out: &[BTreeMap<I::Item, J::Item>],
) where
    I::Item: Clone + Debug + Ord,
    J::Item: Clone + Debug + Eq,
{
    exhaustive_b_tree_maps_small_helper_helper(
        exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range(
            size_a,
            size_b,
            unique_value_count_a,
            unique_value_count_b,
            keys,
            values,
        ),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range() {
    exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range_small_helper(
        2,
        3,
        1,
        1,
        'a'..='c',
        0..3u8,
        12,
        &[
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'a' => 0, 'c' => 0},
            btreemap! {'a' => 1, 'b' => 1},
            btreemap! {'b' => 0, 'c' => 0},
            btreemap! {'a' => 2, 'b' => 2},
            btreemap! {'a' => 1, 'c' => 1},
            btreemap! {'a' => 2, 'c' => 2},
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'b' => 1, 'c' => 1},
            btreemap! {'a' => 1, 'b' => 1, 'c' => 1},
            btreemap! {'b' => 2, 'c' => 2},
            btreemap! {'a' => 2, 'b' => 2, 'c' => 2},
        ],
    );
    exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range_small_helper(
        0,
        2,
        1,
        2,
        'a'..='b',
        0..2u8,
        8,
        &[
            btreemap! {'a' => 0},
            btreemap! {'b' => 0},
            btreemap! {'a' => 1},
            btreemap! {'a' => 0, 'b' => 0},
            btreemap! {'b' => 1},
            btreemap! {'a' => 0, 'b' => 1},
            btreemap! {'a' => 1, 'b' => 1},
            btreemap! {'a' => 1, 'b' => 0},
        ],
    );
    exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range_helper(
        3,
        3,
        1,
        1,
        'a'..,
        0u8..,
        &[
            btreemap! {'a' => 0, 'b' => 0, 'c' => 0},
            btreemap! {'a' => 0, 'b' => 0, 'd' => 0},
            btreemap! {'a' => 1, 'b' => 1, 'c' => 1},
            btreemap! {'a' => 0, 'c' => 0, 'd' => 0},
            btreemap! {'a' => 2, 'b' => 2, 'c' => 2},
            btreemap! {'a' => 1, 'b' => 1, 'd' => 1},
            btreemap! {'a' => 3, 'b' => 3, 'c' => 3},
            btreemap! {'b' => 0, 'c' => 0, 'd' => 0},
            btreemap! {'a' => 4, 'b' => 4, 'c' => 4},
            btreemap! {'a' => 2, 'b' => 2, 'd' => 2},
            btreemap! {'a' => 5, 'b' => 5, 'c' => 5},
            btreemap! {'a' => 1, 'c' => 1, 'd' => 1},
            btreemap! {'a' => 6, 'b' => 6, 'c' => 6},
            btreemap! {'a' => 3, 'b' => 3, 'd' => 3},
            btreemap! {'a' => 7, 'b' => 7, 'c' => 7},
            btreemap! {'a' => 0, 'b' => 0, 'e' => 0},
            btreemap! {'a' => 8, 'b' => 8, 'c' => 8},
            btreemap! {'a' => 4, 'b' => 4, 'd' => 4},
            btreemap! {'a' => 9, 'b' => 9, 'c' => 9},
            btreemap! {'a' => 2, 'c' => 2, 'd' => 2},
        ],
    );
}

// Brute force: every subset of the keys, crossed with every assignment of values, filtered by the
// two restrictions. The generator must produce exactly these, each once.
fn brute(sa: u64, sb: u64, va: u64, vb: u64, nk: u8, nv: u8) -> Vec<BTreeMap<u8, u8>> {
    let mut out = vec![];
    for mask in 0u32..(1 << nk) {
        let ks = (0..nk).filter(|i| mask >> i & 1 == 1).collect_vec();
        let k = u64::try_from(ks.len()).unwrap();
        if k < sa || k > sb {
            continue;
        }
        let total = u64::from(nv)
            .checked_pow(u32::try_from(k).unwrap())
            .unwrap_or(0);
        for code in 0..total {
            let vs = (0..k)
                .map(|i| {
                    u8::try_from(
                        code / u64::from(nv).pow(u32::try_from(i).unwrap()) % u64::from(nv),
                    )
                    .unwrap()
                })
                .collect_vec();
            let d = u64::try_from(vs.iter().collect::<HashSet<_>>().len()).unwrap();
            if d < va || d > vb {
                continue;
            }
            out.push(ks.iter().copied().zip(vs).collect());
        }
    }
    out.sort();
    out.dedup();
    out
}

#[test]
fn exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range_matches_brute_force() {
    for nk in 0..=3u8 {
        for nv in 0..=3u8 {
            for sa in 0..=3u64 {
                for sb in sa..=3 {
                    for va in 0..=3u64 {
                        for vb in va..=3 {
                            let mut actual =
                                exhaustive_b_tree_maps_size_and_unique_value_count_inclusive_range(
                                    sa,
                                    sb,
                                    va,
                                    vb,
                                    0..nk,
                                    0..nv,
                                )
                                .collect_vec();
                            let len = actual.len();
                            actual.sort();
                            actual.dedup();
                            assert_eq!(
                                actual.len(),
                                len,
                                "duplicates: {nk} {nv} {sa} {sb} {va} {vb}"
                            );
                            assert_eq!(
                                actual,
                                brute(sa, sb, va, vb, nk, nv),
                                "{nk} {nv} {sa} {sb} {va} {vb}"
                            );
                        }
                    }
                }
            }
        }
    }
}
