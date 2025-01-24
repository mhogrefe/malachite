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

use crate::natural::arithmetic::add::limbs_add_limb_to_out;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{NegModPowerOf2, PowerOf2, WrappingSubAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

// This is MPFR_CAN_ROUND from mpfr-impl.h, MPFR 4.2.0.
pub fn float_can_round(x: &Natural, err0: u64, prec: u64, rm: RoundingMode) -> bool {
    match x {
        Natural(Small(small)) => limbs_float_can_round(&[*small], err0, prec, rm),
        Natural(Large(xs)) => limbs_float_can_round(xs, err0, prec, rm),
    }
}

pub(crate) fn limbs_float_can_round(
    xs: &[Limb],
    err0: u64,
    mut prec: u64,
    rm: RoundingMode,
) -> bool {
    if rm == Nearest {
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

const WIDTH_M1: u64 = Limb::WIDTH - 1;
const HIGH_BIT: Limb = 1 << WIDTH_M1;
const WIDTH_M1_MASK: Limb = Limb::MAX >> 1;
pub(crate) const MPFR_EVEN_INEX: i8 = 2;

// This is MPFR_RNDRAW_EVEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool`
// signifying whether the returned exponent should be incremented.
pub(crate) fn round_helper_even(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper(out, out_prec, xs, x_prec, rm, |out, xs_hi, ulp| {
        let ulp_mask = !(ulp - 1);
        if xs_hi[0] & ulp == 0 {
            out.copy_from_slice(xs_hi);
            out[0] &= ulp_mask;
            (-MPFR_EVEN_INEX, false)
        } else {
            let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
            if increment {
                *out.last_mut().unwrap() = HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (MPFR_EVEN_INEX, increment)
        }
    })
}

// This is MPFR_RNDRAW from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool` signifying
// whether the returned exponent should be incremented.
pub fn round_helper_raw(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper(out, out_prec, xs, x_prec, rm, |out, xs_hi, ulp| {
        let ulp_mask = !(ulp - 1);
        if xs_hi[0] & ulp == 0 {
            out.copy_from_slice(xs_hi);
            out[0] &= ulp_mask;
            (-1, false)
        } else {
            let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
            if increment {
                *out.last_mut().unwrap() = HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (1, increment)
        }
    })
}

// This is MPFR_RNDRAW_GEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool` signifying
// whether the returned exponent should be incremented.
fn round_helper<F: Fn(&mut [Limb], &[Limb], Limb) -> (i8, bool)>(
    out: &mut [Limb],
    out_prec: u64,
    xs: &[Limb],
    x_prec: u64,
    rm: RoundingMode,
    middle_handler: F,
) -> (i8, bool) {
    let xs_len = xs.len();
    let out_len = out.len();
    // Check trivial case when out mantissa has more bits than source
    if out_prec >= x_prec {
        out[out_len - xs_len..].copy_from_slice(xs);
        (0, false)
    } else {
        // - Nontrivial case: rounding needed
        // - Compute position and shift
        let shift = out_prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
        let i = xs_len.checked_sub(out_len).unwrap();
        let mut sticky_bit;
        let round_bit;
        // General case when prec % Limb::WIDTH != 0
        let ulp = if shift != 0 {
            // Compute rounding bit and sticky bit
            //
            // Note: in directed rounding modes, if the rounding bit is 1, the behavior does not
            // depend on the sticky bit; thus we will not try to compute it in this case (this can
            // be much faster and avoids reading uninitialized data in the current mpfr_mul
            // implementation). We just make sure that sticky_bit is initialized.
            let mask = Limb::power_of_2(shift - 1);
            let x = xs[i];
            round_bit = x & mask;
            sticky_bit = x & (mask - 1);
            if rm == Nearest || round_bit == 0 {
                let mut to = i;
                let mut n = xs_len - out_len;
                while n != 0 && sticky_bit == 0 {
                    to -= 1;
                    sticky_bit = xs[to];
                    n -= 1;
                }
            }
            mask << 1
        } else {
            assert!(out_len < xs_len);
            // Compute rounding bit and sticky bit - see note above
            let x = xs[i - 1];
            round_bit = x & HIGH_BIT;
            sticky_bit = x & WIDTH_M1_MASK;
            if rm == Nearest || round_bit == 0 {
                let mut to = i - 1;
                let mut n = xs_len - out_len - 1;
                while n != 0 && sticky_bit == 0 {
                    to -= 1;
                    sticky_bit = xs[to];
                    n -= 1;
                }
            }
            1
        };
        let xs_hi = &xs[i..];
        let ulp_mask = !(ulp - 1);
        match rm {
            Floor | Down | Exact => {
                out.copy_from_slice(xs_hi);
                out[0] &= ulp_mask;
                (if sticky_bit | round_bit != 0 { -1 } else { 0 }, false)
            }
            Ceiling | Up => {
                if sticky_bit | round_bit == 0 {
                    out.copy_from_slice(xs_hi);
                    out[0] &= ulp_mask;
                    (0, false)
                } else {
                    let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
                    if increment {
                        out[out_len - 1] = HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
            Nearest => {
                if round_bit == 0 {
                    out.copy_from_slice(xs_hi);
                    out[0] &= ulp_mask;
                    (if (sticky_bit | round_bit) != 0 { -1 } else { 0 }, false)
                } else if sticky_bit == 0 {
                    middle_handler(out, xs_hi, ulp)
                } else {
                    let increment = limbs_add_limb_to_out(out, xs_hi, ulp);
                    if increment {
                        out[out_len - 1] = HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
        }
    }
}

// Assuming xs is an approximation of a non-singular number with error at most equal to 2 ^ (EXP(x)
// - err0) (`err0` bits of x are known) of direction unknown, check if we can round x toward zero
// with precision prec.
//
// This is mpfr_round_p from round_p.c, MPFR 4.2.0.
pub(crate) fn round_helper_2(xs: &[Limb], err0: i32, prec: u64) -> bool {
    let len = xs.len();
    assert!(xs.last().unwrap().get_highest_bit());
    let mut err = u64::exact_from(len << Limb::LOG_WIDTH);
    if err0 <= 0 {
        return false;
    }
    let err0 = u64::from(err0.unsigned_abs());
    if err0 <= prec || prec >= err {
        return false;
    }
    err = min(err, err0);
    let k = usize::exact_from(prec >> Limb::LOG_WIDTH);
    let n = usize::exact_from(err >> Limb::LOG_WIDTH) - k;
    assert!(len > k);
    // Check first limb
    let xs = &xs[len - k - n - 1..];
    let (xs_last, xs_init) = xs[..=n].split_last().unwrap();
    let mut tmp = *xs_last;
    let mask = Limb::MAX >> (prec & Limb::WIDTH_MASK);
    tmp &= mask;
    if n == 0 {
        // prec and error are in the same limb
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        let (xs_head, xs_tail) = xs_init.split_first().unwrap();
        // Check if all (n - 1) limbs are 0
        if !slice_test_zero(xs_tail) {
            return true;
        }
        // Check if final error limb is 0
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && *xs_head >> s != 0
    } else if tmp == mask {
        let (xs_head, xs_tail) = xs_init.split_first().unwrap();
        // Check if all (n - 1) limbs are 11111111111111111
        if xs_tail.iter().any(|&x| x != Limb::MAX) {
            return true;
        }
        // Check if final error limb is 0
        let s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && *xs_head >> s != Limb::MAX >> s
    } else {
        // First limb is different from 000000 or 1111111
        true
    }
}
