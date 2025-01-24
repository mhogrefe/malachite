// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::{repeat_n, Itertools};
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::sets::random::random_hash_sets_fixed_length;
use malachite_base::vecs::random::random_ordered_unique_vecs_fixed_length;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

fn random_hash_sets_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[HashSet<I::Item>],
) where
    I::Item: Debug + Eq + Hash,
{
    let xs = random_hash_sets_fixed_length(len, xs);
    let values = xs.take(20).collect_vec();
    assert_eq!(values.as_slice(), expected_values);
}

#[test]
fn test_random_hash_sets_fixed_length() {
    random_hash_sets_fixed_length_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &repeat_n(hashset! {}, 20).collect_vec(),
    );
    random_hash_sets_fixed_length_helper(
        1,
        random_bools(EXAMPLE_SEED),
        &[
            hashset! {true},
            hashset! {false},
            hashset! {false},
            hashset! {false},
            hashset! {true},
            hashset! {true},
            hashset! {true},
            hashset! {false},
            hashset! {true},
            hashset! {true},
            hashset! {true},
            hashset! {true},
            hashset! {false},
            hashset! {true},
            hashset! {true},
            hashset! {true},
            hashset! {true},
            hashset! {false},
            hashset! {true},
            hashset! {false},
        ],
    );
    random_hash_sets_fixed_length_helper(
        3,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            hashset! {69, 113, 239},
            hashset! {108, 210, 228},
            hashset! {87, 161, 168},
            hashset! {32, 83, 110},
            hashset! {34, 89, 188},
            hashset! {93, 200, 238},
            hashset! {115, 149, 189},
            hashset! {149, 201, 217},
            hashset! {31, 117, 146},
            hashset! {72, 151, 169},
            hashset! {7, 33, 174},
            hashset! {38, 81, 144},
            hashset! {72, 113, 127},
            hashset! {107, 128, 233},
            hashset! {12, 46, 119},
            hashset! {18, 164, 243},
            hashset! {59, 114, 174},
            hashset! {39, 174, 247},
            hashset! {104, 160, 184},
            hashset! {37, 100, 252},
        ],
    );
    random_hash_sets_fixed_length_helper(
        2,
        random_ordered_unique_vecs_fixed_length(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            hashset! {vec![69, 108], vec![113, 239]},
            hashset! {vec![161, 168], vec![210, 228]},
            hashset! {vec![32, 87], vec![83, 110]},
            hashset! {vec![34, 188], vec![89, 238]},
            hashset! {vec![93, 200], vec![115, 149]},
            hashset! {vec![149, 189], vec![201, 217]},
            hashset! {vec![31, 72], vec![117, 146]},
            hashset! {vec![33, 174], vec![151, 169]},
            hashset! {vec![7, 38], vec![81, 144]},
            hashset! {vec![72, 127], vec![113, 128]},
            hashset! {vec![46, 119], vec![107, 233]},
            hashset! {vec![12, 18], vec![164, 243]},
            hashset! {vec![59, 247], vec![114, 174]},
            hashset! {vec![39, 174], vec![160, 184]},
            hashset! {vec![37, 104], vec![100, 252]},
            hashset! {vec![69, 107], vec![122, 228]},
            hashset! {vec![142, 179], vec![242, 248]},
            hashset! {vec![61, 189], vec![233, 239]},
            hashset! {vec![7, 192], vec![85, 235]},
            hashset! {vec![90, 200], vec![178, 185]},
        ],
    );
}
