// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::arithmetic::mod_add::mod_add_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Adds the whole polynomials coefficient by coefficient modulo m, and then truncates the result.
pub fn mod_add_truncated_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    mod_add_naive(p, q, m).truncate(len)
}
