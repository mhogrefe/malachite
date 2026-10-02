// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Squares by schoolbook multiplication of the polynomial by itself.
pub fn mod_power_of_2_square_naive<T: PrimitiveUnsigned>(xs: &[T], pow: u64) -> Vec<T> {
    mod_power_of_2_mul_naive(xs, xs, pow)
}

// Squares a polynomial modulo $2^k$ by schoolbook multiplication, as a polynomial.
pub fn mod_power_of_2_square_polynomial_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    pow: u64,
) -> UnsignedPolynomial<T> {
    mod_power_of_2_mul_polynomial_naive(p, p, pow)
}
