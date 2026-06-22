// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// WORK IN PROGRESS: not yet wired into a public API or exercised by demos/tests, so the dead-code
// allow keeps WIP builds quiet; it comes off once the surface is in place.
#![allow(dead_code)]

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, Equal};
use malachite_base::num::arithmetic::traits::CeilingLogBase2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Up};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;

// Returns `x ^ n`, rounded to precision `prec` with rounding mode `rm`, along with an [`Ordering`]
// indicating whether the returned value is less than, equal to, or greater than the exact result.
//
// Two faithful-port deviations: the rare internal-overflow/underflow path (where MPFR recomputes
// with `mpfr_pow_z` because the error analysis no longer holds) is left as a stub pending a full
// exponential/`pow_z` implementation; and the Ziv precision-increase schedule follows Malachite's
// usual increment-halving convention rather than `MPFR_ZIV_NEXT` (any increasing schedule
// converges).
//
// This is `mpfr_pow_ui` from `pow_ui.c`, MPFR 4.2.2.
pub(crate) fn pow_ui_prec_round_ref(
    x: &Float,
    n: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // x^0 = 1 for any x, even a NaN.
    if n == 0 {
        return Float::from_unsigned_prec_round(1u64, prec, rm);
    }
    match &x.0 {
        NaN => return (Float::NAN, Equal),
        // Inf^n = Inf, (-Inf)^n = Inf for n even, -Inf for n odd.
        Infinity { sign } => {
            return (
                if *sign || n & 1 == 0 {
                    Float::INFINITY
                } else {
                    Float::NEGATIVE_INFINITY
                },
                Equal,
            );
        }
        // 0^n = 0 for any n; the result is a negative zero only for a negative zero and odd n.
        Zero { sign } => {
            return (
                if *sign || n & 1 == 0 {
                    Float::ZERO
                } else {
                    Float::NEGATIVE_ZERO
                },
                Equal,
            );
        }
        Finite { .. } => {}
    }
    // x is finite and nonzero.
    if n <= 2 {
        return if n == 1 {
            // x^1 = x
            Float::from_float_prec_round(x.clone(), prec, rm)
        } else {
            // x^2 = sqr(x)
            x.square_prec_round_ref(prec, rm)
        };
    }
    // n >= 3: left-to-right binary exponentiation wrapped in a Ziv loop. `nlen` is the bit length
    // of n, so 2 ^ (nlen - 1) <= n < 2 ^ nlen, and nlen >= 2.
    let nlen = n.significant_bits();
    // Set up the initial working precision.
    let mut working_prec = prec + 3 + Limb::WIDTH + prec.ceiling_log_base_2();
    if working_prec <= nlen {
        working_prec = nlen + 1;
    }
    let mut increment = Limb::WIDTH;
    loop {
        // The number of roundings is r(n) = n - 1, giving (via Higham's method) an absolute error
        // bounded by 2 ^ (1 + nlen) ulp(res), so res is accurate to `working_prec - 1 - nlen` bits.
        let err = working_prec - 1 - nlen;
        // Every squaring and multiplication rounds away from zero (`Up`): a squaring yields a
        // nonnegative value, and each multiplication by x happens immediately after a squaring (so
        // its left operand is positive), which makes MPFR's RNDU squarings and sign-based `rnd1`
        // multiplications uniformly away-from-zero.
        //
        // First step: res = x^2.
        let (mut res, o) = x.square_prec_round_ref(working_prec, Up);
        let mut inexact = o != Equal;
        if n & (1 << (nlen - 2)) != 0 {
            let (r, o) = res.mul_prec_round_val_ref(x, working_prec, Up);
            res = r;
            inexact |= o != Equal;
        }
        for k in (0..nlen - 2).rev() {
            let (r, o) = res.square_prec_round(working_prec, Up);
            res = r;
            inexact |= o != Equal;
            if n & (1 << k) != 0 {
                let (r, o) = res.mul_prec_round_val_ref(x, working_prec, Up);
                res = r;
                inexact |= o != Equal;
            }
        }
        // Internal overflow or underflow invalidates the error analysis above; MPFR recomputes with
        // `mpfr_pow_z`. That path is not yet ported (it needs a full `pow_z`), and is unreachable
        // except for astronomically large or small results.
        if matches!(res.0, Infinity { .. } | Zero { .. }) {
            todo!("pow_ui: internal overflow/underflow requires the mpfr_pow_z fallback");
        }
        if !inexact || float_can_round(res.significand_ref().unwrap(), err, prec, rm) {
            return Float::from_float_prec_round(res, prec, rm);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}
