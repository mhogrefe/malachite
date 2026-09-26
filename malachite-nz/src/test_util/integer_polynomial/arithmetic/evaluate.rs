// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{Mod, Pow};
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

// Evaluates a polynomial at `x` modulo `m` by evaluating it exactly and reducing the value once.
// The value can be large, but nothing is shared with the word-sized evaluation.
pub fn mod_evaluate_u64_naive(p: &IntegerPolynomial, x: u64, m: u64) -> u64 {
    u64::exact_from(&evaluate_naive(p, &Integer::from(x)).mod_op(Integer::from(m)))
}

// Evaluates a polynomial at each of `xs` with `evaluate_naive`.
pub fn evaluate_many_naive(p: &IntegerPolynomial, xs: &[Integer]) -> Vec<Integer> {
    xs.iter().map(|x| evaluate_naive(p, x)).collect()
}
