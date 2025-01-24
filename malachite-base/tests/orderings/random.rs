// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::orderings::random::random_orderings;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;
use std::cmp::Ordering::*;

#[test]
fn test_random_orderings() {
    let xs = random_orderings(EXAMPLE_SEED);
    let values = xs.clone().take(20).collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xs.clone());
    let median = median(xs.take(1000000));
    assert_eq!(
        (values.as_slice(), common_values.as_slice(), median),
        (
            &[
                Less, Equal, Less, Greater, Less, Less, Equal, Less, Equal, Greater, Less, Equal,
                Less, Greater, Greater, Equal, Less, Equal, Greater, Greater
            ][..],
            &[(Less, 333784), (Greater, 333516), (Equal, 332700)][..],
            (Equal, None)
        )
    );
}
