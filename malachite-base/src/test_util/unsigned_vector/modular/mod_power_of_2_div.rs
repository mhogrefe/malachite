// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

// Each element divided by `c` modulo `2^pow`, one division at a time, or `None` if some element has
// no quotient.
pub fn unsigned_vector_mod_power_of_2_div_naive<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    c: T,
    pow: u64,
) -> Option<UnsignedVector<T>> {
    Some(UnsignedVector {
        elements: v
            .elements
            .iter()
            .map(|&x| x.mod_power_of_2_div(c, pow))
            .collect::<Option<_>>()?,
    })
}
