// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2009, 2011, 2015, 2020 William Hart
//
//      Copyright © 2009 Jason Moxham
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_same_length_to_out, limbs_mul_same_length_to_out_scratch_len,
};
use crate::natural::arithmetic::neg::{limbs_neg, limbs_neg_in_place, limbs_neg_to_out};
use crate::natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out,
};
use crate::platform::{Limb, SignedLimb, FFT_TAB, MULMOD_TAB};
use alloc::vec::Vec;
use core::cmp::{max, min, Ordering::*};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, Parity, PowerOf2, WrappingAddAssign, WrappingSubAssign, XXAddYYToZZ,
    XXSubYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::LowMask;
use malachite_base::slices::slice_set_zero;

// This is equivalent to `mpn_addmod_2expp1_1` from `fft.h`, FLINT 2.7.1. `limbs` is one less than
// the length of `r`.
fn limbs_fft_addmod_2expp1_1(xs: &mut [Limb], c: Limb) {
    let x_lo = xs.first_mut().unwrap();
    let sum = x_lo.wrapping_add(c);
    // check if adding c would cause a carry to propagate
    if !(sum ^ *x_lo).get_highest_bit() {
        *x_lo = sum;
    } else if !c.get_highest_bit() {
        limbs_slice_add_limb_in_place(xs, c);
    } else {
        limbs_sub_limb_in_place(xs, c.wrapping_neg());
    }
}

// This is equivalent to `mpn_mul_2expmod_2expp1` from `fft/mul_2expmod_2expp1.c`, FLINT 2.7.1,
// where t != i1. `limbs` is one less than the length of `t` and the length of `i1`.
fn limbs_fft_mul_2expmod_2expp1(out: &mut [Limb], xs: &[Limb], d: u64) {
    assert_eq!(out.len(), xs.len());
    if d == 0 {
        out.copy_from_slice(xs);
    } else {
        let hi1 = Limb::wrapping_from(
            SignedLimb::wrapping_from(*xs.last().unwrap()) >> (Limb::WIDTH - d),
        );
        limbs_shl_to_out(out, xs, d);
        let out_last = out.last_mut().unwrap();
        let hi2 = *out_last;
        *out_last = 0;
        limbs_sub_limb_in_place(out, hi2);
        limbs_fft_addmod_2expp1_1(&mut out[1..], hi1.wrapping_neg());
    }
}

// This is equivalent to `mpn_mul_2expmod_2expp1` from `fft/mul_2expmod_2expp1.c`, FLINT 2.7.1,
// where t == i1. `limbs` is one less than the length of `t`.
fn limbs_fft_mul_2expmod_2expp1_in_place(out: &mut [Limb], d: u64) {
    if d != 0 {
        let hi1 = Limb::wrapping_from(
            SignedLimb::wrapping_from(*out.last().unwrap()) >> (Limb::WIDTH - d),
        );
        limbs_slice_shl_in_place(out, d);
        let out_last = out.last_mut().unwrap();
        let hi2 = *out_last;
        *out_last = 0;
        limbs_sub_limb_in_place(out, hi2);
        limbs_fft_addmod_2expp1_1(&mut out[1..], hi1.wrapping_neg());
    }
}

// This is equivalent to `mpn_normmod_2expp1` from `fft/normmod_2expp1.c`, FLINT 2.8.0. `limbs` is
// one less than the length of `t`.
fn limbs_fft_normmod_2expp1(out: &mut [Limb]) {
    let last_index = out.len() - 1;
    let mut hi = out[last_index];
    if hi != 0 {
        out[last_index] = 0;
        limbs_fft_addmod_2expp1_1(out, hi.wrapping_neg());
        // hi will now be in [-1,1]
        hi = out[last_index];
        if hi != 0 {
            out[last_index] = 0;
            limbs_fft_addmod_2expp1_1(out, hi.wrapping_neg());
            if out[last_index] == Limb::MAX {
                // if we now have -1
                out[last_index] = 0;
                limbs_fft_addmod_2expp1_1(out, 1);
            }
        }
    }
}

fn limbs_fft_mulmod_2expp1_basecase_same2_scratch_len(b: usize) -> usize {
    let n = (b + U_WIDTH - 1) >> Limb::LOG_WIDTH;
    (n << 1) + limbs_square_to_out_scratch_len(n)
}

// - ret + (xp,n) = (yp,n)*(zp,n) % 2^b+1
// - needs (tp,2n) temp space, everything reduced mod 2^b
// - inputs, outputs are fully reduced
// - NOTE: 2n is not the same as 2b rounded up to nearest limb
//
// This is equivalent to `flint_mpn_mulmod_2expp1_internal` from
// `mpn_extras/mulmod_2expp1_basecase.c`, FLINT 2.8.0, where xp == yp == zp. Asserts that b is a
// multiple of `Limb::WIDTH`.
fn limbs_fft_mulmod_2expp1_basecase_same2(
    xs: &mut [Limb],
    carry: Limb,
    b: usize,
    scratch: &mut [Limb],
) -> bool {
    match carry {
        0 => {
            let n = (b + U_WIDTH - 1) >> Limb::LOG_WIDTH;
            let k = (n << Limb::LOG_WIDTH) - b;
            assert_eq!(k, 0);
            let xs = &mut xs[..n];
            let (scratch, square_scratch) = scratch.split_at_mut(n << 1);
            limbs_square_to_out(scratch, xs, square_scratch);
            split_into_chunks_mut!(scratch, n, [scratch_lo, scratch_hi], _unused);
            limbs_sub_same_length_to_out(xs, scratch_lo, scratch_hi)
                && limbs_slice_add_limb_in_place(xs, 1)
        }
        3 => {
            xs[0] = 1;
            let xs_len = xs.len();
            slice_set_zero(&mut xs[1..xs_len - 1]);
            false
        }
        _ => panic!("Unexpected carry: {carry}"),
    }
}

fn limbs_fft_mulmod_2expp1_basecase_same_scratch_len(xs_len: usize) -> usize {
    (xs_len << 1) + limbs_mul_same_length_to_out_scratch_len(xs_len)
}

// c is the top bits of the inputs, must be fully reduced
//
// This is equivalent to `flint_mpn_mulmod_2expp1_basecase` from
// `mpn_extras/mulmod_2expp1_basecase.c`, FLINT 2.8.0, where xp == yp != zp, k is passed in, and n
// is the length of xp and zp.
fn limbs_fft_mulmod_2expp1_basecase_same(
    xs: &mut [Limb],
    ys: &[Limb],
    carry: Limb,
    k: usize,
    scratch: &mut [Limb],
) -> bool {
    assert_eq!(k, 0);
    let n = xs.len();
    assert_eq!(ys.len(), n);
    match carry {
        0 => {
            let (scratch, mul_scratch) = scratch.split_at_mut(n << 1);
            limbs_mul_same_length_to_out(scratch, xs, ys, mul_scratch);
            split_into_chunks_mut!(scratch, n, [scratch_lo, scratch_hi], _unused);
            limbs_sub_same_length_to_out(xs, scratch_lo, scratch_hi)
                && limbs_slice_add_limb_in_place(xs, 1)
        }
        1 => {
            let out = limbs_neg_in_place(xs) && limbs_slice_add_limb_in_place(xs, 1);
            *xs.last_mut().unwrap() &= Limb::MAX >> k;
            out
        }
        2 => {
            let out = limbs_neg_to_out(xs, ys) && limbs_slice_add_limb_in_place(xs, 1);
            *xs.last_mut().unwrap() &= Limb::MAX >> k;
            out
        }
        3 => {
            xs[0] = 1;
            let xs_len = xs.len();
            slice_set_zero(&mut xs[1..xs_len - 1]);
            false
        }
        _ => panic!("Unexpected carry: {carry}"),
    }
}

// This is equivalent to `mpn_div_2expmod_2expp1` from `fft/div_2expmod_2expp1.c`, FLINT 2.8.0,
// where t == i1.
fn limbs_fft_div_2expmod_2expp1_in_place(xs: &mut [Limb], d: u64) {
    if d != 0 {
        let hi = *xs.last().unwrap();
        let lo = limbs_slice_shr_in_place(xs, d);
        let (last, init) = xs.split_last_mut().unwrap();
        let before_last = init.last_mut().unwrap();
        (*last, *before_last) = Limb::xx_sub_yy_to_zz(
            Limb::wrapping_from(SignedLimb::wrapping_from(hi) >> d),
            *before_last,
            0,
            lo,
        );
    }
}

// This is equivalent to `fft_adjust` from `fft/adjust.c`, FLINT 2.7.1. limbs is one less than the
// length of out and the length of xs.
fn limbs_fft_adjust(out: &mut [Limb], xs: &[Limb], i: usize, w: usize) {
    let n = xs.len();
    assert_eq!(out.len(), n);
    let b = (i).checked_mul(w).unwrap();
    let x = b >> Limb::LOG_WIDTH;
    let b = u64::wrapping_from(b) & Limb::WIDTH_MASK;
    if x == 0 {
        limbs_fft_mul_2expmod_2expp1(out, xs, b);
    } else {
        let (out_last, out_init) = out.split_last_mut().unwrap();
        let (xs_lo, xs_hi) = xs.split_at(n - x - 1);
        out_init[x..].copy_from_slice(xs_lo);
        *out_last = 0;
        let (xs_last, xs_hi_init) = xs_hi.split_last().unwrap();
        let carry = limbs_neg(out, xs_hi_init);
        let out_hi = &mut out[x..];
        limbs_fft_addmod_2expp1_1(out_hi, xs_last.wrapping_neg());
        if carry {
            limbs_sub_limb_in_place(out_hi, 1);
        }
        limbs_fft_mul_2expmod_2expp1_in_place(out, b);
    }
}

// This is equivalent to `fft_adjust_sqrt` from `fft/adjust_sqrt.c`, FLINT 2.7.1. limbs is one less
// than the length of out, xs, and scratch.
fn limbs_fft_adjust_sqrt(out: &mut [Limb], xs: &[Limb], i: usize, w: usize, scratch: &mut [Limb]) {
    let n = out.len();
    assert_ne!(n, 0);
    assert_eq!(xs.len(), n);
    assert_eq!(scratch.len(), n);
    let n = n - 1;
    let wn = n << Limb::LOG_WIDTH;
    let mut b = (i >> 1) + (wn >> 2) + i * (w >> 1);
    let negate = b >= wn;
    if negate {
        b -= wn;
    }
    let y = b >> Limb::LOG_WIDTH;
    let b = u64::wrapping_from(b) & Limb::WIDTH_MASK;
    // multiply by 2^{j + wn/4 + i*k}
    if y == 0 {
        limbs_fft_mul_2expmod_2expp1(out, xs, b);
    } else {
        let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
        let (xs_last, xs_init) = xs.split_last().unwrap();
        let (xs_lo, xs_hi) = xs_init.split_at(n - y);
        scratch_init[y..].copy_from_slice(xs_lo);
        let carry = limbs_neg(scratch_init, xs_hi);
        *scratch_last = 0;
        let scratch_hi = &mut scratch[y..];
        limbs_fft_addmod_2expp1_1(scratch_hi, xs_last.wrapping_neg());
        if carry {
            limbs_sub_limb_in_place(scratch_hi, 1);
        }
        limbs_fft_mul_2expmod_2expp1(out, scratch, b);
    }
    // multiply by 2^{wn/2}
    let y = n >> 1;
    let mut carry = false;
    let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
    let (out_last, out_init) = out.split_last_mut().unwrap();
    let (out_lo, out_hi) = out_init.split_at_mut(n - y);
    scratch_init[y..].copy_from_slice(out_lo);
    *scratch_last = 0;
    if y == 0 {
        fail_on_untested_path("limbs_fft_adjust_sqrt, y == 0 second time");
    } else {
        carry = limbs_neg(scratch, out_hi);
    }
    let scratch_hi = &mut scratch[y..];
    limbs_fft_addmod_2expp1_1(scratch_hi, out_last.wrapping_neg());
    if carry {
        limbs_sub_limb_in_place(scratch_hi, 1);
    }
    if n.odd() {
        limbs_fft_mul_2expmod_2expp1_in_place(scratch, Limb::WIDTH >> 1);
    }
    if negate {
        limbs_sub_same_length_in_place_left(out, scratch);
    } else {
        limbs_sub_same_length_in_place_right(scratch, out);
    }
}

// This is equivalent to `fft_sumdiff` from `fft.h`, FLINT 2.8.0, where all the inputs are distinct.
// n is the length of xs and ys.
fn limbs_fft_sumdiff(sum: &mut [Limb], diff: &mut [Limb], xs: &[Limb], ys: &[Limb]) -> Limb {
    assert_eq!(xs.len(), ys.len());
    if xs.is_empty() {
        0
    } else {
        let mut out = if limbs_add_same_length_to_out(sum, xs, ys) {
            2
        } else {
            0
        };
        if limbs_sub_same_length_to_out(diff, xs, ys) {
            out += 1;
        }
        out
    }
}

