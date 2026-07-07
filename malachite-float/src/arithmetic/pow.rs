// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      `mpfr_pow`, `mpfr_pow_general`, and `mpfr_pow_is_exact` from `pow.c`, and `mpfr_pow_z` and
//      `mpfr_pow_pos_z` from `pow_z.c`; MPFR 4.3.0.
//
//      Copyright 2005-2024 Free Software Foundation, Inc. Contributed by the AriC and Caramba
//      projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::arithmetic::exp::{exp_overflow, exp_underflow, one_neighbor};
use crate::emulate_float_float_to_float_fn;
use crate::{Float, float_either_infinity, float_either_zero, float_nan};
use core::cmp::Ordering::{self, *};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, CheckedSqrt, IsPowerOf2, NegAssign, Parity, Pow, PowAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, One,
    Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;

// This is MPFR_POW_EXP_THRESHOLD from `pow.c`, MPFR 4.3.0.
const POW_EXP_THRESHOLD: i64 = 256;

// Whether y is an odd integer. This is equivalent to `mpfr_odd_p` from `mpfr-impl.h`, MPFR 4.3.0,
// for finite nonzero y.
fn float_odd_integer(y: &Float) -> bool {
    if !y.is_finite() || y.is_zero() || !y.is_integer() {
        return false;
    }
    // y = m * 2^(e - b) with m the b-bit significand: y is odd iff its unit bit is set, i.e. the
    // significand's trailing zero count is exactly b - e. (For e > b, y is an even integer.) This
    // avoids materializing the integer, whose bit length is the exponent and can be huge.
    let e = i64::from(y.get_exponent().unwrap());
    let m = y.significand_ref().unwrap();
    let b = i64::exact_from(m.significant_bits());
    e <= b && i64::exact_from(m.trailing_zeros().unwrap()) == b - e
}

// MPFR's `mpfr_underflow` as used by `mpfr_pow`: the callers pre-map Nearest per MPFR's convention.
// A negative result mirrors the positive case with the rounding mode negated.
fn pow_underflow(prec: u64, rm: RoundingMode, negative: bool) -> (Float, Ordering) {
    if negative {
        let (f, o) = exp_underflow(prec, -rm);
        (-f, o.reverse())
    } else {
        exp_underflow(prec, rm)
    }
}

// MPFR's `mpfr_overflow` as used by `mpfr_pow`.
fn pow_overflow(prec: u64, rm: RoundingMode, negative: bool) -> (Float, Ordering) {
    if negative {
        let (f, o) = exp_overflow(prec, -rm);
        (-f, o.reverse())
    } else {
        exp_overflow(prec, rm)
    }
}

// Whether the significand of a finite nonzero Float is a power of 2 (sign-agnostic). This is
// equivalent to `mpfr_powerof2_raw` from `mpfr-impl.h`, MPFR 4.3.0.
fn raw_power_of_2(x: &Float) -> bool {
    x.significand_ref().unwrap().is_power_of_2()
}

// The tiny-argument result 1 +/- ulp(1), following the tiny-x fast path of `mpfr_exp` and
// MPFR_SMALL_INPUT_AFTER_SAVE_EXPO: the exact result is 1 + eps with sign(eps) given by `above`.
fn float_one_plus_tiny(prec: u64, rm: RoundingMode, above: bool) -> (Float, Ordering) {
    match (rm, above) {
        (Up | Ceiling, true) => (one_neighbor(prec, true), Greater),
        (Down | Floor, false) => (one_neighbor(prec, false), Less),
        (_, true) => (Float::one_prec(prec), Less),
        (_, false) => (Float::one_prec(prec), Greater),
    }
}

