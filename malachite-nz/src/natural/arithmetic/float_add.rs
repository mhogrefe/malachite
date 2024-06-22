// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2004-2022 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_limb_to_out, limbs_add_same_length_to_out, limbs_add_to_out_aliased_2,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
};
use crate::natural::arithmetic::float_add::RoundBit::*;
use crate::natural::arithmetic::is_power_of_2::limbs_is_power_of_2;
use crate::natural::arithmetic::shr::{limbs_shr_to_out, limbs_slice_shr_in_place};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::Ordering::{self, *};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    NegModPowerOf2, OverflowingAddAssign, Parity, PowerOf2, ShrRound, WrappingAddAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

const TWICE_WIDTH: u64 = Limb::WIDTH * 2;
const THRICE_WIDTH: u64 = Limb::WIDTH * 3;

pub fn add_float_significands_in_place(
    mut x: &mut Natural,
    x_exp: &mut i32,
    x_prec: u64,
    mut y: &mut Natural,
    y_exp: i32,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Ordering, bool) {
    if x_prec == y_prec && out_prec == x_prec {
        add_float_significands_in_place_same_prec(x, x_exp, y, y_exp, out_prec, rm)
    } else if *x_exp >= y_exp {
        match (&mut x, &mut y) {
            (Natural(Small(small_x)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *small_x = out[0];
                    *x_exp = out_exp;
                    (o, false)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Large(out));
                    *x_exp = out_exp;
                    (o, false)
                }
            }
            (Natural(Small(small_x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        ys,
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *small_x = out[0];
                    *x_exp = out_exp;
                    (o, false)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        ys,
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Large(out));
                    *x_exp = out_exp;
                    (o, false)
                }
            }
            (Natural(Large(xs)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    (o, false)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *xs = out;
                    *x_exp = out_exp;
                    (o, false)
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, xs, *x_exp, ys, y_exp, out_prec, rm,
                    );
                    *x = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    (o, false)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, xs, *x_exp, ys, y_exp, out_prec, rm,
                    );
                    *xs = out;
                    *x_exp = out_exp;
                    (o, false)
                }
            }
        }
    } else {
        match (&mut x, &mut y) {
            (Natural(Small(small_x)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *small_y = out[0];
                    *x_exp = out_exp;
                    (o, true)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *y = Natural(Large(out));
                    *x_exp = out_exp;
                    (o, true)
                }
            }
            (Natural(Small(small_x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        ys,
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *y = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    (o, true)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        ys,
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *ys = out;
                    *x_exp = out_exp;
                    (o, true)
                }
            }
            (Natural(Large(xs)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        xs,
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *small_y = out[0];
                    *x_exp = out_exp;
                    (o, true)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        xs,
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *y = Natural(Large(out));
                    *x_exp = out_exp;
                    (o, true)
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, ys, y_exp, xs, *x_exp, out_prec, rm,
                    );
                    *y = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    (o, true)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, ys, y_exp, xs, *x_exp, out_prec, rm,
                    );
                    *ys = out;
                    *x_exp = out_exp;
                    (o, true)
                }
            }
        }
    }
}

pub fn add_float_significands_in_place_ref(
    mut x: &mut Natural,
    x_exp: &mut i32,
    x_prec: u64,
    y: &Natural,
    y_exp: i32,
    y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> Ordering {
    if x_prec == y_prec && out_prec == x_prec {
        add_float_significands_in_place_same_prec_ref(x, x_exp, y, y_exp, out_prec, rm)
    } else if *x_exp >= y_exp {
        match (&mut x, y) {
            (Natural(Small(small_x)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *small_x = out[0];
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Large(out));
                    *x_exp = out_exp;
                    o
                }
            }
            (Natural(Small(small_x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        ys,
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *small_x = out[0];
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_x],
                        *x_exp,
                        ys,
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Large(out));
                    *x_exp = out_exp;
                    o
                }
            }
            (Natural(Large(xs)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        xs,
                        *x_exp,
                        &[*small_y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    *xs = out;
                    *x_exp = out_exp;
                    o
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, xs, *x_exp, ys, y_exp, out_prec, rm,
                    );
                    *x = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, xs, *x_exp, ys, y_exp, out_prec, rm,
                    );
                    *xs = out;
                    *x_exp = out_exp;
                    o
                }
            }
        }
    } else {
        match (&mut x, y) {
            (Natural(Small(small_x)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *small_x = out[0];
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Large(out));
                    *x_exp = out_exp;
                    o
                }
            }
            (Natural(Small(small_x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        ys,
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *small_x = out[0];
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        ys,
                        y_exp,
                        &[*small_x],
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Large(out));
                    *x_exp = out_exp;
                    o
                }
            }
            (Natural(Large(xs)), Natural(Small(small_y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        xs,
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *x = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*small_y],
                        y_exp,
                        xs,
                        *x_exp,
                        out_prec,
                        rm,
                    );
                    *xs = out;
                    *x_exp = out_exp;
                    o
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, ys, y_exp, xs, *x_exp, out_prec, rm,
                    );
                    *x = Natural(Small(out[0]));
                    *x_exp = out_exp;
                    o
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, ys, y_exp, xs, *x_exp, out_prec, rm,
                    );
                    *xs = out;
                    *x_exp = out_exp;
                    o
                }
            }
        }
    }
}

pub fn add_float_significands_ref_ref<'a>(
    mut x: &'a Natural,
    mut x_exp: i32,
    mut x_prec: u64,
    mut y: &'a Natural,
    mut y_exp: i32,
    mut y_prec: u64,
    out_prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    if x_prec == y_prec && out_prec == x_prec {
        add_float_significands_same_prec_ref_ref(x, x_exp, y, y_exp, out_prec, rm)
    } else {
        if x_exp < y_exp {
            swap(&mut x, &mut y);
            swap(&mut x_exp, &mut y_exp);
            swap(&mut x_prec, &mut y_prec);
        }
        match (x, y) {
            (Natural(Small(x)), Natural(Small(y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        &[*y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    (Natural(Small(out[0])), out_exp, o)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        &[*y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    (Natural(Large(out)), out_exp, o)
                }
            }
            (Natural(Small(x)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        ys,
                        y_exp,
                        out_prec,
                        rm,
                    );
                    (Natural(Small(out[0])), out_exp, o)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        &[*x],
                        x_exp,
                        ys,
                        y_exp,
                        out_prec,
                        rm,
                    );
                    (Natural(Large(out)), out_exp, o)
                }
            }
            (Natural(Large(xs)), Natural(Small(y))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        xs,
                        x_exp,
                        &[*y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    (Natural(Small(out[0])), out_exp, o)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out,
                        xs,
                        x_exp,
                        &[*y],
                        y_exp,
                        out_prec,
                        rm,
                    );
                    (Natural(Large(out)), out_exp, o)
                }
            }
            (Natural(Large(xs)), Natural(Large(ys))) => {
                if out_prec <= Limb::WIDTH {
                    let mut out = [0];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, xs, x_exp, ys, y_exp, out_prec, rm,
                    );
                    (Natural(Small(out[0])), out_exp, o)
                } else {
                    let mut out =
                        vec![0; usize::exact_from(out_prec.shr_round(Limb::LOG_WIDTH, Ceiling).0)];
                    let (out_exp, o) = add_float_significands_general(
                        &mut out, xs, x_exp, ys, y_exp, out_prec, rm,
                    );
                    (Natural(Large(out)), out_exp, o)
                }
            }
        }
    }
}

// This is mpfr_add1sp from add1sp.c, MPFR 4.2.0.
fn add_float_significands_in_place_same_prec(
    x: &mut Natural,
    x_exp: &mut i32,
    y: &mut Natural,
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Ordering, bool) {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (sum, sum_exp, o) = if prec == Limb::WIDTH {
                add_float_significands_same_prec_w(*x, *x_exp, *y, y_exp, rm)
            } else {
                add_float_significands_same_prec_lt_w(*x, *x_exp, *y, y_exp, prec, rm)
            };
            *x = sum;
            *x_exp = sum_exp;
            (o, false)
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_mut_slice()) {
            ([x_0, x_1], [y_0, y_1]) => {
                let (sum_0, sum_1, sum_exp, o) = if prec == TWICE_WIDTH {
                    add_float_significands_same_prec_2w(*x_0, *x_1, *x_exp, *y_0, *y_1, y_exp, rm)
                } else {
                    add_float_significands_same_prec_gt_w_lt_2w(
                        *x_0, *x_1, *x_exp, *y_0, *y_1, y_exp, prec, rm,
                    )
                };
                *x_0 = sum_0;
                *x_1 = sum_1;
                *x_exp = sum_exp;
                (o, false)
            }
            ([x_0, x_1, x_2], [y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (sum_0, sum_1, sum_2, sum_exp, o) =
                    add_float_significands_same_prec_gt_2w_lt_3w(
                        *x_0, *x_1, *x_2, *x_exp, *y_0, *y_1, *y_2, y_exp, prec, rm,
                    );
                *x_0 = sum_0;
                *x_1 = sum_1;
                *x_2 = sum_2;
                *x_exp = sum_exp;
                (o, false)
            }
            (xs_slice, ys_slice) => {
                let (sum_exp, o, swapped) = add_float_significands_same_prec_ge_3w_val_val(
                    xs_slice, *x_exp, ys_slice, y_exp, prec, rm,
                );
                *x_exp = sum_exp;
                (o, swapped)
            }
        },
        _ => unreachable!(),
    }
}

