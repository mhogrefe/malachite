// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::AssignRound;

// rug's `root` binds `mpfr_rootn_ui`, but takes the root order as a `u32`; callers must restrict k
// accordingly.
pub fn rug_root_u_prec_round(
    x: &rug::Float,
    k: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut root = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = root.assign_round(x.root_ref(u32::exact_from(k)), rm);
    (root, o)
}

pub fn rug_root_u_prec(x: &rug::Float, k: u64, prec: u64) -> (rug::Float, Ordering) {
    rug_root_u_prec_round(x, k, prec, Round::Nearest)
}

pub fn rug_root_u_round(x: &rug::Float, k: u64, rm: Round) -> (rug::Float, Ordering) {
    rug_root_u_prec_round(x, k, rug_float_significant_bits(x), rm)
}

pub fn rug_root_u(x: &rug::Float, k: u64) -> rug::Float {
    rug_root_u_prec_round(x, k, rug_float_significant_bits(x), Round::Nearest).0
}

// rug's `root_i` binds `mpfr_rootn_si`, but takes the root order as an `i32`; callers must restrict
// k accordingly. NOTE: `mpfr_rootn_si` (MPFR 4.3.0) has a bug in its exact-case detection for k <
// -2: when |x| is a power of 2 whose exponent is not divisible by k and the |k|th root rounds to a
// power of 2 at the working precision, it wrongly returns that value as exact (for example,
// rootn_si(2, -50000) at low precision returns (1, Equal) instead of the correct rounding of
// 2^(-1/50000) ~ 0.99998614). Callers must exclude such inputs when using this as an oracle.
pub fn rug_root_s_prec_round(
    x: &rug::Float,
    k: i64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let mut root = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = root.assign_round(x.root_i_ref(i32::exact_from(k)), rm);
    (root, o)
}

pub fn rug_root_s_prec(x: &rug::Float, k: i64, prec: u64) -> (rug::Float, Ordering) {
    rug_root_s_prec_round(x, k, prec, Round::Nearest)
}

pub fn rug_root_s_round(x: &rug::Float, k: i64, rm: Round) -> (rug::Float, Ordering) {
    rug_root_s_prec_round(x, k, rug_float_significant_bits(x), rm)
}

pub fn rug_root_s(x: &rug::Float, k: i64) -> rug::Float {
    rug_root_s_prec_round(x, k, rug_float_significant_bits(x), Round::Nearest).0
}
