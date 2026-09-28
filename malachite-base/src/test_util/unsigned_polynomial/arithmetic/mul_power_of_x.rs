// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Builds the result coefficient by coefficient: zero below `n`, and the coefficient `n` places down
// above it, read through `coefficient`. Nothing is shared with the implementation.
pub fn mul_power_of_x_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    n: u64,
) -> UnsignedPolynomial<T> {
    if *p == UnsignedPolynomial::ZERO {
        return UnsignedPolynomial::ZERO;
    }
    UnsignedPolynomial::from_coefficients_asc(
        (0..p.len() + n)
            .map(|i| if i < n { T::ZERO } else { p.coefficient(i - n) })
            .collect::<Vec<T>>(),
    )
}
