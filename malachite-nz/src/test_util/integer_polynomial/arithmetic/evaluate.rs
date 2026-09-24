// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

// Evaluates a polynomial term by term, as the sum of $c_i x^i$, with each power of $x$ computed
// from scratch. This shares nothing with Horner's rule or the divide-and-conquer scheme, so it can
// check both.
pub fn evaluate_naive(p: &IntegerPolynomial, x: &Integer) -> Integer {
    let mut sum = Integer::ZERO;
    for (i, c) in p.coefficients_asc().iter().enumerate() {
        sum += c * x.pow(u64::exact_from(i));
    }
    sum
}
