use core::hash::Hash;
use itertools::Itertools;
use malachite_base::chars::exhaustive::exhaustive_chars;
use malachite_base::chars::random::graphic_weighted_random_char_inclusive_range;
use malachite_base::num::random::geometric::geometric_random_unsigneds;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::{Seed, EXAMPLE_SEED};
use malachite_base::vecs::random::random_ordered_unique_vecs_min_length;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use std::fmt::Debug;

fn random_ordered_unique_vecs_min_length_helper<
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
    let xs = random_ordered_unique_vecs_min_length(
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
fn test_random_ordered_unique_vecs_min_length() {
    random_ordered_unique_vecs_min_length_helper(
        0,
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
    random_ordered_unique_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        7,
        1,
        &[
            vec![11, 85, 136],
            vec![9, 30, 32, 38, 90, 106, 134, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235],
            vec![78, 97, 151, 204, 213, 216, 253],
            vec![39, 91, 170, 175, 191, 232, 233],
            vec![2, 22, 35, 217],
            vec![17, 114, 198],
            vec![25, 32, 65, 114, 121, 144, 173, 222],
            vec![52, 73, 79, 115, 148],
            vec![34, 69, 91, 112, 137, 153, 178],
            vec![95, 106, 167],
            vec![86, 122, 130, 150, 168, 172, 177, 197, 207],
            vec![101, 218, 221],
            vec![9, 74, 115],
            vec![40, 48, 52, 97, 104, 109, 123, 133, 159, 196, 201, 235, 247, 250],
            vec![7, 11, 24, 43, 68, 103, 112, 157, 190, 216, 217],
            vec![84, 135, 211],
            vec![29, 55, 65, 89, 191, 206],
            vec![9, 51, 79],
            vec![3, 20, 22, 34, 62, 114, 118, 148],
            vec![23, 32, 47, 50, 120, 166, 176, 177, 194, 204, 238, 248],
        ],
        &[
            (vec![5, 128, 142], 4),
            (vec![137, 145, 160], 4),
            (vec![2, 4, 52], 3),
            (vec![1, 5, 192], 3),
            (vec![12, 41, 58], 3),
            (vec![2, 95, 171], 3),
            (vec![20, 86, 94], 3),
            (vec![21, 43, 50], 3),
            (vec![3, 81, 122], 3),
            (vec![31, 54, 79], 3),
        ],
        (vec![26, 138, 167], Some(vec![26, 138, 167, 173, 211])),
    );
    random_ordered_unique_vecs_min_length_helper(
        0,
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
    random_ordered_unique_vecs_min_length_helper(
        3,
        &|seed| geometric_random_unsigneds::<u32>(seed, 32, 1),
        7,
        1,
        &[
            vec![1, 14, 42],
            vec![0, 1, 9, 10, 12, 16, 17, 19, 21, 36, 41, 68, 77, 79, 99, 124, 141],
            vec![1, 2, 5, 9, 12, 19, 103],
            vec![6, 7, 15, 18, 51, 52, 159],
            vec![2, 40, 64, 75],
            vec![26, 34, 67],
            vec![4, 5, 7, 30, 31, 43, 49, 51],
            vec![3, 14, 16, 24, 47],
            vec![1, 11, 13, 29, 41, 52, 62],
            vec![3, 47, 109],
            vec![13, 14, 16, 25, 37, 41, 42, 86, 96],
            vec![5, 20, 42],
            vec![2, 74, 82],
            vec![3, 6, 7, 11, 17, 20, 36, 45, 56, 66, 76, 80, 89, 127],
            vec![1, 6, 10, 13, 19, 23, 25, 32, 41, 43, 97],
            vec![7, 41, 134],
            vec![9, 10, 25, 26, 47, 105],
            vec![68, 94, 109],
            vec![1, 3, 9, 13, 28, 43, 44, 84],
            vec![0, 4, 5, 6, 7, 13, 31, 32, 37, 42, 50, 75],
        ],
        &[
            (vec![0, 2, 5], 42),
            (vec![0, 1, 8], 39),
            (vec![0, 3, 4], 38),
            (vec![1, 3, 9], 38),
            (vec![0, 1, 7], 35),
            (vec![0, 2, 8], 34),
            (vec![1, 2, 12], 34),
            (vec![0, 1, 2], 33),
            (vec![1, 2, 3], 33),
            (vec![1, 3, 4], 33),
        ],
        (
            vec![3, 8, 14, 19, 25, 36, 52, 64, 71],
            Some(vec![3, 8, 14, 19, 25, 38, 58, 61]),
        ),
    );
    random_ordered_unique_vecs_min_length_helper(
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
    random_ordered_unique_vecs_min_length_helper(
        3,
        &random_primitive_ints::<u8>,
        13,
        4,
        &[
            vec![11, 85, 136],
            vec![134, 200, 235],
            vec![38, 203, 223, 235],
            vec![32, 162, 177, 217],
            vec![30, 90, 166, 218, 234],
            vec![9, 106, 216],
            vec![151, 204, 213],
            vec![78, 97, 253],
            vec![39, 91, 191],
            vec![170, 175, 232],
            vec![2, 35, 233],
            vec![22, 198, 217],
            vec![17, 32, 65, 114, 173],
            vec![25, 121, 173, 222],
            vec![79, 144, 148],
            vec![52, 69, 73, 115, 137],
            vec![91, 153, 178],
            vec![34, 95, 112],
            vec![106, 167, 197],
            vec![122, 130, 168],
        ],
        &[
            (vec![10, 87, 204], 6),
            (vec![15, 40, 115], 6),
            (vec![108, 193, 199], 6),
            (vec![1, 22, 70], 5),
            (vec![1, 8, 212], 5),
            (vec![2, 40, 169], 5),
            (vec![2, 58, 211], 5),
            (vec![3, 29, 186], 5),
            (vec![3, 97, 112], 5),
            (vec![11, 66, 140], 5),
        ],
        (vec![49, 78, 193], Some(vec![49, 78, 193, 215])),
    );
    random_ordered_unique_vecs_min_length_helper(
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
        &vec![
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
    random_ordered_unique_vecs_min_length_helper(
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
            vec!['g', 'q', 'á'],
            vec![
                'g', 'ª', '³', '´', '»', '½', 'À', 'Á', 'Ã', 'È', 'Ï', 'â', 'ä', 'ì', 'ñ', 'Ā', 'ą',
            ],
            vec!['j', 'u', '½', 'Â', 'Ñ', 'ï', 'ý'],
            vec!['x', '¡', '¬', 'Â', 'õ', 'ù', 'Ċ'],
            vec!['b', 's', '¬', 'Ñ'],
            vec!['n', 'r', 'Â'],
            vec!['t', '¬', 'º', '¿', 'Ø', 'Þ', 'ô', 'ü'],
            vec!['j', 'k', '±', 'Á', 'è'],
            vec!['b', '«', '¹', 'Î', 'Ü', 'æ', 'ā'],
            vec!['~', '´', 'Î'],
            vec!['g', '¯', 'Î', 'Ý', 'Þ', 'â', 'æ', 'é', 'ö'],
            vec!['¼', 'Ç', 'Ü'],
            vec!['¡', '§', 'Ì'],
            vec!['d', 'm', 'z', '{', '¨', '®', '±', '¼', 'Ë', 'Ü', 'ê', 'ì', 'ý', 'þ'],
            vec!['x', 'ª', '½', 'À', 'Õ', 'ì', 'ï', 'û', 'ă', 'Ą', 'ċ'],
            vec!['¢', '«', 'Ć'],
            vec!['{', '¢', '½', 'È', 'ä', 'ÿ'],
            vec!['Ë', 'Õ', 'ê'],
            vec!['p', '¨', '°', 'º', 'Å', 'Ó', '×', 'ü'],
            vec!['d', 'k', 'o', 'v', '¥', '±', 'Ä', 'È', 'Ê', 'ß', 'æ', 'Ć'],
        ],
        &[
            (vec!['m', 'u', 'w'], 6),
            (vec!['b', 'n', 'Ã'], 6),
            (vec!['g', '®', 'Ý'], 6),
            (vec!['x', 'Ä', 'î'], 6),
            (vec!['º', 'Ú', '÷'], 6),
            (vec!['a', 'w', 'ø'], 5),
            (vec!['c', 'e', 'Þ'], 5),
            (vec!['d', 't', 'Ã'], 5),
            (vec!['m', 'r', 'È'], 5),
            (vec!['w', '{', '³'], 5),
        ],
        (
            vec!['o', 's', '×', 'Ý', 'Þ', 'ß', 'î', 'ù'],
            Some(vec!['o', 's', '×', 'à', 'ã', 'ò', 'ċ']),
        ),
    );
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_min_length_fail_1() {
    random_ordered_unique_vecs_min_length(EXAMPLE_SEED, 3, &random_primitive_ints::<u32>, 3, 1);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_min_length_fail_2() {
    random_ordered_unique_vecs_min_length(EXAMPLE_SEED, 1, &random_primitive_ints::<u32>, 1, 0);
}

#[test]
#[should_panic]
fn random_ordered_unique_vecs_min_length_fail_3() {
    random_ordered_unique_vecs_min_length(
        EXAMPLE_SEED,
        0,
        &random_primitive_ints::<u32>,
        u64::MAX,
        u64::MAX - 1,
    );
}
