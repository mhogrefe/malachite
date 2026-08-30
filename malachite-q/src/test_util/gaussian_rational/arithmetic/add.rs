// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::basic::traits::Zero;

pub fn gaussian_rational_sum_naive<I: Iterator<Item = GaussianRational>>(
    xs: I,
) -> GaussianRational {
    let mut s = GaussianRational::ZERO;
    for x in xs {
        s += x;
    }
    s
}
