// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModShl;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Shifts every coefficient, read one at a time through `coefficient`, with the primitive `mod_shl`,
// so that the power of 2 is recomputed for each, and normalizes the result. Nothing is shared with
// the implementation.
pub fn mod_shl_naive<T: PrimitiveUnsigned + ModShl<U, T, Output = T>, U: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    bits: U,
    m: T,
) -> UnsignedPolynomial<T> {
    UnsignedPolynomial::from_coefficients_asc(
        (0..p.len())
            .map(|i| p.coefficient(i).mod_shl(bits, m))
            .collect::<Vec<T>>(),
    )
}
