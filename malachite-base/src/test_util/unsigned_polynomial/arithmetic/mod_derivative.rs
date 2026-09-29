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

// Computes i * a_i mod m, for the coefficient a_i of x^i read through `coefficient`, by adding a_i
// to itself i times modulo m, and normalizes the result. Nothing is shared with the implementation.
pub fn mod_derivative_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    UnsignedPolynomial::from_coefficients_asc(
        (1..p.len())
            .map(|i| {
                let c = p.coefficient(i);
                (0..i).fold(T::ZERO, |s, _| s.mod_add(c, m))
            })
            .collect::<Vec<T>>(),
    )
}
