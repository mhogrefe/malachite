// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::{
    exhaustive_triples_from_single, exhaustive_triples_xxy, exhaustive_triples_xxy_custom_output,
    exhaustive_triples_xyx, exhaustive_triples_xyx_custom_output,
};
use crate::get_sample_output_types;
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::tuples::exhaustive::exhaustive_pairs;
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::iter::once;

#[allow(clippy::needless_pass_by_value)]
fn exhaustive_triples_xxy_helper<
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
    let ts = exhaustive_triples_xxy(xs.clone(), ys.clone());
    assert_eq!(ts.clone().take(20).collect_vec(), out);
    assert_eq!(ts.count(), out_len);

    let output_types = get_sample_output_types(3);
    let ts = exhaustive_triples_xxy_custom_output(
        xs.clone(),
        ys.clone(),
        output_types[0][0],
        output_types[0][1],
        output_types[0][2],
    );
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
    for alt_output_types in &output_types[1..] {
        let ts = exhaustive_triples_xxy_custom_output(
            xs.clone(),
            ys.clone(),
            alt_output_types[0],
            alt_output_types[1],
            alt_output_types[2],
        );
        ts.clone().take(20).for_each(drop);
        assert_eq!(ts.count(), out_len);
    }
}

#[test]
fn test_exhaustive_triples_xxy() {
    exhaustive_triples_xxy_helper(nevers(), nevers(), 0, &[]);
    exhaustive_triples_xxy_helper(nevers(), 0..4, 0, &[]);
    exhaustive_triples_xxy_helper(once('a'), once(1), 1, &[('a', 'a', 1)]);
    exhaustive_triples_xxy_helper(
        once('a'),
        0..4,
        4,
        &[('a', 'a', 0), ('a', 'a', 1), ('a', 'a', 2), ('a', 'a', 3)],
    );
    exhaustive_triples_xxy_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1 << 18,
        &[
            (0, 0, 'a'),
            (0, 0, 'b'),
            (0, 1, 'a'),
            (0, 1, 'b'),
            (1, 0, 'a'),
            (1, 0, 'b'),
            (1, 1, 'a'),
            (1, 1, 'b'),
            (0, 0, 'c'),
            (0, 0, 'd'),
            (0, 1, 'c'),
            (0, 1, 'd'),
            (1, 0, 'c'),
            (1, 0, 'd'),
            (1, 1, 'c'),
            (1, 1, 'd'),
            (0, 2, 'a'),
            (0, 2, 'b'),
            (0, 3, 'a'),
            (0, 3, 'b'),
        ],
    );
    exhaustive_triples_xxy_helper(
        exhaustive_bools(),
        0..4,
        16,
        &[
            (false, false, 0),
            (false, false, 1),
            (false, true, 0),
            (false, true, 1),
            (true, false, 0),
            (true, false, 1),
            (true, true, 0),
            (true, true, 1),
            (false, false, 2),
            (false, false, 3),
            (false, true, 2),
            (false, true, 3),
            (true, false, 2),
            (true, false, 3),
            (true, true, 2),
            (true, true, 3),
        ],
    );
    exhaustive_triples_xxy_helper(
        'a'..'f',
        0..3,
        75,
        &[
            ('a', 'a', 0),
            ('a', 'a', 1),
            ('a', 'b', 0),
            ('a', 'b', 1),
            ('b', 'a', 0),
            ('b', 'a', 1),
            ('b', 'b', 0),
            ('b', 'b', 1),
            ('a', 'a', 2),
            ('a', 'b', 2),
            ('b', 'a', 2),
            ('b', 'b', 2),
            ('a', 'c', 0),
            ('a', 'c', 1),
            ('a', 'd', 0),
            ('a', 'd', 1),
            ('b', 'c', 0),
            ('b', 'c', 1),
            ('b', 'd', 0),
            ('b', 'd', 1),
        ],
    );
    exhaustive_triples_xxy_helper(
        ['a', 'b', 'c'].iter().copied(),
        exhaustive_orderings(),
        27,
        &[
            ('a', 'a', Equal),
            ('a', 'a', Less),
            ('a', 'b', Equal),
            ('a', 'b', Less),
            ('b', 'a', Equal),
            ('b', 'a', Less),
            ('b', 'b', Equal),
            ('b', 'b', Less),
            ('a', 'a', Greater),
            ('a', 'b', Greater),
            ('b', 'a', Greater),
            ('b', 'b', Greater),
            ('a', 'c', Equal),
            ('a', 'c', Less),
            ('b', 'c', Equal),
            ('b', 'c', Less),
            ('a', 'c', Greater),
            ('b', 'c', Greater),
            ('c', 'a', Equal),
            ('c', 'a', Less),
        ],
    );
    exhaustive_triples_xxy_helper(
        exhaustive_pairs(exhaustive_orderings(), exhaustive_bools()),
        exhaustive_triples_from_single([Less, Greater].iter().copied()),
        288,
        &[
            ((Equal, false), (Equal, false), (Less, Less, Less)),
            ((Equal, false), (Equal, false), (Less, Less, Greater)),
            ((Equal, false), (Equal, true), (Less, Less, Less)),
            ((Equal, false), (Equal, true), (Less, Less, Greater)),
            ((Equal, true), (Equal, false), (Less, Less, Less)),
            ((Equal, true), (Equal, false), (Less, Less, Greater)),
            ((Equal, true), (Equal, true), (Less, Less, Less)),
            ((Equal, true), (Equal, true), (Less, Less, Greater)),
            ((Equal, false), (Equal, false), (Less, Greater, Less)),
            ((Equal, false), (Equal, false), (Less, Greater, Greater)),
            ((Equal, false), (Equal, true), (Less, Greater, Less)),
            ((Equal, false), (Equal, true), (Less, Greater, Greater)),
            ((Equal, true), (Equal, false), (Less, Greater, Less)),
            ((Equal, true), (Equal, false), (Less, Greater, Greater)),
            ((Equal, true), (Equal, true), (Less, Greater, Less)),
            ((Equal, true), (Equal, true), (Less, Greater, Greater)),
            ((Equal, false), (Less, false), (Less, Less, Less)),
            ((Equal, false), (Less, false), (Less, Less, Greater)),
            ((Equal, false), (Less, true), (Less, Less, Less)),
            ((Equal, false), (Less, true), (Less, Less, Greater)),
        ],
    );
}

