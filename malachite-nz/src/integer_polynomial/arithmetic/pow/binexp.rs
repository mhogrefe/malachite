// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_polynomial::arithmetic::mul::mul_greater_to_out;
use crate::integer_polynomial::arithmetic::square::square_to_out;
use alloc::vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{FloorLogBase2, Parity, PowerOf2};

// Returns the bit of `e` one place below its most significant bit, where binary exponentiation
// starts, and whether the exponentiation swaps its two buffers an odd number of times. It swaps
// once for that bit if it is set, and once for each zero bit below it. `e` must be at least 2.
//
// This replaces the trial run of `_fmpz_poly_pow_binexp` from `fmpz_poly/pow_binexp.c`, FLINT
// 3.6.0, which counts the swaps by walking over the bits.
pub(crate) fn binexp_start(e: u64) -> (u64, bool) {
    let bit = u64::power_of_2(e.floor_log_base_2()) >> 1;
    let lower_zeros = bit.trailing_zeros() - (e & (bit - 1)).count_ones();
    (bit, (e & bit != 0) != lower_zeros.odd())
}

// Sets `out` to the coefficients of the `e`th power of the polynomial with coefficients `xs`, which
// has length at least 2, where `e` is at least 3. `out` must have length `e * (xs.len() - 1) + 1`.
//
// This is left-to-right binary exponentiation, alternating between `out` and a scratch buffer,
// starting in whichever makes the last result land in `out`.
//
// This is equivalent to `_fmpz_poly_pow_binexp` from `fmpz_poly/pow_binexp.c`, FLINT 3.6.0.
crate_test_fn! {pow_to_out_binexp<C: PolynomialCoefficient>(out: &mut [C], xs: &[C], e: u64) {
    let len = xs.len();
    let mut v = vec![C::ZERO; out.len()];
    let (mut bit, swaps) = binexp_start(e);
    let (mut r, mut s): (&mut [C], &mut [C]) = if swaps { (&mut v, out) } else { (out, &mut v) };
    // The first step squares xs itself
    let mut rlen = (len << 1) - 1;
    square_to_out(&mut r[..rlen], xs);
    if bit & e != 0 {
        mul_greater_to_out(&mut s[..rlen + len - 1], &r[..rlen], xs);
        rlen += len - 1;
        swap(&mut r, &mut s);
    }
    loop {
        bit >>= 1;
        if bit == 0 {
            break;
        }
        square_to_out(&mut s[..(rlen << 1) - 1], &r[..rlen]);
        rlen += rlen - 1;
        if bit & e != 0 {
            mul_greater_to_out(&mut r[..rlen + len - 1], &s[..rlen], xs);
            rlen += len - 1;
        } else {
            swap(&mut r, &mut s);
        }
    }
}}
