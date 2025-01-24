// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use itertools::Itertools;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;

pub fn from_bits_asc_naive<I: Iterator<Item = bool>>(bits: I) -> Natural {
    let mut n = Natural::ZERO;
    for i in bits.enumerate().filter_map(|(index, bit)| {
        if bit {
            Some(u64::exact_from(index))
        } else {
            None
        }
    }) {
        n.set_bit(i);
    }
    n
}

pub fn from_bits_desc_naive<I: Iterator<Item = bool>>(bits: I) -> Natural {
    let bits = bits.collect_vec();
    let mut n = Natural::ZERO;
    for i in bits.iter().rev().enumerate().filter_map(|(index, &bit)| {
        if bit {
            Some(u64::exact_from(index))
        } else {
            None
        }
    }) {
        n.set_bit(i);
    }
    n
}
