// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{
    lex_octuples_from_single, lex_quadruples_from_single, lex_quintuples_from_single,
    lex_triples_from_single,
};
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::tuples::exhaustive::{exhaustive_units, lex_pairs_from_single};
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
    lex_pairs_from_single,
    lex_pairs_from_single_helper,
    lex_pairs_from_single_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item),
    lex_triples_from_single,
    lex_triples_from_single_helper,
    _lex_triples_from_single_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item),
    lex_quadruples_from_single,
    _lex_quadruples_from_single_helper,
    lex_quadruples_from_single_small_helper
);
helpers!(
    (I::Item, I::Item, I::Item, I::Item, I::Item),
    lex_quintuples_from_single,
    _lex_quintuples_from_single_helper,
    lex_quintuples_from_single_small_helper
);
helpers!(
    (
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item,
        I::Item
    ),
    lex_octuples_from_single,
    _lex_octuples_from_single_helper,
    lex_octuples_from_single_small_helper
);

#[test]
fn test_lex_tuples_from_single() {
    lex_pairs_from_single_small_helper(nevers(), 0, &[]);
    lex_quintuples_from_single_small_helper(nevers(), 0, &[]);
    lex_pairs_from_single_small_helper(exhaustive_units(), 1, &[((), ())]);
    lex_quintuples_from_single_small_helper(exhaustive_units(), 1, &[((), (), (), (), ())]);
    lex_pairs_from_single_helper(
        exhaustive_unsigneds::<u8>(),
        &[
            (0, 0),
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
        ],
    );
    lex_triples_from_single_helper(
        exhaustive_unsigneds::<u8>(),
        &[
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (0, 0, 3),
            (0, 0, 4),
            (0, 0, 5),
            (0, 0, 6),
            (0, 0, 7),
            (0, 0, 8),
            (0, 0, 9),
            (0, 0, 10),
            (0, 0, 11),
            (0, 0, 12),
            (0, 0, 13),
            (0, 0, 14),
            (0, 0, 15),
            (0, 0, 16),
            (0, 0, 17),
            (0, 0, 18),
            (0, 0, 19),
        ],
    );
    lex_pairs_from_single_small_helper(
        exhaustive_ascii_chars(),
        0x4000,
        &[
            ('a', 'a'),
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
        ],
    );
    lex_pairs_from_single_small_helper(
        exhaustive_bools(),
        4,
        &[(false, false), (false, true), (true, false), (true, true)],
    );
    lex_quadruples_from_single_small_helper(
        exhaustive_bools(),
        16,
        &[
            (false, false, false, false),
            (false, false, false, true),
            (false, false, true, false),
            (false, false, true, true),
            (false, true, false, false),
            (false, true, false, true),
            (false, true, true, false),
            (false, true, true, true),
            (true, false, false, false),
            (true, false, false, true),
            (true, false, true, false),
            (true, false, true, true),
            (true, true, false, false),
            (true, true, false, true),
            (true, true, true, false),
            (true, true, true, true),
        ],
    );
    lex_octuples_from_single_small_helper(
        exhaustive_bools(),
        256,
        &[
            (false, false, false, false, false, false, false, false),
            (false, false, false, false, false, false, false, true),
            (false, false, false, false, false, false, true, false),
            (false, false, false, false, false, false, true, true),
            (false, false, false, false, false, true, false, false),
            (false, false, false, false, false, true, false, true),
            (false, false, false, false, false, true, true, false),
            (false, false, false, false, false, true, true, true),
            (false, false, false, false, true, false, false, false),
            (false, false, false, false, true, false, false, true),
            (false, false, false, false, true, false, true, false),
            (false, false, false, false, true, false, true, true),
            (false, false, false, false, true, true, false, false),
            (false, false, false, false, true, true, false, true),
            (false, false, false, false, true, true, true, false),
            (false, false, false, false, true, true, true, true),
            (false, false, false, true, false, false, false, false),
            (false, false, false, true, false, false, false, true),
            (false, false, false, true, false, false, true, false),
            (false, false, false, true, false, false, true, true),
        ],
    );
    lex_octuples_from_single_small_helper(
        0..3,
        6561,
        &[
            (0, 0, 0, 0, 0, 0, 0, 0),
            (0, 0, 0, 0, 0, 0, 0, 1),
            (0, 0, 0, 0, 0, 0, 0, 2),
            (0, 0, 0, 0, 0, 0, 1, 0),
            (0, 0, 0, 0, 0, 0, 1, 1),
            (0, 0, 0, 0, 0, 0, 1, 2),
            (0, 0, 0, 0, 0, 0, 2, 0),
            (0, 0, 0, 0, 0, 0, 2, 1),
            (0, 0, 0, 0, 0, 0, 2, 2),
            (0, 0, 0, 0, 0, 1, 0, 0),
            (0, 0, 0, 0, 0, 1, 0, 1),
            (0, 0, 0, 0, 0, 1, 0, 2),
            (0, 0, 0, 0, 0, 1, 1, 0),
            (0, 0, 0, 0, 0, 1, 1, 1),
            (0, 0, 0, 0, 0, 1, 1, 2),
            (0, 0, 0, 0, 0, 1, 2, 0),
            (0, 0, 0, 0, 0, 1, 2, 1),
            (0, 0, 0, 0, 0, 1, 2, 2),
            (0, 0, 0, 0, 0, 2, 0, 0),
            (0, 0, 0, 0, 0, 2, 0, 1),
        ],
    );
    lex_pairs_from_single_helper(
        lex_pairs_from_single(exhaustive_unsigneds::<u8>()),
        &[
            ((0, 0), (0, 0)),
            ((0, 0), (0, 1)),
            ((0, 0), (0, 2)),
            ((0, 0), (0, 3)),
            ((0, 0), (0, 4)),
            ((0, 0), (0, 5)),
            ((0, 0), (0, 6)),
            ((0, 0), (0, 7)),
            ((0, 0), (0, 8)),
            ((0, 0), (0, 9)),
            ((0, 0), (0, 10)),
            ((0, 0), (0, 11)),
            ((0, 0), (0, 12)),
            ((0, 0), (0, 13)),
            ((0, 0), (0, 14)),
            ((0, 0), (0, 15)),
            ((0, 0), (0, 16)),
            ((0, 0), (0, 17)),
            ((0, 0), (0, 18)),
            ((0, 0), (0, 19)),
        ],
    );
}
