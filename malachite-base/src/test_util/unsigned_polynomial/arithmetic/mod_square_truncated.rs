// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::arithmetic::mod_mul::mod_mul_naive;
use alloc::vec::Vec;

// Squares the whole polynomial by schoolbook multiplication and then keeps the first `len`
// coefficients of the square, padding with zeros if it is shorter.
pub fn mod_square_truncated_naive<T: PrimitiveUnsigned>(xs: &[T], len: usize, m: T) -> Vec<T> {
    let mut out = mod_mul_naive(xs, xs, m);
    out.resize(len, T::ZERO);
    out
}
