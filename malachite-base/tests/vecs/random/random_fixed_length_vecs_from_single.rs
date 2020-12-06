use core::hash::Hash;
use std::fmt::Debug;
use std::iter::repeat;

use itertools::Itertools;
use malachite_base_test_util::stats::common_values_map::common_values_map_debug;
use malachite_base_test_util::stats::median;

use malachite_base::bools::random::random_bools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::vecs::random::random_fixed_length_vecs_from_single;

fn random_fixed_length_vecs_from_single_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[Vec<I::Item>],
    expected_common_values: &[(Vec<I::Item>, usize)],
    expected_median: (Vec<I::Item>, Option<Vec<I::Item>>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_fixed_length_vecs_from_single(len, xs);
    let values = xs.clone().take(20).collect::<Vec<_>>();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_random_fixed_length_vecs_from_single() {
    random_fixed_length_vecs_from_single_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &repeat(vec![]).take(20).collect_vec(),
        &[(vec![], 1000000)],
        (vec![], None),
    );
    random_fixed_length_vecs_from_single_helper(
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
    random_fixed_length_vecs_from_single_helper(
        3,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &[
            vec![113, 239, 69],
            vec![108, 228, 210],
            vec![168, 161, 87],
            vec![32, 110, 83],
            vec![188, 34, 89],
            vec![238, 93, 200],
            vec![149, 115, 189],
            vec![149, 217, 201],
            vec![117, 146, 31],
            vec![72, 151, 169],
            vec![174, 33, 7],
            vec![38, 81, 144],
            vec![72, 127, 113],
            vec![128, 233, 107],
            vec![46, 119, 12],
            vec![18, 164, 243],
            vec![114, 174, 59],
            vec![247, 39, 174],
            vec![160, 184, 104],
            vec![37, 100, 252],
        ],
        &[
            (vec![222, 60, 79], 4),
            (vec![26, 110, 13], 4),
            (vec![41, 254, 55], 4),
            (vec![109, 134, 76], 4),
            (vec![165, 174, 73], 4),
            (vec![236, 57, 174], 4),
            (vec![73, 168, 192], 4),
            (vec![89, 197, 244], 4),
            (vec![91, 170, 115], 4),
            (vec![142, 168, 231], 4),
        ],
        (vec![127, 253, 76], Some(vec![127, 253, 86])),
    );
    random_fixed_length_vecs_from_single_helper(
        2,
        random_fixed_length_vecs_from_single(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
        &[
            vec![vec![113, 239], vec![69, 108]],
            vec![vec![228, 210], vec![168, 161]],
            vec![vec![87, 32], vec![110, 83]],
            vec![vec![188, 34], vec![89, 238]],
            vec![vec![93, 200], vec![149, 115]],
            vec![vec![189, 149], vec![217, 201]],
            vec![vec![117, 146], vec![31, 72]],
            vec![vec![151, 169], vec![174, 33]],
            vec![vec![7, 38], vec![81, 144]],
            vec![vec![72, 127], vec![113, 128]],
            vec![vec![233, 107], vec![46, 119]],
            vec![vec![12, 18], vec![164, 243]],
            vec![vec![114, 174], vec![59, 247]],
            vec![vec![39, 174], vec![160, 184]],
            vec![vec![104, 37], vec![100, 252]],
            vec![vec![228, 122], vec![107, 69]],
            vec![vec![242, 248], vec![179, 142]],
            vec![vec![239, 233], vec![61, 189]],
            vec![vec![235, 85], vec![192, 7]],
            vec![vec![200, 90], vec![185, 178]],
        ],
        &[
            (vec![vec![28, 96], vec![0, 11]], 2),
            (vec![vec![2, 43], vec![64, 233]], 2),
            (vec![vec![20, 33], vec![14, 10]], 2),
            (vec![vec![223, 84], vec![7, 22]], 2),
            (vec![vec![43, 33], vec![131, 6]], 2),
            (vec![vec![6, 233], vec![45, 89]], 2),
            (vec![vec![65, 26], vec![6, 146]], 2),
            (vec![vec![71, 80], vec![68, 88]], 2),
            (vec![vec![9, 85], vec![186, 55]], 2),
            (vec![vec![96, 254], vec![9, 37]], 2),
        ],
        (
            vec![vec![127, 243], vec![125, 130]],
            Some(vec![vec![127, 243], vec![134, 100]]),
        ),
    );
}
