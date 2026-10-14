// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::ModDiv;

// Each element divided by `c` modulo `m`, one division at a time, or `None` if some element has no
// quotient.
pub fn natural_vector_mod_div_naive(
    v: &NaturalVector,
    c: &Natural,
    m: &Natural,
) -> Option<NaturalVector> {
    Some(NaturalVector {
        elements: v
            .elements
            .iter()
            .map(|x| x.mod_div(c, m))
            .collect::<Option<_>>()?,
    })
}
