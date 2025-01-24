// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2000-2002 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::divisible_by::limbs_divisible_by_limb;
use crate::natural::arithmetic::mod_op::limbs_mod_limb;
use crate::platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use num::{BigUint, Integer, Zero};

pub fn num_divisible_by(x: &BigUint, y: &BigUint) -> bool {
    *x == BigUint::zero() || *y != BigUint::zero() && x.is_multiple_of(y)
}

/// Benchmarks show that this is never faster than just calling `limbs_divisible_by_limb`.
///
/// ns.len() must be greater than 1; divisor must be nonzero.
///
/// This is equivalent to `mpz_divisible_ui_p` from `mpz/divis_ui.c`, GMP 6.2.1, where `a` is
/// non-negative.
#[allow(clippy::absurd_extreme_comparisons)]
pub fn combined_limbs_divisible_by_limb(ns: &[Limb], d: Limb) -> bool {
    if ns.len() <= BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_divisible_by_limb(ns, d)
    } else {
        limbs_mod_limb(ns, d) == 0
    }
}
