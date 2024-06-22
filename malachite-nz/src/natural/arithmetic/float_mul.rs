// Copyright © 2024 Mikhail Hogrefe
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

use crate::natural::arithmetic::add::{
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use crate::natural::arithmetic::float_extras::{limbs_float_can_round, round_helper_raw};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out_basecase, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb, MUL_FFT_THRESHOLD};
use alloc::vec::Vec;
use core::cmp::{
    max, min,
    Ordering::{self, *},
};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, OverflowingAddAssign, Parity, PowerOf2, ShrRound, Sign, WrappingAddAssign,
    XMulYToZZ, XXAddYYToZZ,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_leading_zeros;

// This is mpfr_mul from mul.c, MPFR 4.2.0.
pub fn mul_float_significands_in_place(
    x: &mut Natural,
    x_prec: u64,
    y: &mut Natural,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        if let Some((decrement_exp, o)) =
            mul_float_significands_in_place_same_prec(x, y, out_prec, rm)
        {
            return (-i32::from(decrement_exp), o);
        }
    }
    let (product, exp_offset, o) = if x_prec >= y_prec {
        match (&mut *x, &mut *y) {
            (Natural(Small(x)), Natural(Small(y))) => {
                mul_float_significands_general(&[*x], x_prec, &[*y], y_prec, out_prec, rm)
            }
            (Natural(Large(xs)), Natural(Small(y))) => {
                mul_float_significands_general(xs, x_prec, &[*y], y_prec, out_prec, rm)
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                mul_float_significands_general(xs, x_prec, ys, y_prec, out_prec, rm)
            }
            _ => unreachable!(),
        }
    } else {
        match (&mut *x, &mut *y) {
            (Natural(Small(x)), Natural(Small(y))) => {
                mul_float_significands_general(&[*y], y_prec, &[*x], x_prec, out_prec, rm)
            }
            (Natural(Small(x)), Natural(Large(ys))) => {
                mul_float_significands_general(ys, y_prec, &[*x], x_prec, out_prec, rm)
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                mul_float_significands_general(ys, y_prec, xs, x_prec, out_prec, rm)
            }
            _ => unreachable!(),
        }
    };
    *x = product;
    (exp_offset, o)
}

// This is mpfr_mul from mul.c, MPFR 4.2.0.
pub fn mul_float_significands_in_place_ref(
    x: &mut Natural,
    x_prec: u64,
    y: &Natural,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        if let Some((decrement_exp, o)) =
            mul_float_significands_in_place_same_prec_ref(x, y, out_prec, rm)
        {
            return (-i32::from(decrement_exp), o);
        }
    }
    let (product, exp_offset, o) = if x_prec >= y_prec {
        match (&mut *x, y) {
            (Natural(Small(x)), Natural(Small(y))) => {
                mul_float_significands_general(&[*x], x_prec, &[*y], y_prec, out_prec, rm)
            }
            (Natural(Large(xs)), Natural(Small(y))) => {
                mul_float_significands_general(xs, x_prec, &[*y], y_prec, out_prec, rm)
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                mul_float_significands_general(xs, x_prec, ys, y_prec, out_prec, rm)
            }
            _ => unreachable!(),
        }
    } else {
        match (&mut *x, y) {
            (Natural(Small(x)), Natural(Small(y))) => {
                mul_float_significands_general(&[*y], y_prec, &[*x], x_prec, out_prec, rm)
            }
            (Natural(Small(x)), Natural(Large(ys))) => {
                mul_float_significands_general(ys, y_prec, &[*x], x_prec, out_prec, rm)
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                mul_float_significands_general(ys, y_prec, xs, x_prec, out_prec, rm)
            }
            _ => unreachable!(),
        }
    };
    *x = product;
    (exp_offset, o)
}

// This is mpfr_mul from mul.c, MPFR 4.2.0.
pub fn mul_float_significands_ref_ref(
    x: &Natural,
    x_prec: u64,
    y: &Natural,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            mul_float_significands_ref_ref_helper(&[*x], x_prec, &[*y], y_prec, out_prec, rm)
        }
        (Natural(Large(xs)), Natural(Small(y))) => {
            mul_float_significands_ref_ref_helper(xs, x_prec, &[*y], y_prec, out_prec, rm)
        }
        (Natural(Small(x)), Natural(Large(ys))) => {
            mul_float_significands_ref_ref_helper(&[*x], x_prec, ys, y_prec, out_prec, rm)
        }
        (Natural(Large(xs)), Natural(Large(ys))) => {
            mul_float_significands_ref_ref_helper(xs, x_prec, ys, y_prec, out_prec, rm)
        }
    }
}

