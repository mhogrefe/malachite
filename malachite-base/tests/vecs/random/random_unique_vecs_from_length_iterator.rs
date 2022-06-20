use core::hash::Hash;
use itertools::Itertools;
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random::random_unique_vecs_from_length_iterator;
use malachite_base::vecs::random_values_from_vec;
use std::fmt::Debug;

fn random_unique_vecs_from_length_iterator_helper<
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
    let xs = random_unique_vecs_from_length_iterator(EXAMPLE_SEED, lengths_gen, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_unique_vecs_from_length_iterator() {
    random_unique_vecs_from_length_iterator_helper(
        &|seed| random_values_from_vec(seed, vec![0, 2]),
        &random_bools,
        &[
            vec![true, false],
            vec![],
            vec![true, false],
            vec![true, false],
            vec![],
            vec![true, false],
            vec![true, false],
            vec![],
            vec![true, false],
            vec![false, true],
            vec![],
            vec![false, true],
            vec![],
            vec![false, true],
            vec![false, true],
            vec![],
            vec![],
            vec![],
            vec![true, false],
            vec![],
        ],
        &[(vec![], 499637), (vec![false, true], 250413), (vec![true, false], 249950)],
        (vec![false, true], None),
    );
    random_unique_vecs_from_length_iterator_helper(
        &|seed| geometric_random_unsigneds::<u64>(seed, 2, 1).map(|x| x << 1),
        &random_primitive_ints::<u8>,
        &[
            vec![85, 11, 136, 200, 235, 134, 203, 223, 38, 217, 177, 162],
            vec![32, 166],
            vec![234, 30, 218, 90, 106, 9, 216, 204, 151, 213, 97, 253, 78, 91, 39, 191],
            vec![175, 170],
            vec![
                232, 233, 2, 35, 22, 217, 198, 114, 17, 32, 173, 65, 121, 222, 25, 144, 148, 79,
                115, 52, 73, 69, 137, 91, 153, 178, 112, 34,
            ],
            vec![],
            vec![95, 106, 167, 197, 130, 168, 122, 207, 172, 177],
            vec![86, 150, 221, 218, 101, 115, 74, 9],
            vec![123, 109],
            vec![],
            vec![52, 201, 159, 247, 250, 48, 133, 235, 196, 40, 97, 104],
            vec![68, 190, 216, 7],
            vec![],
            vec![],
            vec![216, 157],
            vec![43, 112, 217, 24, 11, 103],
            vec![],
            vec![211, 84],
            vec![],
            vec![135, 55],
        ],
        &[
            (vec![], 333981),
            (vec![79, 76], 14),
            (vec![234, 129], 14),
            (vec![119, 62], 13),
            (vec![33, 163], 13),
            (vec![5, 42], 12),
            (vec![28, 91], 12),
            (vec![55, 25], 12),
            (vec![152, 55], 12),
            (vec![224, 77], 12),
        ],
        (
            vec![63, 197, 169, 69, 240, 201],
            Some(vec![63, 197, 181, 249]),
        ),
    );
}