// This is mpfr_add1sp from add1sp.c, MPFR 4.2.0.
fn add_float_significands_in_place_same_prec_ref(
    x: &mut Natural,
    x_exp: &mut i32,
    y: &Natural,
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> Ordering {
    match (x, y) {
        (Natural(Small(ref mut x)), Natural(Small(y))) => {
            let (sum, sum_exp, o) = if prec == Limb::WIDTH {
                add_float_significands_same_prec_w(*x, *x_exp, *y, y_exp, rm)
            } else {
                add_float_significands_same_prec_lt_w(*x, *x_exp, *y, y_exp, prec, rm)
            };
            *x = sum;
            *x_exp = sum_exp;
            o
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_mut_slice(), ys.as_slice()) {
            ([x_0, x_1], &[y_0, y_1]) => {
                let (sum_0, sum_1, sum_exp, o) = if prec == TWICE_WIDTH {
                    add_float_significands_same_prec_2w(*x_0, *x_1, *x_exp, y_0, y_1, y_exp, rm)
                } else {
                    add_float_significands_same_prec_gt_w_lt_2w(
                        *x_0, *x_1, *x_exp, y_0, y_1, y_exp, prec, rm,
                    )
                };
                *x_0 = sum_0;
                *x_1 = sum_1;
                *x_exp = sum_exp;
                o
            }
            ([x_0, x_1, x_2], &[y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (sum_0, sum_1, sum_2, sum_exp, o) =
                    add_float_significands_same_prec_gt_2w_lt_3w(
                        *x_0, *x_1, *x_2, *x_exp, y_0, y_1, y_2, y_exp, prec, rm,
                    );
                *x_0 = sum_0;
                *x_1 = sum_1;
                *x_2 = sum_2;
                *x_exp = sum_exp;
                o
            }
            (xs, ys) => {
                if *x_exp >= y_exp {
                    let (sum_exp, o) = add_float_significands_same_prec_ge_3w_val_ref(
                        xs, *x_exp, ys, y_exp, prec, rm,
                    );
                    *x_exp = sum_exp;
                    o
                } else {
                    let (sum_exp, o) = add_float_significands_same_prec_ge_3w_ref_val(
                        ys, y_exp, xs, *x_exp, prec, rm,
                    );
                    *x_exp = sum_exp;
                    o
                }
            }
        },
        _ => unreachable!(),
    }
}

// This is mpfr_add1sp from add1sp.c, MPFR 4.2.0.
fn add_float_significands_same_prec_ref_ref(
    x: &Natural,
    x_exp: i32,
    y: &Natural,
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Natural, i32, Ordering) {
    match (x, y) {
        (Natural(Small(x)), Natural(Small(y))) => {
            let (sum, sum_exp, o) = if prec == Limb::WIDTH {
                add_float_significands_same_prec_w(*x, x_exp, *y, y_exp, rm)
            } else {
                add_float_significands_same_prec_lt_w(*x, x_exp, *y, y_exp, prec, rm)
            };
            (Natural(Small(sum)), sum_exp, o)
        }
        (Natural(Large(xs)), Natural(Large(ys))) => match (xs.as_slice(), ys.as_slice()) {
            (&[x_0, x_1], &[y_0, y_1]) => {
                let (sum_0, sum_1, sum_exp, o) = if prec == TWICE_WIDTH {
                    add_float_significands_same_prec_2w(x_0, x_1, x_exp, y_0, y_1, y_exp, rm)
                } else {
                    add_float_significands_same_prec_gt_w_lt_2w(
                        x_0, x_1, x_exp, y_0, y_1, y_exp, prec, rm,
                    )
                };
                (Natural(Large(vec![sum_0, sum_1])), sum_exp, o)
            }
            (&[x_0, x_1, x_2], &[y_0, y_1, y_2]) if prec != THRICE_WIDTH => {
                let (sum_0, sum_1, sum_2, sum_exp, o) =
                    add_float_significands_same_prec_gt_2w_lt_3w(
                        x_0, x_1, x_2, x_exp, y_0, y_1, y_2, y_exp, prec, rm,
                    );
                (Natural(Large(vec![sum_0, sum_1, sum_2])), sum_exp, o)
            }
            (xs, ys) => {
                let mut out = vec![0; xs.len()];
                let (sum_exp, o) = add_float_significands_same_prec_ge_3w_ref_ref(
                    &mut out, xs, x_exp, ys, y_exp, prec, rm,
                );
                (Natural(Large(out)), sum_exp, o)
            }
        },
        _ => unreachable!(),
    }
}

const WIDTH_M1: u64 = Limb::WIDTH - 1;
const HIGH_BIT: Limb = 1 << WIDTH_M1;

// This is mpfr_add1sp1 from add1sp.c, MPFR 4.2.0.
fn add_float_significands_same_prec_lt_w(
    mut x: Limb,
    mut x_exp: i32,
    mut y: Limb,
    mut y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, i32, Ordering) {
    assert!(prec < Limb::WIDTH);
    let shift = Limb::WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let (mut sum, sticky_bit, round_bit) = if x_exp == y_exp {
        // The following line is probably better than
        // ```
        // sum = HIGH_BIT | ((x + y) >> 1);
        // ```
        // as it has less dependency and doesn't need a long constant on some processors. On ARM, it
        // can also probably benefit from shift-and-op in a better way. Timings cannot be
        // conclusive.
        let sum = (x >> 1) + (y >> 1);
        x_exp = x_exp.checked_add(1).unwrap();
        let round_bit = sum & (shift_bit >> 1);
        // since x + y fits on prec + 1 bits, the sticky bit is zero
        (sum ^ round_bit, 0, round_bit)
    } else {
        if x_exp < y_exp {
            swap(&mut x_exp, &mut y_exp);
            swap(&mut x, &mut y);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        let mask = shift_bit - 1;
        if exp_diff < shift {
            // we can shift y by exp_diff bits to the right without losing any bit. Moreover, we can
            // shift one more if there is an exponent increase.
            let (mut sum, overflow) = x.overflowing_add(y >> exp_diff);
            if overflow {
                // carry
                assert!(sum.even());
                sum = HIGH_BIT | (sum >> 1);
                x_exp = x_exp.checked_add(1).unwrap();
            }
            let round_bit = sum & (shift_bit >> 1);
            (sum & !mask, (sum & mask) ^ round_bit, round_bit)
        } else if exp_diff < Limb::WIDTH {
            // shift <= exp_diff < Limb::WIDTH
            let mut sticky_bit = y << (Limb::WIDTH - exp_diff); // bits from y[-1] after shift
            let (mut sum, overflow) = x.overflowing_add(y >> exp_diff);
            if overflow {
                // carry
                sticky_bit |= sum & 1;
                sum = HIGH_BIT | (sum >> 1);
                x_exp = x_exp.checked_add(1).unwrap();
            }
            let round_bit = sum & (shift_bit >> 1);
            (
                sum & !mask,
                sticky_bit | ((sum & mask) ^ round_bit),
                round_bit,
            )
        } else {
            // - exp_diff >= Limb::WIDTH
            // - round_bit == 0 since prec < Limb::WIDTH
            // - sticky_bit == 1 since y != 0
            (x, 1, 0)
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (sum, x_exp, Equal)
    } else {
        match rm {
            Exact => panic!("Inexact float addition"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (sum & shift_bit) == 0) {
                    (sum, x_exp, Less)
                } else if sum.overflowing_add_assign(shift_bit) {
                    (HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum, x_exp, Greater)
                }
            }
            Floor | Down => (sum, x_exp, Less),
            Ceiling | Up => {
                if sum.overflowing_add_assign(shift_bit) {
                    (HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum, x_exp, Greater)
                }
            }
        }
    }
}

// This is mpfr_add1sp1n from add1sp.c, MPFR 4.2.0.
fn add_float_significands_same_prec_w(
    mut x: Limb,
    mut x_exp: i32,
    mut y: Limb,
    mut y_exp: i32,
    rm: RoundingMode,
) -> (Limb, i32, Ordering) {
    let (mut sum, sticky_bit, round_bit) = if x_exp == y_exp {
        let sum = x.wrapping_add(y);
        x_exp = x_exp.checked_add(1).unwrap();
        // since x + y fits on Limb::WIDTH + 1 bits, the sticky bit is zero
        (HIGH_BIT | (sum >> 1), 0, sum & 1)
    } else {
        if x_exp < y_exp {
            swap(&mut x, &mut y);
            swap(&mut x_exp, &mut y_exp);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        if exp_diff < Limb::WIDTH {
            // - 1 <= exp_diff < Limb::WIDTH
            // - bits from y[-1] after shift
            let sticky_bit = y << (Limb::WIDTH - exp_diff);
            let (sum, overflow) = x.overflowing_add(y >> exp_diff);
            if overflow {
                // carry
                x_exp = x_exp.checked_add(1).unwrap();
                (HIGH_BIT | (sum >> 1), sticky_bit, sum & 1)
            } else {
                // no carry
                (sum, sticky_bit & !HIGH_BIT, sticky_bit & HIGH_BIT)
            }
        } else {
            let round = exp_diff == Limb::WIDTH;
            // exp_diff >= Limb::WIDTH
            (x, Limb::from(!round || y != HIGH_BIT), Limb::from(round))
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (sum, x_exp, Equal)
    } else {
        match rm {
            Exact => panic!("Inexact float addition"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (sum & 1) == 0) {
                    (sum, x_exp, Less)
                } else if sum.overflowing_add_assign(1) {
                    (HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum, x_exp, Greater)
                }
            }
            Floor | Down => (sum, x_exp, Less),
            Ceiling | Up => {
                if sum.overflowing_add_assign(1) {
                    (HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum, x_exp, Greater)
                }
            }
        }
    }
}

// This is mpfr_add1sp2 from add1sp.c, MPFR 4.2.0.
fn add_float_significands_same_prec_gt_w_lt_2w(
    mut x_0: Limb,
    mut x_1: Limb,
    mut x_exp: i32,
    mut y_0: Limb,
    mut y_1: Limb,
    mut y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, i32, Ordering) {
    let shift = TWICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let shift_m1_bit = shift_bit >> 1;
    let (mut sum_0, mut sum_1, round_bit, sticky_bit) = if x_exp == y_exp {
        // since x_1, y_1 >= HIGH_BIT, a carry always occurs
        let (mut a0, overflow) = x_0.overflowing_add(y_0);
        let mut a1 = x_1.wrapping_add(y_1);
        if overflow {
            a1.wrapping_add_assign(1);
        }
        a0 = (a0 >> 1) | (a1 << WIDTH_M1);
        x_exp = x_exp.checked_add(1).unwrap();
        let round_bit = a0 & shift_m1_bit;
        // Since x + y fits on prec + 1 bits, the sticky bit is zero.
        (a0 ^ round_bit, HIGH_BIT | (a1 >> 1), a0 & shift_m1_bit, 0)
    } else {
        if x_exp < y_exp {
            swap(&mut x_0, &mut y_0);
            swap(&mut x_1, &mut y_1);
            swap(&mut x_exp, &mut y_exp);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        let mask = shift_bit - 1;
        if exp_diff < Limb::WIDTH {
            let comp_diff = Limb::WIDTH - exp_diff;
            // 0 < exp_diff < Limb::WIDTH
            let mut sticky_bit = y_0 << comp_diff; // bits from y[-1] after shift
            let (mut a0, overflow_1) = x_0.overflowing_add((y_1 << comp_diff) | (y_0 >> exp_diff));
            let (mut a1, mut overflow_2) = x_1.overflowing_add(y_1 >> exp_diff);
            if overflow_1 {
                overflow_2 |= a1.overflowing_add_assign(1);
            }
            let sum_1 = if overflow_2 {
                // carry in high word
                sticky_bit |= a0 & 1;
                // shift a by 1
                a0 = (a1 << WIDTH_M1) | (a0 >> 1);
                x_exp = x_exp.checked_add(1).unwrap();
                HIGH_BIT | (a1 >> 1)
            } else {
                a1
            };
            let round_bit = a0 & shift_m1_bit;
            (
                a0 & !mask,
                sum_1,
                round_bit,
                sticky_bit | (a0 & mask) ^ round_bit,
            )
        } else if exp_diff < TWICE_WIDTH {
            // Limb::WIDTH <= exp_diff < Limb::WIDTH * 2
            let mut sticky_bit = if exp_diff == Limb::WIDTH {
                y_0
            } else {
                y_0 | (y_1 << (TWICE_WIDTH - exp_diff))
            };
            let (mut a0, overflow_1) = x_0.overflowing_add(y_1 >> (exp_diff - Limb::WIDTH));
            let (a1, overflow_2) = if overflow_1 {
                x_1.overflowing_add(1)
            } else {
                (x_1, false)
            };
            if overflow_2 {
                sticky_bit |= a0 & 1;
                // shift a by 1
                a0 = (a1 << WIDTH_M1) | (a0 >> 1);
                x_exp = x_exp.checked_add(1).unwrap();
                let round_bit = a0 & shift_m1_bit;
                (
                    a0 & !mask,
                    HIGH_BIT | (a1 >> 1),
                    a0 & shift_m1_bit,
                    sticky_bit | (a0 & mask) ^ round_bit,
                )
            } else {
                let round_bit = a0 & shift_m1_bit;
                (
                    a0 & !mask,
                    a1,
                    round_bit,
                    sticky_bit | (a0 & mask) ^ round_bit,
                )
            }
        } else {
            // - exp_diff >= TWICE_WIDTH
            // - round_bit == 0 since prec < TWICE_WIDTH
            // - sticky_bit == since y != 0
            (x_0, x_1, 0, 1)
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (sum_0, sum_1, x_exp, Equal)
    } else {
        match rm {
            Exact => panic!("Inexact float addition"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (sum_0 & shift_bit) == 0) {
                    (sum_0, sum_1, x_exp, Less)
                } else if sum_0.overflowing_add_assign(shift_bit) && sum_1.overflowing_add_assign(1)
                {
                    (sum_0, HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum_0, sum_1, x_exp, Greater)
                }
            }
            Floor | Down => (sum_0, sum_1, x_exp, Less),
            Ceiling | Up => {
                if sum_0.overflowing_add_assign(shift_bit) && sum_1.overflowing_add_assign(1) {
                    (sum_0, HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum_0, sum_1, x_exp, Greater)
                }
            }
        }
    }
}

// This is mpfr_add1sp2n from add1sp.c, MPFR 4.2.0.
fn add_float_significands_same_prec_2w(
    mut x_0: Limb,
    mut x_1: Limb,
    mut x_exp: i32,
    mut y_0: Limb,
    mut y_1: Limb,
    mut y_exp: i32,
    rm: RoundingMode,
) -> (Limb, Limb, i32, Ordering) {
    let (mut sum_0, mut sum_1, round_bit, sticky_bit) = if x_exp == y_exp {
        // Since x_1, y_1 >= HIGH_BIT, a carry always occurs.
        let (a0, overflow) = x_0.overflowing_add(y_0);
        let mut a1 = x_1.wrapping_add(y_1);
        if overflow {
            a1.wrapping_add_assign(1);
        }
        x_exp = x_exp.checked_add(1).unwrap();
        // Since x + y fits on prec + 1 bits, the sticky bit is zero.
        (
            (a1 << WIDTH_M1) | (a0 >> 1),
            HIGH_BIT | (a1 >> 1),
            a0 & 1,
            0,
        )
    } else {
        if x_exp < y_exp {
            swap(&mut x_0, &mut y_0);
            swap(&mut x_1, &mut y_1);
            swap(&mut x_exp, &mut y_exp);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        if exp_diff >= TWICE_WIDTH {
            if exp_diff == TWICE_WIDTH {
                (x_0, x_1, 1, Limb::from(y_0 != 0 || y_1 > HIGH_BIT))
            } else {
                (x_0, x_1, 0, 1)
            }
        } else {
            // First, compute (a0, a1) = x + (y >> exp_diff), and determine the sticky bit from the
            // bits shifted out such that (MSB, other bits) is regarded as (rounding bit, sticky
            // bit), assuming no carry.
            let (sum_0, sum_1, sticky_bit) = if exp_diff < Limb::WIDTH {
                // 0 < exp_diff < Limb::WIDTH
                let comp_diff = Limb::WIDTH - exp_diff;
                let (sum_0, overflow) = x_0.overflowing_add((y_1 << comp_diff) | (y_0 >> exp_diff));
                let mut sum_1 = x_1.wrapping_add(y_1 >> exp_diff);
                if overflow {
                    sum_1.wrapping_add_assign(1);
                }
                (sum_0, sum_1, y_0 << comp_diff)
            } else {
                // Limb::WIDTH <= exp_diff < Limb::WIDTH * 2 The most significant bit of sb should
                // be the rounding bit, while the other bits represent the sticky bit:
                // * If exp_diff = Limb::WIDTH, we get y_0;
                // * If exp_diff > Limb::WIDTH: we get the least exp_diff - Limb::WIDTH bits of y_1,
                //   and those from y_0 as the LSB of sticky_bit.
                let sticky_bit = if exp_diff == Limb::WIDTH {
                    y_0
                } else {
                    (y_1 << (TWICE_WIDTH - exp_diff)) | Limb::from(y_0 != 0)
                };
                let (sum_0, overflow) = x_0.overflowing_add(y_1 >> (exp_diff - Limb::WIDTH));
                (
                    sum_0,
                    if overflow { x_1.wrapping_add(1) } else { x_1 },
                    sticky_bit,
                )
            };
            if sum_1 < x_1 {
                // carry in high word
                let round_bit = sum_0 << WIDTH_M1;
                // Shift the result by 1 to the right.
                x_exp = x_exp.checked_add(1).unwrap();
                (
                    (sum_1 << WIDTH_M1) | (sum_0 >> 1),
                    HIGH_BIT | (sum_1 >> 1),
                    round_bit,
                    sticky_bit,
                )
            } else {
                (sum_0, sum_1, sticky_bit & HIGH_BIT, sticky_bit << 1)
            }
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (sum_0, sum_1, x_exp, Equal)
    } else {
        match rm {
            Exact => panic!("Inexact float addition"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (sum_0 & 1) == 0) {
                    (sum_0, sum_1, x_exp, Less)
                } else if sum_0.overflowing_add_assign(1) && sum_1.overflowing_add_assign(1) {
                    (sum_0, HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum_0, sum_1, x_exp, Greater)
                }
            }
            Floor | Down => (sum_0, sum_1, x_exp, Less),
            Ceiling | Up => {
                if sum_0.overflowing_add_assign(1) && sum_1.overflowing_add_assign(1) {
                    (sum_0, HIGH_BIT, x_exp.checked_add(1).unwrap(), Greater)
                } else {
                    (sum_0, sum_1, x_exp, Greater)
                }
            }
        }
    }
}

// This is mpfr_add1sp3 from add1sp.c, MPFR 4.2.0.
fn add_float_significands_same_prec_gt_2w_lt_3w(
    mut x_0: Limb,
    mut x_1: Limb,
    mut x_2: Limb,
    mut x_exp: i32,
    mut y_0: Limb,
    mut y_1: Limb,
    mut y_2: Limb,
    mut y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (Limb, Limb, Limb, i32, Ordering) {
    let shift = THRICE_WIDTH - prec;
    let shift_bit = Limb::power_of_2(shift);
    let shift_m1_bit = shift_bit >> 1;
    let (mut sum_0, mut sum_1, mut sum_2, round_bit, sticky_bit) = if x_exp == y_exp {
        // Since x_2, y_2 >= HIGH_BIT, a carry always occurs
        let (mut a0, overflow_1) = x_0.overflowing_add(y_0);
        let (mut a1, mut overflow_2) = x_1.overflowing_add(y_1);
        if overflow_1 {
            overflow_2 |= a1.overflowing_add_assign(1);
        }
        let mut a2 = x_2.wrapping_add(y_2);
        if overflow_2 || (a1 == x_1 && overflow_1) {
            a2.wrapping_add_assign(1);
        }
        // Since prec < 3 * Limb::WIDTH, we lose no bit in a0 >> 1.
        a0 = (a1 << WIDTH_M1) | (a0 >> 1);
        x_exp = x_exp.checked_add(1).unwrap();
        let round_bit = a0 & shift_m1_bit;
        // Since x + y fits on prec + 1 bits, the sticky bit is zero.
        (
            a0 ^ round_bit,
            (a2 << WIDTH_M1) | (a1 >> 1),
            HIGH_BIT | (a2 >> 1),
            round_bit,
            0,
        )
    } else {
        if x_exp < y_exp {
            swap(&mut x_0, &mut y_0);
            swap(&mut x_1, &mut y_1);
            swap(&mut x_2, &mut y_2);
            swap(&mut x_exp, &mut y_exp);
        }
        let exp_diff = u64::exact_from(x_exp - y_exp);
        let mask = shift_bit - 1;
        if exp_diff < Limb::WIDTH {
            // 0 < exp_diff < Limb::WIDTH
            let comp_diff = Limb::WIDTH - exp_diff;
            let mut sticky_bit = y_0 << comp_diff; // bits from y[-1] after shift
            let (mut a0, overflow_1) = x_0.overflowing_add((y_1 << comp_diff) | (y_0 >> exp_diff));
            let (mut a1, mut overflow_2) =
                x_1.overflowing_add((y_2 << comp_diff) | (y_1 >> exp_diff));
            if overflow_1 {
                overflow_2 |= a1.overflowing_add_assign(1);
            }
            let (mut a2, mut overflow_3) = x_2.overflowing_add(y_2 >> exp_diff);
            if overflow_2 {
                overflow_3 |= a2.overflowing_add_assign(1);
            }
            let (sum_1, sum_2) = if overflow_3 || (a2 == x_2 && overflow_2) {
                sticky_bit |= a0 & 1;
                // shift a by 1
                a0 = (a1 << WIDTH_M1) | (a0 >> 1);
                x_exp = x_exp.checked_add(1).unwrap();
                ((a2 << WIDTH_M1) | (a1 >> 1), HIGH_BIT | (a2 >> 1))
            } else {
                (a1, a2)
            };
            let round_bit = a0 & shift_m1_bit;
            sticky_bit |= (a0 & mask) ^ round_bit;
            (a0 & !mask, sum_1, sum_2, round_bit, sticky_bit)
        } else if exp_diff < TWICE_WIDTH {
            // Limb::WIDTH <= exp_diff < Limb::WIDTH * 2
            let comp_diff = exp_diff - Limb::WIDTH;
            let comp_diff_2 = TWICE_WIDTH - exp_diff;
            let mut sticky_bit = if exp_diff == Limb::WIDTH {
                y_0
            } else {
                (y_1 << comp_diff_2) | y_0
            };
            let y0shifted = if exp_diff == Limb::WIDTH {
                y_1
            } else {
                (y_2 << comp_diff_2) | (y_1 >> comp_diff)
            };
            let (mut a0, overflow_1) = x_0.overflowing_add(y0shifted);
            let (mut a1, mut overflow_2) = x_1.overflowing_add(y_2 >> comp_diff);
            if overflow_1 {
                overflow_2 |= a1.overflowing_add_assign(1);
            }
            // If a1 < x_1, there was a carry in the above addition, or when a1 = x_1 and one of the
            // added terms is nonzero (the sum of b_2 >> (exp_diff - Limb::WIDTH) and a0 < x_0 is at
            // most 2 ^ Limb::WIDTH - exp_diff)
            let (a2, overflow_3) = if overflow_2 || (a1 == x_1 && overflow_1) {
                x_2.overflowing_add(1)
            } else {
                (x_2, false)
            };
            if overflow_3 {
                sticky_bit |= a0 & 1;
                // shift a by 1
                a0 = (a1 << WIDTH_M1) | (a0 >> 1);
                x_exp = x_exp.checked_add(1).unwrap();
                let round_bit = a0 & shift_m1_bit;
                sticky_bit |= (a0 & mask) ^ round_bit;
                (
                    a0 & !mask,
                    (a2 << WIDTH_M1) | (a1 >> 1),
                    HIGH_BIT | (a2 >> 1),
                    round_bit,
                    sticky_bit,
                )
            } else {
                let round_bit = a0 & shift_m1_bit;
                sticky_bit |= (a0 & mask) ^ round_bit;
                (a0 & !mask, a1, a2, round_bit, sticky_bit)
            }
        } else if exp_diff < THRICE_WIDTH {
            // Limb::WIDTH * 2 <= exp_diff < Limb::WIDTH * 3
            let mut sticky_bit = if exp_diff == TWICE_WIDTH {
                y_1 | y_0
            } else {
                (y_2 << (THRICE_WIDTH - exp_diff)) | y_1 | y_0
            };
            let (mut a0, overflow_1) = x_0.overflowing_add(y_2 >> (exp_diff - TWICE_WIDTH));
            let (a1, overflow_2) = if overflow_1 {
                x_1.overflowing_add(1)
            } else {
                (x_1, false)
            };
            let a2 = if overflow_2 { x_2.wrapping_add(1) } else { x_2 };
            if a2 == 0 {
                sticky_bit |= a0 & 1;
                // shift a by 1
                a0 = (a1 << WIDTH_M1) | (a0 >> 1);
                x_exp = x_exp.checked_add(1).unwrap();
                let round_bit = a0 & shift_m1_bit;
                sticky_bit |= (a0 & mask) ^ round_bit;
                (
                    a0 & !mask,
                    (a2 << WIDTH_M1) | (a1 >> 1),
                    HIGH_BIT | (a2 >> 1),
                    round_bit,
                    sticky_bit,
                )
            } else {
                let round_bit = a0 & shift_m1_bit;
                sticky_bit |= (a0 & mask) ^ round_bit;
                (a0 & !mask, a1, a2, round_bit, sticky_bit)
            }
        } else {
            // - exp_diff >= Limb::WIDTH * 2
            // - round_bit == 0 since prec < Limb::WIDTH * 3
            // - sticky_bit == 1 since c != 0
            (x_0, x_1, x_2, 0, 1)
        }
    };
    if round_bit == 0 && sticky_bit == 0 {
        (sum_0, sum_1, sum_2, x_exp, Equal)
    } else {
        match rm {
            Exact => panic!("Inexact float addition"),
            Nearest => {
                if round_bit == 0 || (sticky_bit == 0 && (sum_0 & shift_bit) == 0) {
                    (sum_0, sum_1, sum_2, x_exp, Less)
                } else {
                    if sum_0.overflowing_add_assign(shift_bit) {
                        sum_1.wrapping_add_assign(1);
                    }
                    if sum_1 == 0 && sum_0 == 0 {
                        sum_2.wrapping_add_assign(1);
                    }
                    if sum_2 == 0 {
                        (
                            sum_0,
                            sum_1,
                            HIGH_BIT,
                            x_exp.checked_add(1).unwrap(),
                            Greater,
                        )
                    } else {
                        (sum_0, sum_1, sum_2, x_exp, Greater)
                    }
                }
            }
            Floor | Down => (sum_0, sum_1, sum_2, x_exp, Less),
            Ceiling | Up => {
                if sum_0.overflowing_add_assign(shift_bit) {
                    sum_1.wrapping_add_assign(1);
                }
                if sum_1 == 0 && sum_0 == 0 {
                    sum_2.wrapping_add_assign(1);
                }
                if sum_2 == 0 {
                    (
                        sum_0,
                        sum_1,
                        HIGH_BIT,
                        x_exp.checked_add(1).unwrap(),
                        Greater,
                    )
                } else {
                    (sum_0, sum_1, sum_2, x_exp, Greater)
                }
            }
        }
    }
}

// out <- x + y >> r where d = q * Limb::WIDTH + r. Return the carry at out[n + 1] (0 or 1) and
// return low so that:
// * the most significant bit of low would be that of out[-1] if we would compute one more limb of
//   the (infinite) addition
// * the Limb::WIDTH - 1 least significant bits of low are zero iff all bits of out[-1], out[-2],
//   ... would be zero (except the most significant bit of out[-1]).
//
// Assume 0 < exp_diff < Limb::WIDTH * n.
//
// This is mpfr_addrsh from add1sp.c, MPFR 4.2.0, returning `low` before `carry`, where ap != bp and
// ap != cp.
fn add_significands_rsh_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    exp_diff: u64,
) -> (Limb, bool) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    let out = &mut out[..n];
    if exp_diff < Limb::WIDTH {
        // out <- x + y >> d
        assert_ne!(exp_diff, 0);
        let comp_diff = Limb::WIDTH - exp_diff;
        // Thus 0 < Limb::WIDTH - exp_diff < Limb::WIDTH
        let low = ys[0] << comp_diff;
        let mut carry = false;
        let (out_last, out_init) = out.split_last_mut().unwrap();
        let (xs_last, xs_init) = xs.split_last().unwrap();
        for (i, (o, &x)) in out_init.iter_mut().zip(xs_init.iter()).enumerate() {
            let mut carry_2;
            (*o, carry_2) = x.overflowing_add((ys[i + 1] << comp_diff) | (ys[i] >> exp_diff));
            if carry {
                carry_2 |= o.overflowing_add_assign(1);
            }
            carry = carry_2;
        }
        // most significant limb is special
        let mut carry_2;
        (*out_last, carry_2) = xs_last.overflowing_add(ys[n - 1] >> exp_diff);
        if carry {
            carry_2 |= out_last.overflowing_add_assign(1);
        }
        (low, carry_2)
    } else {
        // exp_diff >= Limb::WIDTH
        let q = usize::exact_from(exp_diff >> Limb::LOG_WIDTH);
        let r = exp_diff & Limb::WIDTH_MASK;
        if r == 0 {
            assert_ne!(q, 0);
            let (ys_lo, ys_hi) = ys.split_at(q);
            let (ys_mid, ys_lo) = ys_lo.split_last().unwrap();
            let mut low = *ys_mid;
            if !slice_test_zero(ys_lo) {
                low |= 1;
            }
            let nmq = n - q;
            let (out_lo, out_hi) = out.split_at_mut(nmq);
            let (xs_lo, xs_hi) = xs.split_at(nmq);
            let carry = if limbs_add_same_length_to_out(out_lo, xs_lo, ys_hi) {
                limbs_add_limb_to_out(out_hi, xs_hi, 1)
            } else {
                out_hi.copy_from_slice(xs_hi);
                false
            };
            (low, carry)
        } else {
            // 0 < r < Limb::WIDTH
            let comp_diff = Limb::WIDTH - r;
            let (ys_lo, ys_hi) = ys.split_at(q);
            let mut low = ys_hi[0] << comp_diff;
            if !slice_test_zero(ys_lo) {
                low |= 1;
            }
            let nmq = n - q;
            let (out_lo, out_hi) = out.split_at_mut(nmq);
            let (out_lo_last, out_lo_init) = out_lo.split_last_mut().unwrap();
            let (xs_lo, xs_hi) = xs.split_at(nmq);
            let (xs_lo_last, xs_lo_init) = xs_lo.split_last().unwrap();
            let mut carry = false;
            for (i, (o, &x)) in out_lo_init.iter_mut().zip(xs_lo_init).enumerate() {
                let mut carry_2;
                (*o, carry_2) = x.overflowing_add((ys_hi[i + 1] << comp_diff) | (ys_hi[i] >> r));
                if carry {
                    carry_2 |= o.overflowing_add_assign(1);
                }
                carry = carry_2;
            }
            // most significant limb of y is special
            let mut carry_2;
            (*out_lo_last, carry_2) = xs_lo_last.overflowing_add(ys[n - 1] >> r);
            if carry {
                carry_2 |= out_lo_last.overflowing_add_assign(1);
            }
            // upper limbs are copied
            let carry = if carry_2 {
                limbs_add_limb_to_out(out_hi, xs_hi, 1)
            } else {
                out_hi.copy_from_slice(xs_hi);
                false
            };
            (low, carry)
        }
    }
}

// x <- x + y >> r where d = q * Limb::WIDTH + r. Return the carry at x[n + 1] (0 or 1) and return
// low so that:
// * the most significant bit of low would be that of x[-1] if we would compute one more limb of the
//   (infinite) addition
// * the Limb::WIDTH - 1 least significant bits of low are zero iff all bits of x[-1], x[-2], ...
//   would be zero (except the most significant bit of x[-1]).
//
// Assume 0 < exp_diff < Limb::WIDTH * n.
//
// This is mpfr_addrsh from add1sp.c, MPFR 4.2.0, returning `low` before `carry`, where ap == bp.
fn add_significands_rsh_mut_ref(xs: &mut [Limb], ys: &[Limb], exp_diff: u64) -> (Limb, bool) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if exp_diff < Limb::WIDTH {
        // x <- x + y >> d
        assert_ne!(exp_diff, 0);
        let comp_diff = Limb::WIDTH - exp_diff;
        // Thus 0 < Limb::WIDTH - exp_diff < Limb::WIDTH
        let low = ys[0] << comp_diff;
        let mut carry = false;
        let (xs_last, xs_init) = xs.split_last_mut().unwrap();
        for (i, x) in xs_init.iter_mut().enumerate() {
            let mut carry_2;
            (*x, carry_2) = x.overflowing_add((ys[i + 1] << comp_diff) | (ys[i] >> exp_diff));
            if carry {
                carry_2 |= x.overflowing_add_assign(1);
            }
            carry = carry_2;
        }
        // most significant limb is special
        let mut carry_2 = xs_last.overflowing_add_assign(ys[n - 1] >> exp_diff);
        if carry {
            carry_2 |= xs_last.overflowing_add_assign(1);
        }
        (low, carry_2)
    } else {
        // exp_diff >= Limb::WIDTH
        let q = usize::exact_from(exp_diff >> Limb::LOG_WIDTH);
        let r = exp_diff & Limb::WIDTH_MASK;
        if r == 0 {
            assert_ne!(q, 0);
            let (ys_lo, ys_hi) = ys.split_at(q);
            let (ys_mid, ys_lo) = ys_lo.split_last().unwrap();
            let mut low = *ys_mid;
            if !slice_test_zero(ys_lo) {
                low |= 1;
            }
            let (xs_lo, xs_hi) = xs.split_at_mut(n - q);
            let carry = limbs_slice_add_same_length_in_place_left(xs_lo, ys_hi)
                && limbs_slice_add_limb_in_place(xs_hi, 1);
            (low, carry)
        } else {
            // 0 < r < Limb::WIDTH
            let comp_diff = Limb::WIDTH - r;
            let (ys_lo, ys_hi) = ys.split_at(q);
            let mut low = ys_hi[0] << comp_diff;
            if !slice_test_zero(ys_lo) {
                low |= 1;
            }
            let (xs_lo, xs_hi) = xs.split_at_mut(n - q);
            let (xs_lo_last, xs_lo_init) = xs_lo.split_last_mut().unwrap();
            let mut carry = false;
            for (i, x) in xs_lo_init.iter_mut().enumerate() {
                let mut carry_2;
                (*x, carry_2) = x.overflowing_add((ys_hi[i + 1] << comp_diff) | (ys_hi[i] >> r));
                if carry {
                    carry_2 |= x.overflowing_add_assign(1);
                }
                carry = carry_2;
            }
            // most significant limb of y is special
            let mut carry_2 = xs_lo_last.overflowing_add_assign(ys[n - 1] >> r);
            if carry {
                carry_2 |= xs_lo_last.overflowing_add_assign(1);
            }
            // upper limbs are copied
            let carry = if carry_2 {
                limbs_slice_add_limb_in_place(xs_hi, 1)
            } else {
                false
            };
            (low, carry)
        }
    }
}

// y <- x + y >> r where d = q * Limb::WIDTH + r. Return the carry at y[n + 1] (0 or 1) and return
// low so that:
// * the most significant bit of low would be that of y[-1] if we would compute one more limb of the
//   (infinite) addition
// * the Limb::WIDTH - 1 least significant bits of low are zero iff all bits of y[-1], y[-2], ...
//   would be zero (except the most significant bit of y[-1]).
//
// Assume 0 < exp_diff < Limb::WIDTH * n.
//
// This is mpfr_addrsh from add1sp.c, MPFR 4.2.0, returning `low` before `carry`, where ap == cp.
fn add_significands_rsh_ref_mut(xs: &[Limb], ys: &mut [Limb], exp_diff: u64) -> (Limb, bool) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    if exp_diff < Limb::WIDTH {
        // y <- x + y >> d
        assert_ne!(exp_diff, 0);
        let comp_diff = Limb::WIDTH - exp_diff;
        // Thus 0 < Limb::WIDTH - exp_diff < Limb::WIDTH
        let low = ys[0] << comp_diff;
        let mut carry = false;
        let (xs_last, xs_init) = xs.split_last().unwrap();
        for (i, x) in xs_init.iter().enumerate() {
            let mut carry_2;
            (ys[i], carry_2) = x.overflowing_add((ys[i + 1] << comp_diff) | (ys[i] >> exp_diff));
            if carry {
                carry_2 |= ys[i].overflowing_add_assign(1);
            }
            carry = carry_2;
        }
        // most significant limb is special
        let ys_last = ys.last_mut().unwrap();
        let mut carry_2;
        (*ys_last, carry_2) = xs_last.overflowing_add(*ys_last >> exp_diff);
        if carry {
            carry_2 |= ys_last.overflowing_add_assign(1);
        }
        (low, carry_2)
    } else {
        // exp_diff >= Limb::WIDTH
        let q = usize::exact_from(exp_diff >> Limb::LOG_WIDTH);
        let r = exp_diff & Limb::WIDTH_MASK;
        let nmq = n - q;
        if r == 0 {
            assert_ne!(q, 0);
            let (ys_mid, ys_lo) = ys[..q].split_last_mut().unwrap();
            let mut low = *ys_mid;
            if !slice_test_zero(ys_lo) {
                low |= 1;
            }
            let (xs_lo, xs_hi) = xs.split_at(nmq);
            let carry = if limbs_add_to_out_aliased_2(ys, q, xs_lo) {
                limbs_add_limb_to_out(&mut ys[nmq..], xs_hi, 1)
            } else {
                ys[nmq..].copy_from_slice(xs_hi);
                false
            };
            (low, carry)
        } else {
            // 0 < r < Limb::WIDTH
            let comp_diff = Limb::WIDTH - r;
            let last_ys = ys[n - 1];
            let (ys_lo, ys_hi) = ys.split_at(q);
            let mut low = ys_hi[0] << comp_diff;
            if !slice_test_zero(ys_lo) {
                low |= 1;
            }
            let (xs_lo, xs_hi) = xs.split_at(nmq);
            let (xs_lo_last, xs_lo_init) = xs_lo.split_last().unwrap();
            let mut carry = false;
            for (i, &x) in xs_lo_init.iter().enumerate() {
                let qpi = q + i;
                let mut carry_2;
                (ys[i], carry_2) = x.overflowing_add((ys[qpi + 1] << comp_diff) | (ys[qpi] >> r));
                if carry {
                    carry_2 |= ys[i].overflowing_add_assign(1);
                }
                carry = carry_2;
            }
            // most significant limb of y is special
            let (ys_lo, ys_hi) = ys.split_at_mut(nmq);
            let ys_lo_last = ys_lo.last_mut().unwrap();
            let mut carry_2;
            (*ys_lo_last, carry_2) = xs_lo_last.overflowing_add(last_ys >> r);
            if carry {
                carry_2 |= ys_lo_last.overflowing_add_assign(1);
            }
            let carry = if carry_2 {
                limbs_add_limb_to_out(ys_hi, xs_hi, 1)
            } else {
                ys_hi.copy_from_slice(xs_hi);
                false
            };
            (low, carry)
        }
    }
}

fn add_float_significands_same_prec_ge_3w_ref_ref<'a>(
    out: &mut [Limb],
    mut xs: &'a [Limb],
    mut x_exp: i32,
    mut ys: &'a [Limb],
    mut y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    if x_exp < y_exp {
        swap(&mut xs, &mut ys);
        swap(&mut x_exp, &mut y_exp);
    }
    let n = xs.len();
    assert_eq!(ys.len(), n);
    let out = &mut out[..n];
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let shift_m1_bit = shift_bit >> 1;
    let exp_diff = u64::exact_from(x_exp - y_exp);
    let mut round_bit;
    let mut sticky_bit;
    let last_index = n - 1;
    if exp_diff == 0 {
        x_exp = x_exp.checked_add(1).unwrap();
        assert!(limbs_add_same_length_to_out(out, xs, ys));
        round_bit = out[0] & shift_bit;
        limbs_slice_shr_in_place(out, 1);
        out[last_index] |= HIGH_BIT;
        out[0] &= !(shift_bit - 1);
        if round_bit == 0 {
            (x_exp, Equal)
        } else {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest => {
                    if out[0] & shift_bit == 0 {
                        (x_exp, Less)
                    } else {
                        if limbs_slice_add_limb_in_place(out, shift_bit) {
                            fail_on_untested_path(
                                "add_float_significands_same_prec_ge_3w, exp_diff == 0 && \
                                rm == Nearest && out[0] & shift_bit != 0 && carry",
                            );
                            x_exp = x_exp.checked_add(1).unwrap();
                            out[last_index] = HIGH_BIT;
                        }
                        (x_exp, Greater)
                    }
                }
                Floor | Down => (x_exp, Less),
                Ceiling | Up => {
                    if limbs_slice_add_limb_in_place(out, shift_bit) {
                        fail_on_untested_path(
                            "exp_diff == 0 && (rm == Ceiling || rm == Up) && carry",
                        );
                        x_exp = x_exp.checked_add(1).unwrap();
                        out[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        }
    } else if exp_diff >= prec {
        if exp_diff > prec {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest | Floor | Down => {
                    out.copy_from_slice(xs);
                    (x_exp, Less)
                }
                Ceiling | Up => {
                    out.copy_from_slice(xs);
                    if limbs_slice_add_limb_in_place(out, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        out[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        } else {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest => {
                    // Check if y was a power of 2
                    if limbs_is_power_of_2(ys) && xs[0] & shift_bit == 0 {
                        out.copy_from_slice(xs);
                        (x_exp, Less)
                    } else {
                        out.copy_from_slice(xs);
                        if limbs_slice_add_limb_in_place(out, shift_bit) {
                            x_exp = x_exp.checked_add(1).unwrap();
                            out[last_index] = HIGH_BIT;
                        }
                        (x_exp, Greater)
                    }
                }
                Floor | Down => {
                    out.copy_from_slice(xs);
                    (x_exp, Less)
                }
                Ceiling | Up => {
                    out.copy_from_slice(xs);
                    if limbs_slice_add_limb_in_place(out, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        out[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        }
    } else {
        // - 0 < exp_diff < prec
        // - General case: 1 <= exp_diff < prec
        let mask = !(shift_bit - 1);
        let carry;
        (sticky_bit, carry) = add_significands_rsh_to_out(out, xs, ys, exp_diff);
        // The most significant bit of sticky_bit contains what would be the most significant bit of
        // out[-1], and the remaining bits of stick_bit are 0 iff the remaining bits of out[-1],
        // out[-2], ... are all zero
        if shift != 0 {
            // The round bit and a part of the sticky bit are in out[0].
            round_bit = out[0] & shift_m1_bit;
            sticky_bit |= out[0] & (shift_m1_bit - 1);
        } else {
            // The round bit and possibly a part of the sticky bit are in sticky_bit
            round_bit = sticky_bit & HIGH_BIT;
            sticky_bit &= !HIGH_BIT;
        }
        out[0] &= mask;
        // Check for carry out
        if carry {
            sticky_bit |= round_bit;
            round_bit = out[0] & shift_bit;
            limbs_slice_shr_in_place(out, 1);
            x_exp = x_exp.checked_add(1).unwrap();
            out[last_index] |= HIGH_BIT;
            out[0] &= mask;
        }
        match rm {
            Nearest => {
                if round_bit == 0 {
                    (x_exp, if sticky_bit != 0 { Less } else { Equal })
                } else if sticky_bit == 0 && out[0] & shift_bit == 0 {
                    (x_exp, Less)
                } else {
                    if limbs_slice_add_limb_in_place(out, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        out[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
            Floor | Down | Exact => {
                let inexact = round_bit != 0 || sticky_bit != 0;
                if rm == Exact && inexact {
                    panic!("Inexact float addition");
                } else {
                    (x_exp, if inexact { Less } else { Equal })
                }
            }
            Ceiling | Up => {
                if round_bit != 0 || sticky_bit != 0 {
                    if limbs_slice_add_limb_in_place(out, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        out[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                } else {
                    (x_exp, Equal)
                }
            }
        }
    }
}

fn add_float_significands_same_prec_ge_3w_val_val<'a>(
    xs: &'a mut [Limb],
    x_exp: i32,
    ys: &'a mut [Limb],
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering, bool) {
    if x_exp >= y_exp {
        let (exp, o) =
            add_float_significands_same_prec_ge_3w_val_ref(xs, x_exp, ys, y_exp, prec, rm);
        (exp, o, false)
    } else {
        let (exp, o) =
            add_float_significands_same_prec_ge_3w_val_ref(ys, y_exp, xs, x_exp, prec, rm);
        (exp, o, true)
    }
}

fn add_float_significands_same_prec_ge_3w_val_ref(
    xs: &mut [Limb],
    mut x_exp: i32,
    ys: &[Limb],
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let shift_m1_bit = shift_bit >> 1;
    let exp_diff = u64::exact_from(x_exp - y_exp);
    let mut round_bit;
    let mut sticky_bit;
    let last_index = n - 1;
    if exp_diff == 0 {
        x_exp = x_exp.checked_add(1).unwrap();
        assert!(limbs_slice_add_same_length_in_place_left(xs, ys));
        round_bit = xs[0] & shift_bit;
        limbs_slice_shr_in_place(xs, 1);
        xs[last_index] |= HIGH_BIT;
        xs[0] &= !(shift_bit - 1);
        if round_bit == 0 {
            (x_exp, Equal)
        } else {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest => {
                    if xs[0] & shift_bit == 0 {
                        (x_exp, Less)
                    } else {
                        if limbs_slice_add_limb_in_place(xs, shift_bit) {
                            fail_on_untested_path(
                                "add_float_significands_same_prec_ge_3w, exp_diff == 0 && \
                                rm == Nearest && xs[0] & shift_bit != 0 && carry",
                            );
                            x_exp = x_exp.checked_add(1).unwrap();
                            xs[last_index] = HIGH_BIT;
                        }
                        (x_exp, Greater)
                    }
                }
                Floor | Down => (x_exp, Less),
                Ceiling | Up => {
                    if limbs_slice_add_limb_in_place(xs, shift_bit) {
                        fail_on_untested_path(
                            "exp_diff == 0 && (rm == Ceiling || rm == Up) && carry",
                        );
                        x_exp = x_exp.checked_add(1).unwrap();
                        xs[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        }
    } else if exp_diff >= prec {
        if exp_diff > prec {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest | Floor | Down => (x_exp, Less),
                Ceiling | Up => {
                    if limbs_slice_add_limb_in_place(xs, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        xs[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        } else {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest => {
                    // Check if y was a power of 2
                    if limbs_is_power_of_2(ys) && xs[0] & shift_bit == 0 {
                        (x_exp, Less)
                    } else {
                        if limbs_slice_add_limb_in_place(xs, shift_bit) {
                            x_exp = x_exp.checked_add(1).unwrap();
                            xs[last_index] = HIGH_BIT;
                        }
                        (x_exp, Greater)
                    }
                }
                Floor | Down => (x_exp, Less),
                Ceiling | Up => {
                    if limbs_slice_add_limb_in_place(xs, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        xs[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        }
    } else {
        // - 0 < exp_diff < prec
        // - General case: 1 <= exp_diff < prec
        let mask = !(shift_bit - 1);
        let carry;
        (sticky_bit, carry) = add_significands_rsh_mut_ref(xs, ys, exp_diff);
        // The most significant bit of sticky_bit contains what would be the most significant bit of
        // out[-1], and the remaining bits of stick_bit are 0 iff the remaining bits of out[-1],
        // out[-2], ... are all zero
        if shift != 0 {
            // The round bit and a part of the sticky bit are in out[0].
            round_bit = xs[0] & shift_m1_bit;
            sticky_bit |= xs[0] & (shift_m1_bit - 1);
        } else {
            // The round bit and possibly a part of the sticky bit are in sticky_bit
            round_bit = sticky_bit & HIGH_BIT;
            sticky_bit &= !HIGH_BIT;
        }
        xs[0] &= mask;
        // Check for carry out
        if carry {
            sticky_bit |= round_bit;
            round_bit = xs[0] & shift_bit;
            limbs_slice_shr_in_place(xs, 1);
            x_exp = x_exp.checked_add(1).unwrap();
            xs[last_index] |= HIGH_BIT;
            xs[0] &= mask;
        }
        match rm {
            Nearest => {
                if round_bit == 0 {
                    (x_exp, if sticky_bit != 0 { Less } else { Equal })
                } else if sticky_bit == 0 && xs[0] & shift_bit == 0 {
                    (x_exp, Less)
                } else {
                    if limbs_slice_add_limb_in_place(xs, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        xs[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
            Floor | Down | Exact => {
                let inexact = round_bit != 0 || sticky_bit != 0;
                if rm == Exact && inexact {
                    panic!("Inexact float addition");
                } else {
                    (x_exp, if inexact { Less } else { Equal })
                }
            }
            Ceiling | Up => {
                if round_bit != 0 || sticky_bit != 0 {
                    if limbs_slice_add_limb_in_place(xs, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        xs[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                } else {
                    (x_exp, Equal)
                }
            }
        }
    }
}

fn add_float_significands_same_prec_ge_3w_ref_val(
    xs: &[Limb],
    mut x_exp: i32,
    ys: &mut [Limb],
    y_exp: i32,
    prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    let n = xs.len();
    assert_eq!(ys.len(), n);
    let shift = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let shift_bit = Limb::power_of_2(shift);
    let shift_m1_bit = shift_bit >> 1;
    let exp_diff = u64::exact_from(x_exp - y_exp);
    let mut round_bit;
    let mut sticky_bit;
    let last_index = n - 1;
    if exp_diff == 0 {
        x_exp = x_exp.checked_add(1).unwrap();
        assert!(limbs_slice_add_same_length_in_place_left(ys, xs));
        round_bit = ys[0] & shift_bit;
        limbs_slice_shr_in_place(ys, 1);
        ys[last_index] |= HIGH_BIT;
        ys[0] &= !(shift_bit - 1);
        if round_bit == 0 {
            (x_exp, Equal)
        } else {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest => {
                    if ys[0] & shift_bit == 0 {
                        (x_exp, Less)
                    } else {
                        if limbs_slice_add_limb_in_place(ys, shift_bit) {
                            fail_on_untested_path(
                                "add_float_significands_same_prec_ge_3w, exp_diff == 0 && \
                                rm == Nearest && out[0] & shift_bit != 0 && carry",
                            );
                            x_exp = x_exp.checked_add(1).unwrap();
                            ys[last_index] = HIGH_BIT;
                        }
                        (x_exp, Greater)
                    }
                }
                Floor | Down => (x_exp, Less),
                Ceiling | Up => {
                    if limbs_slice_add_limb_in_place(ys, shift_bit) {
                        fail_on_untested_path(
                            "exp_diff == 0 && (rm == Ceiling || rm == Up) && carry",
                        );
                        x_exp = x_exp.checked_add(1).unwrap();
                        ys[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        }
    } else if exp_diff >= prec {
        if exp_diff > prec {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest | Floor | Down => {
                    ys.copy_from_slice(xs);
                    (x_exp, Less)
                }
                Ceiling | Up => {
                    ys.copy_from_slice(xs);
                    if limbs_slice_add_limb_in_place(ys, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        ys[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        } else {
            match rm {
                Exact => panic!("Inexact float addition"),
                Nearest => {
                    // Check if y was a power of 2
                    if limbs_is_power_of_2(ys) && xs[0] & shift_bit == 0 {
                        ys.copy_from_slice(xs);
                        (x_exp, Less)
                    } else {
                        ys.copy_from_slice(xs);
                        if limbs_slice_add_limb_in_place(ys, shift_bit) {
                            x_exp = x_exp.checked_add(1).unwrap();
                            ys[last_index] = HIGH_BIT;
                        }
                        (x_exp, Greater)
                    }
                }
                Floor | Down => {
                    ys.copy_from_slice(xs);
                    (x_exp, Less)
                }
                Ceiling | Up => {
                    ys.copy_from_slice(xs);
                    if limbs_slice_add_limb_in_place(ys, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        ys[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
        }
    } else {
        // - 0 < exp_diff < prec
        // - General case: 1 <= exp_diff < prec
        let mask = !(shift_bit - 1);
        let carry;
        (sticky_bit, carry) = add_significands_rsh_ref_mut(xs, ys, exp_diff);
        // The most significant bit of sticky_bit contains what would be the most significant bit of
        // y[-1], and the remaining bits of stick_bit are 0 iff the remaining bits of y[-1], y[-2],
        // ... are all zero
        if shift != 0 {
            // The round bit and a part of the sticky bit are in y[0].
            round_bit = ys[0] & shift_m1_bit;
            sticky_bit |= ys[0] & (shift_m1_bit - 1);
        } else {
            // The round bit and possibly a part of the sticky bit are in sticky_bit
            round_bit = sticky_bit & HIGH_BIT;
            sticky_bit &= !HIGH_BIT;
        }
        ys[0] &= mask;
        // Check for carry out
        if carry {
            sticky_bit |= round_bit;
            round_bit = ys[0] & shift_bit;
            limbs_slice_shr_in_place(ys, 1);
            x_exp = x_exp.checked_add(1).unwrap();
            ys[last_index] |= HIGH_BIT;
            ys[0] &= mask;
        }
        match rm {
            Nearest => {
                if round_bit == 0 {
                    (x_exp, if sticky_bit != 0 { Less } else { Equal })
                } else if sticky_bit == 0 && ys[0] & shift_bit == 0 {
                    (x_exp, Less)
                } else {
                    if limbs_slice_add_limb_in_place(ys, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        ys[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                }
            }
            Floor | Down | Exact => {
                let inexact = round_bit != 0 || sticky_bit != 0;
                if rm == Exact && inexact {
                    panic!("Inexact float addition");
                } else {
                    (x_exp, if inexact { Less } else { Equal })
                }
            }
            Ceiling | Up => {
                if round_bit != 0 || sticky_bit != 0 {
                    if limbs_slice_add_limb_in_place(ys, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        ys[last_index] = HIGH_BIT;
                    }
                    (x_exp, Greater)
                } else {
                    (x_exp, Equal)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum RoundBit {
    Uninitialized,
    False,
    True,
}

impl RoundBit {
    #[inline]
    fn flip_assign(&mut self) {
        match self {
            False => *self = True,
            True => *self = False,
            _ => {}
        }
    }
}

impl From<bool> for RoundBit {
    #[inline]
    fn from(b: bool) -> RoundBit {
        if b {
            True
        } else {
            False
        }
    }
}

fn add_float_significands_general_round(
    out: &mut [Limb],
    mut x_exp: i32,
    shift_bit: Limb,
    round_bit: RoundBit,
    following_bits: RoundBit,
    rm: RoundingMode,
) -> (i32, Ordering) {
    if following_bits == False && round_bit == False {
        return (x_exp, Equal);
    }
    match rm {
        Exact => panic!("Inexact float addition"),
        Nearest => {
            if following_bits == False {
                if out[0] & shift_bit != 0 {
                    if limbs_slice_add_limb_in_place(out, shift_bit) {
                        x_exp = x_exp.checked_add(1).unwrap();
                        *out.last_mut().unwrap() = HIGH_BIT;
                    }
                    (x_exp, Greater)
                } else {
                    (x_exp, Less)
                }
            } else if round_bit == False {
                (x_exp, Less)
            } else {
                if limbs_slice_add_limb_in_place(out, shift_bit) {
                    x_exp = x_exp.checked_add(1).unwrap();
                    *out.last_mut().unwrap() = HIGH_BIT;
                }
                (x_exp, Greater)
            }
        }
        Floor | Down => (x_exp, Less),
        Ceiling | Up => {
            if limbs_slice_add_limb_in_place(out, shift_bit) {
                x_exp = x_exp.checked_add(1).unwrap();
                *out.last_mut().unwrap() = HIGH_BIT;
            }
            (x_exp, Greater)
        }
    }
}

fn add_float_significands_general(
    out: &mut [Limb],
    xs: &[Limb],
    mut x_exp: i32,
    ys: &[Limb],
    y_exp: i32,
    out_prec: u64,
    rm: RoundingMode,
) -> (i32, Ordering) {
    assert!(x_exp >= y_exp);
    let out_len = out.len();
    let out_bits = u64::exact_from(out_len << Limb::LOG_WIDTH);
    let shift = out_bits - out_prec; // non-significant bits in low limb
    let shift_bit = Limb::power_of_2(shift);
    let xs_len = xs.len();
    let ys_len = ys.len();
    let exp_diff = u64::exact_from(x_exp - y_exp);
    let k = usize::exact_from(exp_diff >> Limb::LOG_WIDTH);
    // Compute the significant part out', the non-significant bits of out are taken into account.
    //
    // Perform the rounding. At each iteration, we remember:
    // - r = rounding bit
    // - f = following bits (same value)
    // where the result has the form: [number A]rfff...fff + a remaining value in the interval [0,2)
    // ulp. We consider the most significant bits of the remaining value to update the result; a
    // possible carry is immediately taken into account and out is updated accordingly. As soon as
    // the bits f don't have the same value, out can be rounded. Variables:
    //
    // - round_bit = rounding bit (0 or 1).
    // - following_bits = following bits (0 or 1), then sticky bit.
    // - If following_bits == 0, the only thing that can change is the sticky bit.
    // - means: not initialized
    let mut round_bit = Uninitialized;
    let mut following_bits = Uninitialized;
    if out_bits <= exp_diff {
        // y does not overlap with out'
        if out_len > xs_len {
            out[out_len - xs_len..].copy_from_slice(xs);
        } else {
            out.copy_from_slice(&xs[xs_len - out_len..]);
        }
    } else {
        // - out_bits > exp_diff
        // - y overlaps with out'
        // - copy y (shifted) into out
        // overlap is the number of limbs of y which overlap with out'
        let mut overlap =
            usize::exact_from((out_bits - exp_diff).shr_round(Limb::LOG_WIDTH, Ceiling).0);
        // only the highest overlap limbs from y have to be considered
        if overlap > ys_len {
            // y doesn't have enough limbs
            assert!(overlap - ys_len <= out_len);
            overlap = ys_len;
        }
        let ys_hi = &ys[ys_len - overlap..];
        let omk = out_len - k;
        let (out_lo, out_hi) = out.split_at_mut(omk - overlap);
        let out_hi = &mut out_hi[..overlap];
        let shift2 = u64::exact_from(exp_diff & Limb::WIDTH_MASK);
        if shift2 != 0 {
            assert!(omk >= overlap);
            let y = limbs_shr_to_out(out_hi, ys_hi, shift2);
            if omk > overlap {
                *out_lo.last_mut().unwrap() = y;
            }
        } else {
            out_hi.copy_from_slice(ys_hi);
        }
        // add x to out
        let y = if out_len > xs_len {
            limbs_slice_add_same_length_in_place_left(&mut out[out_len - xs_len..], xs)
        } else {
            limbs_slice_add_same_length_in_place_left(out, &xs[xs_len - out_len..])
        };
        if y {
            x_exp = x_exp.checked_add(1).unwrap();
            round_bit = RoundBit::from((out[0] >> shift).odd());
            // LSB(out) --> rounding bit after the shift
            if shift != 0 {
                let mask = shift_bit - 1;
                let x = out[0] & mask;
                out[0] &= !mask << 1;
                if x == 0 {
                    following_bits = False;
                } else if x == mask {
                    following_bits = True;
                }
            }
            limbs_slice_shr_in_place(out, 1);
            out[out_len - 1] |= HIGH_BIT;
            if shift != 0 && following_bits == Uninitialized {
                return add_float_significands_general_round(
                    out,
                    x_exp,
                    shift_bit,
                    round_bit,
                    following_bits,
                    rm,
                );
            }
        }
    }
    if round_bit == Uninitialized && shift != 0 {
        let mut mask = shift_bit - 1;
        let mut x = out[0] & mask;
        out[0] &= !mask;
        round_bit = RoundBit::from(x >> (shift - 1) != 0);
        if shift > 1 {
            mask >>= 1;
            x &= mask;
            if x == 0 {
                following_bits = False;
            } else if x == mask {
                following_bits = True;
            } else {
                return add_float_significands_general_round(
                    out,
                    x_exp,
                    shift_bit,
                    round_bit,
                    following_bits,
                    rm,
                );
            }
        }
    }
    // Determine rounding and sticky bits (and possible carry). In faithful rounding, we may stop
    // two bits after ulp(out): the approximation is regarded as the number formed by out, the
    // rounding bit round_bit and an additional bit following_bits; and the corresponding error is <
    // 1/2 ulp of the unrounded result.
    if xs_len > out_len {
        // there are still limbs from x that haven't been taken into account
        if following_bits == False && out_len <= k {
            // y hasn't been taken into account ==> sticky bit != 0
            return add_float_significands_general_round(
                out, x_exp, shift_bit, round_bit, True, rm,
            );
        }
        // index of lowest considered limb from x, > 0
        let mut xi = xs_len - out_len;
        for _ in 0..k.saturating_sub(out_len) {
            // ulp(next limb from x) > msb(y)
            xi -= 1;
            let mut x = xs[xi];
            assert_ne!(following_bits, False);
            if following_bits == True {
                // Note: Here, we can round to nearest, but the loop may still be necessary to
                // determine whether there is a carry from y, which will have an effect on the
                // ternary value.
                if x != Limb::MAX {
                    return add_float_significands_general_round(
                        out,
                        x_exp,
                        shift_bit,
                        round_bit,
                        following_bits,
                        rm,
                    );
                }
            } else {
                if round_bit == Uninitialized {
                    round_bit = RoundBit::from(x >> (Limb::WIDTH - 1) != 0);
                    x |= HIGH_BIT;
                }
                following_bits = True;
                if x != Limb::MAX {
                    return add_float_significands_general_round(
                        out, x_exp, shift_bit, round_bit, True, rm,
                    );
                }
            }
            if xi == 0 {
                // x has entirely been read y hasn't been taken into account, so sticky_bit != 0
                return add_float_significands_general_round(
                    out, x_exp, shift_bit, round_bit, True, rm,
                );
            }
        }
        let difw = out_len.saturating_sub(k);
        assert!(xi != 0);
        let mut goto_c_read = false;
        if difw <= ys_len {
            let mut yi = ys_len - difw + 1;
            let exp_diff_rem = exp_diff & Limb::WIDTH_MASK;
            if exp_diff_rem != 0 || yi != 1 {
                let mut y_prev = if yi - 1 == ys_len { 0 } else { ys[yi - 1] };
                if following_bits == Uninitialized {
                    let mut y;
                    if exp_diff_rem != 0 {
                        y = y_prev << (Limb::WIDTH - exp_diff_rem);
                        yi -= 1;
                        if yi != 0 {
                            y_prev = ys[yi - 1];
                            y += y_prev >> exp_diff_rem;
                        }
                    } else {
                        yi -= 1;
                        y = ys[yi - 1];
                    }
                    xi -= 1;
                    let mut x = xs[xi].wrapping_add(y);
                    if x < y
                        && (round_bit == Uninitialized || {
                            round_bit.flip_assign();
                            round_bit == False
                        })
                        && limbs_slice_add_limb_in_place(out, shift_bit)
                    {
                        x_exp = x_exp.checked_add(1).unwrap();
                        out[out_len - 1] = HIGH_BIT;
                        round_bit = False;
                    }
                    if round_bit == Uninitialized {
                        round_bit = RoundBit::from(x >> (Limb::WIDTH - 1) != 0);
                        x <<= 1;
                        x |= x >> (Limb::WIDTH - 1);
                    }
                    following_bits = RoundBit::from(x != 0);
                    if x != 0 && x != Limb::MAX {
                        return add_float_significands_general_round(
                            out,
                            x_exp,
                            shift_bit,
                            round_bit,
                            following_bits,
                            rm,
                        );
                    }
                }
                let mut y;
                while xi != 0 {
                    if exp_diff_rem != 0 {
                        if yi == 0 {
                            goto_c_read = true;
                            break;
                        }
                        y = y_prev << (Limb::WIDTH - exp_diff_rem);
                        yi -= 1;
                        if yi != 0 {
                            y_prev = ys[yi - 1];
                            y += y_prev >> exp_diff_rem;
                        }
                    } else {
                        if yi == 1 {
                            goto_c_read = true;
                            break;
                        }
                        yi -= 1;
                        y = ys[yi - 1];
                    }
                    let x = xs[xi - 1].wrapping_add(y);
                    if x < y {
                        following_bits.flip_assign();
                        if following_bits != False {
                            return add_float_significands_general_round(
                                out,
                                x_exp,
                                shift_bit,
                                round_bit,
                                following_bits,
                                rm,
                            );
                        }
                        round_bit.flip_assign();
                        if round_bit == False && limbs_slice_add_limb_in_place(out, shift_bit) {
                            x_exp = x_exp.checked_add(1).unwrap();
                            out[out_len - 1] = HIGH_BIT;
                        }
                    }
                    if following_bits == False && x != 0 {
                        return add_float_significands_general_round(
                            out, x_exp, shift_bit, round_bit, True, rm,
                        );
                    }
                    if following_bits != False && x != Limb::MAX {
                        return add_float_significands_general_round(
                            out,
                            x_exp,
                            shift_bit,
                            round_bit,
                            following_bits,
                            rm,
                        );
                    }
                    xi -= 1;
                }
                if !goto_c_read {
                    if following_bits != False || yi == 0 {
                        return add_float_significands_general_round(
                            out,
                            x_exp,
                            shift_bit,
                            round_bit,
                            following_bits,
                            rm,
                        );
                    }
                    if exp_diff_rem != 0 && y_prev << (Limb::WIDTH - exp_diff_rem) != 0 {
                        return add_float_significands_general_round(
                            out, x_exp, shift_bit, round_bit, True, rm,
                        );
                    }
                    yi -= 1;
                    while yi != 0 {
                        yi -= 1;
                        if ys[yi] != 0 {
                            return add_float_significands_general_round(
                                out, x_exp, shift_bit, round_bit, True, rm,
                            );
                        }
                    }
                }
            } else {
                goto_c_read = true;
            }
        }
        if difw > ys_len || goto_c_read {
            // - y has entirely been read
            // - label c_read:
            if following_bits == Uninitialized {
                assert_ne!(xi, 0);
                xi -= 1;
                let mut x = xs[xi];
                if round_bit == Uninitialized {
                    round_bit = RoundBit::from(x >> (Limb::WIDTH - 1) != 0);
                    x &= !HIGH_BIT;
                }
                following_bits = RoundBit::from(x != 0);
            }
            if following_bits == True {
                return add_float_significands_general_round(
                    out, x_exp, shift_bit, round_bit, True, rm,
                );
            }
            while xi != 0 {
                xi -= 1;
                if xs[xi] != 0 {
                    return add_float_significands_general_round(
                        out, x_exp, shift_bit, round_bit, True, rm,
                    );
                }
            }
        }
    } else if following_bits != True {
        if out_len.saturating_sub(k) > ys_len {
            if round_bit == Uninitialized {
                round_bit = False;
            }
            following_bits = False;
        } else if exp_diff > out_bits {
            // x is followed by at least a zero bit, then by y
            if round_bit == Uninitialized {
                round_bit = False;
            }
            following_bits = True;
        } else {
            // difw is the number of limbs from x (regarded as having an infinite precision) that
            // have already been combined with y; -n if the next n limbs from x won't be combined
            // with y.
            let difw = out_len - k;
            assert!(ys_len >= difw);
            let mut yi = ys_len - difw;
            let exp_diff_rem = exp_diff & Limb::WIDTH_MASK;
            if exp_diff_rem == 0 && yi == 0 {
                // y has entirely been read
                if round_bit == Uninitialized {
                    round_bit = False;
                }
                following_bits = False;
            } else {
                let mut y = if exp_diff_rem != 0 {
                    assert!(yi < ys_len);
                    ys[yi] << (Limb::WIDTH - exp_diff_rem)
                } else {
                    yi -= 1;
                    ys[yi]
                };
                if round_bit == Uninitialized {
                    round_bit = RoundBit::from(y >> (Limb::WIDTH - 1) != 0);
                    y &= !HIGH_BIT;
                }
                while y == 0 {
                    if yi == 0 {
                        return add_float_significands_general_round(
                            out, x_exp, shift_bit, round_bit, False, rm,
                        );
                    }
                    yi -= 1;
                    y = ys[yi];
                }
                following_bits = True;
            }
        }
    }
    add_float_significands_general_round(out, x_exp, shift_bit, round_bit, following_bits, rm)
}
