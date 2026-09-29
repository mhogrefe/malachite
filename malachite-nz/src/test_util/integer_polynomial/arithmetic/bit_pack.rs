// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::polynomial::Evaluate;

// Evaluates the polynomial at 2^bits with Horner's method. Nothing is shared with the
// implementation.
pub fn bit_pack_naive(p: &IntegerPolynomial, bits: u64) -> Integer {
    p.evaluate(&Integer::power_of_2(bits))
}
