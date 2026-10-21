// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2MulAssign, ModPowerOf2SquareAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::BitIterable;
use crate::unsigned_matrix::UnsignedMatrix;

/// The power $A^e$ modulo $2^k$, by right-to-left binary powering with `ModPowerOf2MulAssign` and
/// `ModPowerOf2SquareAssign`.
pub fn unsigned_matrix_mod_power_of_2_pow_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    exp: u64,
    pow: u64,
) -> UnsignedMatrix<T> {
    let mut result = UnsignedMatrix::<T>::identity(a.height()).mod_power_of_2(pow);
    let mut base = a.clone();
    for bit in exp.bits() {
        if bit {
            result.mod_power_of_2_mul_assign(&base, pow);
        }
        base.mod_power_of_2_square_assign(pow);
    }
    result
}
