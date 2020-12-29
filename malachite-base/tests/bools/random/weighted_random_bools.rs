use malachite_base_test_util::stats::common_values_map::common_values_map;
use malachite_base_test_util::stats::median;

use itertools::Itertools;
use malachite_base::bools::random::weighted_random_bools;
use malachite_base::random::EXAMPLE_SEED;

fn weighted_random_bools_helper(
    w_numerator: u64,
    w_denominator: u64,
    expected_values: &[bool],
    expected_common_values: &[(bool, usize)],
    expected_median: (bool, Option<bool>),
) {
    let xs = weighted_random_bools(EXAMPLE_SEED, w_numerator, w_denominator);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[allow(clippy::decimal_literal_representation)]
#[test]
fn test_weighted_random_bools() {
    // w = 1
    weighted_random_bools_helper(
        1,
        1,
        &[
            false, true, true, true, false, false, false, true, false, false, false, false, true,
            false, false, false, false, true, false, true,
        ],
        &[(false, 500473), (true, 499527)],
        (false, None),
    );
    // w = 1/50
    weighted_random_bools_helper(
        1,
        50,
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false,
        ],
        &[(false, 980406), (true, 19594)],
        (false, None),
    );
    // w = 50
    weighted_random_bools_helper(
        50,
        1,
        &[
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, false, true, true, true,
        ],
        &[(true, 980602), (false, 19398)],
        (true, None),
    );
}

#[test]
#[should_panic]
fn weighted_random_bools_fail_1() {
    weighted_random_bools(EXAMPLE_SEED, 0, 1);
}

#[test]
#[should_panic]
fn weighted_random_bools_fail_2() {
    weighted_random_bools(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn weighted_random_bools_fail_3() {
    weighted_random_bools(EXAMPLE_SEED, 1, u64::MAX);
}

#[test]
#[should_panic]
fn weighted_random_bools_fail_4() {
    weighted_random_bools(EXAMPLE_SEED, u64::MAX, 1);
}