// This is equivalent to `butterfly_lshB` from `fft/butterfly_lshB.c`, FLINT 2.8.0.
fn limbs_butterfly_lsh_b(
    ts: &mut [Limb],
    us: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    x: usize,
    y: usize,
) {
    let n = ts.len();
    let n = n - 1;
    match (x, y) {
        (0, 0) => {
            limbs_fft_sumdiff(ts, us, xs, ys);
        }
        (0, y) => {
            let (xs_last, xs_init) = xs.split_last().unwrap();
            let (ys_last, ys_init) = ys.split_last().unwrap();
            let (ts_last, ts_init) = ts.split_last_mut().unwrap();
            let s = n - y;
            let (xs_lo, xs_hi) = xs_init.split_at(s);
            let (ys_lo, ys_hi) = ys_init.split_at(s);
            let (ts_lo, ts_hi) = ts_init.split_at_mut(s);
            let mut carry = limbs_fft_sumdiff(ts_lo, &mut us[y..], xs_lo, ys_lo);
            us[n] = if carry.even() { 0 } else { Limb::MAX };
            let carry_1 = carry >> 1;
            carry = limbs_fft_sumdiff(ts_hi, us, ys_hi, xs_hi);
            *ts_last = carry >> 1;
            limbs_slice_add_limb_in_place(&mut ts[s..], carry_1);
            limbs_fft_addmod_2expp1_1(
                &mut us[y..],
                (if carry.even() { 0 } else { Limb::MAX })
                    .wrapping_add(ys_last.wrapping_sub(*xs_last)),
            );
            limbs_fft_addmod_2expp1_1(ts, xs_last.wrapping_add(*ys_last).wrapping_neg());
        }
        (x, 0) => {
            let (xs_last, xs_init) = xs.split_last().unwrap();
            let (ys_last, ys_init) = ys.split_last().unwrap();
            let (us_last, us_init) = us.split_last_mut().unwrap();
            let (ts_last, ts_init) = ts.split_last_mut().unwrap();
            let s = n - x;
            let (xs_lo, xs_hi) = xs_init.split_at(s);
            let (ys_lo, ys_hi) = ys_init.split_at(s);
            let (us_lo, us_hi) = us_init.split_at_mut(s);
            let mut carry = limbs_fft_sumdiff(&mut ts_init[x..], us_lo, xs_lo, ys_lo);
            *ts_last = carry >> 1;
            let carry_1 = carry.odd();
            carry = limbs_fft_sumdiff(ts, us_hi, xs_hi, ys_hi);
            *us_last = if carry.even() { 0 } else { Limb::MAX };
            if carry_1 {
                limbs_sub_limb_in_place(&mut us[s..], 1);
            }
            let (ts_lo, ts_hi) = ts.split_at_mut(x);
            let mut carry_1 = (carry >> 1).wrapping_neg();
            if limbs_neg_in_place(ts_lo) {
                carry_1.wrapping_sub_assign(1);
            }
            limbs_fft_addmod_2expp1_1(ts_hi, carry_1.wrapping_sub(xs_last.wrapping_add(*ys_last)));
            limbs_fft_addmod_2expp1_1(us, ys_last.wrapping_sub(*xs_last));
        }
        (x, y) => match x.cmp(&y) {
            Greater => {
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let (ys_last, ys_init) = ys.split_last().unwrap();
                let (us_last, us_init) = us.split_last_mut().unwrap();
                let (ts_last, ts_init) = ts.split_last_mut().unwrap();
                let s = n - x;
                let r = x - y;
                let (xs_lo, xs_hi) = xs_init.split_at(s);
                let (xs_mid, xs_hi) = xs_hi.split_at(r);
                let (ys_lo, ys_hi) = ys_init.split_at(s);
                let (ys_mid, ys_hi) = ys_hi.split_at(r);
                let carry = limbs_fft_sumdiff(&mut ts_init[x..], &mut us_init[y..], xs_lo, ys_lo);
                *ts_last = carry >> 1;
                let carry_1 = carry.odd();
                let w = y + s;
                let carry = limbs_fft_sumdiff(ts, &mut us_init[w..], xs_mid, ys_mid);
                *us_last = if carry.even() { 0 } else { Limb::MAX };
                if carry_1 {
                    limbs_sub_limb_in_place(&mut us[w..], 1);
                }
                let mut carry_1 = carry >> 1;
                let (ts_lo, ts_hi) = ts.split_at_mut(r);
                if limbs_neg_in_place(ts_lo) {
                    carry_1 += 1;
                }
                let carry = limbs_fft_sumdiff(&mut ts_hi[..w], &mut us[..n], ys_hi, xs_hi);
                let (ts_mid, ts_hi) = ts_hi.split_at_mut(y);
                let carry_2 = limbs_neg_in_place(ts_mid);
                let carry_3 = limbs_sub_limb_in_place(ts_mid, carry_1);
                carry_1 = (carry >> 1).wrapping_neg();
                if carry_3 {
                    carry_1.wrapping_sub_assign(1);
                }
                if carry_2 {
                    carry_1.wrapping_sub_assign(1);
                }
                limbs_fft_addmod_2expp1_1(
                    ts_hi,
                    carry_1.wrapping_sub(xs_last.wrapping_add(*ys_last)),
                );
                limbs_fft_addmod_2expp1_1(
                    &mut us[y..],
                    (if carry.even() { 0 } else { Limb::MAX })
                        .wrapping_add(*ys_last)
                        .wrapping_sub(*xs_last),
                );
            }
            Less => {
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let (ys_last, ys_init) = ys.split_last().unwrap();
                let (us_last, us_init) = us.split_last_mut().unwrap();
                let (ts_last, ts_init) = ts.split_last_mut().unwrap();
                let s = n - y;
                let r = y - x;
                let (xs_lo, xs_hi) = xs_init.split_at(s);
                let (xs_mid, xs_hi) = xs_hi.split_at(r);
                let (ys_lo, ys_hi) = ys_init.split_at(s);
                let (ys_mid, ys_hi) = ys_hi.split_at(r);
                let carry = limbs_fft_sumdiff(&mut ts_init[x..], &mut us_init[y..], xs_lo, ys_lo);
                *us_last = if carry.even() { 0 } else { Limb::MAX };
                let carry_1 = carry >> 1;
                let w = x + s;
                let carry = limbs_fft_sumdiff(&mut ts_init[w..], us_init, ys_mid, xs_mid);
                *ts_last = carry >> 1;
                limbs_slice_add_limb_in_place(&mut ts[w..], carry_1);
                let us_hi = &mut us[r..];
                let carry_1 = carry.odd();
                let carry = limbs_fft_sumdiff(ts, us_hi, ys_hi, xs_hi);
                let (us_mid, us_hi) = us_hi.split_at_mut(x);
                let carry_3 = carry_1 && limbs_sub_limb_in_place(us_mid, 1);
                let mut carry_1 = if carry.even() { 0 } else { Limb::MAX };
                if carry_3 {
                    carry_1.wrapping_sub_assign(1);
                }
                carry_1.wrapping_add_assign(ys_last.wrapping_sub(*xs_last));
                limbs_fft_addmod_2expp1_1(us_hi, carry_1);
                let (ts_lo, ts_hi) = ts.split_at_mut(x);
                carry_1 = (carry >> 1)
                    .wrapping_neg()
                    .wrapping_sub(xs_last.wrapping_add(*ys_last));
                if limbs_neg_in_place(ts_lo) {
                    carry_1.wrapping_sub_assign(1);
                }
                limbs_fft_addmod_2expp1_1(ts_hi, carry_1);
            }
            Equal => {
                fail_on_untested_path("limbs_butterfly_lsh_b, x != 0 && x == y");
                let (xs_last, xs_init) = xs.split_last().unwrap();
                let (ys_last, ys_init) = ys.split_last().unwrap();
                let (ts_last, ts_init) = ts.split_last_mut().unwrap();
                let (us_last, us_init) = us.split_last_mut().unwrap();
                let s = n - x;
                let (xs_lo, xs_hi) = xs_init.split_at(s);
                let (ys_lo, ys_hi) = ys_init.split_at(s);
                let (ts_lo, ts_hi) = ts_init.split_at_mut(x);
                let (us_lo, us_hi) = us_init.split_at_mut(x);
                let carry = limbs_fft_sumdiff(ts_hi, us_hi, xs_lo, ys_lo);
                *ts_last = carry >> 1;
                *us_last = if carry.even() { 0 } else { Limb::MAX };
                let carry = limbs_fft_sumdiff(ts_lo, us_lo, ys_hi, xs_hi);
                let mut carry_1 = (carry >> 1)
                    .wrapping_neg()
                    .wrapping_sub(xs_last.wrapping_add(*ys_last));
                if limbs_neg_in_place(ts_lo) {
                    carry_1.wrapping_sub_assign(1);
                }
                limbs_fft_addmod_2expp1_1(&mut ts[x..], carry_1);
                limbs_fft_addmod_2expp1_1(
                    &mut us[x..],
                    if carry.even() { 0 } else { Limb::MAX } + *ys_last - *xs_last,
                );
            }
        },
    }
}

// This is equivalent to `butterfly_rshB` from `fft/butterfly_rshB.c`, FLINT 2.8.0.
fn limbs_butterfly_rsh_b(
    ts: &mut [Limb],
    us: &mut [Limb],
    xs: &mut [Limb],
    ys: &mut [Limb],
    x: usize,
    y: usize,
) {
    let n = ts.len() - 1;
    match (x, y) {
        (0, 0) => {
            limbs_fft_sumdiff(ts, us, xs, ys);
        }
        (0, y) => {
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            let (ys_last, ys_init) = ys.split_last_mut().unwrap();
            let s = n - y;
            let (xs_lo, xs_hi) = xs_init.split_at(s);
            let (ys_lo, ys_hi) = ys_init.split_at(y);
            let carry = limbs_fft_sumdiff(ts, us, xs_lo, ys_hi);
            let ts_hi = &mut ts[s..];
            let us_hi = &mut us[s..];
            let (ts_last, ts_init_hi) = ts_hi.split_last_mut().unwrap();
            let (us_last, us_init_hi) = us_hi.split_last_mut().unwrap();
            let carry_1 = carry >> 1;
            let carry_2 = if carry.even() { 0 } else { Limb::MAX };
            let carry = limbs_fft_sumdiff(us_init_hi, ts_init_hi, xs_hi, ys_lo);
            *us_last = (carry >> 1).wrapping_add(*xs_last);
            *ts_last = *xs_last;
            if carry.odd() {
                ts_last.wrapping_sub_assign(1);
            }
            limbs_fft_addmod_2expp1_1(ts_hi, carry_1.wrapping_add(*ys_last));
            limbs_fft_addmod_2expp1_1(us_hi, carry_2.wrapping_sub(*ys_last));
        }
        (x, 0) => {
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            let (ys_last, ys_init) = ys.split_last_mut().unwrap();
            let s = n - x;
            let (xs_lo, xs_hi) = xs_init.split_at_mut(x);
            let (ys_lo, ys_hi) = ys_init.split_at(s);
            let carry = limbs_fft_sumdiff(ts, us, xs_hi, ys_lo);
            let ts_hi = &mut ts[s..];
            let us_hi = &mut us[s..];
            let (ts_last, ts_init_hi) = ts_hi.split_last_mut().unwrap();
            let (us_last, us_init_hi) = us_hi.split_last_mut().unwrap();
            let carry_1 = carry >> 1;
            let carry_2 = if carry.even() { 0 } else { Limb::MAX };
            let carry_3 = limbs_neg_in_place(xs_lo);
            let carry = limbs_fft_sumdiff(ts_init_hi, us_init_hi, xs_lo, ys_hi);
            *us_last = (if carry.even() { 0 } else { Limb::MAX }).wrapping_sub(*ys_last);
            *ts_last = ys_last.wrapping_add(carry >> 1);
            if carry_3 {
                us_last.wrapping_sub_assign(1);
                ts_last.wrapping_sub_assign(1);
            }
            limbs_fft_addmod_2expp1_1(ts_hi, carry_1.wrapping_add(*xs_last));
            limbs_fft_addmod_2expp1_1(us_hi, carry_2.wrapping_add(*xs_last));
        }
        (x, y) => match x.cmp(&y) {
            Greater => {
                let (xs_last, xs_init) = xs.split_last_mut().unwrap();
                let (ys_last, ys_init) = ys.split_last_mut().unwrap();
                let s = n - x;
                let q = n - y;
                let r = x - y;
                let (xs_lo, xs_hi) = xs_init.split_at_mut(r);
                let (xs_mid, xs_hi) = xs_hi.split_at_mut(y);
                let (ys_lo, ys_hi) = ys_init.split_at_mut(y);
                let (ys_mid, ys_hi) = ys_hi.split_at_mut(s);
                let (ts_last, ts_hi) = ts[q..].split_last_mut().unwrap();
                let (us_last, us_hi) = us[q..].split_last_mut().unwrap();
                let carry = limbs_fft_sumdiff(ts_hi, us_hi, ys_lo, xs_mid);
                let carry_3 = limbs_neg_in_place(ts_hi);
                *ts_last = (carry >> 1).wrapping_neg();
                if carry_3 {
                    ts_last.wrapping_sub_assign(1);
                }
                *us_last = if carry.even() { 0 } else { Limb::MAX };
                let carry_3 = limbs_neg_in_place(xs_lo);
                let carry = limbs_fft_sumdiff(&mut ts[s..], &mut us[s..], xs_lo, ys_hi);
                let mut k = (carry >> 1).wrapping_add(*ys_last);
                if carry_3 {
                    k.wrapping_sub_assign(1);
                }
                limbs_fft_addmod_2expp1_1(&mut ts[q..], k);
                let mut k = (if carry.even() { 0 } else { Limb::MAX }).wrapping_sub(*ys_last);
                if carry_3 {
                    k.wrapping_sub_assign(1);
                }
                limbs_fft_addmod_2expp1_1(&mut us[q..], k);
                let carry = limbs_fft_sumdiff(ts, us, xs_hi, ys_mid);
                limbs_fft_addmod_2expp1_1(&mut ts[s..], (carry >> 1).wrapping_add(*xs_last));
                limbs_fft_addmod_2expp1_1(
                    &mut us[s..],
                    (if carry.even() { 0 } else { Limb::MAX }).wrapping_add(*xs_last),
                );
            }
            Less => {
                let (xs_last, xs_init) = xs.split_last_mut().unwrap();
                let (ys_last, ys_init) = ys.split_last_mut().unwrap();
                let (ts_last, ts_init) = ts.split_last_mut().unwrap();
                let (us_last, us_init) = us.split_last_mut().unwrap();
                let s = n - x;
                let q = n - y;
                let r = y - x;
                let (xs_lo, xs_hi) = xs_init.split_at_mut(x);
                let (xs_mid, xs_hi) = xs_hi.split_at_mut(q);
                let (ys_lo, ys_hi) = ys_init.split_at_mut(r);
                let (ys_mid, ys_hi) = ys_hi.split_at_mut(x);
                let ts_hi = &mut ts_init[s..];
                let carry = limbs_fft_sumdiff(ts_hi, &mut us_init[s..], ys_mid, xs_lo);
                let carry_3 = limbs_neg_in_place(ts_hi);
                *ts_last = (carry >> 1).wrapping_neg();
                if carry_3 {
                    ts_last.wrapping_sub_assign(1);
                }
                *us_last = if carry.even() { 0 } else { Limb::MAX };
                let carry_3 = limbs_neg_in_place(ys_lo);
                let carry = limbs_fft_sumdiff(&mut ts_init[q..], &mut us_init[q..], xs_hi, ys_lo);
                let mut k = (carry >> 1).wrapping_add(*xs_last);
                if carry_3 {
                    k.wrapping_sub_assign(1);
                }
                limbs_fft_addmod_2expp1_1(&mut ts[s..], k);
                let mut k = (if carry.even() { 0 } else { Limb::MAX }).wrapping_add(*xs_last);
                if carry_3 {
                    k.wrapping_add_assign(1);
                }
                limbs_fft_addmod_2expp1_1(&mut us[s..], k);
                let carry = limbs_fft_sumdiff(ts, us, xs_mid, ys_hi);
                limbs_fft_addmod_2expp1_1(&mut ts[q..], (carry >> 1).wrapping_add(*ys_last));
                limbs_fft_addmod_2expp1_1(
                    &mut us[q..],
                    (if carry.even() { 0 } else { Limb::MAX }).wrapping_sub(*ys_last),
                );
            }
            Equal => {
                fail_on_untested_path("limbs_butterfly_lsh_b, x != 0 && x == y");
                let (xs_last, xs_init) = xs.split_last_mut().unwrap();
                let (ys_last, ys_init) = ys.split_last_mut().unwrap();
                let (xs_lo, xs_hi) = xs_init.split_at(x);
                let (ys_lo, ys_hi) = ys_init.split_at(y);
                let carry = limbs_fft_sumdiff(ts, us, xs_hi, ys_hi);
                let r = n - x;
                let (ts_last, ts_hi) = ts[r..].split_last_mut().unwrap();
                let (us_last, us_hi) = us[r..].split_last_mut().unwrap();
                let carry_1 = carry >> 1;
                let carry_2 = if carry.even() { 0 } else { Limb::MAX };
                let carry = limbs_fft_sumdiff(ts_hi, us_hi, ys_lo, xs_lo);
                *us_last = if carry.even() { 0 } else { Limb::MAX };
                *ts_last = (carry >> 1).wrapping_neg();
                if limbs_neg_in_place(ts_hi) {
                    ts_last.wrapping_sub_assign(1);
                }
                limbs_fft_addmod_2expp1_1(ts_hi, carry_1 + *xs_last + *ys_last);
                limbs_fft_addmod_2expp1_1(us_hi, carry_2 + *xs_last - *ys_last);
            }
        },
    }
}

