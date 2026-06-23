// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 1999-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's exponential. `mpfr_exp` (`exp.c`) is a dispatcher; the medium-precision workhorse
// `mpfr_exp_2` (`exp_2.c`) uses Brent's method -- reduce x = n*log(2) + 2^K*r, sum the Taylor
// series for the small r, raise to the 2^K power by K squarings, then scale by 2^n -- with the
// series summed in fixed point. That fixed point is represented here as a malachite `Integer`
// mantissa paired with an `i64` 2-exponent (MPFR's `mpz_t` + `mpfr_exp_t`).
//
// The Paterson-Stockmeyer series (exp2_aux2) and the high-precision exp_3 are not yet ported.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::mem::swap;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, Exp, ExpAssign, FloorSqrt, IsPowerOf2, NegAssign, PowerOf2, SquareAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::RoundingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};

// If the number of bits `k` of `z` exceeds `q`, divides `z` by `2 ^ (k - q)` (flooring) and returns
// `k - q`; otherwise leaves `z` unchanged and returns 0.
//
// This is `mpz_normalize` from `exp_2.c`, MPFR 4.2.2.
fn mpz_normalize(z: Integer, q: i64) -> (Integer, i64) {
    let k = z.significant_bits();
    if q < 0 || k > u64::try_from(q).unwrap() {
        let shift = i64::try_from(k).unwrap() - q;
        (z >> shift, shift)
    } else {
        // Currently unreachable from the naive series (`exp2_aux` always grows `t`/`rr` past `q`
        // bits before truncating, and the squaring loop doubles past `q`); exercised once the
        // Paterson-Stockmeyer path (`exp2_aux2`) is ported.
        (z, 0)
    }
}

// Shifts `z` so that its 2-exponent becomes `target`: right (flooring) by `target - expz` if
// `target > expz`, otherwise left by `expz - target`. Returns `target`.
//
// This is `mpz_normalize2` from `exp_2.c`, MPFR 4.2.2. currently only used by exp2_aux2, which is
// ported later. Simple enough that it can be inlined
#[allow(dead_code)]
fn mpz_normalize2(z: Integer, expz: i64, target: i64) -> (Integer, i64) {
    (z >> (target - expz), target)
}

// Returns the integer mantissa `m` and 2-exponent `e` of a finite nonzero `x`, so that `x = m *
// 2^e` (the sign is carried by `m`). For a Malachite `Float`, `m` is the significand as a signed
// integer and `e = exponent - significand_bits` (verified against 1.0: significand 2^63, exponent
// 1, giving 2^63 * 2^(1-64) = 1).
//
// This is equivalent to `mpfr_get_z_2exp` from MPFR 4.2.2.
fn get_z_2exp(x: Float) -> (Integer, i64) {
    if let Finite {
        sign,
        exponent,
        significand,
        ..
    } = x.0
    {
        let bits = significand.significant_bits();
        let m = Integer::from_sign_and_abs(sign, significand);
        (m, i64::from(exponent) - i64::try_from(bits).unwrap())
    } else {
        unreachable!()
    }
}

