// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::get_sample_output_types;
use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::nevers::nevers;
use malachite_base::num::exhaustive::{exhaustive_positive_primitive_ints, exhaustive_unsigneds};
use malachite_base::vecs::exhaustive::{
    exhaustive_vecs_fixed_length_1_input, exhaustive_vecs_fixed_length_2_inputs,
};
use std::fmt::Debug;
use std::iter::empty;

fn exhaustive_vecs_fixed_length_1_input_helper<T, I: Clone + Iterator<Item = T>>(
    xs: &I,
    len: usize,
    out_len: Option<usize>,
    out: &[&[T]],
) where
    T: Clone + Debug + Eq,
{
    let output_types = get_sample_output_types(len);
    let xss = exhaustive_vecs_fixed_length_1_input(xs.clone(), &output_types[0]);
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
    for alt_output_types in &output_types[1..] {
        let xss = exhaustive_vecs_fixed_length_1_input(xs.clone(), alt_output_types);
        xss.clone().take(20).for_each(drop);
        if let Some(out_len) = out_len {
            assert_eq!(xss.count(), out_len);
        }
    }
}

#[test]
fn test_exhaustive_vecs_fixed_length_1_input() {
    exhaustive_vecs_fixed_length_1_input_helper(&nevers(), 2, Some(0), &[]);
    exhaustive_vecs_fixed_length_1_input_helper(
        &exhaustive_unsigneds::<u8>(),
        2,
        Some(1 << 16),
        &[
            &[0, 0],
            &[0, 1],
            &[1, 0],
            &[1, 1],
            &[0, 2],
            &[0, 3],
            &[1, 2],
            &[1, 3],
            &[2, 0],
            &[2, 1],
            &[3, 0],
            &[3, 1],
            &[2, 2],
            &[2, 3],
            &[3, 2],
            &[3, 3],
            &[0, 4],
            &[0, 5],
            &[1, 4],
            &[1, 5],
        ],
    );
    exhaustive_vecs_fixed_length_1_input_helper(
        &exhaustive_positive_primitive_ints::<u64>(),
        2,
        None,
        &[
            &[1, 1],
            &[1, 2],
            &[2, 1],
            &[2, 2],
            &[1, 3],
            &[1, 4],
            &[2, 3],
            &[2, 4],
            &[3, 1],
            &[3, 2],
            &[4, 1],
            &[4, 2],
            &[3, 3],
            &[3, 4],
            &[4, 3],
            &[4, 4],
            &[1, 5],
            &[1, 6],
            &[2, 5],
            &[2, 6],
        ],
    );
    exhaustive_vecs_fixed_length_1_input_helper(
        &['x', 'y', 'z'].iter().copied(),
        2,
        Some(9),
        &[
            &['x', 'x'],
            &['x', 'y'],
            &['y', 'x'],
            &['y', 'y'],
            &['x', 'z'],
            &['y', 'z'],
            &['z', 'x'],
            &['z', 'y'],
            &['z', 'z'],
        ],
    );
    exhaustive_vecs_fixed_length_1_input_helper(
        &['x', 'y', 'z'].iter().copied(),
        3,
        Some(27),
        &[
            &['x', 'x', 'x'],
            &['x', 'x', 'y'],
            &['x', 'y', 'x'],
            &['x', 'y', 'y'],
            &['y', 'x', 'x'],
            &['y', 'x', 'y'],
            &['y', 'y', 'x'],
            &['y', 'y', 'y'],
            &['x', 'x', 'z'],
            &['x', 'y', 'z'],
            &['y', 'x', 'z'],
            &['y', 'y', 'z'],
            &['x', 'z', 'x'],
            &['x', 'z', 'y'],
            &['y', 'z', 'x'],
            &['y', 'z', 'y'],
            &['x', 'z', 'z'],
            &['y', 'z', 'z'],
            &['z', 'x', 'x'],
            &['z', 'x', 'y'],
        ],
    );
    exhaustive_vecs_fixed_length_1_input_helper(
        &exhaustive_ascii_chars(),
        3,
        None,
        &[
            &['a', 'a', 'a'],
            &['a', 'a', 'b'],
            &['a', 'b', 'a'],
            &['a', 'b', 'b'],
            &['b', 'a', 'a'],
            &['b', 'a', 'b'],
            &['b', 'b', 'a'],
            &['b', 'b', 'b'],
            &['a', 'a', 'c'],
            &['a', 'a', 'd'],
            &['a', 'b', 'c'],
            &['a', 'b', 'd'],
            &['b', 'a', 'c'],
            &['b', 'a', 'd'],
            &['b', 'b', 'c'],
            &['b', 'b', 'd'],
            &['a', 'c', 'a'],
            &['a', 'c', 'b'],
            &['a', 'd', 'a'],
            &['a', 'd', 'b'],
        ],
    );
}

fn exhaustive_vecs_fixed_length_2_inputs_helper<
    T,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
>(
    xs: &I,
    ys: &J,
    len: usize,
    input_indices: &[usize],
    out_len: Option<usize>,
    out: &[&[T]],
) where
    T: Clone + Debug + Eq,
{
    let output_types = get_sample_output_types(len);
    let output_configs: Vec<(BitDistributorOutputType, usize)> = output_types[0]
        .iter()
        .copied()
        .zip(input_indices.iter().copied())
        .collect();
    let xss = exhaustive_vecs_fixed_length_2_inputs(xs.clone(), ys.clone(), &output_configs);
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
    for alt_output_types in &output_types[1..] {
        let output_configs: Vec<(BitDistributorOutputType, usize)> = alt_output_types
            .iter()
            .copied()
            .zip(input_indices.iter().copied())
            .collect();
        let xss = exhaustive_vecs_fixed_length_2_inputs(xs.clone(), ys.clone(), &output_configs);
        xss.clone().take(20).for_each(drop);
        if let Some(out_len) = out_len {
            assert_eq!(xss.count(), out_len);
        }
    }
}