// This is equivalent to `fft_combine_bits` from `fft/combine_bits.c`, FLINT 2.8.0.
fn limbs_fft_combine_bits(
    mut out: &mut [Limb],
    poly: &mut [&mut [Limb]],
    bits: usize,
    m: usize,
    scratch: &mut [Limb],
) {
    let top_bits = u64::wrapping_from(bits) & Limb::WIDTH_MASK;
    let n = bits >> Limb::LOG_WIDTH;
    if top_bits == 0 {
        let (poly_last, poly_init) = poly.split_last().unwrap();
        for xs in poly_init {
            limbs_slice_add_greater_in_place_left(out, &xs[..m]);
            out = &mut out[n..];
        }
        limbs_slice_add_greater_in_place_left(out, poly_last);
        return;
    }
    let m = m + 1;
    let mut shift_bits = 0;
    let mut remaining_len = out.len();
    let mut poly_iter = poly.iter();
    for xs in &mut poly_iter {
        let xs = &xs[..m];
        let out_slice = &mut out[..m];
        if shift_bits == 0 {
            limbs_slice_add_greater_in_place_left(out_slice, xs);
        } else {
            limbs_shl_to_out(scratch, xs, shift_bits);
            limbs_slice_add_same_length_in_place_left(out_slice, &scratch[..m]);
        }
        shift_bits += top_bits;
        remaining_len -= n;
        out = &mut out[n..];
        if shift_bits >= Limb::WIDTH {
            remaining_len -= 1;
            out = &mut out[1..];
            shift_bits -= Limb::WIDTH;
        }
        if remaining_len <= m {
            break;
        }
    }
    for xs in poly_iter {
        if shift_bits != 0 {
            limbs_shl_to_out(scratch, &xs[..m], shift_bits);
            limbs_slice_add_same_length_in_place_left(out, &scratch[..remaining_len]);
        } else {
            limbs_slice_add_same_length_in_place_left(out, &xs[..remaining_len]);
        }
        if remaining_len <= n {
            return;
        }
        remaining_len -= n;
        shift_bits += top_bits;
        out = &mut out[n..];
        if shift_bits >= Limb::WIDTH {
            remaining_len -= 1;
            out = &mut out[1..];
            shift_bits -= Limb::WIDTH;
        }
    }
}

// This is equivalent to `fft_split_bits` from `fft/split_bits.c`, FLINT 2.8.0.
fn limbs_fft_split_bits(poly: &mut [&mut [Limb]], xs: &[Limb], bits: usize) -> usize {
    let len = xs.len();
    let length = ((len << Limb::LOG_WIDTH) - 1) / bits + 1;
    let top_bits = u64::wrapping_from(bits) & Limb::WIDTH_MASK;
    let m = bits >> Limb::LOG_WIDTH;
    if top_bits == 0 {
        let len = xs.len();
        let length = (len - 1) / m + 1;
        let num = len / m;
        let mut polys = poly.iter_mut();
        let mut xs_chunks = xs.chunks_exact(m);
        for (ps, xs) in (&mut polys).zip(&mut xs_chunks) {
            slice_set_zero(&mut ps[m..]);
            ps[..m].copy_from_slice(xs);
        }
        let mut last_poly = polys.next();
        if num < length {
            fail_on_untested_path("limbs_fft_split_limbs, num < length");
            slice_set_zero(last_poly.as_mut().unwrap());
        }
        let xs_last = xs_chunks.remainder();
        let xs_last_len = xs_last.len();
        if xs_last_len != 0 {
            fail_on_untested_path("limbs_fft_split_limbs, xs_last_len != 0");
            last_poly.unwrap()[..xs_last_len].copy_from_slice(xs_last);
        }
        return length;
    }
    let coeff_limbs = m + 1;
    let mask = Limb::low_mask(top_bits);
    let mut total_bits = 0;
    let mut q = 0;
    let (poly_last, poly_init) = poly[..length].split_last_mut().unwrap();
    for ps in &mut *poly_init {
        slice_set_zero(ps);
        let xs = &xs[q + usize::exact_from(total_bits >> Limb::LOG_WIDTH)..];
        let mut shift_bits = total_bits & Limb::WIDTH_MASK;
        if shift_bits == 0 {
            ps[..coeff_limbs].copy_from_slice(&xs[..coeff_limbs]);
            ps[coeff_limbs - 1] &= mask;
        } else {
            limbs_shr_to_out(ps, &xs[..coeff_limbs], shift_bits);
            shift_bits += top_bits;
            let ps_last = ps[..coeff_limbs].last_mut().unwrap();
            if shift_bits >= Limb::WIDTH {
                *ps_last += xs[coeff_limbs] << (Limb::WIDTH - (shift_bits - top_bits));
            }
            *ps_last &= mask;
        }
        q += coeff_limbs - 1;
        total_bits += top_bits;
    }
    let xs = &xs[q + usize::exact_from(total_bits >> Limb::LOG_WIDTH)..];
    let shift_bits = total_bits & Limb::WIDTH_MASK;
    slice_set_zero(poly_last);
    if shift_bits == 0 {
        poly_last[..xs.len()].copy_from_slice(xs);
    } else {
        limbs_shr_to_out(poly_last, xs, shift_bits);
    }
    length
}

// This is equivalent to `fft_butterfly` from `fft/fft_radix2.c`, FLINT 2.8.0.
fn limbs_fft_butterfly(
    ss: &mut [Limb],
    ts: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    i: usize,
    w: usize,
) {
    let n = ss.len();
    assert_ne!(n, 0);
    let b = i * w;
    let y = b >> Limb::LOG_WIDTH;
    limbs_butterfly_lsh_b(ss, ts, xs, ys, 0, y);
    limbs_fft_mul_2expmod_2expp1_in_place(ts, u64::wrapping_from(b) & Limb::WIDTH_MASK);
}

// This is equivalent to `fft_radix2` from `fft/fft_radix2.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_fft_radix2<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
) {
    let n = xss.len() >> 1;
    let (xss_lo, xss_hi) = xss.split_at_mut(n);
    if n == 1 {
        limbs_fft_butterfly(ts, us, xss_lo[0], xss_hi[0], 0, w);
        swap(&mut xss_lo[0], ts);
        swap(&mut xss_hi[0], us);
        return;
    }
    for (i, (xs_lo, xs_hi)) in xss_lo.iter_mut().zip(xss_hi.iter_mut()).enumerate() {
        limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
        swap(xs_lo, ts);
        swap(xs_hi, us);
    }
    let two_w = w << 1;
    limbs_fft_radix2(xss_lo, two_w, ts, us);
    limbs_fft_radix2(xss_hi, two_w, ts, us);
}

// This is equivalent to `fft_truncate1` from `fft/fft_truncate.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_fft_truncate1<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    trunc: usize,
) {
    let two_n = xss.len();
    let n = two_n >> 1;
    if trunc == two_n {
        limbs_fft_radix2(xss, w, ts, us);
    } else {
        let (xss_lo, xss_hi) = xss.split_at_mut(n);
        let two_w = w << 1;
        if trunc <= n {
            for (xs_lo, xs_hi) in xss_lo.iter_mut().zip(xss_hi.iter_mut()) {
                limbs_slice_add_same_length_in_place_left(xs_lo, xs_hi);
            }
            limbs_fft_truncate1(xss_lo, two_w, ts, us, trunc);
        } else {
            for (i, (xs_lo, xs_hi)) in xss_lo.iter_mut().zip(xss_hi.iter_mut()).enumerate() {
                limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
            }
            limbs_fft_radix2(xss_lo, two_w, ts, us);
            limbs_fft_truncate1(xss_hi, two_w, ts, us, trunc - n);
        }
    }
}

// This is equivalent to `fft_butterfly_sqrt` from `fft/fft_truncate.c`, FLINT 2.8.0.
fn limbs_fft_butterfly_sqrt(
    ss: &mut [Limb],
    ts: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    i: usize,
    w: usize,
    scratch: &mut [Limb],
) {
    let n = ss.len() - 1;
    let wn = n << Limb::LOG_WIDTH;
    let j = i >> 1;
    let k = w >> 1;
    let mut b = j + (wn >> 2) + i * k;
    let negate = b >= wn;
    if negate {
        b -= wn;
    }
    let y = b >> Limb::LOG_WIDTH;
    let b = u64::wrapping_from(b) & Limb::WIDTH_MASK;
    // sumdiff and multiply by 2^{j + wn/4 + i*k}
    limbs_butterfly_lsh_b(ss, ts, xs, ys, 0, y);
    limbs_fft_mul_2expmod_2expp1_in_place(ts, b);
    // multiply by 2^{wn/2}
    let y = n >> 1;
    let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
    let (ts_last, ts_init) = ts.split_last_mut().unwrap();
    let (ts_lo, ts_hi) = ts_init.split_at_mut(n - y);
    scratch_init[y..].copy_from_slice(ts_lo);
    *scratch_last = 0;
    assert_ne!(y, 0);
    let carry = limbs_neg(scratch_init, ts_hi);
    let scratch_hi = &mut scratch[y..];
    limbs_fft_addmod_2expp1_1(scratch_hi, ts_last.wrapping_neg());
    if carry {
        limbs_sub_limb_in_place(scratch_hi, 1);
    }
    // shift by an additional half limb (rare)
    if n.odd() {
        limbs_fft_mul_2expmod_2expp1_in_place(scratch, Limb::WIDTH >> 1);
    }
    // subtract
    if negate {
        limbs_sub_same_length_in_place_left(ts, scratch);
    } else {
        limbs_sub_same_length_in_place_right(scratch, ts);
    }
}

// This is equivalent to `fft_truncate_sqrt` from `fft/fft_truncate.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_fft_truncate_sqrt<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    scratch: &mut [Limb],
    trunc: usize,
) {
    if w.even() {
        let w = w >> 1;
        let two_n = xss.len();
        let n = two_n >> 1;
        if trunc == two_n {
            limbs_fft_radix2(xss, w, ts, us);
        } else {
            assert!(trunc > n);
            let (xss_lo, xss_hi) = xss.split_at_mut(n);
            let two_w = w << 1;
            let n_comp = trunc - n;
            let (xss_lo_lo, xss_lo_hi) = xss_lo.split_at_mut(n_comp);
            let (xss_hi_lo, xss_hi_hi) = xss_hi.split_at_mut(n_comp);
            for (i, (xs_lo, xs_hi)) in xss_lo_lo.iter_mut().zip(xss_hi_lo.iter_mut()).enumerate() {
                limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
            }
            for (i, (xs_lo, xs_hi)) in xss_lo_hi.iter_mut().zip(xss_hi_hi.iter_mut()).enumerate() {
                limbs_fft_adjust(xs_hi, xs_lo, i + n_comp, w);
            }
            limbs_fft_radix2(xss_lo, two_w, ts, us);
            limbs_fft_truncate1(xss_hi, two_w, ts, us, n_comp);
        }
        return;
    }
    let two_n = xss.len() >> 1;
    let n = two_n >> 1;
    let (xss_lo, xss_hi) = xss.split_at_mut(two_n);
    let trunc = trunc - two_n;
    let mut i = 0;
    while i < trunc {
        limbs_fft_butterfly(ts, us, xss_lo[i], xss_hi[i], i >> 1, w);
        swap(&mut xss_lo[i], ts);
        swap(&mut xss_hi[i], us);
        i += 1;
        limbs_fft_butterfly_sqrt(ts, us, xss_lo[i], xss_hi[i], i, w, scratch);
        swap(&mut xss_lo[i], ts);
        swap(&mut xss_hi[i], us);
        i += 1;
    }
    while i < n << 1 {
        limbs_fft_adjust(xss_hi[i], xss_lo[i], i >> 1, w);
        i += 1;
        limbs_fft_adjust_sqrt(xss_hi[i], xss_lo[i], i, w, scratch);
        i += 1;
    }
    limbs_fft_radix2(xss_lo, w, ts, us);
    limbs_fft_truncate1(xss_hi, w, ts, us, trunc);
}

// This is equivalent to `ifft_butterfly` from `fft/ifft_radix2.c`, FLINT 2.8.0.
fn limbs_ifft_butterfly(
    s: &mut [Limb],
    t: &mut [Limb],
    xs: &mut [Limb],
    ys: &mut [Limb],
    i: usize,
    w: usize,
) {
    let b = i * w;
    let y = b >> Limb::LOG_WIDTH;
    let b = u64::wrapping_from(b) & Limb::WIDTH_MASK;
    limbs_fft_div_2expmod_2expp1_in_place(ys, b);
    limbs_butterfly_rsh_b(s, t, xs, ys, 0, y);
}

// This is equivalent to `ifft_radix2` from `fft/ifft_radix2.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_radix2<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
) {
    let n = xss.len() >> 1;
    let (xss_lo, xss_hi) = xss.split_at_mut(n);
    if n == 1 {
        limbs_ifft_butterfly(ts, us, xss_lo[0], xss_hi[0], 0, w);
        swap(&mut xss_lo[0], ts);
        swap(&mut xss_hi[0], us);
        return;
    }
    let two_w = w << 1;
    limbs_ifft_radix2(xss_lo, two_w, ts, us);
    limbs_ifft_radix2(xss_hi, two_w, ts, us);
    for (i, (xs_lo, xs_hi)) in xss_lo.iter_mut().zip(xss_hi.iter_mut()).enumerate() {
        limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
        swap(xs_lo, ts);
        swap(xs_hi, us);
    }
}

