// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::iter_windows;
use std::fmt::Debug;

fn iter_windows_helper<I: Iterator>(size: usize, xs: I, out: &[&[I::Item]])
where
    I::Item: Clone + Debug + Eq,
{
    let windows = iter_windows(size, xs)
        .map(|ws| ws.iter().cloned().collect_vec())
        .collect_vec();
    assert_eq!(
        windows.iter().map(Vec::as_slice).collect_vec().as_slice(),
        out
    );
}

#[test]
fn test_iter_windows() {
    iter_windows_helper(1, 0..=5, &[&[0], &[1], &[2], &[3], &[4], &[5]]);
    iter_windows_helper(2, 0..=5, &[&[0, 1], &[1, 2], &[2, 3], &[3, 4], &[4, 5]]);
    iter_windows_helper(3, 0..=5, &[&[0, 1, 2], &[1, 2, 3], &[2, 3, 4], &[3, 4, 5]]);
    iter_windows_helper(4, 0..=5, &[&[0, 1, 2, 3], &[1, 2, 3, 4], &[2, 3, 4, 5]]);
    iter_windows_helper(5, 0..=5, &[&[0, 1, 2, 3, 4], &[1, 2, 3, 4, 5]]);
    iter_windows_helper(6, 0..=5, &[&[0, 1, 2, 3, 4, 5]]);
    iter_windows_helper(7, 0..=5, &[]);
}

#[test]
#[should_panic]
fn iter_windows_fail() {
    iter_windows(0, 0..10);
}
