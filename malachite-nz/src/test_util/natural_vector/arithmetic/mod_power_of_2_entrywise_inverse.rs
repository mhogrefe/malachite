// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Inverse, Parity};

// The inverse of each element modulo `2^pow`, one inverse at a time, or `None` if some element is
// not invertible. Modulo 1, every element is 0, which is its own inverse.
pub fn natural_vector_mod_power_of_2_entrywise_inverse_naive(
    v: &NaturalVector,
    pow: u64,
) -> Option<NaturalVector> {
    if pow == 0 {
        return Some(v.clone());
    }
    Some(NaturalVector {
        elements: v
            .elements
            .iter()
            .map(|x| {
                if *x == 0u32 {
                    None
                } else {
                    x.mod_power_of_2_inverse(pow)
                }
            })
            .collect::<Option<_>>()?,
    })
}

// The odd elements of `v`, which are the ones invertible modulo a positive power of 2, in order.
pub fn natural_vector_retain_odd(v: &NaturalVector) -> NaturalVector {
    NaturalVector {
        elements: v.elements.iter().filter(|x| x.odd()).cloned().collect(),
    }
}