// This is equivalent to `ifft_truncate1` from `fft/ifft_truncate.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_truncate1<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    trunc: usize,
) {
    let n = xss.len();
    if trunc == n {
        limbs_ifft_radix2(xss, w, ts, us);
    } else {
        let n = n >> 1;
        let (xss_lo, xss_hi) = xss.split_at_mut(n);
        let two_w = w << 1;
        if trunc <= n {
            for (xs_lo, xs_hi) in xss_lo[trunc..].iter_mut().zip(xss_hi[trunc..].iter_mut()) {
                limbs_slice_add_same_length_in_place_left(xs_lo, xs_hi);
                limbs_fft_div_2expmod_2expp1_in_place(xs_lo, 1);
            }
            limbs_ifft_truncate1(xss_lo, two_w, ts, us, trunc);
            for (xs_lo, xs_hi) in xss_lo.iter_mut().zip(xss_hi.iter_mut()).take(trunc) {
                limbs_slice_shl_in_place(xs_lo, 1);
                limbs_sub_same_length_in_place_left(xs_lo, xs_hi);
            }
        } else {
            limbs_ifft_radix2(xss_lo, two_w, ts, us);
            let trunc = trunc - n;
            for (i, (xs_lo, xs_hi)) in xss_lo[trunc..]
                .iter_mut()
                .zip(xss_hi[trunc..].iter_mut())
                .enumerate()
            {
                limbs_sub_same_length_in_place_right(xs_lo, xs_hi);
                limbs_fft_adjust(ts, xs_hi, i + trunc, w);
                limbs_slice_add_same_length_in_place_left(xs_lo, xs_hi);
                swap(xs_hi, ts);
            }
            limbs_ifft_truncate1(xss_hi, two_w, ts, us, trunc);
            for (i, (xs_lo, xs_hi)) in xss_lo
                .iter_mut()
                .zip(xss_hi.iter_mut())
                .take(trunc)
                .enumerate()
            {
                limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
            }
        }
    }
}

// This is equivalent to `ifft_truncate` from `fft/ifft_truncate.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_truncate<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    trunc: usize,
) {
    let n = xss.len();
    if trunc == n {
        limbs_ifft_radix2(xss, w, ts, us);
    } else {
        let n = n >> 1;
        let two_w = w << 1;
        let (xss_lo, xss_hi) = xss.split_at_mut(n);
        if trunc <= n {
            fail_on_untested_path("limbs_ifft_truncate, trunc != n << 1 && trunc <= n");
            limbs_ifft_truncate(xss_lo, two_w, ts, us, trunc);
            for xs in xss_lo.iter_mut().take(trunc) {
                limbs_slice_shl_in_place(xs, 1);
            }
        } else {
            let trunc = trunc - n;
            limbs_ifft_radix2(xss_lo, two_w, ts, us);
            for (i, (xs_lo, xs_hi)) in xss_lo[trunc..]
                .iter_mut()
                .zip(xss_hi[trunc..].iter_mut())
                .enumerate()
            {
                limbs_fft_adjust(xs_hi, xs_lo, i + trunc, w);
            }
            limbs_ifft_truncate1(xss_hi, two_w, ts, us, trunc);
            for (i, (xs_lo, xs_hi)) in xss_lo
                .iter_mut()
                .zip(xss_hi.iter_mut())
                .enumerate()
                .take(trunc)
            {
                limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
            }
            for xs in &mut xss_lo[trunc..] {
                limbs_slice_shl_in_place(xs, 1);
            }
        }
    }
}

// This is equivalent to `ifft_butterfly_sqrt` from `fft/ifft_truncate_sqrt.c`, FLINT 2.8.0.
fn limbs_ifft_butterfly_sqrt(
    ss: &mut [Limb],
    ts: &mut [Limb],
    xs: &mut [Limb],
    ys: &mut [Limb],
    i: usize,
    w: usize,
    scratch: &mut [Limb],
) {
    let n = ss.len() - 1;
    let wn = n << Limb::LOG_WIDTH;
    let mut b = wn - (i >> 1) - i * (w >> 1) - 1 + (wn >> 2);
    let negate = b < wn;
    if !negate {
        b -= wn;
    }
    let y2 = b >> Limb::LOG_WIDTH;
    let b = u64::wrapping_from(b) & Limb::WIDTH_MASK;
    if b != 0 {
        limbs_fft_mul_2expmod_2expp1_in_place(ys, b);
    }
    let y = n >> 1;
    let (scratch_last, scratch_init) = scratch.split_last_mut().unwrap();
    let (ys_last, ys_init) = ys.split_last().unwrap();
    let (ys_lo, ys_hi) = ys_init.split_at(n - y);
    scratch_init[y..].copy_from_slice(ys_lo);
    *scratch_last = 0;
    assert_ne!(y, 0);
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(y);
    let carry = limbs_neg(scratch_lo, ys_hi);
    limbs_fft_addmod_2expp1_1(scratch_hi, ys_last.wrapping_neg());
    if carry {
        limbs_sub_limb_in_place(scratch_hi, 1);
    }
    if n.odd() {
        limbs_fft_mul_2expmod_2expp1_in_place(scratch, Limb::WIDTH >> 1);
    }
    if negate {
        limbs_sub_same_length_in_place_right(scratch, ys);
    } else {
        limbs_sub_same_length_in_place_left(ys, scratch);
    }
    limbs_butterfly_rsh_b(ss, ts, xs, ys, 0, n - y2);
}

// This is equivalent to `ifft_truncate_sqrt` from `fft/ifft_truncate_sqrt.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_truncate_sqrt<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    scratch: &mut &'a mut [Limb],
    trunc: usize,
) {
    let two_n = xss.len() >> 1;
    let n = two_n >> 1;
    if w.even() {
        limbs_ifft_truncate(xss, w >> 1, ts, us, trunc);
        return;
    }
    let (xss_lo, xss_hi) = xss.split_at_mut(two_n);
    limbs_ifft_radix2(xss_lo, w, ts, us);
    let limit = trunc - two_n;
    let mut i = limit;
    while i < n << 1 {
        limbs_fft_adjust(xss_hi[i], xss_lo[i], i >> 1, w);
        i += 1;
        limbs_fft_adjust_sqrt(xss_hi[i], xss_lo[i], i, w, scratch);
        i += 1;
    }
    limbs_ifft_truncate1(xss_hi, w, ts, us, limit);
    let mut i = 0;
    while i < limit {
        let xs_lo = &mut xss_lo[i];
        let xs_hi = &mut xss_hi[i];
        limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i >> 1, w);
        swap(xs_lo, ts);
        swap(xs_hi, us);
        i += 1;
        let xs_lo = &mut xss_lo[i];
        let xs_hi = &mut xss_hi[i];
        limbs_ifft_butterfly_sqrt(ts, us, xs_lo, xs_hi, i, w, scratch);
        swap(xs_lo, ts);
        swap(xs_hi, us);
        i += 1;
    }
    for xs in &mut xss_lo[limit..] {
        limbs_slice_shl_in_place(xs, 1);
    }
}

// This is equivalent to `fft_radix2_twiddle` from `fft/fft_mfa_truncate_sqrt.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_fft_radix2_twiddle<'a>(
    xss: &mut [&'a mut [Limb]],
    m: usize,
    n: usize,
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    v: usize,
    r: usize,
    c: usize,
    q: usize,
) {
    let (xss_lo, xss_hi) = xss.split_at_mut(n * m);
    if n == 1 {
        let tw1 = r * c;
        let tw2 = tw1 + q * c;
        let xs_lo = &mut xss_lo[0];
        let xs_hi = &mut xss_hi[0];
        let mut b1 = tw1 * v;
        let mut b2 = tw2 * v;
        let limbs = ts.len() - 1;
        let nw = limbs << Limb::LOG_WIDTH;
        let negate2 = b1 >= nw;
        if negate2 {
            fail_on_untested_path("limbs_fft_butterfly_twiddle, negate2");
            b1 -= nw;
        }
        let x = b1 >> Limb::LOG_WIDTH;
        let b1 = u64::wrapping_from(b1) & Limb::WIDTH_MASK;
        let negate1 = b2 >= nw;
        if negate1 {
            b2 -= nw;
        }
        let y = b2 >> Limb::LOG_WIDTH;
        let b2 = u64::wrapping_from(b2) & Limb::WIDTH_MASK;
        limbs_butterfly_lsh_b(ts, us, xs_lo, xs_hi, x, y);
        limbs_fft_mul_2expmod_2expp1_in_place(ts, b1);
        if negate2 {
            limbs_neg_in_place(ts);
        }
        limbs_fft_mul_2expmod_2expp1_in_place(us, b2);
        if negate1 {
            limbs_neg_in_place(us);
        }
        swap(xs_lo, ts);
        swap(xs_hi, us);
        return;
    }
    let mut j = 0;
    for i in 0..n {
        let xs_lo = &mut xss_lo[j];
        let xs_hi = &mut xss_hi[j];
        limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
        swap(xs_lo, ts);
        swap(xs_hi, us);
        j += m;
    }
    let half_n = n >> 1;
    let two_w = w << 1;
    let two_q = q << 1;
    limbs_fft_radix2_twiddle(xss_lo, m, half_n, two_w, ts, us, v, r, c, two_q);
    limbs_fft_radix2_twiddle(xss_hi, m, half_n, two_w, ts, us, v, r + q, c, two_q);
}

// Computes the reverse binary representation of a number of b bits.
//
// This is equivalent to `n_revbin` from `ulong_extras/revbin.c`, FLINT 2.8.0.
const fn n_revbin(n: usize, b: u64) -> usize {
    n.reverse_bits() >> (usize::WIDTH - b)
}

// This is equivalent to `fft_truncate1_twiddle` from `fft/fft_mfa_truncate_sqrt.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_fft_truncate1_twiddle<'a>(
    xss: &mut [&'a mut [Limb]],
    m: usize,
    n: usize,
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    v: usize,
    r: usize,
    c: usize,
    q: usize,
    trunc: usize,
) {
    if trunc == n << 1 {
        limbs_fft_radix2_twiddle(xss, m, n, w, ts, us, v, r, c, q);
    } else {
        let (xss_lo, xss_hi) = xss.split_at_mut(n * m);
        let mut j = 0;
        if trunc <= n {
            for _ in 0..n {
                limbs_slice_add_same_length_in_place_left(xss_lo[j], xss_hi[j]);
                j += m;
            }
            limbs_fft_truncate1_twiddle(xss, m, n >> 1, w << 1, ts, us, v, r, c, q << 1, trunc);
        } else {
            for i in 0..n {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
                j += m;
            }
            let half_n = n >> 1;
            let two_w = w << 1;
            let two_q = q << 1;
            limbs_fft_radix2_twiddle(xss_lo, m, half_n, two_w, ts, us, v, r, c, two_q);
            limbs_fft_truncate1_twiddle(
                xss_hi,
                m,
                half_n,
                two_w,
                ts,
                us,
                v,
                r + q,
                c,
                two_q,
                trunc - n,
            );
        }
    }
}

// This is equivalent to `fft_mfa_truncate_sqrt_outer` from `fft/fft_mfa_truncate_sqrt.c`, FLINT
// 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_fft_mfa_truncate_sqrt_outer<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    scratch: &mut &'a mut [Limb],
    xs_len: usize,
    trunc: usize,
) {
    let two_n = xss.len() >> 1;
    let ys_len = two_n / xs_len;
    let trunc_comp = trunc - two_n;
    let trunc2 = trunc_comp / xs_len;
    let depth = ys_len.ceiling_log_base_2();
    let (xss_lo, xss_hi) = xss.split_at_mut(two_n);
    let half_ys_len = ys_len >> 1;
    let wx = w * xs_len;
    // - first half matrix fourier FFT : ys_len rows, xs_len cols
    // - FFTs on columns
    for i in 0..xs_len {
        // relevant part of first layer of full sqrt FFT
        if w.odd() {
            let mut j = i;
            while j < trunc_comp {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                if j.odd() {
                    limbs_fft_butterfly_sqrt(ts, us, xs_lo, xs_hi, j, w, scratch);
                } else {
                    limbs_fft_butterfly(ts, us, xs_lo, xs_hi, j >> 1, w);
                }
                swap(xs_lo, ts);
                swap(xs_hi, us);
                j += xs_len;
            }
            while j < two_n {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                if i.odd() {
                    limbs_fft_adjust_sqrt(xs_hi, xs_lo, j, w, scratch);
                } else {
                    limbs_fft_adjust(xs_hi, xs_lo, j >> 1, w);
                }
                j += xs_len;
            }
        } else {
            let half_w = w >> 1;
            let mut j = i;
            while j < trunc_comp {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                limbs_fft_butterfly(ts, us, xs_lo, xs_hi, j, half_w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
                j += xs_len;
            }
            while j < two_n {
                limbs_fft_adjust(xss_hi[j], xss_lo[j], j, half_w);
                j += xs_len;
            }
        }
        //  FFT of length ys_len on column i, applying z^{r*i} for rows going up in steps of 1
        //  starting at row 0, where z => w bits
        let xss_mid = &mut xss_lo[i..];
        limbs_fft_radix2_twiddle(xss_mid, xs_len, half_ys_len, wx, ts, us, w, 0, i, 1);
        let mut k = 0;
        for j in 0..ys_len {
            let s = n_revbin(j, depth);
            if j < s {
                xss_mid.swap(k, s * xs_len);
            }
            k += xs_len;
        }
    }
    // - second half matrix fourier FFT : ys_len rows, xs_len cols
    // - FFTs on columns
    for i in 0..xs_len {
        //  FFT of length ys_len on column i, applying z^{r*i} for rows going up in steps of 1
        //  starting at row 0, where z => w bits
        let xss_hi = &mut xss_hi[i..];
        limbs_fft_truncate1_twiddle(xss_hi, xs_len, half_ys_len, wx, ts, us, w, 0, i, 1, trunc2);
        let mut k = 0;
        for j in 0..ys_len {
            let s = n_revbin(j, depth);
            if j < s {
                xss_hi.swap(k, s * xs_len);
            }
            k += xs_len;
        }
    }
}

// This is equivalent to `fft_negacyclic` from `fft/fft_negacylic.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_fft_negacyclic<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    scratch: &mut &'a mut [Limb],
) {
    let n = xss.len() >> 1;
    let (xss_lo, xss_hi) = xss.split_at_mut(n);
    // first apply twiddle factors corresponding to shifts of w*i/2 bits
    if w.odd() {
        fail_on_untested_path("limbs_fft_negacyclic, w.odd()");
        let mut i = 0;
        while i < n {
            let xs_lo = &mut xss_lo[i];
            let xs_hi = &mut xss_hi[i];
            limbs_fft_adjust(ts, xs_lo, i >> 1, w);
            swap(xs_lo, ts);
            limbs_fft_adjust(us, xs_hi, (n + i) >> 1, w);
            swap(xs_hi, us);
            limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
            swap(xs_lo, ts);
            swap(xs_hi, us);
            i += 1;
            let xs_lo = &mut xss_lo[i];
            let xs_hi = &mut xss_hi[i];
            limbs_fft_adjust_sqrt(ts, xs_lo, i, w, scratch);
            swap(xs_lo, ts);
            limbs_fft_adjust_sqrt(us, xs_hi, n + i, w, scratch);
            swap(xs_hi, us);
            limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
            swap(xs_lo, ts);
            swap(xs_hi, us);
            i += 1;
        }
    } else {
        let half_w = w >> 1;
        for (i, (xs_lo, xs_hi)) in xss_lo.iter_mut().zip(xss_hi.iter_mut()).enumerate() {
            limbs_fft_adjust(ts, xs_lo, i, half_w);
            swap(xs_lo, ts);
            limbs_fft_adjust(us, xs_hi, n + i, half_w);
            swap(xs_hi, us);
            limbs_fft_butterfly(ts, us, xs_lo, xs_hi, i, w);
            swap(xs_lo, ts);
            swap(xs_hi, us);
        }
    }
    let two_w = w << 1;
    limbs_fft_radix2(xss_lo, two_w, ts, us);
    limbs_fft_radix2(xss_hi, two_w, ts, us);
}

