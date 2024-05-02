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

use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::WrappingSubAssign;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode;

// This is MPFR_CAN_ROUND from mpfr-impl.h, MPFR 4.2.0.
pub fn float_can_round(x: &Natural, err0: u64, mut prec: u64, rm: RoundingMode) -> bool {
    let single_limb;
    let xs: &[Limb] = match x {
        Natural(Small(small)) => {
            single_limb = [*small];
            &single_limb
        }
        Natural(Large(xs)) => xs,
    };
    if rm == RoundingMode::Nearest {
        prec += 1;
    }
    let len = xs.len();
    assert!(xs[len - 1].get_highest_bit());
    let err = min(err0, u64::exact_from(len << Limb::LOG_WIDTH));
    if err <= prec {
        return false;
    }
    let k = usize::exact_from(prec >> Limb::LOG_WIDTH);
    let mut s = Limb::WIDTH - (prec & Limb::WIDTH_MASK);
    let n = usize::exact_from(err >> Limb::LOG_WIDTH) - k;
    assert!(len > k);
    // Check first limb
    let mut i = len - k - 1;
    let mask = if s == Limb::WIDTH {
        Limb::MAX
    } else {
        Limb::low_mask(s)
    };
    let mut tmp = xs[i] & mask;
    i.wrapping_sub_assign(1);
    if n == 0 {
        // prec and error are in the same limb
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        // Check if all (n - 1) limbs are 0
        let j = i + 1 - n;
        if xs[j + 1..=i].iter().any(|&x| x != 0) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j] >> s != 0
    } else if tmp == mask {
        // Check if all (n - 1) limbs are 11111111111111111
        let j = i + 1 - n;
        if xs[j + 1..=i].iter().any(|&x| x != Limb::MAX) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j] >> s != Limb::MAX >> s
    } else {
        // First limb is different from 000000 or 1111111
        true
    }
}
