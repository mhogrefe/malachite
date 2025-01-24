// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::lex_vecs_length_3;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::exhaustive_unsigneds;
use malachite_base::orderings::exhaustive::exhaustive_orderings;
use malachite_base::test_util::vecs::exhaustive::{
    exhaustive_vecs_helper_helper, exhaustive_vecs_small_helper_helper,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_vecs_fixed_length_from_single, lex_vecs_length_2,
};
use std::cmp::Ordering::*;
use std::fmt::Debug;
use std::iter::{empty, once};

fn lex_vecs_length_2_helper<T: Clone + Debug + Eq, I: Iterator<Item = T>, J: Iterator<Item = T>>(
    xs: I,
    ys: J,
    out: &[&[T]],
) {
    exhaustive_vecs_helper_helper(lex_vecs_length_2(xs, ys), out);
}

fn lex_vecs_length_2_finite_helper<
    T: Clone + Debug + Eq,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
>(
    xs: I,
    ys: J,
    out_len: usize,
    out: &[&[T]],
) {
    exhaustive_vecs_small_helper_helper(lex_vecs_length_2(xs, ys), out_len, out);
}

#[test]
fn test_lex_vecs_length_2() {
    lex_vecs_length_2_finite_helper(nevers(), nevers(), 0, &[]);
    lex_vecs_length_2_finite_helper(empty(), 0..4, 0, &[]);
    lex_vecs_length_2_finite_helper(once(0), once(1), 1, &[&[0, 1]]);
    lex_vecs_length_2_finite_helper(once(0), 0..4, 4, &[&[0, 0], &[0, 1], &[0, 2], &[0, 3]]);
    lex_vecs_length_2_finite_helper(
        exhaustive_unsigneds::<u8>(),
        0..4,
        1024,
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
            &[3, 0],
            &[3, 1],
            &[3, 2],
            &[3, 3],
            &[4, 0],
            &[4, 1],
            &[4, 2],
            &[4, 3],
        ],
    );
    lex_vecs_length_2_helper(
        exhaustive_unsigneds::<u64>(),
        0..4,
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[2, 3],
            &[3, 0],
            &[3, 1],
            &[3, 2],
            &[3, 3],
            &[4, 0],
            &[4, 1],
            &[4, 2],
            &[4, 3],
        ],
    );
    lex_vecs_length_2_finite_helper(
        0..2,
        0..4,
        8,
        &[&[0, 0], &[0, 1], &[0, 2], &[0, 3], &[1, 0], &[1, 1], &[1, 2], &[1, 3]],
    );
    lex_vecs_length_2_finite_helper(
        0..5,
        0..3,
        15,
        &[
            &[0, 0],
            &[0, 1],
            &[0, 2],
            &[1, 0],
            &[1, 1],
            &[1, 2],
            &[2, 0],
            &[2, 1],
            &[2, 2],
            &[3, 0],
            &[3, 1],
            &[3, 2],
            &[4, 0],
            &[4, 1],
            &[4, 2],
        ],
    );
    lex_vecs_length_2_finite_helper(
        ['a', 'b', 'c'].iter().copied(),
        ['x', 'y', 'z'].iter().copied(),
        9,
        &[
            &['a', 'x'],
            &['a', 'y'],
            &['a', 'z'],
            &['b', 'x'],
            &['b', 'y'],
            &['b', 'z'],
            &['c', 'x'],
            &['c', 'y'],
            &['c', 'z'],
        ],
    );
    lex_vecs_length_2_finite_helper(
        lex_vecs_length_2(exhaustive_orderings(), [Less, Greater].iter().copied()),
        exhaustive_vecs_fixed_length_from_single(3, [Less, Greater].iter().copied()),
        48,
        &[
            &[vec![Equal, Less], vec![Less, Less, Less]],
            &[vec![Equal, Less], vec![Less, Less, Greater]],
            &[vec![Equal, Less], vec![Less, Greater, Less]],
            &[vec![Equal, Less], vec![Less, Greater, Greater]],
            &[vec![Equal, Less], vec![Greater, Less, Less]],
            &[vec![Equal, Less], vec![Greater, Less, Greater]],
            &[vec![Equal, Less], vec![Greater, Greater, Less]],
            &[vec![Equal, Less], vec![Greater, Greater, Greater]],
            &[vec![Equal, Greater], vec![Less, Less, Less]],
            &[vec![Equal, Greater], vec![Less, Less, Greater]],
            &[vec![Equal, Greater], vec![Less, Greater, Less]],
            &[vec![Equal, Greater], vec![Less, Greater, Greater]],
            &[vec![Equal, Greater], vec![Greater, Less, Less]],
            &[vec![Equal, Greater], vec![Greater, Less, Greater]],
            &[vec![Equal, Greater], vec![Greater, Greater, Less]],
            &[vec![Equal, Greater], vec![Greater, Greater, Greater]],
            &[vec![Less, Less], vec![Less, Less, Less]],
            &[vec![Less, Less], vec![Less, Less, Greater]],
            &[vec![Less, Less], vec![Less, Greater, Less]],
            &[vec![Less, Less], vec![Less, Greater, Greater]],
        ],
    );
}

