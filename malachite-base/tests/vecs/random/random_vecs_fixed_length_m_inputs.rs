// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::hash::Hash;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_range};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::{
    random_vecs_fixed_length_2_inputs, random_vecs_fixed_length_from_single,
};
use std::fmt::Debug;

fn random_vecs_fixed_length_2_inputs_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    output_to_input_map: &[usize],
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_vecs_fixed_length_2_inputs(EXAMPLE_SEED, xs_gen, ys_gen, output_to_input_map),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_vecs_fixed_length_2_inputs() {
    random_vecs_fixed_length_2_inputs_helper(
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_range(seed, 0, 10),
        &[0, 0, 1],
        &[
            &[85, 11, 2],
            &[136, 200, 6],
            &[235, 134, 8],
            &[203, 223, 6],
            &[38, 235, 8],
            &[217, 177, 2],
            &[162, 32, 4],
            &[166, 234, 1],
            &[30, 218, 2],
            &[90, 106, 7],
            &[9, 216, 5],
            &[204, 151, 4],
            &[213, 97, 8],
            &[253, 78, 8],
            &[91, 39, 4],
            &[191, 175, 4],
            &[170, 232, 3],
            &[233, 2, 5],
            &[35, 22, 6],
            &[217, 198, 7],
        ],
        &[
            (&[156, 162, 3], 11),
            (&[248, 1, 7], 10),
            (&[178, 121, 1], 10),
            (&[36, 97, 6], 9),
            (&[46, 27, 2], 9),
            (&[64, 75, 6], 9),
            (&[135, 80, 5], 9),
            (&[215, 11, 3], 9),
            (&[39, 178, 7], 9),
            (&[75, 164, 6], 9),
        ],
        (&[127, 197, 7], None),
    );
    random_vecs_fixed_length_2_inputs_helper(
        &|seed| random_vecs_fixed_length_from_single(2, random_primitive_ints::<u8>(seed)),
        &|seed| random_vecs_fixed_length_from_single(3, random_primitive_ints::<u8>(seed)),
        &[0, 1, 0],
        &[
            &[vec![85, 11], vec![98, 168, 198], vec![136, 200]],
            &[vec![235, 134], vec![40, 20, 252], vec![203, 223]],
            &[vec![38, 235], vec![47, 87, 132], vec![217, 177]],
            &[vec![162, 32], vec![72, 77, 63], vec![166, 234]],
            &[vec![30, 218], vec![91, 108, 127], vec![90, 106]],
            &[vec![9, 216], vec![53, 141, 84], vec![204, 151]],
            &[vec![213, 97], vec![18, 10, 112], vec![253, 78]],
            &[vec![91, 39], vec![154, 104, 53], vec![191, 175]],
            &[vec![170, 232], vec![75, 238, 149], vec![233, 2]],
            &[vec![35, 22], vec![190, 51, 147], vec![217, 198]],
            &[vec![114, 17], vec![100, 114, 140], vec![32, 173]],
            &[vec![114, 65], vec![2, 63, 189], vec![121, 222]],
            &[vec![173, 25], vec![222, 67, 119], vec![144, 148]],
            &[vec![79, 115], vec![0, 223, 5], vec![52, 73]],
            &[vec![69, 137], vec![236, 232, 50], vec![91, 153]],
            &[vec![178, 112], vec![44, 241, 21], vec![34, 95]],
            &[vec![106, 167], vec![22, 94, 27], vec![197, 130]],
            &[vec![168, 122], vec![128, 220, 25], vec![207, 172]],
            &[vec![177, 86], vec![251, 243, 50], vec![150, 221]],
            &[vec![218, 101], vec![137, 235, 46], vec![115, 74]],
        ],
        &[
            (&[vec![8, 24], vec![0, 54, 59], vec![5, 3]], 1),
            (&[vec![8, 72], vec![6, 5, 9], vec![11, 57]], 1),
            (&[vec![80, 9], vec![84, 9, 10], vec![9, 5]], 1),
            (&[vec![86, 2], vec![2, 0, 27], vec![49, 4]], 1),
            (&[vec![0, 2], vec![207, 31, 7], vec![92, 5]], 1),
            (&[vec![1, 15], vec![51, 5, 47], vec![12, 5]], 1),
            (&[vec![1, 25], vec![70, 65, 7], vec![3, 66]], 1),
            (&[vec![1, 72], vec![8, 49, 246], vec![2, 1]], 1),
            (&[vec![1, 82], vec![86, 3, 70], vec![6, 26]], 1),
            (&[vec![1, 85], vec![3, 5, 53], vec![14, 92]], 1),
        ],
        (
            &[vec![128, 20], vec![252, 3, 74], vec![108, 132]],
            Some(&[vec![128, 21], vec![6, 87, 236], vec![223, 197]]),
        ),
    );
}

#[test]
#[should_panic]
fn random_vecs_fixed_length_2_inputs_fail_1() {
    random_vecs_fixed_length_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[],
    );
}

#[test]
#[should_panic]
fn random_vecs_fixed_length_2_inputs_fail_2() {
    random_vecs_fixed_length_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[0],
    );
}

#[test]
#[should_panic]
fn random_vecs_fixed_length_2_inputs_fail_3() {
    random_vecs_fixed_length_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[1],
    );
}

#[test]
#[should_panic]
fn random_vecs_fixed_length_2_inputs_fail_4() {
    random_vecs_fixed_length_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[0, 1, 2],
    );
}
