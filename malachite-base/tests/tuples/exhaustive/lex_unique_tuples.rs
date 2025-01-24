// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{lex_unique_quadruples, lex_unique_quintuples, lex_unique_triples};
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::{exhaustive_units, lex_unique_pairs};
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
    lex_unique_pairs,
    _lex_unique_pairs_helper,
    lex_unique_pairs_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item),
    lex_unique_triples,
    lex_unique_triples_helper,
    _lex_unique_triples_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item),
    lex_unique_quadruples,
    _lex_unique_quadruples_helper,
    lex_unique_quadruples_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    lex_unique_quintuples,
    _lex_unique_quintuples_helper,
    lex_unique_quintuples_small_helper
);

#[test]
fn test_lex_unique_tuples() {
    lex_unique_pairs_small_helper(nevers(), 0, &[]);
    lex_unique_quintuples_small_helper(nevers(), 0, &[]);
    lex_unique_pairs_small_helper(exhaustive_units(), 0, &[]);
    lex_unique_quintuples_small_helper(exhaustive_units(), 0, &[]);
    lex_unique_pairs_small_helper(
        exhaustive_unsigneds::<u8>(),
        65280,
        &[
            (0, 1),
            (0, 2),
            (0, 3),
            (0, 4),
            (0, 5),
            (0, 6),
            (0, 7),
            (0, 8),
            (0, 9),
            (0, 10),
            (0, 11),
            (0, 12),
            (0, 13),
            (0, 14),
            (0, 15),
            (0, 16),
            (0, 17),
            (0, 18),
            (0, 19),
            (0, 20),
        ],
    );
    lex_unique_triples_helper(
        exhaustive_unsigneds::<u8>(),
        &[
            (0, 1, 2),
            (0, 1, 3),
            (0, 1, 4),
            (0, 1, 5),
            (0, 1, 6),
            (0, 1, 7),
            (0, 1, 8),
            (0, 1, 9),
            (0, 1, 10),
            (0, 1, 11),
            (0, 1, 12),
            (0, 1, 13),
            (0, 1, 14),
            (0, 1, 15),
            (0, 1, 16),
            (0, 1, 17),
            (0, 1, 18),
            (0, 1, 19),
            (0, 1, 20),
            (0, 1, 21),
        ],
    );
    lex_unique_pairs_small_helper(
        exhaustive_ascii_chars(),
        16256,
        &[
            ('a', 'b'),
            ('a', 'c'),
            ('a', 'd'),
            ('a', 'e'),
            ('a', 'f'),
            ('a', 'g'),
            ('a', 'h'),
            ('a', 'i'),
            ('a', 'j'),
            ('a', 'k'),
            ('a', 'l'),
            ('a', 'm'),
            ('a', 'n'),
            ('a', 'o'),
            ('a', 'p'),
            ('a', 'q'),
            ('a', 'r'),
            ('a', 's'),
            ('a', 't'),
            ('a', 'u'),
        ],
    );
    lex_unique_pairs_small_helper(exhaustive_bools(), 2, &[(false, true), (true, false)]);
    lex_unique_quadruples_small_helper(
        1..=6,
        360,
        &[
            (1, 2, 3, 4),
            (1, 2, 3, 5),
            (1, 2, 3, 6),
            (1, 2, 4, 3),
            (1, 2, 4, 5),
            (1, 2, 4, 6),
            (1, 2, 5, 3),
            (1, 2, 5, 4),
            (1, 2, 5, 6),
            (1, 2, 6, 3),
            (1, 2, 6, 4),
            (1, 2, 6, 5),
            (1, 3, 2, 4),
            (1, 3, 2, 5),
            (1, 3, 2, 6),
            (1, 3, 4, 2),
            (1, 3, 4, 5),
            (1, 3, 4, 6),
            (1, 3, 5, 2),
            (1, 3, 5, 4),
        ],
    );
}
