// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::iterators::matching_intervals_in_iterator;

fn matching_intervals_in_iterator_helper<F: Fn(&u8) -> bool>(xs: &[u8], f: F, result: &[(u8, u8)]) {
    assert_eq!(
        matching_intervals_in_iterator(xs.iter().copied(), f).as_slice(),
        result
    );
}

#[test]
fn test_matching_intervals_in_iterator() {
    let xs = &[1, 2, 10, 11, 12, 7, 8, 16, 5];
    matching_intervals_in_iterator_helper(xs, |&x| x >= 10, &[(10, 12), (16, 16)]);
    matching_intervals_in_iterator_helper(xs, |&x| x < 10, &[(1, 2), (7, 8), (5, 5)]);
    matching_intervals_in_iterator_helper(xs, |&x| x >= 100, &[]);
    matching_intervals_in_iterator_helper(xs, |&x| x < 100, &[(1, 5)]);
}
