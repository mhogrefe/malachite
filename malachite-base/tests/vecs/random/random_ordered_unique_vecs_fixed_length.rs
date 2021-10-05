use core::hash::Hash;
use itertools::{repeat_n, Itertools};
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::vecs::random::random_ordered_unique_vecs_fixed_length;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;
use std::fmt::Debug;

fn random_ordered_unique_vecs_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[Vec<I::Item>],
    expected_common_values: &[(Vec<I::Item>, usize)],
    expected_median: (Vec<I::Item>, Option<Vec<I::Item>>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_ordered_unique_vecs_fixed_length(len, xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_ordered_unique_vecs_fixed_length() {
    random_ordered_unique_vecs_fixed_length_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &repeat_n(vec![], 20).collect_vec(),
        &[(vec![], 1000000)],
        (vec![], None),
    );
    random_ordered_unique_vecs_fixed_length_helper(
        1,
        random_bools(EXAMPLE_SEED),
        &[
            vec![true],
            vec![false],
            vec![false],
            vec![false],
            vec![true],
            vec![true],
            vec![true],
            vec![false],
            vec![true],
            vec![true],
            vec![true],
            vec![true],
            vec![false],
            vec![true],
            vec![true],
            vec![true],
            vec![true],
            vec![false],
            vec![true],
            vec![false],
        ],
        &[(vec![true], 500473), (vec![false], 499527)],
        (vec![true], None),
    );
    random_ordered_unique_vecs_fixed_length_helper(
        3,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            vec![69, 113, 239],
            vec![108, 210, 228],
            vec![87, 161, 168],
            vec![32, 83, 110],
            vec![34, 89, 188],
            vec![93, 200, 238],
            vec![115, 149, 189],
            vec![149, 201, 217],
            vec![31, 117, 146],
            vec![72, 151, 169],
            vec![7, 33, 174],
            vec![38, 81, 144],
            vec![72, 113, 127],
            vec![107, 128, 233],
            vec![12, 46, 119],
            vec![18, 164, 243],
            vec![59, 114, 174],
            vec![39, 174, 247],
            vec![104, 160, 184],
            vec![37, 100, 252],
        ],
        &[
            (vec![57, 142, 207], 7),
            (vec![32, 68, 169], 6),
            (vec![36, 70, 195], 6),
            (vec![125, 168, 194], 6),
            (vec![0, 97, 205], 5),
            (vec![2, 33, 227], 5),
            (vec![5, 46, 239], 5),
            (vec![9, 68, 189], 5),
            (vec![9, 78, 240], 5),
            (vec![1, 110, 203], 5),
        ],
        (vec![52, 133, 241], Some(vec![52, 133, 242])),
    );
    random_ordered_unique_vecs_fixed_length_helper(
        2,
        random_ordered_unique_vecs_fixed_length(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            vec![vec![69, 108], vec![113, 239]],
            vec![vec![161, 168], vec![210, 228]],
            vec![vec![32, 87], vec![83, 110]],
            vec![vec![34, 188], vec![89, 238]],
            vec![vec![93, 200], vec![115, 149]],
            vec![vec![149, 189], vec![201, 217]],
            vec![vec![31, 72], vec![117, 146]],
            vec![vec![33, 174], vec![151, 169]],
            vec![vec![7, 38], vec![81, 144]],
            vec![vec![72, 127], vec![113, 128]],
            vec![vec![46, 119], vec![107, 233]],
            vec![vec![12, 18], vec![164, 243]],
            vec![vec![59, 247], vec![114, 174]],
            vec![vec![39, 174], vec![160, 184]],
            vec![vec![37, 104], vec![100, 252]],
            vec![vec![69, 107], vec![122, 228]],
            vec![vec![142, 179], vec![242, 248]],
            vec![vec![61, 189], vec![233, 239]],
            vec![vec![7, 192], vec![85, 235]],
            vec![vec![90, 200], vec![178, 185]],
        ],
        &[
            (vec![vec![0, 78], vec![34, 52]], 2),
            (vec![vec![1, 58], vec![6, 112]], 2),
            (vec![vec![1, 63], vec![8, 154]], 2),
            (vec![vec![1, 97], vec![7, 250]], 2),
            (vec![vec![2, 33], vec![40, 81]], 2),
            (vec![vec![3, 160], vec![7, 29]], 2),
            (vec![vec![3, 32], vec![12, 60]], 2),
            (vec![vec![6, 130], vec![7, 20]], 2),
            (vec![vec![6, 68], vec![7, 126]], 2),
            (vec![vec![6, 77], vec![36, 54]], 2),
        ],
        (
            vec![vec![40, 193], vec![94, 142]],
            Some(vec![vec![40, 193], vec![97, 243]]),
        ),
    );
}
