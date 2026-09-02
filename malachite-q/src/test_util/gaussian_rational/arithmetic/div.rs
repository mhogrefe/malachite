// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;

// Schoolbook: (a + bi) / (c + di) = ((ac + bd) + (bc - ad)i) / (c^2 + d^2).
pub fn gaussian_rational_div_naive(x: &GaussianRational, y: &GaussianRational) -> GaussianRational {
    assert!(y.real != 0u32 || y.imaginary != 0u32, "division by zero");
    let norm: Rational = &y.real * &y.real + &y.imaginary * &y.imaginary;
    GaussianRational {
        real: (&x.real * &y.real + &x.imaginary * &y.imaginary) / &norm,
        imaginary: (&x.imaginary * &y.real - &x.real * &y.imaginary) / norm,
    }
}
