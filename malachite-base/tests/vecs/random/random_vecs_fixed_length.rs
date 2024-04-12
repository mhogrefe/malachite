// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::extra_variadic::random_vecs_length_3;
use core::hash::Hash;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_range};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::vecs::random::random_vecs_helper_helper;
use malachite_base::vecs::random::{random_vecs_fixed_length_from_single, random_vecs_length_2};
use std::fmt::Debug;

fn random_vecs_length_2_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_vecs_length_2(EXAMPLE_SEED, xs_gen, ys_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_vecs_length_2() {
    random_vecs_length_2_helper(
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_range(seed, 0, 10),
        &[
            &[85, 2],
            &[11, 6],
            &[136, 8],
            &[200, 6],
            &[235, 8],
            &[134, 2],
            &[203, 4],
            &[223, 1],
            &[38, 2],
            &[235, 7],
            &[217, 5],
            &[177, 4],
            &[162, 8],
            &[32, 8],
            &[166, 4],
            &[234, 4],
            &[30, 3],
            &[218, 5],
            &[90, 6],
            &[106, 7],
        ],
        &[
            (&[196, 6], 466),
            (&[162, 5], 457),
            (&[132, 5], 455),
            (&[200, 2], 455),
            (&[61, 3], 454),
            (&[117, 5], 453),
            (&[28, 0], 446),
            (&[148, 5], 446),
            (&[194, 9], 446),
            (&[44, 3], 444),
        ],
        (&[127, 9], None),
    );
    random_vecs_length_2_helper(
        &|seed| random_vecs_fixed_length_from_single(2, random_primitive_ints::<u8>(seed)),
        &|seed| random_vecs_fixed_length_from_single(3, random_primitive_ints::<u8>(seed)),
        &[
            &[vec![85, 11], vec![98, 168, 198]],
            &[vec![136, 200], vec![40, 20, 252]],
            &[vec![235, 134], vec![47, 87, 132]],
            &[vec![203, 223], vec![72, 77, 63]],
            &[vec![38, 235], vec![91, 108, 127]],
            &[vec![217, 177], vec![53, 141, 84]],
            &[vec![162, 32], vec![18, 10, 112]],
            &[vec![166, 234], vec![154, 104, 53]],
            &[vec![30, 218], vec![75, 238, 149]],
            &[vec![90, 106], vec![190, 51, 147]],
            &[vec![9, 216], vec![100, 114, 140]],
            &[vec![204, 151], vec![2, 63, 189]],
            &[vec![213, 97], vec![222, 67, 119]],
            &[vec![253, 78], vec![0, 223, 5]],
            &[vec![91, 39], vec![236, 232, 50]],
            &[vec![191, 175], vec![44, 241, 21]],
            &[vec![170, 232], vec![22, 94, 27]],
            &[vec![233, 2], vec![128, 220, 25]],
            &[vec![35, 22], vec![251, 243, 50]],
            &[vec![217, 198], vec![137, 235, 46]],
        ],
        &[
            (&[vec![0, 5], vec![6, 7, 42]], 1),
            (&[vec![8, 8], vec![18, 5, 6]], 1),
            (&[vec![9, 1], vec![5, 3, 23]], 1),
            (&[vec![0, 0], vec![97, 7, 73]], 1),
            (&[vec![0, 2], vec![12, 20, 6]], 1),
            (&[vec![0, 99], vec![20, 8, 6]], 1),
            (&[vec![1, 81], vec![3, 21, 3]], 1),
            (&[vec![1, 9], vec![219, 9, 7]], 1),
            (&[vec![1, 9], vec![4, 95, 15]], 1),
            (&[vec![15, 2], vec![56, 0, 8]], 1),
        ],
        (
            &[vec![127, 197], vec![162, 123, 217]],
            Some(&[vec![127, 197], vec![163, 170, 161]]),
        ),
    );
}

fn random_vecs_length_3_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
    K: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    zs_gen: &dyn Fn(Seed) -> K,
    expected_values: &[&[T]],
    expected_common_values: &[(&[T], usize)],
    expected_median: (&[T], Option<&[T]>),
) {
    random_vecs_helper_helper(
        random_vecs_length_3(EXAMPLE_SEED, xs_gen, ys_gen, zs_gen),
        expected_values,
        expected_common_values,
        expected_median,
    );
}

