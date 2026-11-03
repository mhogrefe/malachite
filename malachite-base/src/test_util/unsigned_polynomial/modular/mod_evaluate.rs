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

// Evaluates a polynomial at x modulo m term by term, as the sum of c_i x^i, with each power of x
// computed from scratch by mod_pow and every operation reduced. It shares nothing with the Horner's
// rule of the real evaluation.
pub fn mod_evaluate_naive<T: PrimitiveUnsigned>(p: &UnsignedPolynomial<T>, x: T, m: T) -> T {
    let mut sum = T::ZERO;
    for (i, &c) in p.coefficients_asc().iter().enumerate() {
        sum.mod_add_assign(c.mod_mul(x.mod_pow(u64::exact_from(i), m), m), m);
    }
    sum
}
