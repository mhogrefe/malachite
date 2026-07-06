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

use crate::Float;
use crate::arithmetic::exp::{exp_overflow, exp_underflow, one_neighbor};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, CheckedSqrt, IsPowerOf2, NegAssign, Parity, Pow, PowAssign,
};
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
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
    let (zz, o) = Float::from_integer_prec(z.clone(), z_bits);
    assert_eq!(o, Equal);
    let (y2, o) = pow_general(x, &zz, 2, Nearest, true);
    let (result, oo) = Float::from_float_prec(y2, prec);
    assert_eq!(oo, Equal);
    (result, o)
}

// This is `mpfr_pow_z` from `pow_z.c`, MPFR 4.3.0.
fn pow_integer(x: &Float, z: &Integer, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if *z == 0u32 {
        return Float::from_float_prec_round(Float::ONE, prec, rm);
    }
    if x.is_nan() {
        return (Float::NAN, Equal);
    }
    let z_pos = *z > 0u32;
    let z_odd = z.odd();
    if x.is_infinite() {
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
        let new_exp = z.clone() * Integer::from(ex - 1) + Integer::ONE;
        let base = if sign_negative {
            -Float::one_prec(prec)
        } else {
            Float::one_prec(prec)
        };
        return if new_exp < Float::MIN_EXPONENT {
            pow_underflow(prec, if rm == Nearest { Down } else { rm }, sign_negative)
        } else if new_exp > Float::MAX_EXPONENT {
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
    let (log2_x, _) = x.abs().log_base_2_prec(64);
    let log2_x = f64::rounding_from(&log2_x, Nearest).0;
    let z_f = f64::rounding_from(z, Nearest).0;
    let est = log2_x * z_f;
    let negative = x.is_sign_negative() && z_odd;
    if est > f64::from(Float::MAX_EXPONENT) + 64.0 {
        return pow_overflow(prec, rm, negative);
    }
    if est < f64::from(Float::MIN_EXPONENT) - 64.0 {
        return pow_underflow(prec, if rm == Nearest { Down } else { rm }, negative);
    }
    if z_pos {
        let abs_z = z.unsigned_abs_ref();
        let (result, o) = pow_pos_natural(x, abs_z, prec, rm, true);
        if result.is_zero() {
            // pow_pos_natural only returns zero when the result underflowed.
            let negative = x.is_sign_negative() && z_odd;
            return if rm == Nearest {
                pow_integer_underflow_nearest(x, z, prec)
            } else {
                pow_underflow(prec, rm, negative)
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
            let (t, _) = Float::ONE.div_prec_round_val_ref(x, wprec, rnd1);
            if t.is_infinite() {
                return pow_overflow(prec, rm, t.is_sign_negative());
            }
            let (t, _) = pow_pos_natural(&t, abs_z, wprec, rm, false);
            if t.is_infinite() {
                return pow_overflow(prec, rm, t.is_sign_negative());
            }
            if t.is_zero() {
                if rm == Nearest {
                    return pow_integer_underflow_nearest(x, z, prec);
                }
                let negative = x.is_sign_negative() && z_odd;
                return pow_underflow(prec, rm, negative);
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
    let (tmp, o) = Float::from_natural_prec(a, tmp_prec);
    assert_eq!(o, Equal);
    let (tmp, o) = tmp.shl_prec(b, tmp_prec);
    assert_eq!(o, Equal);
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
            rm = -rm; // invert directed modes; Nearest stays
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
        let (mut t, _) =
            abs_x.ln_prec_round_ref(wprec, if y.is_sign_negative() { Floor } else { Ceiling });
        t.mul_prec_round_assign_ref(y, wprec, Ceiling);
        let exp_t = t.get_exponent().map_or(0, i64::from);
        if let Some(kv) = &k {
            let (ln2, _) = Float::ln_2_prec_round(wprec, Floor);
            let (kf, _) = Float::from_signed_prec(i64::exact_from(kv), wprec);
            let (u, _) = ln2.mul_prec_round(kf, wprec, Floor);
            t.sub_prec_round_assign(u, wprec, Ceiling);
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
        let (mut t, _) = t.exp_prec(wprec);
        let _ = &mut t;
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
            // After a 2^k rescue the computation stays comfortably in range, so a singular
            // result cannot recur (MPFR_ASSERTN(!k_non_zero) in mpfr_pow_general).
            assert!(k.is_none());
            if t.is_zero() {
                // real underflow of |x|^y
                (result, o) = pow_underflow(prec, if rm == Nearest { Down } else { rm }, false);
                break;
            }
            if t.is_infinite() {
                // possible overflow: recompute a lower bound
                let (mut t2, _) = abs_x
                    .ln_prec_round_ref(wprec, if y.is_sign_negative() { Ceiling } else { Floor });
                t2.mul_prec_round_assign_ref(y, wprec, Floor);
                let (t2, _) = t2.exp_prec_round(wprec, Floor);
                if t2.is_infinite() {
                    (result, o) = pow_overflow(prec, rm, false);
                    break;
                }
            }
            // scale by 2^-k with k ~ y*log2|x|
            let (mut kf, _) = abs_x.log_base_2_prec_ref(64);
            kf.mul_prec_assign_ref(y, 64);
            k = Some(Integer::rounding_from(&kf, Nearest).0);
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
    (
        if x.is_sign_negative() {
            -Integer::from(n)
        } else {
            Integer::from(n)
        },
        d,
    )
}

fn float_to_odd_mantissa_and_exponent_natural(x: &Float) -> (Natural, i64) {
    let m = x.significand_ref().unwrap().clone();
    let e = i64::from(x.get_exponent().unwrap()) - i64::exact_from(m.significant_bits());
    let tz = m.trailing_zeros().unwrap();
    (m >> tz, e + i64::exact_from(tz))
}

impl Float {
    // This is `mpfr_pow` from `pow.c`, MPFR 4.3.0.
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
        if !x.is_normal() || !y.is_normal() {
            // pow(x, 0) = 1 for any x, even NaN
            if y.is_zero() {
                return Self::from_float_prec_round(Self::ONE, prec, rm);
            } else if x.is_nan() {
                return (Self::NAN, Equal);
            } else if y.is_nan() {
                // pow(+1, NaN) = 1
                return if *x == 1u32 {
                    Self::from_float_prec_round(Self::ONE, prec, rm)
                } else {
                    (Self::NAN, Equal)
                };
            } else if y.is_infinite() {
                return if x.is_infinite() {
                    if y.is_sign_positive() {
                        (Self::INFINITY, Equal)
                    } else {
                        (Self::ZERO, Equal)
                    }
                } else {
                    let mut cmp = x.partial_cmp_abs(&Self::ONE).unwrap();
                    if y.is_sign_negative() {
                        cmp = cmp.reverse();
                    }
                    match cmp {
                        Greater => (Self::INFINITY, Equal),
                        Less => (Self::ZERO, Equal),
                        Equal => Self::from_float_prec_round(Self::ONE, prec, rm),
                    }
                };
            } else if x.is_infinite() {
                let negative = x.is_sign_negative() && float_odd_integer(y);
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
            // x is zero
            let negative = x.is_sign_negative() && float_odd_integer(y);
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
                let (mut t, _) = x.abs().log_base_2_prec_round_ref(64, Down);
                t.mul_prec_round_assign_ref(y, 64, Down);
                if t >= const { Self::const_from_signed(Float::MAX_EXPONENT as i64) } {
                    let negative = x.is_sign_negative() && float_odd_integer(y);
                    return pow_overflow(prec, rm, negative);
                }
            }
            // early underflow detection: ebound such that |x^y| < 2^ebound
            if if y.is_sign_negative() { ex > 1 } else { ex < 0 } {
                let (mut tmp, _) = Self::from_signed_prec(ex, 64);
                if y.is_sign_negative() {
                    tmp.sub_prec_assign(Self::ONE, 64);
                }
                tmp.mul_prec_round_assign_ref(y, 64, Ceiling);
                let mut ebound = i64::rounding_from(&tmp, Ceiling).0;
                // For y < 0 the bound |x^y| <= 2^((ex - 1) * y) is not strict, so if the product
                // is an exact integer the exponent bound must be bumped to keep |x^y| < 2^ebound
                // (mpfr_nextabove(tmp) in mpfr_pow); otherwise x = 2^(ex - 1) exactly achieves
                // the bound and a representable result would be misreported as underflow.
                if y.is_sign_negative() && tmp == ebound {
                    ebound += 1;
                }
                let lim = i64::from(Self::MIN_EXPONENT) - if rm == Nearest { 2 } else { 1 };
                if ebound <= lim {
                    let negative = x.is_sign_negative() && float_odd_integer(y);
                    return pow_underflow(prec, if rm == Nearest { Down } else { rm }, negative);
                }
            }
        }
        // y a not-too-large integer: use the multiplication-based algorithm
        if y_is_integer && ey <= POW_EXP_THRESHOLD {
            let zi = Integer::rounding_from(y, Nearest).0;
            return pow_integer(x, &zi, prec, rm);
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
        {
            let expx = if cmp_x_1 == Less { 1 - ex } else { ex };
            let logt = i64::exact_from(u64::exact_from(expx.max(1)).ceiling_log_base_2());
            let err = ey + logt;
            if err < -i64::exact_from(prec) - 1 {
                let above = y.is_sign_positive() == (cmp_x_1 == Greater);
                return float_one_plus_tiny(prec, rm, above);
            }
        }
        pow_general(x, y, prec, rm, y_is_integer)
    }
}

impl Float {
    pub fn pow_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(&other, prec, rm)
    }

    pub fn pow_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(other, prec, rm)
    }

    pub fn pow_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(&other, prec, rm)
    }

    pub fn pow_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_ref_ref(&other, prec)
    }

    pub fn pow_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(other, prec, Nearest)
    }

    pub fn pow_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_ref_ref(&other, prec, rm)
    }

    pub fn pow_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_ref_ref(other, prec, rm)
    }

    #[inline]
    pub fn pow_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        self.pow_round_ref_ref(other, rm)
    }

    #[inline]
    pub fn pow_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        self.pow_round_ref_ref(&other, rm)
    }

    #[inline]
    pub fn pow_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_ref_ref(other, prec)
    }

    #[inline]
    pub fn pow_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_ref_ref(&other, prec)
    }

    pub fn pow_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = self.pow_prec_round_ref_ref(&other, prec, rm);
        *self = result;
        o
    }

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

    #[inline]
    pub fn pow_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.pow_prec_round_assign(other, prec, Nearest)
    }

    #[inline]
    pub fn pow_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.pow_prec_round_assign_ref(other, prec, Nearest)
    }

    pub fn pow_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_assign(other, prec, rm)
    }

    pub fn pow_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_round_assign_ref(other, prec, rm)
    }
}

impl Pow<Self> for Float {
    type Output = Self;

    fn pow(self, other: Self) -> Self {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(&other, prec).0
    }
}

impl Pow<&Self> for Float {
    type Output = Self;

    fn pow(self, other: &Self) -> Self {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(other, prec).0
    }
}

impl Pow<Float> for &Float {
    type Output = Float;

    fn pow(self, other: Float) -> Float {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(&other, prec).0
    }
}

impl Pow<&Float> for &Float {
    type Output = Float;

    fn pow(self, other: &Float) -> Float {
        let prec = self.significant_bits().max(other.significant_bits());
        self.pow_prec_ref_ref(other, prec).0
    }
}

impl PowAssign<Self> for Float {
    fn pow_assign(&mut self, other: Self) {
        let prec = self.significant_bits().max(other.significant_bits());
        let (result, _) = self.pow_prec_ref_ref(&other, prec);
        *self = result;
    }
}

impl PowAssign<&Self> for Float {
    fn pow_assign(&mut self, other: &Self) {
        let prec = self.significant_bits().max(other.significant_bits());
        let (result, _) = self.pow_prec_ref_ref(other, prec);
        *self = result;
    }
}
