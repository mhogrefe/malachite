// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2021-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Compound, CompoundAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// The overflow result of compound (the value, which is positive, exceeds the maximum finite
// `Float`).
//
// This is `mpfr_overflow` (with positive sign) as used by `mpfr_compound_si`, MPFR 4.2.2.
fn compound_overflow(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match rm {
        Nearest | Up | Ceiling => (Float::INFINITY, Greater),
        Down | Floor => (Float::max_finite_value_with_prec(prec), Less),
        Exact => panic!("compound: Exact rounding was requested, but the result overflows"),
    }
}

// The underflow result of compound (the value, which is positive, is below the minimum positive
// `Float`). MPFR maps Nearest to toward-zero here, so Nearest joins Down/Floor.
//
// This is `mpfr_underflow` (with positive sign) as used by `mpfr_compound_si`, MPFR 4.2.2.
fn compound_underflow(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match rm {
        Nearest | Down | Floor => (Float::ZERO, Less),
        Up | Ceiling => (Float::min_positive_value_prec(prec), Greater),
        Exact => panic!("compound: Exact rounding was requested, but the result underflows"),
    }
}

// Rounds (1+x)^n to `prec` bits, assuming |(1+x)^n - 1| < (1/4)ulp(1) = 2^(-prec-2), where `s_pos`
// is the sign of n*log2(1+x) (true if positive; that quantity is nonzero here).
//
// This is mpfr_compound_near_one from compound.c, MPFR 4.2.2.
fn compound_near_one(prec: u64, s_pos: bool, rm: RoundingMode) -> (Float, Ordering) {
    let mut y = Float::one_prec(prec);
    match rm {
        Exact => panic!("compound: Exact rounding was requested, but the result is inexact"),
        // round toward 1
        Nearest => (y, if s_pos { Less } else { Greater }),
        Down | Floor if s_pos => (y, Less),
        Up | Ceiling if !s_pos => (y, Greater),
        // round toward +Inf
        Up | Ceiling => {
            y.increment();
            (y, Greater)
        }
        // necessarily Down or Floor with a negative sign; round toward 0
        _ => {
            y.decrement();
            (y, Less)
        }
    }
}