// This is equivalent to `ifft_negacyclic` from `fft/ifft_negacylic.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_negacyclic<'a>(
    xss: &mut [&'a mut [Limb]],
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    scratch: &mut &'a mut [Limb],
) {
    let two_n = xss.len();
    let n = two_n >> 1;
    let (xss_lo, xss_hi) = xss.split_at_mut(n);
    let two_w = w << 1;
    limbs_ifft_radix2(xss_lo, two_w, ts, us);
    limbs_ifft_radix2(xss_hi, two_w, ts, us);
    if w.odd() {
        let mut i = 0;
        while i < n {
            let xs_lo = &mut xss_lo[i];
            let xs_hi = &mut xss_hi[i];
            limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
            swap(xs_lo, ts);
            swap(xs_hi, us);
            limbs_fft_adjust(ts, xs_lo, n - (i >> 1), w);
            limbs_neg_in_place(ts);
            swap(xs_lo, ts);
            limbs_fft_adjust(us, xs_hi, n - ((n + i) >> 1), w);
            limbs_neg_in_place(us);
            swap(xs_hi, us);
            i += 1;
            let xs_lo = &mut xss_lo[i];
            let xs_hi = &mut xss_hi[i];
            limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
            swap(xs_lo, ts);
            swap(xs_hi, us);
            limbs_fft_adjust_sqrt(ts, xs_lo, two_n - i, w, scratch);
            limbs_neg_in_place(ts);
            swap(xs_lo, ts);
            limbs_fft_adjust_sqrt(us, xs_hi, n - i, w, scratch);
            limbs_neg_in_place(us);
            swap(xs_hi, us);
            i += 1;
        }
    } else {
        let half_w = w >> 1;
        for (i, (xs_lo, xs_hi)) in xss_lo.iter_mut().zip(xss_hi.iter_mut()).enumerate() {
            limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
            swap(xs_lo, ts);
            swap(xs_hi, us);
            limbs_fft_adjust(ts, xs_lo, two_n - i, half_w);
            limbs_neg_in_place(ts);
            swap(xs_lo, ts);
            limbs_fft_adjust(us, xs_hi, n - i, half_w);
            limbs_neg_in_place(us);
            swap(xs_hi, us);
        }
    }
}

const U_WIDTH: usize = Limb::WIDTH as usize;

fn limbs_fft_mulmod_2expp1_scratch_len(n: usize, w: usize) -> usize {
    let bits = n * w;
    let depth = bits.ceiling_log_base_2();
    let off = if depth < 12 {
        MULMOD_TAB[0]
    } else {
        MULMOD_TAB[min(usize::exact_from(depth), MULMOD_TAB.len() + 11) - 12]
    };
    let depth = (depth >> 1) - u64::from(off);
    let w = bits >> (depth << 1);
    let n = usize::power_of_2(depth);
    let size = ((n * w) >> Limb::LOG_WIDTH) + 1;
    let b = n * w;
    let n_2 = (b + U_WIDTH - 1) >> Limb::LOG_WIDTH;
    max(
        size + 1,
        limbs_fft_mulmod_2expp1_basecase_same_scratch_len(n_2),
    ) + (n << 1)
}

