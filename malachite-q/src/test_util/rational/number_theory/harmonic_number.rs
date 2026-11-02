// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::basic::traits::Zero;

// Term-by-term summation, the reference the fast implementation is checked against.
pub fn harmonic_number_naive(n: u64) -> Rational {
    let mut sum = Rational::ZERO;
    for k in 1..=n {
        sum += Rational::from_unsigneds(1u64, k);
    }
    sum
}
