use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::tuples::random::random_units;
use malachite_base::vecs::random::random_vecs;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use std::fmt::Debug;

fn random_vecs_helper<T: Clone + Debug + Eq + Hash + Ord, I: Clone + Iterator<Item = T>>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_vecs(
        EXAMPLE_SEED,
        xs_gen,
        mean_length_numerator,
        mean_length_denominator,
    );
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_vecs() {
    random_vecs_helper(
        &|_| random_units(),
        4,
        1,
        &[
            vec![],
            vec![(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            vec![(), (), (), ()],
            vec![(), (), (), ()],
            vec![()],
            vec![],
            vec![(), (), (), (), ()],
            vec![(), ()],
            vec![(), (), (), ()],
            vec![],
            vec![(), (), (), (), (), ()],
            vec![],
            vec![],
            vec![(), (), (), (), (), (), (), (), (), (), ()],
            vec![(), (), (), (), (), (), (), ()],
            vec![],
            vec![(), (), ()],
            vec![],
            vec![(), (), (), (), ()],
            vec![(), (), (), (), (), (), (), (), ()],
        ],
        &[
            (vec![], 199913),
            (vec![()], 160173),
            (vec![(), ()], 128173),
            (vec![(), (), ()], 102460),
            (vec![(), (), (), ()], 81463),
            (vec![(), (), (), (), ()], 65695),
            (vec![(), (), (), (), (), ()], 52495),
            (vec![(), (), (), (), (), (), ()], 41943),
            (vec![(), (), (), (), (), (), (), ()], 33396),
            (vec![(), (), (), (), (), (), (), (), ()], 27035),
        ],
        (vec![(), (), ()], None),
    );
    random_vecs_helper(
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            vec![],
            vec![
                85, 11, 136, 200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32,
            ],
            vec![166, 234, 30, 218],
            vec![90, 106, 9, 216],
            vec![204],
            vec![],
            vec![151, 213, 97, 253, 78],
            vec![91, 39],
            vec![191, 175, 170, 232],
            vec![],
            vec![233, 2, 35, 22, 217, 198],
            vec![],
            vec![],
            vec![114, 17, 32, 173, 114, 65, 121, 222, 173, 25, 144],
            vec![148, 79, 115, 52, 73, 69, 137, 91],
            vec![],
            vec![153, 178, 112],
            vec![],
            vec![34, 95, 106, 167, 197],
            vec![130, 168, 122, 207, 172, 177, 86, 150, 221],
        ],
        &[
            (vec![], 199913),
            (vec![146], 693),
            (vec![26], 692),
            (vec![185], 688),
            (vec![58], 683),
            (vec![196], 683),
            (vec![81], 678),
            (vec![229], 675),
            (vec![192], 673),
            (vec![233], 673),
        ],
        (vec![96], None),
    );
    random_vecs_helper(
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        4,
        1,
        &[
            vec![],
            vec![5, 0, 0, 1, 1, 1, 4, 2, 4, 6, 2, 2, 0, 1],
            vec![9, 13, 0, 0],
            vec![2, 0, 7, 4],
            vec![6],
            vec![],
            vec![7, 6, 0, 0, 0],
            vec![1, 3],
            vec![5, 1, 2, 1],
            vec![],
            vec![0, 0, 1, 4, 2, 0],
            vec![],
            vec![],
            vec![12, 0, 0, 2, 3, 1, 1, 1, 2, 3, 3],
            vec![9, 1, 0, 2, 1, 11, 1, 0],
            vec![],
            vec![1, 6, 0],
            vec![],
            vec![3, 18, 3, 3, 0],
            vec![5, 1, 2, 5, 0, 0, 2, 3, 1],
        ],
        &[
            (vec![], 199913),
            (vec![0], 53462),
            (vec![1], 35352),
            (vec![2], 23512),
            (vec![3], 16118),
            (vec![0, 0], 14371),
            (vec![4], 10594),
            (vec![0, 1], 9566),
            (vec![1, 0], 9409),
            (vec![5], 7157),
        ],
        (vec![1], None),
    );
    random_vecs_helper(
        &random_primitive_ints::<u8>,
        1,
        4,
        &[
            vec![],
            vec![],
            vec![85],
            vec![11],
            vec![136, 200],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![235, 134],
            vec![203],
            vec![],
            vec![223, 38],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
        &[
            (vec![], 800023),
            (vec![8], 704),
            (vec![162], 691),
            (vec![81], 690),
            (vec![211], 690),
            (vec![108], 688),
            (vec![235], 688),
            (vec![35], 687),
            (vec![65], 682),
            (vec![208], 679),
        ],
        (vec![], None),
    );
    random_vecs_helper(
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        4,
        1,
        &[
            vec![],
            vec![
                'v', 'c', 'q', 'i', 'e', 'p', 'g', 's', 'n', 't', 'm', 'z', 'o', 'm',
            ],
            vec!['f', 'k', 'q', 'y'],
            vec!['u', 'k', 'x', 'h'],
            vec!['u'],
            vec![],
            vec!['n', 'n', 'j', 'n', 'j'],
            vec!['a', 'w'],
            vec!['z', 'l', 'w', 'b'],
            vec![],
            vec!['l', 'u', 'n', 'e', 'l', 'v'],
            vec![],
            vec![],
            vec!['k', 'u', 'h', 'c', 'y', 'i', 'm', 'r', 'm', 'y', 's'],
            vec!['l', 'e', 'a', 's', 'w', 'k', 'o', 'b'],
            vec![],
            vec!['k', 'w', 'g'],
            vec![],
            vec!['d', 'q', 'e', 'f', 'u'],
            vec!['z', 'r', 'g', 'j', 'k', 'r', 's', 'y', 'n'],
        ],
        &[
            (vec![], 199913),
            (vec!['o'], 6313),
            (vec!['y'], 6262),
            (vec!['q'], 6261),
            (vec!['j'], 6245),
            (vec!['p'], 6244),
            (vec!['g'], 6219),
            (vec!['x'], 6215),
            (vec!['e'], 6200),
            (vec!['t'], 6188),
        ],
        (vec!['j', 's', 'z'], None),
    );
}

#[test]
#[should_panic]
fn random_vecs_fail_1() {
    random_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_vecs_fail_2() {
    random_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_vecs_fail_3() {
    random_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
