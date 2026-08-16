// Copyright © 2026 Mikhail Hogrefe
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
use crate::natural::arithmetic::add::{limbs_add_limb_to_out, limbs_slice_add_limb_in_place};
use crate::natural::arithmetic::sub::limbs_sub_limb_to_out;
use crate::natural::{
    LIMB_HIGH_BIT, LIMB_MAX_HALF, Natural, WIDTH_MINUS_1, bit_to_limb_count_floor,
    limb_to_bit_count,
};
use crate::platform::Limb;
use core::cmp::min;
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, ModPowerOf2, NegModPowerOf2, Parity, PowerOf2, ShrRound, WrappingSubAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::slices::slice_test_zero;

// This is MPFR_CAN_ROUND from mpfr-impl.h, MPFR 4.2.0.
pub fn float_can_round(x: &Natural, err0: u64, prec: u64, rm: RoundingMode) -> bool {
    match x {
        Natural(Small(small)) => limb_float_can_round(*small, err0, prec, rm),
        Natural(Large(xs)) => limbs_float_can_round(xs, err0, prec, rm),
    }
}

pub(crate) fn limb_float_can_round(x: Limb, err0: u64, mut prec: u64, rm: RoundingMode) -> bool {
    if rm == Nearest {
        prec += 1;
    }
    assert!(x.get_highest_bit());
    let err = min(err0, u64::power_of_2(Limb::LOG_WIDTH));
    if err <= prec {
        return false;
    }
    let mut s = Limb::WIDTH - (prec & Limb::WIDTH_MASK);
    let n = bit_to_limb_count_floor(err);
    // Check first limb
    let mask = Limb::low_mask(s);
    let mut tmp = x & mask;
    s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
    if n == 0 {
        // prec and error are in the same limb
        assert!(s < Limb::WIDTH);
        tmp >>= s;
        tmp != 0 && tmp != mask >> s
    } else if tmp == 0 {
        // Check if error limb is 0
        s != Limb::WIDTH && x >> s != 0
    } else if tmp == mask {
        // Check if error limb is 0
        s != Limb::WIDTH && x >> s != Limb::MAX >> s
    } else {
        // limb is different from 000000 or 1111111
        true
    }
}

pub fn limbs_float_can_round(xs: &[Limb], err0: u64, mut prec: u64, rm: RoundingMode) -> bool {
    if rm == Nearest {
        prec += 1;
    }
    let len = xs.len();
    assert!(xs[len - 1].get_highest_bit());
    let err = min(err0, limb_to_bit_count(len));
    if err <= prec {
        return false;
    }
    let k = bit_to_limb_count_floor(prec);
    let mut s = Limb::WIDTH - (prec & Limb::WIDTH_MASK);
    let n = bit_to_limb_count_floor(err) - k;
    assert!(len > k);
    // Check first limb
    let mut i = len - k - 1;
    let mask = Limb::low_mask(s);
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
        let j = i.wrapping_add(2) - n;
        if n > 1 && !slice_test_zero(&xs[j..=i]) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j - 1] >> s != 0
    } else if tmp == mask {
        // Check if all (n - 1) limbs are 11111111111111111
        let j = i.wrapping_add(2) - n;
        if n > 1 && xs[j..=i].iter().any(|&x| x != Limb::MAX) {
            return true;
        }
        // Check if final error limb is 0
        s = Limb::WIDTH - (err & Limb::WIDTH_MASK);
        s != Limb::WIDTH && xs[j - 1] >> s != Limb::MAX >> s
    } else {
        // First limb is different from 000000 or 1111111
        true
    }
}

