// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::logic::traits::{BitIterable, HammingDistance, SignificantBits};
use std::iter::repeat;

pub fn natural_hamming_distance_alt_1(x: &Natural, y: &Natural) -> u64 {
    let bit_zip: Box<dyn Iterator<Item = (bool, bool)>> =
        if x.significant_bits() >= y.significant_bits() {
            Box::new(x.bits().zip(y.bits().chain(repeat(false))))
        } else {
            Box::new(x.bits().chain(repeat(false)).zip(y.bits()))
        };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    distance
}

pub fn natural_hamming_distance_alt_2(x: &Natural, y: &Natural) -> u64 {
    let limb_zip: Box<dyn Iterator<Item = (Limb, Limb)>> = if x.limb_count() >= y.limb_count() {
        Box::new(x.limbs().zip(y.limbs().chain(repeat(0))))
    } else {
        Box::new(x.limbs().chain(repeat(0)).zip(y.limbs()))
    };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    distance
}

pub fn rug_hamming_distance(x: &rug::Integer, y: &rug::Integer) -> u64 {
    u64::from(x.hamming_dist(y).unwrap())
}
