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

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::arithmetic::add::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use crate::natural::arithmetic::div_mod::{limbs_div_limb_to_out_mod, limbs_div_mod_to_out};
use crate::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::conversion::digits::general_digits::{
    limbs_from_digits_small_base, limbs_to_digits_small_base,
};
use crate::natural::{
    LIMB_HIGH_BIT, LIMB_MAX_HALF, Natural, bit_to_limb_count_ceiling, bit_to_limb_count_floor,
    limb_to_bit_count,
};
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::cmp::{max, min};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, CheckedLogBase2, DivMod, NegAssign, NegModPowerOf2, Parity, PowerOf2,
    WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, PowerOf2Digits};
use malachite_base::num::logic::traits::{LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::{slice_leading_zeros, slice_test_zero};

const WIDTH_I64: i64 = Limb::WIDTH as i64;

// This is MPFR_CAN_ROUND from mpfr-impl.h, MPFR 4.2.0.
pub fn float_can_round(x: &Natural, err0: u64, prec: u64, rm: RoundingMode) -> bool {
    match x {
        Natural(Small(small)) => limb_float_can_round(*small, err0, prec, rm),
        Natural(Large(xs)) => limbs_float_can_round(xs, err0, prec, rm),
    }
}

pub(crate) fn limb_float_can_round(x: Limb, err0: u64, mut prec: u64, rm: RoundingMode) -> bool {
    if rm == Nearest {
        prec += 1;
    }
    assert!(x.get_highest_bit());
    let err = min(err0, u64::power_of_2(Limb::LOG_WIDTH));
    if err <= prec {
        return false;
    }
    let mut s = Limb::WIDTH - (prec & Limb::WIDTH_MASK);
    let n = bit_to_limb_count_floor(err);
    // Check first limb
    let mask = Limb::low_mask(s);
    let mut tmp = x & mask;
    s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
    if n == 0 {
        // prec and error are in the same limb
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        // Check if error limb is 0
        s != Limb::WIDTH && x >> s != 0
    } else if tmp == mask {
        // Check if error limb is 0
        s != Limb::WIDTH && x >> s != Limb::MAX >> s
    } else {
        // limb is different from 000000 or 1111111
        true
    }
}

pub fn limbs_float_can_round(xs: &[Limb], err0: u64, mut prec: u64, rm: RoundingMode) -> bool {
    if rm == Nearest {
        prec += 1;
    }
    let len = xs.len();
    assert!(xs[len - 1].get_highest_bit());
    let err = min(err0, limb_to_bit_count(len));
    if err <= prec {
        return false;
    }
    let k = bit_to_limb_count_floor(prec);
    let mut s = Limb::WIDTH - (prec & Limb::WIDTH_MASK);
    let n = bit_to_limb_count_floor(err) - k;
    assert!(len > k);
    // Check first limb
    let mut i = len - k - 1;
    let mask = Limb::low_mask(s);
    let mut tmp = xs[i] & mask;
    i.wrapping_sub_assign(1);
    if n == 0 {
        // prec and error are in the same limb
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        // Check if all (n - 1) limbs are 0
        let j = i.wrapping_add(2) - n;
        if n > 1 && xs[j..=i].iter().any(|&x| x != 0) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j - 1] >> s != 0
    } else if tmp == mask {
        // Check if all (n - 1) limbs are 11111111111111111
        let j = i.wrapping_add(2) - n;
        if n > 1 && xs[j..=i].iter().any(|&x| x != Limb::MAX) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j - 1] >> s != Limb::MAX >> s
    } else {
        // First limb is different from 000000 or 1111111
        true
    }
}

// Given the significand `xs` of a nonzero finite `Float` (little-endian limbs, with the most
// significant bit of the most significant limb set), returns `Some(j)` if the significand's bits
// form a run of `j` ones followed by all zeros (that is, the mantissa equals $2^j - 1$), and `None`
// otherwise.
//
// This detects inputs `x` for which $1+x$ is an exact power of 2: combined with the exponent, a
// significand of the form $2^j - 1$ means the value is $2^e - 2^{e-j}$, which equals $2^k - 1$ (for
// `x` positive, when $e = j$, giving $k = j$) or $1 - 2^{-j}$ (for `x` in $(-1, 0)$, when $e = 0$,
// giving $k = -j$).
pub fn limbs_float_significand_leading_ones(xs: &[Limb]) -> Option<u64> {
    let mut i = xs.len();
    let mut count = 0;
    // Skip the all-ones limbs at the top.
    while i > 0 && xs[i - 1] == Limb::MAX {
        count += Limb::WIDTH;
        i -= 1;
    }
    if i == 0 {
        return Some(count);
    }
    // The transition limb (not all ones): it must be a run of ones followed by zeros.
    let m = xs[i - 1];
    let j = m.leading_ones();
    if m << j != 0 {
        // A one-bit appears below the leading run of ones.
        return None;
    }
    count += u64::from(j);
    // Every remaining lower limb must be zero.
    if slice_test_zero(&xs[..i - 1]) {
        Some(count)
    } else {
        None
    }
}

// Given the significand `x` of a nonzero finite `Float`, returns `Some(j)` if the mantissa equals
// $2^j - 1$ (a run of ones followed by all zeros), and `None` otherwise. See
// [`limbs_float_significand_leading_ones`].
pub fn float_significand_leading_ones(x: &Natural) -> Option<u64> {
    match x {
        Natural(Small(small)) => limbs_float_significand_leading_ones(core::slice::from_ref(small)),
        Natural(Large(xs)) => limbs_float_significand_leading_ones(xs),
    }
}

pub(crate) const MPFR_EVEN_INEX: i8 = 2;
pub(crate) const MPFR_ROUND_FAILED: i8 = 3;
const NEG_MPFR_ROUND_FAILED: i8 = -MPFR_ROUND_FAILED;

