use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random::random_unique_vecs_min_length;
use std::fmt::Debug;

fn random_unique_vecs_min_length_helper<
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
    let xs = random_unique_vecs_min_length(
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
fn test_random_unique_vecs_min_length() {
    random_unique_vecs_min_length_helper(
        0,
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
    random_unique_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        7,
        1,
        &[
            vec![85, 11, 136],
            vec![200, 235, 134, 203, 223, 38, 217, 177, 162, 32, 166, 234, 30, 218, 90, 106, 9],
            vec![216, 204, 151, 213, 97, 253, 78],
            vec![91, 39, 191, 175, 170, 232, 233],
            vec![2, 35, 22, 217],
            vec![198, 114, 17],
            vec![32, 173, 114, 65, 121, 222, 25, 144],
            vec![148, 79, 115, 52, 73],
            vec![69, 137, 91, 153, 178, 112, 34],
            vec![95, 106, 167],
            vec![197, 130, 168, 122, 207, 172, 177, 86, 150],
            vec![221, 218, 101],
            vec![115, 74, 9],
            vec![123, 109, 52, 201, 159, 247, 250, 48, 133, 235, 196, 40, 97, 104],
            vec![68, 190, 216, 7, 157, 43, 112, 217, 24, 11, 103],
            vec![211, 84, 135],
            vec![55, 29, 206, 89, 65, 191],
            vec![51, 9, 79],
            vec![148, 34, 22, 62, 3, 114, 118, 20],
            vec![47, 194, 50, 32, 120, 176, 166, 23, 204, 248, 177, 238],
        ],
        &[
            (vec![8, 94, 244], 3),
            (vec![233, 14, 180], 3),
            (vec![46, 247, 166], 3),
            (vec![84, 118, 223], 3),
            (vec![4, 52, 2], 2),
            (vec![80, 2, 0], 2),
            (vec![1, 116, 5], 2),
            (vec![1, 5, 192], 2),
            (vec![1, 96, 39], 2),
            (vec![20, 10, 9], 2),
        ],
        (
            vec![127, 218, 55, 167, 163, 19, 99, 71, 32, 117, 72, 38, 27, 29],
            Some(vec![127, 218, 69, 59, 30, 91]),
        ),
    );
    random_unique_vecs_min_length_helper(
        0,
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
    random_unique_vecs_min_length_helper(
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        7,
        1,
        &[
            vec![1, 14, 42],
            vec![12, 141, 19, 9, 68, 16, 79, 21, 17, 41, 124, 10, 1, 99, 0, 77, 36],
            vec![2, 12, 1, 103, 9, 19, 5],
            vec![7, 6, 51, 159, 15, 18, 52],
            vec![75, 40, 64, 2],
            vec![26, 67, 34],
            vec![51, 30, 31, 49, 43, 7, 5, 4],
            vec![3, 14, 47, 24, 16],
            vec![52, 29, 1, 41, 13, 11, 62],
            vec![47, 3, 109],
            vec![42, 14, 37, 86, 25, 96, 41, 13, 16],
            vec![42, 5, 20],
            vec![2, 82, 74],
            vec![7, 76, 11, 17, 127, 36, 20, 56, 89, 45, 6, 80, 3, 66],
            vec![23, 43, 1, 25, 6, 41, 19, 97, 10, 13, 32],
            vec![41, 7, 134],
            vec![47, 105, 26, 10, 9, 25],
            vec![94, 68, 109],
            vec![1, 9, 84, 3, 43, 44, 28, 13],
            vec![13, 5, 0, 31, 50, 75, 32, 4, 7, 37, 42, 6],
        ],
        &[
            (vec![0, 1, 7], 13),
            (vec![0, 2, 8], 12),
            (vec![6, 2, 8], 12),
            (vec![0, 2, 5], 11),
            (vec![2, 7, 3], 11),
            (vec![9, 3, 1], 11),
            (vec![0, 9, 2], 10),
            (vec![1, 8, 7], 10),
            (vec![3, 5, 1], 10),
            (vec![5, 0, 2], 10),
        ],
        (vec![22, 28, 18, 89, 4, 52, 51], Some(vec![22, 28, 19])),
    );
    random_unique_vecs_min_length_helper(
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
    random_unique_vecs_min_length_helper(
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
            vec![114, 17, 32, 173, 65],
            vec![121, 222, 173, 25],
            vec![144, 148, 79],
            vec![115, 52, 73, 69, 137],
            vec![91, 153, 178],
            vec![112, 34, 95],
            vec![106, 167, 197],
            vec![130, 168, 122],
        ],
        &[
            (vec![8, 57, 90], 4),
            (vec![185, 30, 26], 4),
            (vec![106, 20, 234], 4),
            (vec![152, 245, 81], 4),
            (vec![108, 192, 235], 4),
            (vec![75, 2, 5], 3),
            (vec![0, 106, 9], 3),
            (vec![2, 91, 90], 3),
            (vec![212, 1, 8], 3),
            (vec![49, 5, 75], 3),
        ],
        (vec![128, 38, 40], Some(vec![128, 38, 42])),
    );
    random_unique_vecs_min_length_helper(
        0,
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
    random_unique_vecs_min_length_helper(
        3,
        &|seed| {
            graphic_weighted_random_char_inclusive_range(
                seed,
                'a',
                exhaustive_chars().nth(200).unwrap(),
                1,
                1,
            )
        },
        7,
        1,
        &[
            vec!['q', 'á', 'g'],
            vec![
                'Á', 'À', 'È', '»', 'ì', '³', 'ą', 'â', 'g', 'Ã', 'ñ', 'Ā', 'ª', 'ä', '´', '½', 'Ï',
            ],
            vec!['ý', 'j', 'ï', 'Ñ', 'u', 'Â', '½'],
            vec!['õ', 'x', 'Â', 'ù', '¡', 'Ċ', '¬'],
            vec!['b', 'Ñ', '¬', 's'],
            vec!['Â', 'r', 'n'],
            vec!['¿', 'Þ', 'ô', 'Ø', 'º', 'ü', 't', '¬'],
            vec!['j', 'Á', '±', 'è', 'k'],
            vec!['æ', 'b', 'Î', 'ā', 'Ü', '¹', '«'],
            vec!['Î', '~', '´'],
            vec!['Ý', 'â', '¯', 'é', 'æ', 'Þ', 'ö', 'g', 'Î'],
            vec!['Ç', 'Ü', '¼'],
            vec!['¡', '§', 'Ì'],
            vec!['±', 'd', 'ê', 'm', '®', 'ì', '¨', 'ý', 'þ', 'Ë', '{', 'Ü', 'z', '¼'],
            vec!['ï', 'ă', 'Ą', 'û', 'ª', 'ċ', 'x', 'À', 'ì', 'Õ', '½'],
            vec!['¢', '«', 'Ć'],
            vec!['{', 'È', 'ÿ', '½', '¢', 'ä'],
            vec!['Ë', 'Õ', 'ê'],
            vec!['º', 'ü', '×', '¨', 'Å', 'p', '°', 'Ó'],
            vec!['d', 'Ê', 'æ', 'ß', 'v', 'Ć', 'k', 'Ä', '±', 'È', '¥', 'o'],
        ],
        &[
            (vec!['a', 'Ã', '¤'], 4),
            (vec!['Ā', 'Ì', 'k'], 4),
            (vec!['w', 'm', 'u'], 3),
            (vec!['a', 'k', 'Ä'], 3),
            (vec!['b', 'Ã', 'n'], 3),
            (vec!['c', '|', 'Æ'], 3),
            (vec!['d', 'm', 'ï'], 3),
            (vec!['d', 'z', '¢'], 3),
            (vec!['d', '¯', 't'], 3),
            (vec!['f', 'g', '¹'], 3),
        ],
        (
            vec!['Ç', 'Ĉ', 's', 'ò', 'c', '¿', 'Å', 'Ô', 'Æ'],
            Some(vec!['Ç', 'Ĉ', 'v', '³', 'ò']),
        ),
    );
}

#[test]
#[should_panic]
fn random_unique_vecs_min_length_fail_1() {
    random_unique_vecs_min_length(EXAMPLE_SEED, 3, &random_primitive_ints::<u32>, 3, 1);
}

#[test]
#[should_panic]
fn random_unique_vecs_min_length_fail_2() {
    random_unique_vecs_min_length(EXAMPLE_SEED, 1, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_unique_vecs_min_length_fail_3() {
    random_unique_vecs_min_length(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
