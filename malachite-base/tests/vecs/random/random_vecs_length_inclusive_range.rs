use core::hash::Hash;
use std::fmt::Debug;

use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;

use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::tuples::random::random_units;
use malachite_base::vecs::random::random_vecs_length_inclusive_range;

fn random_vecs_length_inclusive_range_helper<
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
    let xs = random_vecs_length_inclusive_range(EXAMPLE_SEED, a, b, xs_gen);
    let values = xs.clone().take(20).collect::<Vec<_>>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_vecs_length_inclusive_range() {
    random_vecs_length_inclusive_range_helper(
        2,
        3,
        &|_| random_units(),
        &[
            vec![(), (), ()],
            vec![(), ()],
            vec![(), (), ()],
            vec![(), (), ()],
            vec![(), ()],
            vec![(), (), ()],
            vec![(), (), ()],
            vec![(), ()],
            vec![(), (), ()],
            vec![(), (), ()],
            vec![(), ()],
            vec![(), (), ()],
            vec![(), ()],
            vec![(), (), ()],
            vec![(), (), ()],
            vec![(), ()],
            vec![(), ()],
            vec![(), ()],
            vec![(), (), ()],
            vec![(), ()],
        ],
        &[(vec![(), (), ()], 500363), (vec![(), ()], 499637)],
        (vec![(), (), ()], None),
    );
    random_vecs_length_inclusive_range_helper(
        2,
        3,
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
            (vec![234, 192], 23),
            (vec![0, 40], 19),
            (vec![68, 88], 19),
            (vec![188, 21], 19),
            (vec![215, 22], 19),
            (vec![221, 92], 19),
            (vec![255, 26], 19),
            (vec![34, 253], 19),
            (vec![61, 159], 19),
            (vec![155, 140], 19),
        ],
        (vec![128, 5, 208], Some(vec![128, 5, 239])),
    );
    random_vecs_length_inclusive_range_helper(
        2,
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        &[
            vec![5, 0, 0],
            vec![1, 1],
            vec![1, 4, 2],
            vec![4, 6, 2],
            vec![2, 0],
            vec![1, 9, 13],
            vec![0, 0, 2],
            vec![0, 7],
            vec![4, 6, 7],
            vec![6, 0, 0],
            vec![0, 1],
            vec![3, 5, 1],
            vec![2, 1],
            vec![0, 0, 1],
            vec![4, 2, 0],
            vec![12, 0],
            vec![0, 2],
            vec![3, 1],
            vec![1, 1, 2],
            vec![3, 3],
        ],
        &[
            (vec![0, 0], 55357),
            (vec![0, 1], 37179),
            (vec![1, 0], 37106),
            (vec![0, 2], 24784),
            (vec![2, 0], 24772),
            (vec![1, 1], 24686),
            (vec![0, 0, 0], 18703),
            (vec![3, 0], 16656),
            (vec![2, 1], 16622),
            (vec![1, 2], 16275),
        ],
        (vec![1, 3], None),
    );
    random_vecs_length_inclusive_range_helper(
        2,
        3,
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
            vec!['n', 'j', 'n'],
            vec!['j', 'a'],
            vec!['w', 'z', 'l'],
            vec!['w', 'b'],
            vec!['l', 'u', 'n'],
            vec!['e', 'l', 'v'],
            vec!['k', 'u'],
            vec!['h', 'c'],
            vec!['y', 'i'],
            vec!['m', 'r', 'm'],
            vec!['y', 's'],
        ],
        &[
            (vec!['w', 'o'], 822),
            (vec!['f', 's'], 814),
            (vec!['w', 'u'], 810),
            (vec!['g', 'c'], 806),
            (vec!['w', 'f'], 806),
            (vec!['m', 'z'], 805),
            (vec!['q', 'k'], 805),
            (vec!['i', 'b'], 802),
            (vec!['u', 'k'], 800),
            (vec!['h', 'p'], 798),
        ],
        (vec!['m', 'z', 'w'], None),
    );
}

#[test]
#[should_panic]
fn random_vecs_length_inclusive_range_fail() {
    random_vecs_length_inclusive_range(EXAMPLE_SEED, 2, 1, &random_primitive_ints::<u32>);
}