// Computes `s = 1 + r/1! + r^2/2! + ... + r^l/l!` (continuing while the term is still significant
// at precision `q`) in fixed point, where the returned `Integer` `s` and 2-exponent `exps` satisfy
// (sum) = s * 2^exps. `r` must be pure FP (here it is positive and tiny). The naive method, O(l)
// multiplications; the absolute error on the sum is less than `3*l*(l+1)*2^(-q)`, and that
// `3*l*(l+1)` bound is the returned value. (`l` stays small for the precisions `exp_2` handles, so
// the bound fits in a `u64`.)
//
// This is `mpfr_exp2_aux` from `exp_2.c`, MPFR 4.2.2.
fn exp2_aux(r: Float, q: u64) -> (Integer, i64, u64) {
    let qi = i64::try_from(q).unwrap();
    let mut expt: i64 = 0;
    let exps: i64 = 1 - qi; // s = 2^(q-1), i.e. the value 1
    let mut t = Integer::ONE;
    let mut s = Integer::power_of_2(q - 1);
    let (mut rr, mut expr) = get_z_2exp(r); // rr * 2^expr = r, no error
    let mut l: u64 = 0;
    loop {
        l += 1;
        t *= &rr;
        expt += expr;
        let sbit = i64::try_from(s.significant_bits()).unwrap();
        let tbit = i64::try_from(t.significant_bits()).unwrap();
        let dif = exps + sbit - expt - tbit;
        // truncate the bits of t that are below ulp(s) = 2^(1-q); error at most 2^(1-q)
        let (t2, sh) = mpz_normalize(t, qi - dif);
        t = t2;
        expt += sh;
        if l > 1 {
            // divide by l to build r^l/l! (t >= 0, so truncation equals MPFR's floored division)
            if l.is_power_of_2() {
                // GMP doesn't optimize the power-of-2 case
                t >>= l.ceiling_log_base_2();
            } else {
                t /= Integer::from(l);
            }
            debug_assert_eq!(expt, exps);
        }
        if t == 0 {
            break;
        }
        s += &t; // exact
        // keep rr the same size as t: the error on rr stays at most ulp(t) = ulp(s)
        let tbit = i64::try_from(t.significant_bits()).unwrap();
        let (rr2, sh) = mpz_normalize(rr, tbit);
        rr = rr2;
        expr += sh;
    }
    (s, exps, 3 * l * (l + 1))
}

// Computes `exp(x)` rounded to precision `precy` with rounding mode `rm`, returning the rounded
// value and an [`Ordering`] comparing it to the exact result. `x` must be finite and nonzero and
// `exp(x)` must be in range; the dispatcher (`exp`) guarantees both. Uses Brent's method: `exp(x) =
// (1 + r + r^2/2! + ...)^(2^K) * 2^n` with `x = n*log(2) + 2^K*r`.
//
// For now the naive series (`exp2_aux`) is always used, so `K` follows the square-root formula; the
// Paterson-Stockmeyer path (`exp2_aux2`, with the cube-root `K` above `MPFR_EXP_2_THRESHOLD`) is
// added later.
//
// This is `mpfr_exp_2` from `exp_2.c`, MPFR 4.2.2.
pub(crate) fn exp_2(x: &Float, precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    let expx = i64::from(x.get_exponent().unwrap());
    // Argument reduction: n ~ round(x / log(2)) (need not be exact).
    let mut n: i64 = if expx <= -2 {
        // |x| <= 0.25, so n = 0
        0
    } else {
        let log2_est = Float::ln_2_prec_round(Limb::WIDTH - 1, Down).0;
        let r_est = x
            .div_prec_round_ref_val(log2_est, Limb::WIDTH - 1, Nearest)
            .0;
        i64::rounding_from(r_est, Nearest).0
    };
    // error_r bounds the bits cancelled in x - n*log(2)
    let error_r: u64 = if n == 0 {
        0
    } else {
        (n.unsigned_abs() + 1).significant_bits()
    };
    // Working-precision setup. (Square-root K; the cube-root branch arrives with exp2_aux2.)
    let k_param = precy.div_ceil(2).floor_sqrt() + 3;
    let l = (precy - 1) / k_param + 1;
    let mut err = k_param + ((l << 1) + 18).ceiling_log_base_2();
    let mut q = precy + err + k_param + 10;
    // if |x| >> 1, account for the cancelled bits
    if expx > 0 {
        q += u64::try_from(expx).unwrap();
    }
    let mut increment = Limb::WIDTH;
    loop {
        let working = q + error_r;
        // s is within 1 ulp of log(2), rounded so that r = x - n*log(2) is bounded above.
        let s = Float::ln_2_prec_round(working, if n >= 0 { Down } else { Up }).0;
        // r = |n| * log(2) (directed); negate when n < 0, so r <= n*log(2) within 3 ulps.
        let mut r = s
            .mul_prec_round_ref_val(
                Float::from(n.unsigned_abs()),
                working,
                if n >= 0 { Down } else { Up },
            )
            .0;
        if n < 0 {
            r.neg_assign();
        }
        r = x.sub_prec_round_ref_val(r, working, Up).0;
        // if the initial n was too large, r came out negative: reduce n
        while r.is_normal() && r.is_sign_negative() {
            n -= 1;
            r.add_prec_round_assign_ref(&s, working, Up);
        }
        // if r is 0 we cannot round correctly; otherwise sum the series
        if r.is_normal() {
            // the cancelled low error_r bits of r are non-significant, so drop them
            if error_r > 0 {
                r.set_prec_round(q, Up);
            }
            // r = (x - n*log(2)) / 2^K, exact
            r >>= k_param;
            // ss <- 1 + r + r^2/2! + ... (naive method)
            let (mut ss, mut exps, l_err) = exp2_aux(r, q);
            // raise to the 2^K power by K squarings
            for _ in 0..k_param {
                ss.square_assign();
                exps <<= 1;
                let (ss2, sh) = mpz_normalize(ss, i64::try_from(q).unwrap());
                ss = ss2;
                exps += sh;
            }
            // s = ss * 2^exps (exact: ss has at most q bits and working >= q)
            let s = Float::from_integer_prec_round(ss, working, Nearest).0 << exps;
            // error is at most 2^K * l_err, plus 2 for the 3-ulp error on r
            err = k_param + l_err.ceiling_log_base_2() + 2;
            if float_can_round(s.significand_ref().unwrap(), q - err, precy, rm) {
                // y = s * 2^n. exp of a finite nonzero value is irrational, so the result is never
                // exactly representable; if the rounding came out exact we landed on a precision
                // boundary, and must add precision to determine the correct rounding direction.
                let (y, o) = s.shl_prec_round(n, precy, rm);
                if o != Equal {
                    return (y, o);
                }
                // The working approximation landed exactly on a precy-bit boundary; no test input
                // reaches this (it needs ss's tail below precy to be all-zero -- probability
                // ~2^-(q-precy)), but correctness requires looping rather than returning Equal.
                fail_on_untested_path("exp_2, approximation is exactly representable at precy");
            }
        } else {
            // r reduced to exactly zero, so the series can't be summed; add precision and retry.
            // Unreachable for dyadic x (x - n*log(2) is never exactly 0 at working precision), but
            // kept as a faithful port of MPFR's MPFR_IS_ZERO(r) guard.
            fail_on_untested_path("exp_2, argument reduction produced r = 0");
        }
        q += increment;
        increment = q >> 1;
    }
}

