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
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::Mod;

// Evaluates a polynomial at `x` modulo `m` by evaluating it exactly and reducing the value once.
// The value can be large, but nothing is shared with the word-sized evaluation.
pub fn mod_evaluate_u64_naive(p: &IntegerPolynomial, x: u64, m: u64) -> u64 {
    u64::exact_from(&evaluate_naive(p, &Integer::from(x)).mod_op(Integer::from(m)))
}