#[test]
fn test_exhaustive_vecs_fixed_length_2_inputs() {
    exhaustive_vecs_fixed_length_2_inputs_helper(&nevers(), &nevers(), 2, &[0, 1], Some(0), &[]);
    exhaustive_vecs_fixed_length_2_inputs_helper(
        &empty(),
        &exhaustive_unsigneds::<u8>(),
        2,
        &[0, 1],
        Some(0),
        &[],
    );
    exhaustive_vecs_fixed_length_2_inputs_helper(
        &exhaustive_unsigneds::<u64>(),
        &exhaustive_positive_primitive_ints::<u64>(),
        2,
        &[0, 1],
        None,
        &[
            &[0, 1],
            &[0, 2],
            &[1, 1],
            &[1, 2],
            &[0, 3],
            &[0, 4],
            &[1, 3],
            &[1, 4],
            &[2, 1],
            &[2, 2],
            &[3, 1],
            &[3, 2],
            &[2, 3],
            &[2, 4],
            &[3, 3],
            &[3, 4],
            &[0, 5],
            &[0, 6],
            &[1, 5],
            &[1, 6],
        ],
    );
    exhaustive_vecs_fixed_length_2_inputs_helper(
        &exhaustive_ascii_chars(),
        &['x', 'y', 'z'].iter().copied(),
        2,
        &[0, 1],
        Some(384),
        &[
            &['a', 'x'],
            &['a', 'y'],
            &['b', 'x'],
            &['b', 'y'],
            &['a', 'z'],
            &['b', 'z'],
            &['c', 'x'],
            &['c', 'y'],
            &['d', 'x'],
            &['d', 'y'],
            &['c', 'z'],
            &['d', 'z'],
            &['e', 'x'],
            &['e', 'y'],
            &['f', 'x'],
            &['f', 'y'],
            &['e', 'z'],
            &['f', 'z'],
            &['g', 'x'],
            &['g', 'y'],
        ],
    );
    exhaustive_vecs_fixed_length_2_inputs_helper(
        &exhaustive_ascii_chars(),
        &['x', 'y', 'z'].iter().copied(),
        3,
        &[0, 1, 1],
        Some(1152),
        &[
            &['a', 'x', 'x'],
            &['a', 'x', 'y'],
            &['a', 'y', 'x'],
            &['a', 'y', 'y'],
            &['b', 'x', 'x'],
            &['b', 'x', 'y'],
            &['b', 'y', 'x'],
            &['b', 'y', 'y'],
            &['a', 'x', 'z'],
            &['a', 'y', 'z'],
            &['b', 'x', 'z'],
            &['b', 'y', 'z'],
            &['a', 'z', 'x'],
            &['a', 'z', 'y'],
            &['b', 'z', 'x'],
            &['b', 'z', 'y'],
            &['a', 'z', 'z'],
            &['b', 'z', 'z'],
            &['c', 'x', 'x'],
            &['c', 'x', 'y'],
        ],
    );
    exhaustive_vecs_fixed_length_2_inputs_helper(
        &exhaustive_ascii_chars(),
        &['x', 'y', 'z'].iter().copied(),
        3,
        &[0, 1, 0],
        Some(49152),
        &[
            &['a', 'x', 'a'],
            &['a', 'x', 'b'],
            &['a', 'y', 'a'],
            &['a', 'y', 'b'],
            &['b', 'x', 'a'],
            &['b', 'x', 'b'],
            &['b', 'y', 'a'],
            &['b', 'y', 'b'],
            &['a', 'x', 'c'],
            &['a', 'x', 'd'],
            &['a', 'y', 'c'],
            &['a', 'y', 'd'],
            &['b', 'x', 'c'],
            &['b', 'x', 'd'],
            &['b', 'y', 'c'],
            &['b', 'y', 'd'],
            &['a', 'z', 'a'],
            &['a', 'z', 'b'],
            &['b', 'z', 'a'],
            &['b', 'z', 'b'],
        ],
    );
}

#[test]
#[should_panic]
fn exhaustive_vecs_fixed_length_2_inputs_fail_1() {
    exhaustive_vecs_fixed_length_2_inputs(0..2, 0..3, &[]);
}

#[test]
#[should_panic]
fn exhaustive_vecs_fixed_length_2_inputs_fail_2() {
    exhaustive_vecs_fixed_length_2_inputs(0..2, 0..3, &[(BitDistributorOutputType::normal(1), 0)]);
}

#[test]
#[should_panic]
fn exhaustive_vecs_fixed_length_2_inputs_fail_3() {
    exhaustive_vecs_fixed_length_2_inputs(0..2, 0..3, &[(BitDistributorOutputType::normal(1), 1)]);
}

#[test]
#[should_panic]
fn exhaustive_vecs_fixed_length_2_inputs_fail_4() {
    exhaustive_vecs_fixed_length_2_inputs(
        0..2,
        0..3,
        &[
            (BitDistributorOutputType::normal(1), 0),
            (BitDistributorOutputType::normal(1), 1),
            (BitDistributorOutputType::normal(1), 2),
        ],
    );
}
