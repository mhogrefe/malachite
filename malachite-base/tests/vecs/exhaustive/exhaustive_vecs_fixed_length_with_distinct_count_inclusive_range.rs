// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::*;
use std::fmt::Debug;

fn exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_helper<I: Clone + Iterator>(
    k: u64,
    a: u64,
    b: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(
        exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range(k, a, b, xs),
        out,
    );
}

fn exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper<
    I: Clone + Iterator,
>(
    k: u64,
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(
        exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range(k, a, b, xs),
        out_len,
        out,
    );
}

#[test]
fn test_exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range() {
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        0,
        0,
        0,
        1..=3u8,
        1,
        &[&[]],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        0,
        1,
        1,
        1..=3u8,
        0,
        &[],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        1,
        1,
        1,
        1..=3u8,
        3,
        &[&[1], &[2], &[3]],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        2,
        1,
        1,
        1..=3u8,
        3,
        &[&[1, 1], &[2, 2], &[3, 3]],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        2,
        2,
        2,
        1..=3u8,
        6,
        &[&[1, 2], &[2, 1], &[1, 3], &[3, 1], &[2, 3], &[3, 2]],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        2,
        1,
        2,
        1..=3u8,
        9,
        &[&[1, 1], &[1, 2], &[2, 2], &[3, 3], &[2, 1], &[1, 3], &[3, 1], &[2, 3], &[3, 2]],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        3,
        1,
        2,
        1..=3u8,
        21,
        &[
            &[1, 1, 1],
            &[1, 1, 2],
            &[2, 2, 2],
            &[3, 3, 3],
            &[2, 2, 1],
            &[1, 2, 1],
            &[2, 1, 2],
            &[1, 1, 3],
            &[3, 3, 1],
            &[1, 3, 1],
            &[3, 1, 3],
            &[1, 2, 2],
            &[2, 1, 1],
            &[1, 3, 3],
            &[3, 1, 1],
            &[2, 2, 3],
            &[3, 3, 2],
            &[2, 3, 2],
            &[3, 2, 3],
            &[2, 3, 3],
        ],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        3,
        2,
        3,
        1..=3u8,
        24,
        &[
            &[1, 1, 2],
            &[1, 2, 3],
            &[2, 2, 1],
            &[1, 2, 1],
            &[2, 1, 2],
            &[1, 3, 2],
            &[1, 1, 3],
            &[2, 1, 3],
            &[3, 3, 1],
            &[2, 3, 1],
            &[1, 3, 1],
            &[3, 1, 3],
            &[1, 2, 2],
            &[3, 1, 2],
            &[2, 1, 1],
            &[1, 3, 3],
            &[3, 1, 1],
            &[3, 2, 1],
            &[2, 2, 3],
            &[3, 3, 2],
        ],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        3,
        4,
        5,
        1..=3u8,
        0,
        &[],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        4,
        2,
        2,
        1..=3u8,
        42,
        &[
            &[1, 1, 1, 2],
            &[2, 2, 2, 1],
            &[1, 1, 2, 1],
            &[2, 2, 1, 2],
            &[1, 1, 1, 3],
            &[3, 3, 3, 1],
            &[1, 1, 3, 1],
            &[3, 3, 1, 3],
            &[1, 1, 2, 2],
            &[2, 2, 1, 1],
            &[1, 2, 1, 1],
            &[2, 1, 2, 2],
            &[1, 1, 3, 3],
            &[3, 3, 1, 1],
            &[1, 3, 1, 1],
            &[3, 1, 3, 3],
            &[2, 2, 2, 3],
            &[3, 3, 3, 2],
            &[2, 2, 3, 2],
            &[3, 3, 2, 3],
        ],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        3,
        1,
        2,
        exhaustive_bools(),
        8,
        &[
            &[false, false, false],
            &[false, false, true],
            &[true, true, true],
            &[true, true, false],
            &[false, true, false],
            &[true, false, true],
            &[false, true, true],
            &[true, false, false],
        ],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        2,
        1,
        2,
        nevers(),
        0,
        &[],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_small_helper(
        2,
        1,
        1,
        exhaustive_units(),
        1,
        &[&[(), ()]],
    );
    exhaustive_vecs_fixed_length_with_distinct_count_inclusive_range_helper(
        3,
        1,
        3,
        0u32..,
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[1, 1, 1],
            &[0, 1, 2],
            &[2, 2, 2],
            &[1, 1, 0],
            &[3, 3, 3],
            &[4, 4, 4],
            &[5, 5, 5],
            &[0, 1, 0],
            &[6, 6, 6],
            &[0, 1, 3],
            &[7, 7, 7],
            &[1, 0, 1],
            &[8, 8, 8],
            &[0, 0, 2],
            &[9, 9, 9],
            &[2, 2, 0],
            &[10, 10, 10],
            &[0, 2, 1],
        ],
    );
}
