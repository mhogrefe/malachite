use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random::random_unique_vecs;
use std::fmt::Debug;

fn random_unique_vecs_helper<T: Clone + Debug + Eq + Hash + Ord, I: Clone + Iterator<Item = T>>(
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[Vec<T>],
    expected_common_values: &[(Vec<T>, usize)],
    expected_median: (Vec<T>, Option<Vec<T>>),
) {
    let xs = random_unique_vecs(
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
fn test_random_unique_vecs() {
    random_unique_vecs_helper(
        &random_primitive_ints::<u8>,
        4,
        1,
        &[
            vec![],
            vec![85, 11, 136, 200, 235, 134, 203, 223, 38, 217, 177, 162, 32, 166],
            vec![234, 30, 218, 90],
            vec![106, 9, 216, 204],
            vec![151],
            vec![],
            vec![213, 97, 253, 78, 91],
            vec![39, 191],
            vec![175, 170, 232, 233],
            vec![],
            vec![2, 35, 22, 217, 198, 114],
            vec![],
            vec![],
            vec![17, 32, 173, 114, 65, 121, 222, 25, 144, 148, 79],
            vec![115, 52, 73, 69, 137, 91, 153, 178],
            vec![],
            vec![112, 34, 95],
            vec![],
            vec![106, 167, 197, 130, 168],
            vec![122, 207, 172, 177, 86, 150, 221, 218, 101],
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
        (vec![96], None),
    );
    random_unique_vecs_helper(
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        4,
        1,
        &[
            vec![],
            vec![1, 14, 42, 12, 141, 19, 9, 68, 16, 79, 21, 17, 41, 124],
            vec![10, 1, 99, 0],
            vec![77, 36, 2, 12],
            vec![1],
            vec![],
            vec![103, 9, 19, 1, 5],
            vec![7, 6],
            vec![51, 159, 15, 18],
            vec![],
            vec![52, 75, 40, 64, 2, 26],
            vec![],
            vec![],
            vec![67, 34, 51, 30, 31, 49, 43, 7, 5, 4, 3],
            vec![14, 47, 24, 16, 52, 29, 1, 41],
            vec![],
            vec![13, 11, 62],
            vec![],
            vec![47, 3, 109, 42, 14],
            vec![37, 86, 25, 96, 41, 13, 16, 42, 5],
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
        (vec![15, 3], None),
    );
    random_unique_vecs_helper(
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
        &vec![
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
    random_unique_vecs_helper(
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
            vec!['q', 'á', 'g', 'Á', 'À', 'È', '»', 'ì', '³', 'ą', 'â', 'Ã', 'ñ', 'Ā'],
            vec!['Ã', 'ª', 'ä', '´'],
            vec!['½', 'Á', 'Ï', 'ý'],
            vec!['j'],
            vec![],
            vec!['ï', 'Ñ', 'u', 'Â', '½'],
            vec!['õ', 'x'],
            vec!['Â', 'ù', '¡', 'Ċ'],
            vec![],
            vec!['¬', 'b', 'Ñ', 's', 'Â', 'r'],
            vec![],
            vec![],
            vec!['n', '¿', 'Þ', 'ô', 'Ø', 'º', 'ü', 't', '¬', 'j', 'Á'],
            vec!['±', 'è', 'k', 'æ', 'b', 'Î', 'ā', 'Ü'],
            vec![],
            vec!['¹', '«', 'Î'],
            vec![],
            vec!['~', '´', 'Ý', 'â', '¯'],
            vec!['é', 'æ', 'Þ', 'ö', 'g', 'Î', 'Ç', 'Ü', '¼'],
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
            vec!['¶', 'Ă', 'ą', '®', 'Á', 'í', '¬', '¾', '¸', 'Ã', '}', 'ù', 'ý', '½', 'a'],
            Some(vec!['¶', 'Ă', 'ć']),
        ),
    );
}

#[test]
#[should_panic]
fn random_unique_vecs_fail_1() {
    random_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 0, 1);
}

#[test]
#[should_panic]
fn random_unique_vecs_fail_2() {
    random_unique_vecs(EXAMPLE_SEED, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_unique_vecs_fail_3() {
    random_unique_vecs(
        EXAMPLE_SEED,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
