// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::bools::random::weighted_random_bools;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;

fn weighted_random_bools_helper(
    p_numerator: u64,
    p_denominator: u64,
    expected_values: &[bool],
    expected_common_values: &[(bool, usize)],
    expected_median: (bool, Option<bool>),
) {
    let xs = weighted_random_bools(EXAMPLE_SEED, p_numerator, p_denominator);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_weighted_random_bools() {
    // p = 0
    weighted_random_bools_helper(0, 1, &[false; 20], &[(false, 1000000)], (false, None));
    // p = 1
    weighted_random_bools_helper(1, 1, &[true; 20], &[(true, 1000000)], (true, None));
    // p = 1/2
    weighted_random_bools_helper(
        1,
        2,
        &[
            false, true, true, true, false, false, false, true, false, false, false, false, true,
            false, false, false, false, true, false, true,
        ],
        &[(false, 500473), (true, 499527)],
        (false, None),
    );
    // p = 1/51
    weighted_random_bools_helper(
        1,
        51,
        &[
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false,
        ],
        &[(false, 980406), (true, 19594)],
        (false, None),
    );
    // w = 50/51
    weighted_random_bools_helper(
        50,
        51,
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
    weighted_random_bools(EXAMPLE_SEED, 1, 0);
}

#[test]
#[should_panic]
fn weighted_random_bools_fail_2() {
    weighted_random_bools(EXAMPLE_SEED, 2, 1);
}
