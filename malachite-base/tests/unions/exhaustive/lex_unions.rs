// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::lex_union3s;
use crate::extra_variadic::Union3;
use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::options::exhaustive::exhaustive_somes;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::unions::exhaustive::lex_union2s;
use malachite_base::unions::Union2;
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::iter::once;

fn lex_union2s_helper<
    X: Clone + Debug + Eq,
    I: Clone + Iterator<Item = X>,
    Y: Clone + Debug + Eq,
    J: Clone + Iterator<Item = Y>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[Union2<X, Y>],
) {
    let us = lex_union2s(xs, ys);
    let us_prefix = us.clone().take(20).collect_vec();
    assert_eq!(us_prefix.as_slice(), out);
    assert_eq!(us.count(), out_len);
}

#[test]
fn test_lex_union2s() {
    lex_union2s_helper(nevers(), nevers(), 0, &[]);
    lex_union2s_helper(
        nevers(),
        0..4,
        4,
        &[Union2::B(0), Union2::B(1), Union2::B(2), Union2::B(3)],
    );
    lex_union2s_helper(once('a'), once(1), 2, &[Union2::A('a'), Union2::B(1)]);
    lex_union2s_helper(
        once('a'),
        0..4,
        5,
        &[Union2::A('a'), Union2::B(0), Union2::B(1), Union2::B(2), Union2::B(3)],
    );
    lex_union2s_helper(
        'a'..'e',
        exhaustive_unsigneds::<u8>(),
        260,
        &[
            Union2::A('a'),
            Union2::A('b'),
            Union2::A('c'),
            Union2::A('d'),
            Union2::B(0),
            Union2::B(1),
            Union2::B(2),
            Union2::B(3),
            Union2::B(4),
            Union2::B(5),
            Union2::B(6),
            Union2::B(7),
            Union2::B(8),
            Union2::B(9),
            Union2::B(10),
            Union2::B(11),
            Union2::B(12),
            Union2::B(13),
            Union2::B(14),
            Union2::B(15),
        ],
    );
    lex_union2s_helper(
        exhaustive_bools(),
        0..4,
        6,
        &[
            Union2::A(false),
            Union2::A(true),
            Union2::B(0),
            Union2::B(1),
            Union2::B(2),
            Union2::B(3),
        ],
    );
    lex_union2s_helper(
        'a'..'f',
        0..3,
        8,
        &[
            Union2::A('a'),
            Union2::A('b'),
            Union2::A('c'),
            Union2::A('d'),
            Union2::A('e'),
            Union2::B(0),
            Union2::B(1),
            Union2::B(2),
        ],
    );
    lex_union2s_helper(
        ['a', 'b', 'c'].iter().copied(),
        exhaustive_orderings(),
        6,
        &[
            Union2::A('a'),
            Union2::A('b'),
            Union2::A('c'),
            Union2::B(Equal),
            Union2::B(Less),
            Union2::B(Greater),
        ],
    );
}

fn lex_union3s_helper<
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
    out: &[Union3<X, Y, Z>],
) {
    let ts = lex_union3s(xs, ys, zs);
    let ts_prefix = ts.clone().take(20).collect_vec();
    assert_eq!(ts_prefix.as_slice(), out);
    assert_eq!(ts.count(), out_len);
}

#[test]
fn test_lex_union3s() {
    lex_union3s_helper(nevers(), nevers(), nevers(), 0, &[]);
    lex_union3s_helper(
        nevers(),
        0..4,
        'a'..'f',
        9,
        &[
            Union3::B(0),
            Union3::B(1),
            Union3::B(2),
            Union3::B(3),
            Union3::C('a'),
            Union3::C('b'),
            Union3::C('c'),
            Union3::C('d'),
            Union3::C('e'),
        ],
    );
    lex_union3s_helper(
        once('a'),
        once(false),
        once(5),
        3,
        &[Union3::A('a'), Union3::B(false), Union3::C(5)],
    );
    lex_union3s_helper(
        once('a'),
        once(false),
        0..4,
        6,
        &[Union3::A('a'), Union3::B(false), Union3::C(0), Union3::C(1), Union3::C(2), Union3::C(3)],
    );
    lex_union3s_helper(
        exhaustive_bools(),
        0..3,
        'a'..'d',
        8,
        &[
            Union3::A(false),
            Union3::A(true),
            Union3::B(0),
            Union3::B(1),
            Union3::B(2),
            Union3::C('a'),
            Union3::C('b'),
            Union3::C('c'),
        ],
    );
    lex_union3s_helper(
        0..11,
        exhaustive_somes(0..12),
        'a'..'n',
        36,
        &[
            Union3::A(0),
            Union3::A(1),
            Union3::A(2),
            Union3::A(3),
            Union3::A(4),
            Union3::A(5),
            Union3::A(6),
            Union3::A(7),
            Union3::A(8),
            Union3::A(9),
            Union3::A(10),
            Union3::B(Some(0)),
            Union3::B(Some(1)),
            Union3::B(Some(2)),
            Union3::B(Some(3)),
            Union3::B(Some(4)),
            Union3::B(Some(5)),
            Union3::B(Some(6)),
            Union3::B(Some(7)),
            Union3::B(Some(8)),
        ],
    );
    lex_union3s_helper(
        ['a', 'b', 'c'].iter().copied(),
        ["xx", "yy", "zz"].iter().copied(),
        0..3,
        9,
        &[
            Union3::A('a'),
            Union3::A('b'),
            Union3::A('c'),
            Union3::B("xx"),
            Union3::B("yy"),
            Union3::B("zz"),
            Union3::C(0),
            Union3::C(1),
            Union3::C(2),
        ],
    );
}