fn mul_float_significands_ref_ref_helper(
    xs: &[Limb],
    x_prec: u64,
    ys: &[Limb],
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        if let Some((product, decrement_exp, o)) =
            mul_float_significands_same_prec_ref_ref(xs, ys, out_prec, rm)
        {
            return (product, -i32::from(decrement_exp), o);
        }
    }
    if x_prec >= y_prec {
        mul_float_significands_general(xs, x_prec, ys, y_prec, out_prec, rm)
    } else {
        mul_float_significands_general(ys, y_prec, xs, x_prec, out_prec, rm)
    }
}

fn mul_float_significands_in_place_same_prec(
    x: &mut Natural,
    y: &mut Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(bool, Ordering)> {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (product, decrement_exp, o) = if prec == Limb::WIDTH {
                mul_float_significands_same_prec_w(*x, *y, rm)
            } else {
                mul_float_significands_same_prec_lt_w(*x, *y, prec, rm)
            };
            *x = product;
            Some((decrement_exp, o))
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_mut_slice()) {
            ([x_0, x_1], [y_0, y_1]) if prec != TWICE_WIDTH => {
                let (product_0, product_1, decrement_exp, o) =
                    mul_float_significands_same_prec_gt_w_lt_2w(*x_0, *x_1, *y_0, *y_1, prec, rm);
                *x_0 = product_0;
                *x_1 = product_1;
                Some((decrement_exp, o))
            }
            ([x_0, x_1, x_2], [y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (product_0, product_1, product_2, decrement_exp, o) =
                    mul_float_significands_same_prec_gt_2w_lt_3w(
                        *x_0, *x_1, *x_2, *y_0, *y_1, *y_2, prec, rm,
                    );
                *x_0 = product_0;
                *x_1 = product_1;
                *x_2 = product_2;
                Some((decrement_exp, o))
            }
            _ => None,
        },
        _ => unreachable!(),
    }
}

fn mul_float_significands_in_place_same_prec_ref(
    x: &mut Natural,
    y: &Natural,
    prec: u64,
    rm: RoundingMode,
) -> Option<(bool, Ordering)> {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (product, decrement_exp, o) = if prec == Limb::WIDTH {
                mul_float_significands_same_prec_w(*x, *y, rm)
            } else {
                mul_float_significands_same_prec_lt_w(*x, *y, prec, rm)
            };
            *x = product;
            Some((decrement_exp, o))
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_slice()) {
            ([x_0, x_1], [y_0, y_1]) if prec != TWICE_WIDTH => {
                let (product_0, product_1, decrement_exp, o) =
                    mul_float_significands_same_prec_gt_w_lt_2w(*x_0, *x_1, *y_0, *y_1, prec, rm);
                *x_0 = product_0;
                *x_1 = product_1;
                Some((decrement_exp, o))
            }
            ([x_0, x_1, x_2], [y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (product_0, product_1, product_2, decrement_exp, o) =
                    mul_float_significands_same_prec_gt_2w_lt_3w(
                        *x_0, *x_1, *x_2, *y_0, *y_1, *y_2, prec, rm,
                    );
                *x_0 = product_0;
                *x_1 = product_1;
                *x_2 = product_2;
                Some((decrement_exp, o))
            }
            _ => None,
        },
        _ => unreachable!(),
    }
}

