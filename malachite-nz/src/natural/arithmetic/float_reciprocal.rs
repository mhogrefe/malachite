// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::malachite_base::num::basic::integers::PrimitiveInt;
use crate::malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::div_mod::{div_mod_by_preinversion, limbs_invert_limb};
use crate::natural::arithmetic::float_div::{
    limbs_div_helper, limbs_float_div_high, limbs_float_div_high_scratch_len, Cleanup,
    MPFR_DIV_THRESHOLD,
};
use crate::natural::arithmetic::mul::{limbs_mul_to_out, limbs_mul_to_out_scratch_len};
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_same_length_in_place_left,
};
use crate::natural::comparison::cmp::limbs_cmp_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::Ordering::{self, *};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    NegModPowerOf2, OverflowingAddAssign, OverflowingNegAssign, Parity, PowerOf2, ShrRound,
    WrappingAddAssign, WrappingNegAssign, WrappingSubAssign, XMulYToZZ, XXSubYYToZZ,
};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

const WIDTH_M1: u64 = Limb::WIDTH - 1;
const HIGH_BIT: Limb = 1 << WIDTH_M1;
const TWICE_WIDTH: u64 = Limb::WIDTH * 2;

// This is mpfr_div from div.c, MPFR 4.3.0, specialized for reciprocation.
pub fn reciprocal_float_significand_in_place(
    x: &mut Natural,
    x_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    if out_prec == x_prec {
        if let Some((increment_exp, o)) =
            reciprocal_float_significand_in_place_same_prec(x, out_prec, rm)
        {
            return (u64::from(increment_exp), o);
        }
    }
    match &mut *x {
        Natural(Small(small_x)) => {
            let (qs, exp_offset, o) = reciprocal_float_significand_short(*small_x, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(qs);
            (exp_offset, o)
        }
        Natural(Large(xs)) => {
            let (out, exp_offset, o) = reciprocal_float_significand_general(xs, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(out);
            (exp_offset, o)
        }
    }
}

// This is mpfr_div from div.c, MPFR 4.3.0, specialized for reciprocation.
pub fn reciprocal_float_significand_ref(
    x: &Natural,
    x_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, u64, Ordering) {
    if out_prec == x_prec {
        if let Some((reciprocal, increment_exp, o)) =
            reciprocal_float_significand_same_prec_ref(x, out_prec, rm)
        {
            return (reciprocal, u64::from(increment_exp), o);
        }
    }
    match x {
        Natural(Small(small_x)) => {
            let (qs, exp_offset, o) = reciprocal_float_significand_short(*small_x, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
        Natural(Large(xs)) => {
            let mut xs = xs.clone();
            let (qs, exp_offset, o) = reciprocal_float_significand_general(&mut xs, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
    }
}

fn reciprocal_float_significand_in_place_same_prec(
    x: &mut Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(bool, Ordering)> {
    match x {
        Natural(Small(x)) => {
            let (reciprocal, increment_exp, o) = if prec == Limb::WIDTH {
                reciprocal_float_significand_same_prec_w(*x, rm)
            } else {
                reciprocal_float_significand_same_prec_lt_w(*x, prec, rm)
            };
            *x = reciprocal;
            Some((increment_exp, o))
        }
        Natural(Large(xs)) => match xs.as_mut_slice() {
            [x_0, x_1] if prec != TWICE_WIDTH => {
                let (reciprocal_0, reciprocal_1, increment_exp, o) =
                    reciprocal_float_significand_same_prec_gt_w_lt_2w(*x_0, *x_1, prec, rm);
                *x_0 = reciprocal_0;
                *x_1 = reciprocal_1;
                Some((increment_exp, o))
            }
            _ => None,
        },
    }
}

fn reciprocal_float_significand_same_prec_ref(
    x: &Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Natural, bool, Ordering)> {
    match x {
        Natural(Small(x)) => {
            let (reciprocal, increment_exp, o) = if prec == Limb::WIDTH {
                reciprocal_float_significand_same_prec_w(*x, rm)
            } else {
                reciprocal_float_significand_same_prec_lt_w(*x, prec, rm)
            };
            Some((Natural(Small(reciprocal)), increment_exp, o))
        }
        Natural(Large(xs)) => match xs.as_slice() {
            [x_0, x_1] if prec != TWICE_WIDTH => {
                let (reciprocal_0, reciprocal_1, increment_exp, o) =
                    reciprocal_float_significand_same_prec_gt_w_lt_2w(*x_0, *x_1, prec, rm);
                Some((
                    Natural(Large(vec![reciprocal_0, reciprocal_1])),
                    increment_exp,
                    o,
                ))
            }
            _ => None,
        },
    }
}

// x cannot be equal to `2 ^ (WIDTH - 1)`.
//
// This is mpfr_div_1 from mul.c, MPFR 4.3.0, specialized for reciprocation.
fn reciprocal_float_significand_same_prec_lt_w(
    x: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, bool, Ordering) {
    let shift = Limb::WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let half_shift_bit = shift_bit >> 1;
    let mask = shift_bit - 1;
    // First try with an approximate reciprocal.
    let q = HIGH_BIT | (limbs_invert_limb(x) >> 1);
    // round_bit does not exceed the true reciprocal floor(HIGH_BIT * 2 ^ WIDTH / x), with error at
    // most 2, which means the rational reciprocal q satisfies round_bit <= q < round_bit + 3. We
    // can round correctly except when the last shift - 1 bits of q0 are 000..000 or 111..111 or
    // 111..110.
    let (round_bit, sticky_bit) = if (q + 2) & (mask >> 1) > 2 {
        // result cannot be exact in this case
        (q & half_shift_bit, 1)
    } else {
        let (mut hi, mut lo) = Limb::x_mul_y_to_zz(q, x);
        assert!(hi < HIGH_BIT || (hi == HIGH_BIT && lo == 0));
        // subtract {hi, lo} from {HIGH_BIT, 0}
        (hi, lo) = Limb::xx_sub_yy_to_zz(HIGH_BIT, 0, hi, lo);
        assert!(hi == 0 && lo < x);
        (q & half_shift_bit, lo | (q & (mask >> 1)))
    };
    let reciprocal = (HIGH_BIT | q) & !mask;
    match rm {
        Exact => panic!("Inexact float reciprocation"),
        Nearest => {
            if round_bit == 0 || sticky_bit == 0 && reciprocal & shift_bit == 0 {
                (reciprocal, false, Less)
            } else {
                (reciprocal.wrapping_add(shift_bit), false, Greater)
            }
        }
        Floor | Down => (reciprocal, false, Less),
        Ceiling | Up => (reciprocal.wrapping_add(shift_bit), false, Greater),
    }
}

// x cannot be equal to `2 ^ (WIDTH - 1)`.
fn reciprocal_float_significand_same_prec_w(x: Limb, rm: RoundingMode) -> (Limb, bool, Ordering) {
    // First compute an approximate reciprocal.
    let q = HIGH_BIT | (limbs_invert_limb(x) >> 1);
    // round_bit does not exceed the true reciprocal floor(2 ^ WIDTH / x), with error at most 2,
    // which means the rational reciprocal q satisfies round_bit <= q < round_bit + 3, thus the true
    // reciprocal is round_bit, round_bit + 1 or round_bit + 2.
    let (mut hi, mut lo) = Limb::x_mul_y_to_zz(q, x);
    assert!(hi < HIGH_BIT || (hi == HIGH_BIT && lo == 0));
    // subtract {hi, lo} from {HIGH_BIT, 0}
    (hi, lo) = Limb::xx_sub_yy_to_zz(HIGH_BIT, 0, hi, lo);
    assert!(hi == 0 && lo < x);
    // now (HIGH_BIT - extra * x) * 2 ^ WIDTH = q * x + lo with 0 <= lo < x
    //
    // If !increment_exp, the reciprocal is q0, the round bit is 1 if l >= x0 / 2, and sticky_bit
    // are the remaining bits from l. If increment_exp, the reciprocal is HIGH_BIT + (q >> 1), the
    // round bit is the least significant bit of q, and sticky_bit is lo.
    //
    // If "2 * lo < lo", then there is a carry in 2 * lo, thus 2 * lo > x. Otherwise if there is no
    // carry, we check whether 2 * lo >= y0.
    let two_lo = lo << 1;
    let round_bit = (two_lo < lo) || (two_lo >= x);
    let mut reciprocal = q;
    let sticky_bit = if round_bit {
        two_lo.wrapping_sub(x)
    } else {
        lo
    };
    match rm {
        Exact => panic!("Inexact float reciprocation"),
        Nearest => {
            if !round_bit || sticky_bit == 0 && reciprocal.even() {
                (reciprocal, false, Less)
            } else {
                reciprocal.wrapping_add_assign(1);
                (reciprocal, false, Greater)
            }
        }
        Floor | Down => (reciprocal, false, Less),
        Ceiling | Up => {
            reciprocal.wrapping_add_assign(1);
            (reciprocal, false, Greater)
        }
    }
}

// Given (B << WIDTH) < x = x_1 * B + x_0 with x normalized (high bit of x_1 set), put in q = Q1
// * B + Q0 an approximation of floor(B ^ 2 / x), with: B = 2 ^ WIDTH and q <= floor(B ^ 2 /
// x) <= q + 21.
//
// This is mpfr_div2_approx from div.c, MPFR 4.3.0, where Q0 and Q1 are returned, specialized for
// reciprocation.
fn reciprocal_float_2_approx(x_1: Limb, x_0: Limb) -> (Limb, Limb) {
    // First compute an approximation of q_1, using a lower approximation of B ^ 2 / (x_1 + 1) - B
    let inv = if x_1 == Limb::MAX {
        0
    } else {
        limbs_invert_limb(x_1 + 1)
    };
    // Now inv <= B ^ 2 / (x_1 + 1) - B.
    let mut q_1 = HIGH_BIT | (inv >> 1);
    // Now q_1 <= x_1 * B / (x_1 + 1) < (x_1 * B + x_0) * B / (x_1 * B + x_0).
    //
    // Compute q_1 * (x_1 * B + x_0) into r_1 : r_0 : xx and subtract from u_1 : x_0 : 0.
    let (mut r_1, mut r_0) = Limb::x_mul_y_to_zz(q_1, x_1);
    let (xx, yy) = Limb::x_mul_y_to_zz(q_1, x_0);
    if r_0.overflowing_add_assign(xx) {
        r_1.wrapping_add_assign(1);
    }
    // We ignore yy below, but first increment r_0, to ensure we get a lower approximation of the
    // remainder.
    if yy != 0 {
        r_0.wrapping_add_assign(1);
    }
    if r_0 == 0 && yy != 0 {
        r_1.wrapping_add_assign(1);
    }
    r_1 = HIGH_BIT.wrapping_sub(r_1);
    let carry;
    (r_0, carry) = r_0.overflowing_neg();
    if carry {
        r_1.wrapping_sub_assign(1);
    }
    // r_1 : r_0 should be non-negative.
    assert!(!r_1.get_highest_bit());
    // The second reciprocal limb is approximated by (r_1 * B ^ 2 + r_0 * B) / x_1, and since (B +
    // inv) / B approximates B / x_1, this is in turn approximated by (r * B + r_0) * (B + inv) / B
    // = r_1 * B * r_1 * inv + r_0 + (r0 * inv / B).
    q_1.wrapping_add_assign(r_1);
    // Add floor(r_0 * inv / B) to q_0.
    if r_0.overflowing_add_assign(Limb::wrapping_from(
        (DoubleLimb::from(r_0) * DoubleLimb::from(inv)) >> Limb::WIDTH,
    )) {
        q_1.wrapping_add_assign(1);
    }
    assert!(r_1 <= 4);
    for _ in 0..r_1 {
        if r_0.overflowing_add_assign(inv) {
            q_1.wrapping_add_assign(1);
        }
    }
    (q_1, r_0)
}

// [x_0, x_1] cannot be equal to `2 ^ (2 * WIDTH - 1)`.
//
// This is mpfr_div_2 from div.c, MPFR 4.3.0, where Q0 and Q1 are returned, specialized for
// reciprocation.
fn reciprocal_float_significand_same_prec_gt_w_lt_2w(
    x_0: Limb,
    x_1: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, bool, Ordering) {
    let shift = TWICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    assert!(HIGH_BIT < x_1 || (HIGH_BIT == x_1 && x_0 != 0));
    let (mut q_1, mut q_0) = reciprocal_float_2_approx(x_1, x_0);
    // We know q1 * B + q0 is smaller or equal to the exact reciprocal, with difference at most 21.
    let mut sticky_bit = if (q_0.wrapping_add(21)) & (mask >> 1) > 21 {
        // The result is not exact when we can round with an approximation.
        1
    } else {
        // We know q_1 : q_0 is a good-enough approximation, so use it!
        //
        // Since we know the difference should be at most 21 * (x_1 : x_0) after the subtraction
        // below, thus at most 21 * 2 ^ 128, it suffices to compute the lower 3 limbs of (q_1 : q_0)
        // * (x_1 : x_0).
        let (mut s_1, mut s_0) = Limb::x_mul_y_to_zz(q_0, x_0);
        let (mut s_2, mut lo) = Limb::x_mul_y_to_zz(q_0, x_1);
        if s_1.overflowing_add_assign(lo) {
            s_2.wrapping_add_assign(1);
        }
        let hi;
        (hi, lo) = Limb::x_mul_y_to_zz(q_1, x_0);
        s_2.wrapping_add_assign(hi);
        if s_1.overflowing_add_assign(lo) {
            s_2.wrapping_add_assign(1);
        }
        s_2.wrapping_add_assign(q_1.wrapping_mul(x_1));
        // Subtract s_2 : s_1 : s_0 from 0 : 0 : 0, with result in s_2 : s_1 : s_0.
        s_2.wrapping_neg_assign();
        // Now negate s_1 : s_0.
        s_1.wrapping_neg_assign();
        if s_0.overflowing_neg_assign() {
            s_1.wrapping_sub_assign(1);
        }
        // There is a borrow in s_2 when s_0 and s_1 are not both zero.
        if s_1 != 0 || s_0 != 0 {
            s_2.wrapping_sub_assign(1);
        }
        while s_2 > 0 || s_1 > x_1 || s_1 == x_1 && s_0 >= x_0 {
            // Add 1 to q_1 : q_0.
            if q_0.overflowing_add_assign(1) {
                q_1.wrapping_add_assign(1);
            }
            // Subtract x_1 : x_0 to s_2 : s_1 : s_0
            if s_1 < x_1 || s_1 == x_1 && s_0 < x_0 {
                s_2.wrapping_sub_assign(1);
            }
            (s_1, s_0) = Limb::xx_sub_yy_to_zz(s_1, s_0, x_1, x_0);
        }
        s_1 | s_0
    };
    let round_bit = q_0 & (shift_bit >> 1);
    sticky_bit |= (q_0 & mask) ^ round_bit;
    let mut z_1 = q_1;
    let mut z_0 = q_0 & !mask;
    match rm {
        Exact => panic!("Inexact float reciprocation"),
        Nearest => {
            if round_bit == 0 || sticky_bit == 0 && z_0 & shift_bit == 0 {
                (z_0, z_1, false, Less)
            } else if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, false, Greater)
            }
        }
        Floor | Down => (z_0, z_1, false, Less),
        Ceiling | Up => {
            if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, false, Greater)
            }
        }
    }
}

// This is mpfr_div_ui from div_ui.c, MPFR 4.3.0, specialized for reciprocation.
fn reciprocal_float_significand_short(
    y: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Vec<Limb>, u64, Ordering) {
    let out_len = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let mut out = vec![0; out_len + 1];
    let (exp_offset, o) = reciprocal_float_significand_short_to_out(&mut out, y, prec, rm);
    out.truncate(out_len);
    (out, exp_offset, o)
}

fn limbs_reciprocal_limb_to_out_mod_with_fraction(
    out: &mut [Limb],
    fraction_len: usize,
    d: Limb,
) -> Limb {
    assert_ne!(d, 0);
    let len = fraction_len.checked_add(1).unwrap();
    assert_ne!(len, 0);
    let out = &mut out[..len];
    assert!(d.get_highest_bit());
    let (out_last, out_init) = out.split_last_mut().unwrap();
    *out_last = 0;
    // Multiply-by-inverse, divisor already normalized.
    let d_inv = limbs_invert_limb(d);
    let mut r = HIGH_BIT;
    for out_q in out_init[..fraction_len].iter_mut().rev() {
        (*out_q, r) = div_mod_by_preinversion(r, 0, d, d_inv);
    }
    r
}

// y cannot be a power of 2.
//
// This is mpfr_div_ui from div_ui.c, MPFR 4.3.0, specialized for reciprocation.
fn reciprocal_float_significand_short_to_out(
    out: &mut [Limb],
    y: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    let diff = out.len().abs_diff(1);
    // We need to store out_len + 1 = 1 + diff limbs of the reciprocal. used the entire dividend
    //
    // X = ({scratch, 1 + diff} * y + c) * B ^ (-diff} = ({scratch, out_len + 1} * y + c) * B ^
    // (-dif)
    let c = limbs_reciprocal_limb_to_out_mod_with_fraction(out, diff, y);
    // Let r = {xp, -diff} / B ^ (-diff) if diff < 0, r = 0 otherwise; 0 <= r < 1.
    //
    // Then X = ({scratch, out_len + 1} * y + c + r) * B ^ (-dif). x / y = (X / y) * B ^ (-1) * 2 ^
    // exp = ({scratch, out_len + 1} + (c + r) / y) * B ^ (-(out_len + 1)) * 2 ^ exp where 0 <= (c +
    // r) / y < 1.
    //
    // sticky_bit != 0 iff r != 0
    //
    // If the highest limb of the result is 0 (xs[0] < y), remove it. Otherwise, compute the left
    // shift to be performed to normalize. In the latter case, we discard some low bits computed.
    // They contain information useful for the rounding, hence the updating of middle and inexact.
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let shift_mask = shift_bit - 1;
    let out_head = out[0];
    // round bit is 1 iff (c + r) / u >= 1/2
    let (mut exp_offset, round_bit, sticky_bit) = if shift == 0 {
        // In this case scratch[out_len] = 0 and shift = 0, the round bit is not in {scratch,
        // out_len + 1}. It is 1 iff 2 * (c + r) - y >= 0. This means that in some cases, we should
        // look at the most significant bit of r.
        if c >= y - c {
            // i.e. 2 * c >= y: round bit is always 1
            //
            // The sticky bit is 1 unless 2 * c - y = 0 and r = 0.
            (0, 1, (c << 1).wrapping_sub(y))
        } else {
            // 2 * c < y
            //
            // The round bit is 1 iff r >= 1 / 2 and 2 * (c + 1 / 2) = y.
            //
            // If round_bit is set, we need to recompute sticky_bit, since it might have taken into
            // account the most-significant bit of xs[-diff - 1].
            (0, 0, c)
        }
    } else {
        // round bit is in scratch[0]
        (
            0,
            out_head & (shift_bit >> 1),
            (out_head & (shift_mask >> 1)) | c,
        )
    };
    // Clear the lowest `shift` bits
    out[0] &= !shift_mask;
    let (_, out) = out.split_last_mut().unwrap();
    match rm {
        Exact => panic!("Inexact float reciprocation"),
        Nearest => {
            if round_bit == 0 || sticky_bit == 0 && out[0] & shift_bit == 0 {
                (exp_offset, Less)
            } else {
                if limbs_slice_add_limb_in_place(out, shift_bit) {
                    exp_offset += 1;
                    *out.last_mut().unwrap() = HIGH_BIT;
                }
                (exp_offset, Greater)
            }
        }
        Floor | Down => (exp_offset, Less),
        Ceiling | Up => {
            if limbs_slice_add_limb_in_place(out, shift_bit) {
                exp_offset += 1;
                *out.last_mut().unwrap() = HIGH_BIT;
            }
            (exp_offset, Greater)
        }
    }
}

#[inline]
fn reciprocal_float_significand_general(
    ys: &mut [Limb],
    prec: u64,
    rm: RoundingMode,
) -> (Vec<Limb>, u64, Ordering) {
    let mut out = vec![0; usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
    let (exp_offset, o) = reciprocal_float_significand_general_to_out(&mut out, ys, prec, rm);
    (out, exp_offset, o)
}

// TODO special case qs == ds
//
// This is mpfr_div from div.c, MPFR 4.2.0, skipping over various special cases, specialized for
// reciprocation.
fn reciprocal_float_significand_general_to_out(
    qs: &mut [Limb],
    ds: &mut [Limb],
    prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    let ds_len = ds.len();
    let qs_len = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let qs = &mut qs[..qs_len];
    // Determine if an extra bit comes from the division, i.e. if the significand of X (as a
    // fraction in [1/2, 1) ) is larger than that of Y
    let ds_last = *ds.last().unwrap();
    let extra_bit = if ds_last == HIGH_BIT {
        // k = 0: no more dividend limb
        slice_test_zero(&ds[..ds_len - 1])
    } else {
        HIGH_BIT > ds_last
    };
    let mut exp_offset = u64::from(extra_bit);
    // shift is the number of zero bits in the low limb of the reciprocal
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let mut shift_bit = Limb::power_of_2(shift);
    let shift_mask = shift_bit - 1;
    let mut ys_vec;
    let mut ys: &mut [Limb];
    // We first try Mulders' short division (for large operands)
    if qs_len >= MPFR_DIV_THRESHOLD && ds_len >= MPFR_DIV_THRESHOLD {
        // We will perform a short (2 * n) / n division
        let n = qs_len + 1;
        let two_n = n << 1;
        // since Mulders' short division clobbers the dividend, we have to copy it
        let mut xs = vec![0; two_n];
        // zero-pad the dividend
        *xs.last_mut().unwrap() = HIGH_BIT;
        if ds_len >= n {
            // truncate the divisor
            ys = &mut ds[ds_len - n..];
        } else {
            // zero-pad the divisor
            ys_vec = vec![0; n];
            ys = &mut ys_vec;
            ys[n - ds_len..].copy_from_slice(ds);
        }
        // Since n = qs_len + 1, we have n >= 2 here.
        let mut scratch = vec![0; n + limbs_float_div_high_scratch_len(n)];
        let (qs_2, scratch) = scratch.split_at_mut(n);
        let q_high = limbs_float_div_high(qs_2, &mut xs, &ys[..n], scratch);
        // In all cases, the error is at most (2 * n + 2) ulps on q_high * B ^ n + {qs_2, n}.
        //
        // If rm == Nearest, we need to be able to round with a directed rounding and one more bit.
        if q_high {
            let qs_2_lo = &mut qs_2[..n];
            limbs_slice_shr_in_place(qs_2_lo, 1);
            *qs_2_lo.last_mut().unwrap() |= HIGH_BIT;
            // round_helper_2 would always return false, so no need to call it
        }
    }
    // Mulders' short division failed: we revert to integer division
    let mut qs_2_vec = vec![];
    let mut qs_2: &mut [Limb] = if rm == Nearest && shift == 0 {
        // We compute the reciprocal with one more limb, in order to get the round bit in the
        // reciprocal, and the remainder only contains sticky bits. Need to allocate memory for the
        // reciprocal
        qs_2_vec = vec![0; qs_len + 1];
        &mut qs_2_vec
    } else {
        qs // directly put the reciprocal in the destination
    };
    let qs_2_len = qs_2.len();
    let two_qs_2_len = qs_2_len << 1;
    // prepare the dividend
    let mut xs = vec![0; two_qs_2_len];
    // use the full dividend
    xs[two_qs_2_len - 1] = if extra_bit { HIGH_BIT >> 1 } else { HIGH_BIT };
    // Prepare the divisor
    let (mut k, sticky_y) = if ds_len >= qs_2_len {
        let k = ds_len - qs_2_len;
        let sy = !slice_test_zero(&ds[..k]);
        ys = &mut ds[k..]; // avoid copying the divisor
        (0, sy)
    } else {
        // ds_len < qs_2_len: small divisor case
        ys = ds;
        (qs_2_len - ds_len, false)
    };
    // If Mulders' short division failed, we revert to division with remainder.
    let mut q_high = limbs_div_helper(qs_2, &mut xs[k..], &ys[..qs_2_len - k]);
    k = qs_2_len;
    let sticky_x = !slice_test_zero(&xs[..k]);
    let mut sticky_bit = Limb::from(sticky_x | sticky_y);
    // now sticky_bit is non-zero iff one of the following holds:
    // - the truncated part of u is non-zero
    // - the truncated part of v is non-zero
    // - the remainder from division is non-zero
    let (mut sticky_3, shift_2) = if qs_2_len == qs_len {
        // does nothing when shift = 0
        (qs_2[0] & shift_mask, shift)
    } else {
        // qs_2_len = qs_len + 1: only happens when rm == Nearest and shift = 0
        qs.copy_from_slice(&qs_2_vec[1..=qs_len]);
        qs_2 = &mut qs_2_vec;
        (qs_2[0], Limb::WIDTH)
    };
    qs_2[0] ^= sticky_3;
    // sticky_3 contains the truncated bits from the reciprocal, including the round bit, and 1 <=
    // shift_2 <= WIDTH is the number of bits in sticky_3 to round, we distinguish two cases:
    // - ds_len <= qs_2_len: we used the full divisor
    // - ds_len > qs_2_len: the divisor was truncated
    let mut inex = Greater;
    let mut round_bit = 0;
    let mut cleanup = Cleanup::None;
    if ds_len <= qs_2_len {
        // use the full divisor
        sticky_bit = if rm == Nearest {
            round_bit = sticky_3 & Limb::power_of_2(shift_2 - 1);
            (sticky_3 ^ round_bit) | Limb::from(sticky_x)
        } else if rm == Exact {
            panic!("Inexact float reciprocation");
        } else {
            1
        };
    } else {
        // ds_len > qs_2_len: need to truncate the divisor
        //
        // We know the estimated reciprocal is an upper bound of the exact reciprocal (with rounding
        // toward zero), with a difference of at most 2 in qs_2[0]. Thus we can round except when
        // sticky_3 is 000...000 or 000...001 for directed rounding, and 100...000 or 100...001 for
        // rounding to nearest. (For rounding to nearest, we cannot determine the inexact flag for
        // 000...000 or 000...001.)
        let sticky_3_orig = sticky_3;
        if rm == Nearest {
            round_bit = sticky_3 & Limb::power_of_2(shift_2 - 1);
            sticky_3 ^= round_bit;
        }
        if sticky_3 > 1 {
            sticky_bit = sticky_3;
        } else {
            // hard case: we have to compare q1 * v0 and r + u0, where q1 * v0 has qs_2_len +
            // (ds_len-qs_2_len) = ds_len limbs, and r + u0 has qs_2_len + (usize-2*qs_2_len) =
            // usize-qs_2_len limbs
            let k = ds_len - qs_2_len;
            // sp <- {qs_2, qs_2_len} * {ds, ds_len - qs_2_len}
            let mut scratch = vec![0; ds_len + limbs_mul_to_out_scratch_len(qs_2_len, k)];
            let (sp, scratch) = scratch.split_at_mut(ds_len);
            qs_2[0] ^= sticky_3_orig; // restore original reciprocal
            let ds_lo = &ds[..k];
            limbs_mul_to_out(sp, qs_2, ds_lo, scratch);
            let q_high_2 = if q_high {
                limbs_slice_add_same_length_in_place_left(&mut sp[qs_2_len..], ds_lo)
            } else {
                false
            };
            qs_2[0] ^= sticky_3_orig;
            // restore truncated reciprocal
            //
            // Compare q_high_2 + {sp, ds_len} to {xs, qs_2_len} + u0
            let (sp_lo, sp_hi) = sp.split_at_mut(k);
            let mut cmp_s_r = if q_high_2 {
                Greater
            } else {
                limbs_cmp_same_length(sp_hi, &xs[..qs_2_len])
            };
            if cmp_s_r == Equal {
                // compare {sp, k} and u0
                cmp_s_r = if slice_test_zero(sp_lo) {
                    Equal
                } else {
                    Greater
                };
            }
            // now
            // - cmp_s_r > 0 if {sp, ds_len} > {xs, qs_2_len} + u0
            // - cmp_s_r = 0 if {sp, ds_len} = {xs, qs_2_len} + u0
            // - cmp_s_r < 0 if {sp, ds_len} < {xs, qs_2_len} + u0
            if cmp_s_r <= Equal {
                // reciprocal is in [q1, q1+1)
                sticky_bit = if cmp_s_r == Equal { sticky_3 } else { 1 };
            } else {
                // cmp_s_r > 0, reciprocal is < q1: to determine if it is in [q1 - 2, q1 - 1] or in
                // [q1 - 1, q1], we need to subtract the low part u0 of the dividend from q*v0
                // subtract u0 >> extra_bit if non-zero
                if q_high_2 {
                    // whatever the value of {ns, m + k}, it will be smaller than q_high_2 + {sp, k}
                    cmp_s_r = Greater;
                } else {
                    // subtract r
                    limbs_sub_same_length_in_place_left(sp_hi, &xs[..qs_2_len]);
                    // now compare {sp, ds_len} to y
                    cmp_s_r = limbs_cmp_same_length(sp, ds);
                }
                if cmp_s_r <= Equal {
                    // q1 - 1 <= x / y < q1
                    if sticky_3 == 1 {
                        // q1 - 1 is either representable (directed rounding), or the middle of two
                        // numbers (nearest)
                        sticky_bit = Limb::from(cmp_s_r != Equal);
                    } else if round_bit == 0 {
                        // round_bit=0, sticky_3=0: q1 - 1 is exact only when sh=0
                        inex = if cmp_s_r != Equal || shift != 0 {
                            Less
                        } else {
                            Equal
                        };
                        cleanup = if rm == Nearest || ((rm == Ceiling || rm == Up) && inex != Equal)
                        {
                            inex = Greater;
                            Cleanup::TruncateCheckQHigh
                        } else if inex != Equal && rm == Exact {
                            panic!("Inexact float reciprocation");
                        } else {
                            Cleanup::Sub1Ulp
                        };
                    } else {
                        // sticky_3 = 0, round_bit = 1 ==> rounding to nearest
                        return (exp_offset, cmp_s_r);
                    }
                } else {
                    // q1 - 2 < x / y < q1 - 1
                    //
                    // if rm == Nearest, the result is q1 when q1 - 2 >= q1 - 2 ^ (shift - 1), i.e.
                    // shift >= 2, otherwise (shift = 1) it is q1 - 2
                    (inex, cleanup) = if rm == Exact {
                        panic!("Inexact float reciprocation");
                    } else if rm == Nearest {
                        // shift > 0
                        //
                        // Case shift = 1: sticky_bit = 0 always, and q1 - round_bit is exactly
                        // representable, like q1 - round_bit - 2.
                        // ```
                        // round_bit action
                        // 0         subtract two ulps, inex = Less
                        // 1         truncate, inex = Greater
                        // ```
                        //
                        // Case shift > 1: one ulp is 2 ^ (shift - 1) >= 2
                        // ```
                        // round_bit sticky_bit action
                        // 0         0          truncate, inex = Greater
                        // 0         1          truncate, inex = Greater
                        // 1         x          truncate, inex = Less
                        // ```
                        if shift == 1 {
                            if round_bit == 0 {
                                shift_bit = 1;
                                (Less, Cleanup::Sub2Ulp)
                            } else {
                                (Greater, Cleanup::TruncateCheckQHigh)
                            }
                        } else {
                            (
                                if round_bit == 0 { Greater } else { Less },
                                Cleanup::TruncateCheckQHigh,
                            )
                        }
                    } else if rm == Floor || rm == Down {
                        // The result is down(q1 - 2), i.e. subtract one ulp if shift > 0, and two
                        // ulps if shift = 0
                        (
                            Less,
                            if shift == 0 {
                                Cleanup::Sub2Ulp
                            } else {
                                Cleanup::Sub1Ulp
                            },
                        )
                    } else {
                        (
                            Greater,
                            if shift == 0 {
                                Cleanup::Sub1Ulp
                            } else {
                                Cleanup::TruncateCheckQHigh
                            },
                        )
                    };
                }
            }
        }
    }
    match cleanup {
        Cleanup::None => {
            // reciprocal is in [q1, q1 + 1), round_bit is the round_bit (0 for directed rounding)
            return if rm == Floor || rm == Down || round_bit == 0 && sticky_bit == 0 {
                (
                    exp_offset,
                    if round_bit == 0 && sticky_bit == 0 {
                        Equal
                    } else {
                        Less
                    },
                )
            } else if rm == Exact {
                panic!("Inexact float reciprocation");
            } else if rm == Nearest {
                // sticky_bit != 0 or round != 0
                if round_bit == 0 {
                    // necessarily sticky_bit != 0
                    (exp_offset, Less)
                } else if sticky_bit != 0 {
                    if limbs_slice_add_limb_in_place(qs, shift_bit) {
                        exp_offset += 1;
                        // else qexp is now incorrect, but one will still get an overflow
                        *qs.last_mut().unwrap() = HIGH_BIT;
                    }
                    (exp_offset, Greater)
                } else {
                    fail_on_untested_path(
                        "div_float_significands_long_by_short, round_bit != 0 && sticky_bit != 0",
                    );
                    // round_bit = 1, sticky_bit = 0
                    if qs[0] & shift_bit == 0 {
                        (exp_offset, Less)
                    } else {
                        if limbs_slice_add_limb_in_place(qs, shift_bit) {
                            exp_offset += 1;
                            // else qexp is now incorrect, but one will still get an overflow
                            *qs.last_mut().unwrap() = HIGH_BIT;
                        }
                        (exp_offset, Greater)
                    }
                }
            } else {
                // round away from zero, sticky_bit != 0
                if limbs_slice_add_limb_in_place(qs, shift_bit) {
                    exp_offset += 1;
                    // else qexp is now incorrect, but one will still get an overflow
                    *qs.last_mut().unwrap() = HIGH_BIT;
                }
                (exp_offset, Greater)
            };
        }
        Cleanup::Sub1Ulp => {
            // we cannot subtract 1 << (shift + 1), since this is undefined for shift = WIDTH
            if limbs_sub_limb_in_place(qs, shift_bit) {
                q_high = false;
            }
        }
        Cleanup::Sub2Ulp => {
            if limbs_sub_limb_in_place(qs, shift_bit) {
                q_high = false;
            }
            if limbs_sub_limb_in_place(qs, shift_bit) {
                q_high = false;
            }
        }
        _ => {}
    }
    if q_high {
        exp_offset += 1;
        // else qexp is now incorrect, but one will still get an overflow
        *qs.last_mut().unwrap() = HIGH_BIT;
    }
    (exp_offset, inex)
}
