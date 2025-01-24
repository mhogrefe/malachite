// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{lex_triples_from_single, lex_triples_xxy, lex_triples_xyx};
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::tuples::exhaustive::lex_pairs;
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::iter::once;

fn lex_triples_xxy_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[(X, X, Y)],
) {
    let ts = lex_triples_xxy(xs, ys);
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
}

#[test]
fn test_lex_triples_xxy() {
    lex_triples_xxy_helper(nevers(), nevers(), 0, &[]);
    lex_triples_xxy_helper(nevers(), 0..4, 0, &[]);
    lex_triples_xxy_helper(once('a'), once(1), 1, &[('a', 'a', 1)]);
    lex_triples_xxy_helper(
        once('a'),
        0..4,
        4,
        &[('a', 'a', 0), ('a', 'a', 1), ('a', 'a', 2), ('a', 'a', 3)],
    );
    lex_triples_xxy_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1 << 18,
        &[
            (0, 0, 'a'),
            (0, 0, 'b'),
            (0, 0, 'c'),
            (0, 0, 'd'),
            (0, 1, 'a'),
            (0, 1, 'b'),
            (0, 1, 'c'),
            (0, 1, 'd'),
            (0, 2, 'a'),
            (0, 2, 'b'),
            (0, 2, 'c'),
            (0, 2, 'd'),
            (0, 3, 'a'),
            (0, 3, 'b'),
            (0, 3, 'c'),
            (0, 3, 'd'),
            (0, 4, 'a'),
            (0, 4, 'b'),
            (0, 4, 'c'),
            (0, 4, 'd'),
        ],
    );
    lex_triples_xxy_helper(
        exhaustive_bools(),
        0..4,
        16,
        &[
            (false, false, 0),
            (false, false, 1),
            (false, false, 2),
            (false, false, 3),
            (false, true, 0),
            (false, true, 1),
            (false, true, 2),
            (false, true, 3),
            (true, false, 0),
            (true, false, 1),
            (true, false, 2),
            (true, false, 3),
            (true, true, 0),
            (true, true, 1),
            (true, true, 2),
            (true, true, 3),
        ],
    );
    lex_triples_xxy_helper(
        'a'..'f',
        0..3,
        75,
        &[
            ('a', 'a', 0),
            ('a', 'a', 1),
            ('a', 'a', 2),
            ('a', 'b', 0),
            ('a', 'b', 1),
            ('a', 'b', 2),
            ('a', 'c', 0),
            ('a', 'c', 1),
            ('a', 'c', 2),
            ('a', 'd', 0),
            ('a', 'd', 1),
            ('a', 'd', 2),
            ('a', 'e', 0),
            ('a', 'e', 1),
            ('a', 'e', 2),
            ('b', 'a', 0),
            ('b', 'a', 1),
            ('b', 'a', 2),
            ('b', 'b', 0),
            ('b', 'b', 1),
        ],
    );
    lex_triples_xxy_helper(
        ['a', 'b', 'c'].iter().copied(),
        exhaustive_orderings(),
        27,
        &[
            ('a', 'a', Equal),
            ('a', 'a', Less),
            ('a', 'a', Greater),
            ('a', 'b', Equal),
            ('a', 'b', Less),
            ('a', 'b', Greater),
            ('a', 'c', Equal),
            ('a', 'c', Less),
            ('a', 'c', Greater),
            ('b', 'a', Equal),
            ('b', 'a', Less),
            ('b', 'a', Greater),
            ('b', 'b', Equal),
            ('b', 'b', Less),
            ('b', 'b', Greater),
            ('b', 'c', Equal),
            ('b', 'c', Less),
            ('b', 'c', Greater),
            ('c', 'a', Equal),
            ('c', 'a', Less),
        ],
    );
    lex_triples_xxy_helper(
        lex_pairs(exhaustive_orderings(), exhaustive_bools()),
        lex_triples_from_single([Less, Greater].iter().copied()),
        288,
        &[
            ((Equal, false), (Equal, false), (Less, Less, Less)),
            ((Equal, false), (Equal, false), (Less, Less, Greater)),
            ((Equal, false), (Equal, false), (Less, Greater, Less)),
            ((Equal, false), (Equal, false), (Less, Greater, Greater)),
            ((Equal, false), (Equal, false), (Greater, Less, Less)),
            ((Equal, false), (Equal, false), (Greater, Less, Greater)),
            ((Equal, false), (Equal, false), (Greater, Greater, Less)),
            ((Equal, false), (Equal, false), (Greater, Greater, Greater)),
            ((Equal, false), (Equal, true), (Less, Less, Less)),
            ((Equal, false), (Equal, true), (Less, Less, Greater)),
            ((Equal, false), (Equal, true), (Less, Greater, Less)),
            ((Equal, false), (Equal, true), (Less, Greater, Greater)),
            ((Equal, false), (Equal, true), (Greater, Less, Less)),
            ((Equal, false), (Equal, true), (Greater, Less, Greater)),
            ((Equal, false), (Equal, true), (Greater, Greater, Less)),
            ((Equal, false), (Equal, true), (Greater, Greater, Greater)),
            ((Equal, false), (Less, false), (Less, Less, Less)),
            ((Equal, false), (Less, false), (Less, Less, Greater)),
            ((Equal, false), (Less, false), (Less, Greater, Less)),
            ((Equal, false), (Less, false), (Less, Greater, Greater)),
        ],
    );
}