// The overflow result of exp (the value, which is positive, exceeds the maximum finite Float).
//
// This is `mpfr_overflow` (with positive sign) as used by `mpfr_exp`, MPFR 4.2.2.
fn exp_overflow(precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    match rm {
        Nearest | Up | Ceiling => (Float::INFINITY, Greater),
        Down | Floor => (Float::max_finite_value_with_prec(precy), Less),
        Exact => panic!("exp: Exact rounding was requested, but the result overflows"),
    }
}

// The underflow result of exp (the value, which is positive, is below the minimum positive Float).
// MPFR maps Nearest to toward-zero here, so Nearest joins Down/Floor.
//
// This is `mpfr_underflow` (with positive sign) as used by `mpfr_exp`, MPFR 4.2.2.
fn exp_underflow(precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    match rm {
        Nearest | Down | Floor => (Float::ZERO, Less),
        Up | Ceiling => (Float::min_positive_value_prec(precy), Greater),
        Exact => panic!("exp: Exact rounding was requested, but the result underflows"),
    }
}

// Computes `exp(x)` for finite nonzero `x`, rounded to precision `precy` with rounding mode `rm`.
// Detects overflow/underflow against `log(2)`-scaled exponent bounds, takes a fast path for tiny
// `x` (where `exp(x) = 1 +/- ulp(1)`), and otherwise calls `exp_2`. (The high-precision `exp_3`
// path is added later; `exp_2` is correct at all precisions.)
//
// This is the finite-nonzero branch of `mpfr_exp` from `exp.c`, MPFR 4.2.2.
fn exp_prec_round_normal_ref(x: &Float, precy: u64, rm: RoundingMode) -> (Float, Ordering) {
    // exp of a finite nonzero value is transcendental, hence never exactly representable.
    assert_ne!(rm, Exact, "Inexact exp");
    // Overflow/underflow bounds, as ~64-bit Floats. Directed rounding makes `bound_emax` an upper
    // bound on emax*log(2) and `bound_emin` a lower bound on (emin - 2)*log(2), so the comparisons
    // below are sound one-sided tests.
    const BP: u64 = 64;
    let log2_up = Float::ln_2_prec_round(BP, Up).0;
    let bound_emax = log2_up
        .mul_prec_round_ref_val(
            const { Float::const_from_signed(Float::MAX_EXPONENT as SignedLimb) },
            BP,
            Up,
        )
        .0;
    if *x >= bound_emax {
        // x > log(2^emax), so exp(x) > 2^emax
        return exp_overflow(precy, rm);
    }
    let bound_emin = log2_up
        .mul_prec_round(
            const { Float::const_from_signed((Float::MIN_EXPONENT as SignedLimb) - 2) },
            BP,
            Floor,
        )
        .0;
    if *x <= bound_emin {
        // x < log(2^(emin - 2)), so exp(x) < 2^(emin - 2)
        return exp_underflow(precy, rm);
    }
    let expx = i64::from(x.get_exponent().unwrap());
    // tiny x: if x < 2^(-precy), then exp(x) = 1 +/- ulp(1)
    if expx < 0 && u64::try_from(-expx).unwrap() > precy {
        return if x.is_sign_negative() && (rm == Down || rm == Floor) {
            (one_neighbor(precy, false), Less) // 1 - ulp
        } else if x.is_sign_positive() && (rm == Up || rm == Ceiling) {
            (one_neighbor(precy, true), Greater) // 1 + ulp
        } else {
            (
                Float::one_prec(precy),
                if x.is_sign_positive() { Less } else { Greater },
            )
        };
    }
    exp_2(x, precy, rm)
}

