use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::vecs::random::random_ordered_unique_vecs_length_range;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use std::fmt::Debug;

fn random_ordered_unique_vecs_length_range_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_ordered_unique_vecs_length_range(EXAMPLE_SEED, a, b, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_ordered_unique_vecs_length_range() {
    random_ordered_unique_vecs_length_range_helper(
        2,
        4,
        &random_primitive_ints::<u8>,
        &[
            vec![11, 85, 136],
            vec![200, 235],
            vec![134, 203, 223],
            vec![38, 217, 235],
            vec![162, 177],
            vec![32, 166, 234],
            vec![30, 90, 218],
            vec![9, 106],
            vec![151, 204, 216],
            vec![97, 213, 253],
            vec![78, 91],
            vec![39, 175, 191],
            vec![170, 232],
            vec![2, 35, 233],
            vec![22, 198, 217],
            vec![17, 114],
            vec![32, 173],
            vec![65, 114],
            vec![121, 173, 222],
            vec![25, 144],
        ],
        &[
            (vec![106, 108], 34),
            (vec![224, 237], 34),
            (vec![51, 132], 32),
            (vec![82, 117], 32),
            (vec![72, 108], 31),
            (vec![142, 194], 31),
            (vec![0, 34], 30),
            (vec![12, 208], 30),
            (vec![15, 141], 30),
            (vec![30, 248], 30),
        ],
        (vec![62, 131, 203], Some(vec![62, 131, 205])),
    );
    random_ordered_unique_vecs_length_range_helper(
        2,
        4,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            vec![0, 1, 5],
            vec![1, 4],
            vec![2, 4, 6],
            vec![0, 1, 2],
            vec![9, 13],
            vec![0, 2, 7],
            vec![4, 6, 7],
            vec![0, 6],
            vec![0, 1, 3],
            vec![1, 2, 5],
            vec![0, 1],
            vec![0, 1, 4],
            vec![0, 2],
            vec![0, 2, 12],
            vec![1, 2, 3],
            vec![3, 9],
            vec![0, 1],
            vec![1, 2],
            vec![0, 1, 11],
            vec![1, 6],
        ],
        &[
            (vec![0, 1], 103032),
            (vec![0, 1, 2], 84142),
            (vec![0, 2], 66185),
            (vec![0, 1, 3], 52638),
            (vec![0, 3], 42990),
            (vec![1, 2], 40380),
            (vec![0, 1, 4], 33815),
            (vec![0, 2, 3], 31257),
            (vec![0, 4], 28088),
            (vec![1, 3], 26214),
        ],
        (vec![0, 3], None),
    );
    random_ordered_unique_vecs_length_range_helper(
        2,
        4,
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &[
            vec!['c', 'q', 'v'],
            vec!['e', 'i'],
            vec!['g', 'p', 's'],
            vec!['m', 'n', 't'],
            vec!['o', 'z'],
            vec!['f', 'k', 'm'],
            vec!['q', 'u', 'y'],
            vec!['k', 'x'],
            vec!['h', 'n', 'u'],
            vec!['a', 'j', 'n'],
            vec!['w', 'z'],
            vec!['b', 'l', 'w'],
            vec!['l', 'u'],
            vec!['e', 'l', 'n'],
            vec!['k', 'u', 'v'],
            vec!['c', 'h'],
            vec!['i', 'y'],
            vec!['m', 'r'],
            vec!['m', 's', 'y'],
            vec!['e', 'l'],
        ],
        &[
            (vec!['l', 'x'], 1640),
            (vec!['o', 't'], 1636),
            (vec!['b', 'p'], 1630),
            (vec!['m', 'v'], 1623),
            (vec!['h', 'u'], 1621),
            (vec!['a', 'x'], 1614),
            (vec!['d', 'f'], 1613),
            (vec!['e', 'r'], 1613),
            (vec!['o', 'p'], 1612),
            (vec!['c', 'i'], 1611),
        ],
        (vec!['g', 'j'], None),
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_length_range_fail() {
    random_ordered_unique_vecs_length_range(EXAMPLE_SEED, 2, 2, &random_primitive_ints::<u32>);
}