// This is `mpfr_pow_pos_z` from `pow_z.c`, MPFR 4.3.0, with z positive. If `cr` is true the result
// is correctly rounded; otherwise `prec` is used as the working precision. Returns the result and
// its ordering; the result may be infinite or zero on intermediate overflow or underflow (the
// callers handle those cases).
fn pow_pos_natural(
    x: &Float,
    z: &Natural,
    prec: u64,
    rm: RoundingMode,
    cr: bool,
) -> (Float, Ordering) {
    assert_ne!(*z, 0u32);
    if *z == 1u32 {
        return Float::from_float_prec_round_ref(x, prec, rm);
    }
    let size_z = z.significant_bits();
    // Rounding directions chosen so that all intermediate roundings go the same way, making an
    // intermediate overflow or underflow a true exception rather than rounding noise.
    let x_exp_ge_1 = x.get_exponent().unwrap() >= 1;
    let rnd1 = if x_exp_ge_1 {
        Down
    } else if x.is_sign_positive() {
        Up
    } else {
        Floor
    };
    let rnd2 = if x_exp_ge_1 { Floor } else { Up };
    let mut wprec = if cr {
        prec + 3 + size_z + prec.ceiling_log_base_2()
    } else {
        prec
    };
    loop {
        let mut inexmul;
        let err = wprec - 1 - size_z;
        let mut i = size_z;
        let (mut res, o) = x.square_prec_round_ref(wprec, rnd2);
        inexmul = o != Equal;
        assert!(i >= 2);
        if z.get_bit(i - 2) {
            let o = res.mul_prec_round_assign_ref(x, wprec, rnd1);
            inexmul |= o != Equal;
        }
        if i > 2 {
            i -= 3;
            while res.is_finite() && !res.is_zero() {
                let o = res.square_prec_round_assign(wprec, rnd2);
                inexmul |= o != Equal;
                if z.get_bit(i) {
                    let o = res.mul_prec_round_assign_ref(x, wprec, rnd1);
                    inexmul |= o != Equal;
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        // In the shrinking regime (x's exponent < 1), rnd1/rnd2 are Up-directed, so `res` is an
        // upper bound and can never round to zero. An inexact upper bound equal to the minimum
        // positive Float proves the true value lies below it: a true underflow, reported as zero so
        // the caller applies its underflow handling. (Values elsewhere in the bottom binade are
        // representable and pass through normally; in the growing regime magnitudes only increase,
        // so this cannot trigger.)
        if !x_exp_ge_1
            && inexmul
            && res.is_finite()
            && !res.is_zero()
            && i64::from(res.get_exponent().unwrap()) == i64::from(Float::MIN_EXPONENT)
            && raw_power_of_2(&res)
        {
            res = if res.is_sign_negative() {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            };
        }
        let is_zero = res.is_zero();
        let exceptional = res.is_infinite() || is_zero;
        if !inexmul
            || !cr
            || exceptional
            || float_can_round(res.significand_ref().unwrap(), err, prec, rm)
        {
            if exceptional {
                // overflow or underflow: the sign and the exceptional value are already correct
                if !is_zero {
                    // The growing regime rounds toward zero (lower bounds), and the entry check
                    // bounds the true value below 2^MAX_EXPONENT.
                    fail_on_untested_path("pow_pos_natural, overflow");
                }
                let o = if is_zero { Less } else { Greater };
                return (res, o);
            }
            return Float::from_float_prec_round(res, prec, rm);
        }
        wprec += wprec >> 1;
    }
}

// The round-to-nearest underflow fallback of `mpfr_pow_pos_z` from `pow_z.c`, MPFR 4.3.0:
// nearest-mode underflow must choose between 0 and 2^(emin - 1) according to which side of 2^(emin
// - 2) the true value lies, which the multiplication-based path cannot know. Rerun via pow_general
// at 2 bits of precision: its 2^k scaling keeps the computation in range, and the final
// shl_prec_round applies the correct nearest-mode underflow rounding.
fn pow_integer_underflow_nearest(x: &Float, z: &Integer, prec: u64) -> (Float, Ordering) {
    let z_bits = z.significant_bits();
    let zz = Float::from_integer_prec_round(z.clone(), z_bits, Exact).0;
    let (y2, o) = pow_general(x, &zz, 2, Nearest, true);
    (Float::from_float_prec_round(y2, prec, Exact).0, o)
}

// This is `mpfr_pow_z` from `pow_z.c`, MPFR 4.3.0.
fn pow_integer(x: &Float, z: &Integer, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if *z == 0u32 {
        // The public entry handles y = 0 before calling pow_integer.
        fail_on_untested_path("pow_integer, z == 0");
        return Float::from_float_prec_round(Float::ONE, prec, rm);
    }
    if x.is_nan() {
        // The public entry filters singular x before calling pow_integer.
        fail_on_untested_path("pow_integer, NaN x");
        return (Float::NAN, Equal);
    }
    let z_pos = *z > 0u32;
    let z_odd = z.odd();
    if x.is_infinite() {
        // The public entry filters singular x before calling pow_integer.
        fail_on_untested_path("pow_integer, infinite x");
        let negative = x.is_sign_negative() && z_odd;
        return (
            match (z_pos, negative) {
                (true, false) => Float::INFINITY,
                (true, true) => Float::NEGATIVE_INFINITY,
                (false, false) => Float::ZERO,
                (false, true) => Float::NEGATIVE_ZERO,
            },
            Equal,
        );
    }
    if x.is_zero() {
        // The public entry filters singular x before calling pow_integer.
        fail_on_untested_path("pow_integer, zero x");
        let negative = x.is_sign_negative() && z_odd;
        return (
            match (z_pos, negative) {
                (true, false) => Float::ZERO,
                (true, true) => Float::NEGATIVE_ZERO,
                (false, false) => Float::INFINITY,
                (false, true) => Float::NEGATIVE_INFINITY,
            },
            Equal,
        );
    }
    // x = +/-2^b: x^z = (+/-1)^z * 2^(z*(b-1)+1-1)... handled exactly via the exponent.
    if raw_power_of_2(x) {
        let ex = i64::from(x.get_exponent().unwrap());
        let sign_negative = x.is_sign_negative() && z_odd;
        // new exponent = z * (ex - 1) + 1
        let new_exp = z * Integer::from(ex - 1) + Integer::ONE;
        let base = if sign_negative {
            -Float::one_prec(prec)
        } else {
            Float::one_prec(prec)
        };
        return if new_exp < Float::MIN_EXPONENT {
            pow_underflow(prec, if rm == Nearest { Down } else { rm }, sign_negative)
        } else if new_exp > Float::MAX_EXPONENT {
            // z(ex - 1) + 1 > MAX_EXPONENT implies z * log2|x| >= MAX_EXPONENT (the product is an
            // exact integer at 64 bits here), which the entry's early overflow check already
            // caught.
            fail_on_untested_path("pow_integer, power-of-2 overflow");
            pow_overflow(prec, rm, sign_negative)
        } else {
            let sh = i64::exact_from(&(new_exp - Integer::ONE));
            base.shl_prec_round(sh, prec, rm)
        };
    }
    // Pre-bound the result exponent: result_exp ~ z * log2|x|. When it is far outside the exponent
    // range (with a wide margin for the estimate's error), report the exception directly instead of
    // letting the exponentiation saturate; this mirrors the role of MPFR's underflow/overflow
    // flags, which malachite does not have, and keeps the Ziv loop from ballooning on saturated
    // values.
    let est = f64::rounding_from(x.abs().log_base_2_prec(64).0, Nearest).0
        * f64::rounding_from(z, Nearest).0;
    let negative = x.is_sign_negative() && z_odd;
    if est > const { Float::MAX_EXPONENT as f64 + 64.0 } {
        // est > MAX_EXPONENT + 64 implies the entry's early overflow check already caught it.
        fail_on_untested_path("pow_integer, pre-bound overflow");
        return pow_overflow(prec, rm, negative);
    }
    if est < const { Float::MIN_EXPONENT as f64 - 64.0 } {
        return pow_underflow(prec, if rm == Nearest { Down } else { rm }, negative);
    }
    if z_pos {
        let (result, o) = pow_pos_natural(x, z.unsigned_abs_ref(), prec, rm, true);
        if result.is_zero() {
            // pow_pos_natural only returns zero when the result underflowed.
            return if rm == Nearest {
                pow_integer_underflow_nearest(x, z, prec)
            } else {
                pow_underflow(prec, rm, x.is_sign_negative() && z_odd)
            };
        }
        (result, o)
    } else {
        // z < 0: compute (1/x)^|z| via t = 1/x rounded toward 1/-1, then a non-correctly-rounded
        // positive power at extended precision, with a Ziv loop.
        let abs_z = z.unsigned_abs_ref();
        let size_z = abs_z.significant_bits();
        let mut wprec = prec + size_z + 3 + prec.ceiling_log_base_2();
        let rnd1 = if x.get_exponent().unwrap() < 1 {
            Down
        } else if x.is_sign_positive() {
            Up
        } else {
            Floor
        };
        loop {
            let t = Float::ONE.div_prec_round_val_ref(x, wprec, rnd1).0;
            if t.is_infinite() {
                // For |x| < 1 the reciprocal is rounded toward zero and cannot reach infinity; for
                // |x| >= 1 it is at most 1.
                fail_on_untested_path("pow_integer, 1/x overflow");
                return pow_overflow(prec, rm, t.is_sign_negative());
            }
            let t = pow_pos_natural(&t, abs_z, wprec, rm, false).0;
            if t.is_infinite() {
                // The entry check bounds |x^y| < 2^MAX_EXPONENT, and the magnitude-decreasing
                // rounding directions keep the computed value below it.
                fail_on_untested_path("pow_integer, (1/x)^|z| overflow");
                return pow_overflow(prec, rm, t.is_sign_negative());
            }
            if t.is_zero() {
                if rm == Nearest {
                    return pow_integer_underflow_nearest(x, z, prec);
                }
                return pow_underflow(prec, rm, x.is_sign_negative() && z_odd);
            }
            let err = wprec - size_z - 2;
            if float_can_round(t.significand_ref().unwrap(), err, prec, rm) {
                return Float::from_float_prec_round(t, prec, rm);
            }
            wprec += wprec >> 1;
        }
    }
}

// This is `mpfr_pow_is_exact` from `pow.c`, MPFR 4.3.0: assuming x > 0, x not a power of 2, y
// finite non-integer, decides whether x^y is exact, and if so computes it.
fn pow_is_exact(x: &Float, y: &Float, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    if y.is_sign_negative() {
        return None;
    }
    // y = c * 2^d with c an odd integer, d < 0
    let (c, mut d) = float_to_odd_mantissa_and_exponent(y);
    // y is not an integer (the callers filter integers), so it has fractional bits.
    assert!(d < 0);
    // x = a * 2^b with a odd
    let (mut a, mut b) = float_to_odd_mantissa_and_exponent_natural(x);
    while d != 0 {
        if b.odd() {
            a <<= 1u32;
            b -= 1;
        }
        a = a.checked_sqrt()?;
        b >>= 1;
        d += 1;
    }
    // x^y = (a * 2^b)^c with c an odd integer
    let tmp_prec = a.significant_bits();
    let tmp = Float::from_natural_prec_round(a, tmp_prec, Exact)
        .0
        .shl_prec_round(b, tmp_prec, Exact)
        .0;
    Some(pow_integer(&tmp, &c, prec, rm))
}

// This is `mpfr_pow_general` from `pow.c`, MPFR 4.3.0: the Ziv loop computing exp(y * ln|x|), with
// a scaling factor 2^k to dodge intermediate overflow and underflow.
fn pow_general(
    x: &Float,
    y: &Float,
    prec: u64,
    mut rm: RoundingMode,
    y_is_integer: bool,
) -> (Float, Ordering) {
    let abs_x = x.abs();
    let mut neg_result = false;
    if x.is_sign_negative() {
        assert!(y_is_integer);
        if float_odd_integer(y) {
            neg_result = true;
            rm.neg_assign(); // invert directed modes; Nearest stays
        }
    }
    let mut wprec = prec + 9 + prec.ceiling_log_base_2();
    let mut k: Option<Integer> = None;
    let mut check_exact_case = false;
    let mut exact_case = false;
    let mut result;
    let mut o;
    loop {
        // t = ln|x|, rounded so that t is an upper bound on y * ln|x|
        let mut t = abs_x
            .ln_prec_round_ref(wprec, if y.is_sign_negative() { Floor } else { Ceiling })
            .0;
        t.mul_prec_round_assign_ref(y, wprec, Ceiling);
        let exp_t = t.get_exponent().map_or(0, i64::from);
        if let Some(kv) = &k {
            t.sub_prec_round_assign(
                Float::ln_2_prec_round(wprec, Floor)
                    .0
                    .mul_prec_round(
                        Float::from_signed_prec(i64::exact_from(kv), wprec).0,
                        wprec,
                        Floor,
                    )
                    .0,
                wprec,
                Ceiling,
            );
        }
        let mut err = if !t.is_zero() && exp_t >= -1 {
            exp_t + 3
        } else {
            1
        };
        if let Some(kv) = &k {
            let exp_k = i64::exact_from(kv.significant_bits());
            if exp_k > err {
                err = exp_k;
            }
            err += 1;
        }
        let t = t.exp_prec(wprec).0;
        // MPFR checks the underflow flag here, which also fires when the result rounds UP into the
        // bottom binade (e.g. to the minimum positive value); malachite has no flags, so treat any
        // bottom-binade result as "possibly spurious underflow" and take the 2^k rescue path, which
        // recomputes in a comfortable range.
        let t_bottom_binade = t.is_finite()
            && !t.is_zero()
            && k.is_none()
            && t.get_exponent()
                .is_some_and(|e| i64::from(e) == i64::from(Float::MIN_EXPONENT));
        if t.is_zero() || t.is_infinite() || t_bottom_binade {
            // After a 2^k rescue the computation stays comfortably in range, so a singular result
            // cannot recur (MPFR_ASSERTN(!k_non_zero) in mpfr_pow_general).
            assert!(k.is_none());
            if t.is_zero() {
                // real underflow of |x|^y
                (result, o) = pow_underflow(prec, if rm == Nearest { Down } else { rm }, false);
                break;
            }
            if t.is_infinite() {
                // possible overflow: recompute a lower bound
                let t2 = abs_x
                    .ln_prec_round_ref(wprec, if y.is_sign_negative() { Ceiling } else { Floor })
                    .0
                    .mul_prec_round_val_ref(y, wprec, Floor)
                    .0
                    .exp_prec_round(wprec, Floor)
                    .0;
                if t2.is_infinite() {
                    // The entry check bounds |x^y| < 2^MAX_EXPONENT, so the lower-bound
                    // recomputation cannot be infinite.
                    fail_on_untested_path("pow_general, confirmed overflow");
                    (result, o) = pow_overflow(prec, rm, false);
                    break;
                }
            }
            // scale by 2^-k with k ~ y*log2|x|
            k = Some(
                Integer::rounding_from(
                    abs_x.log_base_2_prec_ref(64).0.mul_prec_val_ref(y, 64).0,
                    Nearest,
                )
                .0,
            );
            continue;
        }
        if float_can_round(
            t.significand_ref().unwrap(),
            wprec.checked_sub(u64::exact_from(err.max(0))).unwrap_or(1),
            prec,
            rm,
        ) {
            (result, o) = Float::from_float_prec_round(t, prec, rm);
            break;
        }
        if !check_exact_case && !y_is_integer {
            if let Some((z, oz)) = pow_is_exact(&abs_x, y, prec, rm) {
                result = z;
                o = oz;
                exact_case = true;
                break;
            }
            check_exact_case = true;
        }
        wprec += wprec >> 1;
    }
    if !exact_case && let Some(kv) = &k {
        let lk = i64::exact_from(kv);
        // Double-rounding guard from `mpfr_pow_general`: in rounding to nearest, if the scaled
        // result would be exactly 2^(emin - 2) but the unscaled rounding already went below the
        // exact value, the true result is above the underflow tie point and must round up to
        // 2^(emin - 1), not down to zero. (The result is positive here; the sign is applied below.)
        let mut shift_rm = rm;
        if rm == Nearest
            && o == Less
            && lk < 0
            && result
                .get_exponent()
                .is_some_and(|e| i64::from(e) == i64::from(Float::MIN_EXPONENT) - 1 - lk)
            && raw_power_of_2(&result)
        {
            shift_rm = Ceiling;
        }
        let (shifted, oo) = result.shl_prec_round(lk, prec, shift_rm);
        result = shifted;
        if oo != Equal {
            o = oo;
        }
    }
    if neg_result {
        result.neg_assign();
        o = o.reverse();
    }
    (result, o)
}

// Decomposes a finite nonzero Float into (odd Integer mantissa, exponent): x = c * 2^d.
fn float_to_odd_mantissa_and_exponent(x: &Float) -> (Integer, i64) {
    let (n, d) = float_to_odd_mantissa_and_exponent_natural(&x.abs());
    (Integer::from_sign_and_abs(x.is_sign_positive(), n), d)
}

fn float_to_odd_mantissa_and_exponent_natural(x: &Float) -> (Natural, i64) {
    let m = x.significand_ref().unwrap().clone();
    let e = i64::from(x.get_exponent().unwrap()) - i64::exact_from(m.significant_bits());
    let tz = m.trailing_zeros().unwrap();
    (m >> tz, e + i64::exact_from(tz))
}

impl Float {
    // This is `mpfr_pow` from `pow.c`, MPFR 4.3.0.

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// with the specified rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p,m)=1.0$
    /// - $f(-1.0,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is
    ///   negative and not an odd integer
    /// - $f(0.0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::pow_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::pow_round_ref_ref`] instead. If both of these things are true,
    /// consider using [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_ref(&Float::from(2.5), 5, Floor);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_ref(&Float::from(2.5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_ref(&Float::from(2.5), 5, Nearest);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_ref(&Float::from(2.5), 20, Floor);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_ref(&Float::from(2.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "15.58847");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_ref(&Float::from(2.5), 20, Nearest);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_prec_round_ref_ref(
        &self,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // Exact rounding: compute with Nearest and demand exactness (the exact cases all flow
        // through the integer-power and exact-power paths, which report Equal).
        if rm == Exact {
            let (result, o) = self.pow_prec_ref_ref(y, prec);
            assert_eq!(o, Equal, "Inexact pow");
            return (result, Equal);
        }
        let x = self;
        // Singular cases; see Section F.9.4.4 of the C standard.
        match (x, y) {
            // pow(x, 0) = 1 for any x, even NaN
            (_, float_either_zero!()) => {
                return Self::from_float_prec_round(Self::ONE, prec, rm);
            }
            (float_nan!(), _) => return (Self::NAN, Equal),
            // pow(+1, NaN) = 1
            (_, float_nan!()) => {
                return if *x == 1u32 {
                    Self::from_float_prec_round(Self::ONE, prec, rm)
                } else {
                    (Self::NAN, Equal)
                };
            }
            (float_either_infinity!(), Self(Infinity { sign })) => {
                return if *sign {
                    (Self::INFINITY, Equal)
                } else {
                    (Self::ZERO, Equal)
                };
            }
            (_, Self(Infinity { sign })) => {
                let mut cmp = x.partial_cmp_abs(&Self::ONE).unwrap();
                if !*sign {
                    cmp = cmp.reverse();
                }
                return match cmp {
                    Greater => (Self::INFINITY, Equal),
                    Less => (Self::ZERO, Equal),
                    Equal => Self::from_float_prec_round(Self::ONE, prec, rm),
                };
            }
            (Self(Infinity { sign }), _) => {
                let negative = !*sign && float_odd_integer(y);
                return (
                    match (y.is_sign_positive(), negative) {
                        (true, false) => Self::INFINITY,
                        (true, true) => Self::NEGATIVE_INFINITY,
                        (false, false) => Self::ZERO,
                        (false, true) => Self::NEGATIVE_ZERO,
                    },
                    Equal,
                );
            }
            (Self(Zero { sign }), _) => {
                let negative = !*sign && float_odd_integer(y);
                return (
                    match (y.is_sign_negative(), negative) {
                        (true, false) => Self::INFINITY,
                        (true, true) => Self::NEGATIVE_INFINITY,
                        (false, false) => Self::ZERO,
                        (false, true) => Self::NEGATIVE_ZERO,
                    },
                    Equal,
                );
            }
            _ => {}
        }
        // x^y for x < 0 and y not an integer is not defined
        let y_is_integer = y.is_integer();
        if x.is_sign_negative() && !y_is_integer {
            return (Self::NAN, Equal);
        }
        let cmp_x_1 = x.partial_cmp_abs(&Self::ONE).unwrap();
        if cmp_x_1 == Equal {
            let negative = x.is_sign_negative() && float_odd_integer(y);
            return Self::from_float_prec_round(
                if negative { -Self::ONE } else { Self::ONE },
                prec,
                rm,
            );
        }
        let ex = i64::from(x.get_exponent().unwrap());
        let ey = i64::from(y.get_exponent().unwrap());
        // Fast check for no possible overflow or underflow: |y| <= 2^15 and moderate ex means |y *
        // log2|x|| stays far from the exponent limits.
        let no_over_under = ey <= 15 && -32767 < ex && ex <= 32767;
        if !no_over_under {
            // early overflow detection: lower bound on y * log2|x|
            if (cmp_x_1 == Greater) == y.is_sign_positive() {
                let t = x
                    .abs()
                    .log_base_2_prec_round_ref(64, Down)
                    .0
                    .mul_prec_round_val_ref(y, 64, Down)
                    .0;
                if t >= const { Self::const_from_signed(Self::MAX_EXPONENT as i64) } {
                    return pow_overflow(prec, rm, x.is_sign_negative() && float_odd_integer(y));
                }
            }
            // early underflow detection: ebound such that |x^y| < 2^ebound
            if if y.is_sign_negative() { ex > 1 } else { ex < 0 } {
                let mut tmp = Self::from_signed_prec(ex, 64).0;
                if y.is_sign_negative() {
                    tmp.sub_prec_assign(Self::ONE, 64);
                }
                tmp.mul_prec_round_assign_ref(y, 64, Ceiling);
                let mut ebound = i64::rounding_from(&tmp, Ceiling).0;
                // For y < 0 the bound |x^y| <= 2^((ex - 1) * y) is not strict, so if the product is
                // an exact integer the exponent bound must be bumped to keep |x^y| < 2^ebound
                // (mpfr_nextabove(tmp) in mpfr_pow); otherwise x = 2^(ex - 1) exactly achieves the
                // bound and a representable result would be misreported as underflow.
                if y.is_sign_negative() && tmp == ebound {
                    ebound += 1;
                }
                let lim = i64::from(Self::MIN_EXPONENT) - if rm == Nearest { 2 } else { 1 };
                if ebound <= lim {
                    return pow_underflow(
                        prec,
                        if rm == Nearest { Down } else { rm },
                        x.is_sign_negative() && float_odd_integer(y),
                    );
                }
            }
        }
        // y a not-too-large integer: use the multiplication-based algorithm
        if y_is_integer && ey <= POW_EXP_THRESHOLD {
            return pow_integer(x, &Integer::rounding_from(y, Nearest).0, prec, rm);
        }
        // (+/-2^b)^y, which could be exact
        if raw_power_of_2(x) {
            if x.is_sign_negative() {
                // necessarily ey > threshold; |x| <= 1/2 means underflow (overflow was already
                // detected above)
                let negative = float_odd_integer(y);
                return pow_underflow(prec, if rm == Nearest { Down } else { rm }, negative);
            }
            let b = ex - 1;
            let (tmp, o) = y.mul_prec_ref_val(Self::from(b), y.significant_bits() + 64);
            assert_eq!(o, Equal);
            return Self::power_of_2_of_float_prec_round(tmp, prec, rm);
        }
        // y * ln(x) very small: 1 + tiny
        let expx = if cmp_x_1 == Less { 1 - ex } else { ex };
        let logt = i64::exact_from(u64::exact_from(expx.max(1)).ceiling_log_base_2());
        let err = ey + logt;
        if err < -i64::exact_from(prec) - 1 {
            let above = y.is_sign_positive() == (cmp_x_1 == Greater);
            return float_one_plus_tiny(prec, rm, above);
        }
        pow_general(x, y, prec, rm, y_is_integer)
    }
}

impl Float {
    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// with the specified rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p,m)=1.0$
    /// - $f(-1.0,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is
    ///   negative and not an odd integer
    /// - $f(0.0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::pow_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::pow_round`] instead. If both of these things are true, consider using
    /// [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_prec_round(Float::from(2.5), 5, Floor);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round(Float::from(2.5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round(Float::from(2.5), 5, Nearest);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round(Float::from(2.5), 20, Floor);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round(Float::from(2.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "15.58847");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round(Float::from(2.5), 20, Nearest);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(&other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// with the specified rounding mode. The first [`Float`] is taken by value and the second by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p,m)=1.0$
    /// - $f(-1.0,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is
    ///   negative and not an odd integer
    /// - $f(0.0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::pow_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::pow_round_val_ref`] instead. If both of these things are true,
    /// consider using [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_prec_round_val_ref(&Float::from(2.5), 5, Floor);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round_val_ref(&Float::from(2.5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round_val_ref(&Float::from(2.5), 5, Nearest);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round_val_ref(&Float::from(2.5), 20, Floor);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round_val_ref(&Float::from(2.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "15.58847");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::from(3).pow_prec_round_val_ref(&Float::from(2.5), 20, Nearest);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// with the specified rounding mode. The first [`Float`] is taken by reference and the second
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p,m)=1.0$
    /// - $f(-1.0,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is
    ///   negative and not an odd integer
    /// - $f(0.0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::pow_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::pow_round_ref_val`] instead. If both of these things are true,
    /// consider using [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_val(Float::from(2.5), 5, Floor);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_val(Float::from(2.5), 5, Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_val(Float::from(2.5), 5, Nearest);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_val(Float::from(2.5), 20, Floor);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_val(Float::from(2.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "15.58847");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_round_ref_val(Float::from(2.5), 20, Nearest);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(&other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// to the nearest value. Both [`Float`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p)=1.0$
    /// - $f(-1.0,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_prec(Float::from(2.5), 5);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec(Float::from(2.5), 20);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_ref_ref(&other, prec)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// to the nearest value. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p)=1.0$
    /// - $f(-1.0,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_ref_ref(&Float::from(2.5), 5);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_ref_ref(&Float::from(2.5), 20);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the maximum of the
    /// precisions of the two inputs and with the specified rounding mode. Both [`Float`]s are taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,m)=f(x,\text{NaN},m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,m)=1.0$
    /// - $f(-1.0,y,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_round(Float::from(2.5), Floor);
    /// assert_eq!(p.to_string(), "14.0");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_round(Float::from(2.5), Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::from(3).pow_round(Float::from(2.5), Nearest);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn pow_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_ref_ref(&other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the maximum of the
    /// precisions of the two inputs and with the specified rounding mode. Both [`Float`]s are taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,m)=f(x,\text{NaN},m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,m)=1.0$
    /// - $f(-1.0,y,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec_round_ref_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_round_ref_ref(&Float::from(2.5), Floor);
    /// assert_eq!(p.to_string(), "14.0");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_round_ref_ref(&Float::from(2.5), Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = (&Float::from(3)).pow_round_ref_ref(&Float::from(2.5), Nearest);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn pow_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_ref_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the maximum of the
    /// precisions of the two inputs and with the specified rounding mode. The first [`Float`] is
    /// taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,m)=f(x,\text{NaN},m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,m)=1.0$
    /// - $f(-1.0,y,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec_round_val_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_round_val_ref(&Float::from(2.5), Floor);
    /// assert_eq!(p.to_string(), "14.0");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_round_val_ref(&Float::from(2.5), Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::from(3).pow_round_val_ref(&Float::from(2.5), Nearest);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        self.pow_round_ref_ref(other, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the maximum of the
    /// precisions of the two inputs and with the specified rounding mode. The first [`Float`] is
    /// taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,m)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,m)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,m)=f(x,\text{NaN},m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,m)=1.0$
    /// - $f(-1.0,y,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,m)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above, with
    ///   the rounding directions reflected.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec_round_ref_val`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_round_ref_val(Float::from(2.5), Floor);
    /// assert_eq!(p.to_string(), "14.0");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_round_ref_val(Float::from(2.5), Ceiling);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = (&Float::from(3)).pow_round_ref_val(Float::from(2.5), Nearest);
    /// assert_eq!(p.to_string(), "16.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        self.pow_round_ref_ref(&other, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// to the nearest value. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p)=1.0$
    /// - $f(-1.0,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_prec_val_ref(&Float::from(2.5), 5);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_prec_val_ref(&Float::from(2.5), 20);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_ref_ref(other, prec)
    }

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the specified precision and
    /// to the nearest value. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(\text{NaN},y,p)=f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(-1.0,\pm\infty,p)=1.0$
    /// - $f(-1.0,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and
    ///   not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(0.0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    /// - $f(x,y,p)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Pow::pow`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_ref_val(Float::from(2.5), 5);
    /// assert_eq!(p.to_string(), "15.5");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_prec_ref_val(Float::from(2.5), 20);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_ref_ref(&other, prec)
    }

    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::pow_prec_assign`] instead. If
    /// you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::pow_round_assign`] instead. If both of these things are true,
    /// consider using [`PowAssign::pow_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign(Float::from(2.5), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign(Float::from(2.5), 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "16.0");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign(Float::from(2.5), 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign(Float::from(2.5), 20, Floor), Less);
    /// assert_eq!(x.to_string(), "15.58846");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign(Float::from(2.5), 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "15.58847");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign(Float::from(2.5), 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "15.58846");
    /// ```
    pub fn pow_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = self.pow_prec_round_ref_ref(&other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] on the right-hand side is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::pow_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::pow_round_assign_ref`] instead. If both of these things are
    /// true, consider using [`PowAssign::pow_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign_ref(&Float::from(2.5), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign_ref(&Float::from(2.5), 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "16.0");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign_ref(&Float::from(2.5), 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign_ref(&Float::from(2.5), 20, Floor), Less);
    /// assert_eq!(x.to_string(), "15.58846");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign_ref(&Float::from(2.5), 20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "15.58847");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_round_assign_ref(&Float::from(2.5), 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "15.58846");
    /// ```
    pub fn pow_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = self.pow_prec_round_ref_ref(other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the specified
    /// precision and to the nearest value. The [`Float`] on the right-hand side is taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded power is less than, equal to, or
    /// greater than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`PowAssign::pow_assign`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_assign(Float::from(2.5), 5), Less);
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_assign(Float::from(2.5), 20), Less);
    /// assert_eq!(x.to_string(), "15.58846");
    /// ```
    #[inline]
    pub fn pow_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.pow_prec_round_assign(other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the specified
    /// precision and to the nearest value. The [`Float`] on the right-hand side is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded power is less than,
    /// equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_prec_round_assign_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`PowAssign::pow_assign`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_assign_ref(&Float::from(2.5), 5), Less);
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_prec_assign_ref(&Float::from(2.5), 20), Less);
    /// assert_eq!(x.to_string(), "15.58846");
    /// ```
    #[inline]
    pub fn pow_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.pow_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the maximum of the
    /// precisions of the two inputs and with the specified rounding mode. The [`Float`] on the
    /// right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`PowAssign::pow_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_round_assign(Float::from(2.5), Floor), Less);
    /// assert_eq!(x.to_string(), "14.0");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_round_assign(Float::from(2.5), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "16.0");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_round_assign(Float::from(2.5), Nearest), Greater);
    /// assert_eq!(x.to_string(), "16.0");
    /// ```
    pub fn pow_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_assign(other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the maximum of the
    /// precisions of the two inputs and with the specified rounding mode. The [`Float`] on the
    /// right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::pow_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`PowAssign::pow_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_round_assign_ref(&Float::from(2.5), Floor), Less);
    /// assert_eq!(x.to_string(), "14.0");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_round_assign_ref(&Float::from(2.5), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "16.0");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(x.pow_round_assign_ref(&Float::from(2.5), Nearest), Greater);
    /// assert_eq!(x.to_string(), "16.0");
    /// ```
    pub fn pow_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_assign_ref(other, prec, rm)
    }
}

