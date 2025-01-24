// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use itertools::Itertools;

pub fn median<I: Iterator>(xs: I) -> (I::Item, Option<I::Item>)
where
    I::Item: Eq + Ord,
{
    let mut xs = xs.collect_vec();
    assert!(!xs.is_empty());
    xs.sort_unstable();
    let n = xs.len();
    let half_n = n >> 1;
    if n.even() {
        // swap-remove m_2 first because if n == 2 it's the last element of the list.
        let m_2 = xs.swap_remove(half_n);
        let m_1 = xs.swap_remove(half_n - 1);
        if m_1 == m_2 {
            (m_1, None)
        } else {
            (m_1, Some(m_2))
        }
    } else {
        (xs.swap_remove(half_n), None)
    }
}

pub mod common_values_map;
pub mod median;
pub mod moments;
