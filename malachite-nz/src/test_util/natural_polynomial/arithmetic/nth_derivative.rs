// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::derivative::derivative_naive;

// Differentiates n times, one step at a time, with the one-step naive reference. Nothing is shared
// with the implementation.
pub fn nth_derivative_naive(p: &NaturalPolynomial, n: u64) -> NaturalPolynomial {
    let mut q = p.clone();
    for _ in 0..n {
        q = derivative_naive(&q);
    }
    q
}
