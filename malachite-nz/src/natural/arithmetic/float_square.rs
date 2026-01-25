// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::float_extras::round_helper_raw;
use crate::natural::arithmetic::float_mul::{
    limbs_float_mul_high_same_length, limbs_float_mul_high_same_length_scratch_len,
    mul_float_significands_ref_ref_helper,
};
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::square::{limbs_square_to_out, limbs_square_to_out_scratch_len};
use crate::natural::{LIMB_HIGH_BIT, Natural, bit_to_limb_count_ceiling};
use crate::platform::{DoubleLimb, Limb};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    OverflowingAddAssign, Parity, PowerOf2, Sign, WrappingAddAssign, XMulYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};

// This is mpfr_sqr from sqr.c, MPFR 4.3.0.
pub fn square_float_significand_in_place(
    x: &mut Natural,
    x_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    if out_prec == x_prec
        && let Some((decrement_exp, o)) =
            square_float_significand_in_place_same_prec(x, out_prec, rm)
    {
        return (-i32::from(decrement_exp), o);
    }
    let (square, exp_offset, o) = match &*x {
        Natural(Small(x)) => square_float_significands_general(&[*x], x_prec, out_prec, rm),
        Natural(Large(xs)) => square_float_significands_general(xs, x_prec, out_prec, rm),
    };
    *x = square;
    (exp_offset, o)
}

// This is mpfr_sqr from sqr.c, MPFR 4.3.0.
pub fn square_float_significand_ref(
    x: &Natural,
    x_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    match x {
        Natural(Small(x)) => square_float_significand_ref_helper(&[*x], x_prec, out_prec, rm),
        Natural(Large(xs)) => square_float_significand_ref_helper(xs, x_prec, out_prec, rm),
    }
}

fn square_float_significand_ref_helper(
    xs: &[Limb],
    x_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    if out_prec == x_prec
        && let Some((square, decrement_exp, o)) =
            square_float_significand_same_prec_ref(xs, out_prec, rm)
    {
        return (square, -i32::from(decrement_exp), o);
    }
    square_float_significands_general(xs, x_prec, out_prec, rm)
}

fn square_float_significand_in_place_same_prec(
    x: &mut Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(bool, Ordering)> {
    match x {
        Natural(Small(x)) => {
            let (square, decrement_exp, o) = if prec == Limb::WIDTH {
                square_float_significand_same_prec_w(*x, rm)
            } else {
                square_float_significand_same_prec_lt_w(*x, prec, rm)
            };
            *x = square;
            Some((decrement_exp, o))
        }
        Natural(Large(xs)) => match xs.as_mut_slice() {
            [x_0, x_1] if prec != TWICE_WIDTH => {
                let (square_0, square_1, decrement_exp, o) =
                    square_float_significand_same_prec_gt_w_lt_2w(*x_0, *x_1, prec, rm);
                *x_0 = square_0;
                *x_1 = square_1;
                Some((decrement_exp, o))
            }
            [x_0, x_1, x_2] if prec != THRICE_WIDTH => {
                let (square_0, square_1, square_2, decrement_exp, o) =
                    square_float_significand_same_prec_gt_2w_lt_3w(*x_0, *x_1, *x_2, prec, rm);
                *x_0 = square_0;
                *x_1 = square_1;
                *x_2 = square_2;
                Some((decrement_exp, o))
            }
            _ => None,
        },
    }
}

fn square_float_significand_same_prec_ref(
    xs: &[Limb],
    prec: u64,
    rm: RoundingMode,
) -> Option<(Natural, bool, Ordering)> {
    match xs {
        [x] => {
            let (square, decrement_exp, o) = if prec == Limb::WIDTH {
                square_float_significand_same_prec_w(*x, rm)
            } else {
                square_float_significand_same_prec_lt_w(*x, prec, rm)
            };
            Some((Natural(Small(square)), decrement_exp, o))
        }
        [x_0, x_1] if prec != TWICE_WIDTH => {
            let (square_0, square_1, decrement_exp, o) =
                square_float_significand_same_prec_gt_w_lt_2w(*x_0, *x_1, prec, rm);
            Some((Natural(Large(vec![square_0, square_1])), decrement_exp, o))
        }
        [x_0, x_1, x_2] if prec != THRICE_WIDTH => {
            let (square_0, square_1, square_2, decrement_exp, o) =
                square_float_significand_same_prec_gt_2w_lt_3w(*x_0, *x_1, *x_2, prec, rm);
            Some((
                Natural(Large(vec![square_0, square_1, square_2])),
                decrement_exp,
                o,
            ))
        }
        _ => None,
    }
}

const WIDTH_M1: u64 = Limb::WIDTH - 1;
const COMP_HIGH_BIT: Limb = !LIMB_HIGH_BIT;

// This is mpfr_sqr_1 from sqr.c, MPFR 4.3.0.
fn square_float_significand_same_prec_lt_w(
    x: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, bool, Ordering) {
    let shift = Limb::WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    let (mut z, mut sticky_bit) = Limb::x_mul_y_to_zz(x, x);
    let decrement_exp = !z.get_highest_bit();
    if decrement_exp {
        z <<= 1;
        z |= sticky_bit >> WIDTH_M1;
        sticky_bit <<= 1;
    }
    let round_bit = z & (shift_bit >> 1);
    sticky_bit |= (z & mask) ^ round_bit;
    let mut square = z & !mask;
    if round_bit == 0 && sticky_bit == 0 {
        return (z, decrement_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float squaring"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && square & shift_bit == 0) {
                (square, decrement_exp, Less)
            } else if square.overflowing_add_assign(shift_bit) {
                (LIMB_HIGH_BIT, false, Greater)
            } else {
                (square, decrement_exp, Greater)
            }
        }
        Floor | Down => (square, decrement_exp, Less),
        Ceiling | Up => {
            if square.overflowing_add_assign(shift_bit) {
                (LIMB_HIGH_BIT, false, Greater)
            } else {
                (square, decrement_exp, Greater)
            }
        }
    }
}

