// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{
    exhaustive_ordered_unique_quadruples, exhaustive_ordered_unique_quintuples,
    exhaustive_ordered_unique_triples,
};
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::{exhaustive_ordered_unique_pairs, exhaustive_units};
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
    exhaustive_ordered_unique_pairs,
    _exhaustive_ordered_unique_pairs_helper,
    exhaustive_ordered_unique_pairs_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item),
    exhaustive_ordered_unique_triples,
    exhaustive_ordered_unique_triples_helper,
    _exhaustive_ordered_unique_triples_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item),
    exhaustive_ordered_unique_quadruples,
    _exhaustive_ordered_unique_quadruples_helper,
    exhaustive_ordered_unique_quadruples_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    exhaustive_ordered_unique_quintuples,
    _exhaustive_ordered_unique_quintuples_helper,
    exhaustive_ordered_unique_quintuples_small_helper
);

#[test]
fn test_exhaustive_ordered_unique_tuples() {
    exhaustive_ordered_unique_pairs_small_helper(nevers(), 0, &[]);
    exhaustive_ordered_unique_quintuples_small_helper(nevers(), 0, &[]);
    exhaustive_ordered_unique_pairs_small_helper(exhaustive_units(), 0, &[]);
    exhaustive_ordered_unique_quintuples_small_helper(exhaustive_units(), 0, &[]);
    exhaustive_ordered_unique_pairs_small_helper(
        exhaustive_unsigneds::<u8>(),
        32640,
        &[
            (0, 1),
            (0, 2),
            (1, 2),
            (0, 3),
            (1, 3),
            (2, 3),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
            (0, 5),
            (1, 5),
            (2, 5),
            (3, 5),
            (4, 5),
            (0, 6),
            (1, 6),
            (2, 6),
            (3, 6),
            (4, 6),
        ],
    );
    exhaustive_ordered_unique_triples_helper(
        exhaustive_unsigneds::<u8>(),
        &[
            (0, 1, 2),
            (0, 1, 3),
            (0, 2, 3),
            (1, 2, 3),
            (0, 1, 4),
            (0, 2, 4),
            (1, 2, 4),
            (0, 3, 4),
            (1, 3, 4),
            (2, 3, 4),
            (0, 1, 5),
            (0, 2, 5),
            (1, 2, 5),
            (0, 3, 5),
            (1, 3, 5),
            (2, 3, 5),
            (0, 4, 5),
            (1, 4, 5),
            (2, 4, 5),
            (3, 4, 5),
        ],
    );
    exhaustive_ordered_unique_pairs_small_helper(
        exhaustive_ascii_chars(),
        8128,
        &[
            ('a', 'b'),
            ('a', 'c'),
            ('b', 'c'),
            ('a', 'd'),
            ('b', 'd'),
            ('c', 'd'),
            ('a', 'e'),
            ('b', 'e'),
            ('c', 'e'),
            ('d', 'e'),
            ('a', 'f'),
            ('b', 'f'),
            ('c', 'f'),
            ('d', 'f'),
            ('e', 'f'),
            ('a', 'g'),
            ('b', 'g'),
            ('c', 'g'),
            ('d', 'g'),
            ('e', 'g'),
        ],
    );
    exhaustive_ordered_unique_pairs_small_helper(exhaustive_bools(), 1, &[(false, true)]);
    exhaustive_ordered_unique_quadruples_small_helper(
        1..=6,
        15,
        &[
            (1, 2, 3, 4),
            (1, 2, 3, 5),
            (1, 2, 4, 5),
            (1, 3, 4, 5),
            (2, 3, 4, 5),
            (1, 2, 3, 6),
            (1, 2, 4, 6),
            (1, 3, 4, 6),
            (2, 3, 4, 6),
            (1, 2, 5, 6),
            (1, 3, 5, 6),
            (2, 3, 5, 6),
            (1, 4, 5, 6),
            (2, 4, 5, 6),
            (3, 4, 5, 6),
        ],
    );
}
