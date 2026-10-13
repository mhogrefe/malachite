// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{CoprimeWith, ModInverse};

// The inverse of each element modulo `m`, one inverse at a time, or `None` if some element is not
// invertible. Modulo 1, every element is 0, which is its own inverse.
pub fn natural_vector_mod_entrywise_inverse_naive(
    v: &NaturalVector,
    m: &Natural,
) -> Option<NaturalVector> {
    if *m == 1u32 {
        return Some(v.clone());
    }
    Some(NaturalVector {
        elements: v
            .elements
            .iter()
            .map(|x| if *x == 0u32 { None } else { x.mod_inverse(m) })
            .collect::<Option<_>>()?,
    })
}

// The elements of `v` that are coprime to `m`, and so invertible modulo `m`, in order.
pub fn natural_vector_retain_units(v: &NaturalVector, m: &Natural) -> NaturalVector {
    NaturalVector {
        elements: v
            .elements
            .iter()
            .filter(|&x| x.coprime_with(m))
            .cloned()
            .collect(),
    }
}
