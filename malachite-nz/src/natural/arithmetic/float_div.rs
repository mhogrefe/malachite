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

use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::div::limbs_div_schoolbook_approx;
use crate::natural::arithmetic::div_mod::{
    div_mod_by_preinversion, limbs_div_limb_in_place_mod, limbs_div_limb_to_out_mod,
    limbs_div_mod_by_two_limb_normalized, limbs_div_mod_qs_to_out_rs_to_ns, limbs_invert_limb,
    limbs_two_limb_inverse_helper,
};
use crate::natural::arithmetic::float_extras::round_helper_2;
use crate::natural::arithmetic::float_mul::{
    limbs_float_mul_high_same_length, limbs_float_mul_high_same_length_scratch_len,
};
use crate::natural::arithmetic::mul::{limbs_mul_to_out, limbs_mul_to_out_scratch_len};
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
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
    CeilingLogBase2, NegModPowerOf2, OverflowingAddAssign, OverflowingNegAssign, Parity, PowerOf2,
    ShrRound, WrappingAddAssign, WrappingNegAssign, WrappingSubAssign, XMulYToZZ, XXSubYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

// This is mpfr_div from div.c, MPFR 4.3.0.
pub fn div_float_significands_in_place(
    x: &mut Natural,
    x_prec: u64,
    y: &mut Natural,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        if let Some((increment_exp, o)) =
            div_float_significands_in_place_same_prec(x, y, out_prec, rm)
        {
            return (u64::from(increment_exp), o);
        }
    }
    match (&mut *x, y) {
        (Natural(Small(small_x)), Natural(Small(small_y))) => {
            let mut xs = vec![*small_x];
            let result =
                div_float_significands_long_by_short_in_place(&mut xs, *small_y, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(xs);
            result
        }
        (Natural(Large(ref mut xs)), Natural(Small(small_y))) => {
            let result = div_float_significands_long_by_short_in_place(xs, *small_y, out_prec, rm);
            x.demote_if_small();
            result
        }
        (Natural(Small(small_x)), Natural(Large(ref mut ys))) => {
            let (out, exp_offset, o) =
                div_float_significands_general(&[*small_x], ys, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(out);
            (exp_offset, o)
        }
        (Natural(Large(xs)), Natural(Large(ref mut ys))) => {
            let (out, exp_offset, o) = div_float_significands_general(xs, ys, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(out);
            (exp_offset, o)
        }
    }
}

// This is mpfr_div from div.c, MPFR 4.3.0.
pub fn div_float_significands_in_place_ref(
    x: &mut Natural,
    x_prec: u64,
    y: &Natural,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        if let Some((increment_exp, o)) =
            div_float_significands_in_place_same_prec_ref(x, y, out_prec, rm)
        {
            return (u64::from(increment_exp), o);
        }
    }
    match (&mut *x, y) {
        (Natural(Small(small_x)), Natural(Small(small_y))) => {
            let mut xs = vec![*small_x];
            let result =
                div_float_significands_long_by_short_in_place(&mut xs, *small_y, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(xs);
            result
        }
        (Natural(Large(ref mut xs)), Natural(Small(small_y))) => {
            let result = div_float_significands_long_by_short_in_place(xs, *small_y, out_prec, rm);
            x.demote_if_small();
            result
        }
        (Natural(Small(small_x)), y) => {
            let mut ys = y.to_limbs_asc();
            let (out, exp_offset, o) =
                div_float_significands_general(&[*small_x], &mut ys, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(out);
            (exp_offset, o)
        }
        (Natural(Large(xs)), y) => {
            let mut ys = y.to_limbs_asc();
            let (out, exp_offset, o) = div_float_significands_general(xs, &mut ys, out_prec, rm);
            *x = Natural::from_owned_limbs_asc(out);
            (exp_offset, o)
        }
    }
}

// This is mpfr_div from div.c, MPFR 4.3.0.
pub fn div_float_significands_ref_val(
    x: &Natural,
    x_prec: u64,
    y: &mut Natural,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, u64, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        if let Some((quotient, increment_exp, o)) =
            div_float_significands_same_prec_ref_val(x, y, out_prec, rm)
        {
            return (quotient, u64::from(increment_exp), o);
        }
    }
    match (x, y) {
        (Natural(Small(small_x)), Natural(Small(small_y))) => {
            let (qs, exp_offset, o) =
                div_float_significands_long_by_short(&[*small_x], *small_y, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
        (Natural(Large(xs)), Natural(Small(small_y))) => {
            let (qs, exp_offset, o) =
                div_float_significands_long_by_short(xs, *small_y, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
        (Natural(Small(small_x)), Natural(Large(ref mut ys))) => {
            let (qs, exp_offset, o) = div_float_significands_general(&[*small_x], ys, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
        (Natural(Large(xs)), Natural(Large(ref mut ys))) => {
            let (qs, exp_offset, o) = div_float_significands_general(xs, ys, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
    }
}

// This is mpfr_div from div.c, MPFR 4.3.0.
pub fn div_float_significands_ref_ref(
    x: &Natural,
    x_prec: u64,
    y: &Natural,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, u64, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        if let Some((quotient, increment_exp, o)) =
            div_float_significands_same_prec_ref_ref(x, y, out_prec, rm)
        {
            return (quotient, u64::from(increment_exp), o);
        }
    }
    match (x, y) {
        (Natural(Small(small_x)), Natural(Small(small_y))) => {
            let (qs, exp_offset, o) =
                div_float_significands_long_by_short(&[*small_x], *small_y, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
        (Natural(Large(xs)), Natural(Small(small_y))) => {
            let (qs, exp_offset, o) =
                div_float_significands_long_by_short(xs, *small_y, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
        (Natural(Small(small_x)), y) => {
            let mut ys = y.to_limbs_asc();
            let (qs, exp_offset, o) =
                div_float_significands_general(&[*small_x], &mut ys, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
        (Natural(Large(xs)), y) => {
            let mut ys = y.to_limbs_asc();
            let (qs, exp_offset, o) = div_float_significands_general(xs, &mut ys, out_prec, rm);
            (Natural::from_owned_limbs_asc(qs), exp_offset, o)
        }
    }
}

fn div_float_significands_in_place_same_prec(
    x: &mut Natural,
    y: &mut Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(bool, Ordering)> {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (quotient, increment_exp, o) = if prec == Limb::WIDTH {
                div_float_significands_same_prec_w(*x, *y, rm)
            } else {
                div_float_significands_same_prec_lt_w(*x, *y, prec, rm)
            };
            *x = quotient;
            Some((increment_exp, o))
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_mut_slice()) {
            ([x_0, x_1], [y_0, y_1]) if prec != TWICE_WIDTH => {
                let (quotient_0, quotient_1, increment_exp, o) =
                    div_float_significands_same_prec_gt_w_lt_2w(*x_0, *x_1, *y_0, *y_1, prec, rm);
                *x_0 = quotient_0;
                *x_1 = quotient_1;
                Some((increment_exp, o))
            }
            _ => None,
        },
        _ => None,
    }
}

fn div_float_significands_in_place_same_prec_ref(
    x: &mut Natural,
    y: &Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(bool, Ordering)> {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (quotient, increment_exp, o) = if prec == Limb::WIDTH {
                div_float_significands_same_prec_w(*x, *y, rm)
            } else {
                div_float_significands_same_prec_lt_w(*x, *y, prec, rm)
            };
            *x = quotient;
            Some((increment_exp, o))
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_slice()) {
            ([x_0, x_1], [y_0, y_1]) if prec != TWICE_WIDTH => {
                let (quotient_0, quotient_1, increment_exp, o) =
                    div_float_significands_same_prec_gt_w_lt_2w(*x_0, *x_1, *y_0, *y_1, prec, rm);
                *x_0 = quotient_0;
                *x_1 = quotient_1;
                Some((increment_exp, o))
            }
            _ => None,
        },
        _ => None,
    }
}

fn div_float_significands_same_prec_ref_val(
    x: &Natural,
    y: &mut Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Natural, bool, Ordering)> {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (quotient, increment_exp, o) = if prec == Limb::WIDTH {
                div_float_significands_same_prec_w(*x, *y, rm)
            } else {
                div_float_significands_same_prec_lt_w(*x, *y, prec, rm)
            };
            Some((Natural(Small(quotient)), increment_exp, o))
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_slice(), ys.as_slice()) {
            ([x_0, x_1], [y_0, y_1]) if prec != TWICE_WIDTH => {
                let (quotient_0, quotient_1, increment_exp, o) =
                    div_float_significands_same_prec_gt_w_lt_2w(*x_0, *x_1, *y_0, *y_1, prec, rm);
                Some((
                    Natural(Large(vec![quotient_0, quotient_1])),
                    increment_exp,
                    o,
                ))
            }
            _ => None,
        },
        _ => None,
    }
}
fn div_float_significands_same_prec_ref_ref(
    x: &Natural,
    y: &Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Natural, bool, Ordering)> {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (quotient, increment_exp, o) = if prec == Limb::WIDTH {
                div_float_significands_same_prec_w(*x, *y, rm)
            } else {
                div_float_significands_same_prec_lt_w(*x, *y, prec, rm)
            };
            Some((Natural(Small(quotient)), increment_exp, o))
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_slice(), ys.as_slice()) {
            ([x_0, x_1], [y_0, y_1]) if prec != TWICE_WIDTH => {
                let (quotient_0, quotient_1, increment_exp, o) =
                    div_float_significands_same_prec_gt_w_lt_2w(*x_0, *x_1, *y_0, *y_1, prec, rm);
                Some((
                    Natural(Large(vec![quotient_0, quotient_1])),
                    increment_exp,
                    o,
                ))
            }
            _ => None,
        },
        _ => None,
    }
}

const WIDTH_M1: u64 = Limb::WIDTH - 1;
const HIGH_BIT: Limb = 1 << WIDTH_M1;
const TWICE_WIDTH: u64 = Limb::WIDTH * 2;

// This is mpfr_div_1 from mul.c, MPFR 4.3.0.
fn div_float_significands_same_prec_lt_w(
    mut x: Limb,
    y: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, bool, Ordering) {
    let shift = Limb::WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let half_shift_bit = shift_bit >> 1;
    let mask = shift_bit - 1;
    let increment_exp = x >= y;
    if increment_exp {
        x -= y;
    }
    // First try with an approximate quotient.
    let mut round_bit = Limb::wrapping_from(
        (DoubleLimb::from(x) * DoubleLimb::from(limbs_invert_limb(y))) >> Limb::WIDTH,
    );
    round_bit.wrapping_add_assign(x);
    let mut q = if increment_exp {
        round_bit >> 1
    } else {
        round_bit
    };
    // round_bit does not exceed the true quotient floor(x * 2 ^ WIDTH / y), with error at most 2,
    // which means the rational quotient q satisfies round_bit <= q < round_bit + 3. We can round
    // correctly except when the last shift - 1 bits of q0 are 000..000 or 111..111 or 111..110.
    let sticky_bit = if (q + 2) & (mask >> 1) > 2 {
        round_bit = q & half_shift_bit;
        // result cannot be exact in this case
        1
    } else {
        // The true quotient is round_bit, round_bit + 1, or round_bit + 2
        q = round_bit;
        let (mut hi, mut lo) = Limb::x_mul_y_to_zz(q, y);
        assert!(hi < x || (hi == x && lo == 0));
        // subtract {hi, lo} from {x, 0}
        (hi, lo) = Limb::xx_sub_yy_to_zz(x, 0, hi, lo);
        // the remainder {hi, lo} should be < y.
        if hi != 0 || lo >= y {
            q += 1;
            if lo < y {
                hi.wrapping_sub_assign(1);
            }
            lo.wrapping_sub_assign(y);
        }
        if hi != 0 || lo >= y {
            q += 1;
            if lo < y {
                hi.wrapping_sub_assign(1);
            }
            lo.wrapping_sub_assign(y);
        }
        assert!(hi == 0 && lo < y);
        let sticky_bit;
        if increment_exp {
            sticky_bit = lo | (q & 1);
            q >>= 1;
        } else {
            sticky_bit = lo;
        }
        round_bit = q & half_shift_bit;
        sticky_bit | (q & (mask >> 1))
    };
    let quotient = (HIGH_BIT | q) & !mask;
    if round_bit == 0 && sticky_bit == 0 {
        return (quotient, increment_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float division"),
        Nearest => {
            if round_bit == 0 || sticky_bit == 0 && quotient & shift_bit == 0 {
                (quotient, increment_exp, Less)
            } else {
                (quotient.wrapping_add(shift_bit), increment_exp, Greater)
            }
        }
        Floor | Down => (quotient, increment_exp, Less),
        Ceiling | Up => (quotient.wrapping_add(shift_bit), increment_exp, Greater),
    }
}

// This is mpfr_div_1n from mul.c, MPFR 4.3.0.
fn div_float_significands_same_prec_w(
    mut x: Limb,
    y: Limb,
    rm: RoundingMode,
) -> (Limb, bool, Ordering) {
    let increment_exp = x >= y;
    if increment_exp {
        x -= y;
    }
    // First compute an approximate quotient.
    let mut q = x.wrapping_add(Limb::wrapping_from(
        (DoubleLimb::from(x) * DoubleLimb::from(limbs_invert_limb(y))) >> Limb::WIDTH,
    ));
    // round_bit does not exceed the true quotient floor(x * 2 ^ WIDTH / y), with error at most 2,
    // which means the rational quotient q satisfies round_bit <= q < round_bit + 3, thus the true
    // quotient is round_bit, round_bit + 1 or round_bit + 2.
    let (mut hi, mut lo) = Limb::x_mul_y_to_zz(q, y);
    assert!(hi < x || (hi == x && lo == 0));
    // subtract {hi, lo} from {x, 0}
    (hi, lo) = Limb::xx_sub_yy_to_zz(x, 0, hi, lo);
    // the remainder {hi, lo} should be < y.
    if hi != 0 || lo >= y {
        q += 1;
        if lo < y {
            hi.wrapping_sub_assign(1);
        }
        lo.wrapping_sub_assign(y);
    }
    if hi != 0 || lo >= y {
        q += 1;
        if lo < y {
            hi.wrapping_sub_assign(1);
        }
        lo.wrapping_sub_assign(y);
    }
    assert!(hi == 0 && lo < y);
    // now (x - extra * y) * 2 ^ WIDTH = q * y + lo with 0 <= lo < y
    //
    // If !increment_exp, the quotient is q0, the round bit is 1 if l >= y0 / 2, and sticky_bit are
    // the remaining bits from l. If increment_exp, the quotient is HIGH_BIT + (q >> 1), the round
    // bit is the least significant bit of q, and sticky_bit is lo.
    let round_bit;
    let (quotient, sticky_bit) = if increment_exp {
        round_bit = q.odd();
        (HIGH_BIT | (q >> 1), lo)
    } else {
        // If "lo + lo < lo", then there is a carry in lo + lo, thus 2 * lo > y. Otherwise if there
        // is no carry, we check whether 2 * lo >= v0.
        let two_lo = lo << 1;
        round_bit = (two_lo < lo) || (two_lo >= y);
        (
            q,
            if round_bit {
                two_lo.wrapping_sub(y)
            } else {
                lo
            },
        )
    };
    if !round_bit && sticky_bit == 0 {
        return (quotient, increment_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float division"),
        Nearest => {
            if !round_bit || sticky_bit == 0 && quotient.even() {
                (quotient, increment_exp, Less)
            } else {
                (quotient.wrapping_add(1), increment_exp, Greater)
            }
        }
        Floor | Down => (quotient, increment_exp, Less),
        Ceiling | Up => (quotient.wrapping_add(1), increment_exp, Greater),
    }
}

// Given x = x_1 * B + x_0 < y = y_1 * B + y_0 with y normalized (high bit of y_1 set), put in q =
// Q1
// * B + Q0 an approximation of floor(x * B ^ 2 / y), with: B = 2 ^ WIDTH and q <= floor(x * B ^ 2 /
// y) <= q + 21.
//
// This is mpfr_div2_approx from div.c, MPFR 4.3.0, where Q0 and Q1 are returned.
fn div_float_2_approx(x_1: Limb, x_0: Limb, y_1: Limb, y_0: Limb) -> (Limb, Limb) {
    // First compute an approximation of q_1, using a lower approximation of B ^ 2 / (y_1 + 1) - B
    let inv = if y_1 == Limb::MAX {
        0
    } else {
        limbs_invert_limb(y_1 + 1)
    };
    // Now inv <= B ^ 2 / (y_1 + 1) - B.
    let mut q_1 =
        Limb::wrapping_from((DoubleLimb::from(x_1) * DoubleLimb::from(inv)) >> Limb::WIDTH);
    q_1.wrapping_add_assign(x_1);
    // Now q_1 <= x_1 * B / (y_1 + 1) < (x_1 * B + x_0) * B / (y_1 * B + y_0).
    //
    // Compute q_1 * (y_1 * B + y_0) into r_1 : r_0 : yy and subtract from u_1 : x_0 : 0.
    let (mut r_1, mut r_0) = Limb::x_mul_y_to_zz(q_1, y_1);
    let (xx, yy) = Limb::x_mul_y_to_zz(q_1, y_0);
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
    r_1 = x_1.wrapping_sub(r_1);
    let carry;
    (r_0, carry) = x_0.overflowing_sub(r_0);
    if carry {
        r_1.wrapping_sub_assign(1);
    }
    // r_1 : r_0 should be non-negative.
    assert!(!r_1.get_highest_bit());
    // The second quotient limb is approximated by (r_1 * B ^ 2 + r_0 * B) / y_1, and since (B +
    // inv) / B approximates B / y_1, this is in turn approximated by (r * B + r_0) * (B + inv) / B
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

// This is mpfr_div_2 from div.c, MPFR 4.3.0, where Q1 and Q0 are returned.
fn div_float_significands_same_prec_gt_w_lt_2w(
    mut x_0: Limb,
    mut x_1: Limb,
    y_0: Limb,
    y_1: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, bool, Ordering) {
    let shift = TWICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    let increment_exp = x_1 > y_1 || (x_1 == y_1 && x_0 >= y_0);
    if increment_exp {
        (x_1, x_0) = Limb::xx_sub_yy_to_zz(x_1, x_0, y_1, y_0);
    }
    assert!(x_1 < y_1 || (x_1 == y_1 && x_0 < y_0));
    let (mut q_1, mut q_0) = div_float_2_approx(x_1, x_0, y_1, y_0);
    // We know q1 * B + q0 is smaller or equal to the exact quotient, with difference at most 21.
    let mut sticky_bit = if (q_0.wrapping_add(21)) & (mask >> 1) > 21 {
        // The result is not exact when we can round with an approximation.
        1
    } else {
        // We know q_1 : q_0 is a good-enough approximation, so use it!
        //
        // Since we know the difference should be at most 21 * (y_1 : y_0) after the subtraction
        // below, thus at most 21 * 2 ^ 128, it suffices to compute the lower 3 limbs of (q_1 : q_0)
        // * (y_1 : y_0).
        let (mut s_1, mut s_0) = Limb::x_mul_y_to_zz(q_0, y_0);
        let (mut s_2, mut lo) = Limb::x_mul_y_to_zz(q_0, y_1);
        if s_1.overflowing_add_assign(lo) {
            s_2.wrapping_add_assign(1);
        }
        let hi;
        (hi, lo) = Limb::x_mul_y_to_zz(q_1, y_0);
        s_2.wrapping_add_assign(hi);
        if s_1.overflowing_add_assign(lo) {
            s_2.wrapping_add_assign(1);
        }
        s_2.wrapping_add_assign(q_1.wrapping_mul(y_1));
        // Subtract s_2 : s_1 : s_0 from x_0 : 0 : 0, with result in s_2 : s_1 : s_0.
        s_2 = x_0.wrapping_sub(s_2);
        // Now negate s_1 : s_0.
        s_1.wrapping_neg_assign();
        if s_0.overflowing_neg_assign() {
            s_1.wrapping_sub_assign(1);
        }
        // There is a borrow in s_2 when s_0 and s_1 are not both zero.
        if s_1 != 0 || s_0 != 0 {
            s_2.wrapping_sub_assign(1);
        }
        while s_2 > 0 || s_1 > y_1 || (s_1 == y_1 && s_0 >= y_0) {
            // Add 1 to q_1 : q_0.
            if q_0.overflowing_add_assign(1) {
                q_1.wrapping_add_assign(1);
            }
            // Subtract y_1 : y_0 to s_2 : s_1 : s_0
            if (s_1 < y_1) || (s_1 == y_1 && s_0 < y_0) {
                s_2.wrapping_sub_assign(1);
            }
            (s_1, s_0) = Limb::xx_sub_yy_to_zz(s_1, s_0, y_1, y_0);
        }
        s_1 | s_0
    };
    if increment_exp {
        sticky_bit |= q_0 & 1;
        q_0 = (q_1 << WIDTH_M1) | (q_0 >> 1);
        q_1 = HIGH_BIT | (q_1 >> 1);
    }
    let round_bit = q_0 & (shift_bit >> 1);
    sticky_bit |= (q_0 & mask) ^ round_bit;
    let mut z_1 = q_1;
    let mut z_0 = q_0 & !mask;
    if round_bit == 0 && sticky_bit == 0 {
        return (z_0, z_1, increment_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float division"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && (z_0 & shift_bit) == 0) {
                (z_0, z_1, increment_exp, Less)
            } else if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, increment_exp, Greater)
            }
        }
        Floor | Down => (z_0, z_1, increment_exp, Less),
        Ceiling | Up => {
            if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, increment_exp, Greater)
            }
        }
    }
}

// This is equivalent to `mpn_divrem_1` from `mpn/generic/divrem_1.c`, GMP 6.2.1, assuming the
// highest bit of `d` is set.
fn limbs_div_limb_to_out_mod_with_fraction(
    out: &mut [Limb],
    fraction_len: usize,
    ns: &[Limb],
    d: Limb,
) -> Limb {
    assert_ne!(d, 0);
    let len = ns.len().checked_add(fraction_len).unwrap();
    assert_ne!(len, 0);
    let out = &mut out[..len];
    assert!(d.get_highest_bit());
    // High quotient limb is 0 or 1, skip a divide step.
    let (r, ns_init) = ns.split_last().unwrap();
    let mut r = *r;
    let (out_last, out_init) = out.split_last_mut().unwrap();
    let adjust = r >= d;
    if adjust {
        r -= d;
    }
    *out_last = Limb::from(adjust);
    // Multiply-by-inverse, divisor already normalized.
    let d_inv = limbs_invert_limb(d);
    let (out_lo, out_hi) = out_init.split_at_mut(fraction_len);
    for (out_q, &n) in out_hi.iter_mut().zip(ns_init.iter()).rev() {
        (*out_q, r) = div_mod_by_preinversion(r, n, d, d_inv);
    }
    for out_q in out_lo.iter_mut().rev() {
        (*out_q, r) = div_mod_by_preinversion(r, 0, d, d_inv);
    }
    r
}

// This is equivalent to `mpn_divrem_1` from `mpn/generic/divrem_1.c`, GMP 6.2.1, assuming the
// highest bit of `d` is set.
fn limbs_div_limb_in_place_mod_with_fraction(
    ns: &mut [Limb],
    ns_len: usize,
    fraction_len: usize,
    d: Limb,
) -> Limb {
    assert_ne!(d, 0);
    let len = ns_len.checked_add(fraction_len).unwrap();
    assert_ne!(len, 0);
    let ns = &mut ns[..len];
    assert!(d.get_highest_bit());
    // High quotient limb is 0 or 1, skip a divide step.
    let mut r = ns[ns_len - 1];
    let adjust = r >= d;
    if adjust {
        r -= d;
    }
    ns.copy_within(..ns_len, fraction_len);
    let (ns_high, ns_init) = ns.split_last_mut().unwrap();
    *ns_high = Limb::from(adjust);
    // Multiply-by-inverse, divisor already normalized.
    let d_inv = limbs_invert_limb(d);
    let (ns_lo, ns_hi) = ns_init.split_at_mut(fraction_len);
    for n in ns_hi.iter_mut().rev() {
        (*n, r) = div_mod_by_preinversion(r, *n, d, d_inv);
    }
    for n in ns_lo.iter_mut().rev() {
        (*n, r) = div_mod_by_preinversion(r, 0, d, d_inv);
    }
    r
}

// This is mpfr_div_ui from div_ui.c, MPFR 4.3.0.
fn div_float_significands_long_by_short(
    xs: &[Limb],
    y: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Vec<Limb>, u64, Ordering) {
    let out_len = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let mut out = vec![0; out_len + 1];
    let (exp_offset, o) = div_float_significands_long_by_short_to_out(&mut out, xs, y, prec, rm);
    out.truncate(out_len);
    (out, exp_offset, o)
}

// y cannot be a power of 2.
//
// This is mpfr_div_ui from div_ui.c, MPFR 4.3.0.
fn div_float_significands_long_by_short_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    y: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    let xs_len = xs.len();
    let out_ge_xs = out.len() >= xs_len;
    let diff = out.len().abs_diff(xs_len);
    // We need to store out_len + 1 = xs_len + diff limbs of the quotient.
    let (c, mut sticky_bit) = if out_ge_xs {
        // used the entire dividend
        //
        // X = ({scratch, xs_len + diff} * y + c) * B ^ (-diff} = ({scratch, out_len + 1} * y + c) *
        // B ^ (-dif)
        (limbs_div_limb_to_out_mod_with_fraction(out, diff, xs, y), 0)
    } else {
        // dif < 0, i.e. xs_len > out_len + 1; ignore the (-diff) low limbs from x
        //
        // {xs - dif, out_len + 1} = {scratch, out_len + 1} * y + c, thus X = {xs, -dif} + {xs -
        // diff, out_len + 1} * B ^ (-diff) = {xp, -diff} + ({scratch, out_len + 1} * y + c) * B ^
        // (-dif)
        let (xs_lo, xs_hi) = xs.split_at(diff);
        (
            limbs_div_limb_to_out_mod(out, xs_hi, y),
            Limb::from(!slice_test_zero(xs_lo)),
        )
    };
    // Let r = {xp, -diff} / B ^ (-diff) if diff < 0, r = 0 otherwise; 0 <= r < 1.
    //
    // Then X = ({scratch, out_len + 1} * y + c + r) * B ^ (-dif). x / y = (X / y) * B ^ (-xs_len) *
    // 2 ^ exp = ({scratch, out_len + 1} + (c + r) / y) * B ^ (-(out_len + 1)) * 2 ^ exp where 0 <=
    // (c + r) / y < 1.
    //
    // sticky_bit != 0 iff r != 0
    //
    // If the highest limb of the result is 0 (xs[xs_len - 1] < y), remove it. Otherwise, compute
    // the left shift to be performed to normalize. In the latter case, we discard some low bits
    // computed. They contain information useful for the rounding, hence the updating of middle and
    // inexact.
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let shift_mask = shift_bit - 1;
    let out_head = out[0];
    let out_last = *out.last().unwrap();
    let round_bit;
    let mut exp_offset = if out_last == 0 {
        // round bit is 1 iff (c + r) / u >= 1/2
        if shift == 0 {
            // In this case scratch[out_len] = 0 and shift = 0, the round bit is not in {scratch,
            // out_len + 1}. It is 1 iff 2 * (c + r) - y >= 0. This means that in some cases, we
            // should look at the most significant bit of r.
            if c >= y - c {
                // i.e. 2 * c >= y: round bit is always 1
                round_bit = 1;
                // The sticky bit is 1 unless 2 * c - y = 0 and r = 0.
                sticky_bit |= (c << 1).wrapping_sub(y);
            } else {
                // 2 * c < y
                //
                // The round bit is 1 iff r >= 1 / 2 and 2 * (c + 1 / 2) = y.
                let xdm1 = if diff == 0 {
                    0
                } else {
                    xs.get(diff - 1).copied().unwrap_or_default()
                };
                round_bit = Limb::from(c == y >> 1 && !out_ge_xs && xdm1.get_highest_bit());
                // If round_bit is set, we need to recompute sticky_bit, since it might have taken
                // into account the most-significant bit of xs[-diff - 1].
                if round_bit != 0 {
                    fail_on_untested_path("div_float_significands_long_by_short, round_bit != 0");
                    sticky_bit = xdm1 << 1; // discard the most significant bit
                    if sticky_bit == 0 && !slice_test_zero(&xs[..diff]) {
                        sticky_bit = 1;
                    }
                } else {
                    sticky_bit |= c;
                }
            }
        } else {
            // round bit is in scratch[0]
            round_bit = out_head & (shift_bit >> 1);
            sticky_bit |= (out_head & (shift_mask >> 1)) | c;
        }
        0
    } else {
        // scratch[out_len] != 0
        assert_ne!(out_last, 0);
        let shift_2 = LeadingZeros::leading_zeros(out_last);
        let comp_shift_2 = Limb::WIDTH - shift_2;
        assert!(y >= 2); // see special cases at the beginning
        assert_ne!(shift_2, 0); // since y >= 2, shift left to normalize
        let old_head_1 = out_head >> comp_shift_2;
        let old_head_2 = out_head << shift_2;
        out.copy_within(1.., 0);
        limbs_slice_shl_in_place(out, shift_2);
        let out_head = out.first_mut().unwrap();
        *out_head |= old_head_1;
        // now Y is the approximate quotient, w is the next limb.
        let w = old_head_2;
        if shift == 0 {
            // round bit is upper bit from w
            round_bit = w & HIGH_BIT;
            sticky_bit |= (w - round_bit) | c;
        } else {
            round_bit = *out_head & (shift_bit >> 1);
            sticky_bit |= (*out_head & (shift_mask >> 1)) | w | c;
        }
        comp_shift_2
    };
    // Clear the lowest `shift` bits
    out[0] &= !shift_mask;
    if round_bit == 0 && sticky_bit == 0 {
        return (exp_offset, Equal);
    }
    let (_, out) = out.split_last_mut().unwrap();
    match rm {
        Exact => panic!("Inexact float division"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && (out[0] & shift_bit) == 0) {
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

// y cannot be a power of 2.
//
// This is mpfr_div_ui from div_ui.c, MPFR 4.3.0.
fn div_float_significands_long_by_short_in_place(
    xs: &mut Vec<Limb>,
    y: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    let xs_len = xs.len();
    let out_len = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let out_ge_xs = out_len + 1 >= xs_len;
    let diff = (out_len + 1).abs_diff(xs_len);
    let x_lo_nonzero = diff < xs_len && !slice_test_zero(&xs[..diff]);
    let xdm1 = if diff == 0 {
        0
    } else {
        xs.get(diff - 1).copied().unwrap_or_default()
    };
    // We need to store out_len + 1 = xs_len + diff limbs of the quotient.
    let (c, mut sticky_bit) = if out_ge_xs {
        // used the entire dividend
        //
        // X = ({scratch, xs_len + diff} * y + c) * B ^ (-diff} = ({scratch, out_len + 1} * y + c) *
        // B ^ (-dif)
        xs.resize(out_len + 1, 0);
        (
            limbs_div_limb_in_place_mod_with_fraction(xs, xs_len, diff, y),
            0,
        )
    } else {
        // dif < 0, i.e. xs_len > out_len + 1; ignore the (-diff) low limbs from x
        //
        // {xs - dif, out_len + 1} = {scratch, out_len + 1} * y + c, thus X = {xs, -dif} + {xs -
        // diff, out_len + 1} * B ^ (-diff) = {xp, -diff} + ({scratch, out_len + 1} * y + c) * B ^
        // (-dif)
        let p = (
            limbs_div_limb_in_place_mod(&mut xs[diff..], y),
            Limb::from(x_lo_nonzero),
        );
        xs.drain(..diff);
        p
    };
    // Let r = {xp, -diff} / B ^ (-diff) if diff < 0, r = 0 otherwise; 0 <= r < 1.
    //
    // Then X = ({scratch, out_len + 1} * y + c + r) * B ^ (-dif). x / y = (X / y) * B ^ (-xs_len) *
    // 2 ^ exp = ({scratch, out_len + 1} + (c + r) / y) * B ^ (-(out_len + 1)) * 2 ^ exp where 0 <=
    // (c + r) / y < 1.
    //
    // sticky_bit != 0 iff r != 0
    //
    // If the highest limb of the result is 0 (xs[xs_len - 1] < y), remove it. Otherwise, compute
    // the left shift to be performed to normalize. In the latter case, we discard some low bits
    // computed. They contain information useful for the rounding, hence the updating of middle and
    // inexact.
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let shift_mask = shift_bit - 1;
    let xs_head = xs[0];
    let xs_last = *xs.last().unwrap();
    let round_bit;
    let mut exp_offset = if xs_last == 0 {
        // round bit is 1 iff (c + r) / u >= 1/2
        if shift == 0 {
            // In this case scratch[out_len] = 0 and shift = 0, the round bit is not in {scratch,
            // out_len + 1}. It is 1 iff 2 * (c + r) - y >= 0. This means that in some cases, we
            // should look at the most significant bit of r.
            if c >= y - c {
                // i.e. 2 * c >= y: round bit is always 1
                round_bit = 1;
                // The sticky bit is 1 unless 2 * c - y = 0 and r = 0.
                sticky_bit |= (c << 1).wrapping_sub(y);
            } else {
                // 2 * c < y
                //
                // The round bit is 1 iff r >= 1 / 2 and 2 * (c + 1 / 2) = y.
                round_bit = Limb::from(c == y >> 1 && !out_ge_xs && xdm1.get_highest_bit());
                // If round_bit is set, we need to recompute sticky_bit, since it might have taken
                // into account the most-significant bit of xs[-diff - 1].
                if round_bit != 0 {
                    fail_on_untested_path("div_float_significands_long_by_short, round_bit != 0");
                    sticky_bit = xdm1 << 1; // discard the most significant bit
                    if sticky_bit == 0 && x_lo_nonzero {
                        sticky_bit = 1;
                    }
                } else {
                    sticky_bit |= c;
                }
            }
        } else {
            // round bit is in scratch[0]
            round_bit = xs_head & (shift_bit >> 1);
            sticky_bit |= (xs_head & (shift_mask >> 1)) | c;
        }
        0
    } else {
        // scratch[out_len] != 0
        assert_ne!(xs_last, 0);
        let shift_2 = LeadingZeros::leading_zeros(xs_last);
        let comp_shift_2 = Limb::WIDTH - shift_2;
        assert!(y >= 2); // see special cases at the beginning
        assert_ne!(shift_2, 0); // since y >= 2, shift left to normalize
        let old_head_1 = xs_head >> comp_shift_2;
        let old_head_2 = xs_head << shift_2;
        xs.copy_within(1.., 0);
        limbs_slice_shl_in_place(xs, shift_2);
        let xs_head = xs.first_mut().unwrap();
        *xs_head |= old_head_1;
        // now Y is the approximate quotient, w is the next limb.
        let w = old_head_2;
        if shift == 0 {
            // round bit is upper bit from w
            round_bit = w & HIGH_BIT;
            sticky_bit |= (w - round_bit) | c;
        } else {
            round_bit = *xs_head & (shift_bit >> 1);
            sticky_bit |= (*xs_head & (shift_mask >> 1)) | w | c;
        }
        comp_shift_2
    };
    // Clear the lowest `shift` bits
    xs[0] &= !shift_mask;
    xs.truncate(out_len);
    if round_bit == 0 && sticky_bit == 0 {
        return (exp_offset, Equal);
    }
    match rm {
        Exact => panic!("Inexact float division"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && (xs[0] & shift_bit) == 0) {
                (exp_offset, Less)
            } else {
                if limbs_slice_add_limb_in_place(xs, shift_bit) {
                    exp_offset += 1;
                    *xs.last_mut().unwrap() = HIGH_BIT;
                }
                (exp_offset, Greater)
            }
        }
        Floor | Down => (exp_offset, Less),
        Ceiling | Up => {
            if limbs_slice_add_limb_in_place(xs, shift_bit) {
                exp_offset += 1;
                *xs.last_mut().unwrap() = HIGH_BIT;
            }
            (exp_offset, Greater)
        }
    }
}

pub(crate) const MPFR_DIVHIGH_TAB: [i8; 17] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// This is `mpn_divrem` from `mpn/divrem.c`, GMP 6.2.1, where qxn is 0.
pub(crate) fn limbs_div_helper(qs: &mut [Limb], ns: &mut [Limb], ds: &[Limb]) -> bool {
    let ns_len = ns.len();
    let ds_len = ds.len();
    assert!(ns_len >= ds_len);
    assert_ne!(ds_len, 0);
    assert!(ds[ds_len - 1].get_highest_bit());
    if ds_len == 2 {
        limbs_div_mod_by_two_limb_normalized(qs, ns, ds)
    } else {
        let qs_len = ns_len - ds_len + 1;
        let mut scratch = vec![0; qs_len];
        if ds_len == 1 {
            ns[0] = limbs_div_limb_to_out_mod(&mut scratch, ns, ds[0]);
        } else {
            limbs_div_mod_qs_to_out_rs_to_ns(&mut scratch, ns, ds);
        }
        let (scratch_last, scratch_init) = scratch.split_last().unwrap();
        qs[..qs_len - 1].copy_from_slice(scratch_init);
        assert!(*scratch_last < 2);
        *scratch_last != 0
    }
}

pub(crate) fn limbs_float_div_high_scratch_len(ds_len: usize) -> usize {
    let k = if ds_len < MPFR_DIVHIGH_TAB.len() {
        usize::exact_from(MPFR_DIVHIGH_TAB[ds_len])
    } else {
        (ds_len / 3) << 1
    };
    if k == 0 {
        0
    } else {
        let l = ds_len - k;
        (l << 1) + limbs_float_mul_high_same_length_scratch_len(l)
    }
}

// Put in {qs, len} an approximation of N = {ns, 2 * len} divided by D = {ds, len},
//
// with the most significant limb of the quotient as return value (0 or 1). Assumes the most
// significant bit of D is set. Clobbers N.
//
// This implements the ShortDiv algorithm from Short Division of Long Integers, David Harvey and
// Paul Zimmermann, Proceedings of the 20th Symposium on Computer Arithmetic (ARITH-20), July 25-27,
// 2011, pages 7-14.
//
// Assumes len >= 2 (which should be fulfilled also in the recursive calls).
//
// This is mpfr_divhigh_n from mulders.c, MPFR 4.2.0.
pub(crate) fn limbs_float_div_high(
    qs: &mut [Limb],
    ns: &mut [Limb],
    ds: &[Limb],
    scratch: &mut [Limb],
) -> bool {
    let len = ds.len();
    const LENGTH_VALID: bool = MPFR_DIVHIGH_TAB.len() >= 15;
    assert!(LENGTH_VALID); // so that 2*(n/3) >= (n+4)/2
    assert!(len >= 2);
    let k = if len < MPFR_DIVHIGH_TAB.len() {
        usize::exact_from(MPFR_DIVHIGH_TAB[len])
    } else {
        (len / 3) << 1
    };
    let ns = &mut ns[..len << 1];
    let qs = &mut qs[..len];
    if k == 0 {
        assert!(len > 2, "must implement mpfr_divhigh_n_basecase");
        let inverse = limbs_two_limb_inverse_helper(ds[len - 1], ds[len - 2]);
        return limbs_div_schoolbook_approx(qs, ns, ds, inverse);
    }
    // Check the bounds from. In addition, we forbid k = len - 1, which would give l = 1 in the
    // recursive call. It follows len >= 5.
    assert!((len + 4) >> 1 <= k && k < len - 1);
    let l = len - k;
    let two_l = l << 1;
    let two_len_m_k = len + l;
    // first divide the most significant 2 * k limbs from N by the most significant k limbs of D
    // exact
    let (ds_lo, ds_hi) = ds.split_at(l);
    let mut q_high = limbs_div_helper(&mut qs[l..], &mut ns[two_l..], ds_hi);
    // It remains {ns, 2 * l + k} = {ns, len + l} as remainder
    //
    // now we have to subtract high(Q1) * D0 where Q1 = q_high * B ^ k + {qs + l, k} and D0 = {ds,
    // l}
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(two_l);
    limbs_float_mul_high_same_length(scratch_lo, &qs[k..], ds_lo, scratch_hi);
    // We are only interested in the upper l limbs from {scratch, 2 * l}
    let ns_mid = &mut ns[len..two_len_m_k];
    let mut carry = Limb::from(limbs_sub_same_length_in_place_left(
        ns_mid,
        &scratch_lo[l..],
    ));
    if q_high && limbs_sub_same_length_in_place_left(ns_mid, ds_lo) {
        carry += 1;
    }
    while carry != 0 {
        // Q1 was too large: subtract 1 from Q1 and add D to ns + l
        if limbs_sub_limb_in_place(&mut qs[l..], 1) {
            q_high = false;
        }
        if limbs_slice_add_same_length_in_place_left(&mut ns[l..two_len_m_k], ds) {
            carry -= 1;
        }
    }
    // Now it remains {ns, len + l} to divide by D
    limbs_float_div_high(qs, &mut ns[k..], &ds[k..], scratch)
        && limbs_slice_add_limb_in_place(&mut qs[l..], 1)
        || q_high
}

// Compare {xs, xs_len} and {ys, ys_len} >> extra, aligned by the more significant limbs. Takes into
// account ys[0] for extra = true.
//
// This is mpfr_mpn_cmp_aux from div.c, MPFR 4.2.0.
pub fn cmp_helper(xs: &[Limb], ys: &[Limb], extra: bool) -> Ordering {
    let xs_len = xs.len();
    let mut cmp = Equal;
    if extra {
        let ys_len = ys.len() - 1;
        if xs_len >= ys_len {
            let (xs_lo, xs_hi) = xs.split_at(xs_len - ys_len);
            for (i, x) in xs_hi.iter().enumerate().rev() {
                let y = (ys[i + 1] << WIDTH_M1) | (ys[i] >> 1);
                cmp = x.cmp(&y);
                if cmp != Equal {
                    break;
                }
            }
            let mut y = ys[0] << WIDTH_M1;
            for x in xs_lo.iter().rev() {
                if cmp != Equal {
                    break;
                }
                cmp = x.cmp(&y);
                y = 0; // ensure we consider ys[0] & 1 only once
            }
            if cmp == Equal && y != 0 {
                cmp = Less;
            }
        } else {
            let k = ys_len - xs_len;
            let ys_hi = &ys[k..];
            for (i, x) in xs.iter().enumerate().rev() {
                let y = (ys_hi[i + 1] << WIDTH_M1) | (ys_hi[i] >> 1);
                cmp = x.cmp(&y);
                if cmp != Equal {
                    break;
                }
            }
            for i in (0..k).rev() {
                if cmp != Equal {
                    break;
                }
                let y = (ys[i + 1] << WIDTH_M1) | (ys[i] >> 1);
                cmp = if y != 0 { Less } else { Equal };
            }
            if cmp == Equal && extra && ys[0].odd() {
                cmp = Less;
            }
        }
    } else {
        let ys_len = ys.len();
        if xs_len >= ys_len {
            let (xs_lo, xs_hi) = xs.split_at(xs_len - ys_len);
            cmp = limbs_cmp_same_length(xs_hi, ys);
            if cmp == Equal && !slice_test_zero(xs_lo) {
                cmp = Greater;
            }
        } else {
            let (ys_lo, ys_hi) = ys.split_at(ys_len - xs_len);
            cmp = limbs_cmp_same_length(xs, ys_hi);
            if cmp == Equal && !slice_test_zero(ys_lo) {
                cmp = Less;
            }
        }
    }
    cmp
}

// xs <- xs - ys >> extra - carry, with carry = 0 or 1. Return borrow out.
//
// This is mpfr_mpn_sub_aux from div.c, MPFR 4.2.0.
fn sub_helper(xs: &mut [Limb], ys: &[Limb], mut carry: bool, extra: bool) -> bool {
    if extra {
        for (i, x) in xs.iter_mut().enumerate() {
            let y = (ys[i + 1] << WIDTH_M1) | (ys[i] >> 1);
            let mut diff = x.wrapping_sub(y);
            if carry {
                diff.wrapping_sub_assign(1);
            }
            carry = *x < y || carry && diff == Limb::MAX;
            *x = diff;
        }
    } else {
        for (x, &y) in xs.iter_mut().zip(ys.iter()) {
            let mut diff = x.wrapping_sub(y);
            if carry {
                diff.wrapping_sub_assign(1);
            }
            carry = *x < y || carry && diff == Limb::MAX;
            *x = diff;
        }
    }
    carry
}

#[inline]
fn div_float_significands_general(
    xs: &[Limb],
    ys: &mut [Limb],
    prec: u64,
    rm: RoundingMode,
) -> (Vec<Limb>, u64, Ordering) {
    let mut out = vec![0; usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
    let (exp_offset, o) = div_float_significands_general_to_out(&mut out, xs, ys, prec, rm);
    (out, exp_offset, o)
}

pub(crate) const MPFR_DIV_THRESHOLD: usize = 25;

#[derive(Eq, PartialEq, Clone, Copy)]
pub(crate) enum Cleanup {
    None,
    TruncateCheckQHigh,
    Sub1Ulp,
    Sub2Ulp,
}

// TODO special case qs == ds
//
// This is mpfr_div from div.c, MPFR 4.2.0, skipping over various special cases
fn div_float_significands_general_to_out(
    qs: &mut [Limb],
    ns: &[Limb],
    ds: &mut [Limb],
    prec: u64,
    rm: RoundingMode,
) -> (u64, Ordering) {
    let ns_len = ns.len();
    let ds_len = ds.len();
    let qs_len = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let qs = &mut qs[..qs_len];
    // Determine if an extra bit comes from the division, i.e. if the significand of X (as a
    // fraction in [1/2, 1) ) is larger than that of Y
    let ns_last = *ns.last().unwrap();
    let ds_last = *ds.last().unwrap();
    let extra_bit = if ns_last == ds_last {
        // most significant limbs are equal, must look at further limbs
        if let Some((n, d)) = ns.iter().rev().zip(ds.iter().rev()).find(|&(n, d)| n != d) {
            n > d
        } else if ns_len >= ds_len {
            // no more divisor limb
            true
        } else {
            // k = 0: no more dividend limb
            slice_test_zero(&ds[..ds_len - ns_len])
        }
    } else {
        ns_last > ds_last
    };
    let mut exp_offset = u64::from(extra_bit);
    // shift is the number of zero bits in the low limb of the quotient
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let mut shift_bit = Limb::power_of_2(shift);
    let shift_mask = shift_bit - 1;
    let mut ys_vec;
    let mut ys: &mut [Limb];
    let mut inex;
    // We first try Mulders' short division (for large operands)
    if qs_len >= MPFR_DIV_THRESHOLD && ds_len >= MPFR_DIV_THRESHOLD {
        // We will perform a short (2 * n) / n division
        let n = qs_len + 1;
        let two_n = n << 1;
        // since Mulders' short division clobbers the dividend, we have to copy it
        let mut xs = vec![0; two_n];
        if ns_len >= two_n {
            // truncate the dividend
            xs[..two_n].copy_from_slice(&ns[ns_len - two_n..]);
        } else {
            // zero-pad the dividend
            xs[two_n - ns_len..].copy_from_slice(ns);
        }
        if ds_len >= n {
            // truncate the divisor
            ys = &mut ds[ds_len - n..];
        } else {
            // zero-pad the divisor
            ys_vec = vec![0; n];
            ys = &mut ys_vec;
            ys[n - ds_len..].copy_from_slice(ds);
        }
        // since n = qs_len + 1, we have n >= 2 here
        let mut scratch = vec![0; n + limbs_float_div_high_scratch_len(n)];
        let (qs_2, scratch) = scratch.split_at_mut(n);
        let q_high = limbs_float_div_high(qs_2, &mut xs, &ys[..n], scratch);
        // in all cases, the error is at most (2 * n + 2) ulps on q_high * B ^ n + {qs_2, n}.
        let p = i32::exact_from(
            i64::exact_from(n << Limb::LOG_WIDTH)
                - i64::exact_from((two_n + 2).ceiling_log_base_2()),
        );
        // If rm == Nearest, we need to be able to round with a directed rounding and one more bit.
        if q_high {
            let qs_2_lo = &mut qs_2[..n];
            limbs_slice_shr_in_place(qs_2_lo, 1);
            *qs_2_lo.last_mut().unwrap() |= HIGH_BIT;
            if round_helper_2(qs_2_lo, p, prec + u64::from(rm == Nearest)) {
                // We can round correctly whatever the rounding mode
                qs.copy_from_slice(&qs_2[1..=qs_len]);
                qs[0] &= !shift_mask; // put to zero low `shift` bits
                return if rm == Exact {
                    panic!("Inexact float division");
                } else if rm == Nearest {
                    // round to nearest
                    //
                    // We know we can round, thus we are never in the even rule case:
                    // - if the round bit is 0, we truncate
                    // - if the round bit is 1, we add 1
                    let round_bit = if shift == 0 {
                        qs_2[0].get_highest_bit()
                    } else {
                        (qs_2[1] >> (shift - 1)).odd()
                    };
                    if round_bit {
                        if limbs_slice_add_limb_in_place(qs, shift_bit) {
                            exp_offset += 1;
                            // else exponent is now incorrect, but one will still get an overflow
                            qs[qs_len - 1] = HIGH_BIT;
                        }
                        (exp_offset, Greater)
                    } else {
                        (exp_offset, Less)
                    }
                } else if rm == Up || rm == Ceiling {
                    if limbs_slice_add_limb_in_place(qs, shift_bit) {
                        exp_offset += 1;
                        // else exponent is now incorrect, but one will still get an overflow
                        *qs.last_mut().unwrap() = HIGH_BIT;
                    }
                    (exp_offset, Greater)
                } else {
                    (exp_offset, Less)
                };
            }
        }
    }
    // Mulders' short division failed: we revert to integer division
    let mut qs_2_vec = vec![];
    let mut qs_2: &mut [Limb] = if rm == Nearest && shift == 0 {
        // We compute the quotient with one more limb, in order to get the round bit in the
        // quotient, and the remainder only contains sticky bits. Need to allocate memory for the
        // quotient
        qs_2_vec = vec![0; qs_len + 1];
        &mut qs_2_vec
    } else {
        qs // directly put the quotient in the destination
    };
    let qs_2_len = qs_2.len();
    let two_qs_2_len = qs_2_len << 1;
    // prepare the dividend
    let mut xs = vec![0; two_qs_2_len];
    let mut sticky_x = false;
    if two_qs_2_len > ns_len {
        // use the full dividend
        let (xs_lo, xs_hi) = xs.split_at_mut(two_qs_2_len - ns_len);
        if extra_bit {
            *xs_lo.last_mut().unwrap() = limbs_shr_to_out(xs_hi, ns, 1);
        } else {
            xs_hi.copy_from_slice(ns);
        }
    } else {
        // truncate the dividend
        let (ns_lo, ns_hi) = ns.split_at(ns_len - two_qs_2_len);
        let ns_hi = &ns_hi[..two_qs_2_len];
        if extra_bit {
            sticky_x = limbs_shr_to_out(&mut xs, ns_hi, 1) != 0;
        } else {
            xs.copy_from_slice(ns_hi);
        }
        sticky_x = sticky_x || !slice_test_zero(ns_lo);
    }
    let mut low_x = sticky_x;
    let mut k;
    // Now sticky_x is non-zero iff the truncated part of x is non-zero
    let mut sticky_y = false;
    // Prepare the divisor
    k = if ds_len >= qs_2_len {
        k = ds_len - qs_2_len;
        sticky_y = sticky_y || !slice_test_zero(&ds[..k]);
        ys = &mut ds[k..]; // avoid copying the divisor
        0
    } else {
        // ds_len < qs_2_len: small divisor case
        ys = ds;
        qs_2_len - ds_len
    };
    // Here we perform the real division of {xs + k, two_qs_2_len - k} by {ys, qs_2_len - k} In the
    // general case (ns_len > 2 * qs_2_len and ds_len > qs_2_len), we have:
    // ```
    //   ______________________________________
    //  |                          |           |   x1 has 2 * qs_2_len limbs
    //  |             x1           |     x0    |   x0 has ns_len - 2 * qs_2_len limbs
    //  |__________________________|___________|

    //                  ____________________
    //                 |           |        |      y1 has qs_2_len limbs
    //                 |    y1     |    y0  |      y0 has ds_len - qs_2_len limbs
    //                 |___________|________|
    // ```
    //
    //  We divide x1 by y1, with quotient in q_high + {qs_2, qs_2_len} and remainder (denoted r
    //  below) stored in place of the low qs_2_len limbs of x1.
    //
    // If Mulders' short division failed, we revert to division with remainder
    let mut q_high = limbs_div_helper(qs_2, &mut xs[k..], &ys[..qs_2_len - k]);
    // let x1 be the upper part of x, and y1 the upper part of y (with sticky_x and sticky_y
    // representing the lower parts), then the quotient of x1 by y1 is now in {qs_2, qs_2_len}, with
    // possible carry in q_high, and the remainder in {xs + k, qs_2_len - k}.
    //
    // Warning: q_high may be 1 if x1 == y1, but x < y.
    k = qs_2_len;
    sticky_x = sticky_x || !slice_test_zero(&xs[..k]);
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
    // sticky_3 contains the truncated bits from the quotient, including the round bit, and 1 <=
    // shift_2 <= WIDTH is the number of bits in sticky_3
    inex = if sticky_bit != 0 || sticky_3 != 0 {
        Greater
    } else {
        Equal
    };
    // to round, we distinguish two cases:
    // - ds_len <= qs_2_len: we used the full divisor
    // - ds_len > qs_2_len: the divisor was truncated
    let mut round_bit = 0;
    let mut cleanup = Cleanup::None;
    if ds_len <= qs_2_len {
        // use the full divisor
        sticky_bit = if rm == Nearest {
            round_bit = sticky_3 & Limb::power_of_2(shift_2 - 1);
            (sticky_3 ^ round_bit) | Limb::from(sticky_x)
        } else if rm == Floor || rm == Down || inex == Equal {
            Limb::from(inex != Equal)
        } else if rm == Exact {
            panic!("Inexact float division");
        } else {
            1
        };
    } else {
        // ds_len > qs_2_len: need to truncate the divisor
        if inex == Equal {
            return (exp_offset, Equal);
        }
        // We know the estimated quotient is an upper bound of the exact quotient (with rounding
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
            qs_2[0] ^= sticky_3_orig; // restore original quotient
            let ds_lo = &ds[..k];
            limbs_mul_to_out(sp, qs_2, ds_lo, scratch);
            let q_high_2 = if q_high {
                limbs_slice_add_same_length_in_place_left(&mut sp[qs_2_len..], ds_lo)
            } else {
                false
            };
            qs_2[0] ^= sticky_3_orig;
            // restore truncated quotient
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
                cmp_s_r = if ns_len >= two_qs_2_len {
                    cmp_helper(
                        sp_lo,
                        &ns[..ns_len - two_qs_2_len + usize::from(extra_bit)],
                        extra_bit,
                    )
                } else if slice_test_zero(sp_lo) {
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
                // quotient is in [q1, q1+1)
                sticky_bit = if cmp_s_r == Equal { sticky_3 } else { 1 };
            } else {
                // cmp_s_r > 0, quotient is < q1: to determine if it is in [q1 - 2, q1 - 1] or in
                // [q1 - 1, q1], we need to subtract the low part u0 of the dividend from q*v0
                let mut carry = false;
                // subtract u0 >> extra_bit if non-zero
                if q_high_2 {
                    // whatever the value of {ns, m + k}, it will be smaller than q_high_2 + {sp, k}
                    cmp_s_r = Greater;
                } else {
                    if low_x {
                        let l = ns_len - two_qs_2_len; // number of limbs in u0
                        let m = l.saturating_sub(k);
                        carry = extra_bit && ns[m].odd();
                        if l >= k {
                            // u0 has at least as many limbs than s: first look if {ns, m} is not
                            // zero, and compare {sp, k} and {ns + m, k}
                            if !carry {
                                carry = !slice_test_zero(&ns[..m]);
                            }
                            low_x = carry;
                            carry = sub_helper(
                                sp_lo,
                                &ns[m..m + k + usize::from(extra_bit)],
                                carry,
                                extra_bit,
                            );
                        } else {
                            // l < k: s has more limbs than u0
                            low_x = false;
                            let kml = k - l;
                            if carry {
                                carry = limbs_sub_limb_in_place(&mut sp_lo[kml - 1..kml], HIGH_BIT);
                            }
                            carry = sub_helper(
                                &mut sp_lo[kml..],
                                &ns[..l + usize::from(extra_bit)],
                                carry,
                                extra_bit,
                            );
                        }
                    }
                    if carry {
                        limbs_sub_limb_in_place(sp_hi, 1);
                    }
                    // subtract r
                    limbs_sub_same_length_in_place_left(sp_hi, &xs[..qs_2_len]);
                    // now compare {sp, ds_len} to y
                    cmp_s_r = limbs_cmp_same_length(sp, ds);
                    if cmp_s_r == Equal && low_x {
                        cmp_s_r = Greater;
                        // since in fact we subtracted less than 1
                    }
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
                            panic!("Inexact float division");
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
                        panic!("Inexact float division");
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
            // quotient is in [q1, q1 + 1), round_bit is the round_bit (0 for directed rounding)
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
                panic!("Inexact float division");
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
