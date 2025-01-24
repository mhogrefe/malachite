// Copyright © 2025 Mikhail Hogrefe
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

use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::float_extras::{round_helper_even, MPFR_EVEN_INEX};
use crate::natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
use crate::natural::arithmetic::shl::limbs_slice_shl_in_place;
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::arithmetic::sub::{
    limbs_sub_limb_in_place, limbs_sub_limb_to_out, limbs_sub_same_length_in_place_left,
    limbs_sub_same_length_in_place_right, limbs_sub_same_length_to_out, sub_with_carry,
};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::{
    max,
    Ordering::{self, *},
};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, ModPowerOf2, ModPowerOf2Sub, NegAssign, NegModPowerOf2, OverflowingAddAssign,
    OverflowingNegAssign, PowerOf2, SaturatingAddAssign, SaturatingSubAssign, ShrRound, Sign,
    WrappingAddAssign, WrappingNegAssign, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom, WrappingFrom};
use malachite_base::num::logic::traits::LeadingZeros;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::{slice_set_zero, slice_test_zero};

const WIDTH_M1: u64 = Limb::WIDTH - 1;
const IWIDTH_M1: isize = WIDTH_M1 as isize;
const WIDTH_M1_MASK: Limb = Limb::MAX >> 1;
const WIDTH_M2_MASK: Limb = Limb::MAX >> 2;
const HIGH_BIT: Limb = 1 << WIDTH_M1;
const HALF_HIGH_BIT: Limb = HIGH_BIT >> 1;
const IWIDTH: i32 = Limb::WIDTH as i32;
const NEG_ONE: Limb = Limb::MAX;
const NEG_TWO: Limb = Limb::MAX - 1;
const WIDTH_P1: u64 = Limb::WIDTH + 1;
const TWICE_WIDTH: u64 = Limb::WIDTH * 2;
const THRICE_WIDTH: u64 = Limb::WIDTH * 3;
const TWICE_WIDTH_P1: u64 = Limb::WIDTH * 2 + 1;

pub fn sub_float_significands_in_place(
    mut x: &mut Natural,
    x_exp: &mut i32,
    x_prec: u64,
    mut y: &mut Natural,
    y_exp: i32,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Ordering, bool, bool) {
    if x_prec == y_prec && out_prec == x_prec {
        sub_float_significands_in_place_same_prec(x, x_exp, y, y_exp, out_prec, rm)
    } else {
        match (&mut x, &mut y) {
            (Natural(Small(small_x)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *small_x = out[0];
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Large(out));
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                }
            }
            (Natural(Small(small_x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        ys,
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *small_x = out[0];
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        ys,
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Large(out));
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                }
            }
            (Natural(Large(xs)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Small(out[0]));
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *xs = out;
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out, xs, *x_exp, x_prec, ys, y_exp, y_prec, out_prec, rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Small(out[0]));
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out, xs, *x_exp, x_prec, ys, y_exp, y_prec, out_prec, rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *xs = out;
                        *x_exp = diff_exp;
                    }
                    (o, false, neg)
                }
            }
        }
    }
}

pub fn sub_float_significands_in_place_ref(
    mut x: &mut Natural,
    x_exp: &mut i32,
    x_prec: u64,
    y: &Natural,
    y_exp: i32,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Ordering, bool) {
    if x_prec == y_prec && out_prec == x_prec {
        sub_float_significands_in_place_same_prec_ref(x, x_exp, y, y_exp, out_prec, rm)
    } else {
        match (&mut x, y) {
            (Natural(Small(small_x)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *small_x = out[0];
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Large(out));
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                }
            }
            (Natural(Small(small_x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        ys,
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *small_x = out[0];
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        x_prec,
                        ys,
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Large(out));
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                }
            }
            (Natural(Large(xs)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Small(out[0]));
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        x_prec,
                        &[*small_y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *xs = out;
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out, xs, *x_exp, x_prec, ys, y_exp, y_prec, out_prec, rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *x = Natural(Small(out[0]));
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (diff_exp, o, neg) = sub_float_significands_general(
                        &mut out, xs, *x_exp, x_prec, ys, y_exp, y_prec, out_prec, rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    if *out.last().unwrap() == 0 {
                        *x = Natural::ZERO;
                    } else {
                        *xs = out;
                        *x_exp = diff_exp;
                    }
                    (o, neg)
                }
            }
        }
    }
}

