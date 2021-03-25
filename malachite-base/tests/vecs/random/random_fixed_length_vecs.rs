use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::{random_primitive_ints, random_unsigned_range};
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::vecs::random::{
    random_fixed_length_vecs_from_single, random_length_2_vecs, random_length_3_vecs,
};
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use std::fmt::Debug;

fn random_length_2_vecs_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_length_2_vecs(EXAMPLE_SEED, xs_gen, ys_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_length_2_vecs() {
    random_length_2_vecs_helper(
        &random_primitive_ints::<u8>,
        &|seed| random_unsigned_range(seed, 0, 10),
        &[
            vec![85, 2],
            vec![11, 6],
            vec![136, 8],
            vec![200, 6],
            vec![235, 8],
            vec![134, 2],
            vec![203, 4],
            vec![223, 1],
            vec![38, 2],
            vec![235, 7],
            vec![217, 5],
            vec![177, 4],
            vec![162, 8],
            vec![32, 8],
            vec![166, 4],
            vec![234, 4],
            vec![30, 3],
            vec![218, 5],
            vec![90, 6],
            vec![106, 7],
        ],
        &[
            (vec![196, 6], 466),
            (vec![162, 5], 457),
            (vec![132, 5], 455),
            (vec![200, 2], 455),
            (vec![61, 3], 454),
            (vec![117, 5], 453),
            (vec![28, 0], 446),
            (vec![148, 5], 446),
            (vec![194, 9], 446),
            (vec![44, 3], 444),
        ],
        (vec![127, 9], None),
    );
    random_length_2_vecs_helper(
        &|seed| random_fixed_length_vecs_from_single(2, random_primitive_ints::<u8>(seed)),
        &|seed| random_fixed_length_vecs_from_single(3, random_primitive_ints::<u8>(seed)),
        &[
            vec![vec![85, 11], vec![98, 168, 198]],
            vec![vec![136, 200], vec![40, 20, 252]],
            vec![vec![235, 134], vec![47, 87, 132]],
            vec![vec![203, 223], vec![72, 77, 63]],
            vec![vec![38, 235], vec![91, 108, 127]],
            vec![vec![217, 177], vec![53, 141, 84]],
            vec![vec![162, 32], vec![18, 10, 112]],
            vec![vec![166, 234], vec![154, 104, 53]],
            vec![vec![30, 218], vec![75, 238, 149]],
            vec![vec![90, 106], vec![190, 51, 147]],
            vec![vec![9, 216], vec![100, 114, 140]],
            vec![vec![204, 151], vec![2, 63, 189]],
            vec![vec![213, 97], vec![222, 67, 119]],
            vec![vec![253, 78], vec![0, 223, 5]],
            vec![vec![91, 39], vec![236, 232, 50]],
            vec![vec![191, 175], vec![44, 241, 21]],
            vec![vec![170, 232], vec![22, 94, 27]],
            vec![vec![233, 2], vec![128, 220, 25]],
            vec![vec![35, 22], vec![251, 243, 50]],
            vec![vec![217, 198], vec![137, 235, 46]],
        ],
        &[
            (vec![vec![0, 5], vec![6, 7, 42]], 1),
            (vec![vec![8, 8], vec![18, 5, 6]], 1),
            (vec![vec![9, 1], vec![5, 3, 23]], 1),
            (vec![vec![0, 0], vec![97, 7, 73]], 1),
            (vec![vec![0, 2], vec![12, 20, 6]], 1),
            (vec![vec![0, 99], vec![20, 8, 6]], 1),
            (vec![vec![1, 81], vec![3, 21, 3]], 1),
            (vec![vec![1, 9], vec![219, 9, 7]], 1),
            (vec![vec![1, 9], vec![4, 95, 15]], 1),
            (vec![vec![15, 2], vec![56, 0, 8]], 1),
        ],
        (
            vec![vec![127, 197], vec![162, 123, 217]],
            Some(vec![vec![127, 197], vec![163, 170, 161]]),
        ),
    );
}

fn random_length_3_vecs_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
    J: Clone + Iterator<Item = T>,
    K: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    ys_gen: &dyn Fn(Seed) -> J,
    zs_gen: &dyn Fn(Seed) -> K,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_length_3_vecs(EXAMPLE_SEED, xs_gen, ys_gen, zs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_length_3_vecs() {
    random_length_3_vecs_helper(
        &|seed| random_char_inclusive_range(seed, 'a', 'c'),
        &|seed| random_char_inclusive_range(seed, 'd', 'f'),
        &|seed| random_char_inclusive_range(seed, 'g', 'i'),
        &[
            vec!['b', 'f', 'g'],
            vec!['b', 'd', 'i'],
            vec!['b', 'f', 'i'],
            vec!['b', 'e', 'i'],
            vec!['c', 'd', 'i'],
            vec!['a', 'f', 'i'],
            vec!['a', 'f', 'g'],
            vec!['a', 'f', 'g'],
            vec!['c', 'f', 'i'],
            vec!['a', 'e', 'i'],
            vec!['c', 'd', 'h'],
            vec!['a', 'd', 'h'],
            vec!['c', 'f', 'i'],
            vec!['a', 'f', 'i'],
            vec!['c', 'd', 'g'],
            vec!['c', 'd', 'h'],
            vec!['c', 'e', 'g'],
            vec!['b', 'e', 'h'],
            vec!['a', 'd', 'g'],
            vec!['c', 'd', 'g'],
        ],
        &[
            (vec!['b', 'f', 'i'], 37416),
            (vec!['a', 'f', 'g'], 37345),
            (vec!['c', 'd', 'i'], 37278),
            (vec!['b', 'f', 'g'], 37274),
            (vec!['a', 'd', 'h'], 37207),
            (vec!['b', 'f', 'h'], 37188),
            (vec!['b', 'd', 'i'], 37153),
            (vec!['b', 'e', 'g'], 37117),
            (vec!['a', 'd', 'g'], 37092),
            (vec!['c', 'f', 'g'], 37068),
        ],
        (vec!['b', 'e', 'h'], None),
    );
    random_length_3_vecs_helper(
        &|seed| random_fixed_length_vecs_from_single(1, random_primitive_ints::<u8>(seed)),
        &|seed| random_fixed_length_vecs_from_single(2, random_primitive_ints::<u8>(seed)),
        &|seed| random_fixed_length_vecs_from_single(3, random_primitive_ints::<u8>(seed)),
        &[
            vec![vec![85], vec![98, 168], vec![168, 10, 250]],
            vec![vec![11], vec![198, 40], vec![95, 250, 79]],
            vec![vec![136], vec![20, 252], vec![4, 171, 141]],
            vec![vec![200], vec![47, 87], vec![189, 177, 169]],
            vec![vec![235], vec![132, 72], vec![36, 73, 154]],
            vec![vec![134], vec![77, 63], vec![62, 202, 17]],
            vec![vec![203], vec![91, 108], vec![35, 189, 158]],
            vec![vec![223], vec![127, 53], vec![31, 173, 175]],
            vec![vec![38], vec![141, 84], vec![63, 225, 106]],
            vec![vec![235], vec![18, 10], vec![40, 116, 16]],
            vec![vec![217], vec![112, 154], vec![88, 112, 9]],
            vec![vec![177], vec![104, 53], vec![227, 144, 93]],
            vec![vec![162], vec![75, 238], vec![85, 90, 214]],
            vec![vec![32], vec![149, 190], vec![31, 60, 254]],
            vec![vec![166], vec![51, 147], vec![143, 44, 177]],
            vec![vec![234], vec![100, 114], vec![205, 197, 53]],
            vec![vec![30], vec![140, 2], vec![15, 184, 137]],
            vec![vec![218], vec![63, 189], vec![75, 116, 140]],
            vec![vec![90], vec![222, 67], vec![19, 119, 60]],
            vec![vec![106], vec![119, 0], vec![219, 21, 164]],
        ],
        &[
            (vec![vec![0], vec![47, 4], vec![31, 6, 1]], 1),
            (vec![vec![0], vec![5, 12], vec![9, 6, 54]], 1),
            (vec![vec![6], vec![99, 35], vec![3, 2, 3]], 1),
            (vec![vec![7], vec![7, 56], vec![6, 3, 76]], 1),
            (vec![vec![7], vec![9, 5], vec![148, 1, 1]], 1),
            (vec![vec![9], vec![61, 7], vec![9, 60, 8]], 1),
            (vec![vec![0], vec![0, 55], vec![1, 12, 83]], 1),
            (vec![vec![0], vec![1, 57], vec![60, 4, 55]], 1),
            (vec![vec![0], vec![1, 8], vec![235, 0, 27]], 1),
            (vec![vec![0], vec![73, 15], vec![0, 2, 11]], 1),
        ],
        (
            vec![vec![127], vec![241, 129], vec![232, 173, 11]],
            Some(vec![vec![127], vec![241, 149], vec![219, 172, 49]]),
        ),
    );
}