#[allow(clippy::needless_pass_by_value)]
fn exhaustive_triples_xyx_helper<
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
    let ts = exhaustive_triples_xyx(xs.clone(), ys.clone());
    assert_eq!(ts.clone().take(20).collect_vec(), out);
    assert_eq!(ts.count(), out_len);

    let output_types = get_sample_output_types(3);
    let ts = exhaustive_triples_xyx_custom_output(
        xs.clone(),
        ys.clone(),
        output_types[0][0],
        output_types[0][1],
        output_types[0][2],
    );
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
    for alt_output_types in &output_types[1..] {
        let ts = exhaustive_triples_xyx_custom_output(
            xs.clone(),
            ys.clone(),
            alt_output_types[0],
            alt_output_types[1],
            alt_output_types[2],
        );
        ts.clone().take(20).for_each(drop);
        assert_eq!(ts.count(), out_len);
    }
}

#[test]
fn test_exhaustive_triples_xyx() {
    exhaustive_triples_xyx_helper(nevers(), nevers(), 0, &[]);
    exhaustive_triples_xyx_helper(nevers(), 0..4, 0, &[]);
    exhaustive_triples_xyx_helper(once('a'), once(1), 1, &[('a', 1, 'a')]);
    exhaustive_triples_xyx_helper(
        once('a'),
        0..4,
        4,
        &[('a', 0, 'a'), ('a', 1, 'a'), ('a', 2, 'a'), ('a', 3, 'a')],
    );
    exhaustive_triples_xyx_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1 << 18,
        &[
            (0, 'a', 0),
            (0, 'a', 1),
            (0, 'b', 0),
            (0, 'b', 1),
            (1, 'a', 0),
            (1, 'a', 1),
            (1, 'b', 0),
            (1, 'b', 1),
            (0, 'a', 2),
            (0, 'a', 3),
            (0, 'b', 2),
            (0, 'b', 3),
            (1, 'a', 2),
            (1, 'a', 3),
            (1, 'b', 2),
            (1, 'b', 3),
            (0, 'c', 0),
            (0, 'c', 1),
            (0, 'd', 0),
            (0, 'd', 1),
        ],
    );
    exhaustive_triples_xyx_helper(
        exhaustive_bools(),
        0..4,
        16,
        &[
            (false, 0, false),
            (false, 0, true),
            (false, 1, false),
            (false, 1, true),
            (true, 0, false),
            (true, 0, true),
            (true, 1, false),
            (true, 1, true),
            (false, 2, false),
            (false, 2, true),
            (false, 3, false),
            (false, 3, true),
            (true, 2, false),
            (true, 2, true),
            (true, 3, false),
            (true, 3, true),
        ],
    );
    exhaustive_triples_xyx_helper(
        'a'..'f',
        0..3,
        75,
        &[
            ('a', 0, 'a'),
            ('a', 0, 'b'),
            ('a', 1, 'a'),
            ('a', 1, 'b'),
            ('b', 0, 'a'),
            ('b', 0, 'b'),
            ('b', 1, 'a'),
            ('b', 1, 'b'),
            ('a', 0, 'c'),
            ('a', 0, 'd'),
            ('a', 1, 'c'),
            ('a', 1, 'd'),
            ('b', 0, 'c'),
            ('b', 0, 'd'),
            ('b', 1, 'c'),
            ('b', 1, 'd'),
            ('a', 2, 'a'),
            ('a', 2, 'b'),
            ('b', 2, 'a'),
            ('b', 2, 'b'),
        ],
    );
    exhaustive_triples_xyx_helper(
        ['a', 'b', 'c'].iter().copied(),
        exhaustive_orderings(),
        27,
        &[
            ('a', Equal, 'a'),
            ('a', Equal, 'b'),
            ('a', Less, 'a'),
            ('a', Less, 'b'),
            ('b', Equal, 'a'),
            ('b', Equal, 'b'),
            ('b', Less, 'a'),
            ('b', Less, 'b'),
            ('a', Equal, 'c'),
            ('a', Less, 'c'),
            ('b', Equal, 'c'),
            ('b', Less, 'c'),
            ('a', Greater, 'a'),
            ('a', Greater, 'b'),
            ('b', Greater, 'a'),
            ('b', Greater, 'b'),
            ('a', Greater, 'c'),
            ('b', Greater, 'c'),
            ('c', Equal, 'a'),
            ('c', Equal, 'b'),
        ],
    );
    exhaustive_triples_xyx_helper(
        exhaustive_pairs(exhaustive_orderings(), exhaustive_bools()),
        exhaustive_triples_from_single([Less, Greater].iter().copied()),
        288,
        &[
            ((Equal, false), (Less, Less, Less), (Equal, false)),
            ((Equal, false), (Less, Less, Less), (Equal, true)),
            ((Equal, false), (Less, Less, Greater), (Equal, false)),
            ((Equal, false), (Less, Less, Greater), (Equal, true)),
            ((Equal, true), (Less, Less, Less), (Equal, false)),
            ((Equal, true), (Less, Less, Less), (Equal, true)),
            ((Equal, true), (Less, Less, Greater), (Equal, false)),
            ((Equal, true), (Less, Less, Greater), (Equal, true)),
            ((Equal, false), (Less, Less, Less), (Less, false)),
            ((Equal, false), (Less, Less, Less), (Less, true)),
            ((Equal, false), (Less, Less, Greater), (Less, false)),
            ((Equal, false), (Less, Less, Greater), (Less, true)),
            ((Equal, true), (Less, Less, Less), (Less, false)),
            ((Equal, true), (Less, Less, Less), (Less, true)),
            ((Equal, true), (Less, Less, Greater), (Less, false)),
            ((Equal, true), (Less, Less, Greater), (Less, true)),
            ((Equal, false), (Less, Greater, Less), (Equal, false)),
            ((Equal, false), (Less, Greater, Less), (Equal, true)),
            ((Equal, false), (Less, Greater, Greater), (Equal, false)),
            ((Equal, false), (Less, Greater, Greater), (Equal, true)),
        ],
    );
}
