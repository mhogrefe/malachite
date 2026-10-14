// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2Mul};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Composes two polynomials modulo 2^pow with Horner's rule over whole polynomials, using only the
// public modular arithmetic: $p(q) = (\cdots(p_{n-1} q + p_{n-2}) q + \cdots) q + p_0$.
pub fn mod_power_of_2_compose_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let mut result = UnsignedPolynomial::ZERO;
    for &c in p.coefficients_asc().iter().rev() {
        result = result
            .mod_power_of_2_mul(q, pow)
            .mod_power_of_2_add(UnsignedPolynomial::from(c), pow);
    }
    result
}