// The neighbor of 1 at precision `prec`: the successor `1 + 2 ^ (1 - prec)` if `above`, otherwise
// the predecessor `1 - 2 ^ (-prec)`. Both are exactly representable at precision `prec`. (Note that
// `Float::increment`/`decrement` cannot be used here: they keep the ulp of the current binade, so
// they bump the precision when crossing into the next binade and overshoot the true predecessor.)
fn one_neighbor(prec: u64, above: bool) -> Float {
    let (m, shift) = if above {
        (Natural::power_of_2(prec - 1) + Natural::ONE, prec - 1)
    } else {
        (Natural::power_of_2(prec) - Natural::ONE, prec)
    };
    Float::from_natural_prec(m, prec).0 >> shift
}

// Public API. Variants delegate naively (val -> ref, prec -> prec_round with Nearest, round ->
// prec_round at the input's precision); optimizing these is step 6. Docs are step 7.
impl Float {
    #[inline]
    pub fn exp_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.exp_prec_round_ref(prec, rm)
    }

    pub fn exp_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // exp(+inf) = +inf; exp(-inf) = +0
            Infinity { sign } => {
                if *sign {
                    (Self::INFINITY, Equal)
                } else {
                    (Self::ZERO, Equal)
                }
            }
            // exp(+0) = exp(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => exp_prec_round_normal_ref(self, prec, rm),
        }
    }

    #[inline]
    pub fn exp_prec(self, prec: u64) -> (Self, Ordering) {
        self.exp_prec_round(prec, Nearest)
    }

    #[inline]
    pub fn exp_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.exp_prec_round_ref(prec, Nearest)
    }

    #[inline]
    pub fn exp_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.exp_prec_round(prec, rm)
    }

    #[inline]
    pub fn exp_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.exp_prec_round_ref(prec, rm)
    }

    #[inline]
    pub fn exp_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        let o;
        (*self, o) = x.exp_prec_round(prec, rm);
        o
    }

    #[inline]
    pub fn exp_prec_assign(&mut self, prec: u64) -> Ordering {
        self.exp_prec_round_assign(prec, Nearest)
    }

    #[inline]
    pub fn exp_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.exp_prec_round_assign(prec, rm)
    }
}

impl Exp for Float {
    type Output = Self;

    #[inline]
    fn exp(self) -> Self {
        let prec = self.significant_bits();
        self.exp_prec_round(prec, Nearest).0
    }
}

impl Exp for &Float {
    type Output = Float;

    #[inline]
    fn exp(self) -> Float {
        let prec = self.significant_bits();
        self.exp_prec_round_ref(prec, Nearest).0
    }
}

impl ExpAssign for Float {
    #[inline]
    fn exp_assign(&mut self) {
        let prec = self.significant_bits();
        self.exp_prec_round_assign(prec, Nearest);
    }
}
