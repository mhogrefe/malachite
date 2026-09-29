// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::arithmetic::mod_derivative::mod_derivative_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Differentiates n times, one step at a time, with the one-step naive reference. Nothing is shared
// with the implementation.
pub fn mod_nth_derivative_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    n: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    let mut q = p.clone();
    for _ in 0..n {
        q = mod_derivative_naive(&q, m);
    }
    q
}
