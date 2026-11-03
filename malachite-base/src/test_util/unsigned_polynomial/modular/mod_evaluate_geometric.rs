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

// Evaluates a polynomial at the first `k` powers of `q` modulo `m` with `mod_evaluate_naive`, each
// power computed from scratch by mod_pow.
pub fn mod_evaluate_geometric_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: T,
    k: u64,
    m: T,
) -> Vec<T> {
    (0..k)
        .map(|j| mod_evaluate_naive(p, q.mod_pow(j, m), m))
        .collect()
}
