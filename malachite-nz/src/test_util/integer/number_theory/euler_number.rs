// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;

// E_0, ..., E_{len - 1}, from the Seidel–Entringer triangle of the zigzag numbers: |E_n| for an
// even n is the n-th zigzag number, the number of alternating permutations of n elements, and its
// sign is (-1)^(n / 2).
pub fn euler_numbers_prefix_naive(len: u64) -> Vec<Integer> {
    let len = usize::exact_from(len);
    let mut result = Vec::with_capacity(len);
    let mut row = vec![Natural::ONE];
    for n in 0..len {
        if n != 0 {
            let mut new = vec![Natural::ZERO; n + 1];
            for k in 1..=n {
                new[k] = &new[k - 1] + &row[n - k];
            }
            row = new;
        }
        result.push(if n.odd() {
            Integer::ZERO
        } else if n % 4 == 2 {
            -Integer::from(&row[n])
        } else {
            Integer::from(&row[n])
        });
    }
    result
}