fn mul_float_significands_same_prec_ref_ref(
    xs: &[Limb],
    ys: &[Limb],
    prec: u64,
    rm: RoundingMode,
) -> Option<(Natural, bool, Ordering)> {
    match (xs, ys) {
        ([x], [y]) => {
            let (product, decrement_exp, o) = if prec == Limb::WIDTH {
                mul_float_significands_same_prec_w(*x, *y, rm)
            } else {
                mul_float_significands_same_prec_lt_w(*x, *y, prec, rm)
            };
            Some((Natural(Small(product)), decrement_exp, o))
        }
        ([x_0, x_1], [y_0, y_1]) if prec != TWICE_WIDTH => {
            let (product_0, product_1, decrement_exp, o) =
                mul_float_significands_same_prec_gt_w_lt_2w(*x_0, *x_1, *y_0, *y_1, prec, rm);
            Some((Natural(Large(vec![product_0, product_1])), decrement_exp, o))
        }
        ([x_0, x_1, x_2], [y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
            let (product_0, product_1, product_2, decrement_exp, o) =
                mul_float_significands_same_prec_gt_2w_lt_3w(
                    *x_0, *x_1, *x_2, *y_0, *y_1, *y_2, prec, rm,
                );
            Some((
                Natural(Large(vec![product_0, product_1, product_2])),
                decrement_exp,
                o,
            ))
        }
        _ => None,
    }
}

const WIDTH_M1: u64 = Limb::WIDTH - 1;
const HIGH_BIT: Limb = 1 << WIDTH_M1;
const COMP_HIGH_BIT: Limb = !HIGH_BIT;

// This is mpfr_mul_1 from mul.c, MPFR 4.2.0.
fn mul_float_significands_same_prec_lt_w(
    x: Limb,
    y: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, bool, Ordering) {
    let shift = Limb::WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    let (mut z, mut sticky_bit) = Limb::x_mul_y_to_zz(x, y);
    let decrement_exp = !z.get_highest_bit();
    if decrement_exp {
        z <<= 1;
        z |= sticky_bit >> WIDTH_M1;
        sticky_bit <<= 1;
    }
    let round_bit = z & (shift_bit >> 1);
    sticky_bit |= (z & mask) ^ round_bit;
    let mut product = z & !mask;
    if round_bit == 0 && sticky_bit == 0 {
        return (z, decrement_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float multiplication"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && (product & shift_bit) == 0) {
                (product, decrement_exp, Less)
            } else if product.overflowing_add_assign(shift_bit) {
                (HIGH_BIT, false, Greater)
            } else {
                (product, decrement_exp, Greater)
            }
        }
        Floor | Down => (product, decrement_exp, Less),
        Ceiling | Up => {
            if product.overflowing_add_assign(shift_bit) {
                (HIGH_BIT, false, Greater)
            } else {
                (product, decrement_exp, Greater)
            }
        }
    }
}

// This is mpfr_mul_1n from mul.c, MPFR 4.2.0.
fn mul_float_significands_same_prec_w(
    x: Limb,
    y: Limb,
    rm: RoundingMode,
) -> (Limb, bool, Ordering) {
    let (mut z, mut sticky_bit) = Limb::x_mul_y_to_zz(x, y);
    let decrement_exp = !z.get_highest_bit();
    if decrement_exp {
        z <<= 1;
        z |= sticky_bit >> WIDTH_M1;
        sticky_bit <<= 1;
    }
    let round_bit = sticky_bit & HIGH_BIT;
    sticky_bit &= COMP_HIGH_BIT;
    let mut product = z;
    if round_bit == 0 && sticky_bit == 0 {
        return (z, decrement_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float multiplication"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && product.even()) {
                (product, decrement_exp, Less)
            } else if product.overflowing_add_assign(1) {
                (HIGH_BIT, false, Greater)
            } else {
                (product, decrement_exp, Greater)
            }
        }
        Floor | Down => (product, decrement_exp, Less),
        Ceiling | Up => {
            if product.overflowing_add_assign(1) {
                (HIGH_BIT, false, Greater)
            } else {
                (product, decrement_exp, Greater)
            }
        }
    }
}

const TWICE_WIDTH: u64 = Limb::WIDTH * 2;
const THRICE_WIDTH: u64 = Limb::WIDTH * 3;

