// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

// The inverse of each element modulo `2^pow`, one inverse at a time, or `None` if some element is
// not invertible. Modulo 1, every element is 0, which is its own inverse.
pub fn unsigned_vector_mod_power_of_2_entrywise_inverse_naive<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    pow: u64,
) -> Option<UnsignedVector<T>> {
    if pow == 0 {
        return Some(v.clone());
    }
    Some(UnsignedVector {
        elements: v
            .elements
            .iter()
            .map(|&x| {
                if x == T::ZERO {
                    None
                } else {
                    x.mod_power_of_2_inverse(pow)
                }
            })
            .collect::<Option<_>>()?,
    })
}

// The odd elements of `v`, which are the ones invertible modulo a positive power of 2, in order.
pub fn unsigned_vector_retain_odd<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
) -> UnsignedVector<T> {
    UnsignedVector {
        elements: v.elements.iter().copied().filter(|&x| x.odd()).collect(),
    }
}
