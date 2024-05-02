// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};

pub fn to_bits_asc_naive(n: &Integer) -> Vec<bool> {
    let mut bits = Vec::new();
    if *n == 0 {
        return bits;
    }
    for i in 0..n.significant_bits() {
        bits.push(n.get_bit(i));
    }
    let last_bit = *bits.last().unwrap();
    if last_bit != (*n < 0) {
        bits.push(!last_bit);
    }
    bits
}

pub fn to_bits_desc_naive(n: &Integer) -> Vec<bool> {
    let mut bits = Vec::new();
    if *n == 0 {
        return bits;
    }
    let significant_bits = n.significant_bits();
    let last_bit = n.get_bit(significant_bits - 1);
    if last_bit != (*n < 0) {
        bits.push(!last_bit);
    }
    for i in (0..significant_bits).rev() {
        bits.push(n.get_bit(i));
    }
    bits
}