impl Pow<Self> for Float {
    type Output = Self;

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the nearest value. Both
    /// [`Float`]s are taken by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// power is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec`] instead. If
    /// you want both of these things, consider using [`Float::pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(3).pow(Float::from(2.5)).to_string(), "16.0");
    /// assert_eq!(Float::from(10).pow(Float::from(-0.5)).to_string(), "0.3");
    /// ```
    fn pow(self, other: Self) -> Self {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(&other, prec).0
    }
}

impl Pow<&Self> for Float {
    type Output = Self;

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the nearest value. The first
    /// [`Float`] is taken by value and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// power is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec`] instead. If
    /// you want both of these things, consider using [`Float::pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(3).pow(&Float::from(2.5)).to_string(), "16.0");
    /// assert_eq!(Float::from(10).pow(&Float::from(-0.5)).to_string(), "0.3");
    /// ```
    fn pow(self, other: &Self) -> Self {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(other, prec).0
    }
}

impl Pow<Float> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the nearest value. The first
    /// [`Float`] is taken by reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// power is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec`] instead. If
    /// you want both of these things, consider using [`Float::pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(3)).pow(Float::from(2.5)).to_string(), "16.0");
    /// assert_eq!((&Float::from(10)).pow(Float::from(-0.5)).to_string(), "0.3");
    /// ```
    fn pow(self, other: Float) -> Float {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(&other, prec).0
    }
}

