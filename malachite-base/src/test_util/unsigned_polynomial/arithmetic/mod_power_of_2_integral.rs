// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Whether the integral modulo 2^pow of `p` is defined: whether every nonzero coefficient belongs to
// an even power of x.
pub fn mod_power_of_2_integral_is_defined<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    pow: u64,
) -> bool {
    pow == 0 || (0..p.len()).all(|i| i.even() || p.coefficient(i) == T::ZERO)
}

// Integrates a polynomial modulo 2^pow coefficient by coefficient, inverting each index separately.
// It returns `None` if the integral is not defined. Nothing is shared with the implementation.
pub fn mod_power_of_2_integral_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    pow: u64,
) -> Option<UnsignedPolynomial<T>> {
    let mut coefficients = Vec::with_capacity(usize::try_from(p.len()).unwrap() + 1);
    if p.len() != 0 {
        coefficients.push(T::ZERO);
    }
    for i in 0..p.len() {
        let c = p.coefficient(i);
        if c == T::ZERO {
            coefficients.push(T::ZERO);
            continue;
        }
        let k: u128 = u128::from(i + 1) % (1u128 << pow.min(127));
        let k = T::wrapping_from(k).mod_power_of_2(pow);
        if k == T::ZERO {
            return None;
        }
        let inverse = k.mod_power_of_2_inverse(pow)?;
        coefficients.push(c.mod_power_of_2_mul(inverse, pow));
    }
    Some(UnsignedPolynomial::from_coefficients_asc(coefficients))
}
