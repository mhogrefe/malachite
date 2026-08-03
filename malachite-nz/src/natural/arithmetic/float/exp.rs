// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2022 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::shl::limbs_shl_to_out;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::limb_to_bit_count;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{NegAssign, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::slices::{slice_leading_zeros, slice_test_zero};

const WIDTH_I64: i64 = Limb::WIDTH as i64;

// Computes an approximation to `base ^ e` in `{xs, len}`, where `len` is `xs.len()`, returning the
// pair `(exp, err)`. The computed value is rounded toward zero (truncated), and `xs * 2 ^ exp`
// represents it, where `xs` is the integer `xs[0] + xs[1] * B + ... + xs[n - 1] * B ^ (n - 1)` with
// `B = 2 ^ Limb::WIDTH`.
//
// `err` is an integer `f` such that the final error is bounded by `2 ^ f` ulps; that is, `xs * 2 ^
// exp <= base ^ e <= 2 ^ exp * (xs + 2 ^ f)`. `err` is -1 if the result is exact, or -2 if an
// overflow occurred while computing `exp`.
//
// `len` must be positive, `e` must be positive, and `base` must be between 2 and 62, inclusive.
//
// This is equivalent to `mpfr_mpn_exp` from `mpn_exp.c`, MPFR 4.x.
#[doc(hidden)]
pub fn limbs_float_exp(xs: &mut [Limb], base: u64, e: i64) -> (i64, i32) {
    let len = xs.len();
    assert_ne!(len, 0);
    assert!(e > 0);
    assert!(const { 2..=62 }.contains(&base));
    let bit_len = i64::exact_from(limb_to_bit_count(len));
    // Normalize the base.
    let mut limb_base = Limb::exact_from(base);
    let mut h = i64::from(limb_base.leading_zeros());
    limb_base <<= h;
    h.neg_assign();
    // Allocate space for the running square or product, and set X to B. The scratch for the
    // squarings is sized inside the loop: the length being squared varies with the number of zero
    // low limbs, and `limbs_square_to_out_scratch_len` is not monotonic (the FFT range above
    // `SQR_FFT_THRESHOLD` needs no scratch while the Toom range below it does), so a buffer sized
    // once for `len` may be too small for a shorter operand.
    let two_len = len << 1;
    let mut ys = vec![0; two_len];
    let mut square_scratch: Vec<Limb> = Vec::new();
    let (xs_last, xs_init) = xs.split_last_mut().unwrap();
    *xs_last = limb_base;
    xs_init.fill(0);
    // The initial exponent for X; the invariant is X = {xs, len} * 2 ^ f.
    let mut f = h - (bit_len - WIDTH_I64);
    // The number of bits in e.
    let t = i32::exact_from(e.significant_bits());
    // `error == t` means that the result is still exact.
    let mut error = t;
    // The error counters are the numbers of left shifts when squaring (`err_s_a2`) and multiplying
    // (`err_s_ab`) after the first inexact loop.
    let mut err_s_a2: i32 = 0;
    let mut err_s_ab: i32 = 0;
    for i in (0..=t - 2).rev() {
        // xs_zeros is the number of zero low limbs of {xs, len} (that is, mpn_scan1(xs, 0) /
        // Limb::WIDTH).
        let xs_zeros = slice_leading_zeros(xs);
        let two_n1 = xs_zeros << 1;
        // Square of X: {c + 2 * xs_zeros, 2 * (len - xs_zeros)} = {xs + xs_zeros, len - xs_zeros} ^
        // 2. (`resize` trims or grows the scratch to the exact length this squaring needs, reusing
        // the allocation across iterations.)
        square_scratch.resize(limbs_square_to_out_scratch_len(len - xs_zeros), 0);
        limbs_square_to_out(&mut ys[two_n1..], &xs[xs_zeros..], &mut square_scratch);
        // Check for overflow on f.
        if !const { i64::MIN >> 1..=i64::MAX >> 1 }.contains(&f) {
            return (f, -2);
        }
        f <<= 1;
        if let Some(g) = f.checked_add(bit_len) {
            f = g;
        } else {
            // Reachable only when `f` lands within `Limb::WIDTH / 2` below `i64::MAX / 2`, so that
            // doubling and adding `len * Limb::WIDTH` overflows without the check above catching it
            // first. Every overflow found by testing is caught by that check instead, so this arm
            // is untested.
            fail_on_untested_path("limbs_float_exp, f overflow in checked_add");
            return (f, -2);
        }
        let (ys_lo, ys_hi) = ys.split_at(len);
        if ys_hi.last().unwrap().get_highest_bit() {
            xs.copy_from_slice(ys_hi);
        } else {
            limbs_shl_to_out(xs, ys_hi, 1);
            xs[0] |= Limb::from(ys_lo.last().unwrap().get_highest_bit());
            f -= 1;
            if error != t {
                err_s_a2 += 1;
            }
        }
        if error == t && two_n1 <= len && !slice_test_zero(&ys_lo[two_n1..]) {
            error = i;
        }
        if (e >> i).odd() {
            // Multiply A by B.
            let (ys_last, ys_init) = ys.split_last_mut().unwrap();
            let carry =
                limbs_mul_limb_to_out::<DoubleLimb, Limb>(&mut ys_init[len - 1..], xs, limb_base);
            *ys_last = carry;
            f += h + WIDTH_I64;
            let (ys_lo, ys_hi) = ys.split_at(len);
            if ys_hi.last().unwrap().get_highest_bit() {
                xs.copy_from_slice(ys_hi);
                if error != t {
                    err_s_ab += 1;
                }
            } else {
                limbs_shl_to_out(xs, ys_hi, 1);
                xs[0] |= Limb::from(ys_lo.last().unwrap().get_highest_bit());
                f -= 1;
            }
            if error == t && *ys_lo.last().unwrap() != 0 {
                error = i;
            }
        }
    }
    (
        f,
        if error == t {
            -1 // the result is exact
        } else {
            error + err_s_ab + (err_s_a2 >> 1) + 3
        },
    )
}