// This is mpfr_mul_2 from mul.c, MPFR 4.2.0.
fn mul_float_significands_same_prec_gt_w_lt_2w(
    x_0: Limb,
    x_1: Limb,
    y_0: Limb,
    y_1: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, bool, Ordering) {
    let shift = TWICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    // we store the 4-limb product in h = z[1], l = z[0], sticky_bit = z[-1], sticky_bit_2 = z[-2]
    let (mut hi, mut lo) = Limb::x_mul_y_to_zz(x_1, y_1);
    let (u, v) = Limb::x_mul_y_to_zz(x_1, y_0);
    if lo.overflowing_add_assign(u) {
        hi += 1;
    }
    let (u, w) = Limb::x_mul_y_to_zz(x_0, y_1);
    if lo.overflowing_add_assign(u) {
        hi.wrapping_add_assign(1);
    }
    // now the full product is {hi, lo, v + w + high(x_0 * y_0), low(x_0 * y_0)}, where the lower
    // part contributes to less than 3 ulps to {hi, lo}.
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
        (sticky_bit, sticky_bit_2) = Limb::x_mul_y_to_zz(x_0, y_0);
        // The full product is {h, l, sticky_bit + v + w, sticky_bit_2}
        if sticky_bit.overflowing_add_assign(v) && lo.overflowing_add_assign(1) {
            hi.wrapping_add_assign(1);
        }
        if sticky_bit.overflowing_add_assign(w) && lo.overflowing_add_assign(1) {
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
        Exact => panic!("Inexact float multiplication"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && (z_0 & shift_bit) == 0) {
                (z_0, z_1, decrement_exp, Less)
            } else if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, decrement_exp, Greater)
            }
        }
        Floor | Down => (z_0, z_1, decrement_exp, Less),
        Ceiling | Up => {
            if z_0.overflowing_add_assign(shift_bit) && z_1.overflowing_add_assign(1) {
                (z_0, HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, decrement_exp, Greater)
            }
        }
    }
}

const LIMB_MASK: DoubleLimb = (1 << Limb::WIDTH) - 1;