// This is mpfr_compound_si from compound.c, MPFR 4.2.2. MPFR runs the computation in its extended
// exponent range and maps back at the end via mpfr_check_range; we instead cut overflow and
// underflow against the real exponent range up front. This is safe because u is rounded toward
// zero (making the cuts sound), and because 2^u is rounded toward 1, which keeps the intermediate
// t representable whenever u survives the cuts.
fn compound_prec_round_helper(x: &Float, n: i64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    // Special cases
    match x {
        // compound(-Inf, n) is NaN
        Float(Infinity { sign: false }) => return (Float::NAN, Equal),
        Float(NaN | Infinity { .. } | Zero { .. }) => {
            return if n == 0 || matches!(x, Float(Zero { .. })) {
                // compound(x, 0) = 1 for x >= -1 or NaN (the only special value of x that is not
                // concerned is -Inf, already handled); compound(0, n) = 1
                (Float::one_prec(prec), Equal)
            } else if matches!(x, Float(NaN)) {
                // compound(NaN, n) is NaN, except for n = 0, already handled
                (Float::NAN, Equal)
            } else if n < 0 {
                // (1+Inf)^n = +0 for n < 0
                (Float::ZERO, Equal)
            } else {
                // n > 0: (1+Inf)^n = +Inf
                (Float::INFINITY, Equal)
            };
        }
        _ => {}
    }
    // (1+x)^n = NaN for x < -1
    let compared = x.partial_cmp(&-1i32).unwrap();
    if compared == Less {
        return (Float::NAN, Equal);
    }
    // compound(x, 0) gives 1 for x >= -1
    if n == 0 {
        return (Float::one_prec(prec), Equal);
    }
    if compared == Equal {
        return if n < 0 {
            // compound(-1, n) = +Inf (MPFR also raises the divide-by-zero exception)
            (Float::INFINITY, Equal)
        } else {
            // compound(-1, n) = +0
            (Float::ZERO, Equal)
        };
    }
    if n == 1 {
        return x.add_prec_round_ref_val(Float::ONE, prec, rm);
    }
    let py = prec;
    let mut wprec = py + py.ceiling_log_base_2() + 6;
    // |n| <= 2^k
    let k = n.unsigned_abs().ceiling_log_base_2();
    // We compute u = log2p1(x) with wprec + extra bits, since we lose some bits in 2^u.
    let mut extra = 0u64;
    let rnd1 = if (n > 0) == x.is_sign_positive() {
        Floor
    } else {
        Ceiling
    };
    let mut increment = Limb::WIDTH;
    let mut nloop = 0u32;
    let t = loop {
        let precu = wprec + extra;
        // We compute (1+x)^n as 2^(n*log2p1(x)), and we round toward 1, thus we round
        // n*log2p1(x) toward 0, thus for x*n > 0 we round log2p1(x) toward -Inf, and for x*n < 0
        // we round log2p1(x) toward +Inf.
        let (lg, o_lg) = x.log_base_2_1_plus_x_prec_round_ref(precu, rnd1);
        let mut inex = o_lg != Equal;
        let Some(e0) = lg.get_exponent() else {
            // log2p1 underflowed to zero (MPFR's extended exponent range prevents this). The true
            // |n*log2(1+x)| is below 2^(MIN_EXPONENT+65), so unless prec is within ~70 bits of the
            // full exponent range, the result is within (1/4)ulp(1) of 1.
            if py >= const { (Float::MAX_EXPONENT - 70) as u64 } {
                fail_on_untested_path("compound, log2p1 underflow with huge prec");
            }
            return compound_near_one(py, (n > 0) == x.is_sign_positive(), rm);
        };
        let mut e = i64::from(e0);
        // |lg - log2(1+x)| <= ulp(lg) = 2^(e-precu)
        let (u, o_mul) = lg.mul_prec_round(Float::from(n), precu, Down);
        inex |= o_mul != Equal;
        // u is nonzero: |lg| >= 2^(MIN_EXPONENT-1) and |n| >= 1, and the toward-zero rounding of
        // the product cannot reach below the minimum positive Float.
        let e2 = i64::from(u.get_exponent().unwrap());
        // |u - n*log2(1+x)| <= 2^(e2-precu) + |n|*2^(e-precu)
        //                   <= 2^(e2-precu) + 2^(e+k-precu) <= 2^(e+k+1-precu)
        // where |n| <= 2^k, and e2 is the new exponent of u.
        debug_assert!(e2 <= e + i64::exact_from(k));
        e += i64::exact_from(k) + 1;
        let new_extra = if e2 > 0 { u64::exact_from(e2) } else { 0 };
        // |u - n*log2(1+x)| <= 2^(e-precu)
        // detect overflow: since we rounded n*log2p1(x) toward 0, if n*log2p1(x) >= MAX_EXPONENT,
        // we are sure there is overflow.
        if u >= Float::MAX_EXPONENT {
            return compound_overflow(py, rm);
        }
        // detect underflow: similarly, since we rounded n*log2p1(x) toward 0, if
        // n*log2p1(x) < MIN_EXPONENT - 1, we are sure there is underflow.
        if u < const { Float::MIN_EXPONENT - 1 } {
            return compound_underflow(py, if rm == Nearest { Down } else { rm });
        }
        // Detect cases where the result is 1 or 1+ulp(1) or 1-(1/2)ulp(1):
        // |2^u - 1| = |exp(u*log(2)) - 1| <= |u|*log(2) < |u|
        if nloop == 0 && e2 < -i64::exact_from(py) {
            // since ulp(1) = 2^(1-py), we have |u| < (1/4)ulp(1)
            return compound_near_one(py, u.is_sign_positive(), rm);
        }
        // round 2^u toward 1
        let rnd2 = if u.is_sign_positive() { Floor } else { Ceiling };
        let (mut t, o_exp2) = Float::power_of_2_of_float_prec_round(u, wprec, rnd2);
        inex |= o_exp2 != Equal;
        // we had |u - n*log2(1+x)| < 2^(e-precu), thus u = n*log2(1+x) + delta with
        // |delta| < 2^(e-precu), then 2^u = (1+x)^n * 2^delta. For |delta| < 0.5,
        // |2^delta - 1| <= |delta| thus
        // |t - (1+x)^n| <= ulp(t) + |t|*2^(e-precu) < 2^(EXP(t)-wprec) + 2^(EXP(t)+e-precu)
        let extra_i = i64::exact_from(precu - wprec);
        let err = if extra_i >= e { 1 } else { e + 1 - extra_i };
        // now |t - (1+x)^n| < 2^(EXP(t)+err-wprec)
        if !inex
            || (rm != Exact
                && i64::exact_from(wprec) > err
                && float_can_round(
                    t.significand_ref().unwrap(),
                    wprec - u64::exact_from(err),
                    py,
                    rm,
                ))
        {
            break t;
        }
        // If t fits in the target precision (or with 1 more bit), then we can round, assuming the
        // working precision is large enough, but the above float_can_round will fail because we
        // cannot determine the ternary value. However, since we rounded t toward 1, we can
        // determine it. Since the error in the approximation t is at most 2^err ulp(t), this
        // error should be less than (1/2)ulp(y), thus we should have wprec - py >= err + 1. (For
        // Exact rounding we skip this escape, since nudging t would turn an exactly-representable
        // result into a spurious panic; the exact-1+x escape below decides exactness instead.)
        if rm != Exact && t.get_min_prec().unwrap() <= py + 1 && i64::exact_from(wprec - py) > err {
            // we step t one place away from 1 to get the correct rounding
            if rnd2 == Floor {
                // t was rounded downwards. t cannot be the largest finite significand (its
                // min_prec is at most py + 1 < wprec), so this cannot overflow.
                t.increment();
                break t;
            }
            if t.get_min_prec() != Some(1) || t.get_exponent() != Some(Float::MIN_EXPONENT) {
                t.decrement();
                break t;
            }
            // Otherwise, stepping below t would leave the representable exponent range (MPFR's
            // extended exponent range has no such problem). Let the Ziv loop refine instead; if
            // the true result is below the minimum positive Float, a later iteration's underflow
            // cut will catch it.
            fail_on_untested_path("compound, min_prec escape at minimum exponent");
        }
        // Detect particular cases where Ziv's strategy may take too much memory and be too long,
        // i.e. when x^n fits in the target precision (+ 1 additional bit for rounding to nearest)
        // and the exact result (1+x)^n is very close to x^n. Necessarily, x is a large even
        // integer and n > 0 (thus n > 1). Since this does not depend on the working precision, we
        // only check this at the first iteration (nloop == 0). Hence the first "if" below and the
        // kx < ex test of the second "if" (x is an even integer iff its least bit 1 has exponent
        // >= 1). The second test of the second "if" corresponds to another simple condition that
        // implies that x^n fits in the target precision. Here are the details:
        // Let k be the minimum length of the significand of x, and x' the odd (integer)
        // significand of x. This means that 2^(k-1) <= x' < 2^k. Thus 2^(n*(k-1)) <= (x')^n <
        // 2^(k*n), and x^n has between n*(k-1)+1 and k*n bits. So x^n can fit into p bits only if
        // p >= n*(k-1)+1, i.e. n*(k-1) <= p-1.
        debug_assert!(!(0..=1).contains(&n));
        if nloop == 0 && n > 1 {
            let ex = i64::from(x.get_exponent().unwrap());
            if ex >= 17 {
                let kx = x.get_min_prec().unwrap();
                let p = py + u64::from(rm == Nearest);
                if kx < u64::exact_from(ex)
                    && u128::from(n.unsigned_abs()) * u128::from(kx - 1) <= u128::from(p - 1)
                {
                    // Check whether x^n really fits into p bits.
                    let (v, o_v) = x.pow_u_prec_round_ref(u64::exact_from(n), p, Down);
                    if o_v == Equal {
                        // (x+1)^n = x^n * (1 + 1/x)^n
                        // For directed rounding, we can round when (1 + 1/x)^n < 1 + 2^-p, and
                        // then the result is x^n, except for rounding up. Indeed, if
                        // (1 + 1/x)^n < 1 + 2^-p,
                        // 1 <= (x+1)^n < x^n * (1 + 2^-p) = x^n + x^n/2^p < x^n + ulp(x^n).
                        // For rounding to nearest, we can round when (1 + 1/x)^n < 1 + 2^-p, and
                        // then the result is x^n when x^n fits into p-1 bits, and
                        // nextabove(x^n) otherwise.
                        let mut r = x.reciprocal_prec_round_ref(wprec, Up).0;
                        r.add_prec_round_assign(Float::ONE, wprec, Up);
                        r.pow_u_round_assign(u64::exact_from(n), Up);
                        r.sub_prec_round_assign(Float::ONE, wprec, Up);
                        // r cannot be zero
                        if i64::from(r.get_exponent().unwrap()) < -i64::exact_from(py) {
                            let v_min_prec = v.get_min_prec().unwrap();
                            let mut y = Float::from_float_prec_round(v, py, Down).0;
                            return if (rm == Nearest && v_min_prec == p)
                                || rm == Up
                                || rm == Ceiling
                            {
                                // round up
                                y.increment();
                                (y, Greater)
                            } else {
                                (y, Less)
                            };
                        }
                    }
                }
            }
        }
        // Exact cases like compound(0.5, 2) = 9/4 must be detected, since except for 1+x a power
        // of 2, the log2p1 above will be inexact, so that in the Ziv test, inex != 0 and
        // float_can_round will fail (even for Nearest, as the ternary value cannot be
        // determined), yielding an infinite loop. For an exact case in precision py, 1+x will
        // necessarily be exact in precision py, thus also in wprec, where wprec >= py, and we can
        // use pow_integer under this condition (which will also evaluate some non-exact cases).
        let (s, o_s) = x.add_prec_round_ref_val(Float::ONE, wprec, Down);
        if o_s == Equal {
            return s.pow_integer_prec_round(Integer::from(n), py, rm);
        }
        wprec += increment;
        increment = wprec >> 1;
        extra = new_extra;
        nloop += 1;
    };
    Float::from_float_prec_round(t, py, rm)
}

