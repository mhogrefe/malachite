// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::arithmetic::traits::{MulAddMul, MulSubMul};

// A reference implementation of Gaussian-integer multiplication: the four products, via the fused
// kernels, with no size-based special cases. Any multiplication algorithm agrees with it.
pub fn gaussian_integer_mul_naive(x: &GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    GaussianInteger {
        real: (&x.real).mul_sub_mul(&y.real, &x.imaginary, &y.imaginary),
        imaginary: (&x.real).mul_add_mul(&y.imaginary, &x.imaginary, &y.real),
    }
}
