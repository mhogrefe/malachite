// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Evaluates a polynomial at x modulo 2^pow term by term, as the sum of c_i x^i, with each power of
// x computed from scratch by mod_power_of_2_pow and every operation reduced. It shares nothing with
// the wrapping Horner's rule of the real evaluation.
pub fn evaluate_mod_power_of_2_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    x: T,
    pow: u64,
) -> T {
    let mut sum = T::ZERO;
    for (i, &c) in p.coefficients_asc().iter().enumerate() {
        let term = c.mod_power_of_2_mul(x.mod_power_of_2_pow(u64::exact_from(i), pow), pow);
        sum = sum.mod_power_of_2_add(term, pow);
    }
    sum
}

// Evaluates a polynomial at x modulo m term by term, as the sum of c_i x^i, with each power of x
// computed from scratch by mod_pow and every operation reduced. It shares nothing with the Horner's
// rule of the real evaluation.
pub fn evaluate_mod_naive<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, x: T, m: T) -> T {
    let mut sum = T::ZERO;
    for (i, &c) in p.coefficients_asc().iter().enumerate() {
        sum.mod_add_assign(c.mod_mul(x.mod_pow(u64::exact_from(i), m), m), m);
    }
    sum
}

// Evaluates a polynomial at each of `xs` modulo `m` with `evaluate_mod_naive`.
pub fn evaluate_many_mod_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    xs: &[T],
    m: T,
) -> Vec<T> {
    xs.iter().map(|&x| evaluate_mod_naive(p, x, m)).collect()
}

// Evaluates a polynomial at the first `k` powers of `q` modulo `m` with `evaluate_mod_naive`, each
// power computed from scratch by mod_pow.
pub fn evaluate_geometric_mod_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: T,
    k: u64,
    m: T,
) -> Vec<T> {
    (0..k)
        .map(|j| evaluate_mod_naive(p, q.mod_pow(j, m), m))
        .collect()
}
