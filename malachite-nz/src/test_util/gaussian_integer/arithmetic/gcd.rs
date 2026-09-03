// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::test_util::gaussian_integer::factorization::remove_one_plus_i::*;
use malachite_base::num::arithmetic::traits::{AbsSquared, CanonicalizeUnit, DivRem};

// The plain Euclidean algorithm over the nearest-quotient division.
pub fn gaussian_integer_gcd_euclidean(x: &GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    let mut x = x.clone();
    let mut y = y.clone();
    while y != 0u32 {
        let (_, r) = (&x).div_rem(&y);
        x = y;
        y = r;
    }
    x.canonicalize_unit()
}

// The (1 + i)-ary analogue of the binary GCD (FLINT's `fmpzi_gcd_binary`), with exact norms: strip
// all factors of 1 + i from both, then repeatedly replace the larger by whichever of x ± y and x
// ± iy is shortest (all four are divisible by 1 + i, since both part-sums are odd) with its
// factors of 1 + i removed, and finally restore the smaller of the two stripped powers.
pub fn gaussian_integer_gcd_binary(x: &GaussianInteger, y: &GaussianInteger) -> GaussianInteger {
    if *x == 0u32 {
        return y.clone().canonicalize_unit();
    } else if *y == 0u32 {
        return x.clone().canonicalize_unit();
    }
    let (mut x, hx) = x.remove_one_plus_i();
    let (mut y, hy) = y.remove_one_plus_i();
    if (&x).abs_squared() < (&y).abs_squared() {
        core::mem::swap(&mut x, &mut y);
    }
    while y != 0u32 {
        let iy = GaussianInteger {
            real: -&y.imaginary,
            imaginary: y.real.clone(),
        };
        let candidates = [&x + &y, &x - &y, &x + &iy, &x - &iy];
        let z = candidates
            .into_iter()
            .min_by_key(|z| z.abs_squared())
            .unwrap();
        x = z.remove_one_plus_i().0;
        if (&x).abs_squared() < (&y).abs_squared() {
            core::mem::swap(&mut x, &mut y);
        }
    }
    (x * gaussian_integer_one_plus_i_pow(hx.min(hy))).canonicalize_unit()
}
