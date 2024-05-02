// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::platform::Limb;
use malachite_base::num::logic::traits::{BitIterable, HammingDistance};
use std::iter::repeat;

pub fn integer_checked_hamming_distance_alt_1(x: &Integer, y: &Integer) -> Option<u64> {
    let negative = *x < 0;
    if negative != (*y < 0) {
        return None;
    }
    let bit_zip: Box<dyn Iterator<Item = (bool, bool)>> = if x.bits().count() >= y.bits().count() {
        Box::new(x.bits().zip(y.bits().chain(repeat(negative))))
    } else {
        Box::new(x.bits().chain(repeat(negative)).zip(y.bits()))
    };
    let mut distance = 0u64;
    for (b, c) in bit_zip {
        if b != c {
            distance += 1;
        }
    }
    Some(distance)
}

pub fn rug_checked_hamming_distance(x: &rug::Integer, y: &rug::Integer) -> Option<u64> {
    x.hamming_dist(y).map(u64::from)
}

pub fn integer_checked_hamming_distance_alt_2(x: &Integer, y: &Integer) -> Option<u64> {
    if (*x < 0) != (*y < 0) {
        return None;
    }
    let extension = if *x < 0 { Limb::MAX } else { 0 };
    let limb_zip: Box<dyn Iterator<Item = (Limb, Limb)>> =
        if x.twos_complement_limbs().count() >= y.twos_complement_limbs().count() {
            Box::new(
                x.twos_complement_limbs()
                    .zip(y.twos_complement_limbs().chain(repeat(extension))),
            )
        } else {
            Box::new(
                x.twos_complement_limbs()
                    .chain(repeat(extension))
                    .zip(y.twos_complement_limbs()),
            )
        };
    let mut distance = 0u64;
    for (x, y) in limb_zip {
        distance += x.hamming_distance(y);
    }
    Some(distance)
}
