// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use itertools::Itertools;
use malachite_base::iterators::comparison::delta_directions;
use std::cmp::Ordering::{self, *};

fn delta_directions_helper(xs: &[u8], result: &[Ordering]) {
    assert_eq!(delta_directions(xs.iter()).collect_vec(), result);
    assert_eq!(result.len(), xs.len().saturating_sub(1));
    assert_eq!(
        delta_directions(xs.iter().rev())
            .map(Ordering::reverse)
            .collect_vec(),
        result.iter().copied().rev().collect_vec()
    );
}

#[test]
fn test_delta_directions() {
    delta_directions_helper(&[], &[]);
    delta_directions_helper(&[5], &[]);
    delta_directions_helper(&[5, 6], &[Greater]);
    delta_directions_helper(&[5, 5], &[Equal]);
    delta_directions_helper(&[5, 4], &[Less]);
    delta_directions_helper(&[1, 2, 3, 4], &[Greater; 3]);
    delta_directions_helper(&[1, 2, 2, 4], &[Greater, Equal, Greater]);
    delta_directions_helper(&[1, 3, 2, 4], &[Greater, Less, Greater]);
    delta_directions_helper(
        &[3, 1, 4, 1, 5, 9],
        &[Less, Greater, Less, Greater, Greater],
    );
}
