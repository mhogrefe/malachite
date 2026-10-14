// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModAdd, ModMul};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Composes two polynomials modulo m with Horner's rule over whole polynomials, using only the
// public modular arithmetic: $p(q) = (\cdots(p_{n-1} q + p_{n-2}) q + \cdots) q + p_0$.
pub fn mod_compose_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    let mut result = UnsignedPolynomial::ZERO;
    for &c in p.coefficients_asc().iter().rev() {
        result = result.mod_mul(q, m).mod_add(UnsignedPolynomial::from(c), m);
    }
    result
}