// This is equivalent to `fft_mulmod_2expp1` from `fft/mulmod_2expp1.c`, FLINT 2.8.0, where r == i1
// != i2 and (n * w) >> Limb::LOG_WIDTH > cutoff
#[allow(clippy::mut_mut)]
fn limbs_fft_mulmod_2expp1<'a>(
    xs: &mut [Limb],
    ys: &[Limb],
    n: usize,
    w: usize,
    xss: &mut [&'a mut [Limb]],
    xss0: &mut [Limb],
    yss: &mut [&'a mut [Limb]],
    yss0: &mut [Limb],
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    ss: &mut &'a mut [Limb],
    scratch: &mut [Limb],
) {
    let bits = n * w;
    let limbs = bits >> Limb::LOG_WIDTH;
    assert_eq!(xs[limbs], 0);
    assert_eq!(ys[limbs], 0);
    let depth = bits.ceiling_log_base_2();
    let off = if depth < 12 {
        MULMOD_TAB[0]
    } else {
        MULMOD_TAB[min(usize::exact_from(depth), MULMOD_TAB.len() + 11) - 12]
    };
    let depth = (depth >> 1) - u64::from(off);
    let w = bits >> (depth << 1);
    let n = usize::power_of_2(depth);
    let bits = (limbs << Limb::LOG_WIDTH) / (n << 1);
    let size = ((n * w) >> Limb::LOG_WIDTH) + 1;
    let two_n = n << 1;
    let (out, scratch) = scratch.split_at_mut(two_n);
    let xs = &mut xs[..=limbs];
    let (_, xs_init) = xs.split_last_mut().unwrap();
    let j = limbs_fft_split_bits(xss, xs_init, bits);
    for ps in &mut xss[j..] {
        slice_set_zero(ps);
    }
    for (x0, ps) in xss0.iter_mut().zip(xss.iter()) {
        *x0 = ps[0];
    }
    limbs_fft_negacyclic(xss, w, ts, us, ss);
    for ps in &mut *xss {
        limbs_fft_normmod_2expp1(ps);
    }
    let j = limbs_fft_split_bits(yss, &ys[..limbs], bits);
    for qs in &mut yss[j..] {
        slice_set_zero(qs);
    }
    for (y0, qs) in yss0.iter_mut().zip(yss.iter()) {
        *y0 = qs[0];
    }
    limbs_fft_negacyclic(yss, w, ts, us, ss);
    let b = n * w;
    let n_2 = (b + U_WIDTH - 1) >> Limb::LOG_WIDTH;
    let k = (n_2 << Limb::LOG_WIDTH) - b;
    for (ps, qs) in xss.iter_mut().zip(yss.iter_mut()) {
        limbs_fft_normmod_2expp1(qs);
        assert_eq!(*ps.last().unwrap(), 0);
        assert_eq!(*qs.last().unwrap(), 0);
        *ps.last_mut().unwrap() = Limb::from(limbs_fft_mulmod_2expp1_basecase_same(
            &mut ps[..n_2],
            &qs[..n_2],
            0,
            k,
            scratch,
        ));
    }
    limbs_ifft_negacyclic(xss, w, ts, us, ss);
    for (o, y) in out.iter_mut().zip(yss0.iter()) {
        *o = xss0[0].wrapping_mul(*y);
    }
    let m = xss0.len();
    for (i, x) in xss0.iter().enumerate().skip(1) {
        let (yss0_lo, yss0_hi) = yss0.split_at(m - i);
        let (out_lo, out_hi) = out.split_at_mut(i);
        for (o, y) in out_hi.iter_mut().zip(yss0_lo.iter()) {
            o.wrapping_add_assign(x.wrapping_mul(*y));
        }
        for (o, y) in out_lo.iter_mut().zip(yss0_hi.iter()) {
            o.wrapping_sub_assign(x.wrapping_mul(*y));
        }
    }
    let depth = depth + 1;
    for (r, ps) in out.iter_mut().zip(xss.iter_mut()) {
        limbs_fft_div_2expmod_2expp1_in_place(ps, depth);
        limbs_fft_normmod_2expp1(ps);
        let (ps_last, ps_init) = ps.split_last_mut().unwrap();
        let t = *ps_last;
        *ps_last = r.wrapping_sub(ps_init[0]);
        let ps_last = *ps_last;
        let carry = limbs_slice_add_limb_in_place(ps, ps_last);
        let ps_last = ps.last_mut().unwrap();
        (*r, *ps_last) = Limb::xx_add_yy_to_zz(0, *ps_last, 0, t);
        if carry {
            *r += 1;
        }
    }
    slice_set_zero(xs);
    limbs_fft_combine_bits(xs, &mut xss[..two_n - 1], bits, size, scratch);
    // as the negacyclic convolution has effectively done subtractions some of the coefficients will
    // be negative, so need to subtract p
    let limb_add = bits >> Limb::LOG_WIDTH;
    let mut xs_hi = &mut xs[1..];
    for j in 0..two_n - 2 {
        if out[j] != 0 {
            fail_on_untested_path("limbs_fft_mulmod_2expp1_helper, out[j] != 0");
            limbs_sub_limb_in_place(xs_hi, 1);
        } else if xss[j].last().unwrap().get_highest_bit() {
            // coefficient was -ve
            limbs_sub_limb_in_place(xs_hi, 1);
            limbs_sub_limb_in_place(&mut xs_hi[size - 1..], 1);
        }
        xs_hi = &mut xs_hi[limb_add..];
    }
    // penultimate coefficient, top bit was already ignored
    let j = two_n - 2;
    if out[j] != 0 || xss[j].last().unwrap().get_highest_bit() {
        // coefficient was -ve
        limbs_sub_limb_in_place(xs_hi, 1);
    }
    // final coefficient wraps around
    let (xs_last, xs_init) = xs.split_last_mut().unwrap();
    let (xss_lo, xss_hi) = xss[two_n - 1].split_at(limb_add);
    if limb_add != 0 {
        if limbs_slice_add_same_length_in_place_left(&mut xs_init[limbs - limb_add..], xss_lo) {
            xs_last.wrapping_add_assign(1);
        }
    } else {
        fail_on_untested_path("limbs_fft_mulmod_2expp1_helper, limb_add == 0");
    }
    let (xs_lo, xs_hi) = xs.split_at_mut(size - limb_add);
    if limbs_sub_same_length_in_place_left(xs_lo, xss_hi) {
        limbs_fft_addmod_2expp1_1(xs_hi, Limb::MAX);
    }
    limbs_fft_normmod_2expp1(xs);
}

fn limbs_fft_mulmod_2expp1_same_scratch_len(n: usize, w: usize) -> usize {
    let bits = n * w;
    let depth = bits.ceiling_log_base_2();
    let off = if depth < 12 {
        MULMOD_TAB[0]
    } else {
        MULMOD_TAB[min(usize::exact_from(depth), MULMOD_TAB.len() + 11) - 12]
    };
    let depth = (depth >> 1) - u64::from(off);
    let w = bits >> (depth << 1);
    let n = usize::power_of_2(depth);
    let size = ((n * w) >> Limb::LOG_WIDTH) + 1;
    let nw = n * w;
    max(
        size + 1,
        limbs_fft_mulmod_2expp1_basecase_same2_scratch_len(nw),
    ) + (n << 1)
}

// This is equivalent to `fft_mulmod_2expp1` from `fft/mulmod_2expp1.c`, FLINT 2.8.0, where r == i1
// == i2 and (n * w) >> Limb::LOG_WIDTH > cutoff.
#[allow(clippy::mut_mut)]
fn limbs_fft_mulmod_2expp1_same<'a>(
    xs: &mut [Limb],
    n: usize,
    w: usize,
    xss: &mut [&'a mut [Limb]],
    xss0: &mut [Limb],
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    ss: &mut &'a mut [Limb],
    scratch: &mut [Limb],
) {
    let bits = n * w;
    let limbs = bits >> Limb::LOG_WIDTH;
    assert_eq!(xs[limbs], 0);
    let depth = bits.ceiling_log_base_2();
    let off = if depth < 12 {
        MULMOD_TAB[0]
    } else {
        MULMOD_TAB[min(usize::exact_from(depth), MULMOD_TAB.len() + 11) - 12]
    };
    let depth = (depth >> 1) - u64::from(off);
    let w = bits >> (depth << 1);
    let n = usize::power_of_2(depth);
    let bits = (limbs << Limb::LOG_WIDTH) / (n << 1);
    let size = ((n * w) >> Limb::LOG_WIDTH) + 1;
    let two_n = n << 1;
    let (out, scratch) = scratch.split_at_mut(two_n);
    let xs = &mut xs[..=limbs];
    let (_, xs_init) = xs.split_last_mut().unwrap();
    let j = limbs_fft_split_bits(xss, xs_init, bits);
    for ps in &mut xss[j..] {
        slice_set_zero(ps);
    }
    for (x0, ps) in xss0.iter_mut().zip(xss.iter()) {
        *x0 = ps[0];
    }
    limbs_fft_negacyclic(xss, w, ts, us, ss);
    for ps in &mut *xss {
        limbs_fft_normmod_2expp1(ps);
    }
    let nw = n * w;
    for ps in &mut *xss {
        assert_eq!(*ps.last_mut().unwrap(), 0);
        *ps.last_mut().unwrap() =
            Limb::from(limbs_fft_mulmod_2expp1_basecase_same2(ps, 0, nw, scratch));
    }
    limbs_ifft_negacyclic(xss, w, ts, us, ss);
    let x = xss0[0];
    for (o, y) in out.iter_mut().zip(xss0.iter()) {
        *o = x.wrapping_mul(*y);
    }
    let m = xss0.len();
    for i in 1..m {
        let x = xss0[i];
        let (xss0_lo, xss0_hi) = xss0.split_at(m - i);
        let (out_lo, out_hi) = out.split_at_mut(i);
        for (o, y) in out_hi.iter_mut().zip(xss0_lo.iter()) {
            o.wrapping_add_assign(x.wrapping_mul(*y));
        }
        for (o, y) in out_lo.iter_mut().zip(xss0_hi.iter()) {
            o.wrapping_sub_assign(x.wrapping_mul(*y));
        }
    }
    let depth = depth + 1;
    for (r, ps) in out.iter_mut().zip(xss.iter_mut()) {
        limbs_fft_div_2expmod_2expp1_in_place(ps, depth);
        limbs_fft_normmod_2expp1(ps);
        let (ps_last, ps_init) = ps.split_last_mut().unwrap();
        let t = *ps_last;
        *ps_last = r.wrapping_sub(ps_init[0]);
        let ps_last = *ps_last;
        let carry = limbs_slice_add_limb_in_place(ps, ps_last);
        let ps_last = ps.last_mut().unwrap();
        (*r, *ps_last) = Limb::xx_add_yy_to_zz(0, *ps_last, 0, t);
        if carry {
            *r += 1;
        }
    }
    slice_set_zero(xs);
    limbs_fft_combine_bits(xs, &mut xss[..two_n - 1], bits, size, scratch);
    // as the negacyclic convolution has effectively done subtractions some of the coefficients will
    // be negative, so need to subtract p
    let limb_add = bits >> Limb::LOG_WIDTH;
    let mut xs_hi = &mut xs[1..];
    for j in 0..two_n - 2 {
        if out[j] != 0 {
            fail_on_untested_path("limbs_fft_mulmod_2expp1_helper, out[j] != 0");
            limbs_sub_limb_in_place(xs_hi, 1);
        } else if xss[j].last().unwrap().get_highest_bit() {
            // coefficient was -ve
            limbs_sub_limb_in_place(xs_hi, 1);
            limbs_sub_limb_in_place(&mut xs_hi[size - 1..], 1);
        }
        xs_hi = &mut xs_hi[limb_add..];
    }
    // penultimate coefficient, top bit was already ignored
    let j = two_n - 2;
    if out[j] != 0 || xss[j].last().unwrap().get_highest_bit() {
        // coefficient was -ve
        limbs_sub_limb_in_place(xs_hi, 1);
    }
    // final coefficient wraps around
    let (xs_last, xs_init) = xs.split_last_mut().unwrap();
    let (xss_lo, xss_hi) = xss[two_n - 1].split_at(limb_add);
    if limb_add != 0 {
        if limbs_slice_add_same_length_in_place_left(&mut xs_init[limbs - limb_add..], xss_lo) {
            xs_last.wrapping_add_assign(1);
        }
    } else {
        fail_on_untested_path("limbs_fft_mulmod_2expp1_helper, limb_add == 0");
    }
    let (xs_lo, xs_hi) = xs.split_at_mut(size - limb_add);
    if limbs_sub_same_length_in_place_left(xs_lo, xss_hi) {
        limbs_fft_addmod_2expp1_1(xs_hi, Limb::MAX);
    }
    limbs_fft_normmod_2expp1(xs);
}

// This is equivalent to `ifft_radix2_twiddle` from `fft/ifft_mfa_truncate_sqrt.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_radix2_twiddle<'a>(
    xss: &mut [&'a mut [Limb]],
    k: usize,
    n: usize,
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    v: usize,
    r: usize,
    c: usize,
    s: usize,
) {
    let (xss_lo, xss_hi) = xss.split_at_mut(n * k);
    if n == 1 {
        let tw1 = r * c;
        let tw2 = tw1 + s * c;
        let xs_lo = &mut xss_lo[0];
        let xs_hi = &mut xss_hi[0];
        let mut b1 = tw1 * v;
        let mut b2 = tw2 * v;
        let nw = (ts.len() - 1) << Limb::LOG_WIDTH;
        let negate1 = b1 >= nw;
        let negate2 = b2 >= nw;
        if negate1 {
            fail_on_untested_path("limbs_ifft_butterfly_twiddle, b1 >= nw");
            b1 -= nw;
        }
        let x = b1 >> Limb::LOG_WIDTH;
        let b1 = u64::wrapping_from(b1) & Limb::WIDTH_MASK;
        if negate2 {
            b2 -= nw;
        }
        let y = b2 >> Limb::LOG_WIDTH;
        let b2 = u64::wrapping_from(b2) & Limb::WIDTH_MASK;
        if negate1 {
            fail_on_untested_path("limbs_ifft_butterfly_twiddle, negate1");
            limbs_neg_in_place(xs_lo);
        }
        limbs_fft_div_2expmod_2expp1_in_place(xs_lo, b1);
        if negate2 {
            limbs_neg_in_place(xs_hi);
        }
        limbs_fft_div_2expmod_2expp1_in_place(xs_hi, b2);
        limbs_butterfly_rsh_b(ts, us, xs_lo, xs_hi, x, y);
        swap(&mut xss_lo[0], ts);
        swap(&mut xss_hi[0], us);
        return;
    }
    let half_n = n >> 1;
    let two_w = w << 1;
    let two_s = s << 1;
    limbs_ifft_radix2_twiddle(xss_lo, k, half_n, two_w, ts, us, v, r, c, two_s);
    limbs_ifft_radix2_twiddle(xss_hi, k, half_n, two_w, ts, us, v, r + s, c, two_s);
    let mut j = 0;
    for i in 0..n {
        let xs_lo = &mut xss_lo[j];
        let xs_hi = &mut xss_hi[j];
        limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
        swap(xs_lo, ts);
        swap(xs_hi, us);
        j += k;
    }
}

// This is equivalent to `ifft_truncate1_twiddle` from `fft/ifft_mfa_truncate_sqrt.c`, FLINT 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_truncate1_twiddle<'a>(
    xss: &mut [&'a mut [Limb]],
    k: usize,
    n: usize,
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    ws: usize,
    r: usize,
    c: usize,
    s: usize,
    trunc: usize,
) {
    if trunc == n << 1 {
        limbs_ifft_radix2_twiddle(xss, k, n, w, ts, us, ws, r, c, s);
    } else {
        let (xss_lo, xss_hi) = xss.split_at_mut(n * k);
        let half_n = n >> 1;
        let two_w = w << 1;
        let two_s = s << 1;
        if trunc <= n {
            let mut j = trunc * k;
            for _ in trunc..n {
                let xs_lo = &mut xss_lo[j];
                limbs_slice_add_same_length_in_place_left(xs_lo, xss_hi[j]);
                limbs_fft_div_2expmod_2expp1_in_place(xs_lo, 1);
                j += k;
            }
            limbs_ifft_truncate1_twiddle(xss_lo, k, half_n, two_w, ts, us, ws, r, c, two_s, trunc);
            j = 0;
            for _ in 0..trunc {
                let xs_lo = &mut xss_lo[j];
                limbs_slice_shl_in_place(xs_lo, 1);
                limbs_sub_same_length_in_place_left(xs_lo, xss_hi[j]);
                j += k;
            }
        } else {
            limbs_ifft_radix2_twiddle(xss_lo, k, n >> 1, w << 1, ts, us, ws, r, c, s << 1);
            let trunc = trunc - n;
            let mut j = trunc * k;
            for i in trunc..n {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                limbs_sub_same_length_in_place_right(xs_lo, xs_hi);
                limbs_fft_adjust(ts, xs_hi, i, w);
                limbs_slice_add_same_length_in_place_left(xs_lo, xs_hi);
                swap(xs_hi, ts);
                j += k;
            }
            let r = r + s;
            limbs_ifft_truncate1_twiddle(xss_hi, k, half_n, two_w, ts, us, ws, r, c, two_s, trunc);
            j = 0;
            for i in 0..trunc {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i, w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
                j += k;
            }
        }
    }
}

// This is equivalent to `ifft_mfa_truncate_sqrt_outer` from `fft/ifft_mfa_truncate_sqrt.c`, FLINT
// 2.8.0.
#[allow(clippy::mut_mut)]
fn limbs_ifft_mfa_truncate_sqrt_outer<'a>(
    xss: &mut [&'a mut [Limb]],
    n: usize,
    w: usize,
    ts: &mut &'a mut [Limb],
    us: &mut &'a mut [Limb],
    temp: &mut &'a mut [Limb],
    xs_len: usize,
    trunc: usize,
) {
    let two_n = n << 1;
    let ys_len = two_n / xs_len;
    let trunc2 = (trunc - two_n) / xs_len;
    let depth = ys_len.ceiling_log_base_2();
    let depth2 = xs_len.ceiling_log_base_2();
    let half_ys_len = ys_len >> 1;
    let wx = w * xs_len;
    // - first half mfa IFFT : ys_len rows, xs_len cols
    // - column IFFTs
    let mut xss_hi = &mut *xss;
    for i in 0..xs_len {
        let mut k = 0;
        for j in 0..ys_len {
            let s = n_revbin(j, depth);
            if j < s {
                xss_hi.swap(k, s * xs_len);
            }
            k += xs_len;
        }
        //  IFFT of length ys_len on column i, applying z^{r*i} for rows going up in steps of 1
        //  starting at row 0, where z => w bits
        limbs_ifft_radix2_twiddle(xss_hi, xs_len, half_ys_len, wx, ts, us, w, 0, i, 1);
        xss_hi = &mut xss_hi[1..];
    }
    // second half IFFT : ys_len rows, xs_len cols
    let half_w = w >> 1;
    let mut xss = &mut *xss;
    for i in 0..xs_len {
        let (xss_lo, xss_hi) = xss.split_at_mut(two_n);
        let mut j = 0;
        for k in 0..trunc2 {
            let s = n_revbin(k, depth);
            if k < s {
                xss_hi.swap(j, s * xs_len);
            }
            j += xs_len;
        }
        for _ in trunc2..ys_len {
            let k = i + j;
            if w.odd() {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                if i.odd() {
                    limbs_fft_adjust_sqrt(xs_hi, xs_lo, k, w, temp);
                } else {
                    limbs_fft_adjust(xs_hi, xs_lo, k >> 1, w);
                }
            } else {
                limbs_fft_adjust(xss_hi[j], xss_lo[j], k, half_w);
            }
            j += xs_len;
        }
        // IFFT of length ys_len on column i, applying z^{r*i} for rows going up in steps of 1
        // starting at row 0, where z => w bits
        limbs_ifft_truncate1_twiddle(xss_hi, xs_len, half_ys_len, wx, ts, us, w, 0, i, 1, trunc2);
        // relevant components of final sqrt layer of IFFT
        let mut j = 0;
        let limit = trunc - two_n - i;
        if w.odd() {
            while j < limit {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                let k = i + j;
                if k.odd() {
                    limbs_ifft_butterfly_sqrt(ts, us, xs_lo, xs_hi, k, w, temp);
                } else {
                    limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, k >> 1, w);
                }
                swap(xs_lo, ts);
                swap(xs_hi, us);
                j += xs_len;
            }
        } else {
            while j < limit {
                let xs_lo = &mut xss_lo[j];
                let xs_hi = &mut xss_hi[j];
                limbs_ifft_butterfly(ts, us, xs_lo, xs_hi, i + j, half_w);
                swap(xs_lo, ts);
                swap(xs_hi, us);
                j += xs_len;
            }
        }
        let mut j = trunc - two_n;
        let limit = two_n - i;
        while j < limit {
            limbs_slice_shl_in_place(xss_lo[j], 1);
            j += xs_len;
        }
        let depth = depth + depth2 + 1;
        let mut j = 0;
        for _ in 0..ys_len {
            let xs_lo = &mut xss_lo[j];
            limbs_fft_div_2expmod_2expp1_in_place(xs_lo, depth);
            limbs_fft_normmod_2expp1(xs_lo);
            j += xs_len;
        }
        let mut j = 0;
        for _ in 0..trunc2 {
            let xs_hi = &mut xss_hi[j];
            limbs_fft_div_2expmod_2expp1_in_place(xs_hi, depth);
            limbs_fft_normmod_2expp1(xs_hi);
            j += xs_len;
        }
        xss = &mut xss[1..];
    }
}

pub_const_test! {
    #[cfg(feature = "32_bit_limbs")]
    limbs_mul_greater_to_out_fft_is_valid(xs_len: usize, ys_len: usize) -> bool {
    xs_len + ys_len > 112
}}

pub_const_test! {
    #[cfg(not(feature = "32_bit_limbs"))]
    limbs_mul_greater_to_out_fft_is_valid(xs_len: usize, ys_len: usize) -> bool {
    xs_len + ys_len > 56
}}

pub_const_test! {
    #[cfg(feature = "32_bit_limbs")]
    limbs_square_to_out_fft_is_valid(xs_len: usize) -> bool {
    xs_len > 56
}}

pub_const_test! {
    #[cfg(not(feature = "32_bit_limbs"))]
    limbs_square_to_out_fft_is_valid(xs_len: usize) -> bool {
    xs_len > 28
}}

const FFT_MULMOD_2EXPP1_CUTOFF: usize = 50;

