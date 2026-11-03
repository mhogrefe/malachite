// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::modular::mod_evaluate::mod_evaluate_naive;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;

// Evaluates a polynomial at each of `xs` modulo `m` with `mod_evaluate_naive`.
pub fn mod_evaluate_many_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    xs: &[T],
    m: T,
) -> Vec<T> {
    xs.iter().map(|&x| mod_evaluate_naive(p, x, m)).collect()
}
