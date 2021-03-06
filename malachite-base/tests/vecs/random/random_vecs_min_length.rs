use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::random::random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::tuples::random::random_units;
use malachite_base::vecs::random::random_vecs_min_length;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use std::fmt::Debug;

fn random_vecs_min_length_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_vecs_min_length(
        EXAMPLE_SEED,
        min_length,
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
fn test_random_vecs_min_length() {
    random_vecs_min_length_helper(
        0,
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
    random_vecs_min_length_helper(
        3,
        &|_| random_units(),
        7,
        1,
        &[
            vec![(), (), ()],
            vec![
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
                (),
            ],
            vec![(), (), (), (), (), (), ()],
            vec![(), (), (), (), (), (), ()],
            vec![(), (), (), ()],
            vec![(), (), ()],
            vec![(), (), (), (), (), (), (), ()],
            vec![(), (), (), (), ()],
            vec![(), (), (), (), (), (), ()],
            vec![(), (), ()],
            vec![(), (), (), (), (), (), (), (), ()],
            vec![(), (), ()],
            vec![(), (), ()],
            vec![(), (), (), (), (), (), (), (), (), (), (), (), (), ()],
            vec![(), (), (), (), (), (), (), (), (), (), ()],
            vec![(), (), ()],
            vec![(), (), (), (), (), ()],
            vec![(), (), ()],
            vec![(), (), (), (), (), (), (), ()],
            vec![(), (), (), (), (), (), (), (), (), (), (), ()],
        ],
        &[
            (vec![(), (), ()], 199913),
            (vec![(), (), (), ()], 160173),
            (vec![(), (), (), (), ()], 128173),
            (vec![(), (), (), (), (), ()], 102460),
            (vec![(), (), (), (), (), (), ()], 81463),
            (vec![(), (), (), (), (), (), (), ()], 65695),
            (vec![(), (), (), (), (), (), (), (), ()], 52495),
            (vec![(), (), (), (), (), (), (), (), (), ()], 41943),
            (vec![(), (), (), (), (), (), (), (), (), (), ()], 33396),
            (vec![(), (), (), (), (), (), (), (), (), (), (), ()], 27035),
        ],
        (vec![(), (), (), (), (), ()], None),
    );
    random_vecs_min_length_helper(
        0,
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
    random_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        7,
        1,
        &[
            vec![85, 11, 136],
            vec![
                200, 235, 134, 203, 223, 38, 235, 217, 177, 162, 32, 166, 234, 30, 218, 90, 106,
            ],
            vec![9, 216, 204, 151, 213, 97, 253],
            vec![78, 91, 39, 191, 175, 170, 232],
            vec![233, 2, 35, 22],
            vec![217, 198, 114],
            vec![17, 32, 173, 114, 65, 121, 222, 173],
            vec![25, 144, 148, 79, 115],
            vec![52, 73, 69, 137, 91, 153, 178],
            vec![112, 34, 95],
            vec![106, 167, 197, 130, 168, 122, 207, 172, 177],
            vec![86, 150, 221],
            vec![218, 101, 115],
            vec![
                74, 9, 123, 109, 52, 201, 159, 247, 250, 48, 133, 235, 196, 40,
            ],
            vec![97, 104, 68, 190, 216, 7, 216, 157, 43, 43, 112],
            vec![217, 24, 11],
            vec![103, 211, 84, 135, 55, 29],
            vec![206, 89, 65],
            vec![191, 51, 9, 79, 148, 34, 22, 22],
            vec![62, 3, 114, 118, 20, 47, 194, 50, 32, 120, 176, 166],
        ],
        &[
            (vec![65, 71, 68], 3),
            (vec![34, 39, 234], 3),
            (vec![207, 218, 62], 3),
            (vec![89, 175, 228], 3),
            (vec![198, 166, 242], 3),
            (vec![2, 94, 4], 2),
            (vec![3, 91, 9], 2),
            (vec![6, 3, 61], 2),
            (vec![0, 18, 20], 2),
            (vec![1, 48, 93], 2),
        ],
        (
            vec![128, 11, 100, 4, 101, 167, 125],
            Some(vec![128, 11, 104]),
        ),
    );
    random_vecs_min_length_helper(
        0,
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
    random_vecs_min_length_helper(
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 2, 1),
        7,
        1,
        &[
            vec![5, 0, 0],
            vec![1, 1, 1, 4, 2, 4, 6, 2, 2, 0, 1, 9, 13, 0, 0, 2, 0],
            vec![7, 4, 6, 7, 6, 0, 0],
            vec![0, 1, 3, 5, 1, 2, 1],
            vec![0, 0, 1, 4],
            vec![2, 0, 12],
            vec![0, 0, 2, 3, 1, 1, 1, 2],
            vec![3, 3, 9, 1, 0],
            vec![2, 1, 11, 1, 0, 1, 6],
            vec![0, 3, 18],
            vec![3, 3, 0, 5, 1, 2, 5, 0, 0],
            vec![2, 3, 1],
            vec![0, 2, 1],
            vec![2, 3, 1, 3, 3, 0, 0, 7, 2, 0, 0, 4, 2, 1],
            vec![0, 3, 1, 4, 2, 2, 1, 5, 0, 2, 0],
            vec![2, 0, 0],
            vec![1, 4, 1, 3, 3, 1],
            vec![0, 0, 0],
            vec![1, 4, 2, 1, 1, 0, 1, 0],
            vec![0, 0, 1, 0, 2, 0, 7, 0, 0, 0, 2, 12],
        ],
        &[
            (vec![0, 0, 0], 7500),
            (vec![1, 0, 0], 5048),
            (vec![0, 0, 1], 4956),
            (vec![0, 1, 0], 4876),
            (vec![0, 0, 2], 3336),
            (vec![0, 2, 0], 3320),
            (vec![1, 1, 0], 3305),
            (vec![2, 0, 0], 3303),
            (vec![0, 1, 1], 3242),
            (vec![1, 0, 1], 3239),
        ],
        (
            vec![1, 3, 1, 1, 4, 0, 2, 3],
            Some(vec![1, 3, 1, 1, 4, 0, 4, 0, 0]),
        ),
    );
    random_vecs_min_length_helper(
        0,
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
    random_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        13,
        4,
        &[
            vec![85, 11, 136],
            vec![200, 235, 134],
            vec![203, 223, 38, 235],
            vec![217, 177, 162, 32],
            vec![166, 234, 30, 218, 90],
            vec![106, 9, 216],
            vec![204, 151, 213],
            vec![97, 253, 78],
            vec![91, 39, 191],
            vec![175, 170, 232],
            vec![233, 2, 35],
            vec![22, 217, 198],
            vec![114, 17, 32, 173, 114],
            vec![65, 121, 222, 173],
            vec![25, 144, 148],
            vec![79, 115, 52, 73, 69],
            vec![137, 91, 153],
            vec![178, 112, 34],
            vec![95, 106, 167],
            vec![197, 130, 168],
        ],
        &[
            (vec![34, 248, 94], 4),
            (vec![155, 89, 108], 4),
            (vec![232, 167, 146], 4),
            (vec![255, 244, 186], 4),
            (vec![0, 70, 96], 3),
            (vec![18, 1, 93], 3),
            (vec![30, 0, 90], 3),
            (vec![50, 3, 37], 3),
            (vec![82, 1, 99], 3),
            (vec![9, 42, 49], 3),
        ],
        (vec![127, 228, 182], Some(vec![127, 228, 193])),
    );
    random_vecs_min_length_helper(
        0,
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
    random_vecs_min_length_helper(
        3,
        &|seed| random_char_inclusive_range(seed, 'a', 'z'),
        7,
        1,
        &[
            vec!['v', 'c', 'q'],
            vec![
                'i', 'e', 'p', 'g', 's', 'n', 't', 'm', 'z', 'o', 'm', 'f', 'k', 'q', 'y', 'u', 'k',
            ],
            vec!['x', 'h', 'u', 'n', 'n', 'j', 'n'],
            vec!['j', 'a', 'w', 'z', 'l', 'w', 'b'],
            vec!['l', 'u', 'n', 'e'],
            vec!['l', 'v', 'k'],
            vec!['u', 'h', 'c', 'y', 'i', 'm', 'r', 'm'],
            vec!['y', 's', 'l', 'e', 'a'],
            vec!['s', 'w', 'k', 'o', 'b', 'k', 'w'],
            vec!['g', 'd', 'q'],
            vec!['e', 'f', 'u', 'z', 'r', 'g', 'j', 'k', 'r'],
            vec!['s', 'y', 'n'],
            vec!['f', 't', 's'],
            vec![
                'f', 'e', 's', 'p', 'j', 'n', 'h', 'n', 'r', 'f', 'i', 'u', 'k', 'p',
            ],
            vec!['p', 'g', 'l', 'd', 'l', 'l', 'z', 's', 'w', 'w', 'l'],
            vec!['w', 'z', 'j'],
            vec!['j', 'j', 'y', 'g', 'e', 'z'],
            vec!['v', 'p', 'y'],
            vec!['u', 'q', 'l', 'h', 'r', 'r', 's', 'q'],
            vec!['b', 'n', 'e', 's', 'p', 'r', 'd', 'a', 'k', 'w', 'c', 'y'],
        ],
        &[
            (vec!['b', 'c', 'j'], 25),
            (vec!['e', 'k', 'd'], 25),
            (vec!['a', 'x', 'n'], 24),
            (vec!['b', 'e', 'z'], 24),
            (vec!['c', 'c', 'b'], 24),
            (vec!['d', 'g', 'h'], 24),
            (vec!['g', 'l', 'i'], 24),
            (vec!['i', 'w', 'n'], 24),
            (vec!['j', 'd', 'w'], 24),
            (vec!['m', 'y', 'a'], 24),
        ],
        (
            vec!['m', 'z', 'z', 'r', 'e', 'r'],
            Some(vec!['m', 'z', 'z', 'r', 'g', 'i']),
        ),
    );
}

#[test]
#[should_panic]
fn random_vecs_min_length_fail_1() {
    random_vecs_min_length(EXAMPLE_SEED, 3, &random_primitive_ints::<u32>, 3, 1);
}

#[test]
#[should_panic]
fn random_vecs_min_length_fail_2() {
    random_vecs_min_length(EXAMPLE_SEED, 1, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_vecs_min_length_fail_3() {
    random_vecs_min_length(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
