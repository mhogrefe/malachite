// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::exhaustive_ordered_vecs;
use std::fmt::Debug;

fn exhaustive_ordered_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(exhaustive_ordered_vecs(xs), out);
}

fn exhaustive_ordered_vecs_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(exhaustive_ordered_vecs(xs), out_len, out);
}

#[test]
fn test_exhaustive_ordered_vecs() {
    exhaustive_ordered_vecs_helper(
        1..=3,
        &[
            &[],
            &[1],
            &[2],
            &[1, 1, 1],
            &[3],
            &[1, 1],
            &[2, 2],
            &[1, 1, 1, 1, 1],
            &[1, 2],
            &[2, 2, 2],
            &[1, 3],
            &[1, 1, 1, 1],
            &[3, 3],
            &[1, 2, 2],
            &[2, 3],
            &[1, 1, 1, 1, 1, 1],
            &[1, 3, 3],
            &[2, 2, 2, 2],
            &[1, 1, 2],
            &[2, 2, 2, 2, 2],
        ],
    );
    exhaustive_ordered_vecs_small_helper(nevers(), 1, &[&[]]);
    exhaustive_ordered_vecs_helper(
        exhaustive_units(),
        &[
            &[],
            &[()],
            &[(), ()],
            &[(), (), (), ()],
            &[(), (), ()],
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
        ],
    );
    exhaustive_ordered_vecs_helper(
        exhaustive_bools(),
        &[
            &[],
            &[false],
            &[true],
            &[false, false, false],
            &[false, false],
            &[true, true, true],
            &[true, true],
            &[false, false, false, false, false],
            &[false, true],
            &[false, true, true],
            &[false, false, true],
            &[true, true, true, true, true],
            &[false, false, false, false],
            &[false, true, true, true, true],
            &[true, true, true, true],
            &[false, false, false, false, false, false, false, false],
            &[false, true, true, true],
            &[false, false, true, true, true],
            &[false, false, true, true],
            &[false, false, false, false, false, false],
        ],
    );
    exhaustive_ordered_vecs_helper(
        'a'..='c',
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'a', 'a'],
            &['c'],
            &['a', 'a'],
            &['b', 'b'],
            &['a', 'a', 'a', 'a', 'a'],
            &['a', 'b'],
            &['b', 'b', 'b'],
            &['a', 'c'],
            &['a', 'a', 'a', 'a'],
            &['c', 'c'],
            &['a', 'b', 'b'],
            &['b', 'c'],
            &['a', 'a', 'a', 'a', 'a', 'a'],
            &['a', 'c', 'c'],
            &['b', 'b', 'b', 'b'],
            &['a', 'a', 'b'],
            &['b', 'b', 'b', 'b', 'b'],
        ],
    );
    exhaustive_ordered_vecs_helper(
        1..=6,
        &[
            &[],
            &[1],
            &[2],
            &[1, 1, 1],
            &[3],
            &[1, 1],
            &[5],
            &[1, 1, 1, 1],
            &[4],
            &[2, 2],
            &[6],
            &[2, 2, 2],
            &[1, 2],
            &[1, 2, 2],
            &[1, 3],
            &[1, 1, 1, 1, 1, 1],
            &[3, 3],
            &[1, 3, 3],
            &[2, 3],
            &[2, 2, 2, 2],
        ],
    );
    exhaustive_ordered_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['a', 'a', 'a'],
            &['c'],
            &['a', 'a'],
            &['e'],
            &['a', 'a', 'a', 'a'],
            &['d'],
            &['b', 'b'],
            &['f'],
            &['b', 'b', 'b'],
            &['g'],
            &['a', 'b'],
            &['j'],
            &['a', 'a', 'a', 'a', 'a'],
            &['h'],
            &['a', 'c'],
            &['i'],
            &['a', 'b', 'b'],
        ],
    );
}

#[test]
fn exhaustive_ordered_vecs_are_in_source_order() {
    for xs in exhaustive_ordered_vecs(0..20u8).take(5000) {
        assert!(xs.windows(2).all(|w| w[0] <= w[1]));
    }
}

// The lengths grow logarithmically: over a source of 3 elements, the first 5000 `Vec`s have at most
// 39 elements, where a subset-first design reached thousands.
#[test]
fn exhaustive_ordered_vecs_lengths_grow_slowly() {
    let max_len = exhaustive_ordered_vecs(1..=3u8)
        .take(5000)
        .map(|xs| xs.len())
        .max()
        .unwrap();
    assert!(max_len <= 40, "{max_len}");
}