// This is mpfr_sqr_1n from sqr.c, MPFR 4.2.0.
fn square_float_significand_same_prec_w(x: Limb, rm: RoundingMode) -> (Limb, bool, Ordering) {
    let (mut z, mut sticky_bit) = Limb::x_mul_y_to_zz(x, x);
    let decrement_exp = !z.get_highest_bit();
    if decrement_exp {
        z <<= 1;
        z |= sticky_bit >> WIDTH_M1;
        sticky_bit <<= 1;
    }
    let round_bit = sticky_bit & LIMB_HIGH_BIT;
    sticky_bit &= COMP_HIGH_BIT;
    let mut square = z;
    if round_bit == 0 && sticky_bit == 0 {
        return (z, decrement_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float squaring"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && square.even()) {
                (square, decrement_exp, Less)
            } else if square.overflowing_add_assign(1) {
                (LIMB_HIGH_BIT, false, Greater)
            } else {
                (square, decrement_exp, Greater)
            }
        }
        Floor | Down => (square, decrement_exp, Less),
        Ceiling | Up => {
            if square.overflowing_add_assign(1) {
                (LIMB_HIGH_BIT, false, Greater)
            } else {
                (square, decrement_exp, Greater)
            }
        }
    }
}

const TWICE_WIDTH: u64 = Limb::WIDTH * 2;
const THRICE_WIDTH: u64 = Limb::WIDTH * 3;

