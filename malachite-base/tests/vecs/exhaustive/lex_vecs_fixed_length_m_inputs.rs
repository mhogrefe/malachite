// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::{exhaustive_positive_primitive_ints, exhaustive_unsigneds};
use malachite_base::vecs::exhaustive::lex_vecs_fixed_length_2_inputs;
use std::fmt::Debug;
use std::iter::empty;

fn lex_vecs_fixed_length_2_inputs_helper<
    T: Clone + Debug + Eq,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
>(
    xs: &I,
    ys: &J,
    output_to_input_map: &[usize],
    out_len: Option<usize>,
    out: &[&[T]],
) {
    let xss = lex_vecs_fixed_length_2_inputs(xs.clone(), ys.clone(), output_to_input_map);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(
        xss_prefix
            .iter()
            .map(Vec::as_slice)
            .collect_vec()
            .as_slice(),
        out
    );
    if let Some(out_len) = out_len {
        assert_eq!(xss.count(), out_len);
    }
}

#[test]
fn test_lex_vecs_fixed_length_2_inputs() {
    lex_vecs_fixed_length_2_inputs_helper(&nevers(), &nevers(), &[0, 1], Some(0), &[]);
    lex_vecs_fixed_length_2_inputs_helper(
        &empty(),
        &exhaustive_unsigneds::<u8>(),
        &[0, 1],
        Some(0),
        &[],
    );
    lex_vecs_fixed_length_2_inputs_helper(
        &exhaustive_unsigneds::<u64>(),
        &exhaustive_positive_primitive_ints::<u64>(),
        &[0, 1],
        None,
        &[
            &[0, 1],
            &[0, 2],
            &[0, 3],
            &[0, 4],
            &[0, 5],
            &[0, 6],
            &[0, 7],
            &[0, 8],
            &[0, 9],
            &[0, 10],
            &[0, 11],
            &[0, 12],
            &[0, 13],
            &[0, 14],
            &[0, 15],
            &[0, 16],
            &[0, 17],
            &[0, 18],
            &[0, 19],
            &[0, 20],
        ],
    );
    lex_vecs_fixed_length_2_inputs_helper(
        &exhaustive_ascii_chars(),
        &['x', 'y', 'z'].iter().copied(),
        &[0, 1],
        Some(384),
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
            &['d', 'x'],
            &['d', 'y'],
            &['d', 'z'],
            &['e', 'x'],
            &['e', 'y'],
            &['e', 'z'],
            &['f', 'x'],
            &['f', 'y'],
            &['f', 'z'],
            &['g', 'x'],
            &['g', 'y'],
        ],
    );
    lex_vecs_fixed_length_2_inputs_helper(
        &exhaustive_ascii_chars(),
        &['x', 'y', 'z'].iter().copied(),
        &[0, 1, 1],
        Some(1152),
        &[
            &['a', 'x', 'x'],
            &['a', 'x', 'y'],
            &['a', 'x', 'z'],
            &['a', 'y', 'x'],
            &['a', 'y', 'y'],
            &['a', 'y', 'z'],
            &['a', 'z', 'x'],
            &['a', 'z', 'y'],
            &['a', 'z', 'z'],
            &['b', 'x', 'x'],
            &['b', 'x', 'y'],
            &['b', 'x', 'z'],
            &['b', 'y', 'x'],
            &['b', 'y', 'y'],
            &['b', 'y', 'z'],
            &['b', 'z', 'x'],
            &['b', 'z', 'y'],
            &['b', 'z', 'z'],
            &['c', 'x', 'x'],
            &['c', 'x', 'y'],
        ],
    );
    lex_vecs_fixed_length_2_inputs_helper(
        &exhaustive_ascii_chars(),
        &['x', 'y', 'z'].iter().copied(),
        &[0, 1, 0],
        Some(49152),
        &[
            &['a', 'x', 'a'],
            &['a', 'x', 'b'],
            &['a', 'x', 'c'],
            &['a', 'x', 'd'],
            &['a', 'x', 'e'],
            &['a', 'x', 'f'],
            &['a', 'x', 'g'],
            &['a', 'x', 'h'],
            &['a', 'x', 'i'],
            &['a', 'x', 'j'],
            &['a', 'x', 'k'],
            &['a', 'x', 'l'],
            &['a', 'x', 'm'],
            &['a', 'x', 'n'],
            &['a', 'x', 'o'],
            &['a', 'x', 'p'],
            &['a', 'x', 'q'],
            &['a', 'x', 'r'],
            &['a', 'x', 's'],
            &['a', 'x', 't'],
        ],
    );
}

#[test]
#[should_panic]
fn lex_vecs_fixed_length_2_inputs_fail_1() {
    lex_vecs_fixed_length_2_inputs(0..2, 0..3, &[]);
}

#[test]
#[should_panic]
fn lex_vecs_fixed_length_2_inputs_fail_2() {
    lex_vecs_fixed_length_2_inputs(0..2, 0..3, &[0]);
}

#[test]
#[should_panic]
fn lex_vecs_fixed_length_2_inputs_fail_3() {
    lex_vecs_fixed_length_2_inputs(0..2, 0..3, &[1]);
}

#[test]
#[should_panic]
fn lex_vecs_fixed_length_2_inputs_fail_4() {
    lex_vecs_fixed_length_2_inputs(0..2, 0..3, &[0, 1, 2]);
}
