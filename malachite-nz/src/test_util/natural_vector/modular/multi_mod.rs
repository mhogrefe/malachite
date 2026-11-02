// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::unsigned_vector::UnsignedVector;

// Reduces the vector modulo each modulus separately, so nothing is shared between the moduli.
pub fn natural_vector_multi_mod_naive(
    v: &NaturalVector,
    moduli: &[Limb],
) -> Vec<UnsignedVector<Limb>> {
    moduli.iter().map(|&m| v % m).collect()
}