impl Pow<&Float> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to a [`Float`] power, rounding the result to the nearest value. Both
    /// [`Float`]s are taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// power is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec`] instead. If
    /// you want both of these things, consider using [`Float::pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(3)).pow(&Float::from(2.5)).to_string(), "16.0");
    /// assert_eq!((&Float::from(10)).pow(&Float::from(-0.5)).to_string(), "0.3");
    /// ```
    fn pow(self, other: &Float) -> Float {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(other, prec).0
    }
}

impl PowAssign<Self> for Float {
    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the nearest value.
    /// The [`Float`] on the right-hand side is taken by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// power is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec`] instead. If
    /// you want both of these things, consider using [`Float::pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3);
    /// x.pow_assign(Float::from(2.5));
    /// assert_eq!(x.to_string(), "16.0");
    /// ```
    fn pow_assign(&mut self, other: Self) {
        let prec = self.significant_bits().max(other.significant_bits());
        *self = self.pow_prec_ref_ref(&other, prec).0;
    }
}

impl PowAssign<&Self> for Float {
    /// Raises a [`Float`] to a [`Float`] power in place, rounding the result to the nearest value.
    /// The [`Float`] on the right-hand side is taken by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the
    /// power is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^y|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::pow_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_prec`] instead. If
    /// you want both of these things, consider using [`Float::pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(3);
    /// x.pow_assign(&Float::from(2.5));
    /// assert_eq!(x.to_string(), "16.0");
    /// ```
    fn pow_assign(&mut self, other: &Self) {
        let prec = self.significant_bits().max(other.significant_bits());
        *self = self.pow_prec_ref_ref(other, prec).0;
    }
}