// This is mpfr_mul_3 from mul.c, MPFR 4.2.0.
fn mul_float_significands_same_prec_gt_2w_lt_3w(
    x_0: Limb,
    x_1: Limb,
    x_2: Limb,
    y_0: Limb,
    y_1: Limb,
    y_2: Limb,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, Limb, bool, Ordering) {
    let shift = THRICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let mask = shift_bit - 1;
    // we store the upper 3-limb product in z2, z1, z0: x2 * y2, x2 * y1 + x1 * y2, x2 * y0 + x1 *
    // y1 + x0 * y2
    let x_0 = DoubleLimb::from(x_0);
    let x_1 = DoubleLimb::from(x_1);
    let x_2 = DoubleLimb::from(x_2);
    let y_0 = DoubleLimb::from(y_0);
    let y_1 = DoubleLimb::from(y_1);
    let y_2 = DoubleLimb::from(y_2);
    let x_2_y_2 = x_2 * y_2;
    let x_2_y_1 = x_2 * y_1;
    let x_1_y_2 = x_1 * y_2;
    let x_2_y_0 = x_2 * y_0;
    let x_1_y_1 = x_1 * y_1;
    let x_0_y_2 = x_0 * y_2;
    let (mut a2, mut a1) = x_2_y_2.split_in_half();
    let (hi, mut a0) = x_2_y_1.split_in_half();
    if a1.overflowing_add_assign(hi) {
        a2 += 1;
    }
    let (hi, lo) = x_1_y_2.split_in_half();
    if a1.overflowing_add_assign(hi) {
        a2.wrapping_add_assign(1);
    }
    let mut carry = Limb::from(a0.overflowing_add_assign(lo));
    if a0.overflowing_add_assign(Limb::wrapping_from(x_2_y_0 >> Limb::WIDTH)) {
        carry += 1;
    }
    if a0.overflowing_add_assign(Limb::wrapping_from(x_1_y_1 >> Limb::WIDTH)) {
        carry += 1;
    }
    if a0.overflowing_add_assign(Limb::wrapping_from(x_0_y_2 >> Limb::WIDTH)) {
        carry += 1;
    }
    // now propagate carry
    if a1.overflowing_add_assign(carry) {
        a2.wrapping_add_assign(1);
    }
    // Now the approximate product {a2, a1, a0} has an error of less than 5 ulps (3 ulps for the
    // ignored low limbs of x_2 * y_0 + x_1 * y_1 + x_0 * y2, plus 2 ulps for the ignored x_1 * y_0
    // + x_0 * y_1 (plus x_0 * y_0)). Since we might shift by 1 bit, we make sure the low shift - 2
    // bits of a0 are not 0, -1, -2, -3 or -4.
    let (mut sticky_bit, sticky_bit_2) = if a0.wrapping_add(4) & (mask >> 2) > 4 {
        // result cannot be exact in that case
        (1, 1)
    } else {
        let out = x_0 * y_0;
        let p_0 = out & LIMB_MASK;
        let out = x_1 * y_0 + (out >> Limb::WIDTH);
        let mut p_1 = out & LIMB_MASK;
        let out = x_2_y_0 + (out >> Limb::WIDTH);
        let mut p_2 = out & LIMB_MASK;
        let mut p_3 = out >> Limb::WIDTH;
        let out = p_1 + x_0 * y_1;
        p_1 = out & LIMB_MASK;
        let out = p_2 + x_1_y_1 + (out >> Limb::WIDTH);
        p_2 = out & LIMB_MASK;
        let out = p_3 + x_2_y_1 + (out >> Limb::WIDTH);
        p_3 = out & LIMB_MASK;
        let p_4 = out >> Limb::WIDTH;
        let out = p_2 + x_0_y_2;
        p_2 = out & LIMB_MASK;
        let out = p_3 + x_1_y_2 + (out >> Limb::WIDTH);
        p_3 = out & LIMB_MASK;
        let out = p_4 + x_2_y_2 + (out >> Limb::WIDTH);
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
        Exact => panic!("Inexact float multiplication"),
        Nearest => {
            if round_bit == 0 || (sticky_bit == 0 && z_0 & shift_bit == 0) {
                (z_0, z_1, z_2, decrement_exp, Less)
            } else {
                if z_0.overflowing_add_assign(shift_bit) {
                    z_1.wrapping_add_assign(1);
                }
                if z_1 == 0 && z_0 == 0 {
                    z_2.wrapping_add_assign(1);
                }
                if z_2 == 0 {
                    (z_0, z_1, HIGH_BIT, false, Greater)
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
                (z_0, z_1, HIGH_BIT, false, Greater)
            } else {
                (z_0, z_1, z_2, decrement_exp, Greater)
            }
        }
    }
}

pub(crate) const MPFR_MULHIGH_TAB: [i8; 17] =
    [-1, -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

// This is mpfr_mulhigh_n_basecase from mulders.c, MPFR 4.2.0.
fn limbs_float_mul_high_same_length_basecase(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    // We neglect xs[0..len - 2] * ys[0], which is less than B ^ len
    let out = &mut out[len - 1..];
    (out[1], out[0]) = Limb::x_mul_y_to_zz(*xs.last().unwrap(), ys[0]);
    for (i, y) in ys.iter().enumerate() {
        let i = i + 1;
        // Here, we neglect xs[0..len - i - 2] * ys[i], which is less than B ^ len too
        let (out_lo, out_hi) = out.split_at_mut(i);
        out_hi[0] = limbs_slice_add_mul_limb_same_length_in_place_left(out_lo, &xs[len - i..], *y);
        // In total, we neglect less than n * B ^ len, i.e., n ulps of out[len].
    }
}

fn limbs_float_mul_high_same_length_scratch_len(len: usize) -> usize {
    if len > MUL_FFT_THRESHOLD {
        limbs_mul_same_length_to_out_scratch_len(len)
    } else {
        let k = MPFR_MULHIGH_TAB.get(len).map_or_else(
            || 3 * (len >> 2),
            |&m| if m == -1 { 0 } else { usize::wrapping_from(m) },
        );
        if k == 0 {
            0
        } else {
            limbs_mul_same_length_to_out_scratch_len(max(len, len - k))
        }
    }
}

// Put in out[n..2 * len - 1] an approximation of the n high limbs of xs * ys. The error is less
// than len ulps of out[len] (and the approximation is always less or equal to the truncated full
// product).
//
// Implements Algorithm ShortMul from:
//
// [1] Short Division of Long Integers, David Harvey and Paul Zimmermann, Proceedings of the 20th
// Symposium on Computer Arithmetic (ARITH-20), July 25-27, 2011, pages 7-14.
//
// This is mpfr_mulhigh_n from mulders.c, MPFR 4.2.0.
pub(crate) fn limbs_float_mul_high_same_length(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    const LENGTH_VALID: bool = MPFR_MULHIGH_TAB.len() >= 8;
    assert!(LENGTH_VALID); // so that 3 * (len / 4) > len / 2
    let k = MPFR_MULHIGH_TAB.get(len).map_or_else(
        || Some(3 * (len >> 2)),
        |&m| {
            if m == -1 {
                None
            } else {
                Some(usize::wrapping_from(m))
            }
        },
    );
    assert!(k.is_none() || k == Some(0) || (k.unwrap() >= (len + 4) >> 1 && k.unwrap() < len));
    if k.is_none() {
        // result is exact, no error
        limbs_mul_greater_to_out_basecase(out, xs, ys);
    } else if k == Some(0) {
        // basecase error < len ulps
        limbs_float_mul_high_same_length_basecase(out, xs, ys);
    } else if len > MUL_FFT_THRESHOLD {
        // result is exact, no error
        limbs_mul_same_length_to_out(out, xs, ys, scratch);
    } else {
        let k = k.unwrap();
        let l = len - k;
        let out = &mut out[..len << 1];
        let (out_lo, out_hi) = out.split_at_mut(l << 1);
        let (ys_lo, ys_hi) = ys.split_at(l);
        limbs_mul_same_length_to_out(out_hi, &xs[l..], ys_hi, scratch);
        limbs_float_mul_high_same_length(out_lo, &xs[k..], ys_lo, scratch);
        let out_hi = &mut out_hi[k - l - 1..k];
        let mut carry = Limb::from(limbs_slice_add_same_length_in_place_left(
            out_hi,
            &out_lo[l - 1..],
        ));
        limbs_float_mul_high_same_length(out_lo, &xs[..l], &ys[k..], scratch);
        if limbs_slice_add_same_length_in_place_left(out_hi, &out_lo[l - 1..]) {
            carry += 1;
        }
        limbs_slice_add_limb_in_place(&mut out[len + l..], carry);
    }
}

const MPFR_MUL_THRESHOLD: usize = 20;

// x.limb_count() >= y.limb_count()
fn mul_float_significands_general(
    xs: &[Limb],
    x_prec: u64,
    ys: &[Limb],
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert!(xs_len >= ys_len);
    let mut new_xs;
    let mut new_ys;
    let orig_xs = xs;
    let orig_ys = ys;
    let mut xs = xs;
    let mut ys = ys;
    let k = xs_len.checked_add(ys_len).unwrap();
    let tmp_len = usize::wrapping_from(
        x_prec
            .checked_add(y_prec)
            .unwrap()
            .shr_round(Limb::LOG_WIDTH, Ceiling)
            .0,
    );
    assert!(tmp_len <= k);
    let mut tmp_vec: Vec<Limb>;
    let mut tmp: &mut [Limb];
    let mut b1 = false;
    let mut goto_full_multiply = xs_len > 2 && ys_len <= MPFR_MUL_THRESHOLD;
    let mut to = 0;
    if xs_len <= 2 {
        tmp_vec = vec![0; k];
        tmp = &mut tmp_vec;
        // The 3 cases perform the same first operation.
        (tmp[1], tmp[0]) = Limb::x_mul_y_to_zz(xs[0], ys[0]);
        b1 = if xs_len == 1 {
            // 1 limb * 1 limb
            tmp[1]
        } else if ys_len == 1 {
            // 2 limbs * 1 limb
            let t;
            (tmp[2], t) = Limb::x_mul_y_to_zz(xs[1], ys[0]);
            (tmp[2], tmp[1]) = Limb::xx_add_yy_to_zz(tmp[2], tmp[1], 0, t);
            tmp[2]
        } else {
            // 2 limbs * 2 limbs
            //
            // First 2 limbs * 1 limb
            let mut t1;
            (tmp[2], t1) = Limb::x_mul_y_to_zz(xs[1], ys[0]);
            (tmp[2], tmp[1]) = Limb::xx_add_yy_to_zz(tmp[2], tmp[1], 0, t1);
            let t2;
            // Second, the other 2 limbs * 1 limb product
            (t1, t2) = Limb::x_mul_y_to_zz(xs[0], ys[1]);
            let t3;
            (tmp[3], t3) = Limb::x_mul_y_to_zz(xs[1], ys[1]);
            (tmp[3], t1) = Limb::xx_add_yy_to_zz(tmp[3], t1, 0, t3);
            // Sum those two partial products
            (tmp[2], tmp[1]) = Limb::xx_add_yy_to_zz(tmp[2], tmp[1], t1, t2);
            let tmp_2 = tmp[2];
            if tmp_2 < t1 {
                tmp[3].wrapping_add_assign(1);
            }
            tmp[3]
        }
        .get_highest_bit();
        to = k - tmp_len;
        if !b1 {
            limbs_slice_shl_in_place(&mut tmp[to..to + tmp_len], 1);
        }
    } else if ys_len > MPFR_MUL_THRESHOLD {
        // xs_len >= ys_len and xs_len >= 3
        //
        // Mulders' mulhigh.
        //
        // First check if we can reduce the precision of x or y: exact values are a nightmare for
        // the short product trick
        if xs[0] == 0 && xs[1] == 0 || ys[0] == 0 && ys[1] == 0 {
            let xs_leading_zeros = slice_leading_zeros(xs);
            assert_ne!(xs_leading_zeros, xs_len);
            let ys_leading_zeros = slice_leading_zeros(ys);
            assert_ne!(ys_leading_zeros, ys_len);
            return mul_float_significands_ref_ref_helper(
                &xs[xs_leading_zeros..],
                u64::exact_from((xs_len - xs_leading_zeros) << Limb::LOG_WIDTH),
                &ys[ys_leading_zeros..],
                u64::exact_from((ys_len - ys_leading_zeros) << Limb::LOG_WIDTH),
                out_prec,
                rm,
            );
        }
        // Compute estimated precision of mulhigh.
        let mut len = min(
            usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0) + 1,
            ys_len,
        );
        assert!(len >= 1 && len << 1 <= k && len <= ys_len && len <= xs_len);
        let mut p = u64::exact_from(len << Limb::LOG_WIDTH) - (len + 2).ceiling_log_base_2();
        // Check if MulHigh can produce a roundable result. We may lose 1 bit due to Nearest, 1 due
        // to final shift.
        let mut tmp_alloc = k;
        if out_prec > p - 5 {
            if out_prec > p - 5 + Limb::WIDTH || xs_len <= MPFR_MUL_THRESHOLD + 1 {
                // MulHigh can't produce a roundable result.
                goto_full_multiply = true;
                xs = &xs[xs_len - len..];
                ys = &ys[ys_len - len..];
            } else {
                // Add one extra limb to mantissa of x and y.
                if xs_len > len {
                    xs = &xs[xs_len - len - 1..];
                } else {
                    new_xs = vec![0; len + 1];
                    new_xs[1..].copy_from_slice(&xs[xs_len - len..xs_len]);
                    xs = &new_xs;
                }
                #[cfg(feature = "32_bit_limbs")]
                {
                    if ys_len > len {
                        ys = &ys[ys_len - len - 1..];
                        // This can only happen with 32-bit limbs, and is very unlikely to happen.
                        // Indeed, since len = min(z_len + 1, ys_len), with z_len = prec /
                        // Limb::WIDTH, we can have ys_len > len only when len = z_len + 1 < ys_len.
                        // We are in the case prec > p - 5, p = len * Limb::WIDTH - ceil(log_2(len +
                        // 2)), thus z_len * Limb::WIDTH - shift > len * Limb::WIDTH -
                        // ceil(log_2(len + 2)) - 5. Thus len < z_len + (ceil(log_2(len + 2)) + 5 -
                        // shift) / Limb::WIDTH. To get len = z_len + 1, we need ceil(log_2(len +
                        // 2)) + 5 - shift > Limb::WIDTH, thus since shift >= 0 we need
                        // ceil(log_2(len + 2)) + 5 > Limb::WIDTH. With Limb::WIDTH = 32 this can
                        // only happen for len >= 2^27 - 1, thus for a precision of 2 ^ 32 - 64 for
                        // z, and with Limb::WIDTH = 64 for n >= 2 ^ 59-1, which would give a
                        // precision >= 2^64.
                    } else {
                        new_ys = vec![0; len + 1];
                        new_ys[1..].copy_from_slice(&ys[ys_len - len..ys_len]);
                        ys = &new_ys;
                    }
                }
                #[cfg(not(feature = "32_bit_limbs"))]
                {
                    new_ys = vec![0; len + 1];
                    new_ys[1..].copy_from_slice(&ys[ys_len - len..ys_len]);
                    ys = &new_ys;
                }
                // We will compute with one extra limb.
                len += 1;
                // ceil(log_2(len + 2)) takes into account the lost bits due to Mulders' short
                // product.
                p = u64::exact_from(len << Limb::LOG_WIDTH) - (len + 2).ceiling_log_base_2();
                // Due to some nasty reasons we can have only 4 bits
                assert!(out_prec <= p - 4);
                let twice_len = len << 1;
                if k < twice_len {
                    tmp_alloc = twice_len;
                    to = twice_len - k;
                } else {
                    fail_on_untested_path("mul_float_significands_general, k >= len << 1 ");
                }
            }
        } else {
            xs = &xs[xs_len - len..];
            ys = &ys[ys_len - len..];
        }
        if goto_full_multiply {
            tmp_vec = vec![0; tmp_alloc];
            tmp = &mut tmp_vec;
        } else {
            // Compute an approximation of the product of x and y
            tmp_vec = vec![0; limbs_float_mul_high_same_length_scratch_len(len) + tmp_alloc];
            let scratch: &mut [Limb];
            (tmp, scratch) = tmp_vec.split_at_mut(tmp_alloc);
            limbs_float_mul_high_same_length(&mut tmp[to + k - (len << 1)..], xs, ys, scratch);
            // now tmp[k - len]..tmp[k - 1] contains an approximation of the `len` upper limbs of
            // the product, with tmp[k - 1] >= 2 ^ (Limb::WIDTH - 2)
            //
            // msb from the product
            //
            // If the mantissas of x and y are uniformly distributed in (1/2, 1], then their product
            // is in (1/4, 1/2] with probability 2 * ln(2) - 1 ~ 0.386 and in [1/2, 1] with
            // probability 2 - 2 * ln(2) ~ 0.614
            b1 = tmp[to + k - 1].get_highest_bit();
            if !b1 {
                limbs_slice_shl_in_place(&mut tmp[to + k - len - 1..to + k], 1);
            }
            // Now the approximation is in tmp[temp_len - len]...tmp[temp_len - 1]
            assert!(tmp[to + k - 1].get_highest_bit());
            // if the most significant bit b1 is zero, we have only p - 1 correct bits
            if limbs_float_can_round(
                &tmp[to + k - tmp_len..to + k],
                p + u64::from(b1) - 1,
                out_prec,
                rm,
            ) {
                to += k - tmp_len;
            } else {
                goto_full_multiply = true;
            }
        }
    } else {
        tmp_vec = vec![0; k];
        tmp = &mut tmp_vec;
    }
    tmp = &mut tmp[to..];
    if goto_full_multiply {
        let mut scratch = vec![0; limbs_mul_to_out_scratch_len(xs_len, ys_len)];
        b1 = limbs_mul_to_out(tmp, orig_xs, orig_ys, &mut scratch).get_highest_bit();
        // Now tmp[0]..tmp[k - 1] contains the product of both mantissas, with tmp[k - 1] >= 2 ^
        // (Limb::WIDTH - 2).
        //
        // msb from the product
        //
        // If the mantissas of x and y are uniformly distributed in (1/2, 1], then their product is
        // in (1/4, 1/2] with probability 2 * ln(2) - 1 ~ 0.386 and in [1/2, 1] with probability 2 -
        // 2 * ln(2) ~ 0.614
        tmp = &mut tmp[k - tmp_len..];
        if !b1 {
            limbs_slice_shl_in_place(&mut tmp[..tmp_len], 1);
        }
    }
    let mut out = vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
    let (inexact, increment_exp) = round_helper_raw(
        &mut out,
        out_prec,
        tmp,
        x_prec.checked_add(y_prec).unwrap(),
        rm,
    );
    assert!(inexact == 0 || rm != Exact, "Inexact float multiplication");
    let mut exp_offset = -i32::from(!b1);
    if increment_exp {
        exp_offset += 1;
    }
    (
        Natural::from_owned_limbs_asc(out),
        exp_offset,
        inexact.sign(),
    )
}