// Given the significand `xs` of a nonzero finite `Float` (little-endian limbs, with the most
// significant bit of the most significant limb set), returns `Some(j)` if the significand's bits
// form a run of `j` ones followed by all zeros (that is, the mantissa equals $2^j - 1$), and `None`
// otherwise.
//
// This detects inputs `x` for which $1+x$ is an exact power of 2: combined with the exponent, a
// significand of the form $2^j - 1$ means the value is $2^e - 2^{e-j}$, which equals $2^k - 1$ (for
// `x` positive, when $e = j$, giving $k = j$) or $1 - 2^{-j}$ (for `x` in $(-1, 0)$, when $e = 0$,
// giving $k = -j$).
pub fn limbs_float_significand_leading_ones(xs: &[Limb]) -> Option<u64> {
    let mut i = xs.len();
    let mut count = 0;
    // Skip the all-ones limbs at the top.
    while i > 0 && xs[i - 1] == Limb::MAX {
        count += Limb::WIDTH;
        i -= 1;
    }
    if i == 0 {
        return Some(count);
    }
    // The transition limb (not all ones): it must be a run of ones followed by zeros.
    let m = xs[i - 1];
    let j = m.leading_ones();
    if m << j != 0 {
        // A one-bit appears below the leading run of ones.
        return None;
    }
    count += u64::from(j);
    // Every remaining lower limb must be zero.
    if slice_test_zero(&xs[..i - 1]) {
        Some(count)
    } else {
        None
    }
}

// Given the significand `x` of a nonzero finite `Float`, returns `Some(j)` if the mantissa equals
// $2^j - 1$ (a run of ones followed by all zeros), and `None` otherwise. See
// [`limbs_float_significand_leading_ones`].
pub fn float_significand_leading_ones(x: &Natural) -> Option<u64> {
    match x {
        Natural(Small(small)) => limbs_float_significand_leading_ones(core::slice::from_ref(small)),
        Natural(Large(xs)) => limbs_float_significand_leading_ones(xs),
    }
}

pub(crate) const MPFR_EVEN_INEX: i8 = 2;
pub(crate) const MPFR_ROUND_FAILED: i8 = 3;
pub(crate) const NEG_MPFR_ROUND_FAILED: i8 = -MPFR_ROUND_FAILED;

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
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (MPFR_EVEN_INEX, increment)
        }
    })
}

// This is MPFR_RNDRAW and mpfr_round_raw from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a
// `bool` signifying whether the returned exponent should be incremented.
#[inline]
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
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
            }
            out[0] &= ulp_mask;
            (1, increment)
        }
    })
}

