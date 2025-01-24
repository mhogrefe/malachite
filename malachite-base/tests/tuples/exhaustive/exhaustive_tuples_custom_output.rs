// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::exhaustive_triples_from_single;
use crate::get_sample_output_types;
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::options::exhaustive::exhaustive_somes;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::tuples::exhaustive::{
    exhaustive_pairs, exhaustive_pairs_custom_output, exhaustive_pairs_from_single,
    exhaustive_triples, exhaustive_triples_custom_output,
};
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::iter::once;

#[allow(clippy::needless_pass_by_value)]
fn exhaustive_pairs_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[(X, Y)],
) {
    let ps = exhaustive_pairs(xs.clone(), ys.clone());
    assert_eq!(ps.clone().take(20).collect_vec(), out);
    assert_eq!(ps.count(), out_len);

    let output_types = get_sample_output_types(2);
    let ps = exhaustive_pairs_custom_output(
        xs.clone(),
        ys.clone(),
        output_types[0][0],
        output_types[0][1],
    );
    assert_eq!(ps.clone().take(20).collect_vec(), out);
    assert_eq!(ps.count(), out_len);
    for alt_output_types in &output_types[1..] {
        let ps = exhaustive_pairs_custom_output(
            xs.clone(),
            ys.clone(),
            alt_output_types[0],
            alt_output_types[1],
        );
        ps.clone().take(20).for_each(drop);
        assert_eq!(ps.count(), out_len);
    }
}

#[test]
fn test_exhaustive_pairs_and_exhaustive_pairs_custom_output() {
    exhaustive_pairs_helper(nevers(), nevers(), 0, &[]);
    exhaustive_pairs_helper(nevers(), 0..4, 0, &[]);
    exhaustive_pairs_helper(once('a'), once(1), 1, &[('a', 1)]);
    exhaustive_pairs_helper(
        once('a'),
        0..4,
        4,
        &[('a', 0), ('a', 1), ('a', 2), ('a', 3)],
    );
    exhaustive_pairs_helper(
        exhaustive_unsigneds::<u8>(),
        'a'..'e',
        1024,
        &[
            (0, 'a'),
            (0, 'b'),
            (1, 'a'),
            (1, 'b'),
            (0, 'c'),
            (0, 'd'),
            (1, 'c'),
            (1, 'd'),
            (2, 'a'),
            (2, 'b'),
            (3, 'a'),
            (3, 'b'),
            (2, 'c'),
            (2, 'd'),
            (3, 'c'),
            (3, 'd'),
            (4, 'a'),
            (4, 'b'),
            (5, 'a'),
            (5, 'b'),
        ],
    );
    exhaustive_pairs_helper(
        exhaustive_bools(),
        0..4,
        8,
        &[
            (false, 0),
            (false, 1),
            (true, 0),
            (true, 1),
            (false, 2),
            (false, 3),
            (true, 2),
            (true, 3),
        ],
    );
    exhaustive_pairs_helper(
        'a'..'f',
        0..3,
        15,
        &[
            ('a', 0),
            ('a', 1),
            ('b', 0),
            ('b', 1),
            ('a', 2),
            ('b', 2),
            ('c', 0),
            ('c', 1),
            ('d', 0),
            ('d', 1),
            ('c', 2),
            ('d', 2),
            ('e', 0),
            ('e', 1),
            ('e', 2),
        ],
    );
    exhaustive_pairs_helper(
        ['a', 'b', 'c'].iter().copied(),
        exhaustive_orderings(),
        9,
        &[
            ('a', Equal),
            ('a', Less),
            ('b', Equal),
            ('b', Less),
            ('a', Greater),
            ('b', Greater),
            ('c', Equal),
            ('c', Less),
            ('c', Greater),
        ],
    );
    exhaustive_pairs_helper(
        exhaustive_pairs(exhaustive_orderings(), exhaustive_bools()),
        exhaustive_triples_from_single([Less, Greater].iter().copied()),
        48,
        &[
            ((Equal, false), (Less, Less, Less)),
            ((Equal, false), (Less, Less, Greater)),
            ((Equal, true), (Less, Less, Less)),
            ((Equal, true), (Less, Less, Greater)),
            ((Equal, false), (Less, Greater, Less)),
            ((Equal, false), (Less, Greater, Greater)),
            ((Equal, true), (Less, Greater, Less)),
            ((Equal, true), (Less, Greater, Greater)),
            ((Less, false), (Less, Less, Less)),
            ((Less, false), (Less, Less, Greater)),
            ((Less, true), (Less, Less, Less)),
            ((Less, true), (Less, Less, Greater)),
            ((Less, false), (Less, Greater, Less)),
            ((Less, false), (Less, Greater, Greater)),
            ((Less, true), (Less, Greater, Less)),
            ((Less, true), (Less, Greater, Greater)),
            ((Equal, false), (Greater, Less, Less)),
            ((Equal, false), (Greater, Less, Greater)),
            ((Equal, true), (Greater, Less, Less)),
            ((Equal, true), (Greater, Less, Greater)),
        ],
    );
}

#[allow(clippy::needless_pass_by_value)]
fn exhaustive_triples_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
    Z: Clone + Debug + Eq,
    K: Clone + Iterator<Item = Z>,
