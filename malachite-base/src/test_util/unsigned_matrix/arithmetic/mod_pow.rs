// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::arithmetic::traits::{ModMulAssign, ModSquareAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::BitIterable;
use crate::unsigned_matrix::UnsignedMatrix;

/// The power $A^e$ modulo $m$, by right-to-left binary powering with `ModMulAssign` and
/// `ModSquareAssign`.
pub fn unsigned_matrix_mod_pow_naive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    exp: u64,
    m: T,
) -> UnsignedMatrix<T> {
    let mut result = UnsignedMatrix::<T>::identity(a.height()) % m;
    let mut base = a.clone();
    for bit in exp.bits() {
        if bit {
            result.mod_mul_assign(&base, m);
        }
        base.mod_square_assign(m);
    }
    result
}
