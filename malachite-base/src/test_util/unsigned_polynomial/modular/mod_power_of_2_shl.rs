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

// Doubles every coefficient, read one at a time through `coefficient`, `bits` times, reducing
// modulo 2^pow after each doubling, and normalizes the result. Nothing is shared with the
// implementation.
pub fn mod_power_of_2_shl_naive<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    bits: U,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let bits: u64 = bits.exact_into();
    UnsignedPolynomial::from_coefficients_asc(
        (0..p.len())
            .map(|i| {
                let mut c = p.coefficient(i);
                for _ in 0..bits {
                    c = c.wrapping_add(c).mod_power_of_2(pow);
                }
                c
            })
            .collect::<Vec<T>>(),
    )
}