fn lex_triples_xyx_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[(X, Y, X)],
) {
    let ts = lex_triples_xyx(xs, ys);
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
}

#[test]
fn test_lex_triples_xyx() {
    lex_triples_xyx_helper(nevers(), nevers(), 0, &[]);
    lex_triples_xyx_helper(nevers(), 0..4, 0, &[]);
    lex_triples_xyx_helper(once('a'), once(1), 1, &[('a', 1, 'a')]);
    lex_triples_xyx_helper(
        once('a'),
        0..4,
        4,
        &[('a', 0, 'a'), ('a', 1, 'a'), ('a', 2, 'a'), ('a', 3, 'a')],
    );
    lex_triples_xyx_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1 << 18,
        &[
            (0, 'a', 0),
            (0, 'a', 1),
            (0, 'a', 2),
            (0, 'a', 3),
            (0, 'a', 4),
            (0, 'a', 5),
            (0, 'a', 6),
            (0, 'a', 7),
            (0, 'a', 8),
            (0, 'a', 9),
            (0, 'a', 10),
            (0, 'a', 11),
            (0, 'a', 12),
            (0, 'a', 13),
            (0, 'a', 14),
            (0, 'a', 15),
            (0, 'a', 16),
            (0, 'a', 17),
            (0, 'a', 18),
            (0, 'a', 19),
        ],
    );
    lex_triples_xyx_helper(
        exhaustive_bools(),
        0..4,
        16,
        &[
            (false, 0, false),
            (false, 0, true),
            (false, 1, false),
            (false, 1, true),
            (false, 2, false),
            (false, 2, true),
            (false, 3, false),
            (false, 3, true),
            (true, 0, false),
            (true, 0, true),
            (true, 1, false),
            (true, 1, true),
            (true, 2, false),
            (true, 2, true),
            (true, 3, false),
            (true, 3, true),
        ],
    );
    lex_triples_xyx_helper(
        'a'..'f',
        0..3,
        75,
        &[
            ('a', 0, 'a'),
            ('a', 0, 'b'),
            ('a', 0, 'c'),
            ('a', 0, 'd'),
            ('a', 0, 'e'),
            ('a', 1, 'a'),
            ('a', 1, 'b'),
            ('a', 1, 'c'),
            ('a', 1, 'd'),
            ('a', 1, 'e'),
            ('a', 2, 'a'),
            ('a', 2, 'b'),
            ('a', 2, 'c'),
            ('a', 2, 'd'),
            ('a', 2, 'e'),
            ('b', 0, 'a'),
            ('b', 0, 'b'),
            ('b', 0, 'c'),
            ('b', 0, 'd'),
            ('b', 0, 'e'),
        ],
    );
    lex_triples_xyx_helper(
        ['a', 'b', 'c'].iter().copied(),
        exhaustive_orderings(),
        27,
        &[
            ('a', Equal, 'a'),
            ('a', Equal, 'b'),
            ('a', Equal, 'c'),
            ('a', Less, 'a'),
            ('a', Less, 'b'),
            ('a', Less, 'c'),
            ('a', Greater, 'a'),
            ('a', Greater, 'b'),
            ('a', Greater, 'c'),
            ('b', Equal, 'a'),
            ('b', Equal, 'b'),
            ('b', Equal, 'c'),
            ('b', Less, 'a'),
            ('b', Less, 'b'),
            ('b', Less, 'c'),
            ('b', Greater, 'a'),
            ('b', Greater, 'b'),
            ('b', Greater, 'c'),
            ('c', Equal, 'a'),
            ('c', Equal, 'b'),
        ],
    );
    lex_triples_xyx_helper(
        lex_pairs(exhaustive_orderings(), exhaustive_bools()),
        lex_triples_from_single([Less, Greater].iter().copied()),
        288,
        &[
            ((Equal, false), (Less, Less, Less), (Equal, false)),
            ((Equal, false), (Less, Less, Less), (Equal, true)),
            ((Equal, false), (Less, Less, Less), (Less, false)),
            ((Equal, false), (Less, Less, Less), (Less, true)),
            ((Equal, false), (Less, Less, Less), (Greater, false)),
            ((Equal, false), (Less, Less, Less), (Greater, true)),
            ((Equal, false), (Less, Less, Greater), (Equal, false)),
            ((Equal, false), (Less, Less, Greater), (Equal, true)),
            ((Equal, false), (Less, Less, Greater), (Less, false)),
            ((Equal, false), (Less, Less, Greater), (Less, true)),
            ((Equal, false), (Less, Less, Greater), (Greater, false)),
            ((Equal, false), (Less, Less, Greater), (Greater, true)),
            ((Equal, false), (Less, Greater, Less), (Equal, false)),
            ((Equal, false), (Less, Greater, Less), (Equal, true)),
            ((Equal, false), (Less, Greater, Less), (Less, false)),
            ((Equal, false), (Less, Greater, Less), (Less, true)),
            ((Equal, false), (Less, Greater, Less), (Greater, false)),
            ((Equal, false), (Less, Greater, Less), (Greater, true)),
            ((Equal, false), (Less, Greater, Greater), (Equal, false)),
            ((Equal, false), (Less, Greater, Greater), (Equal, true)),
        ],
    );
}
