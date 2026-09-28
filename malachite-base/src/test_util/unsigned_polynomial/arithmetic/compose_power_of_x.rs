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
use alloc::vec;
use alloc::vec::Vec;

// Builds p(x^k) coefficient by coefficient, reading the coefficient i/k through `coefficient` where
// k divides i and giving zero elsewhere; for k = 0, sums the coefficients, with a checked sum.
// Nothing is shared with the implementation.
pub fn compose_power_of_x_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    k: u64,
) -> UnsignedPolynomial<T> {
    if k == 0 {
        let sum = (0..p.len()).fold(T::ZERO, |s, i| s.checked_add(p.coefficient(i)).unwrap());
        return UnsignedPolynomial::from_coefficients_asc(vec![sum]);
    }
    if p.len() == 0 {
        return UnsignedPolynomial::ZERO;
    }
    UnsignedPolynomial::from_coefficients_asc(
        (0..=(p.len() - 1) * k)
            .map(|i| {
                if i % k == 0 {
                    p.coefficient(i / k)
                } else {
                    T::ZERO
                }
            })
            .collect::<Vec<T>>(),
    )
}
