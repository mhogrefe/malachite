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
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    CheckedRoot, CheckedSqrt, FloorSqrt, Pow, Square, UnsignedAbs,
};
use malachite_base::num::basic::traits::{One, Zero};

// Brute force: any root w has N(w) = N(z)^(1/k), and there are few Gaussian integers of a given
// small norm, so enumerate the real part and verify each candidate by raising it to the kth power.
// Only sensible for small norms.
pub fn gaussian_integer_checked_roots_naive(z: &GaussianInteger, k: u64) -> Vec<GaussianInteger> {
    assert_ne!(k, 0);
    if *z == 0u32 {
        return vec![GaussianInteger::ZERO];
    }
    let norm: Natural =
        z.real.unsigned_abs_ref().square() + z.imaginary.unsigned_abs_ref().square();
    let Some(n) = norm.checked_root(k) else {
        return Vec::new();
    };
    let mut roots = Vec::new();
    let max_x = (&n).floor_sqrt();
    let mut x = -Integer::from(&max_x);
    while x <= max_x {
        let y_squared = &n - (&x).unsigned_abs().square();
        if let Some(y) = y_squared.checked_sqrt() {
            let y = Integer::from(y);
            for y in if y == 0u32 {
                vec![y]
            } else {
                vec![y.clone(), -y]
            } {
                let w = GaussianInteger {
                    real: x.clone(),
                    imaginary: y,
                };
                if (&w).pow(k) == *z {
                    roots.push(w);
                }
            }
        }
        x += Integer::ONE;
    }
    roots
}
