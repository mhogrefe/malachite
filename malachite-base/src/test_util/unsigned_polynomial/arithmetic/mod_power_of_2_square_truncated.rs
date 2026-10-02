// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Squares the whole polynomial by schoolbook multiplication and then keeps the first `len`
// coefficients of the square, padding with zeros if it is shorter.
pub fn mod_power_of_2_square_truncated_naive<T: PrimitiveUnsigned>(
    xs: &[T],
    len: usize,
    pow: u64,
) -> Vec<T> {
    let mut out = mod_power_of_2_mul_naive(xs, xs, pow);
    out.resize(len, T::ZERO);
    out
}

// Squares a polynomial modulo $2^k$ by schoolbook multiplication and keeps the first `len`
// coefficients of the square, as a polynomial.
pub fn mod_power_of_2_square_truncated_polynomial_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    mod_power_of_2_mul_polynomial_naive(p, p, pow).truncate(len)
}