impl Float {
    #[inline]
    pub fn compound_prec_round(self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        compound_prec_round_helper(&self, n, prec, rm)
    }

    #[inline]
    pub fn compound_prec_round_ref(&self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        compound_prec_round_helper(self, n, prec, rm)
    }

    #[inline]
    pub fn compound_prec(self, n: i64, prec: u64) -> (Self, Ordering) {
        self.compound_prec_round(n, prec, Nearest)
    }

    #[inline]
    pub fn compound_prec_ref(&self, n: i64, prec: u64) -> (Self, Ordering) {
        self.compound_prec_round_ref(n, prec, Nearest)
    }

    #[inline]
    pub fn compound_round(self, n: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.compound_prec_round(n, prec, rm)
    }

    #[inline]
    pub fn compound_round_ref(&self, n: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.compound_prec_round_ref(n, prec, rm)
    }

    pub fn compound_prec_round_assign(&mut self, n: i64, prec: u64, rm: RoundingMode) -> Ordering {
        let (y, o) = self.compound_prec_round_ref(n, prec, rm);
        *self = y;
        o
    }

    #[inline]
    pub fn compound_prec_assign(&mut self, n: i64, prec: u64) -> Ordering {
        self.compound_prec_round_assign(n, prec, Nearest)
    }

    #[inline]
    pub fn compound_round_assign(&mut self, n: i64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.compound_prec_round_assign(n, prec, rm)
    }
}

impl Compound<i64> for Float {
    type Output = Self;

    #[inline]
    fn compound(self, n: i64) -> Self {
        let prec = self.significant_bits();
        self.compound_prec_round(n, prec, Nearest).0
    }
}

impl Compound<i64> for &Float {
    type Output = Float;

    #[inline]
    fn compound(self, n: i64) -> Float {
        let prec = self.significant_bits();
        self.compound_prec_round_ref(n, prec, Nearest).0
    }
}

impl CompoundAssign<i64> for Float {
    #[inline]
    fn compound_assign(&mut self, n: i64) {
        let prec = self.significant_bits();
        self.compound_prec_round_assign(n, prec, Nearest);
    }
}