#[test]
fn test_random_vecs_length_3() {
    random_vecs_length_3_helper(
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_char_inclusive_range(seed, 'd', 'f'),
        &|seed| random_char_inclusive_range(seed, 'g', 'i'),
        &[
            &['b', 'f', 'g'],
            &['b', 'd', 'i'],
            &['b', 'f', 'i'],
            &['b', 'e', 'i'],
            &['c', 'd', 'i'],
            &['a', 'f', 'i'],
            &['a', 'f', 'g'],
            &['a', 'f', 'g'],
            &['c', 'f', 'i'],
            &['a', 'e', 'i'],
            &['c', 'd', 'h'],
            &['a', 'd', 'h'],
            &['c', 'f', 'i'],
            &['a', 'f', 'i'],
            &['c', 'd', 'g'],
            &['c', 'd', 'h'],
            &['c', 'e', 'g'],
            &['b', 'e', 'h'],
            &['a', 'd', 'g'],
            &['c', 'd', 'g'],
        ],
        &[
            (&['b', 'f', 'i'], 37416),
            (&['a', 'f', 'g'], 37345),
            (&['c', 'd', 'i'], 37278),
            (&['b', 'f', 'g'], 37274),
            (&['a', 'd', 'h'], 37207),
            (&['b', 'f', 'h'], 37188),
            (&['b', 'd', 'i'], 37153),
            (&['b', 'e', 'g'], 37117),
            (&['a', 'd', 'g'], 37092),
            (&['c', 'f', 'g'], 37068),
        ],
        (&['b', 'e', 'h'], None),
    );
    random_vecs_length_3_helper(
        &|seed| random_vecs_fixed_length_from_single(1, random_primitive_ints::<u8>(seed)),
        &|seed| random_vecs_fixed_length_from_single(2, random_primitive_ints::<u8>(seed)),
        &|seed| random_vecs_fixed_length_from_single(3, random_primitive_ints::<u8>(seed)),
        &[
            &[vec![85], vec![98, 168], vec![168, 10, 250]],
            &[vec![11], vec![198, 40], vec![95, 250, 79]],
            &[vec![136], vec![20, 252], vec![4, 171, 141]],
            &[vec![200], vec![47, 87], vec![189, 177, 169]],
            &[vec![235], vec![132, 72], vec![36, 73, 154]],
            &[vec![134], vec![77, 63], vec![62, 202, 17]],
            &[vec![203], vec![91, 108], vec![35, 189, 158]],
            &[vec![223], vec![127, 53], vec![31, 173, 175]],
            &[vec![38], vec![141, 84], vec![63, 225, 106]],
            &[vec![235], vec![18, 10], vec![40, 116, 16]],
            &[vec![217], vec![112, 154], vec![88, 112, 9]],
            &[vec![177], vec![104, 53], vec![227, 144, 93]],
            &[vec![162], vec![75, 238], vec![85, 90, 214]],
            &[vec![32], vec![149, 190], vec![31, 60, 254]],
            &[vec![166], vec![51, 147], vec![143, 44, 177]],
            &[vec![234], vec![100, 114], vec![205, 197, 53]],
            &[vec![30], vec![140, 2], vec![15, 184, 137]],
            &[vec![218], vec![63, 189], vec![75, 116, 140]],
            &[vec![90], vec![222, 67], vec![19, 119, 60]],
            &[vec![106], vec![119, 0], vec![219, 21, 164]],
        ],
        &[
            (&[vec![0], vec![47, 4], vec![31, 6, 1]], 1),
            (&[vec![0], vec![5, 12], vec![9, 6, 54]], 1),
            (&[vec![6], vec![99, 35], vec![3, 2, 3]], 1),
            (&[vec![7], vec![7, 56], vec![6, 3, 76]], 1),
            (&[vec![7], vec![9, 5], vec![148, 1, 1]], 1),
            (&[vec![9], vec![61, 7], vec![9, 60, 8]], 1),
            (&[vec![0], vec![0, 55], vec![1, 12, 83]], 1),
            (&[vec![0], vec![1, 57], vec![60, 4, 55]], 1),
            (&[vec![0], vec![1, 8], vec![235, 0, 27]], 1),
            (&[vec![0], vec![73, 15], vec![0, 2, 11]], 1),
        ],
        (
            &[vec![127], vec![241, 129], vec![232, 173, 11]],
            Some(&[vec![127], vec![241, 149], vec![219, 172, 49]]),
        ),
    );
}