// This is MPFR_RNDRAW_EVEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool`
// signifying whether the returned exponent should be incremented.
pub(crate) fn round_helper_even(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper(out, out_prec, xs, x_prec, rm, |out, xs_hi, ulp| {
        let ulp_mask = !(ulp - 1);
        if xs_hi[0] & ulp == 0 {
            out.copy_from_slice(xs_hi);
            out[0] &= ulp_mask;
            (-MPFR_EVEN_INEX, false)
        } else {
            let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
            if increment {
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (MPFR_EVEN_INEX, increment)
        }
    })
}

// This is MPFR_RNDRAW and mpfr_round_raw from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a
// `bool` signifying whether the returned exponent should be incremented.
#[inline]
pub fn round_helper_raw(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper(out, out_prec, xs, x_prec, rm, |out, xs_hi, ulp| {
        let ulp_mask = !(ulp - 1);
        if xs_hi[0] & ulp == 0 {
            out.copy_from_slice(xs_hi);
            out[0] &= ulp_mask;
            (-1, false)
        } else {
            let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
            if increment {
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (1, increment)
        }
    })
}

// This is MPFR_RNDRAW and mpfr_round_raw from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a
// `bool` signifying whether the returned exponent should be incremented. The output is written to
// &mut xs[out_offset..].
#[inline]
pub fn round_helper_raw_aliased(
    out_offset: usize,
    out_prec: u64,
    xs: &mut [Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper_aliased(out_offset, out_prec, xs, x_prec, rm, |out, ulp| {
        let ulp_mask = !(ulp - 1);
        if out[0] & ulp == 0 {
            out[0] &= ulp_mask;
            (-1, false)
        } else {
            let increment = limbs_slice_add_limb_in_place(out, ulp);
            if increment {
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (1, increment)
        }
    })
}

// This is MPFR_RNDRAW_GEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool` signifying
// whether the returned exponent should be incremented.
fn round_helper<F: Fn(&mut [Limb], &[Limb], Limb) -> (i8, bool)>(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
    middle_handler: F,
) -> (i8, bool) {
    let xs_len = xs.len();
    let out_len = out.len();
    // Check trivial case when out mantissa has more bits than source
    if out_prec >= x_prec {
        out[out_len - xs_len..].copy_from_slice(xs);
        (0, false)
    } else {
        // - Nontrivial case: rounding needed
        // - Compute position and shift
        let shift = out_prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
        let i = xs_len.checked_sub(out_len).unwrap();
        let mut sticky_bit;
        let round_bit;
        // General case when prec % Limb::WIDTH != 0
        let ulp = if shift != 0 {
            // Compute rounding bit and sticky bit
            //
            // Note: in directed rounding modes, if the rounding bit is 1, the behavior does not
            // depend on the sticky bit; thus we will not try to compute it in this case (this can
            // be much faster and avoids reading uninitialized data in the current mpfr_mul
            // implementation). We just make sure that sticky_bit is initialized.
            let mask = Limb::power_of_2(shift - 1);
            let x = xs[i];
            round_bit = x & mask;
            sticky_bit = x & (mask - 1);
            if rm == Nearest || round_bit == 0 {
                let mut to = i;
                let mut n = xs_len - out_len;
                while n != 0 && sticky_bit == 0 {
                    to -= 1;
                    sticky_bit = xs[to];
                    n -= 1;
                }
            }
            mask << 1
        } else {
            assert!(out_len < xs_len);
            // Compute rounding bit and sticky bit - see note above
            let x = xs[i - 1];
            round_bit = x & LIMB_HIGH_BIT;
            sticky_bit = x & LIMB_MAX_HALF;
            if rm == Nearest || round_bit == 0 {
                let mut to = i - 1;
                let mut n = xs_len - out_len - 1;
                while n != 0 && sticky_bit == 0 {
                    to -= 1;
                    sticky_bit = xs[to];
                    n -= 1;
                }
            }
            1
        };
        let xs_hi = &xs[i..];
        let ulp_mask = !(ulp - 1);
        match rm {
            Floor | Down | Exact => {
                out.copy_from_slice(xs_hi);
                out[0] &= ulp_mask;
                (if sticky_bit | round_bit != 0 { -1 } else { 0 }, false)
            }
            Ceiling | Up => {
                if sticky_bit | round_bit == 0 {
                    out.copy_from_slice(xs_hi);
                    out[0] &= ulp_mask;
                    (0, false)
                } else {
                    let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
            Nearest => {
                if round_bit == 0 {
                    out.copy_from_slice(xs_hi);
                    out[0] &= ulp_mask;
                    (if (sticky_bit | round_bit) != 0 { -1 } else { 0 }, false)
                } else if sticky_bit == 0 {
                    middle_handler(out, xs_hi, ulp)
                } else {
                    let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
        }
    }
}

// This is MPFR_RNDRAW_GEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool` signifying
// whether the returned exponent should be incremented. The output is written to &mut
// xs[out_offset..].
fn round_helper_aliased<F: Fn(&mut [Limb], Limb) -> (i8, bool)>(
    out_offset: usize,
    out_prec: u64,
    xs: &mut [Limb],
    x_prec: u64,
    rm: RoundingMode,
    middle_handler: F,
) -> (i8, bool) {
    let xs_len = xs.len();
    let out_len = xs_len - out_offset;
    // Check trivial case when out mantissa has more bits than source
    if out_prec >= x_prec {
        (0, false)
    } else {
        // - Nontrivial case: rounding needed
        // - Compute position and shift
        let shift = out_prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
        let mut sticky_bit;
        let round_bit;
        // General case when prec % Limb::WIDTH != 0
        let ulp = if shift != 0 {
            // Compute rounding bit and sticky bit
            //
            // Note: in directed rounding modes, if the rounding bit is 1, the behavior does not
            // depend on the sticky bit; thus we will not try to compute it in this case (this can
            // be much faster and avoids reading uninitialized data in the current mpfr_mul
            // implementation). We just make sure that sticky_bit is initialized.
            let mask = Limb::power_of_2(shift - 1);
            let x = xs[out_offset];
            round_bit = x & mask;
            sticky_bit = x & (mask - 1);
            if rm == Nearest || round_bit == 0 {
                let mut n = out_offset;
                while n != 0 && sticky_bit == 0 {
                    n -= 1;
                    sticky_bit = xs[n];
                }
            }
            mask << 1
        } else {
            assert_ne!(out_offset, 0);
            // Compute rounding bit and sticky bit - see note above
            let x = xs[out_offset - 1];
            round_bit = x & LIMB_HIGH_BIT;
            sticky_bit = x & LIMB_MAX_HALF;
            if rm == Nearest || round_bit == 0 {
                let mut n = out_offset - 1;
                while n != 0 && sticky_bit == 0 {
                    n -= 1;
                    sticky_bit = xs[n];
                }
            }
            1
        };
        let out = &mut xs[out_offset..];
        let ulp_mask = !(ulp - 1);
        match rm {
            Floor | Down | Exact => {
                out[0] &= ulp_mask;
                (if sticky_bit | round_bit != 0 { -1 } else { 0 }, false)
            }
            Ceiling | Up => {
                if sticky_bit | round_bit == 0 {
                    out[0] &= ulp_mask;
                    (0, false)
                } else {
                    let increment = limbs_slice_add_limb_in_place(out, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
            Nearest => {
                if round_bit == 0 {
                    out[0] &= ulp_mask;
                    (if (sticky_bit | round_bit) != 0 { -1 } else { 0 }, false)
                } else if sticky_bit == 0 {
                    middle_handler(out, ulp)
                } else {
                    let increment = limbs_slice_add_limb_in_place(out, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
        }
    }
}

// Assuming xs is an approximation of a non-singular number with error at most equal to 2 ^ (EXP(x)
// - err0) (`err0` bits of x are known) of direction unknown, check if we can round x toward zero
// with precision prec.
//
// This is mpfr_round_p from round_p.c, MPFR 4.2.0.
pub(crate) fn round_helper_2(xs: &[Limb], err0: i32, prec: u64) -> bool {
    let len = xs.len();
    assert!(xs.last().unwrap().get_highest_bit());
    let mut err = limb_to_bit_count(len);
    if err0 <= 0 {
        return false;
    }
    let err0 = u64::from(err0.unsigned_abs());
    if err0 <= prec || prec >= err {
        return false;
    }
    err = min(err, err0);
    let k = bit_to_limb_count_floor(prec);
    let n = bit_to_limb_count_floor(err) - k;
    assert!(len > k);
    // Check first limb
    let xs = &xs[len - k - n - 1..];
    let (xs_last, xs_init) = xs[..=n].split_last().unwrap();
    let mut tmp = *xs_last;
    let mask = Limb::MAX >> (prec & Limb::WIDTH_MASK);
    tmp &= mask;
    if n == 0 {
        // prec and error are in the same limb
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        let (xs_head, xs_tail) = xs_init.split_first().unwrap();
        // Check if all (n - 1) limbs are 0
        if !slice_test_zero(xs_tail) {
            return true;
        }
        // Check if final error limb is 0
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && *xs_head >> s != 0
    } else if tmp == mask {
        let (xs_head, xs_tail) = xs_init.split_first().unwrap();
        // Check if all (n - 1) limbs are 11111111111111111
        if xs_tail.iter().any(|&x| x != Limb::MAX) {
            return true;
        }
        // Check if final error limb is 0
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && *xs_head >> s != Limb::MAX >> s
    } else {
        // First limb is different from 000000 or 1111111
        true
    }
}

#[inline]
pub fn limbs_significand_slice_add_limb_in_place(xs: &mut [Limb], y: Limb) -> bool {
    limbs_slice_add_limb_in_place(xs, y)
}

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

// `num_to_text36[d]` is the character for digit `d`, using lowercase letters; for base 2..=36.
const NUM_TO_TEXT_36: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz";
// `num_to_text62[d]` is the character for digit `d`, using uppercase letters for `d` in 10..=35 and
// lowercase letters for `d` in 36..=61; for negative bases and for bases 37..=62.
const NUM_TO_TEXT_62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

// Input: an approximation `xs * 2 ^ -neg_f` to a real `Y`, with `|xs * 2 ^ -neg_f - Y| <= 2 ^ (e -
// neg_f)`.
//
// If rounding is possible, returns:
// - in `out`: the characters of the significand corresponding to the integer nearest to `Y`, in the
//   direction `rm`;
// - in `exp`: the exponent (the number of superfluous characters).
//
// `n` is the number of limbs of `xs` (that is, `xs.len()`). `e` represents the maximal error in the
// approximation to `Y` (`e < 0` means that the approximation is known to be exact, that is, `xs * 2
// ^ -neg_f = Y`). `base` is the wanted base (`2 <= base <= 62` or `-36 <= base <= -2`), with
// magnitude `b = base.unsigned_abs()`. `digit_len` is the number of wanted digits in the
// significand. `rm` is the rounding mode. It is assumed that `b ^ (digit_len - 1) <= Y < b ^
// (digit_len + 1)`, thus the returned value satisfies `b ^ (digit_len - 1) <= rm(Y) < b ^
// (digit_len + 1)`.
//
// Rounding may fail for two reasons:
// - the error is too large to determine the integer `N` nearest to `Y`;
// - either the number of digits of `N` in base `b` is too large (`digit_len + 1`), or
//   `N=2*N1+(b/2)` and the rounding mode is to nearest. This can only happen when `b` is even.
//
// The first returned value is the direction of rounding:
// - the direction of rounding (-1, 0, 1) if rounding is possible;
// - `-MPFR_ROUND_FAILED` if rounding is not possible because of `digit_len + 1` digits;
// - `MPFR_ROUND_FAILED` otherwise (too large error).
//
// This is `mpfr_get_str_aux` from `get_str.c`, MPFR 4.2.2.
pub_test! {limbs_get_str_aux(
    out: &mut [u8],
    xs: &mut [Limb],
    neg_f: u64,
    e: i64,
    base: i64,
    digit_len: usize,
    rm: RoundingMode,
) -> (i8, i64) {
    let n = xs.len();
    let n_width = limb_to_bit_count(n);
    assert!(neg_f < n_width);
    let b = base.unsigned_abs();
    let mut exp = 0;
    // check if it is possible to round xs with rounding mode rm, where |xs * 2 ^ -neg_f - Y| <= 2 ^
    // (e - neg_f). xs contains exactly neg_f bits after the integer point; to determine the nearest
    // integer, we thus need a precision of n * Limb::WIDTH - neg_f.
    let exact = e < 0;
    if exact
        || round_helper_2(
            xs,
            i32::exact_from(i64::exact_from(n_width) - e),
            n_width - neg_f + u64::from(rm == Nearest),
        )
    {
        // compute the nearest integer to xs
        //
        // bit of weight 0 in xs has position j0 in limb xs[i0]
        let mut i0 = bit_to_limb_count_floor(neg_f);
        let j0 = neg_f & Limb::WIDTH_MASK;
        // mpfr_round_raw writes the rounded high limbs of xs back into xs starting at index i0,
        // while reading the original xs. Malachite uses a special function to handle this aliasing.
        let (mut dir, carry) = round_helper_raw_aliased(i0, n_width - neg_f, xs, n_width, rm);
        assert_ne!(dir, MPFR_ROUND_FAILED);
        if carry {
            // Y is a power of 2
            xs[n - 1] = if j0 != 0 {
                LIMB_HIGH_BIT >> (j0 - 1)
            } else {
                // j0 == 0, necessarily i0 >= 1, otherwise neg_f = 0 and xs is exact
                i0 -= 1;
                xs[i0] = 0; // set to zero the new low limb
                Limb::from(carry)
            };
        } else if j0 != 0 {
            // shift xs to the right by neg_f bits (i0 already done)
            limbs_slice_shr_in_place(&mut xs[i0..], j0);
        }
        // now the rounded value Y is in {xs + i0, n - i0}
        //
        // convert xs + i0 into base b: we use base, which might be in -36..-2 one extra character
        // is needed for limbs_to_digits_small_base
        let mut str1 = vec![0; digit_len + 3];
        let size_s1 = limbs_to_digits_small_base(&mut str1, b, &mut xs[i0..], None);
        // round str1
        assert!(size_s1 >= digit_len);
        exp = i64::exact_from(size_s1 - digit_len); // number of superfluous characters

        // if size_s1 = digit_len + 2, necessarily we have b ^ (digit_len + 1) as result, and the
        // result will not change; so we have to double-round only when size_s1 = digit_len + 1 and
        // (i) the result is inexact (ii) or the last digit is nonzero
        let size_s1_m1 = size_s1 - 1;
        if size_s1 == digit_len + 1 && (dir != 0 || str1[size_s1_m1] != 0) {
            // rounding mode
            let rnd1 = if rm == Nearest {
                let twice_last = u64::from(str1[size_s1_m1]) << 1;
                match twice_last.cmp(&b) {
                    Equal => {
                        if dir == 0 && exact {
                            // exact: even rounding
                            if str1[size_s1 - 2].even() {
                                Floor
                            } else {
                                Ceiling
                            }
                        } else {
                            // otherwise we cannot round correctly: for example if b = 10, we might
                            // have a mantissa of xxxxxxx5.00000000 which can be rounded to nearest
                            // to 8 digits but not to 7
                            return (NEG_MPFR_ROUND_FAILED, exp);
                        }
                    }
                    Less => Floor,
                    Greater => Ceiling,
                }
            } else {
                rm
            };
            // now rnd1 is either Floor or Down -> truncate, or Ceiling or Up -> round toward
            // infinity
            if rnd1 == Ceiling || rnd1 == Up {
                // round away from zero
                if str1[size_s1_m1] != 0 {
                    // the carry cannot propagate to the whole string, since Y = x * b ^ (digit_len
                    // - g) < 2 * b ^ digit_len <= b ^ (digit_len + 1) - b, where x is the input
                    // float
                    assert!(size_s1 >= 2);
                    let mut i = size_s1 - 2;
                    let target = u8::exact_from(b - 1);
                    while str1[i] == target {
                        assert_ne!(i, 0);
                        str1[i] = 0;
                        i -= 1;
                    }
                    str1[i] += 1;
                }
                dir = 1;
            } else if str1[size_s1_m1] != 0 {
                // Round toward zero (truncate). When the dropped digit is nonzero the digit
                // rounding dominates the earlier integer rounding (|V - N| >= 1 > |N - Y|), so the
                // overall direction is toward zero.
                dir = -1;
            }
            // Otherwise the dropped digit is zero, so the truncation is exact (V == N) and the
            // overall direction is the integer rounding's `dir`, which we leave unchanged.
            //
            // MPFR's `mpfr_get_str_aux` sets `dir = -1` unconditionally here, since it uses only
            // `dir != 0` (an inexact flag) and the sign is incidental; Malachite returns the
            // direction as an `Ordering`, so it must be correct.
        }
        // copy str1 into out and convert to characters (digits and letters from the source
        // character set)
        let num_to_text = if (2..=36).contains(&base) {
            NUM_TO_TEXT_36
        } else {
            NUM_TO_TEXT_62
        };
        for i in 0..digit_len {
            out[i] = num_to_text[usize::from(str1[i])];
        }
        (dir, exp)
    } else {
        // round_helper_2 failed: rounding is not possible
        (MPFR_ROUND_FAILED, exp)
    }
}}

// Computes the mantissa digits and exponent of a nonzero finite `Float` whose normalized
// little-endian significand is `xs` and whose MPFR-style exponent (one more than the scientific
// exponent) is `x_exp`, in base `abs_base` (the absolute value of the wanted base `base`), with
// `digit_len` digits, rounding with `rm`. Returns the `digit_len` digit characters and the
// exponent.
//
// `g`, `prec`, and `exp` are the initial values computed by the caller (see `mpfr_get_str`): `g =
// ceil_mul(x_exp - 1, abs_base, 1)`, the radix-2 working precision, and `|digit_len - g|`.
//
// This is the non-power-of-two, non-special branch of `mpfr_get_str` from `get_str.c`, MPFR 4.2.2.
#[doc(hidden)]
pub fn limbs_get_str(
    xs: &[Limb],
    x_exp: i64,
    abs_base: u64,
    base: i64,
    digit_len: usize,
    rm: RoundingMode,
    mut g: i64,
    mut prec: u64,
    mut exp: i64,
) -> (Vec<u8>, i64, i8) {
    let xs_len = xs.len();
    let digit_len_i = i64::exact_from(digit_len);
    // MPFR_ZIV_INIT: the initial precision increment.
    let mut ziv_step = Limb::WIDTH;
    loop {
        let mut exact = true;
        // number of limbs for the working precision
        let n = bit_to_limb_count_ceiling(prec);
        let mut a = vec![0; n];
        let mut exp_a: i64;
        let mut err: i64;
        match digit_len_i.cmp(&g) {
            Equal => {
                // final exponent is 0: no multiplication or division to perform
                err = if n < xs_len {
                    let (xs_lo, xs_hi) = xs.split_at(xs_len - n);
                    exact = slice_test_zero(xs_lo);
                    a.copy_from_slice(xs_hi);
                    i64::from(!exact)
                } else {
                    a[n - xs_len..].copy_from_slice(xs);
                    0
                };
                exp_a = x_exp - i64::exact_from(limb_to_bit_count(n));
            }
            Greater => {
                // multiply x by abs_base ^ exp; the error on a is at most 2 ^ err ulps
                let err_e;
                (exp_a, err_e) = limbs_float_exp(&mut a, abs_base, exp);
                exact = err_e == -1;
                // x = x1 * 2 ^ (n * Limb::WIDTH): the top min(n, xs_len) limbs of x
                let (x1, nx1) = if n < xs_len {
                    let (xs_lo, xs_hi) = xs.split_at(xs_len - n);
                    if exact {
                        exact = slice_test_zero(xs_lo);
                    }
                    (xs_hi, n)
                } else {
                    (xs, xs_len)
                };
                // we lose one more bit in the multiplication, except when err = 0 (two bits)
                err = if err_e <= 0 { 2 } else { i64::from(err_e) + 1 };
                let result = limbs_mul(&a, x1);
                let (result_lo, result_hi) = result.split_at(nx1);
                let result_hi = &result_hi[..n];
                if !slice_test_zero(result_lo) {
                    exact = false;
                }
                exp_a += x_exp;
                // normalize a and truncate
                if result_hi.last().unwrap().get_highest_bit() {
                    a.copy_from_slice(result_hi);
                } else {
                    limbs_shl_to_out(&mut a, result_hi, 1);
                    a[0] |= Limb::from(result_lo.last().unwrap().get_highest_bit());
                    exp_a -= 1;
                }
            }
            Less => {
                // digit_len < g: divide x by abs_base ^ exp
                let err_e;
                (exp_a, err_e) = limbs_float_exp(&mut a, abs_base, exp);
                exact = err_e == -1;
                let two_n = n << 1;
                let mut scratch;
                let rem;
                let result;
                let x1 = if two_n <= xs_len {
                    scratch = vec![0; two_n + 1];
                    (rem, result) = scratch.split_at_mut(n);
                    let (xs_lo, xs_hi) = xs.split_at(xs_len - two_n);
                    // we ignore the low xs_len - 2 * n limbs of x
                    if exact && !slice_test_zero(xs_lo) {
                        exact = false;
                    }
                    xs_hi
                } else {
                    scratch = vec![0; (two_n << 1) + 1];
                    let scratch_2;
                    (rem, scratch_2) = scratch.split_at_mut(n);
                    let x1_mut;
                    (x1_mut, result) = scratch_2.split_at_mut(two_n);
                    // copy the xs_len most significant limbs of x into the top of x1
                    x1_mut[two_n - xs_len..].copy_from_slice(xs);
                    &*x1_mut
                };
                // result = x / a
                if n == 1 {
                    rem[0] = limbs_div_limb_to_out_mod(result, x1, a[0]);
                } else {
                    limbs_div_mod_to_out(result, rem, x1, &a);
                }
                exp_a = x_exp - exp_a - i64::exact_from(limb_to_bit_count(two_n));
                // test if the division was exact
                if exact {
                    exact = slice_test_zero(rem);
                }
                // normalize the result and copy into a
                let (result_last, result_init) = result.split_last().unwrap();
                if *result_last == 1 {
                    limbs_shr_to_out(&mut a, result_init, 1);
                    a[n - 1] |= LIMB_HIGH_BIT;
                    exp_a += 1;
                } else {
                    a.copy_from_slice(result_init);
                }
                err = if err_e == -1 { 2 } else { i64::from(err_e) + 2 };
            }
        }
        if exact {
            err = -1;
        }
        let mut s = vec![0; digit_len];
        assert!(exp_a < 0);
        let (ret, e) = limbs_get_str_aux(
            &mut s,
            &mut a,
            exp_a.unsigned_abs(),
            err,
            base,
            digit_len,
            rm,
        );
        match ret {
            MPFR_ROUND_FAILED => {
                // error too large: increase the working precision (MPFR_ZIV_NEXT)
                prec += ziv_step;
                ziv_step = prec >> 1;
            }
            NEG_MPFR_ROUND_FAILED => {
                // too many digits in the mantissa: adjust the final exponent g and exp = |digit_len
                // - g|
                if digit_len_i > g {
                    exp -= 1;
                } else {
                    exp += 1;
                }
                g += 1;
            }
            _ => {
                // the exponent of s is its own exponent plus g; ret is the rounding direction
                return (s, e + g, ret);
            }
        }
    }
}

// Computes the mantissa digit characters and exponent of a nonzero finite `Float` whose normalized
// little-endian significand is `xs`, whose precision is `x_prec`, and whose MPFR-style exponent
// (one more than the scientific exponent) is `x_exp`, in the power-of-two base `abs_base` (the
// absolute value of the wanted base `base`), with `digit_len` digits, rounding the magnitude with
// `rm`.
//
// This is the power-of-two-base branch of `mpfr_get_str` from `get_str.c`, MPFR 4.2.2.
#[doc(hidden)]
pub fn limbs_get_str_power_of_2(
    xs: &[Limb],
    x_exp: i64,
    x_prec: u64,
    abs_base: u64,
    base: i64,
    digit_len: usize,
    rm: RoundingMode,
) -> (Vec<u8>, i64, i8) {
    let pow2 = abs_base.significant_bits() - 1; // base = 2 ^ pow2
    // x_exp = f * pow2 + r, with 1 <= r <= pow2 (a 1-indexed remainder, so split x_exp - 1)
    let (mut f, r) = (x_exp - 1).div_mod(i64::exact_from(pow2));
    f += 1;
    let r = u64::exact_from(r) + 1;
    // the first digit holds only r bits; prec is the total number of bits
    let prec = (u64::exact_from(digit_len) - 1) * pow2 + r;
    let len = bit_to_limb_count_ceiling(prec);
    let bit_len = limb_to_bit_count(len) - prec;
    let mut scratch = vec![0; len + 1];
    // round xs to prec bits into scratch, with the carry going into scratch[len]; the conversion to
    // base 2 ^ pow2 is then exact, so this rounding's direction is the overall direction
    let (dir, carry) = round_helper_raw(&mut scratch[..len], prec, xs, x_prec, rm);
    if carry {
        // mpfr_round_raw returns the wrapped value [0, ..., 0] and the carry; round_helper_raw
        // renormalizes the top limb to the high bit instead, so clear it to recover scratch = 2 ^
        // prec.
        scratch[len - 1] = 0;
        scratch[len] = 1;
        if r == pow2 {
            // prec = digit_len * pow2: 2 ^ prec needs digit_len + 1 digits in base 2 ^ pow2, so
            // divide by 2 ^ pow2
            limbs_slice_shr_in_place(&mut scratch, pow2);
            f += 1;
        }
    }
    // shift scratch right by bit_len bits, so the digit conversion sees a right-normalized number
    if bit_len != 0 {
        limbs_slice_shr_in_place(&mut scratch, bit_len);
        // the most significant limb may have become zero
        if *scratch.last().unwrap() == 0 {
            scratch.pop();
        }
    }
    // convert scratch to base abs_base = 2 ^ pow2, most significant digit first, and map to
    // characters
    let digits: Vec<u8> = Natural::from_owned_limbs_asc(scratch).to_power_of_2_digits_desc(pow2);
    let num_to_text = if (2..=36).contains(&base) {
        NUM_TO_TEXT_36
    } else {
        NUM_TO_TEXT_62
    };
    let s = digits[..digit_len]
        .iter()
        .map(|&d| num_to_text[usize::from(d)])
        .collect();
    (s, f, dir)
}

// `RED_INV_LOG_2[b - 2]`, as a `(numerator, denominator)` pair, is an upper approximation to
// `log(2) / log(b)`, no larger than 1. Both entries fit in 16 bits.
//
// This is `RedInvLog2Table` from `strtofr.c`, MPFR 4.3.0.
const RED_INV_LOG_2: [(u16, u16); 61] = [
    (1, 1),
    (53, 84),
    (1, 2),
    (4004, 9297),
    (53, 137),
    (2393, 6718),
    (1, 3),
    (665, 2108),
    (4004, 13301),
    (949, 3283),
    (53, 190),
    (5231, 19357),
    (2393, 9111),
    (247, 965),
    (1, 4),
    (4036, 16497),
    (665, 2773),
    (5187, 22034),
    (4004, 17305),
    (51, 224),
    (949, 4232),
    (3077, 13919),
    (53, 243),
    (73, 339),
    (5231, 24588),
    (665, 3162),
    (2393, 11504),
    (4943, 24013),
    (247, 1212),
    (3515, 17414),
    (1, 5),
    (4415, 22271),
    (4036, 20533),
    (263, 1349),
    (665, 3438),
    (1079, 5621),
    (5187, 27221),
    (2288, 12093),
    (4004, 21309),
    (179, 959),
    (51, 275),
    (495, 2686),
    (949, 5181),
    (3621, 19886),
    (3077, 16996),
    (229, 1272),
    (53, 296),
    (109, 612),
    (73, 412),
    (1505, 8537),
    (5231, 29819),
    (283, 1621),
    (665, 3827),
    (32, 185),
    (2393, 13897),
    (1879, 10960),
    (4943, 28956),
    (409, 2406),
    (247, 1459),
    (231, 1370),
    (3515, 20929),
];

// Converts `digits`, in base `base` and most significant first, to little-endian limbs written to
// `out`, returning the number of limbs written. `out` must have room for one limb beyond the
// result. The most significant limb written is nonzero as long as the first digit is.
//
// This is equivalent to `mpn_set_str` from `mpn/generic/set_str.c`, GMP 6.3.0.
fn limbs_set_str_helper(out: &mut [Limb], digits: &[u8], base: u64) -> usize {
    if let Some(bits) = base.checked_log_base_2() {
        // The base is a power of 2: read the digits from least to most significant, packing them
        // into limbs.
        let mut len = 0;
        let mut digit_out = 0;
        let mut next_bit_index = 0;
        for &digit in digits.iter().rev() {
            let digit = Limb::from(digit);
            digit_out |= digit << next_bit_index;
            next_bit_index += bits;
            if next_bit_index >= Limb::WIDTH {
                out[len] = digit_out;
                len += 1;
                next_bit_index -= Limb::WIDTH;
                digit_out = digit >> (bits - next_bit_index);
            }
        }
        if digit_out != 0 {
            out[len] = digit_out;
            len += 1;
        }
        len
    } else {
        limbs_from_digits_small_base(out, digits, base).unwrap()
    }
}

// The result of `limbs_set_str`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SetStrResult {
    // The significand limbs, the MPFR-style exponent (one more than the scientific exponent), and
    // the direction in which the magnitude was rounded.
    Finite(Vec<Limb>, i64, i8),
    // The exponent computation overflowed; the value is larger than any finite `Float`.
    Overflow,
    // The exponent computation underflowed; the value is smaller than any positive `Float`.
    Underflow,
}

// Adds two exponents, mirroring `MPFR_SADD_OVERFLOW`: `Err(true)` reports an overflow (the `goto
// overflow` branch) and `Err(false)` an underflow (`goto underflow`). Unlike the C macro, which
// performs a plain addition when the arguments have opposite signs, this is checked in every case.
fn add_exp(x: i64, y: i64) -> Result<i64, bool> {
    x.checked_add(y).ok_or(y > 0)
}

// The `goto overflow` and `goto underflow` targets of `parsed_string_to_mpfr`.
const fn out_of_range(overflow: bool) -> SetStrResult {
    if overflow {
        SetStrResult::Overflow
    } else {
        SetStrResult::Underflow
    }
}

// Converts a parsed digit string to a `Float` significand and MPFR-style exponent, correctly
// rounded to `prec_x` bits.
//
// `digits` holds digit values (not characters), most significant first, with leading and trailing
// zeros already stripped; it must be nonempty. `exp_base` is the number of digits before the point
// plus any base-`base` exponent, and `exp_bin` an additional binary exponent (the `p` form of the
// input), zero when there is none. `rm` must already have been inverted if the value is negative,
// since the returned direction refers to the magnitude.
//
// `base` must be between 2 and 62, inclusive, and `prec_x` must be positive.
//
// This is `parsed_string_to_mpfr` from `strtofr.c`, MPFR 4.3.0.
#[doc(hidden)]
pub fn limbs_set_str(
    digits: &[u8],
    base: u64,
    exp_base: i64,
    exp_bin: i64,
    prec_x: u64,
    rm: RoundingMode,
) -> SetStrResult {
    let digits_len = digits.len();
    assert_ne!(digits_len, 0);
    assert!(const { 2..=62 }.contains(&base));
    assert_ne!(prec_x, 0);
    // the initial working precision
    let mut prec = prec_x + prec_x.ceiling_log_base_2();
    // MPFR_ZIV_INIT: the house increment schedule, not MPFR's
    let mut ziv_step = Limb::WIDTH;
    // Compute the value of the leading digits as long as rounding is not possible.
    let (result, ysize_bits, mut exp) = loop {
        // y is regarded as a number of precision prec, occupying ysize limbs.
        let ysize = bit_to_limb_count_ceiling(prec);
        let ysize_bits = limb_to_bit_count(ysize);
        // pstr_size is the number of digits to read to fill at least ysize full limbs: we need base
        // ^ (pstr_size - 1) >= 2 ^ ysize_bits, so pstr_size = 1 + ceil(ysize_bits * Num / Den) with
        // Num / Den an upper approximation to 1 / log2(base). Writing ysize_bits = a * Den + b
        // keeps the products from overflowing.
        let (num, den) = RED_INV_LOG_2[usize::exact_from(base) - 2];
        let (num, den) = (u64::from(num), u64::from(den));
        let (a, b) = ysize_bits.div_mod(den);
        let mut pstr_size = usize::exact_from(a * num + (b * num).div_ceil(den) + 1);
        // Since pstr_size corresponds to at least ysize_bits bits, and ysize_bits >= prec, the
        // weight of the neglected part of the digits (if any) is less than ulp(y) < ulp(x).
        if pstr_size > digits_len {
            pstr_size = digits_len;
        }
        // The digits' value is less than base ^ pstr_size, which bounds the limbs the conversion
        // writes; one more is added because `limbs_set_str_helper` may touch the limb past the
        // result. MPFR instead assumes a fixed couple of limbs beyond ysize, which holds only when
        // pstr_size is the exact ceiling above. `Num / Den` overshoots it by a number of digits
        // proportional to ysize_bits, so at high precision the value really does need more, and no
        // fixed allowance is enough.
        let y_len =
            bit_to_limb_count_ceiling(u64::exact_from(pstr_size) * base.ceiling_log_base_2()) + 1;
        // y starts at offset ysize; the low ysize limbs are the scratch that the two exponentiation
        // cases below use.
        let mut y0 = vec![0; ysize + max(ysize, y_len)];
        // Convert the (possibly truncated) digits to binary; they are big-endian, so no offset is
        // needed.
        let real_ysize = limbs_set_str_helper(&mut y0[ysize..], &digits[..pstr_size], base);
        // `exact` tracks whether the result is known to be exact, which lets the loop terminate
        // even when the rounding test fails. It starts by accounting for the part of the input that
        // was ignored: trailing zeros were stripped in parsing, so anything ignored is nonzero.
        let mut exact = pstr_size == digits_len;
        // Normalize y and set the initial value of its exponent, which is 0 when y is not shifted.
        // The digits were normalized, so limbs_set_str_helper leaves a nonzero top limb.
        let y = &mut y0[ysize..];
        assert_ne!(y[real_ysize - 1], 0);
        let count = u64::from(y[real_ysize - 1].leading_zeros());
        let mut exp;
        if let Some(diff_ysize) = ysize.checked_sub(real_ysize) {
            // There is room to store {y, real_ysize} exactly in {y, ysize}, so the left shift loses
            // nothing and `exact` does not change.
            if count != 0 {
                limbs_slice_shl_in_place(&mut y[..real_ysize], count);
            }
            if diff_ysize != 0 {
                y.copy_within(0..real_ysize, diff_ysize);
                y[..diff_ysize].fill(0);
            }
            // the negation of the total shift count
            exp = -(i64::exact_from(limb_to_bit_count(diff_ysize)) + i64::exact_from(count));
        } else {
            // {y, real_ysize} does not fit in ysize limbs. Drop the low limbs that cannot be kept,
            // then shift the rest right by Limb::WIDTH - count bits, leaving the value's top bit at
            // the top of the ysize-th limb. MPFR only ever drops a limb when its limbs are narrower
            // than 12 bits; here the slack in `Num / Den` makes it happen at high precision too.
            let dropped = real_ysize - ysize - 1;
            if dropped != 0 {
                exact = exact && slice_test_zero(&y[..dropped]);
                y.copy_within(dropped..real_ysize, 0);
            }
            let kept = ysize + 1;
            if count != 0 {
                if limbs_slice_shr_in_place(&mut y[..kept], Limb::WIDTH - count) != 0 {
                    // some nonzero bits were shifted out
                    exact = false;
                }
            } else {
                exact = exact && y[0] == 0;
                y.copy_within(1..kept, 0);
            }
            exp = i64::exact_from(limb_to_bit_count(dropped + 1)) - i64::exact_from(count);
        }
        // Compute base ^ (exp_base - pstr_size) on ysize limbs, multiplying or dividing y by it.
        let pstr_size_i = i64::exact_from(pstr_size);
        let ysize_bits_i = i64::exact_from(ysize_bits);
        let mut err;
        // The rounded-toward-zero approximation, and the offset within it of the ysize significant
        // limbs.
        let mut product;
        let result_offset;
        if let Some(pow2) = base.checked_log_base_2() {
            // Case 1: the base is a power of two, so the scaling is exact.
            let pow2 = i64::exact_from(pow2);
            let mut tmp = match add_exp(exp_base, -pstr_size_i) {
                Ok(tmp) => tmp,
                Err(over) => return out_of_range(over),
            };
            tmp = match tmp.checked_mul(pow2) {
                Some(tmp) => tmp,
                None => return out_of_range(tmp > 0),
            };
            tmp = match add_exp(tmp, exp_bin) {
                Ok(tmp) => tmp,
                Err(over) => return out_of_range(over),
            };
            exp = match add_exp(exp, tmp) {
                Ok(exp) => exp,
                Err(over) => return out_of_range(over),
            };
            product = y0;
            result_offset = ysize;
            err = 0;
        } else if exp_base > pstr_size_i {
            // Case 2: multiply y by base ^ (exp_base - pstr_size).
            let (y0_lo, y_hi) = y0.split_at_mut(ysize);
            // z = base ^ (exp_base - pstr_size), rounded toward zero, in the scratch below y
            let (mut exp_z, err_z) = limbs_float_exp(y0_lo, base, exp_base - pstr_size_i);
            if err_z == -2 {
                return SetStrResult::Overflow;
            }
            exact = exact && err_z == -1;
            // Both y and z are rounded toward zero, so the product is too.
            product = limbs_mul(&y_hi[..ysize], y0_lo);
            // one more bit is lost in the multiplication, except when err_z is 0 (two bits)
            err = if err_z == -1 { 0 } else { i64::from(err_z) } + 1;
            exp_z = match add_exp(exp_z, ysize_bits_i) {
                Ok(exp_z) => exp_z,
                Err(over) => return out_of_range(over),
            };
            exp = match add_exp(exp, exp_z) {
                Ok(exp) => exp,
                Err(over) => return out_of_range(over),
            };
            // normalize the product
            if !product[(ysize << 1) - 1].get_highest_bit() {
                limbs_slice_shl_in_place(&mut product[ysize - 1..], 1);
                exp -= 1;
            }
            // if the low ysize limbs are all zero the result is still exact, if it was before
            exact = exact && slice_test_zero(&product[..ysize]);
            result_offset = ysize;
        } else if exp_base < pstr_size_i {
            // Case 3: divide y by base ^ (pstr_size - exp_base).
            //
            // y0 = y * 2 ^ ysize_bits
            y0[..ysize].fill(0);
            // avoid negating the extreme value
            let neg_exp_base = if exp_base == i64::MIN {
                i64::MAX
            } else {
                -exp_base
            };
            // The two overflow branches are swapped here: a larger divisor means a smaller result.
            let mut exp_z = match add_exp(pstr_size_i, neg_exp_base) {
                Ok(exp_z) => exp_z,
                Err(over) => return out_of_range(!over),
            };
            let mut z = vec![0; ysize];
            let err_z;
            (exp_z, err_z) = limbs_float_exp(&mut z, base, exp_z);
            // {z, ysize} * 2 ^ (exp_z - ysize_bits) approximates base ^ exp_z from below, with the
            // error bounded by 2 ^ err_z ulps (or exact when err_z is -1). The truncation errors of
            // the division and of the ignored digits have the opposite sign to the error on z, so
            // they partly compensate; the bound below takes the maximum rather than the sum.
            if err_z == -2 {
                return SetStrResult::Underflow;
            } else if err_z == -1 {
                err = 0;
            } else {
                err = i64::from(err_z);
                exact = false;
            }
            exp_z = match add_exp(exp_z, ysize_bits_i) {
                Ok(exp_z) => exp_z,
                Err(over) => return out_of_range(!over),
            };
            exp = match add_exp(exp, -exp_z) {
                Ok(exp) => exp,
                Err(over) => return out_of_range(over),
            };
            // Divide, rounding toward zero: the quotient has ysize + 1 limbs and the remainder
            // ysize. Both operands are normalized.
            assert!(y0[(ysize << 1) - 1].get_highest_bit());
            assert!(z[ysize - 1].get_highest_bit());
            let mut quotient = vec![0; ysize + 1];
            let mut remainder = vec![0; ysize];
            if ysize == 1 {
                remainder[0] = limbs_div_limb_to_out_mod(&mut quotient, &y0[..2], z[0]);
            } else {
                limbs_div_mod_to_out(&mut quotient, &mut remainder, &y0[..ysize << 1], &z);
            }
            assert!(quotient[ysize] <= 1);
            // see the note above on the compensating errors
            err += 1;
            // if the remainder is zero the result is still exact, if it was before
            exact = exact && slice_test_zero(&remainder);
            if quotient[ysize] == 1 {
                exact = exact && quotient[0].even();
                limbs_slice_shr_in_place(&mut quotient, 1);
                exp += 1;
            }
            product = quotient;
            result_offset = 0;
        } else {
            // Case 4: exp_base == pstr_size, so base ^ (exp_base - pstr_size) is 1 and there is
            // nothing to compute.
            product = y0;
            result_offset = ysize;
            err = 0;
        }
        // `product[result_offset..]` is an approximation, rounded toward zero, of the pstr_size
        // most significant digits, with equality when `exact`.
        let result = &product[result_offset..result_offset + ysize];
        // Test whether rounding is possible. The precx + (rnd == RNDN) trick is needed because the
        // ternary value must be determined too: for xxx...xxx111...111 under Nearest the correct
        // rounding is known but the ternary value is not.
        if exact
            || round_helper_2(
                result,
                i32::exact_from(ysize_bits_i - err - 1),
                prec_x + u64::from(rm == Nearest),
            )
        {
            break (result.to_vec(), ysize_bits, exp);
        }
        // MPFR_ZIV_NEXT
        prec += ziv_step;
        ziv_step = prec >> 1;
    };
    // round the result to prec_x bits
    let mut out = vec![0; bit_to_limb_count_ceiling(prec_x)];
    let (dir, increment) = round_helper_raw(&mut out, prec_x, &result, ysize_bits, rm);
    if increment {
        // round_helper_raw has already renormalized the top limb
        exp += 1;
    }
    // If the approximation was exact then no double rounding can occur, so `dir` is the correct
    // direction. The exponent may be out of range; the caller checks it.
    // `add_exp` reports a downward overflow only when its second argument is negative, and
    // `ysize_bits` is always positive, so the only failure possible here is an upward one.
    match add_exp(exp, i64::exact_from(ysize_bits)) {
        Ok(exp) => SetStrResult::Finite(out, exp, dir),
        Err(_) => SetStrResult::Overflow,
    }
}
