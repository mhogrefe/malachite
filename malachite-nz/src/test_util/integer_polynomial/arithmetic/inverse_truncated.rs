// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::test_util::integer_polynomial::arithmetic::div_truncated::div_truncated_naive;
use malachite_base::polynomial::Polynomial;

// Inverts a power series by dividing 1 by it one coefficient at a time, or returns `None` if the
// inverse does not have integer coefficients.
pub fn inverse_truncated_naive(p: &IntegerPolynomial, len: u64) -> Option<IntegerPolynomial> {
    div_truncated_naive(&IntegerPolynomial::one(), p, len)
}
