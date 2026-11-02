// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;
use core::cmp::max;

// Subtracts the polynomials coefficient by coefficient with `ModPowerOf2Sub` for `T`, reading each
// coefficient through `coefficient`, which gives zero past the degree, and building the result with
// `from_coefficients_asc`, which trims. Nothing is shared with the implementation.
pub fn mod_power_of_2_sub_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let len = max(p.len(), q.len());
    UnsignedPolynomial::from_coefficients_asc(
        (0..len)
            .map(|i| p.coefficient(i).mod_power_of_2_sub(q.coefficient(i), pow))
            .collect::<Vec<T>>(),
    )
}
