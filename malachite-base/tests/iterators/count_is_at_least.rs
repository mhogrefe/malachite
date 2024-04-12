// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::count_is_at_least;
use std::iter::repeat;

fn count_is_at_least_helper(xs: &[u8], n: usize, result: bool) {
    assert_eq!(count_is_at_least(xs.iter(), n), result);
    assert_eq!(count_is_at_least(xs.iter().rev(), n), result);
}

#[test]
fn test_count_is_at_least() {
    count_is_at_least_helper(&[], 0, true);
    count_is_at_least_helper(&[], 1, false);
    count_is_at_least_helper(&[5], 0, true);
    count_is_at_least_helper(&[5], 1, true);
    count_is_at_least_helper(&[5], 2, false);
    count_is_at_least_helper(&[1, 2, 3, 4], 4, true);
    count_is_at_least_helper(&[1, 2, 3, 4], 5, false);
    count_is_at_least_helper(&[4; 100], 90, true);
    count_is_at_least_helper(&[4; 100], 101, false);

    assert_eq!(count_is_at_least(repeat(10), 20), true);
}
