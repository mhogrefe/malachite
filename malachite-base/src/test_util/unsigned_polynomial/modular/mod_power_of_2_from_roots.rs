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
use crate::test_util::unsigned_polynomial::modular::mod_power_of_2_mul::mod_power_of_2_mul_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;

// Builds the product of the factors $x - r$ modulo 2^pow one factor at a time, multiplying each
// into the running product with schoolbook multiplication.
pub fn mod_power_of_2_from_roots_naive<T: PrimitiveUnsigned>(
    roots: &[T],
    pow: u64,
) -> UnsignedPolynomial<T> {
    if pow == 0 {
        return UnsignedPolynomial::ZERO;
    }
    let mut product = vec![T::ONE];
    for &r in roots {
        product = mod_power_of_2_mul_naive(&product, &[r.mod_power_of_2_neg(pow), T::ONE], pow);
    }
    UnsignedPolynomial::from_coefficients_asc(product)
}