fn lex_vecs_length_3_helper<
    T,
    I: Iterator<Item = T>,
    J: Iterator<Item = T>,
    K: Iterator<Item = T>,
>(
    xs: I,
    ys: J,
    zs: K,
    out: &[&[T]],
) where
    T: Clone + Debug + Eq,
{
    exhaustive_vecs_helper_helper(lex_vecs_length_3(xs, ys, zs), out);
}

fn lex_vecs_length_3_finite_helper<
    T,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
    K: Clone + Iterator<Item = T>,
>(
    xs: I,
    ys: J,
    zs: K,
    out_len: usize,
    out: &[&[T]],
) where
    T: Clone + Debug + Eq,
{
    exhaustive_vecs_small_helper_helper(lex_vecs_length_3(xs, ys, zs), out_len, out);
}

#[test]
fn test_lex_vecs_length_3() {
    lex_vecs_length_3_finite_helper(nevers(), nevers(), nevers(), 0, &[]);
    lex_vecs_length_3_finite_helper(empty(), 0..4, 0..5, 0, &[]);
    lex_vecs_length_3_finite_helper(once(0), once(1), once(5), 1, &[&[0, 1, 5]]);
    lex_vecs_length_3_finite_helper(
        once(0),
        once(3),
        0..4,
        4,
        &[&[0, 3, 0], &[0, 3, 1], &[0, 3, 2], &[0, 3, 3]],
    );
    lex_vecs_length_3_finite_helper(
        exhaustive_unsigneds::<u8>(),
        0..4,
        0..4,
        4096,
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 1, 0],
            &[0, 1, 1],
            &[0, 1, 2],
            &[0, 1, 3],
            &[0, 2, 0],
            &[0, 2, 1],
            &[0, 2, 2],
            &[0, 2, 3],
            &[0, 3, 0],
            &[0, 3, 1],
            &[0, 3, 2],
            &[0, 3, 3],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 0, 2],
            &[1, 0, 3],
        ],
    );
    lex_vecs_length_3_helper(
        exhaustive_unsigneds::<u64>(),
        0..4,
        0..4,
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 1, 0],
            &[0, 1, 1],
            &[0, 1, 2],
            &[0, 1, 3],
            &[0, 2, 0],
            &[0, 2, 1],
            &[0, 2, 2],
            &[0, 2, 3],
            &[0, 3, 0],
            &[0, 3, 1],
            &[0, 3, 2],
            &[0, 3, 3],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 0, 2],
            &[1, 0, 3],
        ],
    );
    lex_vecs_length_3_finite_helper(
        0..2,
        0..3,
        0..3,
        18,
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 0, 2],
            &[0, 1, 0],
            &[0, 1, 1],
            &[0, 1, 2],
            &[0, 2, 0],
            &[0, 2, 1],
            &[0, 2, 2],
            &[1, 0, 0],
            &[1, 0, 1],
            &[1, 0, 2],
            &[1, 1, 0],
            &[1, 1, 1],
            &[1, 1, 2],
            &[1, 2, 0],
            &[1, 2, 1],
            &[1, 2, 2],
        ],
    );
    lex_vecs_length_3_finite_helper(
        0..11,
        0..12,
        0..13,
        1716,
        &[
            &[0, 0, 0],
            &[0, 0, 1],
            &[0, 0, 2],
            &[0, 0, 3],
            &[0, 0, 4],
            &[0, 0, 5],
            &[0, 0, 6],
            &[0, 0, 7],
            &[0, 0, 8],
            &[0, 0, 9],
            &[0, 0, 10],
            &[0, 0, 11],
            &[0, 0, 12],
            &[0, 1, 0],
            &[0, 1, 1],
            &[0, 1, 2],
            &[0, 1, 3],
            &[0, 1, 4],
            &[0, 1, 5],
            &[0, 1, 6],
        ],
    );
    lex_vecs_length_3_finite_helper(
        ['a', 'b', 'c'].iter().copied(),
        ['x', 'y', 'z'].iter().copied(),
        ['0', '1', '2'].iter().copied(),
        27,
        &[
            &['a', 'x', '0'],
            &['a', 'x', '1'],
            &['a', 'x', '2'],
            &['a', 'y', '0'],
            &['a', 'y', '1'],
            &['a', 'y', '2'],
            &['a', 'z', '0'],
            &['a', 'z', '1'],
            &['a', 'z', '2'],
            &['b', 'x', '0'],
            &['b', 'x', '1'],
            &['b', 'x', '2'],
            &['b', 'y', '0'],
            &['b', 'y', '1'],
            &['b', 'y', '2'],
            &['b', 'z', '0'],
            &['b', 'z', '1'],
            &['b', 'z', '2'],
            &['c', 'x', '0'],
            &['c', 'x', '1'],
        ],
    );
}