pub fn sub_float_significands_ref_ref<'a>(
    x: &'a Natural,
    x_exp: i32,
    x_prec: u64,
    y: &'a Natural,
    y_exp: i32,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering, bool) {
    if x_prec == y_prec && out_prec == x_prec {
        sub_float_significands_same_prec_ref_ref(x, x_exp, y, y_exp, out_prec, rm)
    } else {
        match (x, y) {
            (Natural(Small(x)), Natural(Small(y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        x_prec,
                        &[*y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (Natural(Small(out[0])), out_exp, o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        x_prec,
                        &[*y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (
                        if *out.last().unwrap() == 0 {
                            Natural::ZERO
                        } else {
                            Natural(Large(out))
                        },
                        out_exp,
                        o,
                        neg,
                    )
                }
            }
            (Natural(Small(x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        x_prec,
                        ys,
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (Natural(Small(out[0])), out_exp, o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        x_prec,
                        ys,
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (
                        if *out.last().unwrap() == 0 {
                            Natural::ZERO
                        } else {
                            Natural(Large(out))
                        },
                        out_exp,
                        o,
                        neg,
                    )
                }
            }
            (Natural(Large(xs)), Natural(Small(y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        xs,
                        x_exp,
                        x_prec,
                        &[*y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (Natural(Small(out[0])), out_exp, o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out,
                        xs,
                        x_exp,
                        x_prec,
                        &[*y],
                        y_exp,
                        y_prec,
                        out_prec,
                        rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (
                        if *out.last().unwrap() == 0 {
                            Natural::ZERO
                        } else {
                            Natural(Large(out))
                        },
                        out_exp,
                        o,
                        neg,
                    )
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out, xs, x_exp, x_prec, ys, y_exp, y_prec, out_prec, rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (Natural(Small(out[0])), out_exp, o, neg)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o, neg) = sub_float_significands_general(
                        &mut out, xs, x_exp, x_prec, ys, y_exp, y_prec, out_prec, rm,
                    );
                    assert!(rm != Exact || o == Equal, "Inexact float subtraction");
                    (
                        if *out.last().unwrap() == 0 {
                            Natural::ZERO
                        } else {
                            Natural(Large(out))
                        },
                        out_exp,
                        o,
                        neg,
                    )
                }
            }
        }
    }
}

// This is mpfr_sub1sp from sub1sp.c, MPFR 4.2.0.
fn sub_float_significands_in_place_same_prec(
    x: &mut Natural,
    x_exp: &mut i32,
    y: &mut Natural,
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Ordering, bool, bool) {
    match (&mut *x, &mut *y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (diff, diff_exp, o, neg) = if prec == Limb::WIDTH {
                sub_float_significands_same_prec_w(*x, *x_exp, *y, y_exp, rm)
            } else {
                sub_float_significands_same_prec_lt_w(*x, *x_exp, *y, y_exp, prec, rm)
            };
            *x = diff;
            *x_exp = diff_exp;
            (o, false, neg)
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_mut_slice()) {
            ([x_0, x_1], [y_0, y_1]) => {
                let (diff_0, diff_1, diff_exp, o, neg) = if prec == TWICE_WIDTH {
                    sub_float_significands_same_prec_2w(*x_0, *x_1, *x_exp, *y_0, *y_1, y_exp, rm)
                } else {
                    sub_float_significands_same_prec_gt_w_lt_2w(
                        *x_0, *x_1, *x_exp, *y_0, *y_1, y_exp, prec, rm,
                    )
                };
                if diff_1 == 0 {
                    *x = Natural::ZERO;
                } else {
                    *x_0 = diff_0;
                    *x_1 = diff_1;
                }
                *x_exp = diff_exp;
                (o, false, neg)
            }
            ([x_0, x_1, x_2], [y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (diff_0, diff_1, diff_2, diff_exp, o, neg) =
                    sub_float_significands_same_prec_gt_2w_lt_3w(
                        *x_0, *x_1, *x_2, *x_exp, *y_0, *y_1, *y_2, y_exp, prec, rm,
                    );
                if diff_2 == 0 {
                    *x = Natural::ZERO;
                } else {
                    *x_0 = diff_0;
                    *x_1 = diff_1;
                    *x_2 = diff_2;
                }
                *x_exp = diff_exp;
                (o, false, neg)
            }
            (xs, ys) => {
                let (diff_exp, o, neg) =
                    sub_float_significands_same_prec_ge_3w_val_val(xs, *x_exp, ys, y_exp, prec, rm);
                if *xs.last().unwrap() == 0 {
                    *x = Natural::ZERO;
                } else {
                    *x_exp = diff_exp;
                }
                (o, neg, neg)
            }
        },
        _ => unreachable!(),
    }
}

// This is mpfr_sub1sp from sub1sp.c, MPFR 4.2.0.
fn sub_float_significands_in_place_same_prec_ref(
    x: &mut Natural,
    x_exp: &mut i32,
    y: &Natural,
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Ordering, bool) {
    match (&mut *x, y) {
        (Natural(Small(ref mut x)), Natural(Small(y))) => {
            let (diff, diff_exp, o, neg) = if prec == Limb::WIDTH {
                sub_float_significands_same_prec_w(*x, *x_exp, *y, y_exp, rm)
            } else {
                sub_float_significands_same_prec_lt_w(*x, *x_exp, *y, y_exp, prec, rm)
            };
            *x = diff;
            *x_exp = diff_exp;
            (o, neg)
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_slice()) {
            ([x_0, x_1], &[y_0, y_1]) => {
                let (diff_0, diff_1, diff_exp, o, neg) = if prec == TWICE_WIDTH {
                    sub_float_significands_same_prec_2w(*x_0, *x_1, *x_exp, y_0, y_1, y_exp, rm)
                } else {
                    sub_float_significands_same_prec_gt_w_lt_2w(
                        *x_0, *x_1, *x_exp, y_0, y_1, y_exp, prec, rm,
                    )
                };
                if diff_1 == 0 {
                    *x = Natural::ZERO;
                } else {
                    *x_0 = diff_0;
                    *x_1 = diff_1;
                }
                *x_exp = diff_exp;
                (o, neg)
            }
            ([x_0, x_1, x_2], &[y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (diff_0, diff_1, diff_2, diff_exp, o, neg) =
                    sub_float_significands_same_prec_gt_2w_lt_3w(
                        *x_0, *x_1, *x_2, *x_exp, y_0, y_1, y_2, y_exp, prec, rm,
                    );
                if diff_2 == 0 {
                    *x = Natural::ZERO;
                } else {
                    *x_0 = diff_0;
                    *x_1 = diff_1;
                    *x_2 = diff_2;
                }
                *x_exp = diff_exp;
                (o, neg)
            }
            (xs, ys) => {
                let (diff_exp, o, neg) =
                    sub_float_significands_same_prec_ge_3w_val_ref(xs, *x_exp, ys, y_exp, prec, rm);
                if *xs.last().unwrap() == 0 {
                    *x = Natural::ZERO;
                } else {
                    *x_exp = diff_exp;
                }
                (o, neg)
            }
        },
        _ => unreachable!(),
    }
}

// This is mpfr_sub1sp from sub1sp.c, MPFR 4.2.0.
fn sub_float_significands_same_prec_ref_ref(
    x: &Natural,
    x_exp: i32,
    y: &Natural,
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering, bool) {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (diff, diff_exp, o, neg) = if prec == Limb::WIDTH {
                sub_float_significands_same_prec_w(*x, x_exp, *y, y_exp, rm)
            } else {
                sub_float_significands_same_prec_lt_w(*x, x_exp, *y, y_exp, prec, rm)
            };
            (Natural(Small(diff)), diff_exp, o, neg)
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_slice(), ys.as_slice()) {
            (&[x_0, x_1], &[y_0, y_1]) => {
                let (diff_0, diff_1, diff_exp, o, neg) = if prec == TWICE_WIDTH {
                    sub_float_significands_same_prec_2w(x_0, x_1, x_exp, y_0, y_1, y_exp, rm)
                } else {
                    sub_float_significands_same_prec_gt_w_lt_2w(
                        x_0, x_1, x_exp, y_0, y_1, y_exp, prec, rm,
                    )
                };
                (
                    if diff_1 == 0 {
                        Natural::ZERO
                    } else {
                        Natural(Large(vec![diff_0, diff_1]))
                    },
                    diff_exp,
                    o,
                    neg,
                )
            }
            (&[x_0, x_1, x_2], &[y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (diff_0, diff_1, diff_2, diff_exp, o, neg) =
                    sub_float_significands_same_prec_gt_2w_lt_3w(
                        x_0, x_1, x_2, x_exp, y_0, y_1, y_2, y_exp, prec, rm,
                    );
                (
                    if diff_2 == 0 {
                        Natural::ZERO
                    } else {
                        Natural(Large(vec![diff_0, diff_1, diff_2]))
                    },
                    diff_exp,
                    o,
                    neg,
                )
            }
            (xs, ys) => {
                let mut out = vec![0; xs.len()];
                let (diff_exp, o, neg) = sub_float_significands_same_prec_ge_3w_ref_ref(
                    &mut out, xs, x_exp, ys, y_exp, prec, rm,
                );
                (
                    if slice_test_zero(&out) {
                        Natural::ZERO
                    } else {
                        Natural(Large(out))
                    },
                    diff_exp,
                    o,
                    neg,
                )
            }
        },
        _ => unreachable!(),
    }
}

// This is mpfr_sub1sp1 from sub1sp.c, MPFR 4.2.0.
fn sub_float_significands_same_prec_lt_w(
    mut x: Limb,
    mut x_exp: i32,
    mut y: Limb,
    mut y_exp: i32,
    prec: u64,
    mut rm: RoundingMode,
) -> (Limb, i32, Ordering, bool) {
    {
        let (mut diff, sticky_bit, round_bit, shift_bit, neg) = if x_exp == y_exp {
            let (a0, neg) = match x.cmp(&y) {
                Equal => return (0, 0, Equal, false),
                Less => (y - x, true),
                Greater => (x - y, false),
            };
            let leading_zeros = a0.leading_zeros();
            x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
            (a0 << leading_zeros, 0, 0, 0, neg)
        } else {
            let neg = x_exp < y_exp;
            if neg {
                // swap x and y
                swap(&mut x_exp, &mut y_exp);
                swap(&mut x, &mut y);
            }
            let exp_diff = u64::exact_from(x_exp - y_exp);
            let shift = Limb::WIDTH - prec;
            let shift_bit = Limb::power_of_2(shift);
            let mask = shift_bit - 1;
            if exp_diff < Limb::WIDTH {
                // neglected part of -y
                let mut sticky_bit = (y << (Limb::WIDTH - exp_diff)).wrapping_neg();
                let mut a0 = x - Limb::from(sticky_bit != 0) - (y >> exp_diff);
                // a0 cannot be zero here since:
                // - if exp_diff >= 2, then a0 >= 2^(w-1) - (2^(w-2)-1) with w = Limb::WIDTH, thus
                //   a0 - 1 >= 2 ^ (w - 2),
                // - if exp_diff = 1, then since prec < Limb::WIDTH we have sticky_bit = 0.
                //
                assert_ne!(a0, 0);
                let leading_zeros = LeadingZeros::leading_zeros(a0);
                if leading_zeros != 0 {
                    a0 = (a0 << leading_zeros) | (sticky_bit >> (Limb::WIDTH - leading_zeros));
                }
                sticky_bit <<= leading_zeros;
                x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                // shift > 0 since prec < Limb::WIDTH
                assert_ne!(shift, 0);
                let round_bit = a0 & (shift_bit >> 1);
                (
                    a0 & !mask,
                    sticky_bit | (a0 & mask) ^ round_bit,
                    round_bit,
                    shift_bit,
                    neg,
                )
            } else if x > HIGH_BIT {
                // We compute x - ulp(x), and the remainder ulp(x) - y satisfies: 1/2 ulp(x) <
                // ulp(x) - y < ulp(x), thus round_bit = sticky_bit = 1.
                (x - shift_bit, 1, 1, shift_bit, neg)
            } else {
                // - Warning: since we have an exponent decrease, when prec = Limb::WIDTH - 1 and d
                //   = Limb::WIDTH, the round bit corresponds to the upper bit of -y. In that case
                //   round_bit = 0 and sticky_bit = 1, except when y0 = HIGH_BIT where round_bit = 1
                //   and sticky_bit = 0.
                // - sticky_bit = 1 below is incorrect when prec = Limb::WIDTH - 1, exp_diff =
                //   Limb::WIDTH and y0 = HIGH_BIT, but in that case the even rule would round up
                //   too.
                // - Warning: if exp_diff = Limb::WIDTH and y0 = 1000...000, then x0 - y0 =
                //   |0111...111|1000...000|, which after the shift becomes |111...111|000...000|
                //   thus if prec = Limb::WIDTH - 1 we have round_bit = 1 but sticky_bit = 0.
                //   However, in this case the round even rule will round up, which is what we get
                //   with sticky_bit = 1: the final result will be correct, while sb is incorrect.
                x_exp.saturating_sub_assign(1);
                (
                    !mask,
                    1,
                    Limb::from(shift > 1 || exp_diff > Limb::WIDTH || y == HIGH_BIT),
                    shift_bit,
                    neg,
                )
            }
        };
        if round_bit == 0 && sticky_bit == 0 {
            (diff, x_exp, Equal, neg)
        } else {
            if neg {
                rm.neg_assign();
            }
            match rm {
                Exact => panic!("Inexact float subtraction"),
                Nearest => {
                    if round_bit == 0 || (sticky_bit == 0 && (diff & shift_bit) == 0) {
                        (diff, x_exp, Less, neg)
                    } else if diff.overflowing_add_assign(shift_bit) {
                        (HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                    } else {
                        (diff, x_exp, Greater, neg)
                    }
                }
                Floor | Down => (diff, x_exp, Less, neg),
                Ceiling | Up => {
                    if diff.overflowing_add_assign(shift_bit) {
                        (HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                    } else {
                        (diff, x_exp, Greater, neg)
                    }
                }
            }
        }
    }
}

// This is mpfr_sub1sp1n from sub1sp.c, MPFR 4.2.0.
fn sub_float_significands_same_prec_w(
    mut x: Limb,
    mut x_exp: i32,
    mut y: Limb,
    mut y_exp: i32,
    mut rm: RoundingMode,
) -> (Limb, i32, Ordering, bool) {
    let (mut diff, sticky_bit, round_bit, neg) = if x_exp == y_exp {
        let (a0, neg) = match x.cmp(&y) {
            Equal => return (0, 0, Equal, false),
            Less => (y - x, true),
            Greater => (x - y, false),
        };
        let leading_zeros = LeadingZeros::leading_zeros(a0);
        x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
        (a0 << leading_zeros, 0, 0, neg)
    } else {
        let neg = x_exp < y_exp;
        if neg {
            // swap x and y
            swap(&mut x_exp, &mut y_exp);
            swap(&mut x, &mut y);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        if exp_diff < Limb::WIDTH {
            let mut sticky_bit = (y << (Limb::WIDTH - exp_diff)).wrapping_neg();
            let mut a0 = x.wrapping_sub(y >> exp_diff);
            if sticky_bit != 0 {
                a0.wrapping_sub_assign(1);
            }
            // a0 can only be zero when exp_diff = 1, x0 = B / 2, and y0 = B-1, where B = 2 ^
            // Limb::WIDTH, thus x0 - y0 / 2 = 1/2
            if a0 == 0 {
                x_exp.saturating_sub_assign(IWIDTH);
                (HIGH_BIT, 0, 0, neg)
            } else {
                let leading_zeros = LeadingZeros::leading_zeros(a0);
                if leading_zeros != 0 {
                    a0 = (a0 << leading_zeros) | (sticky_bit >> (Limb::WIDTH - leading_zeros));
                }
                sticky_bit <<= leading_zeros;
                x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                let round_bit = sticky_bit & HIGH_BIT;
                (a0, sticky_bit & !HIGH_BIT, round_bit, neg)
            }
        } else {
            // We compute x - ulp(x)
            if x > HIGH_BIT {
                // If exp_diff = Limb::WIDTH, round_bit = 0 and sticky_bit = 1, unless c0 = HIGH_BIT
                // in which case round_bit = 1 and sticky_bit = 0. If exp_diff > Limb::WIDTH,
                // round_bit = sticky_bit = 1.
                let b = exp_diff > Limb::WIDTH;
                (
                    x - 1,
                    Limb::from(b || y != HIGH_BIT),
                    Limb::from(b || y == HIGH_BIT),
                    neg,
                )
            } else {
                // Warning: in this case a0 is shifted by one!
                //
                // If exp_diff = Limb::WIDTH
                // - a) If y0 = HIGH_BIT, a0 = 111...111, round_bit = sticky_bit = 0
                // - b) Otherwise, a0 = 111...110, round_bit = -y0 >= 01000...000, sticky_bit =
                //   (-y0) << 2
                //
                // If exp_diff = Limb::WIDTH + 1: a0 = 111...111
                // - c) If y0 = HIGH_BIT, round_bit = 1 and sticky_bit = 0
                // - d) Otherwise round_bit = 0 and sticky_bit = 1
                //
                // If exp_diff > Limb::WIDTH + 1:
                // - e) a0 = 111...111, round_bit = sticky_bit = 1
                x_exp.saturating_sub_assign(1);
                if exp_diff == Limb::WIDTH && y > HIGH_BIT {
                    // case (b)
                    (
                        NEG_TWO,
                        y.wrapping_neg() << 2,
                        Limb::from(y.wrapping_neg() >= (HIGH_BIT >> 1)),
                        neg,
                    )
                } else {
                    // cases (a), (c), (d) and (e)
                    // - round_bit = 1 in case (e) and case (c)
                    // - sticky_bit = 1 in case (d) and (e)
                    let b1 = exp_diff > WIDTH_P1;
                    let b2 = exp_diff == WIDTH_P1;
                    (
                        NEG_ONE,
                        Limb::from(b1 || (b2 && y > HIGH_BIT)),
                        Limb::from(b1 || (b2 && y == HIGH_BIT)),
                        neg,
                    )
                }
            }
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (diff, x_exp, Equal, neg)
    } else {
        if neg {
            rm.neg_assign();
        }
        match rm {
            Exact => panic!("Inexact float subtraction"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (diff & 1) == 0) {
                    (diff, x_exp, Less, neg)
                } else if diff.overflowing_add_assign(1) {
                    (HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                } else {
                    (diff, x_exp, Greater, neg)
                }
            }
            Floor | Down => (diff, x_exp, Less, neg),
            Ceiling | Up => {
                if diff.overflowing_add_assign(1) {
                    (HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                } else {
                    (diff, x_exp, Greater, neg)
                }
            }
        }
    }
}

// This is mpfr_sub1sp2 from sub1sp.c, MPFR 4.2.0.
fn sub_float_significands_same_prec_gt_w_lt_2w(
    mut x_0: Limb,
    mut x_1: Limb,
    mut x_exp: i32,
    mut y_0: Limb,
    mut y_1: Limb,
    mut y_exp: i32,
    prec: u64,
    mut rm: RoundingMode,
) -> (Limb, Limb, i32, Ordering, bool) {
    let (mut diff_0, mut diff_1, sticky_bit, round_bit, shift_bit, neg) = if x_exp == y_exp {
        // subtraction is exact in this case
        //
        // first compute a0: if the compiler is smart enough, it will use the generated borrow to
        // get for free the term (x_0 < y_0)
        let (mut a0, overflow) = x_0.overflowing_sub(y_0);
        let mut a1 = x_1.wrapping_sub(y_1);
        if overflow {
            a1.wrapping_sub_assign(1);
        }
        let neg = if a1 == 0 && a0 == 0 {
            return (0, 0, 0, Equal, false);
        } else if a1 >= x_1 {
            // out = x - y mod 2 ^ (2 * Limb::WIDTH)
            let overflow = a0.overflowing_neg_assign();
            a1.wrapping_neg_assign();
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            true
        } else {
            false
        };
        if a1 == 0 {
            a1 = a0;
            a0 = 0;
            x_exp.saturating_sub_assign(IWIDTH);
        }
        // now a1 != 0
        let leading_zeros = LeadingZeros::leading_zeros(a1);
        if leading_zeros != 0 {
            x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
            (
                a0 << leading_zeros,
                (a1 << leading_zeros) | (a0 >> (Limb::WIDTH - leading_zeros)),
                0,
                0,
                0,
                neg,
            )
        } else {
            (a0, a1, 0, 0, 0, neg)
        }
    } else {
        let neg = x_exp < y_exp;
        if neg {
            swap(&mut x_exp, &mut y_exp);
            swap(&mut x_0, &mut y_0);
            swap(&mut x_1, &mut y_1);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        let shift = TWICE_WIDTH - prec;
        let shift_bit = Limb::power_of_2(shift);
        let mask = shift_bit - 1;
        if exp_diff < Limb::WIDTH {
            let comp_diff = Limb::WIDTH - exp_diff;
            let t = (y_1 << comp_diff) | (y_0 >> exp_diff);
            let (mut sticky_bit, overflow_1) = (y_0 << comp_diff).overflowing_neg();
            let (mut a0, overflow_2) = x_0.overflowing_sub(t);
            if overflow_1 {
                a0.wrapping_sub_assign(1);
            }
            let mut a1 = x_1.wrapping_sub(y_1 >> exp_diff);
            if overflow_2 || (x_0 == t && overflow_1) {
                a1.wrapping_sub_assign(1);
            }
            if a1 == 0 {
                // This implies exp_diff = 1, which in turn implies sticky_bit = 0
                assert_eq!(sticky_bit, 0);
                a1 = a0;
                a0 = 0;
                // Since sticky_bit = 0 already, no need to set it to 0
                x_exp.saturating_sub_assign(IWIDTH);
            }
            // now a1 != 0
            assert_ne!(a1, 0);
            let leading_zeros = LeadingZeros::leading_zeros(a1);
            let diff_1 = if leading_zeros != 0 {
                let comp_zeros = Limb::WIDTH - leading_zeros;
                let diff_1 = (a1 << leading_zeros) | (a0 >> comp_zeros);
                a0 = (a0 << leading_zeros) | (sticky_bit >> comp_zeros);
                sticky_bit <<= leading_zeros;
                x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                diff_1
            } else {
                a1
            };
            // shift > 0 since prec < 2 * Limb::WIDTH
            assert_ne!(shift, 0);
            let round_bit = a0 & (shift_bit >> 1);
            (
                a0 & !mask,
                diff_1,
                sticky_bit | ((a0 & mask) ^ round_bit),
                round_bit,
                shift_bit,
                neg,
            )
        } else if exp_diff < TWICE_WIDTH {
            // Warning: the most significant bit of sticky_bit might become the least significant
            // bit of a0 below
            let mut sticky_bit = if exp_diff == Limb::WIDTH {
                y_0
            } else {
                let mut sticky_bit = y_1 << (TWICE_WIDTH - exp_diff);
                if y_0 != 0 {
                    sticky_bit |= 1;
                }
                sticky_bit
            };
            let mut t = y_1 >> (exp_diff - Limb::WIDTH);
            if sticky_bit != 0 {
                t.wrapping_add_assign(1);
            }
            // Warning: t might overflow to 0 if exp_diff == Limb::WIDTH and sticky_bit != 0
            let (mut a0, overflow) = x_0.overflowing_sub(t);
            let mut a1 = x_1;
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            if t == 0 && sticky_bit != 0 {
                a1.wrapping_sub_assign(1);
            }
            sticky_bit.wrapping_neg_assign();
            // since x_1 has its most significant bit set, we can have an exponent decrease of at
            // most one
            let diff_1 = if a1 < HIGH_BIT {
                let diff_1 = (a1 << 1) | (a0 >> WIDTH_M1);
                a0 = (a0 << 1) | (sticky_bit >> WIDTH_M1);
                sticky_bit <<= 1;
                x_exp.saturating_sub_assign(1);
                diff_1
            } else {
                a1
            };
            let round_bit = a0 & (shift_bit >> 1);
            (
                a0 & !mask,
                diff_1,
                sticky_bit | ((a0 & mask) ^ round_bit),
                round_bit,
                shift_bit,
                neg,
            )
        } else {
            // We compute x - ulp(x), and the remainder ulp(x) - y satisfies: 1/2 ulp(x) < ulp(x) -
            // y < ulp(x), thus round_bit = sticky_bit = 1, unless we had an exponent decrease.
            let t = shift_bit;
            let (a0, overflow) = x_0.overflowing_sub(t);
            let mut a1 = x_1;
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            if a1 < HIGH_BIT {
                // Necessarily we had x = 1000...000
                //
                // Warning: since we have an exponent decrease, when prec = Limb::WIDTH * 2 - 1 and
                // exp_diff = Limb::WIDTH * 2, the round bit corresponds to the upper bit of -y. In
                // that case round_bit = 0 and sticky_bit = 1, except when y = 1000...000 where
                // round_bit = 1 and sticky_bit = 0.
                //
                // sticky_bit = 1 below is incorrect when prec = Limb::WIDTH * 2 - 1, exp_diff =
                // Limb::WIDTH * 2, and y = 1000...000, but in that case the even rule would round
                // up too.
                x_exp.saturating_sub_assign(1);
                (
                    !mask,
                    Limb::MAX,
                    1,
                    Limb::from(
                        shift > 1 || exp_diff > TWICE_WIDTH || (y_1 == HIGH_BIT && y_0 == 0),
                    ),
                    shift_bit,
                    neg,
                )
            } else {
                (a0, a1, 1, 1, shift_bit, neg)
            }
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (diff_0, diff_1, x_exp, Equal, neg)
    } else {
        if neg {
            rm.neg_assign();
        }
        match rm {
            Exact => panic!("Inexact float subtraction"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (diff_0 & shift_bit) == 0) {
                    (diff_0, diff_1, x_exp, Less, neg)
                } else if diff_0.overflowing_add_assign(shift_bit)
                    && diff_1.overflowing_add_assign(1)
                {
                    (diff_0, HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                } else {
                    (diff_0, diff_1, x_exp, Greater, neg)
                }
            }
            Floor | Down => (diff_0, diff_1, x_exp, Less, neg),
            Ceiling | Up => {
                if diff_0.overflowing_add_assign(shift_bit) && diff_1.overflowing_add_assign(1) {
                    (diff_0, HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                } else {
                    (diff_0, diff_1, x_exp, Greater, neg)
                }
            }
        }
    }
}

// This is mpfr_sub1sp2n from add1sp.c, MPFR 4.2.0.
fn sub_float_significands_same_prec_2w(
    mut x_0: Limb,
    mut x_1: Limb,
    mut x_exp: i32,
    mut y_0: Limb,
    mut y_1: Limb,
    mut y_exp: i32,
    mut rm: RoundingMode,
) -> (Limb, Limb, i32, Ordering, bool) {
    let (mut diff_0, mut diff_1, sticky_bit, round_bit, neg) = if x_exp == y_exp {
        let (mut a0, overflow) = x_0.overflowing_sub(y_0);
        let mut a1 = x_1.wrapping_sub(y_1);
        if overflow {
            a1.wrapping_sub_assign(1);
        }
        let neg = if a1 == 0 && a0 == 0 {
            return (0, 0, 0, Equal, false);
        } else if a1 >= x_1 {
            // since B/2 <= x_1, y_1 < B with B = 2 ^ Limb::WIDTH, if no borrow we have 0 <= x_1 -
            // y_1 - x < B / 2, where x = (x_0 < y_0) is 0 or 1, thus a1 < B / 2 <= x_1
            //
            // negate [a1,a0]
            let overflow = a0.overflowing_neg_assign();
            a1.wrapping_neg_assign();
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            true
        } else {
            false
        };
        // now [a1,a0] is the absolute value of x - y, maybe not normalized
        if a1 == 0 {
            a1 = a0;
            a0 = 0;
            x_exp.saturating_sub_assign(IWIDTH);
        }
        let leading_zeros = LeadingZeros::leading_zeros(a1);
        if leading_zeros != 0 {
            // shift [a1, a0] left by leading_zeros bits and store in result
            x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
            (
                a0 << leading_zeros,
                (a1 << leading_zeros) | (a0 >> (Limb::WIDTH - leading_zeros)),
                0,
                0,
                neg,
            )
        } else {
            (a0, a1, 0, 0, neg)
        }
    } else {
        let neg = x_exp < y_exp;
        if neg {
            swap(&mut x_exp, &mut y_exp);
            swap(&mut x_0, &mut y_0);
            swap(&mut x_1, &mut y_1);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        if exp_diff < Limb::WIDTH {
            let comp_diff = Limb::WIDTH - exp_diff;
            let t = (y_1 << comp_diff) | (y_0 >> exp_diff);
            // t is the part that should be subtracted to x_0:
            // ```
            // |      a1       |      a0       |
            // |     x_1       |     x_0       |
            // |    y_1 >> d   |      t        |     sticky_bit     |
            // ```
            let (mut sticky_bit, overflow_1) = (y_0 << comp_diff).overflowing_neg();
            let (mut a0, overflow_2) = x_0.overflowing_sub(t);
            if overflow_1 {
                a0.wrapping_sub_assign(1);
            }
            let mut a1 = x_1.wrapping_sub(y_1 >> exp_diff);
            if overflow_2 || (x_0 == t && overflow_1) {
                a1.wrapping_sub_assign(1);
            }
            // Now the result is formed of [a1,a0,sticky_bit], which might not be normalized
            if a1 == 0 {
                // this implies d = 1
                assert_eq!(exp_diff, 1);
                a1 = a0;
                a0 = sticky_bit;
                sticky_bit = 0;
                x_exp.saturating_sub_assign(IWIDTH);
            }
            if a1 == 0 {
                assert_eq!(a0, HIGH_BIT);
                x_exp.saturating_sub_assign(IWIDTH);
                (sticky_bit, a0, 0, 0, neg)
            } else {
                let leading_zeros = LeadingZeros::leading_zeros(a1);
                if leading_zeros != 0 {
                    let comp_zeros = Limb::WIDTH - leading_zeros;
                    // shift [a1, a0, sticky_bit] left by leading_zeros bits and adjust exponent
                    a1 = (a1 << leading_zeros) | (a0 >> comp_zeros);
                    a0 = (a0 << leading_zeros) | (sticky_bit >> comp_zeros);
                    sticky_bit <<= leading_zeros;
                    x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                }
                (a0, a1, sticky_bit & !HIGH_BIT, sticky_bit & HIGH_BIT, neg)
            }
        } else if exp_diff < TWICE_WIDTH {
            // Compute t, the part to be subtracted to x_0, and sticky_bit, the neglected part of y:
            //
            // ```
            // |      a1       |      a0       |
            // |     diff_1    |     diff_0    |
            //                 |      t        |     sticky_bit     |
            // ```
            //
            // Warning: we should not ignore the low bits from y_0 in case exp_diff > Limb::WIDTH
            let comp_diff_1 = exp_diff - Limb::WIDTH;
            let comp_diff_2 = TWICE_WIDTH - exp_diff;
            let mut sticky_bit = if comp_diff_1 == 0 {
                y_0
            } else {
                let mut sticky_bit = (y_1 << comp_diff_2) | (y_0 >> comp_diff_1);
                if y_0 << comp_diff_2 != 0 {
                    sticky_bit |= 1;
                }
                sticky_bit
            };
            let mut t = y_1 >> comp_diff_1;
            if sticky_bit != 0 {
                t.wrapping_add_assign(1);
            }
            // Warning: t might overflow to 0 if exp_diff = Limb::WIDTH, sticky_bit != 0, and y_1 =
            // 111...111.
            let (mut a0, overflow) = x_0.overflowing_sub(t);
            let mut a1 = x_1;
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            if t == 0 && sticky_bit != 0 {
                a1.wrapping_sub_assign(1);
            }
            sticky_bit.wrapping_neg_assign();
            // Now the result is [a1, a0, sticky_bit]. Since x_1 has its most significant bit set,
            // we can have an exponent decrease of at most one
            if a1 < HIGH_BIT {
                // shift [a1, a0] left by 1 bit
                a1 = (a1 << 1) | (a0 >> WIDTH_M1);
                assert!(a1 >= HIGH_BIT);
                a0 = (a0 << 1) | (sticky_bit >> WIDTH_M1);
                sticky_bit <<= 1;
                x_exp.saturating_sub_assign(1);
            }
            (a0, a1, sticky_bit & !HIGH_BIT, sticky_bit & HIGH_BIT, neg)
        } else {
            // ```
            // |      a1       |      a0       |
            // |      x_1      |      x_0      |
            //                                 |   y_1   |   y_0   |
            // ```
            let tst = y_1 == HIGH_BIT && y_0 == 0;
            // if exp_diff = Limb::WIDTH * 2 and tst = 1, y = 1 / 2 * ulp(x)
            if x_1 > HIGH_BIT || x_0 > 0 {
                let g = exp_diff > TWICE_WIDTH;
                let mut diff_1 = x_1;
                if x_0 == 0 {
                    diff_1.wrapping_sub_assign(1);
                }
                // no borrow in x - ulp(x)
                (
                    x_0.wrapping_sub(1),
                    diff_1,
                    Limb::from(g || !tst),
                    Limb::from(g || tst),
                    neg,
                )
            } else {
                // x = 1000...000, thus subtracting y yields an exponent shift
                x_exp.saturating_sub_assign(1);
                if exp_diff == TWICE_WIDTH && !tst {
                    // y > 1 / 2 * ulp(x)
                    let mut t = y_1.wrapping_neg();
                    if y_0 != 0 {
                        t.wrapping_sub_assign(1);
                    }
                    // The rounding bit is the 2nd most-significant bit of t (where the most
                    // significant bit of t is necessarily 0), and the sticky bit is formed by the
                    // remaining bits of t, and those from -y_0.
                    (
                        NEG_TWO,
                        Limb::MAX,
                        (t << 2) | y_0,
                        Limb::from(t >= HALF_HIGH_BIT),
                        neg,
                    )
                } else {
                    // y <= 1 / 2 * ulp(x)
                    let g = exp_diff > TWICE_WIDTH_P1;
                    (
                        NEG_ONE,
                        NEG_ONE,
                        Limb::from(g || (exp_diff == TWICE_WIDTH_P1 && !tst)),
                        Limb::from(g || (exp_diff == TWICE_WIDTH_P1 && tst)),
                        neg,
                    )
                }
            }
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (diff_0, diff_1, x_exp, Equal, neg)
    } else {
        if neg {
            rm.neg_assign();
        }
        match rm {
            Exact => panic!("Inexact float subtraction"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (diff_0 & 1) == 0) {
                    (diff_0, diff_1, x_exp, Less, neg)
                } else if diff_0.overflowing_add_assign(1) && diff_1.overflowing_add_assign(1) {
                    (diff_0, HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                } else {
                    (diff_0, diff_1, x_exp, Greater, neg)
                }
            }
            Floor | Down => (diff_0, diff_1, x_exp, Less, neg),
            Ceiling | Up => {
                if diff_0.overflowing_add_assign(1) && diff_1.overflowing_add_assign(1) {
                    (diff_0, HIGH_BIT, x_exp.saturating_add(1), Greater, neg)
                } else {
                    (diff_0, diff_1, x_exp, Greater, neg)
                }
            }
        }
    }
}

// This is mpfr_sub1sp3 from add1sp.c, MPFR 4.2.0.
fn sub_float_significands_same_prec_gt_2w_lt_3w(
    mut x_0: Limb,
    mut x_1: Limb,
    mut x_2: Limb,
    mut x_exp: i32,
    mut y_0: Limb,
    mut y_1: Limb,
    mut y_2: Limb,
    mut y_exp: i32,
    prec: u64,
    mut rm: RoundingMode,
) -> (Limb, Limb, Limb, i32, Ordering, bool) {
    let (mut diff_0, mut diff_1, mut diff_2, sticky_bit, round_bit, shift_bit, neg) = if x_exp
        == y_exp
    {
        let (mut a0, overflow_1) = x_0.overflowing_sub(y_0);
        let (mut a1, overflow_2) = x_1.overflowing_sub(y_1);
        if overflow_1 {
            a1.wrapping_sub_assign(1);
        }
        // A borrow is generated for diff when either x_1 < y_1 or x_1 = y_1 and x_0 < y_0.
        let mut a2 = x_2.wrapping_sub(y_2);
        if overflow_2 || x_1 == y_1 && overflow_1 {
            a2.wrapping_sub_assign(1);
        }
        let neg = if a2 == 0 && a1 == 0 && a0 == 0 {
            return (0, 0, 0, 0, Equal, false);
        } else if a2 >= x_2 {
            // a = x - y mod 2 ^ (3 * Limb::WIDTH)
            let overflow_1 = a0.overflowing_neg_assign();
            a1.wrapping_neg_assign();
            if overflow_1 {
                a1.wrapping_sub_assign(1);
            }
            a2.wrapping_neg_assign();
            if overflow_1 || a1 != 0 {
                a2.wrapping_sub_assign(1);
            }
            true
        } else {
            false
        };
        if a2 == 0 {
            a2 = a1;
            a1 = a0;
            a0 = 0;
            x_exp.saturating_sub_assign(IWIDTH);
            if a2 == 0 {
                a2 = a1;
                a1 = 0;
                x_exp.saturating_sub_assign(IWIDTH);
            }
        }
        assert_ne!(a2, 0);
        let leading_zeros = LeadingZeros::leading_zeros(a2);
        if leading_zeros != 0 {
            x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
            let comp_zeros = Limb::WIDTH - leading_zeros;
            (
                a0 << leading_zeros,
                (a1 << leading_zeros) | (a0 >> comp_zeros),
                (a2 << leading_zeros) | (a1 >> comp_zeros),
                0,
                0,
                0,
                neg,
            )
        } else {
            (a0, a1, a2, 0, 0, 0, neg)
        }
    } else {
        let neg = x_exp < y_exp;
        if neg {
            swap(&mut x_exp, &mut y_exp);
            swap(&mut x_0, &mut y_0);
            swap(&mut x_1, &mut y_1);
            swap(&mut x_2, &mut y_2);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        let shift = THRICE_WIDTH - prec;
        let shift_bit = Limb::power_of_2(shift);
        let mask = shift_bit - 1;
        if exp_diff < Limb::WIDTH {
            // Warning: we must have the most significant bit of sticky_bit correct since it might
            // become the round bit below
            let comp_diff = Limb::WIDTH - exp_diff;
            let mut sticky_bit = y_0 << comp_diff;
            let (mut a0, overflow) = x_0.overflowing_sub((y_1 << comp_diff) | (y_0 >> exp_diff));
            let mut a1 = x_1.wrapping_sub((y_2 << comp_diff) | (y_1 >> exp_diff));
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            let carry = a1 > x_1 || (a1 == x_1 && overflow);
            let mut a2 = x_2.wrapping_sub(y_2 >> exp_diff);
            if carry {
                a2.wrapping_sub_assign(1);
            }
            // if sticky_bit is non-zero, subtract 1 from a2, a1, a0 since we want a non-negative
            // neglected part
            if sticky_bit != 0 {
                if a1 == 0 && a0 == 0 {
                    a2.wrapping_sub_assign(1);
                }
                if a0 == 0 {
                    a1.wrapping_sub_assign(1);
                }
                a0.wrapping_sub_assign(1);
                // a = a2, a1, a0 cannot become zero here, since:
                // - if exp_diff >= 2, then a2 >= 2 ^ (w - 1) - (2 ^ (w - 2) - 1) with w =
                //   Limb::WIDTH, thus a2 - 1 >= 2 ^ (w - 2),
                // - if exp_diff = 1, then since prec < 3 * Limb::WIDTH we have sticky_bit = 0.
                assert!(a2 > 0 || a1 > 0 || a0 > 0);
                // 2 ^ Limb::WIDTH - sticky_bit
                sticky_bit.wrapping_neg_assign();
            }
            if a2 == 0 {
                // this implies exp_diff = 1, which in turn implies sticky_bit = 0
                assert_eq!(sticky_bit, 0);
                a2 = a1;
                a1 = a0;
                a0 = 0;
                // since sticky_bit = 0 already, no need to set it to 0
                x_exp.saturating_sub_assign(IWIDTH);
                if a2 == 0 {
                    a2 = a1;
                    a1 = 0;
                    x_exp.saturating_sub_assign(IWIDTH);
                }
            }
            assert_ne!(a2, 0);
            let leading_zeros = LeadingZeros::leading_zeros(a2);
            let (diff_1, diff_2) = if leading_zeros != 0 {
                let comp_zeros = Limb::WIDTH - leading_zeros;
                let diff_1 = (a1 << leading_zeros) | (a0 >> comp_zeros);
                a0 = (a0 << leading_zeros) | (sticky_bit >> comp_zeros);
                sticky_bit <<= leading_zeros;
                x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                (diff_1, (a2 << leading_zeros) | (a1 >> comp_zeros))
            } else {
                (a1, a2)
            };
            // shift > 0 since prec < 2 * Limb::WIDTH
            assert_ne!(shift, 0);
            let round_bit = a0 & (shift_bit >> 1);
            (
                a0 & !mask,
                diff_1,
                diff_2,
                sticky_bit | (a0 & mask) ^ round_bit,
                round_bit,
                shift_bit,
                neg,
            )
        } else if exp_diff < TWICE_WIDTH {
            // Warning: we must have the most significant bit of sticky_bit correct since it might
            // become the round bit below
            let comp_diff = exp_diff - Limb::WIDTH;
            let (mut sticky_bit, y0shifted) = if exp_diff == Limb::WIDTH {
                (y_0, y_1)
            } else {
                let comp_diff_2 = TWICE_WIDTH - exp_diff;
                let mut sticky_bit = y_1 << comp_diff_2;
                if y_0 != 0 {
                    sticky_bit |= 1;
                }
                (sticky_bit, (y_2 << comp_diff_2) | (y_1 >> comp_diff))
            };
            let (mut a0, overflow) = x_0.overflowing_sub(y0shifted);
            let mut a1 = x_1.wrapping_sub(y_2 >> comp_diff);
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            let mut a2 = x_2;
            if a1 > x_1 || (a1 == x_1 && overflow) {
                a2.wrapping_sub_assign(1);
            }
            // if sticky_bit is non-zero, subtract 1 from a2, a1, a0 since we want a non-negative
            // neglected part
            if sticky_bit != 0 {
                if a1 == 0 && a0 == 0 {
                    a2.wrapping_sub_assign(1);
                }
                if a0 == 0 {
                    a1.wrapping_sub_assign(1);
                }
                a0.wrapping_sub_assign(1);
                // a = a2, a1, a0 cannot become zero here, since:
                // - if exp_diff >= 2, then a2 >= 2 ^ (w - 1) - (2 ^ (w - 2) - 1) with w =
                //   Limb::WIDTH, thus a2 - 1 >= 2 ^ (w - 2),
                // - if exp_diff = 1, then since p < 3 * Limb::WIDTH we have sticky_bit = 0.
                assert!(a2 > 0 || a1 > 0 || a0 > 0);
                // 2 ^ Limb::WIDTH - sticky_bit
                sticky_bit.wrapping_neg_assign();
            }
            // since x_2 has its most significant bit set, we can have an exponent decrease of at
            // most one
            let (diff_1, diff_2) = if a2 < HIGH_BIT {
                let diff_1 = (a1 << 1) | (a0 >> WIDTH_M1);
                a0 = (a0 << 1) | (sticky_bit >> WIDTH_M1);
                sticky_bit <<= 1;
                x_exp.saturating_sub_assign(1);
                (diff_1, (a2 << 1) | (a1 >> WIDTH_M1))
            } else {
                (a1, a2)
            };
            let round_bit = a0 & (shift_bit >> 1);
            (
                a0 & !mask,
                diff_1,
                diff_2,
                sticky_bit | (a0 & mask) ^ round_bit,
                round_bit,
                shift_bit,
                neg,
            )
        } else if exp_diff < THRICE_WIDTH {
            // warning: we must have the most significant bit of sticky_bit correct since it might
            // become the round bit below
            let mut sticky_bit;
            if exp_diff == TWICE_WIDTH {
                sticky_bit = y_1;
                if y_0 != 0 {
                    sticky_bit |= 1;
                }
            } else {
                sticky_bit = y_2 << (THRICE_WIDTH - exp_diff);
                if y_1 != 0 || y_0 != 0 {
                    sticky_bit |= 1;
                }
            };
            let overflow = sticky_bit.overflowing_neg_assign();
            let mut a0 = x_0.wrapping_sub(y_2 >> (exp_diff - TWICE_WIDTH));
            if overflow {
                a0.wrapping_sub_assign(1);
            }
            let mut a1 = x_1;
            if a0 > x_0 || (a0 == x_0 && overflow) {
                a1.wrapping_sub_assign(1);
            }
            let mut a2 = x_2;
            if a1 > x_1 {
                a2.wrapping_sub_assign(1);
            }
            let (diff_1, diff_2) = if a2 < HIGH_BIT {
                let diff_1 = (a1 << 1) | (a0 >> WIDTH_M1);
                a0 = (a0 << 1) | (sticky_bit >> WIDTH_M1);
                sticky_bit <<= 1;
                x_exp.saturating_sub_assign(1);
                (diff_1, (a2 << 1) | (a1 >> WIDTH_M1))
            } else {
                (a1, a2)
            };
            let round_bit = a0 & (shift_bit >> 1);
            (
                a0 & !mask,
                diff_1,
                diff_2,
                sticky_bit | (a0 & mask) ^ round_bit,
                round_bit,
                shift_bit,
                neg,
            )
        } else {
            // We compute x - ulp(x), and the remainder ulp(x) - y satisfies: 1/2 ulp(x) < ulp(x) -
            // y < ulp(x), thus round_bit = sticky_bit = 1.
            let (a0, overflow) = x_0.overflowing_sub(shift_bit);
            let mut a1 = x_1;
            if overflow {
                a1.wrapping_sub_assign(1);
            }
            let mut a2 = x_2;
            if a1 > x_1 {
                a2.wrapping_sub_assign(1);
            }
            if a2 < HIGH_BIT {
                // - necessarily we had b = 1000...000
                // - Warning: since we have an exponent decrease, when prec = Limb::WIDTH * 3 - 1
                //   and exp_diff = Limb::WIDTH * 3, the round bit corresponds to the upper bit of
                //   -y. In that case round_bit = 0 and sticky_bit = 1, except when y = 1000...000
                //   where round_bit = 1 and sticky_bit = 0.
                // - sticky_bit = 1 below is incorrect when prec = Limb::WIDTH * 2 - 1, exp_diff =
                //   Limb::WIDTH * 2 and y = 1000...000, but in that case the even rule wound round
                //   up too.
                x_exp.saturating_sub_assign(1);
                (
                    !mask,
                    Limb::MAX,
                    Limb::MAX,
                    1,
                    Limb::from(
                        shift > 1
                            || exp_diff > THRICE_WIDTH
                            || (y_2 == HIGH_BIT && y_1 == 0 && y_0 == 0),
                    ),
                    shift_bit,
                    neg,
                )
            } else {
                (a0, a1, a2, 1, 1, shift_bit, neg)
            }
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (diff_0, diff_1, diff_2, x_exp, Equal, neg)
    } else {
        if neg {
            rm.neg_assign();
        }
        match rm {
            Exact => panic!("Inexact float subtraction"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (diff_0 & shift_bit) == 0) {
                    (diff_0, diff_1, diff_2, x_exp, Less, neg)
                } else {
                    if diff_0.overflowing_add_assign(shift_bit) {
                        diff_1.wrapping_add_assign(1);
                    }
                    if diff_1 == 0 && diff_0 == 0 {
                        diff_2.wrapping_add_assign(1);
                    }
                    if diff_2 == 0 {
                        (
                            diff_0,
                            diff_1,
                            HIGH_BIT,
                            x_exp.saturating_add(1),
                            Greater,
                            neg,
                        )
                    } else {
                        (diff_0, diff_1, diff_2, x_exp, Greater, neg)
                    }
                }
            }
            Floor | Down => (diff_0, diff_1, diff_2, x_exp, Less, neg),
            Ceiling | Up => {
                if diff_0.overflowing_add_assign(shift_bit) {
                    diff_1.wrapping_add_assign(1);
                }
                if diff_1 == 0 && diff_0 == 0 {
                    diff_2.wrapping_add_assign(1);
                }
                if diff_2 == 0 {
                    (
                        diff_0,
                        diff_1,
                        HIGH_BIT,
                        x_exp.saturating_add(1),
                        Greater,
                        neg,
                    )
                } else {
                    (diff_0, diff_1, diff_2, x_exp, Greater, neg)
                }
            }
        }
    }
}

// Equivalent to shifting xs left by 1, then calling limbs_sub_same_length_to_out.
fn limbs_sub_shl1_same_length_to_out(out: &mut [Limb], xs: &[Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(out.len() >= len);
    let mut carry = 0;
    let mut remaining_xs_bits = 0;
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        let shifted_x = (x << 1) | remaining_xs_bits;
        remaining_xs_bits = x >> WIDTH_M1;
        (*out, carry) = sub_with_carry(shifted_x, y, carry);
    }
}

// Equivalent to shifting ys right by `bits`, anding the least-significant limb with `ys0_and`, and
// then calling limbs_sub_same_length_to_out.
fn limbs_sub_shr_same_length_to_out_and_ys0(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    bits: u64,
    ys0_and: Limb,
) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(bits < Limb::WIDTH);
    assert!(out.len() >= len);
    let mut carry = 0;
    let comp_bits = Limb::WIDTH - bits;
    for i in 0..len {
        let mut shifted_y = (ys[i] >> bits) | (ys.get(i + 1).unwrap_or(&0) << comp_bits);
        if i == 0 {
            shifted_y &= ys0_and;
        }
        (out[i], carry) = sub_with_carry(xs[i], shifted_y, carry);
    }
}

// Equivalent to shifting ys right by `bits`, anding the least-significant limb with `ys0_and`, and
// then calling limbs_sub_same_length_to_out. Also allows ys to be shorter than xs, and pretends
// that the missing ys limbs are zeros.
fn limbs_sub_shr_greater_to_out_and_ys0(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    bits: u64,
    ys0_and: Limb,
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(bits < Limb::WIDTH);
    assert!(out.len() >= xs_len);
    let comp_bits = Limb::WIDTH - bits;
    let mut carry = 0;
    for i in 0..xs_len {
        let mut shifted_y = if let Some(y) = ys.get(i) {
            (y >> bits) | (ys.get(i + 1).unwrap_or(&0) << comp_bits)
        } else {
            0
        };
        if i == 0 {
            shifted_y &= ys0_and;
        }
        (out[i], carry) = sub_with_carry(xs[i], shifted_y, carry);
    }
}

// Equivalent to replacing ys[0] with something else, then calling limbs_sub_same_length_to_out.
// Also allows ys to be shorter than xs, and pretends that the missing ys limbs are zeros.
fn limbs_sub_greater_to_out_different_ys0(out: &mut [Limb], xs: &[Limb], ys: &[Limb], ys0: Limb) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(out.len() >= xs_len);
    let mut carry = 0;
    let mut first = true;
    for (out, (&x, &y)) in out.iter_mut().zip(xs.iter().zip(ys.iter())) {
        (*out, carry) = if first {
            first = false;
            sub_with_carry(x, ys0, carry)
        } else {
            sub_with_carry(x, y, carry)
        };
    }
    if carry != 0 {
        limbs_sub_limb_to_out(&mut out[ys_len..], &xs[ys_len..], 1);
    } else {
        out[ys_len..xs_len].copy_from_slice(&xs[ys_len..]);
    }
}

fn cmp_size_helper(
    xs: &[Limb],
    x_exp: i32,
    ys: &[Limb],
    y_exp: i32,
    prec: u64,
) -> (usize, usize, bool) {
    let n = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let nm1 = n - 1;
    let mut k = nm1;
    let neg = match x_exp.cmp(&y_exp) {
        Equal => {
            // Check mantissa since exponents are equal
            let mut x_y_equal = false;
            while xs[k] == ys[k] {
                if k == 0 {
                    x_y_equal = true;
                    break;
                }
                k -= 1;
            }
            // If !x_y_equal, k is the largest integer < n such that xs[k] != ys[k]
            if x_y_equal {
                return (0, 0, false);
            }
            xs[k] < ys[k]
        }
        Less => true,
        Greater => false,
    };
    (n, k, neg)
}

fn sub_float_significands_same_prec_ge_3w_ref_ref<'a>(
    out: &mut [Limb],
    mut xs: &'a [Limb],
    mut x_exp: i32,
    mut ys: &'a [Limb],
    mut y_exp: i32,
    prec: u64,
    mut rm: RoundingMode,
) -> (i32, Ordering, bool) {
    let (n, mut k, neg) = cmp_size_helper(xs, x_exp, ys, y_exp, prec);
    if n == 0 {
        // x == y. Return exact number 0. Setting the most-significant limb to 0 is a sufficient
        // signal to the caller that the entire output is 0, since in every other case the precision
        // of the output is the same as the precision of the inputs, and the most-significant limb
        // is therefore nonzero.
        *out.last_mut().unwrap() = 0;
        return (0, Equal, false);
    }
    let nm1 = n - 1;
    if neg {
        swap(&mut x_exp, &mut y_exp);
        swap(&mut xs, &mut ys);
    }
    let exp_diff = u64::exact_from(x_exp - y_exp);
    let mut round_bit;
    let mut sticky_bit;
    // round_bit_2 is the next bit after the round bit, and sticky_bit_2 the corresponding sticky
    // bit.
    let mut round_bit_2;
    let mut sticky_bit_2;
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let mut goto_exact_normalize = false;
    let mut goto_sub_d1_no_lose = false;
    let mut goto_sub_d1_lose = false;
    let mut limb = 0;
    loop {
        // loop for ExactNormalize, SubD1NoLose, and SubD1Lose
        if !goto_sub_d1_no_lose && !goto_sub_d1_lose && (exp_diff == 0 || goto_exact_normalize) {
            // ```
            // <-- x -->
            // <-- y --> : exact sub
            // ```
            if !goto_exact_normalize {
                limbs_sub_same_length_to_out(out, xs, ys);
            }
            // label ExactNormalize:
            limb = out[nm1];
            if limb != 0 {
                // First limb is not zero.
                let leading_zeros = LeadingZeros::leading_zeros(limb);
                // Warning: leading_zeros can be 0 when we come from the case SubD1Lose with
                // ExactNormalize
                if leading_zeros != 0 {
                    limbs_slice_shl_in_place(out, leading_zeros);
                    x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                }
                // Last limb should be OK
                assert_eq!(out[0] & (shift_bit - 1), 0);
            } else {
                // - First limb is zero: this can only occur for n >= 2
                // - Find the first limb not equal to zero. It necessarily exists since |x| > |y|.
                //   We know that xs[k] > ys[k] and all upper limbs are equal.
                while out[k] == 0 {
                    k -= 1;
                }
                limb = out[k];
                // out[k] is the non-zero limb of largest index, thus we have to consider the k + 1
                // least-significant limbs
                assert_ne!(limb, 0);
                let leading_zeros = LeadingZeros::leading_zeros(limb);
                k += 1;
                let len = n - k; // Number of most significant zero limbs
                assert_ne!(k, 0);
                if leading_zeros != 0 {
                    limbs_slice_shl_in_place(&mut out[..k], leading_zeros);
                }
                out.copy_within(0..k, len);
                slice_set_zero(&mut out[..len]);
                x_exp = i32::saturating_from(
                    i128::from(x_exp)
                        - (i128::from(leading_zeros)
                            + (i128::wrapping_from(len) << Limb::LOG_WIDTH)),
                );
                // out[len] should have its low bits zero: it is x[0] - y[0].
                assert_eq!(out[len] & Limb::wrapping_from(shift), 0);
            }
            // No rounding is necessary since the result is exact
            assert!(out[nm1].get_highest_bit());
            return (x_exp, Equal, neg);
        } else if exp_diff == 1 || goto_sub_d1_no_lose || goto_sub_d1_lose {
            // ```
            // | <-- x -->
            // |  <-- y -->
            // ```
            if !goto_sub_d1_no_lose && !goto_sub_d1_lose {
                // If we lose at least one bit, compute 2 * x - y (exact), else compute x - y / 2
                limb = xs[k] - (ys[k] >> 1);
            }
            // Let W = 2 ^ Limb::WIDTH: we have |x| - |y| >= limb * W ^ k - (2 * W ^ k - 1) / 2 >=
            // limb
            // * W ^ k - W ^ k + 1 / 2. Thus, if limb > W / 2, |x| - |y| >= 1 / 2 * W ^ n. Moreover,
            //   if
            // trunc(|y|) represents the first prec - 1 bits of |y|, minus the last significant bit
            // called y0 below (in fact y0 is that bit shifted by `shift` bits), then we have
            // |x|-trunc(|y|) >= 1 / 2 * W ^ n + 1, thus the two limbs_sub calls below necessarily
            // yield out > 1 / 2 * W ^ n.
            if !goto_sub_d1_lose && (limb > HIGH_BIT || goto_sub_d1_no_lose) {
                // - case limb > W / 2
                // - The exponent cannot decrease: compute x - y / 2.
                // - Shift y in the allocated temporary block
                //
                // label SubD1NoLose:
                let y0 = ys[0] & shift_bit;
                let mask = shift_bit - 1;
                // Zero last bit of y if set
                limbs_sub_shr_same_length_to_out_and_ys0(out, xs, ys, 1, !mask);
                assert!(out[nm1].get_highest_bit());
                if y0 == 0 {
                    // Result is exact: no need of rounding!
                    return (x_exp, Equal, neg);
                }
                // - y0 is non-zero, thus we have to subtract 1 / 2 * ulp(out).
                // - However, we know (see analysis above) that this cannot make the exponent
                //   decrease.
                // - Check last bits
                assert_eq!(out[0] & mask, 0);
                // - No normalization is needed
                // - Rounding is necessary since y0 is non-zero
                // - We have to subtract 1 at the round bit position, and 0 for the lower bits
                round_bit = 1;
                round_bit_2 = 0;
                sticky_bit_2 = 0;
            } else if limb < HIGH_BIT || goto_sub_d1_lose {
                // - |x| - |y| <= (W / 2 - 1) * W ^ k + W ^ k - 1 = 1 / 2 * W ^ n - 1
                // - The exponent decreases by one.
                // - Compute 2 * x - y (Exact)
                //
                // label SubD1Lose:
                goto_sub_d1_lose = false;
                limbs_sub_shl1_same_length_to_out(out, xs, ys);
                x_exp.saturating_sub_assign(1);
                assert_eq!(k, nm1);
                goto_exact_normalize = true;
                continue;
            } else {
                // - Case: limb = 100000000000
                // - Check while b[l] == y'[l] (Y' is Y shifted by 1)
                // - If x[l] < y'[l] => We lose at least one bit
                // - If x[l] > y'[l] => We don't lose any bit
                // - If l == -1 => We don't lose any bit AND the result is 100000000000 0000000000
                //   00000000000
                let mut l = n;
                let mut yl_shifted;
                loop {
                    // The first loop will compare x[n - 2] and y'[n - 2]
                    yl_shifted = ys[l - 1] << WIDTH_M1;
                    l -= 1;
                    if l == 0 {
                        break;
                    }
                    yl_shifted += ys[l - 1] >> 1;
                    if xs[l - 1] != yl_shifted {
                        break;
                    }
                }
                if l == 0 {
                    if yl_shifted != 0 {
                        // Since yl_shifted is what should be subtracted from out[-1], if non-zero
                        // then necessarily the precision is a multiple of Limb::WIDTH, and we lose
                        // one bit, thus the (exact) result is a power of 2 minus 1.
                        for o in out.iter_mut() {
                            *o = Limb::MAX;
                        }
                        x_exp.saturating_sub_assign(1);
                    } else {
                        // yl_shifted = 0: result is a power of 2.
                        let (out_last, out_init) = out.split_last_mut().unwrap();
                        slice_set_zero(out_init);
                        *out_last = HIGH_BIT;
                    }
                    // No Normalize is needed, no Rounding is needed
                    return (x_exp, Equal, neg);
                } else if xs[l - 1] > yl_shifted {
                    // - cl_shifted is the shifted value c'[l]
                    // - |x| - |y| >= 1 / 2 * W ^ n
                    //
                    // goto SubD1NoLose;
                    goto_sub_d1_no_lose = true;
                } else {
                    // We cannot have xs[l] = yl_shifted since the only way we can exit the while
                    // loop above is when xs[l] != yl_shifted or l < 0, and the case l < 0 was
                    // already treated above.
                    assert!(xs[l - 1] < yl_shifted);
                    // |x| - |y| <= 1 / 2 * W ^ n - 1 and is exact
                    goto_sub_d1_lose = true;
                }
                continue;
            }
        } else if exp_diff >= prec {
            // The difference of exponents is larger than the precision of all operands, thus the
            // result is either x or x - 1 ulp, with a possible exact result when x = prec, x = 2 ^
            // e and y = 1 / 2 * ulp(x)
            //
            // - We can't set OUT before since we use ys for rounding...
            // - Perform rounding: check if out = b or out = x - ulp(x)
            if exp_diff == prec {
                // since y is normalized, we need to subtract 1 / 2 * ulp(x)
                round_bit = 1;
                // round_bit_2 is the bit of weight 1 / 4 * ulp(x) in y. We assume a limb has at
                // least 2 bits. If the precision is 1, we read in the unused bits, which should be
                // zero, and this is what we want.
                round_bit_2 = ys[nm1] & HALF_HIGH_BIT;
                // We also need sticky_bit_2
                sticky_bit_2 = ys[nm1] & WIDTH_M2_MASK;
                let mut k = nm1;
                while sticky_bit_2 == 0 && k > 0 {
                    k -= 1;
                    sticky_bit_2 = ys[k];
                }
            } else {
                round_bit = 0;
                if exp_diff == prec + 1 {
                    round_bit_2 = 1;
                    sticky_bit_2 = ys[nm1] & WIDTH_M1_MASK;
                    let mut k = nm1;
                    while sticky_bit_2 == 0 && k > 0 {
                        k -= 1;
                        sticky_bit_2 = ys[k];
                    }
                } else {
                    round_bit_2 = 0;
                    sticky_bit_2 = 1; // since C is non-zero
                }
            }
            // Copy mantissa X in OUT
            out.copy_from_slice(xs);
        } else {
            // case 2 <= exp_diff < prec
            //
            // Compute round_bit = Cp and sticky_bit = C'p + 1
            //
            // Compute round_bit and round_bit_2 from Y The round bit is bit prec - exp_diff in Y,
            // assuming the most significant bit of Y is bit 0
            let x = prec - exp_diff;
            let mut kx = nm1 - usize::exact_from(x >> Limb::LOG_WIDTH);
            let mut sx_bit = Limb::power_of_2(WIDTH_M1 - (x & Limb::WIDTH_MASK));
            // the round bit is in ys[kx], at position sx
            assert!(prec >= exp_diff);
            round_bit = ys[kx] & sx_bit;
            // Now compute rxx: since exp_diff >= 2 it always exists in Y
            sx_bit = if sx_bit == 1 {
                // rxx is in the next limb
                kx = kx.checked_sub(1).unwrap();
                HIGH_BIT
            } else {
                // round_bit and round_bit_2 are in the same limb
                sx_bit >> 1
            };
            round_bit_2 = ys[kx] & sx_bit;
            // Now look at the remaining low bits of Y to determine sticky_bit_2
            sticky_bit_2 = ys[kx] & (sx_bit - 1);
            while sticky_bit_2 == 0 && kx > 0 {
                kx -= 1;
                sticky_bit_2 = ys[kx];
            }
            // Clean shifted Y'
            let mask = shift_bit - 1;
            let dm = exp_diff & Limb::WIDTH_MASK;
            let m = usize::exact_from(exp_diff >> Limb::LOG_WIDTH);
            if dm == 0 {
                assert_ne!(m, 0);
                // - dm = 0 and m > 0: Just copy
                // - Subtract the mantissa y from x in out
                limbs_sub_greater_to_out_different_ys0(out, xs, &ys[m..], ys[m] & !mask);
            } else if m == 0 {
                // dm >=2 and m == 0: just shift
                assert!(dm >= 2);
                // Subtract the mantissa y from x in out
                limbs_sub_shr_same_length_to_out_and_ys0(out, xs, ys, dm, !mask);
            } else {
                // - dm > 0 and m > 0: shift and zero
                // - Subtract the mantissa y from x in out
                limbs_sub_shr_greater_to_out_and_ys0(out, xs, &ys[m..], dm, !mask);
            }
            // Normalize: we lose at most one bit
            if !out[nm1].get_highest_bit() {
                // - High bit is not set and we have to fix it.
                // - OUT >= 010000xxx001
                limbs_slice_shl_in_place(out, 1);
                // OUT >= 100000xxx010
                if round_bit != 0 {
                    // - Check if Y = -1
                    // - Since Y == -1, we have to subtract one more
                    limbs_sub_limb_in_place(out, shift_bit);
                    assert!(out[nm1].get_highest_bit());
                }
                // - OUT >= 10000xxx001
                // - Final exponent -1 since we have shifted the mantissa
                x_exp.saturating_sub_assign(1);
                round_bit = round_bit_2;
                round_bit_2 = sticky_bit_2;
                // We don't have anymore a valid Yp + 1, but since Oyr >= 100000xxx001, the final
                // sub can't unnormalize.
            }
            assert_eq!(out[0] & mask, 0);
        }
        // only loop when emulating gotos
        break;
    }
    let mut out_power_of_2;
    loop {
        // At this point out contains x - high(y), normalized, and we have to subtract round_bit *
        // 1/2 ulp(out), round_bit_2 * 1/4 ulp(out), and sticky_bit_2 * 1/8 ulp(out), interpreting
        // round_bit/round_bit_2/sticky_bit_2 as 1 if non-zero.
        sticky_bit = round_bit_2 | sticky_bit_2;
        if round_bit == 0 && sticky_bit == 0 {
            return (x_exp, Equal, neg);
        }
        out_power_of_2 = limbs_is_power_of_2(out);
        if out_power_of_2 && round_bit != 0 {
            limbs_sub_limb_in_place(out, shift_bit);
            out[nm1] |= HIGH_BIT;
            x_exp.saturating_sub_assign(1);
            round_bit = round_bit_2;
            round_bit_2 = sticky_bit_2;
            sticky_bit_2 = 0;
        } else {
            break;
        }
    }
    // Now if out is a power of two, necessary round_bit = 0, which means the exact result is always
    // in (pred(out), out), and the bounds cannot be attained
    if neg {
        rm.neg_assign();
    }
    match rm {
        Exact => panic!("Inexact float subtraction"),
        Nearest => {
            if out_power_of_2 {
                assert_eq!(round_bit, 0);
                // Since we are at the end of the binade, we have in fact round_bit = round_bit_2
                // and sticky_bit = sticky_bit_2
                round_bit = round_bit_2;
                sticky_bit = sticky_bit_2;
            }
            if (prec == 1 || out[0] & shift_bit == 0 || round_bit == 0)
                && (sticky_bit == 0 || round_bit == 0)
            {
                (x_exp, Greater, neg)
            } else {
                limbs_sub_limb_in_place(out, shift_bit);
                if out_power_of_2 {
                    // deal with cancellation
                    out[nm1] |= HIGH_BIT;
                    x_exp.saturating_sub_assign(1);
                }
                (x_exp, Less, neg)
            }
        }
        Floor | Down => {
            limbs_sub_limb_in_place(out, shift_bit);
            if out_power_of_2 {
                // deal with cancellation
                out[nm1] |= HIGH_BIT;
                x_exp.saturating_sub_assign(1);
            }
            (x_exp, Less, neg)
        }
        Ceiling | Up => (x_exp, Greater, neg),
    }
}

// Equivalent to shifting xs left by 1, then calling limbs_sub_same_length_to_out.
fn limbs_sub_shl1_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    let mut carry = 0;
    let mut remaining_xs_bits = 0;
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        let shifted_x = (*x << 1) | remaining_xs_bits;
        remaining_xs_bits = *x >> WIDTH_M1;
        (*x, carry) = sub_with_carry(shifted_x, y, carry);
    }
}

// Equivalent to shifting ys right by `bits`, anding the least-significant limb with `ys0_and`, and
// then calling limbs_sub_same_length_in_place_left.
fn limbs_sub_shr_same_length_in_place_left_and_ys0(
    xs: &mut [Limb],
    ys: &[Limb],
    bits: u64,
    ys0_and: Limb,
) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(bits < Limb::WIDTH);
    let mut carry = 0;
    let comp_bits = Limb::WIDTH - bits;
    for i in 0..len {
        let mut shifted_y = (ys[i] >> bits) | (ys.get(i + 1).unwrap_or(&0) << comp_bits);
        if i == 0 {
            shifted_y &= ys0_and;
        }
        (xs[i], carry) = sub_with_carry(xs[i], shifted_y, carry);
    }
}

// Equivalent to shifting ys right by `bits`, anding the least-significant limb with `ys0_and`, and
// then calling limbs_sub_same_length_in_place_left. Also allows ys to be shorter than xs, and
// pretends that the missing ys limbs are zeros.
fn limbs_sub_shr_greater_in_place_left_and_ys0(
    xs: &mut [Limb],
    ys: &[Limb],
    bits: u64,
    ys0_and: Limb,
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    assert!(bits < Limb::WIDTH);
    let comp_bits = Limb::WIDTH - bits;
    let mut carry = 0;
    for (i, x) in xs.iter_mut().enumerate() {
        let mut shifted_y = if let Some(y) = ys.get(i) {
            (y >> bits) | (ys.get(i + 1).unwrap_or(&0) << comp_bits)
        } else {
            0
        };
        if i == 0 {
            shifted_y &= ys0_and;
        }
        (*x, carry) = sub_with_carry(*x, shifted_y, carry);
    }
}

// Equivalent to replacing ys[0] with something else, then calling
// limbs_sub_same_length_in_place_left. Also allows ys to be shorter than xs, and pretends that the
// missing ys limbs are zeros.
fn limbs_sub_greater_in_place_left_different_ys0(xs: &mut [Limb], ys: &[Limb], ys0: Limb) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    assert!(xs_len >= ys_len);
    let mut carry = 0;
    let mut first = true;
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        (*x, carry) = if first {
            first = false;
            sub_with_carry(*x, ys0, carry)
        } else {
            sub_with_carry(*x, y, carry)
        };
    }
    if carry != 0 {
        limbs_sub_limb_in_place(&mut xs[ys_len..], 1);
    }
}

fn sub_float_significands_same_prec_ge_3w_val_val<'a>(
    mut xs: &'a mut [Limb],
    mut x_exp: i32,
    mut ys: &'a mut [Limb],
    mut y_exp: i32,
    prec: u64,
    mut rm: RoundingMode,
) -> (i32, Ordering, bool) {
    let (n, _, neg) = cmp_size_helper(xs, x_exp, ys, y_exp, prec);
    if n == 0 {
        // x == y. Return exact number 0. Setting the most-significant limb to 0 is a sufficient
        // signal to the caller that the entire output is 0, since in every other case the precision
        // of the output is the same as the precision of the inputs, and the most-significant limb
        // is therefore nonzero.
        *xs.last_mut().unwrap() = 0;
        return (0, Equal, false);
    }
    if neg {
        rm.neg_assign();
        swap(&mut x_exp, &mut y_exp);
        swap(&mut xs, &mut ys);
    }
    let (exp, o) =
        sub_float_significands_same_prec_ge_3w_val_ref_helper(xs, x_exp, ys, y_exp, prec, rm);
    (exp, o, neg)
}

fn sub_float_significands_same_prec_ge_3w_val_ref(
    xs: &mut [Limb],
    x_exp: i32,
    ys: &[Limb],
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering, bool) {
    let (n, _, neg) = cmp_size_helper(xs, x_exp, ys, y_exp, prec);
    if n == 0 {
        // x == y. Return exact number 0. Setting the most-significant limb to 0 is a sufficient
        // signal to the caller that the entire output is 0, since in every other case the precision
        // of the output is the same as the precision of the inputs, and the most-significant limb
        // is therefore nonzero.
        *xs.last_mut().unwrap() = 0;
        return (0, Equal, false);
    }
    if neg {
        let (exp, o) =
            sub_float_significands_same_prec_ge_3w_ref_val_helper(ys, y_exp, xs, x_exp, prec, -rm);
        (exp, o, true)
    } else {
        let (exp, o) =
            sub_float_significands_same_prec_ge_3w_val_ref_helper(xs, x_exp, ys, y_exp, prec, rm);
        (exp, o, false)
    }
}

fn sub_float_significands_same_prec_ge_3w_val_ref_helper(
    xs: &mut [Limb],
    mut x_exp: i32,
    ys: &[Limb],
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    let n = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let nm1 = n - 1;
    let mut k = nm1;
    let exp_diff = u64::exact_from(x_exp - y_exp);
    let mut round_bit;
    let mut sticky_bit;
    // round_bit_2 is the next bit after the round bit, and sticky_bit_2 the corresponding sticky
    // bit.
    let mut round_bit_2;
    let mut sticky_bit_2;
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let mut goto_exact_normalize = false;
    let mut goto_sub_d1_no_lose = false;
    let mut goto_sub_d1_lose = false;
    let mut limb = 0;
    loop {
        // loop for ExactNormalize, SubD1NoLose, and SubD1Lose
        if !goto_sub_d1_no_lose && !goto_sub_d1_lose && (exp_diff == 0 || goto_exact_normalize) {
            // ```
            // <-- x -->
            // <-- y --> : exact sub
            // ```
            if !goto_exact_normalize {
                limbs_sub_same_length_in_place_left(xs, ys);
            }
            // label ExactNormalize:
            limb = xs[nm1];
            if limb != 0 {
                // First limb is not zero.
                let leading_zeros = LeadingZeros::leading_zeros(limb);
                // Warning: leading_zeros can be 0 when we come from the case SubD1Lose with
                // ExactNormalize
                if leading_zeros != 0 {
                    limbs_slice_shl_in_place(xs, leading_zeros);
                    x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                }
                // Last limb should be OK
                assert_eq!(xs[0] & (shift_bit - 1), 0);
            } else {
                // - First limb is zero: this can only occur for n >= 2
                // - Find the first limb not equal to zero. It necessarily exists since |x| > |y|.
                //   We know that xs[k] > ys[k] and all upper limbs are equal.
                while xs[k] == 0 {
                    k -= 1;
                }
                limb = xs[k];
                // out[k] is the non-zero limb of largest index, thus we have to consider the k + 1
                // least-significant limbs
                assert_ne!(limb, 0);
                let leading_zeros = LeadingZeros::leading_zeros(limb);
                k += 1;
                let len = n - k; // Number of most significant zero limbs
                assert_ne!(k, 0);
                if leading_zeros != 0 {
                    limbs_slice_shl_in_place(&mut xs[..k], leading_zeros);
                }
                xs.copy_within(0..k, len);
                slice_set_zero(&mut xs[..len]);
                x_exp = i32::saturating_from(
                    i128::from(x_exp)
                        - (i128::from(leading_zeros)
                            + (i128::wrapping_from(len) << Limb::LOG_WIDTH)),
                );
                // out[len] should have its low bits zero: it is x[0] - y[0].
                assert_eq!(xs[len] & Limb::wrapping_from(shift), 0);
            }
            // No rounding is necessary since the result is exact
            assert!(xs[nm1].get_highest_bit());
            return (x_exp, Equal);
        } else if exp_diff == 1 || goto_sub_d1_no_lose || goto_sub_d1_lose {
            // ```
            // | <-- x -->
            // |  <-- y -->
            // ```
            if !goto_sub_d1_no_lose && !goto_sub_d1_lose {
                // If we lose at least one bit, compute 2 * x - y (exact), else compute x - y / 2
                limb = xs[k] - (ys[k] >> 1);
            }
            // Let W = 2 ^ Limb::WIDTH: we have |x| - |y| >= limb * W ^ k - (2 * W ^ k - 1) / 2 >=
            // limb
            // * W ^ k - W ^ k + 1 / 2. Thus, if limb > W / 2, |x| - |y| >= 1 / 2 * W ^ n. Moreover,
            //   if
            // trunc(|y|) represents the first prec - 1 bits of |y|, minus the last significant bit
            // called y0 below (in fact y0 is that bit shifted by `shift` bits), then we have
            // |x|-trunc(|y|) >= 1 / 2 * W ^ n + 1, thus the two limbs_sub calls below necessarily
            // yield out > 1 / 2 * W ^ n.
            if !goto_sub_d1_lose && (limb > HIGH_BIT || goto_sub_d1_no_lose) {
                // - case limb > W / 2
                // - The exponent cannot decrease: compute x - y / 2.
                // - Shift y in the allocated temporary block
                //
                // label SubD1NoLose:
                let y0 = ys[0] & shift_bit;
                let mask = shift_bit - 1;
                // Zero last bit of y if set
                limbs_sub_shr_same_length_in_place_left_and_ys0(xs, ys, 1, !mask);
                assert!(xs[nm1].get_highest_bit());
                if y0 == 0 {
                    // Result is exact: no need of rounding!
                    return (x_exp, Equal);
                }
                // - y0 is non-zero, thus we have to subtract 1 / 2 * ulp(out).
                // - However, we know (see analysis above) that this cannot make the exponent
                //   decrease.
                // - Check last bits
                assert_eq!(xs[0] & mask, 0);
                // - No normalization is needed
                // - Rounding is necessary since y0 is non-zero
                // - We have to subtract 1 at the round bit position, and 0 for the lower bits
                round_bit = 1;
                round_bit_2 = 0;
                sticky_bit_2 = 0;
            } else if limb < HIGH_BIT || goto_sub_d1_lose {
                // - |x| - |y| <= (W / 2 - 1) * W ^ k + W ^ k - 1 = 1 / 2 * W ^ n - 1
                // - The exponent decreases by one.
                // - Compute 2 * x - y (Exact)
                //
                // label SubD1Lose:
                goto_sub_d1_lose = false;
                limbs_sub_shl1_same_length_in_place_left(xs, ys);
                x_exp.saturating_sub_assign(1);
                assert_eq!(k, nm1);
                goto_exact_normalize = true;
                continue;
            } else {
                // - Case: limb = 100000000000
                // - Check while b[l] == y'[l] (Y' is Y shifted by 1)
                // - If x[l] < y'[l] => We lose at least one bit
                // - If x[l] > y'[l] => We don't lose any bit
                // - If l == -1 => We don't lose any bit AND the result is 100000000000 0000000000
                //   00000000000
                let mut l = n;
                let mut yl_shifted;
                loop {
                    // The first loop will compare x[n - 2] and y'[n - 2]
                    yl_shifted = ys[l - 1] << WIDTH_M1;
                    l -= 1;
                    if l == 0 {
                        break;
                    }
                    yl_shifted += ys[l - 1] >> 1;
                    if xs[l - 1] != yl_shifted {
                        break;
                    }
                }
                if l == 0 {
                    if yl_shifted != 0 {
                        // Since yl_shifted is what should be subtracted from out[-1], if non-zero
                        // then necessarily the precision is a multiple of Limb::WIDTH, and we lose
                        // one bit, thus the (exact) result is a power of 2 minus 1.
                        for x in xs.iter_mut() {
                            *x = Limb::MAX;
                        }
                        x_exp.saturating_sub_assign(1);
                    } else {
                        // yl_shifted = 0: result is a power of 2.
                        let (xs_last, xs_init) = xs.split_last_mut().unwrap();
                        slice_set_zero(xs_init);
                        *xs_last = HIGH_BIT;
                    }
                    // No Normalize is needed, no Rounding is needed
                    return (x_exp, Equal);
                } else if xs[l - 1] > yl_shifted {
                    // - cl_shifted is the shifted value c'[l]
                    // - |x| - |y| >= 1 / 2 * W ^ n
                    //
                    // goto SubD1NoLose;
                    goto_sub_d1_no_lose = true;
                } else {
                    // We cannot have xs[l] = yl_shifted since the only way we can exit the while
                    // loop above is when xs[l] != yl_shifted or l < 0, and the case l < 0 was
                    // already treated above.
                    assert!(xs[l - 1] < yl_shifted);
                    // |x| - |y| <= 1 / 2 * W ^ n - 1 and is exact
                    goto_sub_d1_lose = true;
                }
                continue;
            }
        } else if exp_diff >= prec {
            // The difference of exponents is larger than the precision of all operands, thus the
            // result is either x or x - 1 ulp, with a possible exact result when x = prec, x = 2 ^
            // e and y = 1 / 2 * ulp(x)
            //
            // - We can't set OUT before since we use ys for rounding...
            // - Perform rounding: check if out = b or out = x - ulp(x)
            if exp_diff == prec {
                // since y is normalized, we need to subtract 1 / 2 * ulp(x)
                round_bit = 1;
                // round_bit_2 is the bit of weight 1 / 4 * ulp(x) in y. We assume a limb has at
                // least 2 bits. If the precision is 1, we read in the unused bits, which should be
                // zero, and this is what we want.
                round_bit_2 = ys[nm1] & HALF_HIGH_BIT;
                // We also need sticky_bit_2
                sticky_bit_2 = ys[nm1] & WIDTH_M2_MASK;
                let mut k = nm1;
                while sticky_bit_2 == 0 && k > 0 {
                    k -= 1;
                    sticky_bit_2 = ys[k];
                }
            } else {
                round_bit = 0;
                if exp_diff == prec + 1 {
                    round_bit_2 = 1;
                    sticky_bit_2 = ys[nm1] & WIDTH_M1_MASK;
                    let mut k = nm1;
                    while sticky_bit_2 == 0 && k > 0 {
                        k -= 1;
                        sticky_bit_2 = ys[k];
                    }
                } else {
                    round_bit_2 = 0;
                    sticky_bit_2 = 1; // since C is non-zero
                }
            }
        } else {
            // case 2 <= exp_diff < prec
            //
            // Compute round_bit = Cp and sticky_bit = C'p + 1
            //
            // Compute round_bit and round_bit_2 from Y The round bit is bit prec - exp_diff in Y,
            // assuming the most significant bit of Y is bit 0
            let x = prec - exp_diff;
            let mut kx = nm1 - usize::exact_from(x >> Limb::LOG_WIDTH);
            let mut sx_bit = Limb::power_of_2(WIDTH_M1 - (x & Limb::WIDTH_MASK));
            // the round bit is in ys[kx], at position sx
            assert!(prec >= exp_diff);
            round_bit = ys[kx] & sx_bit;
            // Now compute rxx: since exp_diff >= 2 it always exists in Y
            sx_bit = if sx_bit == 1 {
                // rxx is in the next limb
                kx = kx.checked_sub(1).unwrap();
                HIGH_BIT
            } else {
                // round_bit and round_bit_2 are in the same limb
                sx_bit >> 1
            };
            round_bit_2 = ys[kx] & sx_bit;
            // Now look at the remaining low bits of Y to determine sticky_bit_2
            sticky_bit_2 = ys[kx] & (sx_bit - 1);
            while sticky_bit_2 == 0 && kx > 0 {
                kx -= 1;
                sticky_bit_2 = ys[kx];
            }
            // Clean shifted Y'
            let mask = shift_bit - 1;
            let dm = exp_diff & Limb::WIDTH_MASK;
            let m = usize::exact_from(exp_diff >> Limb::LOG_WIDTH);
            if dm == 0 {
                assert_ne!(m, 0);
                // - dm = 0 and m > 0: Just copy
                // - Subtract the mantissa y from x in out
                limbs_sub_greater_in_place_left_different_ys0(xs, &ys[m..], ys[m] & !mask);
            } else if m == 0 {
                // dm >=2 and m == 0: just shift
                assert!(dm >= 2);
                // Subtract the mantissa y from x in out
                limbs_sub_shr_same_length_in_place_left_and_ys0(xs, ys, dm, !mask);
            } else {
                // - dm > 0 and m > 0: shift and zero
                // - Subtract the mantissa y from x in out
                limbs_sub_shr_greater_in_place_left_and_ys0(xs, &ys[m..], dm, !mask);
            }
            // Normalize: we lose at most one bit
            if !xs[nm1].get_highest_bit() {
                // - High bit is not set and we have to fix it.
                // - OUT >= 010000xxx001
                limbs_slice_shl_in_place(xs, 1);
                // OUT >= 100000xxx010
                if round_bit != 0 {
                    // - Check if Y = -1
                    // - Since Y == -1, we have to subtract one more
                    limbs_sub_limb_in_place(xs, shift_bit);
                    assert!(xs[nm1].get_highest_bit());
                }
                // - OUT >= 10000xxx001
                // - Final exponent -1 since we have shifted the mantissa
                x_exp.saturating_sub_assign(1);
                round_bit = round_bit_2;
                round_bit_2 = sticky_bit_2;
                // We don't have anymore a valid Yp + 1, but since Oyr >= 100000xxx001, the final
                // sub can't unnormalize.
            }
            assert_eq!(xs[0] & mask, 0);
        }
        // only loop when emulating gotos
        break;
    }
    let mut out_power_of_2;
    loop {
        // At this point out contains x - high(y), normalized, and we have to subtract round_bit *
        // 1/2 ulp(out), round_bit_2 * 1/4 ulp(out), and sticky_bit_2 * 1/8 ulp(out), interpreting
        // round_bit/round_bit_2/sticky_bit_2 as 1 if non-zero.
        sticky_bit = round_bit_2 | sticky_bit_2;
        if round_bit == 0 && sticky_bit == 0 {
            return (x_exp, Equal);
        }
        out_power_of_2 = limbs_is_power_of_2(xs);
        if out_power_of_2 && round_bit != 0 {
            limbs_sub_limb_in_place(xs, shift_bit);
            xs[nm1] |= HIGH_BIT;
            x_exp.saturating_sub_assign(1);
            round_bit = round_bit_2;
            round_bit_2 = sticky_bit_2;
            sticky_bit_2 = 0;
        } else {
            break;
        }
    }
    // Now if out is a power of two, necessary round_bit = 0, which means the exact result is always
    // in (pred(xs), xs), and the bounds cannot be attained
    match rm {
        Exact => panic!("Inexact float subtraction"),
        Nearest => {
            if out_power_of_2 {
                assert_eq!(round_bit, 0);
                // Since we are at the end of the binade, we have in fact round_bit = round_bit_2
                // and sticky_bit = sticky_bit_2
                round_bit = round_bit_2;
                sticky_bit = sticky_bit_2;
            }
            if (prec == 1 || xs[0] & shift_bit == 0 || round_bit == 0)
                && (sticky_bit == 0 || round_bit == 0)
            {
                (x_exp, Greater)
            } else {
                limbs_sub_limb_in_place(xs, shift_bit);
                if out_power_of_2 {
                    // deal with cancellation
                    xs[nm1] |= HIGH_BIT;
                    x_exp.saturating_sub_assign(1);
                }
                (x_exp, Less)
            }
        }
        Floor | Down => {
            limbs_sub_limb_in_place(xs, shift_bit);
            if out_power_of_2 {
                // deal with cancellation
                xs[nm1] |= HIGH_BIT;
                x_exp.saturating_sub_assign(1);
            }
            (x_exp, Less)
        }
        Ceiling | Up => (x_exp, Greater),
    }
}

// Equivalent to shifting xs left by 1, then calling limbs_sub_same_length_in_place_right.
fn limbs_sub_shl1_same_length_in_place_right(xs: &[Limb], ys: &mut [Limb]) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    let mut carry = 0;
    let mut remaining_xs_bits = 0;
    for (&x, y) in xs.iter().zip(ys.iter_mut()) {
        let shifted_x = (x << 1) | remaining_xs_bits;
        remaining_xs_bits = x >> WIDTH_M1;
        (*y, carry) = sub_with_carry(shifted_x, *y, carry);
    }
}

// Equivalent to shifting ys right by `bits`, anding the least-significant limb with `ys0_and`, and
// then calling limbs_sub_same_length_in_place_right.
fn limbs_sub_shr_same_length_in_place_right_and_ys0(
    xs: &[Limb],
    ys: &mut [Limb],
    bits: u64,
    ys0_and: Limb,
) {
    let len = xs.len();
    assert_eq!(len, ys.len());
    assert!(bits < Limb::WIDTH);
    let mut carry = 0;
    let comp_bits = Limb::WIDTH - bits;
    for i in 0..len {
        let mut shifted_y = (ys[i] >> bits) | (ys.get(i + 1).unwrap_or(&0) << comp_bits);
        if i == 0 {
            shifted_y &= ys0_and;
        }
        (ys[i], carry) = sub_with_carry(xs[i], shifted_y, carry);
    }
}

fn limbs_sub_shr_greater_in_place_right_and_ys0(
    xs: &[Limb],
    ys: &mut [Limb],
    bits: u64,
    ys0_and: Limb,
    m: usize,
) {
    let n = xs.len();
    assert_ne!(n, 0);
    assert!(bits < Limb::WIDTH);
    let comp_bits = Limb::WIDTH - bits;
    let mut carry = 0;
    for i in 0..n {
        let mut shifted_y = if let Some(y) = ys.get(i + m) {
            (y >> bits) | (ys.get(i + m + 1).unwrap_or(&0) << comp_bits)
        } else {
            0
        };
        if i == 0 {
            shifted_y &= ys0_and;
        }
        (ys[i], carry) = sub_with_carry(xs[i], shifted_y, carry);
    }
}

fn limbs_sub_greater_in_place_right_different_ys0(
    xs: &[Limb],
    ys: &mut [Limb],
    ys0: Limb,
    m: usize,
) {
    let mut carry = 0;
    for i in 0..xs.len() {
        (ys[i], carry) = sub_with_carry(
            xs[i],
            if i == 0 {
                ys0
            } else {
                ys.get(i + m).copied().unwrap_or(0)
            },
            carry,
        );
    }
}

fn sub_float_significands_same_prec_ge_3w_ref_val_helper(
    xs: &[Limb],
    mut x_exp: i32,
    ys: &mut [Limb],
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    let n = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    let nm1 = n - 1;
    let mut k = nm1;
    let exp_diff = u64::exact_from(x_exp - y_exp);
    let mut round_bit;
    let mut sticky_bit;
    // round_bit_2 is the next bit after the round bit, and sticky_bit_2 the corresponding sticky
    // bit.
    let mut round_bit_2;
    let mut sticky_bit_2;
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let mut goto_exact_normalize = false;
    let mut goto_sub_d1_no_lose = false;
    let mut goto_sub_d1_lose = false;
    let mut limb = 0;
    loop {
        // loop for ExactNormalize, SubD1NoLose, and SubD1Lose
        if !goto_sub_d1_no_lose && !goto_sub_d1_lose && (exp_diff == 0 || goto_exact_normalize) {
            // ```
            // <-- x -->
            // <-- y --> : exact sub
            // ```
            if !goto_exact_normalize {
                limbs_sub_same_length_in_place_right(xs, ys);
            }
            // label ExactNormalize:
            limb = ys[nm1];
            if limb != 0 {
                // First limb is not zero.
                let leading_zeros = LeadingZeros::leading_zeros(limb);
                // Warning: leading_zeros can be 0 when we come from the case SubD1Lose with
                // ExactNormalize
                if leading_zeros != 0 {
                    limbs_slice_shl_in_place(ys, leading_zeros);
                    x_exp.saturating_sub_assign(i32::wrapping_from(leading_zeros));
                }
                // Last limb should be OK
                assert_eq!(ys[0] & (shift_bit - 1), 0);
            } else {
                // - First limb is zero: this can only occur for n >= 2
                // - Find the first limb not equal to zero. It necessarily exists since |x| > |y|.
                //   We know that xs[k] > ys[k] and all upper limbs are equal.
                while ys[k] == 0 {
                    k -= 1;
                }
                limb = ys[k];
                // out[k] is the non-zero limb of largest index, thus we have to consider the k + 1
                // least-significant limbs
                assert_ne!(limb, 0);
                let leading_zeros = LeadingZeros::leading_zeros(limb);
                k += 1;
                let len = n - k; // Number of most significant zero limbs
                assert_ne!(k, 0);
                if leading_zeros != 0 {
                    limbs_slice_shl_in_place(&mut ys[..k], leading_zeros);
                }
                ys.copy_within(0..k, len);
                slice_set_zero(&mut ys[..len]);
                x_exp = i32::saturating_from(
                    i128::from(x_exp)
                        - (i128::from(leading_zeros)
                            + (i128::wrapping_from(len) << Limb::LOG_WIDTH)),
                );
                // out[len] should have its low bits zero: it is x[0] - y[0].
                assert_eq!(ys[len] & Limb::wrapping_from(shift), 0);
            }
            // No rounding is necessary since the result is exact
            assert!(ys[nm1].get_highest_bit());
            return (x_exp, Equal);
        } else if exp_diff == 1 || goto_sub_d1_no_lose || goto_sub_d1_lose {
            // ```
            // | <-- x -->
            // |  <-- y -->
            // ```
            if !goto_sub_d1_no_lose && !goto_sub_d1_lose {
                // If we lose at least one bit, compute 2 * x - y (exact), else compute x - y / 2
                limb = xs[k] - (ys[k] >> 1);
            }
            // Let W = 2 ^ Limb::WIDTH: we have |x| - |y| >= limb * W ^ k - (2 * W ^ k - 1) / 2 >=
            // limb
            // * W ^ k - W ^ k + 1 / 2. Thus, if limb > W / 2, |x| - |y| >= 1 / 2 * W ^ n. Moreover,
            //   if
            // trunc(|y|) represents the first prec - 1 bits of |y|, minus the last significant bit
            // called y0 below (in fact y0 is that bit shifted by `shift` bits), then we have
            // |x|-trunc(|y|) >= 1 / 2 * W ^ n + 1, thus the two limbs_sub calls below necessarily
            // yield out > 1 / 2 * W ^ n.
            if !goto_sub_d1_lose && (limb > HIGH_BIT || goto_sub_d1_no_lose) {
                // - case limb > W / 2
                // - The exponent cannot decrease: compute x - y / 2.
                // - Shift y in the allocated temporary block
                //
                // label SubD1NoLose:
                let y0 = ys[0] & shift_bit;
                let mask = shift_bit - 1;
                // Zero last bit of y if set
                limbs_sub_shr_same_length_in_place_right_and_ys0(xs, ys, 1, !mask);
                assert!(ys[nm1].get_highest_bit());
                if y0 == 0 {
                    // Result is exact: no need of rounding!
                    return (x_exp, Equal);
                }
                // - y0 is non-zero, thus we have to subtract 1 / 2 * ulp(out).
                // - However, we know (see analysis above) that this cannot make the exponent
                //   decrease.
                // - Check last bits
                assert_eq!(ys[0] & mask, 0);
                // - No normalization is needed
                // - Rounding is necessary since y0 is non-zero
                // - We have to subtract 1 at the round bit position, and 0 for the lower bits
                round_bit = 1;
                round_bit_2 = 0;
                sticky_bit_2 = 0;
            } else if limb < HIGH_BIT || goto_sub_d1_lose {
                // - |x| - |y| <= (W / 2 - 1) * W ^ k + W ^ k - 1 = 1 / 2 * W ^ n - 1
                // - The exponent decreases by one.
                // - Compute 2 * x - y (Exact)
                //
                // label SubD1Lose:
                goto_sub_d1_lose = false;
                limbs_sub_shl1_same_length_in_place_right(xs, ys);
                x_exp.saturating_sub_assign(1);
                assert_eq!(k, nm1);
                goto_exact_normalize = true;
                continue;
            } else {
                // - Case: limb = 100000000000
                // - Check while b[l] == y'[l] (Y' is Y shifted by 1)
                // - If x[l] < y'[l] => We lose at least one bit
                // - If x[l] > y'[l] => We don't lose any bit
                // - If l == -1 => We don't lose any bit AND the result is 100000000000 0000000000
                //   00000000000
                let mut l = n;
                let mut yl_shifted;
                loop {
                    // The first loop will compare x[n - 2] and y'[n - 2]
                    yl_shifted = ys[l - 1] << WIDTH_M1;
                    l -= 1;
                    if l == 0 {
                        break;
                    }
                    yl_shifted += ys[l - 1] >> 1;
                    if xs[l - 1] != yl_shifted {
                        break;
                    }
                }
                if l == 0 {
                    if yl_shifted != 0 {
                        // Since yl_shifted is what should be subtracted from out[-1], if non-zero
                        // then necessarily the precision is a multiple of Limb::WIDTH, and we lose
                        // one bit, thus the (exact) result is a power of 2 minus 1.
                        for o in ys.iter_mut() {
                            *o = Limb::MAX;
                        }
                        x_exp.saturating_sub_assign(1);
                    } else {
                        // yl_shifted = 0: result is a power of 2.
                        let (ys_last, ys_init) = ys.split_last_mut().unwrap();
                        slice_set_zero(ys_init);
                        *ys_last = HIGH_BIT;
                    }
                    // No Normalize is needed, no Rounding is needed
                    return (x_exp, Equal);
                } else if xs[l - 1] > yl_shifted {
                    // - cl_shifted is the shifted value c'[l]
                    // - |x| - |y| >= 1 / 2 * W ^ n
                    //
                    // goto SubD1NoLose;
                    goto_sub_d1_no_lose = true;
                } else {
                    // We cannot have xs[l] = yl_shifted since the only way we can exit the while
                    // loop above is when xs[l] != yl_shifted or l < 0, and the case l < 0 was
                    // already treated above.
                    assert!(xs[l - 1] < yl_shifted);
                    // |x| - |y| <= 1 / 2 * W ^ n - 1 and is exact
                    goto_sub_d1_lose = true;
                }
                continue;
            }
        } else if exp_diff >= prec {
            // The difference of exponents is larger than the precision of all operands, thus the
            // result is either x or x - 1 ulp, with a possible exact result when x = prec, x = 2 ^
            // e and y = 1 / 2 * ulp(x)
            //
            // - We can't set OUT before since we use ys for rounding...
            // - Perform rounding: check if out = b or out = x - ulp(x)
            if exp_diff == prec {
                // since y is normalized, we need to subtract 1 / 2 * ulp(x)
                round_bit = 1;
                // round_bit_2 is the bit of weight 1 / 4 * ulp(x) in y. We assume a limb has at
                // least 2 bits. If the precision is 1, we read in the unused bits, which should be
                // zero, and this is what we want.
                round_bit_2 = ys[nm1] & HALF_HIGH_BIT;
                // We also need sticky_bit_2
                sticky_bit_2 = ys[nm1] & WIDTH_M2_MASK;
                let mut k = nm1;
                while sticky_bit_2 == 0 && k > 0 {
                    k -= 1;
                    sticky_bit_2 = ys[k];
                }
            } else {
                round_bit = 0;
                if exp_diff == prec + 1 {
                    round_bit_2 = 1;
                    sticky_bit_2 = ys[nm1] & WIDTH_M1_MASK;
                    let mut k = nm1;
                    while sticky_bit_2 == 0 && k > 0 {
                        k -= 1;
                        sticky_bit_2 = ys[k];
                    }
                } else {
                    round_bit_2 = 0;
                    sticky_bit_2 = 1; // since C is non-zero
                }
            }
            // Copy mantissa X to Y
            ys.copy_from_slice(xs);
        } else {
            // case 2 <= exp_diff < prec
            //
            // Compute round_bit = Cp and sticky_bit = C'p + 1
            //
            // Compute round_bit and round_bit_2 from Y The round bit is bit prec - exp_diff in Y,
            // assuming the most significant bit of Y is bit 0
            let x = prec - exp_diff;
            let mut kx = nm1 - usize::exact_from(x >> Limb::LOG_WIDTH);
            let mut sx_bit = Limb::power_of_2(WIDTH_M1 - (x & Limb::WIDTH_MASK));
            // the round bit is in ys[kx], at position sx
            assert!(prec >= exp_diff);
            round_bit = ys[kx] & sx_bit;
            // Now compute rxx: since exp_diff >= 2 it always exists in Y
            sx_bit = if sx_bit == 1 {
                // rxx is in the next limb
                kx = kx.checked_sub(1).unwrap();
                HIGH_BIT
            } else {
                // round_bit and round_bit_2 are in the same limb
                sx_bit >> 1
            };
            round_bit_2 = ys[kx] & sx_bit;
            // Now look at the remaining low bits of Y to determine sticky_bit_2
            sticky_bit_2 = ys[kx] & (sx_bit - 1);
            while sticky_bit_2 == 0 && kx > 0 {
                kx -= 1;
                sticky_bit_2 = ys[kx];
            }
            // Clean shifted Y'
            let mask = shift_bit - 1;
            let dm = exp_diff & Limb::WIDTH_MASK;
            let m = usize::exact_from(exp_diff >> Limb::LOG_WIDTH);
            if dm == 0 {
                assert_ne!(m, 0);
                // - dm = 0 and m > 0: Just copy
                // - Subtract the mantissa y from x in out
                limbs_sub_greater_in_place_right_different_ys0(xs, ys, ys[m] & !mask, m);
            } else if m == 0 {
                // dm >=2 and m == 0: just shift
                assert!(dm >= 2);
                // Subtract the mantissa y from x in out
                limbs_sub_shr_same_length_in_place_right_and_ys0(xs, ys, dm, !mask);
            } else {
                // - dm > 0 and m > 0: shift and zero
                // - Subtract the mantissa y from x in out
                limbs_sub_shr_greater_in_place_right_and_ys0(xs, ys, dm, !mask, m);
            }
            // Normalize: we lose at most one bit
            if !ys[nm1].get_highest_bit() {
                // - High bit is not set and we have to fix it.
                // - OUT >= 010000xxx001
                limbs_slice_shl_in_place(ys, 1);
                // OUT >= 100000xxx010
                if round_bit != 0 {
                    // - Check if Y = -1
                    // - Since Y == -1, we have to subtract one more
                    limbs_sub_limb_in_place(ys, shift_bit);
                    assert!(ys[nm1].get_highest_bit());
                }
                // - OUT >= 10000xxx001
                // - Final exponent -1 since we have shifted the mantissa
                x_exp.saturating_sub_assign(1);
                round_bit = round_bit_2;
                round_bit_2 = sticky_bit_2;
                // We don't have anymore a valid Yp + 1, but since Oyr >= 100000xxx001, the final
                // sub can't unnormalize.
            }
            assert_eq!(ys[0] & mask, 0);
        }
        // only loop when emulating gotos
        break;
    }
    let mut ys_power_of_2;
    loop {
        // At this point out contains x - high(y), normalized, and we have to subtract round_bit *
        // 1/2 ulp(out), round_bit_2 * 1/4 ulp(out), and sticky_bit_2 * 1/8 ulp(out), interpreting
        // round_bit/round_bit_2/sticky_bit_2 as 1 if non-zero.
        sticky_bit = round_bit_2 | sticky_bit_2;
        if round_bit == 0 && sticky_bit == 0 {
            return (x_exp, Equal);
        }
        ys_power_of_2 = limbs_is_power_of_2(ys);
        if ys_power_of_2 && round_bit != 0 {
            limbs_sub_limb_in_place(ys, shift_bit);
            ys[nm1] |= HIGH_BIT;
            x_exp.saturating_sub_assign(1);
            round_bit = round_bit_2;
            round_bit_2 = sticky_bit_2;
            sticky_bit_2 = 0;
        } else {
            break;
        }
    }
    // Now if out is a power of two, necessary round_bit = 0, which means the exact result is always
    // in (pred(ys), ys), and the bounds cannot be attained
    match rm {
        Exact => panic!("Inexact float subtraction"),
        Nearest => {
            if ys_power_of_2 {
                assert_eq!(round_bit, 0);
                // Since we are at the end of the binade, we have in fact round_bit = round_bit_2
                // and sticky_bit = sticky_bit_2
                round_bit = round_bit_2;
                sticky_bit = sticky_bit_2;
            }
            if (prec == 1 || ys[0] & shift_bit == 0 || round_bit == 0)
                && (sticky_bit == 0 || round_bit == 0)
            {
                (x_exp, Greater)
            } else {
                limbs_sub_limb_in_place(ys, shift_bit);
                if ys_power_of_2 {
                    // deal with cancellation
                    ys[nm1] |= HIGH_BIT;
                    x_exp.saturating_sub_assign(1);
                }
                (x_exp, Less)
            }
        }
        Floor | Down => {
            limbs_sub_limb_in_place(ys, shift_bit);
            if ys_power_of_2 {
                // deal with cancellation
                ys[nm1] |= HIGH_BIT;
                x_exp.saturating_sub_assign(1);
            }
            (x_exp, Less)
        }
        Ceiling | Up => (x_exp, Greater),
    }
}

// This is mpfr_cmp2 from cmp2.c, MPFR 4.2.0, returning `cancel` along with `sign`.
fn exponent_shift_compare<'a>(
    mut xs: &'a [Limb],
    mut x_exp: i32,
    mut x_prec: u64,
    mut ys: &'a [Limb],
    mut y_exp: i32,
    mut y_prec: u64,
) -> (Ordering, u64) {
    // x == y should not happen, since cmp2 is called only from agm (with different variables) and
    // from sub1 (if b=c, then sub1sp would be called instead). So, no need for a particular
    // optimization here.
    //
    // the cases b=0 or c=0 are also treated apart in agm and sub (which calls sub1)
    let sdiff_exp = i64::from(x_exp) - i64::from(y_exp);
    let mut sign;
    let mut diff_exp;
    // index of the most significant limb of x and y
    let mut xi = usize::exact_from((x_prec - 1) >> Limb::LOG_WIDTH);
    let mut yi = usize::exact_from((y_prec - 1) >> Limb::LOG_WIDTH);
    let mut xi_done = false;
    let mut yi_done = false;
    let mut res = 0;
    if sdiff_exp >= 0 {
        sign = Greater; // assumes |x| > |y|; will be changed if not.
        diff_exp = u64::wrapping_from(sdiff_exp);
        let mut cancel = 0;
        // If diff_exp != 0, i.e. diff_exp > 0, then |x| > |y|. Otherwise...
        if diff_exp == 0 {
            // Skip the identical most significant limbs, adding Limb::WIDTH to the number of
            // canceled bits at each iteration.
            while !xi_done && !yi_done && xs[xi] == ys[yi] {
                if xi == 0 {
                    xi_done = true;
                } else {
                    xi -= 1;
                }
                if yi == 0 {
                    yi_done = true;
                } else {
                    yi -= 1;
                }
                res += Limb::WIDTH;
            }
            if xi_done {
                // |x| = |y|
                if yi_done {
                    return (Equal, cancel);
                }
                // x has been read entirely, but not y. Thus |x| <= |y|. Swap xs and ys, and take
                // the opposite sign for the symmetric case below (simulating a swap). Note:
                // "yi_done = true;" is necessary to enter the following "if" (probably less
                // confusing than a "goto").
                swap(&mut xs, &mut ys);
                swap(&mut xi, &mut yi);
                swap(&mut x_exp, &mut y_exp);
                swap(&mut x_prec, &mut y_prec);
                xi_done = yi_done;
                yi_done = true;
                sign = Less;
            }
            if yi_done {
                // y discards exactly the upper part of x
                assert!(!xi_done);
                // Skip null limbs of x (= non-represented null limbs of y), adding Limb::WIDTH to
                // the number of canceled bits at each iteration.
                while xs[xi] == 0 {
                    // |x| = |y|
                    if xi == 0 {
                        xi_done = true;
                    } else {
                        xi -= 1;
                    }
                    if xi_done {
                        return (Equal, cancel);
                    }
                    res += Limb::WIDTH;
                }
                let z = LeadingZeros::leading_zeros(xs[xi]);
                // xs[xn] != 0
                cancel = res + z;
                return (sign, cancel);
            }
            assert!(!xi_done);
            assert!(!yi_done);
            assert!(xs[xi] != ys[yi]);
            // |x| != |y|. If |x| < |y|: swap xs and ys, and take the opposite sign.
            if xs[xi] < ys[yi] {
                swap(&mut xs, &mut ys);
                swap(&mut xi, &mut yi);
                swap(&mut xi_done, &mut yi_done);
                swap(&mut x_exp, &mut y_exp);
                swap(&mut x_prec, &mut y_prec);
                sign = Less;
            }
        }
    } else {
        // We necessarily have |x| < |y|.
        sign = Less;
        diff_exp = u64::exact_from(-sdiff_exp);
        swap(&mut xs, &mut ys);
        swap(&mut xi, &mut yi);
        swap(&mut x_exp, &mut y_exp);
        swap(&mut x_prec, &mut y_prec);
    }
    // Now we have removed the identical upper limbs of x and y (when diff_exp = 0), and after the
    // possible swap, we have |x| > |y|. The value diff_exp = EXP(x) - EXP(y) can be regarded as the
    // number of leading zeros of y, when aligned with x.
    //
    // When a limb of y is read from memory, the part that is not taken into account for the
    // operation with a limb xs[xn] of x will be put in lasty, shifted to the leftmost part (for
    // alignment with x):
    // ```
    // [-------- xs[xn] --------][------- xs[xn-1] -------]
    // [-- old_lasty --][-------- ys[yn] --------]
    //                           [-- new_lastc --]
    // ```
    // Note: if diff_exp == 0, then lasty will always remain 0.
    let mut lasty = 0;
    // Compute the next limb difference, which cannot be 0 (dif >= 1).
    let mut yy;
    if diff_exp < Limb::WIDTH {
        yy = ys[yi] >> diff_exp;
        if diff_exp != 0 {
            lasty = ys[yi] << (Limb::WIDTH - diff_exp);
        }
        if yi == 0 {
            yi_done = true;
        } else {
            yi -= 1;
        }
    } else {
        yy = 0;
        // remove Limb::WIDTH leading zeros
        diff_exp -= Limb::WIDTH;
    }
    // no borrow out in subtraction below
    assert!(xs[xi] >= yy);
    let mut dif = xs[xi] - yy;
    if xi == 0 {
        xi_done = true;
    } else {
        xi -= 1;
    }
    assert!(dif >= 1);
    let mut high_dif = false;
    // The current difference, here and later, is expressed under the form [high_dif][dif], where
    // high_dif is 0 or 1, and dif is a limb. Here, since we have computed a difference of limbs
    // (with x >= y), high_dif = 0.
    //
    // One needs to accumulate canceled bits for the remaining case where x and y are close to each
    // other due to a long borrow propagation:
    // ```
    //   x = [common part]1000...000[low(x)]
    //   y = [common part]0111...111[low(y)]
    // ```
    // After eliminating the common part above, we have computed a difference of the most
    // significant parts, which has been stored in [high_dif][dif] with high_dif = 0. We will loop
    // as long as the currently computed difference [high_dif][dif] = 1 (it is >= 1 by
    // construction). The computation of the difference will be:
    // ```
    //    1xxx...xxx
    //   - yyy...yyy
    // ```
    // where the leading 1 before xxx...xxx corresponds to [high_dif][dif] at the beginning of the
    // loop. We will exit the loop also when y has entirely been taken into account as cancellation
    // is no longer possible in this case (it is no longer possible to cancel the leading 1). Note:
    // We can enter the loop only with diff_exp = 0 (with a non-empty common part, partly or
    // entirely removed) or with diff_exp = 1 (with an empty common part). Indeed, if diff_exp > 1,
    // then no limbs have been skipped, so that xs[xn] had its MSB equal to 1 and the most two
    // significant bits of yy are 0, which implies that dif > 1.
    while (!yi_done || lasty != 0) && !high_dif && dif == 1 {
        // Since we consider the next limb, we assume a cancellation of Limb::WIDTH (the new
        // exponent of the difference now being the one of the MSB of the next limb). But if the
        // leading 1 remains 1 in the difference (i.e. high_dif = 1 at the end of the loop), then we
        // will need to decrease res.
        res += Limb::WIDTH;
        // - See comment before the loop
        // - Next limb of x or non-represented 0
        assert!(diff_exp <= 1);
        let xx = if xi_done {
            0
        } else {
            let r = xs[xi];
            if xi == 0 {
                xi_done = true;
            } else {
                xi -= 1;
            }
            r
        };
        if yi_done {
            yy = lasty;
            lasty = 0;
        } else if diff_exp == 0 {
            yy = ys[yi];
            if yi == 0 {
                yi_done = true;
            } else {
                yi -= 1;
            }
        } else {
            assert_eq!(diff_exp, 1);
            assert!(lasty == 0 || lasty == HIGH_BIT);
            yy = lasty + (ys[yi] >> 1);
            lasty = ys[yi] << (Limb::WIDTH - 1);
            if yi == 0 {
                yi_done = true;
            } else {
                yi -= 1;
            }
        }
        dif = xx.wrapping_sub(yy);
        high_dif = xx >= yy;
    }
    // Now, y has entirely been taken into account or [high_dif][dif] > 1. In any case,
    // [high_dif][dif] >= 1 by construction. First, we determine the currently number of canceled
    // bits, corresponding to the exponent of the current difference. The trailing bits of y, if
    // any, can still decrease the exponent of the difference when [high_dif][dif] is a power of
    // two, but since [high_dif][dif] > 1 in this case, by not more than 1.
    if high_dif {
        // high_dif == 1 See comment at the beginning of the above loop.
        res = res.checked_sub(1).unwrap();
        // Terminate if [high_dif][dif] is not a power of two.
        if dif != 0 {
            return (sign, res);
        }
    } else {
        // high_dif == 0
        assert!(dif >= 1); // [high_dif][dif] >= 1
        res += LeadingZeros::leading_zeros(dif);
        // Terminate if [high_dif][dif] is not a power of two.
        if !dif.is_power_of_2() {
            return (sign, res);
        }
    }
    // Now, the result will be res + (low(x) < low(y)).
    //
    // If y has entirely been taken into account, it can no longer modify the current result.
    if yi_done && lasty == 0 {
        return (sign, res);
    }
    if !xi_done {
        for &x in xs[..=xi].iter().rev() {
            if diff_exp >= Limb::WIDTH {
                diff_exp -= Limb::WIDTH;
                assert_eq!(yy, 0);
            } else if yi_done {
                yy = lasty;
                lasty = 0;
            } else if diff_exp == 0 {
                yy = ys[yi];
                if yi == 0 {
                    yi_done = true;
                } else {
                    yi -= 1;
                }
            } else {
                assert!((1..Limb::WIDTH).contains(&diff_exp));
                yy = lasty + (ys[yi] >> diff_exp);
                lasty = ys[yi] << (Limb::WIDTH - diff_exp);
                if yi == 0 {
                    yi_done = true;
                } else {
                    yi -= 1;
                }
            }
            if x != yy {
                return (sign, if x < yy { res + 1 } else { res });
            }
        }
    }
    // x has entirely been read. Determine whether the trailing part of y is non-zero.
    if lasty != 0 || !slice_test_zero(&ys[..=yi]) {
        res += 1;
    }
    (sign, res)
}

fn sub_float_significands_general<'a>(
    out: &mut [Limb],
    mut xs: &'a [Limb],
    mut x_exp: i32,
    mut x_prec: u64,
    mut ys: &'a [Limb],
    mut y_exp: i32,
    mut y_prec: u64,
    out_prec: u64,
    mut rm: RoundingMode,
) -> (i32, Ordering, bool) {
    let mut xs_len = xs.len();
    let mut ys_len = ys.len();
    let out_len = out.len();
    let mut add_exp = false;
    let (sign, cancel) = exponent_shift_compare(xs, x_exp, x_prec, ys, y_exp, y_prec);
    if sign == Equal {
        // x == y. Return exact number 0. Setting the most-significant limb to 0 is a sufficient
        // signal to the caller that the entire output is 0, since in every other case the precision
        // of the output is the same as the precision of the inputs, and the most-significant limb
        // is therefore nonzero.
        *out.last_mut().unwrap() = 0;
        return (0, Equal, false);
    }
    // sign != 0, so that cancel has a valid value.
    //
    // If subtraction: sign(out) = sign * sign(x) If addition: sign(out) = sign of the larger
    // argument in absolute value.
    //
    // Both cases can be simplified in:
    // ```
    // if (sign>0)
    //    if addition: sign(out) = sign * sign(x) = sign(x)
    //    if subtraction, x is greater, so sign(out) = sign(x)
    // else
    //    if subtraction, sign(out) = -sign(x)
    //    if addition, sign(out) = sign(y) (since y is greater)
    //      But if it is an addition, sign(x) and sign(y) are opposed!
    //      So sign(out) = -sign(x)
    // ```
    let neg = sign == Less;
    if neg {
        // swap x and y so that |x| > |y|
        swap(&mut xs, &mut ys);
        swap(&mut xs_len, &mut ys_len);
        swap(&mut x_exp, &mut y_exp);
        swap(&mut x_prec, &mut y_prec);
        rm.neg_assign();
    }
    let exp_diff = u64::exact_from(x_exp - y_exp);
    // Check if y is too small.
    let mut inexact = 0;
    if max(out_prec, x_prec) + 2 <= exp_diff {
        // Remember, we can't have an exact result!
        // ```
        //   A.AAAAAAAAAAAAAAAAA
        // = B.BBBBBBBBBBBBBBB
        //  -                     C.CCCCCCCCCCCCC
        // A = S*ABS(B) +/- ulp(a)
        // ```
        assert_ne!(rm, Exact, "Inexact float subtraction");
        let mut exp_a = x_exp;
        let increment_exp;
        (inexact, increment_exp) = round_helper_even(out, out_prec, xs, x_prec, rm);
        if increment_exp {
            exp_a += 1;
        }
        if inexact == 0 && rm != Down && rm != Floor {
            // out = x, but the exact value of x - y is a bit below. Then, except for directed
            // rounding similar to toward zero and before overflow checking: a is the correctly
            // rounded value and since |x| - |y| < |out|, the ternary value is given by the sign of
            // out.
            inexact = 1;
        } else if inexact != 0 && inexact != MPFR_EVEN_INEX {
            // ```
            //   O.OOOOOOOOOOOOOO
            // = X.XXXXXXXXXXXXXXX
            //  -                   Y.YYYYYYYYYYYYY
            // ```
            //
            // It isn't exact, so PREC(x) > PREC(out) and the last PREC(x)-PREC(out) bits of x are
            // not all zeros. Subtracting y from x will not have an effect on the rounding except in
            // case of a midpoint in the round-to-nearest mode, when the even rounding was done away
            // from zero instead of toward zero.
            //
            // In case of even rounding:
            // ```
            //   1.BBBBBBBBBBBBBx10
            // -                     1.CCCCCCCCCCCC
            // = 1.BBBBBBBBBBBBBx01  Rounded to PREC(x)
            // = 1.BBBBBBBBBBBBBx    Nearest / Rounded to PREC(out)
            // ```
            //
            // Set gives:
            // ```
            //   1.BBBBBBBBBBBBB0   if inexact == EVEN_INEX  (x == 0)
            //   1.BBBBBBBBBBBBB1+1 if inexact == -EVEN_INEX (x == 1)
            // ````
            // which means we get a wrong rounded result if x == 1, i.e. inexact == MPFR_EVEN_INEX
            // (for positive numbers).
            //
            // Nothing to do.
        } else {
            // We need to take the value preceding |out|. We can't use mpfr_nexttozero due to a
            // possible out-of-range exponent. But this will allow us to have more specific code.
            limbs_sub_limb_in_place(
                out,
                Limb::power_of_2(u64::exact_from(out_len << Limb::LOG_WIDTH) - out_prec),
            );
            let last_out = out.last_mut().unwrap();
            if !last_out.get_highest_bit() {
                exp_a.saturating_sub_assign(1);
                // The following is valid whether out_len = 1 or out_len > 1.
                *last_out |= HIGH_BIT;
            }
            inexact = -1;
        }
        return (exp_a, inexact.sign(), neg);
    }
    // Reserve a space to store x aligned with the result, i.e. shifted by (-cancel) % Limb::WIDTH
    // to the right
    let shift_x = cancel.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let cancel1 = usize::exact_from((cancel + shift_x) >> Limb::LOG_WIDTH);
    let mut shifted_x;
    // the `high cancel1` limbs from x should not be taken into account
    let xs = if shift_x == 0 {
        // no need of an extra space
        xs
    } else {
        shifted_x = vec![0; xs_len + 1];
        let (shifted_head, shifted_tail) = shifted_x.split_first_mut().unwrap();
        *shifted_head = limbs_shr_to_out(shifted_tail, xs, shift_x);
        xs_len += 1;
        &shifted_x
    };
    // Reserve a space to store y aligned with the result, i.e. shifted by (diff_exp - cancel) %
    // Limb::WIDTH to the right
    let shift_y = exp_diff
        .mod_power_of_2(Limb::LOG_WIDTH)
        .mod_power_of_2_sub(cancel.mod_power_of_2(Limb::LOG_WIDTH), Limb::LOG_WIDTH);
    assert!(shift_y < Limb::WIDTH);
    let mut shifted_y;
    let ys = if shift_y == 0 {
        ys
    } else {
        shifted_y = vec![0; ys_len + 1];
        let (shifted_head, shifted_tail) = shifted_y.split_first_mut().unwrap();
        *shifted_head = limbs_shr_to_out(shifted_tail, ys, shift_y);
        ys_len += 1;
        &shifted_y
    };
    // here we have shift_y = (diff_exp - cancel) % Limb::WIDTH, 0 <= shift_y < Limb::WIDTH, thus we
    // want cancel2 = ceil((cancel - diff_exp) / Limb::WIDTH)
    let cancel2 = if cancel >= exp_diff {
        // Note that cancel is signed and will be converted to mpfr_uexp_t (type of diff_exp) in the
        // expression below, so that this will work even if cancel is very large and diff_exp = 0.
        (i128::from(cancel) - i128::from(exp_diff) + i128::wrapping_from(IWIDTH_M1))
            >> Limb::LOG_WIDTH
    } else {
        -((i128::from(exp_diff) - i128::from(cancel)) >> Limb::LOG_WIDTH)
    };
    // The high cancel2 limbs from x should not be taken into account
    //
    // ```
    //                 ap[an-1]        ap[0]
    //             <----------------+-----------|---->
    //             <----------PREC(a)----------><-sh->
    // cancel1
    // limbs        bp[bn-cancel1-1]
    // <--...-----><----------------+-----------+----------->
    //  cancel2
    //  limbs       cp[cn-cancel2-1]                                    cancel2 >= 0
    //    <--...--><----------------+----------------+---------------->
    //                (-cancel2)                                        cancel2 < 0
    //                   limbs      <----------------+---------------->
    // ```
    //
    // First part: put in out[0..out_len - 1] the value of high(x) - high(y), where high(x) consists
    // of the high out_len + cancel1 limbs of x, and high(y) consists of the high out_len + cancel2
    // limbs of y.
    //
    // Copy high(x) into out
    if out_len + cancel1 <= xs_len {
        // ```
        // out: <----------------+-----------|---->
        // xs:  <----------------------------------------->
        // ```
        let xs_hi = &xs[xs_len - out_len - cancel1..];
        out.copy_from_slice(&xs_hi[..out_len]);
    } else {
        // ```
        // out: <----------------+-----------|---->
        // xs:  <------------------------->
        // ```
        if cancel1 < xs_len {
            let (out_lo, out_hi) = out.split_at_mut(out_len + cancel1 - xs_len);
            slice_set_zero(out_lo);
            out_hi.copy_from_slice(&xs[..xs_len - cancel1]);
        }
    }
    // subtract high(y)
    if i128::wrapping_from(out_len) + cancel2 > 0 {
        if cancel2 >= 0 {
            let cancel2 = usize::exact_from(cancel2);
            if out_len + cancel2 <= ys_len {
                // ```
                // out: <----------------------------->
                // ys:  <----------------------------------------->
                // ```
                let ys_hi = &ys[ys_len - out_len - cancel2..];
                limbs_sub_same_length_in_place_left(out, &ys_hi[..out_len]);
            } else {
                // ```
                // out: <---------------------------->
                // ys:  <------------------------->
                // ```
                if ys_len > cancel2 {
                    limbs_sub_same_length_in_place_left(
                        &mut out[out_len + cancel2 - ys_len..],
                        &ys[..ys_len - cancel2],
                    );
                }
            }
        } else {
            // cancel2 < 0
            let neg_cancel2 = usize::exact_from(-cancel2);
            let (out_lo, out_hi) = out.split_at_mut(out_len - neg_cancel2);
            let borrow = if out_len - neg_cancel2 <= ys_len {
                // ```
                // a: <----------------------------->
                // c: <----------------------------->
                // ```
                limbs_sub_same_length_in_place_left(out_lo, &ys[ys_len - (out_len - neg_cancel2)..])
            } else {
                // ```
                // a: <---------------------------->
                // c: <---------------->
                // ```
                let len = out_lo.len();
                limbs_sub_same_length_in_place_left(&mut out_lo[len - ys_len..], ys)
            };
            limbs_sub_limb_in_place(out_hi, Limb::from(borrow));
        }
    }
    // Now perform rounding
    let shift = u64::exact_from(out_len << Limb::LOG_WIDTH) - out_prec;
    let shift_bit = Limb::power_of_2(shift);
    let shift_mask = shift_bit - 1;
    // Last unused bits from out
    let out_head = out.first_mut().unwrap();
    let carry = *out_head & shift_mask;
    *out_head -= carry;
    let mut cmp_low = 0;
    let mut goto_truncate = false;
    let mut goto_end_of_sub = false;
    if rm == Nearest {
        if shift != 0 {
            let half_shift_bit = shift_bit >> 1;
            // Can decide except when carry = 2 ^ (sh - 1) [middle] or carry = 0 [truncate, but
            // cannot decide inexact flag]
            if carry > half_shift_bit {
                if limbs_slice_add_limb_in_place(out, shift_bit) {
                    // result is a power of 2: 11111111111111 + 1 = 1000000000000000
                    out[out_len - 1] = HIGH_BIT;
                    add_exp = true;
                }
                // result larger than exact value
                inexact = 1;
                goto_truncate = true;
            } else if carry != 0 && carry < half_shift_bit {
                inexact = -1; // result if smaller than exact value
                goto_truncate = true;
            } else {
                // now carry = 2 ^ (sh - 1), in which case cmp_low = 2, or carry = 0, in which case
                // cmp_low = 0
                cmp_low = if carry == 0 { 0 } else { 2 };
            }
        }
    } else if carry != 0 {
        if rm == Floor || rm == Down || rm == Exact {
            inexact = -1;
        } else {
            if limbs_slice_add_limb_in_place(out, shift_bit) {
                // result is a power of 2: 11111111111111 + 1 = 1000000000000000
                out[out_len - 1] = HIGH_BIT;
                add_exp = true;
            }
            // result larger than exact value
            inexact = 1;
        }
        goto_truncate = true;
    }
    if !goto_truncate {
        // We have to consider the low (xs_len - (out_len + cancel1)) limbs from x, and the (ys_len
        // - (out_len + cancel2)) limbs from y.
        xs_len.saturating_sub_assign(out_len + cancel1);
        let ys_len0 = ys_len;
        ys_len = usize::saturating_from(
            i128::wrapping_from(ys_len) - (i128::wrapping_from(out_len) + cancel2),
        );
        // For rounding to nearest, we couldn't conclude up to here in the following cases:
        // - shift = 0, then cmp_low = 0: we can either truncate, subtract one ulp or add one ulp:
        //   -1 ulp < low(x) - low(y) < 1 ulp
        // - shift > 0 but the low `shift` bits from high(x) - high(y) equal 2 ^ (shift - 1): -0.5
        //   ulp <= -1 / 2 ^ shift < low(x) - low(y) - 0.5 < 1 / 2 ^ shift <= 0.5 ulp we can't
        //   decide the rounding, in that case cmp_low = 2: either we truncate and flag = -1, or we
        //   add one ulp and flag = 1
        // - The low shift > 0 bits from high(x)-high(y) equal 0: we know we have to truncate but we
        //   can't decide the ternary value, here cmp_low = 0: -0.5 ulp <= -1 / 2 ^ shift < low(x)
        //   -low(y) < 1 / 2 ^ shift <= 0.5 ulp we always truncate and inexact can be any of -1, 0,
        //   1
        // - Note: here ys_len might exceed ys_len0, in which case we consider a zero limb
        let mut k: i32 = 0;
        while xs_len != 0 || ys_len != 0 {
            // - If cmp_low < 0, we know low(x) - low(y) < 0
            // - If cmp_low > 0, we know low(x) - low(y) > 0 (more precisely if cmp_low = 2, low(x)
            //   - low(y) = 0.5 ulp so far)
            // - If cmp_low = 0, so far low(x) - low(y) = 0
            // - get next limbs
            let mut xx = if xs_len != 0 {
                xs_len -= 1;
                xs[xs_len]
            } else {
                0
            };
            let mut yy = if ys_len != 0 && {
                let c = ys_len <= ys_len0;
                ys_len -= 1;
                c
            } {
                ys[ys_len]
            } else {
                0
            };
            // cmp_low compares low(x) and low(y)
            if cmp_low == 0 {
                // case 1 or 3
                cmp_low = match xx.cmp(&yy) {
                    Greater => 1,
                    Less => -2 + k,
                    Equal => 0,
                };
            }
            // Case 1 for k=0 splits into 7 subcases:
            // - 1a: xx > yy + half
            // - 1b: xx = yy + half
            // - 1c: 0 < xx - yy < half
            // - 1d: xx = yy
            // - 1e: -half < xx - yy < 0
            // - 1f: xx - yy = -half
            // - 1g: xx - yy < -half
            //
            // Case 2 splits into 3 subcases:
            // - 2a: xx > yy
            // - 2b: xx = yy
            // - 2c: xx < yy
            //
            // Case 3 splits into 3 subcases:
            // - 3a: xx > yy
            // - 3b: xx = yy
            // - 3c: xx < yy
            //
            // The case rounding to nearest with sh=0 is special since one couldn't subtract above
            // 1/2 ulp in the trailing limb of the result
            if rm == Nearest && shift == 0 && k == 0 {
                // case 1 for k = 0
                // - add one ulp if xx > yy + half
                // - truncate if yy - half < xx < yy + half
                // - sub one ulp if xx < yy - half
                if cmp_low < 0 {
                    // - xx < yy: -1 ulp < low(b) - low(c) < 0,
                    // - cases 1e, 1f and 1g
                    if yy >= HIGH_BIT {
                        yy -= HIGH_BIT;
                    } else {
                        // since xx < yy < half, xx + half < 2 * half
                        xx += HIGH_BIT;
                    }
                    // Now we have xx < yy + half: we have to subtract one ulp if xx < yy, and
                    // truncate if xx > yy
                } else {
                    // xx >= yy, cases 1a to 1d
                    if yy < HIGH_BIT {
                        yy += HIGH_BIT;
                    } else {
                        // since xx >= yy >= half, xx - half >= 0
                        xx -= HIGH_BIT;
                    }
                    // Now we have xx > yy - half: we have to add one ulp if xx > yy, and truncate
                    // if xx < yy
                    if cmp_low > 0 {
                        cmp_low = 2;
                    }
                }
            }
            match cmp_low.sign() {
                Less => {
                    // low(x) - low(y) < 0: either truncate or subtract one ulp
                    match rm {
                        Floor | Down => {
                            limbs_sub_limb_in_place(out, shift_bit);
                            inexact = -1;
                            goto_end_of_sub = true;
                        }
                        Ceiling | Up | Exact => {
                            inexact = 1;
                            goto_truncate = true;
                        }
                        Nearest => {
                            // - If cmp_low < 0 and xx > yy, then -0.5 ulp < low(x) - low(y) < 0,
                            //   whatever the value of shift.
                            // - If shift > 0, then cmp_low < 0 implies that the initial neglected
                            //   shift bits were 0 (otherwise cmp_low = 2 initially), thus the
                            //   weight of the new bits is less than 0.5 ulp too.
                            // - If k > 0 (and shift = 0) this means that either the first neglected
                            //   limbs xx and yy were equal (thus cmp_low was 0 for k = 0), or we
                            //   had xx - yy = -0.5 ulp or 0.5 ulp.
                            // - The last case is not possible here since we would have cmp_low > 0
                            //   which is sticky.
                            // - In the first case (where we have cmp_low = -1), we truncate,
                            //   whereas in the 2nd case we have cmp_low = -2 and we subtract one
                            //   ulp.
                            if xx > yy || shift > 0 || cmp_low == -1 {
                                // - -0.5 ulp < low(b)-low(c) < 0,
                                // - xx > yy corresponds to cases 1e and 1f1
                                // - shift > 0 corresponds to cases 3c and 3b3
                                // - cmp_low = -1 corresponds to case 1d3 (also 3b3)
                                inexact = 1;
                                goto_truncate = true;
                            } else if xx < yy {
                                // Here shift = 0 and low(x) - low(y) < -0.5 ulp, this corresponds
                                // to cases 1g and 1f3
                                //
                                limbs_sub_limb_in_place(out, shift_bit);
                                inexact = -1;
                                goto_end_of_sub = true;
                            }
                            // The only case where we can't conclude is shift = 0 and xx = yy, i.e.,
                            // we have low(x)
                            // - low(y) = -0.5 ulp (up to now), thus we don't know if we must
                            //   truncate or
                            // subtract one ulp. Note: for shift = 0 we can't have low(x) - low(y) =
                            // -0.5 ulp up to now, since low(x) - low(y) > 1 / 2 ^ shift
                        }
                    }
                }
                Greater => {
                    // 0 < low(x) - low(y): either truncate or add one ulp
                    match rm {
                        Floor | Down | Exact => {
                            inexact = -1;
                            goto_truncate = true;
                        }
                        Ceiling | Up => {
                            if limbs_slice_add_limb_in_place(out, shift_bit) {
                                // result is a power of 2: 11111111111111 + 1 = 1000000000000000
                                out[out_len - 1] = HIGH_BIT;
                                add_exp = true;
                            }
                            // result larger than exact value
                            inexact = 1;
                            goto_truncate = true;
                        }
                        Nearest => {
                            match xx.cmp(&yy) {
                                Greater => {
                                    // If sh = 0, then xx > yy means that low(x) - low(y) > 0.5 ulp,
                                    // and similarly when cmp_low = 2.
                                    if cmp_low == 2 {
                                        // cases 1a, 1b1, 2a and 2b1
                                        //
                                        // shift > 0 and cmp_low > 0: this implies that the `shift`
                                        // initial neglected bits were 0, and the remaining low(x) -
                                        // low(y) > 0, but its weight is less than 0.5 ulp
                                        if limbs_slice_add_limb_in_place(out, shift_bit) {
                                            // result is a power of 2: 11111111111111 + 1 =
                                            // 1000000000000000
                                            out[out_len - 1] = HIGH_BIT;
                                            add_exp = true;
                                        }
                                        // result larger than exact value
                                        inexact = 1;
                                    } else {
                                        // 0 < low(x) - low(y) < 0.5 ulp, this corresponds to cases
                                        // 3a, 1d1 and 3b1
                                        inexact = -1;
                                    }
                                    goto_truncate = true;
                                }
                                Less => {
                                    // 0 < low(x) - low(y) < 0.5 ulp, cases 1c, 1b3, 2b3 and 2c
                                    inexact = -1;
                                    goto_truncate = true;
                                }
                                Equal => {
                                    // The only case where we can't conclude is xx = yy, i.e.,
                                    // low(x) - low(y) = 0.5 ulp (up to now), thus we don't know if
                                    // we must truncate or add one ulp.
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            // After k = 0, we cannot conclude in the following cases, we split them according to
            // the values of xx and yy for k = 1:
            // ```
            // 1b. shift = 0 and cmp_low = 1 and xx-yy = half [around 0.5 ulp]
            //     1b1. xx > yy: add one ulp, inex = 1
            //     1b2: xx = yy: cannot conclude
            //     1b3: xx < yy: truncate, inex = -1
            // 1d. shift = 0 and cmp_low = 0 and xx-yy = 0 [around 0]
            //     1d1: xx > yy: truncate, inex = -1
            //     1d2: xx = yy: cannot conclude
            //     1d3: xx < yy: truncate, inex = +1
            // 1f. shift = 0 and cmp_low = -1 and xx-yy = -half [around -0.5 ulp]
            //     1f1: xx > yy: truncate, inex = +1
            //     1f2: xx = yy: cannot conclude
            //     1f3: xx < yy: sub one ulp, inex = -1
            // 2b. shift > 0 and cmp_low = 2 and xx=yy [around 0.5 ulp]
            //     2b1. xx > yy: add one ulp, inex = 1
            //     2b2: xx = yy: cannot conclude
            //     2b3: xx < yy: truncate, inex = -1
            // 3b. shift > 0 and cmp_low = 0 [around 0]
            //     3b1. xx > yy: truncate, inex = -1
            //     3b2: xx = yy: cannot conclude
            //     3b3: xx < yy: truncate, inex = +1
            // ```
            if goto_truncate || goto_end_of_sub {
                break;
            }
            k = 1;
        }
        if !goto_truncate && !goto_end_of_sub {
            inexact = if rm == Nearest && cmp_low != 0 {
                // Even rounding rule
                if (out[0] >> shift) & 1 != 0 {
                    if cmp_low < 0 {
                        limbs_sub_limb_in_place(out, shift_bit);
                        goto_end_of_sub = true;
                        -1
                    } else {
                        if limbs_slice_add_limb_in_place(out, shift_bit) {
                            // result is a power of 2: 11111111111111 + 1 = 1000000000000000
                            out[out_len - 1] = HIGH_BIT;
                            add_exp = true;
                        }
                        // result larger than exact value
                        1
                    }
                } else if cmp_low > 0 {
                    -1
                } else {
                    1
                }
            } else {
                0
            };
        }
    }
    let last_out = &mut out[out_len - 1];
    if !goto_end_of_sub && *last_out >> WIDTH_M1 == 0 {
        // case 1 - varepsilon
        *last_out = HIGH_BIT;
        add_exp = true;
    }
    // We have to set MPFR_EXP(out) to MPFR_EXP(x) - cancel + diff_exp, taking care of
    // underflows/overflows in that computation, and of the allowed exponent range
    let exp_a = if cancel != 0 {
        x_exp = i32::saturating_from(i128::from(x_exp) - i128::from(cancel));
        if add_exp {
            x_exp.saturating_add_assign(1);
        }
        x_exp
    } else {
        // cancel = 0: MPFR_EXP(out) <- MPFR_EXP(x) + diff_exp
        //
        // In case cancel = 0, diff_exp can still be 1, in case x is just below a power of two, y is
        // very small, prec(out) < prec(x), and rnd = away or nearest
        if add_exp {
            x_exp.saturating_add_assign(1);
        }
        x_exp
    };
    // check that result is msb-normalized
    assert!(last_out.get_highest_bit());
    (exp_a, inexact.sign(), neg)
}
