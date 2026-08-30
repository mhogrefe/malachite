// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{MulAddMul, MulSubMul};

// A reference implementation of Gaussian-rational squaring: the four-product multiplication formula
// with both operands equal. `x * x` would not do here: the multiplication operator detects aliased
// operands and delegates to the squaring algorithm under test.
pub fn gaussian_rational_square_naive(x: &GaussianRational) -> GaussianRational {
    GaussianRational {
        real: (&x.real).mul_sub_mul(&x.real, &x.imaginary, &x.imaginary),
        imaginary: (&x.real).mul_add_mul(&x.imaginary, &x.imaginary, &x.real),
    }
}
