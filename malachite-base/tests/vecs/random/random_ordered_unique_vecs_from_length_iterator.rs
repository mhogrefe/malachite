use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::vecs::random::random_ordered_unique_vecs_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use std::fmt::Debug;

fn random_ordered_unique_vecs_from_length_iterator_helper<
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
    let xs = random_ordered_unique_vecs_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_ordered_unique_vecs_from_length_iterator() {
    random_ordered_unique_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2]),
        &random_bools,
        &[
            vec![false, true],
            vec![],
            vec![false, true],
            vec![false, true],
            vec![],
            vec![false, true],
            vec![false, true],
            vec![],
            vec![false, true],
            vec![false, true],
            vec![],
            vec![false, true],
            vec![],
            vec![false, true],
            vec![false, true],
            vec![],
            vec![],
            vec![],
            vec![false, true],
            vec![],
        ],
        &[(vec![false, true], 500363), (vec![], 499637)],
        (vec![false, true], None),
    );
    random_ordered_unique_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            vec![11, 38, 85, 134, 136, 162, 177, 200, 203, 217, 223, 235],
            vec![32, 166],
            vec![9, 30, 39, 78, 90, 91, 97, 106, 151, 191, 204, 213, 216, 218, 234, 253],
            vec![170, 175],
            vec![
                2, 17, 22, 25, 32, 34, 35, 52, 65, 69, 73, 79, 91, 112, 114, 115, 121, 137, 144,
                148, 153, 173, 178, 198, 217, 222, 232, 233,
            ],
            vec![],
            vec![95, 106, 122, 130, 167, 168, 172, 177, 197, 207],
            vec![9, 74, 86, 101, 115, 150, 218, 221],
            vec![109, 123],
            vec![],
            vec![40, 48, 52, 97, 104, 133, 159, 196, 201, 235, 247, 250],
            vec![7, 68, 190, 216],
            vec![],
            vec![],
            vec![157, 216],
            vec![11, 24, 43, 103, 112, 217],
            vec![],
            vec![84, 211],
            vec![],
            vec![55, 135],
        ],
        &[
            (vec![], 333981),
            (vec![33, 163], 22),
            (vec![76, 233], 19),
            (vec![5, 42], 18),
            (vec![76, 79], 18),
            (vec![32, 134], 18),
            (vec![69, 234], 18),
            (vec![74, 164], 18),
            (vec![86, 192], 18),
            (vec![99, 145], 18),
        ],
        (vec![12, 190], None),
    );
}