// This is mpfr_sqr_2 from sqr.c, MPFR 4.2.0.
fn square_float_significand_same_prec_gt_w_lt_2w(
    x_0: Limb,
    x_1: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, bool, Ordering) {
    let shift = TWICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    // we store the 4-limb square in h = z[1], l = z[0], sticky_bit = z[-1], sticky_bit_2 = z[-2]
    let (mut hi, mut lo) = Limb::x_mul_y_to_zz(x_1, x_1);
    let (u, v) = Limb::x_mul_y_to_zz(x_0, x_1);
    if lo.overflowing_add_assign(u) {
        hi += 1;
    }
    if lo.overflowing_add_assign(u) {
        hi.wrapping_add_assign(1);
    }
    // now the full square is {hi, lo, 2 * v + high(x_0 ^ 2), low(x_0 ^ 2)}, where the lower part
    // contributes to less than 3 ulps to {hi, lo}.
    //
    // If hi has its most significant bit set and the low shift - 1 bits of lo are not 000...000 nor
    // 111...111 nor 111...110, then we can round correctly; if hi has zero as most significant bit,
    // we have to shift left hi and lo, thus if the low sh-2 bits are not 000...000 nor 111...111
    // nor 111...110, then we can round correctly. To avoid an extra test we consider the latter
    // case (if we can round, we can also round in the former case). For shift <= 3, we have mask <=
    // 7, thus (mask >> 2) <= 1, and the approximation cannot be enough.
    let (mut sticky_bit, sticky_bit_2);
    if lo.wrapping_add(2) & (mask >> 2) > 2 {
        // result cannot be exact in that case
        sticky_bit = 1;
        sticky_bit_2 = 1;
    } else {
        (sticky_bit, sticky_bit_2) = Limb::x_mul_y_to_zz(x_0, x_0);
        // The full square is {h, l, sticky_bit + v + w, sticky_bit_2}
        if sticky_bit.overflowing_add_assign(v) && lo.overflowing_add_assign(1) {
            hi.wrapping_add_assign(1);
        }
        if sticky_bit.overflowing_add_assign(v) && lo.overflowing_add_assign(1) {
            hi.wrapping_add_assign(1);
        }
    }
    let decrement_exp = !hi.get_highest_bit();
    if decrement_exp {
        hi <<= 1;
        hi |= lo >> WIDTH_M1;
        lo <<= 1;
        lo |= sticky_bit >> WIDTH_M1;
        sticky_bit <<= 1;
        // no need to shift sticky_bit_2 since we only want to know if it is zero or not
    }
    let mut z_1 = hi;
    let round_bit = lo & (shift_bit >> 1);
    sticky_bit |= ((lo & mask) ^ round_bit) | sticky_bit_2;
    let mut z_0 = lo & !mask;
    if round_bit == 0 && sticky_bit == 0 {
        return (z_0, z_1, decrement_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float squaring"),
        Nearest => {
            if round_bit == 0 || sticky_bit == 0 && (z_0 & shift_bit) == 0 {
                (z_0, z_1, decrement_exp, Less)
            } else if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, LIMB_HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, decrement_exp, Greater)
            }
        }
        Floor | Down => (z_0, z_1, decrement_exp, Less),
        Ceiling | Up => {
            if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, LIMB_HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, decrement_exp, Greater)
            }
        }
    }
}

const LIMB_MASK: DoubleLimb = (1 << Limb::WIDTH) - 1;

