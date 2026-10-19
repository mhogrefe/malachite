// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Reduces the matrix modulo each modulus separately, so nothing is shared between the moduli.
pub fn integer_matrix_multi_mod_naive(
    a: &IntegerMatrix,
    moduli: &[Limb],
) -> Vec<UnsignedMatrix<Limb>> {
    moduli.iter().map(|&m| a.mod_op(m)).collect()
}
