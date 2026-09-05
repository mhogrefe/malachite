// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{FloorSqrt, Square, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;

// Guesses the root from floor square roots of the norm and of the two halves, tries both signs of
// the imaginary part, and keeps a guess only if it squares back to the input; the principal root is
// then the one with positive real part or, failing that, non-negative imaginary part.
pub fn gaussian_integer_checked_sqrt_naive(z: &GaussianInteger) -> Option<GaussianInteger> {
    let norm: Natural =
        z.real.unsigned_abs_ref().square() + z.imaginary.unsigned_abs_ref().square();
    let n = Integer::from(norm.floor_sqrt());
    let x = Integer::from(((&n + &z.real) >> 1u32).unsigned_abs().floor_sqrt());
    let y = Integer::from(((n - &z.real) >> 1u32).unsigned_abs().floor_sqrt());
    for candidate in [
        GaussianInteger {
            real: x.clone(),
            imaginary: y.clone(),
        },
        GaussianInteger {
            real: x,
            imaginary: -y,
        },
    ] {
        if (&candidate).square() == *z {
            return Some(
                if (&candidate.real, &candidate.imaginary) >= (&Integer::ZERO, &Integer::ZERO) {
                    candidate
                } else {
                    -candidate
                },
            );
        }
    }
    None
}