pub_test! {limbs_mul_greater_to_out_fft_with_cutoff_scratch_len(
    xs_len: usize,
    ys_len: usize,
    cutoff: usize,
) -> usize {
    let mut depth = 6;
    let mut w = 1;
    let mut n = 64;
    let mut bits = 28;
    let bits1 = xs_len << Limb::LOG_WIDTH;
    let bits2 = ys_len << Limb::LOG_WIDTH;
    let mut j1 = (bits1 - 1) / bits + 1;
    let mut j2 = (bits2 - 1) / bits + 1;
    assert!(j1 + j2 - 1 > n << 1);
    // find initial n, w
    while j1 + j2 - 1 > n << 2 {
        if w == 1 {
            w = 2;
        } else {
            depth += 1;
            w = 1;
            n <<= 1;
        }
        bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
        j1 = (bits1 - 1) / bits + 1;
        j2 = (bits2 - 1) / bits + 1;
    }
    if depth < 11 {
        let mut wadj = 1;
        // adjust n and w
        let off = u64::from(FFT_TAB[usize::wrapping_from(depth - 6)][w - 1]);
        depth -= off;
        n = usize::power_of_2(depth);
        w *= usize::power_of_2(off << 1);
        if depth < 6 {
            wadj = usize::power_of_2(6 - depth);
        }
        if w > wadj {
            loop {
                // see if a smaller w will work
                w -= wadj;
                bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
                j1 = (bits1 - 1) / bits + 1;
                j2 = (bits2 - 1) / bits + 1;
                if j1 + j2 - 1 > (n << 2) || w <= wadj {
                    break;
                }
            }
            w += wadj;
        }
        let b = n * w;
        let len = b >> Limb::LOG_WIDTH;
        let size = len + 1;
        ((n * size) << 3)
            + 3 * size
            + max(
                size,
                limbs_fft_mulmod_2expp1_basecase_same_scratch_len(size),
            )
    } else {
        if j1 + j2 - 1 <= 3 * n {
            depth -= 1;
            w *= 3;
        }
        let n = usize::power_of_2(depth);
        let nw = n * w;
        let bits = (nw - (usize::exact_from(depth) + 1)) >> 1;
        let limbs = nw >> Limb::LOG_WIDTH;
        let size = limbs + 1;
        let bits2 = nw;
        let s = if bits >> Limb::LOG_WIDTH <= cutoff {
            let n_2 = (bits2 + U_WIDTH - 1) >> Limb::LOG_WIDTH;
            limbs_fft_mulmod_2expp1_basecase_same_scratch_len(n_2)
        } else {
            let depth2 = bits2.ceiling_log_base_2();
            let off = if depth2 < 12 {
                MULMOD_TAB[0]
            } else {
                MULMOD_TAB[min(usize::exact_from(depth2), MULMOD_TAB.len() + 11) - 12]
            };
            let depth2 = (depth2 >> 1) - u64::from(off);
            let w2 = bits2 >> (depth2 << 1);
            let n3 = usize::power_of_2(depth2);
            let size = ((n3 * w2) >> Limb::LOG_WIDTH) + 1;
            let two_n3 = n3 << 1;
            ((n3 + n3 * size) << 1)
                + two_n3 * size
                + two_n3
                + 3 * size
                + limbs_fft_mulmod_2expp1_scratch_len(n, w)
        };
        ((n * size) << 3) + 3 * size + max(s, size)
    }
}}

// This is equivalent to `flint_mpn_mul_fft_main` from `fft/mul_fft_main.c`, FLINT 2.8.0, where i1
// != i2.
pub_test! {limbs_mul_greater_to_out_fft_with_cutoff(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    cutoff: usize,
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    let mut depth = 6;
    let mut w = 1;
    let mut n = 64;
    let mut bits = 28;
    let bits1 = xs_len << Limb::LOG_WIDTH;
    let bits2 = ys_len << Limb::LOG_WIDTH;
    let mut j1 = (bits1 - 1) / bits + 1;
    let mut j2 = (bits2 - 1) / bits + 1;
    assert!(j1 + j2 - 1 > n << 1);
    // find initial n, w
    while j1 + j2 - 1 > n << 2 {
        if w == 1 {
            w = 2;
        } else {
            depth += 1;
            w = 1;
            n <<= 1;
        }
        bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
        j1 = (bits1 - 1) / bits + 1;
        j2 = (bits2 - 1) / bits + 1;
    }
    if depth < 11 {
        let mut wadj = 1;
        // adjust n and w
        let off = u64::from(FFT_TAB[usize::wrapping_from(depth - 6)][w - 1]);
        depth -= off;
        n = usize::power_of_2(depth);
        w *= usize::power_of_2(off << 1);
        if depth < 6 {
            wadj = usize::power_of_2(6 - depth);
        }
        if w > wadj {
            loop {
                // see if a smaller w will work
                w -= wadj;
                bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
                j1 = (bits1 - 1) / bits + 1;
                j2 = (bits2 - 1) / bits + 1;
                if j1 + j2 - 1 > (n << 2) || w <= wadj {
                    break;
                }
            }
            w += wadj;
        }
        let b = n * w;
        let bits = (b - (usize::exact_from(depth) + 1)) >> 1;
        let out = &mut out[..xs_len + ys_len];
        let len = b >> Limb::LOG_WIDTH;
        let size = len + 1;
        let mut j1 = ((xs_len << Limb::LOG_WIDTH) - 1) / bits + 1;
        let mut j2 = ((ys_len << Limb::LOG_WIDTH) - 1) / bits + 1;
        let four_n = n << 2;
        let (scratch, combine_scratch) = scratch.split_at_mut(((n * size) << 3) + 3 * size);
        let (mut yss_scratch, mut xss_scratch) = scratch.split_at_mut((n * size) << 2);
        let mut xss: Vec<&mut [Limb]> = Vec::with_capacity(four_n);
        for _ in 0..four_n {
            let (lo, hi) = xss_scratch.split_at_mut(size);
            xss.push(lo);
            xss_scratch = hi;
        }
        let (mut ts, scratch_hi) = xss_scratch.split_at_mut(size);
        let (mut us, mut ss) = scratch_hi.split_at_mut(size);
        let mut yss: Vec<&mut [Limb]> = Vec::with_capacity(four_n);
        for _ in 0..four_n {
            let (lo, hi) = yss_scratch.split_at_mut(size);
            yss.push(lo);
            yss_scratch = hi;
        }
        let mut trunc = j1 + j2 - 1;
        let two_n = n << 1;
        if trunc <= two_n {
            // trunc must be greater than 2n
            trunc = two_n + 1;
        }
        // trunc must be divisible by 2
        trunc = (trunc + 1) >> 1 << 1;
        j1 = limbs_fft_split_bits(&mut xss, xs, bits);
        for xs in &mut xss[j1..] {
            slice_set_zero(xs);
        }
        limbs_fft_truncate_sqrt(&mut xss, w, &mut ts, &mut us, ss, trunc);
        j2 = limbs_fft_split_bits(&mut yss, ys, bits);
        for xs in &mut yss[j2..] {
            slice_set_zero(xs);
        }
        limbs_fft_truncate_sqrt(&mut yss, w, &mut ts, &mut us, ss, trunc);
        let n_2 = (b + U_WIDTH - 1) >> Limb::LOG_WIDTH;
        assert_eq!(n_2, len);
        let k = (n_2 << Limb::LOG_WIDTH) - b;
        for (xs, ys) in xss.iter_mut().zip(yss.iter_mut()).take(trunc) {
            limbs_fft_normmod_2expp1(xs);
            limbs_fft_normmod_2expp1(ys);
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            let (ys_last, ys_init) = ys.split_last().unwrap();
            let c = (*xs_last << 1) + *ys_last;
            *xs_last = Limb::from(limbs_fft_mulmod_2expp1_basecase_same(
                xs_init,
                ys_init,
                c,
                k,
                combine_scratch,
            ));
        }
        limbs_ifft_truncate_sqrt(&mut xss, w, &mut ts, &mut us, &mut ss, trunc);
        let depth = depth + 2;
        for xs in &mut xss[..trunc] {
            limbs_fft_div_2expmod_2expp1_in_place(xs, depth);
            limbs_fft_normmod_2expp1(xs);
        }
        slice_set_zero(out);
        limbs_fft_combine_bits(out, &mut xss[..j1 + j2 - 1], bits, len, combine_scratch);
    } else {
        if j1 + j2 - 1 <= 3 * n {
            depth -= 1;
            w *= 3;
        }
        let n = usize::power_of_2(depth);
        let nw = n * w;
        let bits = (nw - (usize::exact_from(depth) + 1)) >> 1;
        let sqrt = usize::power_of_2(depth >> 1);
        let limbs = nw >> Limb::LOG_WIDTH;
        let size = limbs + 1;
        let mut j1 = ((xs_len << Limb::LOG_WIDTH) - 1) / bits + 1;
        let mut j2 = ((ys_len << Limb::LOG_WIDTH) - 1) / bits + 1;
        let (scratch, misc_scratch) = scratch.split_at_mut(((n * size) << 3) + 3 * size);
        let (mut yss_scratch, mut xss_scratch) = scratch.split_at_mut((n * size) << 2);
        let four_n = n << 2;
        let mut xss: Vec<&mut [Limb]> = Vec::with_capacity(four_n);
        for _ in 0..four_n {
            let (lo, hi) = xss_scratch.split_at_mut(size);
            xss.push(lo);
            xss_scratch = hi;
        }
        let (mut ss, scratch_hi) = xss_scratch.split_at_mut(size);
        let (mut ts, mut us) = scratch_hi.split_at_mut(size);
        let mut yss: Vec<&mut [Limb]> = Vec::with_capacity(four_n);
        for _ in 0..four_n {
            let (lo, hi) = yss_scratch.split_at_mut(size);
            yss.push(lo);
            yss_scratch = hi;
        }
        let mut trunc = j1 + j2 - 1;
        assert!(trunc > n << 1);
        // trunc must be divisible by 2*sqrt
        let two_sqrt = sqrt << 1;
        trunc = two_sqrt * ((trunc + two_sqrt - 1) / two_sqrt);
        j1 = limbs_fft_split_bits(&mut xss, xs, bits);
        for ps in &mut xss[j1..] {
            slice_set_zero(ps);
        }
        limbs_fft_mfa_truncate_sqrt_outer(&mut xss, w, &mut ts, &mut us, &mut ss, sqrt, trunc);
        j2 = limbs_fft_split_bits(&mut yss, ys, bits);
        for qs in &mut yss[j2..] {
            slice_set_zero(qs);
        }
        limbs_fft_mfa_truncate_sqrt_outer(&mut yss, w, &mut ts, &mut us, &mut ss, sqrt, trunc);
        let two_n = four_n >> 1;
        let len = two_n / sqrt;
        let depth = len.ceiling_log_base_2();
        // convolutions on relevant rows
        let xss_hi = &mut xss[two_n..];
        let yss_hi = &mut yss[two_n..];
        let wy = w * len;
        if bits >> Limb::LOG_WIDTH <= cutoff {
            let n_2 = (nw + U_WIDTH - 1) >> Limb::LOG_WIDTH;
            for s in 0..(trunc - two_n) / sqrt {
                let start = sqrt * n_revbin(s, depth);
                let xss_hi = &mut xss_hi[start..][..sqrt];
                let yss_hi = &mut yss_hi[start..][..sqrt];
                limbs_fft_radix2(xss_hi, wy, &mut ts, &mut us);
                limbs_fft_radix2(yss_hi, wy, &mut ts, &mut us);
                for (xs, ys) in xss_hi.iter_mut().zip(yss_hi.iter_mut()) {
                    limbs_fft_normmod_2expp1(xs);
                    limbs_fft_normmod_2expp1(ys);
                    let k = (n_2 << Limb::LOG_WIDTH) - nw;
                    xs[limbs] = Limb::from(limbs_fft_mulmod_2expp1_basecase_same(
                        &mut xs[..n_2],
                        &ys[..n_2],
                        0,
                        k,
                        misc_scratch,
                    ));
                }
                limbs_ifft_radix2(xss_hi, wy, &mut ts, &mut us);
            }
            // convolutions on rows
            for (xss_chunk, yss_chunk) in xss.chunks_mut(sqrt).zip(yss.chunks_mut(sqrt)).take(len) {
                limbs_fft_radix2(xss_chunk, wy, &mut ts, &mut us);
                limbs_fft_radix2(yss_chunk, wy, &mut ts, &mut us);
                for (xs, ys) in xss_chunk.iter_mut().zip(yss_chunk.iter_mut()) {
                    limbs_fft_normmod_2expp1(xs);
                    limbs_fft_normmod_2expp1(ys);
                    let k = (n_2 << Limb::LOG_WIDTH) - nw;
                    xs[limbs] = Limb::from(limbs_fft_mulmod_2expp1_basecase_same(
                        &mut xs[..n_2],
                        &ys[..n_2],
                        0,
                        k,
                        misc_scratch,
                    ));
                }
                limbs_ifft_radix2(xss_chunk, wy, &mut ts, &mut us);
            }
        } else {
            let depth2 = nw.ceiling_log_base_2();
            let off = if depth2 < 12 {
                MULMOD_TAB[0]
            } else {
                MULMOD_TAB[min(usize::exact_from(depth2), MULMOD_TAB.len() + 11) - 12]
            };
            let depth2 = (depth2 >> 1) - u64::from(off);
            let w2 = nw >> (depth2 << 1);
            let n3 = usize::power_of_2(depth2);
            let size = ((n3 * w2) >> Limb::LOG_WIDTH) + 1;
            let two_n3 = n3 << 1;
            let yss_scratch_len = (n3 + n3 * size) << 1;
            let (scratch, combine_scratch) =
                misc_scratch.split_at_mut((yss_scratch_len << 1) + 3 * size);
            let (mut yss_scratch, mut xss_scratch) = scratch.split_at_mut(yss_scratch_len);
            let mut xss2: Vec<&mut [Limb]> = Vec::with_capacity(two_n3);
            for _ in 0..two_n3 {
                let (lo, hi) = xss_scratch.split_at_mut(size);
                xss2.push(lo);
                xss_scratch = hi;
            }
            let (xss0, scratch_hi) = xss_scratch.split_at_mut(two_n3);
            let (mut ts2, scratch_hi) = scratch_hi.split_at_mut(size);
            let (mut us2, mut ss2) = scratch_hi.split_at_mut(size);
            let mut yss2: Vec<&mut [Limb]> = Vec::with_capacity(two_n3);
            for _ in 0..two_n3 {
                let (lo, hi) = yss_scratch.split_at_mut(size);
                yss2.push(lo);
                yss_scratch = hi;
            }
            let yss0 = yss_scratch;
            for s in 0..(trunc - two_n) / sqrt {
                let start = sqrt * n_revbin(s, depth);
                let xss_hi = &mut xss_hi[start..][..sqrt];
                let yss_hi = &mut yss_hi[start..][..sqrt];
                limbs_fft_radix2(xss_hi, wy, &mut ts, &mut us);
                limbs_fft_radix2(yss_hi, wy, &mut ts, &mut us);
                for (xs, ys) in xss_hi.iter_mut().zip(yss_hi.iter_mut()) {
                    limbs_fft_normmod_2expp1(xs);
                    limbs_fft_normmod_2expp1(ys);
                    limbs_fft_mulmod_2expp1(
                        xs,
                        ys,
                        n,
                        w,
                        &mut xss2,
                        xss0,
                        &mut yss2,
                        yss0,
                        &mut ts2,
                        &mut us2,
                        &mut ss2,
                        combine_scratch,
                    );
                }
                limbs_ifft_radix2(xss_hi, wy, &mut ts, &mut us);
            }
            // convolutions on rows
            for (xss_chunk, yss_chunk) in xss.chunks_mut(sqrt).zip(yss.chunks_mut(sqrt)).take(len) {
                limbs_fft_radix2(xss_chunk, wy, &mut ts, &mut us);
                limbs_fft_radix2(yss_chunk, wy, &mut ts, &mut us);
                for (xs, ys) in xss_chunk.iter_mut().zip(yss_chunk.iter_mut()) {
                    limbs_fft_normmod_2expp1(xs);
                    limbs_fft_normmod_2expp1(ys);
                    limbs_fft_mulmod_2expp1(
                        xs,
                        ys,
                        n,
                        w,
                        &mut xss2,
                        xss0,
                        &mut yss2,
                        yss0,
                        &mut ts2,
                        &mut us2,
                        &mut ss2,
                        combine_scratch,
                    );
                }
                limbs_ifft_radix2(xss_chunk, wy, &mut ts, &mut us);
            }
        }
        limbs_ifft_mfa_truncate_sqrt_outer(&mut xss, n, w, &mut ts, &mut us, &mut ss, sqrt, trunc);
        let out = &mut out[..xs_len + ys_len];
        slice_set_zero(out);
        limbs_fft_combine_bits(out, &mut xss[..j1 + j2 - 1], bits, limbs, misc_scratch);
    }
}}

