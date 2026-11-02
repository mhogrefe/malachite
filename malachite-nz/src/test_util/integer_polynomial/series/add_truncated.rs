// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::polynomial::Polynomial;

// Adds the whole polynomials and then truncates the result.
pub fn add_truncated_naive(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    (p + q).truncate(len)
}
