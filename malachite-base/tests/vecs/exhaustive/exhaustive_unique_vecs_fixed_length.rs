// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_unique_vecs_fixed_length;
use std::fmt::Debug;

fn exhaustive_unique_vecs_fixed_length_helper<I: Iterator>(len: u64, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_unique_vecs_fixed_length(len, xs), out);
}

fn exhaustive_unique_vecs_fixed_length_small_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(exhaustive_unique_vecs_fixed_length(len, xs), out_len, out);
}

#[test]
fn test_exhaustive_unique_vecs_fixed_length() {
    // This demonstrates that 0 ^ 0 == 1:
    exhaustive_unique_vecs_fixed_length_small_helper(0, nevers(), 1, &[&[]]);
    exhaustive_unique_vecs_fixed_length_small_helper(1, nevers(), 0, &[]);
    exhaustive_unique_vecs_fixed_length_small_helper(2, nevers(), 0, &[]);
    exhaustive_unique_vecs_fixed_length_small_helper(5, nevers(), 0, &[]);
    exhaustive_unique_vecs_fixed_length_small_helper(1, exhaustive_units(), 1, &[&[()]]);
    exhaustive_unique_vecs_fixed_length_small_helper(2, exhaustive_units(), 0, &[]);
    exhaustive_unique_vecs_fixed_length_small_helper(5, exhaustive_units(), 0, &[]);
    exhaustive_unique_vecs_fixed_length_small_helper(0, exhaustive_unsigneds::<u8>(), 1, &[&[]]);
    exhaustive_unique_vecs_fixed_length_small_helper(
        1,
        exhaustive_unsigneds::<u8>(),
        256,
        &[
            &[0],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[7],
            &[8],
            &[9],
            &[10],
            &[11],
            &[12],
            &[13],
            &[14],
            &[15],
            &[16],
            &[17],
            &[18],
            &[19],
        ],
    );
    exhaustive_unique_vecs_fixed_length_helper(
        1,
        exhaustive_unsigneds::<u64>(),
        &[
            &[0],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[7],
            &[8],
            &[9],
            &[10],
            &[11],
            &[12],
            &[13],
            &[14],
            &[15],
            &[16],
            &[17],
            &[18],
            &[19],
        ],
    );
    exhaustive_unique_vecs_fixed_length_small_helper(
        2,
        exhaustive_unsigneds::<u8>(),
        65280,
        &[
            &[0, 1],
            &[1, 0],
            &[0, 2],
            &[2, 0],
            &[1, 2],
            &[2, 1],
            &[0, 3],
            &[3, 0],
            &[1, 3],
            &[3, 1],
            &[2, 3],
            &[3, 2],
            &[0, 4],
            &[4, 0],
            &[1, 4],
            &[4, 1],
            &[2, 4],
            &[4, 2],
            &[3, 4],
            &[4, 3],
        ],
    );
    exhaustive_unique_vecs_fixed_length_helper(
        3,
        exhaustive_unsigneds::<u8>(),
        &[
            &[0, 1, 2],
            &[0, 1, 3],
            &[0, 2, 1],
            &[0, 2, 3],
            &[1, 0, 2],
            &[0, 3, 1],
            &[1, 2, 0],
            &[1, 2, 3],
            &[2, 0, 1],
            &[1, 0, 3],
            &[2, 1, 0],
            &[0, 3, 2],
            &[1, 3, 0],
            &[2, 0, 3],
            &[3, 0, 1],
            &[0, 2, 4],
            &[3, 1, 0],
            &[2, 3, 0],
            &[3, 0, 2],
            &[0, 1, 4],
        ],
    );
    exhaustive_unique_vecs_fixed_length_small_helper(
        2,
        exhaustive_ascii_chars(),
        16256,
        &[
            &['a', 'b'],
            &['b', 'a'],
            &['a', 'c'],
            &['c', 'a'],
            &['b', 'c'],
            &['c', 'b'],
            &['a', 'd'],
            &['d', 'a'],
            &['b', 'd'],
            &['d', 'b'],
            &['c', 'd'],
            &['d', 'c'],
            &['a', 'e'],
            &['e', 'a'],
            &['b', 'e'],
            &['e', 'b'],
            &['c', 'e'],
            &['e', 'c'],
            &['d', 'e'],
            &['e', 'd'],
        ],
    );
    exhaustive_unique_vecs_fixed_length_small_helper(
        1,
        exhaustive_bools(),
        2,
        &[&[false], &[true]],
    );
    exhaustive_unique_vecs_fixed_length_small_helper(
        2,
        exhaustive_bools(),
        2,
        &[&[false, true], &[true, false]],
    );
    exhaustive_unique_vecs_fixed_length_small_helper(4, exhaustive_bools(), 0, &[]);
    exhaustive_unique_vecs_fixed_length_small_helper(
        4,
        1..=6,
        360,
        &[
            &[1, 2, 3, 4],
            &[1, 2, 3, 5],
            &[1, 2, 4, 3],
            &[1, 2, 4, 5],
            &[1, 3, 2, 4],
            &[1, 2, 5, 3],
            &[1, 3, 4, 2],
            &[1, 3, 4, 5],
            &[1, 4, 2, 3],
            &[1, 3, 2, 5],
            &[1, 4, 3, 2],
            &[1, 2, 5, 4],
            &[2, 1, 3, 4],
            &[1, 3, 5, 2],
            &[2, 1, 4, 3],
            &[2, 3, 4, 5],
            &[2, 3, 1, 4],
            &[1, 5, 2, 3],
            &[2, 3, 4, 1],
            &[1, 4, 2, 5],
        ],
    );
    exhaustive_unique_vecs_fixed_length_helper(
        2,
        exhaustive_unique_vecs_fixed_length(2, exhaustive_unsigneds::<u8>()),
        &[
            &[vec![0, 1], vec![1, 0]],
            &[vec![1, 0], vec![0, 1]],
            &[vec![0, 1], vec![0, 2]],
            &[vec![0, 2], vec![0, 1]],
            &[vec![1, 0], vec![0, 2]],
            &[vec![0, 2], vec![1, 0]],
            &[vec![0, 1], vec![2, 0]],
            &[vec![2, 0], vec![0, 1]],
            &[vec![1, 0], vec![2, 0]],
            &[vec![2, 0], vec![1, 0]],
            &[vec![0, 2], vec![2, 0]],
            &[vec![2, 0], vec![0, 2]],
            &[vec![0, 1], vec![1, 2]],
            &[vec![1, 2], vec![0, 1]],
            &[vec![1, 0], vec![1, 2]],
            &[vec![1, 2], vec![1, 0]],
            &[vec![0, 2], vec![1, 2]],
            &[vec![1, 2], vec![0, 2]],
            &[vec![2, 0], vec![1, 2]],
            &[vec![1, 2], vec![2, 0]],
        ],
    );
}
