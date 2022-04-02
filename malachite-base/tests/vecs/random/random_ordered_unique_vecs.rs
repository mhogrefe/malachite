use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random::random_ordered_unique_vecs;
use std::fmt::Debug;

fn random_ordered_unique_vecs_helper<
    T: Clone + Debug + Eq + Hash + Ord,
    I: Clone + Iterator<Item = T>,
>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_ordered_unique_vecs(
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
fn test_random_ordered_unique_vecs() {
    random_ordered_unique_vecs_helper(
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            vec![],
            vec![11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235],
            vec![30, 90, 218, 234],
            vec![9, 106, 204, 216],
            vec![151],
            vec![],
            vec![78, 91, 97, 213, 253],
            vec![39, 191],
            vec![170, 175, 232, 233],
            vec![],
            vec![2, 22, 35, 114, 198, 217],
            vec![],
            vec![],
            vec![17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222],
            vec![52, 69, 73, 91, 115, 137, 153, 178],
            vec![],
            vec![34, 95, 112],
            vec![],
            vec![106, 130, 167, 168, 197],
            vec![86, 101, 122, 150, 172, 177, 207, 218, 221],
        ],
        &[
            (vec![], 199913),
            (vec![7], 705),
            (vec![25], 689),
            (vec![184], 681),
            (vec![213], 681),
            (vec![255], 676),
            (vec![215], 675),
            (vec![54], 673),
            (vec![122], 672),
            (vec![207], 672),
        ],
        (vec![27, 31, 211, 238], Some(vec![27, 31, 247, 251])),
    );
    random_ordered_unique_vecs_helper(
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        4,
        1,
        &[
            vec![],
            vec![1, 9, 12, 14, 16, 17, 19, 21, 41, 42, 68, 79, 124, 141],
            vec![0, 1, 10, 99],
            vec![2, 12, 36, 77],
            vec![1],
            vec![],
            vec![1, 5, 9, 19, 103],
            vec![6, 7],
            vec![15, 18, 51, 159],
            vec![],
            vec![2, 26, 40, 52, 64, 75],
            vec![],
            vec![],
            vec![3, 4, 5, 7, 30, 31, 34, 43, 49, 51, 67],
            vec![1, 14, 16, 24, 29, 41, 47, 52],
            vec![],
            vec![11, 13, 62],
            vec![],
            vec![3, 14, 42, 47, 109],
            vec![5, 13, 16, 25, 37, 41, 42, 86, 96],
        ],
        &[
            (vec![], 199913),
            (vec![0], 4861),
            (vec![1], 4593),
            (vec![2], 4498),
            (vec![3], 4405),
            (vec![4], 4330),
            (vec![5], 4078),
            (vec![6], 4050),
            (vec![7], 3858),
            (vec![8], 3848),
        ],
        (
            vec![3, 9, 14, 22, 36, 56, 107],
            Some(vec![3, 9, 14, 22, 42, 54, 73, 150]),
        ),
    );
    random_ordered_unique_vecs_helper(
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
            vec![134, 235],
            vec![203],
            vec![],
            vec![38, 223],
            vec![],
            vec![],
            vec![],
            vec![],
        ],
        &[
            (vec![], 800023),
            (vec![162], 692),
            (vec![235], 690),
            (vec![90], 688),
            (vec![65], 687),
            (vec![249], 686),
            (vec![175], 684),
            (vec![108], 683),
            (vec![211], 682),
            (vec![237], 680),
        ],
        (vec![], None),
    );
    random_ordered_unique_vecs_helper(
        &|seed| {
            graphic_weighted_random_char_inclusive_range(
                seed,
                'a',
                exhaustive_chars().nth(200).unwrap(),
                1,
                1,
            )
        },
        4,
        1,
        &[
            vec![],
            vec!['g', 'q', '³', '»', 'À', 'Á', 'Ã', 'È', 'á', 'â', 'ì', 'ñ', 'Ā', 'ą'],
            vec!['ª', '´', 'Ã', 'ä'],
            vec!['½', 'Á', 'Ï', 'ý'],
            vec!['j'],
            vec![],
            vec!['u', '½', 'Â', 'Ñ', 'ï'],
            vec!['x', 'õ'],
            vec!['¡', 'Â', 'ù', 'Ċ'],
            vec![],
            vec!['b', 'r', 's', '¬', 'Â', 'Ñ'],
            vec![],
            vec![],
            vec!['j', 'n', 't', '¬', 'º', '¿', 'Á', 'Ø', 'Þ', 'ô', 'ü'],
            vec!['b', 'k', '±', 'Î', 'Ü', 'æ', 'è', 'ā'],
            vec![],
            vec!['«', '¹', 'Î'],
            vec![],
            vec!['~', '¯', '´', 'Ý', 'â'],
            vec!['g', '¼', 'Ç', 'Î', 'Ü', 'Þ', 'æ', 'é', 'ö'],
        ],
        &[
            (vec![], 199913),
            (vec!['Ó'], 1270),
            (vec!['Â'], 1249),
            (vec!['§'], 1244),
            (vec!['¿'], 1243),
            (vec!['õ'], 1241),
            (vec!['ĉ'], 1234),
            (vec!['¤'], 1232),
            (vec!['¼'], 1232),
            (vec!['Ì'], 1229),
        ],
        (
            vec!['o', 'v', '¢', '±', 'Ä', 'Ć'],
            Some(vec!['o', 'v', '¢', '³', 'ã']),
        ),
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_fail_1() {
    random_ordered_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_fail_2() {
    random_ordered_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_fail_3() {
    random_ordered_unique_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
