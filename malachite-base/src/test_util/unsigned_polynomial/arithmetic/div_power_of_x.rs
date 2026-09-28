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

// Builds the result coefficient by coefficient, reading the coefficient `n` places up through
// `coefficient`. Nothing is shared with the implementation.
pub fn div_power_of_x_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    n: u64,
) -> UnsignedPolynomial<T> {
    UnsignedPolynomial::from_coefficients_asc(
        (n..p.len().max(n))
            .map(|i| p.coefficient(i))
            .collect::<Vec<T>>(),
    )
}
