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

// Squares the polynomial with coefficients `xs`, nonempty and reduced modulo $m$, by multiplying it
// by itself.
pub fn mod_square_naive<T: PrimitiveUnsigned>(xs: &[T], m: T) -> Vec<T> {
    mod_mul_naive(xs, xs, m)
}
