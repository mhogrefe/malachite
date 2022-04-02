use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random::random_vecs_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use std::fmt::Debug;

fn random_vecs_from_length_iterator_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = u64>,
    J: Clone + Iterator<Item = T>,
>(
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_vecs_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_vecs_from_length_iterator() {
    random_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
        &random_bools,
        &[
            vec![true, false],
            vec![true, false, true, false],
            vec![true, false],
            vec![true, true, false, true],
            vec![false, false, false, false],
            vec![false, false],
            vec![],
            vec![false, true],
            vec![],
            vec![false, false, false, true],
            vec![false, false, false, true],
            vec![false, false],
            vec![true, true, true, true],
            vec![false, true],
            vec![false, true, true, true],
            vec![false, true, true, false],
            vec![false, false, false, true],
            vec![],
            vec![true, true, false, true],
            vec![],
        ],
        &[
            (vec![], 333820),
            (vec![true, false], 83553),
            (vec![false, false], 83348),
            (vec![false, true], 83319),
            (vec![true, true], 82905),
            (vec![true, true, true, false], 21126),
            (vec![false, false, false, true], 20940),
            (vec![false, false, true, false], 20931),
            (vec![true, true, false, true], 20925),
            (vec![true, false, false, false], 20899),
        ],
        (vec![false, false, true, true], None),
    );
    random_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            vec![85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177],
            vec![162, 32],
            vec![166, 234, 30, 218, 90, 106, 9, 216, 204, 151, 213, 97, 253, 78, 91, 39],
            vec![191, 175],
            vec![
                170, 232, 233, 2, 35, 22, 217, 198, 114, 17, 32, 173, 114, 65, 121, 222, 173, 25,
                144, 148, 79, 115, 52, 73, 69, 137, 91, 153,
            ],
            vec![],
            vec![178, 112, 34, 95, 106, 167, 197, 130, 168, 122],
            vec![207, 172, 177, 86, 150, 221, 218, 101],
            vec![115, 74],
            vec![],
            vec![9, 123, 109, 52, 201, 159, 247, 250, 48, 133, 235, 196],
            vec![40, 97, 104, 68],
            vec![],
            vec![],
            vec![190, 216],
            vec![7, 216, 157, 43, 43, 112],
            vec![],
            vec![217, 24],
            vec![],
            vec![11, 103],
        ],
        &[
            (vec![], 333981),
            (vec![198, 47], 13),
            (vec![203, 121], 13),
            (vec![77, 29], 12),
            (vec![97, 58], 12),
            (vec![174, 43], 12),
            (vec![80, 107], 12),
            (vec![100, 118], 12),
            (vec![176, 218], 12),
            (vec![203, 110], 12),
        ],
        (vec![63, 135], None),
    );
}
