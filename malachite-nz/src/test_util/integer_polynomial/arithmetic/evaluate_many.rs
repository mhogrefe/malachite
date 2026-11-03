// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::test_util::integer_polynomial::arithmetic::evaluate::evaluate_naive;
use alloc::vec::Vec;

// Evaluates a polynomial at each of `xs` with `evaluate_naive`.
pub fn evaluate_many_naive(p: &IntegerPolynomial, xs: &[Integer]) -> Vec<Integer> {
    xs.iter().map(|x| evaluate_naive(p, x)).collect()
}