// This is MPFR_RNDRAW and mpfr_round_raw from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a
// `bool` signifying whether the returned exponent should be incremented. The output is written to
// &mut xs[out_offset..].
#[inline]
pub fn round_helper_raw_aliased(
    out_offset: usize,
    out_prec: u64,
    xs: &mut [Limb],
    x_prec: u64,
    rm: RoundingMode,
) -> (i8, bool) {
    round_helper_aliased(out_offset, out_prec, xs, x_prec, rm, |out, ulp| {
        let ulp_mask = !(ulp - 1);
        if out[0] & ulp == 0 {
            out[0] &= ulp_mask;
            (-1, false)
        } else {
            let increment = limbs_slice_add_limb_in_place(out, ulp);
            if increment {
                *out.last_mut().unwrap() = LIMB_HIGH_BIT;
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
            round_bit = x & LIMB_HIGH_BIT;
            sticky_bit = x & LIMB_MAX_HALF;
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
                        out[out_len - 1] = LIMB_HIGH_BIT;
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
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
        }
    }
}

// This is MPFR_RNDRAW_GEN from mpfr-impl.h, MPFR 4.2.0, returning `inexact` and a `bool` signifying
// whether the returned exponent should be incremented. The output is written to &mut
// xs[out_offset..].
fn round_helper_aliased<F: Fn(&mut [Limb], Limb) -> (i8, bool)>(
    out_offset: usize,
    out_prec: u64,
    xs: &mut [Limb],
    x_prec: u64,
    rm: RoundingMode,
    middle_handler: F,
) -> (i8, bool) {
    let xs_len = xs.len();
    let out_len = xs_len - out_offset;
    // Check trivial case when out mantissa has more bits than source
    if out_prec >= x_prec {
        (0, false)
    } else {
        // - Nontrivial case: rounding needed
        // - Compute position and shift
        let shift = out_prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
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
            let x = xs[out_offset];
            round_bit = x & mask;
            sticky_bit = x & (mask - 1);
            if rm == Nearest || round_bit == 0 {
                let mut n = out_offset;
                while n != 0 && sticky_bit == 0 {
                    n -= 1;
                    sticky_bit = xs[n];
                }
            }
            mask << 1
        } else {
            assert_ne!(out_offset, 0);
            // Compute rounding bit and sticky bit - see note above
            let x = xs[out_offset - 1];
            round_bit = x & LIMB_HIGH_BIT;
            sticky_bit = x & LIMB_MAX_HALF;
            if rm == Nearest || round_bit == 0 {
                let mut n = out_offset - 1;
                while n != 0 && sticky_bit == 0 {
                    n -= 1;
                    sticky_bit = xs[n];
                }
            }
            1
        };
        let out = &mut xs[out_offset..];
        let ulp_mask = !(ulp - 1);
        match rm {
            Floor | Down | Exact => {
                out[0] &= ulp_mask;
                (if sticky_bit | round_bit != 0 { -1 } else { 0 }, false)
            }
            Ceiling | Up => {
                if sticky_bit | round_bit == 0 {
                    out[0] &= ulp_mask;
                    (0, false)
                } else {
                    let increment = limbs_slice_add_limb_in_place(out, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
                    }
                    out[0] &= ulp_mask;
                    (1, increment)
                }
            }
            Nearest => {
                if round_bit == 0 {
                    out[0] &= ulp_mask;
                    (if (sticky_bit | round_bit) != 0 { -1 } else { 0 }, false)
                } else if sticky_bit == 0 {
                    middle_handler(out, ulp)
                } else {
                    let increment = limbs_slice_add_limb_in_place(out, ulp);
                    if increment {
                        out[out_len - 1] = LIMB_HIGH_BIT;
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
    let mut err = limb_to_bit_count(len);
    if err0 <= 0 {
        return false;
    }
    let err0 = u64::from(err0.unsigned_abs());
    if err0 <= prec || prec >= err {
        return false;
    }
    err = min(err, err0);
    let k = bit_to_limb_count_floor(prec);
    let n = bit_to_limb_count_floor(err) - k;
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

#[inline]
pub fn limbs_significand_slice_add_limb_in_place(xs: &mut [Limb], y: Limb) -> bool {
    limbs_slice_add_limb_in_place(xs, y)
}

// Returns whether the given rounding mode, applied to a value of the given sign, rounds toward
// zero. This is MPFR_IS_LIKE_RNDZ from mpfr-impl.h, MPFR 4.2.2, restricted to the modes that can
// reach it here.
const fn is_like_rounding_toward_zero(rm: RoundingMode, neg: bool) -> bool {
    match rm {
        Down => true,
        Up => false,
        Floor => !neg,
        Ceiling => neg,
        _ => panic!(),
    }
}

// This is mpfr_round_raw2 (mpfr_round_raw_2, that is, round_raw_generic with flag = 1 and use_inexp
// = 0) from round_raw_generic.c, MPFR 4.2.2. All bits of `xs` are considered significant. `rm` must
// already be sign-normalized: `Down` means toward zero, `Up` away from zero, and `Nearest` ties to
// even. Returns whether rounding to `prec` bits with `rm` would increment the significand at the
// ulp position of `prec`.
pub fn limbs_round_would_increment(xs: &[Limb], prec: u64, rm: RoundingMode) -> bool {
    let x_len = xs.len();
    if limb_to_bit_count(x_len) <= prec || rm == Down {
        return false;
    }
    let mut nw = usize::exact_from(prec >> Limb::LOG_WIDTH);
    let rw = prec & Limb::WIDTH_MASK;
    let mut k = x_len - nw - 1;
    let (lomask, himask) = if rw != 0 {
        nw += 1;
        let lomask = Limb::low_mask(Limb::WIDTH - rw);
        (lomask, !lomask)
    } else {
        (Limb::MAX, Limb::MAX)
    };
    let mut sb = xs[k] & lomask;
    match rm {
        Nearest => {
            let rbmask = Limb::power_of_2(WIDTH_MINUS_1 - rw);
            if sb & rbmask == 0 {
                // the rounding bit is 0, so behave like rounding toward zero
                false
            } else {
                sb &= !rbmask;
                while sb == 0 && k > 0 {
                    k -= 1;
                    sb = xs[k];
                }
                if sb == 0 {
                    // an exact tie: round to even, incrementing when the lowest kept bit is 1
                    xs[x_len - nw] & (himask ^ (himask << 1)) != 0
                } else {
                    true
                }
            }
        }
        Up => {
            while sb == 0 && k > 0 {
                k -= 1;
                sb = xs[k];
            }
            sb != 0
        }
        _ => unreachable!(),
    }
}

// This is mpfr_can_round_raw from round_prec.c, MPFR 4.2.2, without the faithful-rounding (RNDF)
// cases, which have no counterpart among Malachite's rounding modes. `xs` is the significand of a
// nonzero finite value of the given sign, an approximation of some real number x in the direction
// `rnd1` with error at most 2^(EXP - err), where EXP is the raw exponent; the result is whether x
// can be correctly rounded to `prec` bits in the direction `rnd2`, meaning that every real
// consistent with the approximation rounds to the same value.
pub fn limbs_float_can_round_raw(
    xs: &[Limb],
    neg: bool,
    err: i64,
    rnd1: RoundingMode,
    rnd2: RoundingMode,
    prec: u64,
) -> bool {
    assert_ne!(prec, 0);
    let mut bn = xs.len();
    assert!(xs[bn - 1].get_highest_bit());
    // Transform Floor and Ceiling to Down (toward zero) and Up (away from zero) using the sign
    let rnd1 = if rnd1 == Nearest {
        Nearest
    } else if is_like_rounding_toward_zero(rnd1, neg) {
        Down
    } else {
        Up
    };
    let rnd2 = if rnd2 == Nearest {
        Nearest
    } else if is_like_rounding_toward_zero(rnd2, neg) {
        Down
    } else {
        Up
    };
    // For err < prec (+ 1 when rnd1 is Nearest) we can never round correctly, since the error is at
    // least 2 ulps of the rounded value; at equality only rare cases work, requiring rnd1 to be
    // Down or Nearest and rnd2 to be Up or Nearest.
    let iprec = i64::exact_from(prec);
    let n1 = i64::from(rnd1 == Nearest);
    if err < iprec + n1 || err == iprec + n1 && (rnd1 == Up || rnd2 == Down) {
        return false;
    }
    let err = u64::exact_from(err);
    let bits = limb_to_bit_count(bn);
    if prec > bits {
        // prec exceeds the precision of xs; we can round iff rnd2 is compatible with rnd1 and the
        // error is at most half an ulp of xs, except at the boundary when a change of binade could
        // occur
        return if (rnd1 == rnd2 || rnd2 == Nearest) && err > prec {
            !(rnd1 != Down && err == prec + 1 && limbs_is_power_of_2_significand(xs))
        } else {
            false
        };
    }
    if err > bits {
        // the error is smaller than one ulp of the full significand
        return if limbs_is_power_of_2_significand(xs) {
            if (rnd2 == Down || rnd2 == Up) && rnd1 != rnd2 {
                false
            } else if rnd1 == Down {
                true
            } else {
                err > prec + 1
            }
        } else if rnd2 == Nearest {
            if err == prec + 1 && xs[0].odd() {
                false
            } else if prec < bits {
                let k1 = usize::exact_from((prec + 1).shr_round(Limb::LOG_WIDTH, Ceiling).0);
                let s1 = (prec + 1).neg_mod_power_of_2(Limb::LOG_WIDTH);
                if (xs[bn - k1] >> s1).odd() && !limbs_round_would_increment(xs, prec + 1, Up) {
                    // xs is exactly in the middle of two numbers representable at prec
                    if rnd1 == Nearest {
                        false
                    } else {
                        let k1 = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
                        let s1 = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
                        (rnd1 == Down) ^ (xs[bn - k1] >> s1).even()
                    }
                } else {
                    true
                }
            } else {
                true
            }
        } else {
            rnd1 == rnd2 || limbs_round_would_increment(xs, prec, Up)
        };
    }
    // Now err <= bits. The error corresponds to bit s in limb k (counting the most significant limb
    // as limb 0); the least significant kept bit is bit s1 in limb k1.
    let mut k = usize::exact_from((err - 1) >> Limb::LOG_WIDTH);
    let s = err.neg_mod_power_of_2(Limb::LOG_WIDTH);
    let k1 = usize::exact_from((prec - 1) >> Limb::LOG_WIDTH);
    let s1 = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    // The k1 most significant limbs are not needed for the rounding comparisons; they are only
    // consulted later to detect a change of binade when adding or subtracting the error.
    k -= k1;
    bn -= k1;
    let prec2 = prec - limb_to_bit_count(k1);
    k += 1;
    let mut tmp = vec![0; bn];
    if bn > k {
        tmp[..bn - k].copy_from_slice(&xs[..bn - k]);
    }
    // We can round iff rounding the two ends of the interval containing x gives the same result at
    // the target precision: depending on rnd1, the ends are b and b + eps (Down), b - eps and b +
    // eps (Nearest), or b - eps and b (Up).
    let cc;
    let eps = Limb::power_of_2(s);
    if rnd1 == Down {
        cc = (xs[bn - 1] >> s1).odd() ^ limbs_round_would_increment(&xs[..bn], prec2, rnd2);
        // now round b + eps
        let mut cy = limbs_add_limb_to_out(&mut tmp[bn - k..bn], &xs[bn - k..bn], eps);
        // propagate the carry through the truncated limbs
        let mut tn = 0;
        while tn + 1 < k1 && cy {
            cy = xs[bn + tn] == Limb::MAX;
            tn += 1;
        }
        if !cy && err == prec {
            return false;
        }
        if cy {
            // b + eps crosses a power of 2, so b rounds below it and b + eps to it or above
            return match rnd2 {
                Down => false,
                Up => err > prec && k == bn && tmp[0] == 0,
                _ => !cc,
            };
        }
    } else if rnd1 == Nearest {
        // first round b + eps
        let mut cy = limbs_add_limb_to_out(&mut tmp[bn - k..bn], &xs[bn - k..bn], eps);
        let mut tn = 0;
        while tn + 1 < k1 && cy {
            cy = xs[bn + tn] == Limb::MAX;
            tn += 1;
        }
        cc = (tmp[bn - 1] >> s1).odd() ^ limbs_round_would_increment(&tmp[..bn], prec2, rnd2);
        if cy {
            return match rnd2 {
                Down => false,
                Up => err > prec + 1 && k == bn && tmp[0] == 0,
                _ => err > prec + 1,
            };
        }
    } else {
        cc = (xs[bn - 1] >> s1).odd() ^ limbs_round_would_increment(&xs[..bn], prec2, rnd2);
    }
    if rnd1 != Down {
        // round b - eps, for rnd1 Nearest or Up
        let mut cy = limbs_sub_limb_to_out(&mut tmp[bn - k..bn], &xs[bn - k..bn], eps);
        // propagate the potential borrow through the truncated limbs; it cannot propagate beyond
        // them, since the most significant limb has its top bit set
        let mut tmp_hi = tmp[bn - 1];
        let mut tn = 0;
        while tn < k1 && cy {
            let (diff, borrow) = xs[bn + tn].overflowing_sub(Limb::from(cy));
            tmp_hi = diff;
            cy = borrow;
            tn += 1;
        }
        if tn == k1 && !tmp_hi.get_highest_bit() {
            // a change of binade: b - eps falls below a power of 2 that b (or b + eps) reaches
            if rnd2 == Down || rnd1 == Nearest && rnd2 == Up || cc {
                return false;
            }
            return limbs_round_would_increment(&tmp[..bn], prec2 + 1, rnd2);
        }
        if err == prec + u64::from(rnd1 == Nearest) {
            // the interval has width one ulp of b, with no binade change: only the Nearest target
            // mode can round, when b itself is representable and even
            return rnd2 == Nearest
                && (xs[bn - 1] >> s1).even()
                && limbs_round_would_increment(&xs[..bn], prec2, Down)
                    == limbs_round_would_increment(&xs[..bn], prec2, Up);
        }
    }
    let cc2 = (tmp[bn - 1] >> s1).odd();
    cc == (cc2 ^ limbs_round_would_increment(&tmp[..bn], prec2, rnd2))
}

// Returns whether the significand consists of a single one bit.
fn limbs_is_power_of_2_significand(xs: &[Limb]) -> bool {
    let (xs_last, xs_init) = xs.split_last().unwrap();
    xs_last.is_power_of_2() && slice_test_zero(xs_init)
}

// This is mpfr_can_round_raw from round_prec.c, MPFR 4.2.2, taking the significand as a
// [`Natural`].
pub fn float_can_round_raw(
    x: &Natural,
    neg: bool,
    err: i64,
    rnd1: RoundingMode,
    rnd2: RoundingMode,
    prec: u64,
) -> bool {
    match x {
        Natural(Small(small)) => {
            limbs_float_can_round_raw(core::slice::from_ref(small), neg, err, rnd1, rnd2, prec)
        }
        Natural(Large(xs)) => limbs_float_can_round_raw(xs, neg, err, rnd1, rnd2, prec),
    }
}

// The integer-part rounding core of mpfr_rint from rint.c, MPFR 4.2.2, for the case exp > 0 (that
// is, |u| >= 1). `up` is the significand of the input, `exp` its raw exponent, `prec` the target
// precision; `rnd_away` is the magnitude direction (`None` for the nearest modes, decided here),
// with `ties_away` selecting MPFR_RNDNA tie behavior. `neg` only affects which nearest tie rule is
// even. Returns the rounded significand (aligned for `prec`), whether the exponent must be
// incremented (a carry into the next binade), the MPFR uflags value (0 for an integer representable
// at `prec`, 1 for an integer not representable, 2 for a non-integer), and the decided `rnd_away`.
pub fn limbs_float_round_to_integer(
    up: &[Limb],
    exp: u64,
    prec: u64,
    rnd_away: Option<bool>,
    ties_away: bool,
) -> (Vec<Limb>, bool, u8, bool) {
    let un = up.len();
    let rn =
        usize::exact_from((prec + prec.neg_mod_power_of_2(Limb::LOG_WIDTH)) >> Limb::LOG_WIDTH);
    let mut sh = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
    // uflags: 0 if u is an integer representable at prec, 1 if an integer not representable, 2 if
    // not an integer
    let mut uflags: u8;
    let ui;
    let mut idiff = 0;
    if (exp - 1) >> Limb::LOG_WIDTH >= u64::exact_from(un) {
        ui = un;
        uflags = 0; // u is an integer, representable or not at prec
    } else {
        ui = usize::exact_from((exp - 1) >> Limb::LOG_WIDTH) + 1;
        let uj = un - ui; // lowest limb of the integer part
        idiff = exp & Limb::WIDTH_MASK; // integer-part bits in up[uj], or 0
        uflags = if idiff == 0 || up[uj] << idiff == 0 {
            0
        } else {
            2
        };
        if uflags == 0 && !slice_test_zero(&up[..uj]) {
            uflags = 2;
        }
    }
    let mut rp = vec![0; rn];
    let mut rnd_away = rnd_away;
    // The slice of rp holding the integer part; below it, limbs stay zero.
    let rp_offset;
    if ui > rn {
        // More limbs in the integer part of u than in the result: round u at prec.
        rp.copy_from_slice(&up[un - rn..]);
        rp_offset = 0;
        if rnd_away.is_none() {
            rnd_away = Some(if !ties_away && !rp[0].get_bit(sh) {
                // a halfway case rounds toward zero: the kept low bit is even
                let (a, b) = if sh != 0 {
                    (rp[0].mod_power_of_2(sh), Limb::power_of_2(sh - 1))
                } else {
                    (up[un - rn - 1], LIMB_HIGH_BIT)
                };
                a > b || a == b && !slice_test_zero(&up[..un - rn - usize::from(sh == 0)])
            } else if sh != 0 {
                // a halfway case rounds away from zero: the rounding bit decides
                rp[0].get_bit(sh - 1)
            } else {
                up[un - rn - 1].get_highest_bit()
            });
        }
        if uflags == 0
            && (sh != 0 && rp[0] << (Limb::WIDTH - sh) != 0 || !slice_test_zero(&up[..un - rn]))
        {
            // u is an integer, but not representable at prec
            uflags = 1;
        }
    } else {
        // The integer part of u fits in the result.
        let uj = un - ui;
        let rj = rn - ui;
        rp[rj..].copy_from_slice(&up[uj..]);
        rp_offset = rj;
        // the number of fractional bits in the boundary limb of the result
        let ush = if idiff == 0 { 0 } else { Limb::WIDTH - idiff };
        if rj == 0 && ush < sh {
            // If u is an integer, it is representable at prec iff its bits between ush and sh are
            // all 0.
            if uflags == 0 && rp[rj] & (Limb::low_mask(sh) - Limb::low_mask(ush)) != 0 {
                uflags = 1;
            }
        } else {
            // The integer part of u fits at prec; round to it.
            sh = ush;
        }
        if rnd_away.is_none() {
            rnd_away = Some(if uj == 0 && sh == 0 {
                // the rounding bit is 0 (not represented in u)
                false
            } else if !ties_away && !rp[rp_offset].get_bit(sh) {
                // a halfway case rounds toward zero: the kept low bit is even
                let (a, b) = if sh != 0 {
                    (rp[rp_offset].mod_power_of_2(sh), Limb::power_of_2(sh - 1))
                } else {
                    (up[uj - 1], LIMB_HIGH_BIT)
                };
                a > b || a == b && !slice_test_zero(&up[..uj - usize::from(sh == 0)])
            } else if sh != 0 {
                // a halfway case rounds away from zero: the rounding bit decides
                rp[rp_offset].get_bit(sh - 1)
            } else {
                up[uj - 1].get_highest_bit()
            });
        }
    }
    if sh != 0 {
        rp[rp_offset] &= Limb::MAX << sh;
    }
    // If u is an integer representable at prec, there is no rounding.
    if uflags == 0 {
        return (rp, false, 0, false);
    }
    let rnd_away = rnd_away.unwrap();
    let mut exp_increment = false;
    if rnd_away && limbs_slice_add_limb_in_place(&mut rp[rp_offset..], Limb::power_of_2(sh)) {
        exp_increment = true;
        *rp.last_mut().unwrap() = LIMB_HIGH_BIT;
    }
    (rp, exp_increment, uflags, rnd_away)
}

// The significand of `x` as a little-endian limb slice, via a callback (a `Natural` stores a single
// small limb out of line from the multi-limb representation).
pub fn with_float_significand_limbs<T, F: FnOnce(&[Limb]) -> T>(x: &Natural, f: F) -> T {
    match x {
        Natural(Small(small)) => f(core::slice::from_ref(small)),
        Natural(Large(xs)) => f(xs),
    }
}
