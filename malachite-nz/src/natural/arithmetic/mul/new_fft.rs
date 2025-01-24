// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      TODO!
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::malachite_base::num::basic::integers::PrimitiveInt;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{PowerOf2, XMulYToZZ};
use malachite_base::num::conversion::traits::{ExactFrom, SciMantissaAndExponent};
use malachite_base::num::logic::traits::{SignificantBits, TrailingZeros};

// This is next_fft_number from fft_small/mpn_mul.c, FLINT 3.1.2.
pub fn next_fft_number(p: u64) -> u64 {
    let bits = p.significant_bits();
    let l = TrailingZeros::trailing_zeros(p - 1);
    let q = p - u64::power_of_2(l + 1);
    if bits < 15 {
        panic!();
    } else if q.significant_bits() == bits {
        q
    } else if l < 5 {
        u64::power_of_2(bits - 2) + 1
    } else {
        return u64::power_of_2(bits) - u64::power_of_2(l - 1) + 1;
    }
}

const D_BITS: i64 = 53;

pub fn ldexp(x: f64, exp: i64) -> f64 {
    let (m, e) = x.sci_mantissa_and_exponent();
    f64::from_sci_mantissa_and_exponent(m, e + exp).unwrap()
}

// This is fft_small_mulmod_satisfies_bounds from fft_small/mulmod_statisfies_bounds.c, FLINT 3.1.2.
pub fn fft_small_mulmod_satisfies_bounds(nn: Limb) -> bool {
    let n = nn as f64;
    let ninv = 1.0 / n;
    let t1 = n.mul_add(ninv, -1.0); // epsilon ~= t1/n  good enough
    let n1bits = nn.significant_bits();
    let (n2hi, n2lo) = Limb::x_mul_y_to_zz(nn, nn);
    let n2bits = if n2hi != 0 {
        Limb::WIDTH + n2hi.significant_bits()
    } else {
        n2lo.significant_bits()
    };
    // for |a*b| < 2*n^2
    //
    // |h*n_inv| < 2*n, so rounding in mul(h, ninv) at least B bits after the .
    let mut b = D_BITS - i64::exact_from(n1bits) - 1;
    if b < 2 {
        return false;
    }
    let limit2 = 2.0 * n * t1
        + ldexp(ninv, 1 + i64::exact_from(n2bits) - D_BITS - 1)
        + 0.5
        + ldexp(1.0, -(b + 1));
    // for |a * b| < 4 * n ^ 2
    b -= 1;
    let limit4 = 4.0 * n * t1
        + ldexp(ninv, 2 + i64::exact_from(n2bits) - D_BITS - 1)
        + 0.5
        + ldexp(1.0, -(b + 1));
    // fudge the limits 1 and 3/2 because the above is double arithmetic
    limit2 < 0.99 && limit4 < 1.49
}