// This is equivalent to `flint_mpn_mul_fft_main` from `fft/mul_fft_main.c`, FLINT 2.8.0, where i1
// != i2.
pub_crate_test! {
    #[inline]
    limbs_mul_greater_to_out_fft(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    limbs_mul_greater_to_out_fft_with_cutoff(out, xs, ys, FFT_MULMOD_2EXPP1_CUTOFF, scratch);
}}

pub_crate_test! {
    #[inline]
    limbs_mul_greater_to_out_fft_scratch_len(xs_len: usize, ys_len: usize) -> usize {
    limbs_mul_greater_to_out_fft_with_cutoff_scratch_len(xs_len, ys_len, FFT_MULMOD_2EXPP1_CUTOFF)
}}

pub_test! {limbs_square_to_out_fft_with_cutoff_scratch_len(xs_len: usize, cutoff: usize) -> usize {
    let mut depth = 6;
    let mut w = 1;
    let mut n = 64;
    let mut bits = 28;
    let bits1 = xs_len << Limb::LOG_WIDTH;
    let mut j1 = (bits1 - 1) / bits + 1;
    assert!((j1 << 1) - 1 > n << 1);
    // find initial n, w
    while (j1 << 1) - 1 > n << 2 {
        if w == 1 {
            w = 2;
        } else {
            depth += 1;
            w = 1;
            n <<= 1;
        }
        bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
        j1 = (bits1 - 1) / bits + 1;
    }
    if depth < 11 {
        let mut wadj = 1;
        // adjust n and w
        let off = u64::from(FFT_TAB[usize::wrapping_from(depth - 6)][w - 1]);
        depth -= off;
        n = usize::power_of_2(depth);
        w *= usize::power_of_2(off << 1);
        if depth < 6 {
            wadj = usize::power_of_2(6 - depth);
        }
        if w > wadj {
            loop {
                // see if a smaller w will work
                w -= wadj;
                bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
                j1 = (bits1 - 1) / bits + 1;
                if (j1 << 1) - 1 > n << 2 || w <= wadj {
                    break;
                }
            }
            w += wadj;
        }
        let b = n * w;
        let len = b >> Limb::LOG_WIDTH;
        let size = len + 1;
        ((n * size) << 2)
            + 3 * size
            + max(size, limbs_fft_mulmod_2expp1_basecase_same2_scratch_len(b))
    } else {
        if (j1 << 1) - 1 <= 3 * n {
            depth -= 1;
            w *= 3;
        }
        let n = usize::power_of_2(depth);
        let nw = n * w;
        let limbs = nw >> Limb::LOG_WIDTH;
        let size = limbs + 1;
        let bits2 = n * w;
        let s = if bits2 >> Limb::LOG_WIDTH <= cutoff {
            limbs_fft_mulmod_2expp1_basecase_same2_scratch_len(bits2)
        } else {
            let depth2 = bits2.ceiling_log_base_2();
            let off = if depth2 < 12 {
                MULMOD_TAB[0]
            } else {
                MULMOD_TAB[min(usize::exact_from(depth2), MULMOD_TAB.len() + 11) - 12]
            };
            let depth2 = (depth2 >> 1) - u64::from(off);
            let w3 = bits2 >> (depth2 << 1);
            let n3 = usize::power_of_2(depth2);
            let size = ((n3 * w3) >> Limb::LOG_WIDTH) + 1;
            ((n * size) << 2) + 3 * size + limbs_fft_mulmod_2expp1_same_scratch_len(n, w)
        };
        ((n * size) << 2) + 3 * size + max(s, size)
    }
}}

pub_test! {limbs_square_to_out_fft_with_cutoff(
    out: &mut [Limb],
    xs: &[Limb],
    cutoff: usize,
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let mut depth = 6;
    let mut w = 1;
    let mut n = 64;
    let mut bits = 28;
    let bits1 = xs_len << Limb::LOG_WIDTH;
    let mut j1 = (bits1 - 1) / bits + 1;
    assert!((j1 << 1) - 1 > n << 1);
    // find initial n, w
    while (j1 << 1) - 1 > n << 2 {
        if w == 1 {
            w = 2;
        } else {
            depth += 1;
            w = 1;
            n <<= 1;
        }
        bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
        j1 = (bits1 - 1) / bits + 1;
    }
    if depth < 11 {
        let mut wadj = 1;
        // adjust n and w
        let off = u64::from(FFT_TAB[usize::wrapping_from(depth - 6)][w - 1]);
        depth -= off;
        n = usize::power_of_2(depth);
        w *= usize::power_of_2(off << 1);
        if depth < 6 {
            wadj = usize::power_of_2(6 - depth);
        }
        if w > wadj {
            loop {
                // see if a smaller w will work
                w -= wadj;
                bits = (n * w - (usize::wrapping_from(depth) + 1)) >> 1;
                j1 = (bits1 - 1) / bits + 1;
                if (j1 << 1) - 1 > n << 2 || w <= wadj {
                    break;
                }
            }
            w += wadj;
        }
        let b = n * w;
        let bits = (b - (usize::exact_from(depth) + 1)) >> 1;
        let out = &mut out[..xs_len << 1];
        let len = b >> Limb::LOG_WIDTH;
        let size = len + 1;
        let mut j1 = ((xs_len << Limb::LOG_WIDTH) - 1) / bits + 1;
        let (mut xss_scratch, combine_scratch) = scratch.split_at_mut(((n * size) << 2) + 3 * size);
        let four_n = n << 2;
        let mut xss: Vec<&mut [Limb]> = Vec::with_capacity(four_n);
        for _ in 0..four_n {
            let (lo, hi) = xss_scratch.split_at_mut(size);
            xss.push(lo);
            xss_scratch = hi;
        }
        let (mut ts, scratch_hi) = xss_scratch.split_at_mut(size);
        let (mut us, mut ss) = scratch_hi.split_at_mut(size);
        let mut trunc = (j1 << 1) - 1;
        let two_n = n << 1;
        if trunc <= two_n {
            // trunc must be greater than 2n
            trunc = two_n + 1;
        }
        trunc = (trunc + 1) >> 1 << 1; // trunc must be divisible by 2
        j1 = limbs_fft_split_bits(&mut xss, xs, bits);
        for xs in &mut xss[j1..] {
            slice_set_zero(xs);
        }
        limbs_fft_truncate_sqrt(&mut xss, w, &mut ts, &mut us, ss, trunc);
        for xs in &mut xss[..trunc] {
            limbs_fft_normmod_2expp1(xs);
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            *xs_last = Limb::from(limbs_fft_mulmod_2expp1_basecase_same2(
                xs_init,
                *xs_last * 3,
                b,
                combine_scratch,
            ));
        }
        limbs_ifft_truncate_sqrt(&mut xss, w, &mut ts, &mut us, &mut ss, trunc);
        let depth = depth + 2;
        for xs in &mut xss[0..trunc] {
            limbs_fft_div_2expmod_2expp1_in_place(xs, depth);
            limbs_fft_normmod_2expp1(xs);
        }
        slice_set_zero(out);
        limbs_fft_combine_bits(out, &mut xss[..(j1 << 1) - 1], bits, len, combine_scratch);
    } else {
        if (j1 << 1) - 1 <= 3 * n {
            depth -= 1;
            w *= 3;
        }
        let n = usize::power_of_2(depth);
        let nw = n * w;
        let bits = (nw - (usize::exact_from(depth) + 1)) >> 1;
        let sqrt = usize::power_of_2(depth >> 1);
        let limbs = nw >> Limb::LOG_WIDTH;
        let size = limbs + 1;
        let mut j1 = ((xs_len << Limb::LOG_WIDTH) - 1) / bits + 1;
        let (mut xss_scratch, misc_scratch) = scratch.split_at_mut(((n * size) << 2) + 3 * size);
        let four_n = n << 2;
        let mut xss: Vec<&mut [Limb]> = Vec::with_capacity(four_n);
        for _ in 0..four_n {
            let (lo, hi) = xss_scratch.split_at_mut(size);
            xss.push(lo);
            xss_scratch = hi;
        }
        let (mut ss, scratch_hi) = xss_scratch.split_at_mut(size);
        let (mut ts, mut us) = scratch_hi.split_at_mut(size);
        let mut trunc = (j1 << 1) - 1;
        assert!(trunc > n << 1);
        // trunc must be divisible by 2*sqrt
        let two_sqrt = sqrt << 1;
        trunc = two_sqrt * ((trunc + two_sqrt - 1) / two_sqrt);
        j1 = limbs_fft_split_bits(&mut xss, xs, bits);
        for ps in &mut xss[j1..] {
            slice_set_zero(ps);
        }
        limbs_fft_mfa_truncate_sqrt_outer(&mut xss, w, &mut ts, &mut us, &mut ss, sqrt, trunc);
        let two_n = four_n >> 1;
        let len = two_n / sqrt;
        let depth = len.ceiling_log_base_2();
        // convolutions on relevant rows
        let xss_hi = &mut xss[two_n..];
        let wy = w * len;
        if nw >> Limb::LOG_WIDTH <= cutoff {
            for s in 0..(trunc - two_n) / sqrt {
                let start = sqrt * n_revbin(s, depth);
                let xss_hi = &mut xss_hi[start..][..sqrt];
                limbs_fft_radix2(xss_hi, wy, &mut ts, &mut us);
                for xs in &mut *xss_hi {
                    limbs_fft_normmod_2expp1(xs);
                    xs[limbs] =
                        Limb::from(limbs_fft_mulmod_2expp1_basecase_same2(xs, 0, nw, misc_scratch));
                }
                limbs_ifft_radix2(xss_hi, wy, &mut ts, &mut us);
            }
            // convolutions on rows
            for xss_chunk in xss.chunks_mut(sqrt).take(len) {
                limbs_fft_radix2(xss_chunk, wy, &mut ts, &mut us);
                for xs in &mut *xss_chunk {
                    limbs_fft_normmod_2expp1(xs);
                    xs[limbs] =
                        Limb::from(limbs_fft_mulmod_2expp1_basecase_same2(xs, 0, nw, misc_scratch));
                }
                limbs_ifft_radix2(xss_chunk, wy, &mut ts, &mut us);
            }
        } else {
            let depth2 = nw.ceiling_log_base_2();
            let off = if depth2 < 12 {
                MULMOD_TAB[0]
            } else {
                MULMOD_TAB[min(usize::exact_from(depth2), MULMOD_TAB.len() + 11) - 12]
            };
            let depth2 = (depth2 >> 1) - u64::from(off);
            let w3 = nw >> (depth2 << 1);
            let n3 = usize::power_of_2(depth2);
            let size = ((n3 * w3) >> Limb::LOG_WIDTH) + 1;
            let two_n3 = n3 << 1;
            let (mut xss_scratch, combine_scratch) =
                misc_scratch.split_at_mut(((n * size) << 2) + 3 * size);
            let mut xss2: Vec<&mut [Limb]> = Vec::with_capacity(two_n3);
            for _ in 0..two_n3 {
                let (lo, hi) = xss_scratch.split_at_mut(size);
                xss2.push(lo);
                xss_scratch = hi;
            }
            let (xss0, scratch_hi) = xss_scratch.split_at_mut(two_n3);
            let (mut ts2, scratch_hi) = scratch_hi.split_at_mut(size);
            let (mut us2, mut ss2) = scratch_hi.split_at_mut(size);
            for s in 0..(trunc - two_n) / sqrt {
                let start = sqrt * n_revbin(s, depth);
                let xss_hi = &mut xss_hi[start..][..sqrt];
                limbs_fft_radix2(xss_hi, wy, &mut ts, &mut us);
                for xs in &mut *xss_hi {
                    limbs_fft_normmod_2expp1(xs);
                    limbs_fft_mulmod_2expp1_same(
                        xs,
                        n,
                        w,
                        &mut xss2,
                        xss0,
                        &mut ts2,
                        &mut us2,
                        &mut ss2,
                        combine_scratch,
                    );
                }
                limbs_ifft_radix2(xss_hi, wy, &mut ts, &mut us);
            }
            // convolutions on rows
            for xss_chunk in xss.chunks_mut(sqrt).take(len) {
                limbs_fft_radix2(xss_chunk, wy, &mut ts, &mut us);
                for xs in &mut *xss_chunk {
                    limbs_fft_normmod_2expp1(xs);
                    limbs_fft_mulmod_2expp1_same(
                        xs,
                        n,
                        w,
                        &mut xss2,
                        xss0,
                        &mut ts2,
                        &mut us2,
                        &mut ss2,
                        combine_scratch,
                    );
                }
                limbs_ifft_radix2(xss_chunk, wy, &mut ts, &mut us);
            }
        }
        limbs_ifft_mfa_truncate_sqrt_outer(&mut xss, n, w, &mut ts, &mut us, &mut ss, sqrt, trunc);
        let out = &mut out[..xs_len << 1];
        slice_set_zero(out);
        limbs_fft_combine_bits(out, &mut xss[..(j1 << 1) - 1], bits, limbs, misc_scratch);
    }
}}

pub_crate_test! {
#[inline]
limbs_square_to_out_fft_scratch_len(xs_len: usize) -> usize {
    limbs_square_to_out_fft_with_cutoff_scratch_len(xs_len, FFT_MULMOD_2EXPP1_CUTOFF)
}}

// This is equivalent to `flint_mpn_mul_fft_main` from `fft/mul_fft_main.c`, FLINT 2.8.0, where i1
// == i2.
pub_crate_test! {
#[inline]
limbs_square_to_out_fft(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    limbs_square_to_out_fft_with_cutoff(out, xs, FFT_MULMOD_2EXPP1_CUTOFF, scratch);
}}
