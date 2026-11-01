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
use crate::test_util::unsigned_polynomial::arithmetic::mod_mul_mod_polynomial::*;
use crate::unsigned_polynomial::UnsignedPolynomial;

// The power of `a` with exponent `exp` modulo `f`, with coefficients modulo `m`: `exp` naive
// multiplications modulo `f`, starting from 1. `f` must be nonzero with a leading coefficient
// invertible modulo `m`, unless `m` is 1.
pub fn mod_pow_mod_polynomial_naive<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    exp: u64,
    f: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    if m == T::ONE || f.len() == 1 {
        return UnsignedPolynomial::ZERO;
    }
    let mut r = UnsignedPolynomial::one();
    for _ in 0..exp {
        r = mod_mul_mod_polynomial_naive(&r, a, f, m);
    }
    r
}
