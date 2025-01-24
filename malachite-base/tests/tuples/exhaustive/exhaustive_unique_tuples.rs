// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{
    exhaustive_unique_quadruples, exhaustive_unique_quintuples, exhaustive_unique_triples,
};
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::{exhaustive_unique_pairs, exhaustive_units};
use std::fmt::Debug;

macro_rules! helpers {
    ($t: ty, $ts: ident, $ts_helper: ident, $ts_small_helper: ident) => {
        fn $ts_helper<I: Iterator>(xs: I, out: &[$t])
        where
            I::Item: Clone + Debug + Eq,
        {
            let ts = $ts(xs).take(20).collect_vec();
            assert_eq!(ts.as_slice(), out);
        }

        fn $ts_small_helper<I: Clone + Iterator>(xs: I, out_len: usize, out: &[$t])
        where
            I::Item: Clone + Debug + Eq,
        {
            let ts = $ts(xs);
            let ts_prefix = ts.clone().take(20).collect_vec();
            assert_eq!(ts_prefix.as_slice(), out);
            assert_eq!(ts.count(), out_len);
        }
    };
}
helpers!(
    (I::Item, I::Item),
    exhaustive_unique_pairs,
    _exhaustive_unique_pairs_helper,
    exhaustive_unique_pairs_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item),
    exhaustive_unique_triples,
    exhaustive_unique_triples_helper,
    _exhaustive_unique_triples_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item),
    exhaustive_unique_quadruples,
    _exhaustive_unique_quadruples_helper,
    exhaustive_unique_quadruples_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    exhaustive_unique_quintuples,
    _exhaustive_unique_quintuples_helper,
    exhaustive_unique_quintuples_small_helper
);

#[test]
fn test_exhaustive_unique_tuples() {
    exhaustive_unique_pairs_small_helper(nevers(), 0, &[]);
    exhaustive_unique_quintuples_small_helper(nevers(), 0, &[]);
    exhaustive_unique_pairs_small_helper(exhaustive_units(), 0, &[]);
    exhaustive_unique_quintuples_small_helper(exhaustive_units(), 0, &[]);
    exhaustive_unique_pairs_small_helper(
        exhaustive_unsigneds::<u8>(),
        65280,
        &[
            (0, 1),
            (1, 0),
            (0, 2),
            (2, 0),
            (1, 2),
            (2, 1),
            (0, 3),
            (3, 0),
            (1, 3),
            (3, 1),
            (2, 3),
            (3, 2),
            (0, 4),
            (4, 0),
            (1, 4),
            (4, 1),
            (2, 4),
            (4, 2),
            (3, 4),
            (4, 3),
        ],
    );
    exhaustive_unique_triples_helper(
        exhaustive_unsigneds::<u8>(),
        &[
            (0, 1, 2),
            (0, 1, 3),
            (0, 2, 1),
            (0, 2, 3),
            (1, 0, 2),
            (0, 3, 1),
            (1, 2, 0),
            (1, 2, 3),
            (2, 0, 1),
            (1, 0, 3),
            (2, 1, 0),
            (0, 3, 2),
            (1, 3, 0),
            (2, 0, 3),
            (3, 0, 1),
            (0, 2, 4),
            (3, 1, 0),
            (2, 3, 0),
            (3, 0, 2),
            (0, 1, 4),
        ],
    );
    exhaustive_unique_pairs_small_helper(
        exhaustive_ascii_chars(),
        16256,
        &[
            ('a', 'b'),
            ('b', 'a'),
            ('a', 'c'),
            ('c', 'a'),
            ('b', 'c'),
            ('c', 'b'),
            ('a', 'd'),
            ('d', 'a'),
            ('b', 'd'),
            ('d', 'b'),
            ('c', 'd'),
            ('d', 'c'),
            ('a', 'e'),
            ('e', 'a'),
            ('b', 'e'),
            ('e', 'b'),
            ('c', 'e'),
            ('e', 'c'),
            ('d', 'e'),
            ('e', 'd'),
        ],
    );
    exhaustive_unique_pairs_small_helper(exhaustive_bools(), 2, &[(false, true), (true, false)]);
    exhaustive_unique_quadruples_small_helper(
        1..=6,
        360,
        &[
            (1, 2, 3, 4),
            (1, 2, 3, 5),
            (1, 2, 4, 3),
            (1, 2, 4, 5),
            (1, 3, 2, 4),
            (1, 2, 5, 3),
            (1, 3, 4, 2),
            (1, 3, 4, 5),
            (1, 4, 2, 3),
            (1, 3, 2, 5),
            (1, 4, 3, 2),
            (1, 2, 5, 4),
            (2, 1, 3, 4),
            (1, 3, 5, 2),
            (2, 1, 4, 3),
            (2, 3, 4, 5),
            (2, 3, 1, 4),
            (1, 5, 2, 3),
            (2, 3, 4, 1),
            (1, 4, 2, 5),
        ],
    );
}
