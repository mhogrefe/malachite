// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2000-2002, 2012 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::divisible_by::{
    limbs_divisible_by_limb, limbs_divisible_by_val_ref,
};
use crate::natural::arithmetic::eq_mod::limbs_eq_limb_mod_limb;
use crate::natural::arithmetic::mod_op::{limbs_mod, limbs_mod_limb};
use crate::natural::arithmetic::sub::{limbs_sub, limbs_sub_limb};
use crate::natural::comparison::cmp::limbs_cmp;
use crate::platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::slices::slice_trailing_zeros;
use std::cmp::Ordering::*;

pub fn limbs_eq_limb_mod_naive_1(xs: &[Limb], y: Limb, ms: &[Limb]) -> bool {
    assert!(xs.len() > 1);
    assert!(ms.len() > 1);
    let mut xs_mod = if xs.len() >= ms.len() {
        limbs_mod(xs, ms)
    } else {
        xs.to_vec()
    };
    xs_mod.truncate(xs_mod.len() - slice_trailing_zeros(&xs_mod));
    xs_mod == [y]
}

pub fn limbs_eq_limb_mod_naive_2(xs: &[Limb], y: Limb, ms: &[Limb]) -> bool {
    let mut diff = limbs_sub_limb(xs, y).0;
    diff.truncate(diff.len() - slice_trailing_zeros(&diff));
    diff.len() >= ms.len() && limbs_divisible_by_val_ref(&mut diff, ms)
}

pub fn limbs_eq_mod_limb_naive_1(xs: &[Limb], ys: &[Limb], ms: Limb) -> bool {
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    limbs_mod_limb(xs, ms) == limbs_mod_limb(ys, ms)
}

pub fn limbs_eq_mod_limb_naive_2(xs: &[Limb], ys: &[Limb], ms: Limb) -> bool {
    if xs == ys {
        return true;
    }
    let mut diff = if limbs_cmp(xs, ys) >= Equal {
        limbs_sub(xs, ys)
    } else {
        limbs_sub(ys, xs)
    }
    .0;
    diff.truncate(diff.len() - slice_trailing_zeros(&diff));
    if diff.len() == 1 {
        diff[0].divisible_by(ms)
    } else {
        limbs_divisible_by_limb(&diff, ms)
    }
}

pub fn limbs_eq_mod_naive_1(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    let mut xs_mod = if xs.len() >= ms.len() {
        limbs_mod(xs, ms)
    } else {
        xs.to_vec()
    };
    let mut ys_mod = if ys.len() >= ms.len() {
        limbs_mod(ys, ms)
    } else {
        ys.to_vec()
    };
    xs_mod.truncate(xs_mod.len() - slice_trailing_zeros(&xs_mod));
    ys_mod.truncate(ys_mod.len() - slice_trailing_zeros(&ys_mod));
    limbs_cmp(&xs_mod, &ys_mod) == Equal
}

pub fn limbs_eq_mod_naive_2(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    if xs == ys {
        return true;
    }
    let mut diff = if limbs_cmp(xs, ys) >= Equal {
        limbs_sub(xs, ys)
    } else {
        limbs_sub(ys, xs)
    }
    .0;
    diff.truncate(diff.len() - slice_trailing_zeros(&diff));
    diff.len() >= ms.len() && limbs_divisible_by_val_ref(&mut diff, ms)
}

/// Benchmarks show that this is never faster than just calling `limbs_eq_limb_mod_limb`.
///
/// xs.len() must be greater than 1; m must be nonzero.
///
/// This is equivalent to `mpz_congruent_ui_p` from `mpz/cong_ui.c`, GMP 6.2.1, where `a` is
/// non-negative.
pub fn combined_limbs_eq_limb_mod_limb(xs: &[Limb], y: Limb, m: Limb) -> bool {
    if xs.len() < BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_mod_limb(xs, m) == y % m
    } else {
        limbs_eq_limb_mod_limb(xs, y, m)
    }
}
