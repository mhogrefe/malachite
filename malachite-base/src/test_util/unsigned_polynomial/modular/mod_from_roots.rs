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
use crate::test_util::unsigned_polynomial::modular::mod_mul::mod_mul_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;

// Builds the product of the factors $x - r$ modulo m one factor at a time, multiplying each into
// the running product with schoolbook multiplication.
pub fn mod_from_roots_naive<T: PrimitiveUnsigned>(roots: &[T], m: T) -> UnsignedPolynomial<T> {
    if m == T::ONE {
        return UnsignedPolynomial::ZERO;
    }
    let mut product = vec![T::ONE];
    for &r in roots {
        product = mod_mul_naive(&product, &[r.mod_neg(m), T::ONE], m);
    }
    UnsignedPolynomial::from_coefficients_asc(product)
}