// This is mpfr_sqr_3 from sqr.c, MPFR 4.2.0.
fn square_float_significand_same_prec_gt_2w_lt_3w(
    x_0: Limb,
    x_1: Limb,
    x_2: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, Limb, bool, Ordering) {
    let shift = THRICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    // we store the upper 3-limb square in z2, z1, z0: x2 ^ 2, 2 * x1 * x2, 2 * x0 * x2 + x1 ^ 2
    let x_0 = DoubleLimb::from(x_0);
    let x_1 = DoubleLimb::from(x_1);
    let x_2 = DoubleLimb::from(x_2);
    let x_2_x_2 = x_2 * x_2;
    let x_1_x_2 = x_1 * x_2;
    let x_1_x_1 = x_1 * x_1;
    let x_0_x_2 = x_0 * x_2;
    let (mut a2, mut a1) = x_2_x_2.split_in_half();
    let (hi, mut a0) = x_1_x_2.split_in_half();
    if a1.overflowing_add_assign(hi) {
        a2 += 1;
    }
    if a1.overflowing_add_assign(hi) {
        a2.wrapping_add_assign(1);
    }
    let mut carry = Limb::from(a0.overflowing_add_assign(a0));
    let x_0_x_2_hi = Limb::wrapping_from(x_0_x_2 >> Limb::WIDTH);
    if a0.overflowing_add_assign(x_0_x_2_hi) {
        carry += 1;
    }
    if a0.overflowing_add_assign(x_0_x_2_hi) {
        carry += 1;
    }
    if a0.overflowing_add_assign(Limb::wrapping_from(x_1_x_1 >> Limb::WIDTH)) {
        carry += 1;
    }
    // now propagate carry
    if a1.overflowing_add_assign(carry) {
        a2.wrapping_add_assign(1);
    }
    // Now the approximate square {a2, a1, a0} has an error of less than 5 ulps (3 ulps for the
    // ignored low limbs of x_2 * y_0 + x_1 * y_1 + x_0 * y2, plus 2 ulps for the ignored x_1 * y_0
    // + x_0 * y_1 (plus x_0 * y_0)). Since we might shift by 1 bit, we make sure the low shift - 2
    // bits of a0 are not 0, -1, -2, -3 or -4.
    let (mut sticky_bit, sticky_bit_2) = if a0.wrapping_add(4) & (mask >> 2) > 4 {
        // result cannot be exact in that case
        (1, 1)
    } else {
        let out = x_0 * x_0;
        let p_0 = out & LIMB_MASK;
        let x_0_x_1 = x_0 * x_1;
        let out = x_0_x_1 + (out >> Limb::WIDTH);
        let mut p_1 = out & LIMB_MASK;
        let out = x_0_x_2 + (out >> Limb::WIDTH);
        let mut p_2 = out & LIMB_MASK;
        let mut p_3 = out >> Limb::WIDTH;
        let out = p_1 + x_0_x_1;
        p_1 = out & LIMB_MASK;
        let out = p_2 + x_1_x_1 + (out >> Limb::WIDTH);
        p_2 = out & LIMB_MASK;
        let out = p_3 + x_1_x_2 + (out >> Limb::WIDTH);
        p_3 = out & LIMB_MASK;
        let p_4 = out >> Limb::WIDTH;
        let out = p_2 + x_0_x_2;
        p_2 = out & LIMB_MASK;
        let out = p_3 + x_1_x_2 + (out >> Limb::WIDTH);
        p_3 = out & LIMB_MASK;
        let out = p_4 + x_2_x_2 + (out >> Limb::WIDTH);
        (a2, a1) = out.split_in_half();
        a0 = Limb::wrapping_from(p_3);
        (
            Limb::wrapping_from(p_2),
            Limb::wrapping_from(p_1) | Limb::wrapping_from(p_0),
        )
    };
    let decrement_exp = !a2.get_highest_bit();
    if decrement_exp {
        a2 <<= 1;
        a2 |= a1 >> WIDTH_M1;
        a1 <<= 1;
        a1 |= a0 >> WIDTH_M1;
        a0 <<= 1;
        a0 |= sticky_bit >> WIDTH_M1;
        sticky_bit <<= 1;
        // no need to shift sticky_bit_2: we only need to know if it is zero or not
    }
    let mut z_2 = a2;
    let mut z_1 = a1;
    let round_bit = a0 & (shift_bit >> 1);
    sticky_bit |= ((a0 & mask) ^ round_bit) | sticky_bit_2;
    let mut z_0 = a0 & !mask;
    if round_bit == 0 && sticky_bit == 0 {
        return (z_0, z_1, z_2, decrement_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float squaring"),
        Nearest => {
            if round_bit == 0 || sticky_bit == 0 && z_0 & shift_bit == 0 {
                (z_0, z_1, z_2, decrement_exp, Less)
            } else {
                if z_0.overflowing_add_assign(shift_bit) {
                    z_1.wrapping_add_assign(1);
                }
                if z_1 == 0 && z_0 == 0 {
                    z_2.wrapping_add_assign(1);
                }
                if z_2 == 0 {
                    (z_0, z_1, LIMB_HIGH_BIT, false, Greater)
                } else {
                    (z_0, z_1, z_2, decrement_exp, Greater)
                }
            }
        }
        Floor | Down => (z_0, z_1, z_2, decrement_exp, Less),
        Ceiling | Up => {
            if z_0.overflowing_add_assign(shift_bit) {
                z_1.wrapping_add_assign(1);
            }
            if z_1 == 0 && z_0 == 0 {
                z_2.wrapping_add_assign(1);
            }
            if z_2 == 0 {
                (z_0, z_1, LIMB_HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, z_2, decrement_exp, Greater)
            }
        }
    }
}

pub(crate) fn limbs_float_square_high_scratch_len(n: usize) -> usize {
    let k = MPFR_SQRHIGH_TAB
        .get(n)
        .map_or_else(|| isize::exact_from((n + 4) >> 1), |&x| isize::from(x));
    if k < 0 {
        limbs_square_to_out_scratch_len(n)
    } else if k == 0 {
        0
    } else {
        let k = usize::wrapping_from(k);
        max(
            limbs_square_to_out_scratch_len(k),
            limbs_float_mul_high_same_length_scratch_len(n - k),
        )
    }
}

pub(crate) const MPFR_SQRHIGH_TAB: [i8; 17] =
    [-1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// This is mpfr_mulhigh_n_basecase from mulders.c, MPFR 4.2.0, specialized for squaring.
fn limbs_float_sqr_high_same_length_basecase(out: &mut [Limb], xs: &[Limb]) {
    let len = xs.len();
    // We neglect xs[0..len - 2] * xs[0], which is less than B ^ len
    let out = &mut out[len - 1..];
    (out[1], out[0]) = Limb::x_mul_y_to_zz(*xs.last().unwrap(), xs[0]);
    for (i, x) in xs.iter().enumerate() {
        let i = i + 1;
        // Here, we neglect xs[0..len - i - 2] * xs[i], which is less than B ^ len too
        let (out_lo, out_hi) = out.split_at_mut(i);
        out_hi[0] = limbs_slice_add_mul_limb_same_length_in_place_left(out_lo, &xs[len - i..], *x);
        // In total, we neglect less than n * B ^ len, i.e., n ulps of out[len].
    }
}

// Put in out[n..2 * len - 1] an approximation of the n high limbs of xs ^ 2. The error is less than
// len ulps of out[len] (and the approximation is always less or equal to the truncated full
// square).
//
// Implements Algorithm ShortMul from:
//
// [1] Short Division of Long Integers, David Harvey and Paul Zimmermann, Proceedings of the 20th
// Symposium on Computer Arithmetic (ARITH-20), July 25-27, 2011, pages 7-14.
//
// This is mpfr_sqrhigh_n from mulders.c, MPFR 4.2.0.
pub(crate) fn limbs_float_square_high(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    const LEN_ASSERT: bool = MPFR_SQRHIGH_TAB.len() > 2;
    assert!(LEN_ASSERT);
    let n = xs.len();
    let k = MPFR_SQRHIGH_TAB
        .get(n)
        .map_or_else(|| isize::exact_from((n + 4) >> 1), |&x| isize::from(x));
    assert!(
        k == -1 || k == 0 || (k >= isize::exact_from((n + 4) >> 1) && k < isize::exact_from(n))
    );
    if k < 0 {
        limbs_square_to_out(out, xs, scratch);
    } else if k == 0 {
        limbs_float_sqr_high_same_length_basecase(out, xs);
    } else {
        let k = usize::wrapping_from(k);
        let l = n - k;
        limbs_square_to_out(&mut out[l << 1..], &xs[l..], scratch);
        let (xs_lo, xs_hi) = xs.split_at(k);
        limbs_float_mul_high_same_length(out, &xs_lo[..l], xs_hi, scratch);
        let (out_lo, out_hi) = out.split_at_mut(n - 1);
        let out_lo = &mut out_lo[l - 1..l << 1];
        let mut carry = limbs_slice_shl_in_place(out_lo, 1);
        if limbs_slice_add_same_length_in_place_left(&mut out_hi[..=l], out_lo) {
            carry += 1;
        }
        limbs_slice_add_limb_in_place(&mut out[n + l..n << 1], carry);
    }
}

const MPFR_SQR_THRESHOLD: usize = 20;

fn square_float_significands_general(
    xs: &[Limb],
    x_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    let xs_len = xs.len();
    let tn = bit_to_limb_count_ceiling(x_prec << 1);
    if xs_len > MPFR_SQR_THRESHOLD {
        return mul_float_significands_ref_ref_helper(xs, x_prec, xs, x_prec, out_prec, rm);
    }
    let xs_len_2 = xs_len << 1;
    let mut scratch = vec![0; xs_len_2 + limbs_square_to_out_scratch_len(xs.len())];
    let (tmp, scratch) = scratch.split_at_mut(xs_len_2);
    // Multiplies the mantissa in temporary allocated space
    limbs_square_to_out(tmp, xs, scratch);
    let mut b1 = tmp[xs_len_2 - 1];
    // now tmp[0]..tmp[2 * xs_len - 1] contains the square of the mantissa, with tmp[2 * xs_len - 1]
    // >= 2 ^ (Limb::WIDTH - 2)
    b1 >>= const { Limb::WIDTH - 1 }; // msb from the product
    // if the mantissas of b and c are uniformly distributed in (1/2, 1], then their product is in
    // (1/4, 1/2] with probability 2 * ln(2) - 1 ~ 0.386 and in [1/2, 1] with probability 2 - 2 *
    // ln(2) ~ 0.614
    let tmp = &mut tmp[xs_len_2 - tn..];
    if b1 == 0 {
        limbs_slice_shl_in_place(&mut tmp[..tn], 1);
    }
    let mut out = vec![0; bit_to_limb_count_ceiling(out_prec)];
    let (inexact, increment_exp) = round_helper_raw(&mut out, out_prec, tmp, x_prec << 1, rm);
    assert!(inexact == 0 || rm != Exact, "Inexact float squaring");
    let mut exp_offset = -i32::from(b1 == 0);
    if increment_exp {
        exp_offset += 1;
    }
    (
        Natural::from_owned_limbs_asc(out),
        exp_offset,
        inexact.sign(),
    )
}
