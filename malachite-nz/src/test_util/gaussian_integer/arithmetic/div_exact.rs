// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{DivExact, MulAddMul, MulSubMul, Square};

// Schoolbook: (a + bi) / (c + di) = ((ac + bd) + (bc - ad)i) / (c^2 + d^2), with exact integer
// divisions.
pub fn gaussian_integer_div_exact_naive(
    x: &GaussianInteger,
    y: &GaussianInteger,
) -> GaussianInteger {
    assert!(y.real != 0u32 || y.imaginary != 0u32, "division by zero");
    let norm: Integer = (&y.real).square() + (&y.imaginary).square();
    GaussianInteger {
        real: (&x.real)
            .mul_add_mul(&y.real, &x.imaginary, &y.imaginary)
            .div_exact(&norm),
        imaginary: (&x.imaginary)
            .mul_sub_mul(&y.real, &x.real, &y.imaginary)
            .div_exact(norm),
    }
}
