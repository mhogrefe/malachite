// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::nevers::Never;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::slices::random_values_from_slice;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

#[test]
fn test_random_values_from_slice() {
    let test = |slice: &[u32],
                values: &[u32],
                common_values: &[(u32, usize)],
                actual_median: (u32, Option<u32>)| {
        let xs = random_values_from_slice(EXAMPLE_SEED, slice).cloned();
        let expected_values = xs.clone().take(20).collect_vec();
        let expected_common_values = common_values_map_debug(1000000, 10, xs.clone());
        let expected_median = median(xs.take(1000000));
        assert_eq!(
            (
                expected_values.as_slice(),
                expected_common_values.as_slice(),
                expected_median
            ),
            (values, common_values, actual_median)
        );
    };
    test(&[5], &[5; 20], &[(5, 1000000)], (5, None));
    test(
        &[0, 1],
        &[1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0],
        &[(1, 500473), (0, 499527)],
        (1, None),
    );
    test(
        &[1, 1, 1, 10],
        &[
            1, 1, 10, 1, 10, 10, 1, 10, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1, 1, 10,
        ],
        &[(1, 749985), (10, 250015)],
        (1, None),
    );
    test(
        &[2, 3, 5, 7, 11],
        &[
            3, 7, 3, 5, 11, 3, 5, 11, 2, 2, 5, 5, 2, 11, 2, 11, 5, 11, 5, 3,
        ],
        &[
            (2, 200420),
            (7, 200369),
            (11, 200347),
            (5, 199589),
            (3, 199275),
        ],
        (5, None),
    );
}

#[test]
#[should_panic]
fn random_values_from_slice_fail() {
    random_values_from_slice::<Never>(EXAMPLE_SEED, &[]);
}
