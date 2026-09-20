// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::{
    shortlex_ordered_vecs, shortlex_ordered_vecs_length_inclusive_range,
};
use std::fmt::Debug;

fn shortlex_ordered_vecs_helper<I: Clone + Iterator>(xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(shortlex_ordered_vecs(xs), out);
}

fn shortlex_ordered_vecs_small_helper<I: Clone + Iterator>(
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(shortlex_ordered_vecs(xs), out_len, out);
}

#[test]
fn test_shortlex_ordered_vecs() {
    shortlex_ordered_vecs_small_helper(nevers(), 1, &[&[]]);
    shortlex_ordered_vecs_helper(
        exhaustive_units(),
        &[
            &[],
            &[()],
            &[(), ()],
            &[(), (), ()],
            &[(), (), (), ()],
            &[(), (), (), (), ()],
            &[(), (), (), (), (), ()],
            &[(), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            &[(), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), (), ()],
        ],
    );
    shortlex_ordered_vecs_helper(
        exhaustive_bools(),
        &[
            &[],
            &[false],
            &[true],
            &[false, false],
            &[false, true],
            &[true, true],
            &[false, false, false],
            &[false, false, true],
            &[false, true, true],
            &[true, true, true],
            &[false, false, false, false],
            &[false, false, false, true],
            &[false, false, true, true],
            &[false, true, true, true],
            &[true, true, true, true],
            &[false, false, false, false, false],
            &[false, false, false, false, true],
            &[false, false, false, true, true],
            &[false, false, true, true, true],
            &[false, true, true, true, true],
        ],
    );
    shortlex_ordered_vecs_helper(
        'a'..='c',
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'a'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'b'],
            &['b', 'c'],
            &['c', 'c'],
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'a', 'c'],
            &['a', 'b', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'c'],
            &['b', 'b', 'b'],
            &['b', 'b', 'c'],
            &['b', 'c', 'c'],
            &['c', 'c', 'c'],
        ],
    );
    shortlex_ordered_vecs_helper(
        1..=3,
        &[
            &[],
            &[1],
            &[2],
            &[3],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[2, 2],
            &[2, 3],
            &[3, 3],
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 1, 3],
            &[1, 2, 2],
            &[1, 2, 3],
            &[1, 3, 3],
            &[2, 2, 2],
            &[2, 2, 3],
            &[2, 3, 3],
            &[3, 3, 3],
        ],
    );
    shortlex_ordered_vecs_helper(
        1..=6,
        &[
            &[],
            &[1],
            &[2],
            &[3],
            &[4],
            &[5],
            &[6],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[1, 4],
            &[1, 5],
            &[1, 6],
            &[2, 2],
            &[2, 3],
            &[2, 4],
            &[2, 5],
            &[2, 6],
            &[3, 3],
            &[3, 4],
        ],
    );
    shortlex_ordered_vecs_helper(
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['d'],
            &['e'],
            &['f'],
            &['g'],
            &['h'],
            &['i'],
            &['j'],
            &['k'],
            &['l'],
            &['m'],
            &['n'],
            &['o'],
            &['p'],
            &['q'],
            &['r'],
            &['s'],
        ],
    );
}

// The unbounded generator begins with everything the bounded one produces, in the same order.
#[test]
fn shortlex_ordered_vecs_extends_bounded() {
    let expected = shortlex_ordered_vecs_length_inclusive_range(0, 3, 0..4u8).collect_vec();
    assert_eq!(
        shortlex_ordered_vecs(0..4u8)
            .take(expected.len())
            .collect_vec(),
        expected
    );
}
