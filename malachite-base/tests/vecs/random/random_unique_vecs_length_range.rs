use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random::random_unique_vecs_length_range;
use std::fmt::Debug;

fn random_unique_vecs_length_range_helper<
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
    let xs = random_unique_vecs_length_range(EXAMPLE_SEED, a, b, xs_gen);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_unique_vecs_length_range() {
    random_unique_vecs_length_range_helper(
        2,
        4,
        &random_primitive_ints::<u8>,
        &[
            vec![85, 11, 136],
            vec![200, 235],
            vec![134, 203, 223],
            vec![38, 235, 217],
            vec![177, 162],
            vec![32, 166, 234],
            vec![30, 218, 90],
            vec![106, 9],
            vec![216, 204, 151],
            vec![213, 97, 253],
            vec![78, 91],
            vec![39, 191, 175],
            vec![170, 232],
            vec![233, 2, 35],
            vec![22, 217, 198],
            vec![114, 17],
            vec![32, 173],
            vec![114, 65],
            vec![121, 222, 173],
            vec![25, 144],
        ],
        &[
            (vec![149, 194], 23),
            (vec![237, 224], 23),
            (vec![109, 76], 21),
            (vec![187, 29], 21),
            (vec![96, 105], 21),
            (vec![233, 132], 21),
            (vec![25, 96], 20),
            (vec![92, 85], 20),
            (vec![108, 72], 20),
            (vec![128, 48], 20),
        ],
        (vec![127, 247], None),
    );
    random_unique_vecs_length_range_helper(
        2,
        4,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            vec![5, 0, 1],
            vec![1, 4],
            vec![2, 4, 6],
            vec![2, 0, 1],
            vec![9, 13],
            vec![0, 2, 7],
            vec![4, 6, 7],
            vec![6, 0],
            vec![0, 1, 3],
            vec![5, 1, 2],
            vec![1, 0],
            vec![0, 1, 4],
            vec![2, 0],
            vec![12, 0, 2],
            vec![3, 1, 2],
            vec![3, 9],
            vec![1, 0],
            vec![2, 1],
            vec![11, 1, 0],
            vec![1, 6],
        ],
        &[
            (vec![0, 1], 55434),
            (vec![1, 0], 47598),
            (vec![0, 2], 37211),
            (vec![2, 0], 28974),
            (vec![0, 3], 24737),
            (vec![1, 2], 21227),
            (vec![2, 1], 19153),
            (vec![0, 1, 2], 18604),
            (vec![3, 0], 18253),
            (vec![0, 4], 16195),
        ],
        (vec![1, 4], None),
    );
    random_unique_vecs_length_range_helper(
        2,
        4,
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        &[
            vec!['v', 'c', 'q'],
            vec!['i', 'e'],
            vec!['p', 'g', 's'],
            vec!['n', 't', 'm'],
            vec!['z', 'o'],
            vec!['m', 'f', 'k'],
            vec!['q', 'y', 'u'],
            vec!['k', 'x'],
            vec!['h', 'u', 'n'],
            vec!['n', 'j', 'a'],
            vec!['w', 'z'],
            vec!['l', 'w', 'b'],
            vec!['l', 'u'],
            vec!['n', 'e', 'l'],
            vec!['v', 'k', 'u'],
            vec!['h', 'c'],
            vec!['y', 'i'],
            vec!['m', 'r'],
            vec!['m', 'y', 's'],
            vec!['l', 'e'],
        ],
        &[
            (vec!['i', 'p'], 855),
            (vec!['o', 't'], 845),
            (vec!['c', 'i'], 842),
            (vec!['h', 'u'], 841),
            (vec!['x', 'l'], 841),
            (vec!['a', 'o'], 833),
            (vec!['g', 'h'], 833),
            (vec!['z', 'n'], 832),
            (vec!['j', 'n'], 831),
            (vec!['l', 'c'], 829),
        ],
        (vec!['m', 'z', 'l'], None),
    );
}

#[test]
#[should_panic]
fn random_unique_vecs_length_range_fail() {
    random_unique_vecs_length_range(EXAMPLE_SEED, 2, 2, &random_primitive_ints::<u32>);
}
