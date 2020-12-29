use core::hash::Hash;
use itertools::Itertools;
use std::fmt::Debug;

use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;

use malachite_base::num::random::{random_primitive_ints, random_unsigned_range};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::vecs::random::{
    random_fixed_length_vecs_2_inputs, random_fixed_length_vecs_from_single,
};

fn random_fixed_length_vecs_2_inputs_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    output_to_input_map: &[usize],
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_fixed_length_vecs_2_inputs(EXAMPLE_SEED, xs_gen, ys_gen, output_to_input_map);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_fixed_length_vecs_2_inputs() {
    random_fixed_length_vecs_2_inputs_helper(
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_range(seed, 0, 10),
        &[0, 0, 1],
        &[
            vec![85, 11, 2],
            vec![136, 200, 6],
            vec![235, 134, 8],
            vec![203, 223, 6],
            vec![38, 235, 8],
            vec![217, 177, 2],
            vec![162, 32, 4],
            vec![166, 234, 1],
            vec![30, 218, 2],
            vec![90, 106, 7],
            vec![9, 216, 5],
            vec![204, 151, 4],
            vec![213, 97, 8],
            vec![253, 78, 8],
            vec![91, 39, 4],
            vec![191, 175, 4],
            vec![170, 232, 3],
            vec![233, 2, 5],
            vec![35, 22, 6],
            vec![217, 198, 7],
        ],
        &[
            (vec![156, 162, 3], 11),
            (vec![248, 1, 7], 10),
            (vec![178, 121, 1], 10),
            (vec![36, 97, 6], 9),
            (vec![46, 27, 2], 9),
            (vec![64, 75, 6], 9),
            (vec![135, 80, 5], 9),
            (vec![215, 11, 3], 9),
            (vec![39, 178, 7], 9),
            (vec![75, 164, 6], 9),
        ],
        (vec![127, 197, 7], None),
    );
    random_fixed_length_vecs_2_inputs_helper(
        &|seed| random_fixed_length_vecs_from_single(2, random_primitive_ints::<u8>(seed)),
        &|seed| random_fixed_length_vecs_from_single(3, random_primitive_ints::<u8>(seed)),
        &[0, 1, 0],
        &[
            vec![vec![85, 11], vec![98, 168, 198], vec![136, 200]],
            vec![vec![235, 134], vec![40, 20, 252], vec![203, 223]],
            vec![vec![38, 235], vec![47, 87, 132], vec![217, 177]],
            vec![vec![162, 32], vec![72, 77, 63], vec![166, 234]],
            vec![vec![30, 218], vec![91, 108, 127], vec![90, 106]],
            vec![vec![9, 216], vec![53, 141, 84], vec![204, 151]],
            vec![vec![213, 97], vec![18, 10, 112], vec![253, 78]],
            vec![vec![91, 39], vec![154, 104, 53], vec![191, 175]],
            vec![vec![170, 232], vec![75, 238, 149], vec![233, 2]],
            vec![vec![35, 22], vec![190, 51, 147], vec![217, 198]],
            vec![vec![114, 17], vec![100, 114, 140], vec![32, 173]],
            vec![vec![114, 65], vec![2, 63, 189], vec![121, 222]],
            vec![vec![173, 25], vec![222, 67, 119], vec![144, 148]],
            vec![vec![79, 115], vec![0, 223, 5], vec![52, 73]],
            vec![vec![69, 137], vec![236, 232, 50], vec![91, 153]],
            vec![vec![178, 112], vec![44, 241, 21], vec![34, 95]],
            vec![vec![106, 167], vec![22, 94, 27], vec![197, 130]],
            vec![vec![168, 122], vec![128, 220, 25], vec![207, 172]],
            vec![vec![177, 86], vec![251, 243, 50], vec![150, 221]],
            vec![vec![218, 101], vec![137, 235, 46], vec![115, 74]],
        ],
        &[
            (vec![vec![8, 24], vec![0, 54, 59], vec![5, 3]], 1),
            (vec![vec![8, 72], vec![6, 5, 9], vec![11, 57]], 1),
            (vec![vec![80, 9], vec![84, 9, 10], vec![9, 5]], 1),
            (vec![vec![86, 2], vec![2, 0, 27], vec![49, 4]], 1),
            (vec![vec![0, 2], vec![207, 31, 7], vec![92, 5]], 1),
            (vec![vec![1, 15], vec![51, 5, 47], vec![12, 5]], 1),
            (vec![vec![1, 25], vec![70, 65, 7], vec![3, 66]], 1),
            (vec![vec![1, 72], vec![8, 49, 246], vec![2, 1]], 1),
            (vec![vec![1, 82], vec![86, 3, 70], vec![6, 26]], 1),
            (vec![vec![1, 85], vec![3, 5, 53], vec![14, 92]], 1),
        ],
        (
            vec![vec![128, 20], vec![252, 3, 74], vec![108, 132]],
            Some(vec![vec![128, 21], vec![6, 87, 236], vec![223, 197]]),
        ),
    );
}

#[test]
#[should_panic]
fn random_fixed_length_vecs_2_inputs_fail_1() {
    random_fixed_length_vecs_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[],
    );
}

#[test]
#[should_panic]
fn random_fixed_length_vecs_2_inputs_fail_2() {
    random_fixed_length_vecs_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[0],
    );
}

#[test]
#[should_panic]
fn random_fixed_length_vecs_2_inputs_fail_3() {
    random_fixed_length_vecs_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[1],
    );
}

#[test]
#[should_panic]
fn random_fixed_length_vecs_2_inputs_fail_4() {
    random_fixed_length_vecs_2_inputs(
        EXAMPLE_SEED,
        &|seed| random_unsigned_range::<u32>(seed, 0, 10),
        &|seed| random_unsigned_range(seed, 0, 5),
        &[0, 1, 2],
    );
}