/// Raises a primitive float to a primitive float power, returning a primitive float.
///
/// The result is correctly rounded to the nearest value, unlike [`f32::powf`] and [`f64::powf`],
/// which are not guaranteed to be correctly rounded.
///
/// $$
/// f(x,y) = x^y+\varepsilon.
/// $$
/// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x^y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^y|\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(x,\pm0.0)=1.0$ for any $x$, even `NaN`
/// - $f(1.0,y)=1.0$ for any $y$, even `NaN`
/// - $f(\text{NaN},y)=f(x,\text{NaN})=\text{NaN}$ otherwise
/// - $f(x,\infty)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
/// - $f(x,-\infty)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
/// - $f(-1.0,\pm\infty)=1.0$
/// - $f(-1.0,y)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
/// - $f(\infty,y)=\infty$ if $y>0$, and $0.0$ if $y<0$
/// - $f(-\infty,y)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and not
///   an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative and not
///   an odd integer
/// - $f(0.0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$
/// - $f(-0.0,y)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an odd
///   integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative and not
///   an odd integer
/// - $f(x,y)=\text{NaN}$ if $x$ is finite and negative and $y$ is finite and not an integer
///
/// If the result overflows, $\pm\infty$ is returned, and if it underflows, $\pm0.0$ is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::pow::primitive_float_pow;
///
/// assert_eq!(
///     NiceFloat(primitive_float_pow(3.0, 2.5)),
///     NiceFloat(15.588457268119896)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_pow(2.0, 0.5)),
///     NiceFloat(1.4142135623730951)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_pow(10.0, -0.5)),
///     NiceFloat(0.31622776601683794)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_pow<T: PrimitiveFloat>(x: T, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(Float::pow_prec, x, y)
}