>(
    xs: I,
    ys: J,
    zs: K,
    out_len: usize,
    out: &[(X, Y, Z)],
) {
    let ts = exhaustive_triples(xs.clone(), ys.clone(), zs.clone());
    assert_eq!(ts.clone().take(20).collect_vec(), out);
    assert_eq!(ts.count(), out_len);

    let output_types = get_sample_output_types(3);
    let ts = exhaustive_triples_custom_output(
        xs.clone(),
        ys.clone(),
        zs.clone(),
        output_types[0][0],
        output_types[0][1],
        output_types[0][2],
    );
    assert_eq!(ts.clone().take(20).collect_vec(), out);
    assert_eq!(ts.count(), out_len);
    for alt_output_types in &output_types[1..] {
        let ts = exhaustive_triples_custom_output(
            xs.clone(),
            ys.clone(),
            zs.clone(),
            alt_output_types[0],
            alt_output_types[1],
            alt_output_types[2],
        );
        ts.clone().take(20).for_each(drop);
        assert_eq!(ts.count(), out_len);
    }
}

#[test]
fn test_exhaustive_triples_and_exhaustive_triples_custom_output() {
    exhaustive_triples_helper(nevers(), nevers(), nevers(), 0, &[]);
    exhaustive_triples_helper(nevers(), 0..4, 'a'..'f', 0, &[]);
    exhaustive_triples_helper(once('a'), once(false), once(5), 1, &[('a', false, 5)]);
    exhaustive_triples_helper(
        once('a'),
        once(false),
        0..4,
        4,
        &[('a', false, 0), ('a', false, 1), ('a', false, 2), ('a', false, 3)],
    );
    exhaustive_triples_helper(
        exhaustive_unsigneds::<u8>(),
        exhaustive_pairs_from_single(exhaustive_bools()),
        'a'..'e',
        4096,
        &[
            (0, (false, false), 'a'),
            (0, (false, false), 'b'),
            (0, (false, true), 'a'),
            (0, (false, true), 'b'),
            (1, (false, false), 'a'),
            (1, (false, false), 'b'),
            (1, (false, true), 'a'),
            (1, (false, true), 'b'),
            (0, (false, false), 'c'),
            (0, (false, false), 'd'),
            (0, (false, true), 'c'),
            (0, (false, true), 'd'),
            (1, (false, false), 'c'),
            (1, (false, false), 'd'),
            (1, (false, true), 'c'),
            (1, (false, true), 'd'),
            (0, (true, false), 'a'),
            (0, (true, false), 'b'),
            (0, (true, true), 'a'),
            (0, (true, true), 'b'),
        ],
    );
    exhaustive_triples_helper(
        exhaustive_bools(),
        0..3,
        'a'..'d',
        18,
        &[
            (false, 0, 'a'),
            (false, 0, 'b'),
            (false, 1, 'a'),
            (false, 1, 'b'),
            (true, 0, 'a'),
            (true, 0, 'b'),
            (true, 1, 'a'),
            (true, 1, 'b'),
            (false, 0, 'c'),
            (false, 1, 'c'),
            (true, 0, 'c'),
            (true, 1, 'c'),
            (false, 2, 'a'),
            (false, 2, 'b'),
            (true, 2, 'a'),
            (true, 2, 'b'),
            (false, 2, 'c'),
            (true, 2, 'c'),
        ],
    );
    exhaustive_triples_helper(
        0..11,
        exhaustive_somes(0..12),
        'a'..'n',
        1716,
        &[
            (0, Some(0), 'a'),
            (0, Some(0), 'b'),
            (0, Some(1), 'a'),
            (0, Some(1), 'b'),
            (1, Some(0), 'a'),
            (1, Some(0), 'b'),
            (1, Some(1), 'a'),
            (1, Some(1), 'b'),
            (0, Some(0), 'c'),
            (0, Some(0), 'd'),
            (0, Some(1), 'c'),
            (0, Some(1), 'd'),
            (1, Some(0), 'c'),
            (1, Some(0), 'd'),
            (1, Some(1), 'c'),
            (1, Some(1), 'd'),
            (0, Some(2), 'a'),
            (0, Some(2), 'b'),
            (0, Some(3), 'a'),
            (0, Some(3), 'b'),
        ],
    );
    exhaustive_triples_helper(
        ['a', 'b', 'c'].iter().copied(),
        ["xx", "yy", "zz"].iter().copied(),
        0..3,
        27,
        &[
            ('a', "xx", 0),
            ('a', "xx", 1),
            ('a', "yy", 0),
            ('a', "yy", 1),
            ('b', "xx", 0),
            ('b', "xx", 1),
            ('b', "yy", 0),
            ('b', "yy", 1),
            ('a', "xx", 2),
            ('a', "yy", 2),
            ('b', "xx", 2),
            ('b', "yy", 2),
            ('a', "zz", 0),
            ('a', "zz", 1),
            ('b', "zz", 0),
            ('b', "zz", 1),
            ('a', "zz", 2),
            ('b', "zz", 2),
            ('c', "xx", 0),
            ('c', "xx", 1),
        ],
    );
}
