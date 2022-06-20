use core::hash::Hash;
use itertools::{repeat_n, Itertools};
use malachite_base::bools::random::random_bools;
use malachite_base::num::random::random_primitive_ints;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use malachite_base::vecs::random::random_unique_vecs_fixed_length;
use std::fmt::Debug;

fn random_unique_vecs_fixed_length_helper<I: Clone + Iterator>(
    len: u64,
    xs: I,
    expected_values: &[Vec<I::Item>],
    expected_common_values: &[(Vec<I::Item>, usize)],
    expected_median: (Vec<I::Item>, Option<Vec<I::Item>>),
) where
    I::Item: Clone + Debug + Eq + Hash + Ord,
{
    let xs = random_unique_vecs_fixed_length(len, xs);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_random_unique_vecs_fixed_length() {
    random_unique_vecs_fixed_length_helper(
        0,
        random_primitive_ints::<u8>(EXAMPLE_SEED),
        &repeat_n(vec![], 20).collect_vec(),
        &[(vec![], 1000000)],
        (vec![], None),
    );
    random_unique_vecs_fixed_length_helper(
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
    random_unique_vecs_fixed_length_helper(
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
            (vec![205, 0, 97], 4),
            (vec![102, 18, 19], 4),
            (vec![105, 70, 13], 4),
            (vec![22, 45, 192], 4),
            (vec![87, 100, 26], 4),
            (vec![15, 107, 109], 4),
            (vec![134, 245, 157], 4),
            (vec![138, 164, 179], 4),
            (vec![219, 253, 196], 4),
            (vec![237, 197, 239], 4),
        ],
        (vec![128, 16, 107], Some(vec![128, 16, 116])),
    );
    random_unique_vecs_fixed_length_helper(
        2,
        random_unique_vecs_fixed_length(2, random_primitive_ints::<u8>(EXAMPLE_SEED)),
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
            (vec![vec![60, 12], vec![3, 32]], 2),
            (vec![vec![0, 80], vec![88, 210]], 2),
            (vec![vec![1, 3], vec![216, 183]], 2),
            (vec![vec![159, 0], vec![69, 30]], 2),
            (vec![vec![199, 6], vec![95, 79]], 2),
            (vec![vec![2, 98], vec![221, 19]], 2),
            (vec![vec![212, 65], vec![99, 2]], 2),
            (vec![vec![3, 14], vec![61, 170]], 2),
            (vec![vec![41, 155], vec![3, 72]], 2),
            (vec![vec![47, 85], vec![69, 66]], 2),
        ],
        (
            vec![vec![128, 41], vec![252, 44]],
            Some(vec![vec![128, 42], vec![8, 241]]),
        ),
    );
}
