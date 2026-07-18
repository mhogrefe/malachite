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

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::arithmetic::exp::{exp_overflow, exp_rational_near_one, exp_underflow, one_neighbor};
use crate::arithmetic::ln::ln_1_plus_rational_brackets;
use crate::arithmetic::log_base_2::log_2_rational_brackets;
use crate::arithmetic::round_near_x::float_round_near_x;
use crate::emulate_float_float_to_float_fn;
use crate::emulate_float_to_float_fn;
use crate::{
    Float, float_either_infinity, float_either_zero, float_nan, float_negative_zero,
    floor_and_ceiling,
};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, CheckedLogBase2, CheckedRoot, CheckedSqrt, DivisibleBy, IsPowerOf2,
    NegAssign, Parity, Pow, PowAssign, Square, UnsignedAbs,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, One,
    Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::OrdAbs;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::{BitAccess, BitIterable, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_q::Rational;
use std::mem::swap;

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

// The outcome of `pow_near_one_fast_path`.
enum NearOne {
    // The result was rounded directly from 1 (or -1).
    Rounded(Float, Ordering),
    // The result is close to 1, but its interesting bits land within the output's window: the Ziv
    // loop must run, but should start with this many extra bits of working precision, since the
    // result's significand begins with about this many 0s or 1s after the leading bit. Without the
    // jump start the loop would balloon, recomputing the power ~log(extra) times at growing
    // precisions until the working precision covers the run.
    JumpStart(u64),
    // The fast path does not apply.
    No,
}

// Fast path for x^z when x is so close to +/-1 that the result is very close to +/-1. Writing |x| =
// 1 + d with d nonzero and fld = EXP(d), and sb_z = the bit length of |z| (z != 0, with its sign
// given by `z_negative`), the path engages when fld + sb_z <= -3. Then |d| < 2^fld <= 2^-4 and
// |z||d| < 2^(fld + sb_z) <= 2^-3, and with t = z ln(1 + d):
// - |ln(1 + d)| <= |d|/(1 - |d|) <= (4/3)|d|, so |t| <= (4/3)|z||d| <= 1/6;
// - |e^t - 1| <= |t| + t^2 <= (3/2)|t| for |t| <= 1/2;
// so ||x|^z - 1| = |e^t - 1| <= 2|z||d| < 2^(fld + sb_z + 1), strictly (both |z| < 2^sb_z and |d| <
// 2^fld are strict). This is exactly the error contract of `float_round_near_x` with v = 1 and err
// = -(fld + sb_z).
//
// `float_round_near_x` also requires the exact result not to be representable, which holds whenever
// it succeeds (it requires err > prec + 1): for positive z, the exact (1 + d)^z is a dyadic
// rational whose bits span from its leading 1 down to exactly z*j, where 2^j is the lowest set bit
// of d; since j <= fld and -fld >= err - sb_z > prec + 1 - sb_z, the span exceeds prec + 1 bits, so
// the value is neither representable at prec nor a `Nearest` midpoint. For negative z the exact
// value is not even dyadic (1/(1 + d)^|z| is dyadic only if (1 + d)^|z| is a power of 2, impossible
// for 0 < |d| <= 2^-4).
//
// `negate` is true when the result is negative (x negative and z odd); the rounding is then
// performed on the magnitude with the inverted rounding mode, and the ternary value is reversed.
fn pow_near_one_fast_path(
    x: &Float,
    sb_z: u64,
    z_negative: bool,
    negate: bool,
    prec: u64,
    rm: RoundingMode,
) -> NearOne {
    // `Exact` is left entirely to the callers, so that this path never has to decide exactness.
    if rm == Exact {
        return NearOne::No;
    }
    let ex = i64::from(x.get_exponent().unwrap());
    // |x| must be in [1/2, 2) for x to be near +/-1.
    if ex != 0 && ex != 1 {
        return NearOne::No;
    }
    // d = |x| - 1, exactly (the difference of two dyadic values whose bits span at most
    // significant_bits(x) + 2 positions here).
    let d = x
        .abs()
        .sub_prec_round(Float::ONE, x.significant_bits() + 2, Exact)
        .0;
    if d == 0u32 {
        // |x| = 1 exactly; the callers' loops handle this case exactly and quickly.
        return NearOne::No;
    }
    let fld = i64::from(d.get_exponent().unwrap());
    let Some(shift) = fld.checked_add(i64::exact_from(sb_z)) else {
        return NearOne::No;
    };
    if shift > -3 {
        return NearOne::No;
    }
    let err = u64::exact_from(-shift);
    // |x|^z > 1 iff |x| > 1 and z > 0, or |x| < 1 and z < 0.
    let above = (d > 0u32) != z_negative;
    let rm_abs = if negate { -rm } else { rm };
    if let Some((v, o)) = float_round_near_x(&Float::ONE, err, above, prec, rm_abs) {
        return if negate {
            NearOne::Rounded(-v, o.reverse())
        } else {
            NearOne::Rounded(v, o)
        };
    }
    NearOne::JumpStart(err)
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
    extra_prec: u64,
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
    // `extra_prec` is the near-1 jump start computed by the caller; see `pow_near_one_fast_path`.
    let mut wprec = if cr {
        prec + 3 + size_z + prec.ceiling_log_base_2() + extra_prec
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
                    // The growing regime rounds toward zero (lower bounds), and the callers decide
                    // the overflow boundary exactly before descending here.
                    fail_on_untested_path("pow_pos_natural, overflow");
                }
                // A zero lies toward zero from the true value and an infinity away from it, so the
                // ternary depends on the sign: +0 and -inf are less than the true value, -0 and
                // +inf greater.
                let o = if is_zero == res.is_sign_positive() {
                    Less
                } else {
                    Greater
                };
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
        return (Float::one_prec(prec), Equal);
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
            // exact integer at 64 bits here): a definite overflow. When called from `Float::pow`
            // the entry's early overflow check already caught this; when called from the
            // integer-exponent path of `Float::pow_rational` (which has no such pre-check), this is
            // the first detection.
            pow_overflow(prec, rm, sign_negative)
        } else {
            let sh = i64::exact_from(&(new_exp - Integer::ONE));
            base.shl_prec_round(sh, prec, rm)
        };
    }
    let negative = x.is_sign_negative() && z_odd;
    // Near-1 fast path, checked before the exponent pre-bounds below: for x very close to +/-1,
    // computing the 64-bit log2 estimate is itself expensive (the tiny logarithm must be resolved,
    // which costs as much as the power itself), and in this regime |x^z| lies in (5/6, 6/5), so no
    // overflow or underflow is possible and the pre-bounds are unnecessary.
    let mut jump_extra = 0;
    match pow_near_one_fast_path(
        x,
        z.unsigned_abs_ref().significant_bits(),
        !z_pos,
        negative,
        prec,
        rm,
    ) {
        NearOne::Rounded(v, o) => return (v, o),
        NearOne::JumpStart(extra) => jump_extra = extra,
        NearOne::No => {}
    }
    if jump_extra == 0 {
        // Pre-bound the result exponent: result_exp ~ z * log2|x|. When it is far outside the
        // exponent range (with a wide margin for the estimate's error), report the exception
        // directly instead of letting the exponentiation saturate; this mirrors the role of MPFR's
        // underflow/overflow flags, which malachite does not have, and keeps the Ziv loop from
        // ballooning on saturated values.
        let est = f64::rounding_from(x.abs().log_base_2_prec(64).0, Nearest).0
            * f64::rounding_from(z, Nearest).0;
        if est > const { Float::MAX_EXPONENT as f64 + 64.0 } {
            // est > MAX_EXPONENT + 64: a definite overflow. When called from `Float::pow`, the
            // entry's early overflow check already caught this; when called from the exact-power
            // path of `Float::pow_rational` (which has no such pre-check), this is the first
            // detection.
            return pow_overflow(prec, rm, negative);
        }
        if est < const { Float::MIN_EXPONENT as f64 - 64.0 } {
            return pow_underflow(prec, if rm == Nearest { Down } else { rm }, negative);
        }
        // Within the estimate's error margin of MAX_EXPONENT the overflow question is still open,
        // and it must be decided here: every rounding used by `pow_pos_natural`'s growing regime
        // and by the reciprocal path below decreases the magnitude, so an overflow would saturate
        // at the largest finite value instead of reaching infinity, and the saturated all-ones
        // significand is one that `float_can_round` never certifies -- the Ziv loop would grow
        // forever. (Underflow needs no such decision: magnitude-decreasing rounding turns a true
        // underflow into an exact zero, which the loops detect directly.) The check mirrors the
        // role of MPFR's overflow flag.
        if est >= const { Float::MAX_EXPONENT as f64 - 66.0 }
            && pow_exponent_at_least(x, z, i64::from(Float::MAX_EXPONENT))
        {
            return pow_overflow(prec, rm, negative);
        }
    }
    if z_pos {
        let (result, o) = pow_pos_natural(x, z.unsigned_abs_ref(), prec, rm, true, jump_extra);
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
        let mut wprec = prec + size_z + 3 + prec.ceiling_log_base_2() + jump_extra;
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
                // For |x| < 1 the reciprocal is rounded toward zero, so an overflowing 1/x
                // saturates at the largest finite value rather than reaching infinity (and the
                // exact overflow decision above has already returned in that case); for |x| >= 1 it
                // is at most 1.
                fail_on_untested_path("pow_integer, 1/x overflow");
                return pow_overflow(prec, rm, t.is_sign_negative());
            }
            let t = pow_pos_natural(&t, abs_z, wprec, rm, false, 0).0;
            if t.is_infinite() {
                // The exact overflow decision above bounds |x^z| < 2^MAX_EXPONENT, and the
                // magnitude-decreasing rounding directions keep the computed value below it.
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

// This is `mpfr_pow_ui` (`POW_U`) from `pow_ui.c`, MPFR 4.3.0: x^n for a `u64` n, by binary
// exponentiation with a Ziv loop, falling back to `pow_integer` (`mpfr_pow_z`) on an internal
// overflow or underflow.
fn pow_u(x: Float, n: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // x^0 = 1 for any x, even NaN
    if n == 0 {
        return (Float::one_prec(prec), Equal);
    }
    if x.is_nan() {
        return (Float::NAN, Equal);
    }
    if x.is_infinite() {
        // Inf^n = Inf; (-Inf)^n = Inf for n even, -Inf for n odd
        return (
            if x.is_sign_negative() && n.odd() {
                Float::NEGATIVE_INFINITY
            } else {
                Float::INFINITY
            },
            Equal,
        );
    }
    if x.is_zero() {
        // 0^n = 0 for any n; positive unless x is negative and n is odd
        return (
            if x.is_sign_negative() && n.odd() {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            },
            Equal,
        );
    }
    if n <= 2 {
        return if n == 1 {
            // x^1 = x
            Float::from_float_prec_round(x, prec, rm)
        } else {
            // x^2 = sqr(x)
            x.square_prec_round(prec, rm)
        };
    }
    // n >= 3: square-and-multiply. `nlen` is the bit length of n, so 2^(nlen - 1) <= n < 2^nlen.
    let nlen = n.significant_bits();
    // Multiplications round away from zero (squares round up; their results are non-negative), so
    // that an intermediate overflow or underflow is a true exception rather than rounding noise.
    let rnd1 = if x.is_sign_positive() { Ceiling } else { Floor };
    let mut wprec = {
        let p = prec + 67 + prec.ceiling_log_base_2();
        if p <= nlen {
            // Unreachable for a `u64` n: p >= 1 + 3 + 64 = 68 always exceeds nlen, which is at most
            // 64. (In MPFR, where GMP_NUMB_BITS may be 32 and n may be wider, this clamp matters.)
            fail_on_untested_path("pow_u, working precision clamped up to nlen + 1");
            nlen + 1
        } else {
            p
        }
    };
    match pow_near_one_fast_path(&x, nlen, false, x.is_sign_negative() && n.odd(), prec, rm) {
        NearOne::Rounded(v, o) => return (v, o),
        NearOne::JumpStart(extra) => wprec += extra,
        NearOne::No => {}
    }
    loop {
        let err = wprec - 1 - nlen;
        let (mut res, o) = x.square_prec_round_ref(wprec, Ceiling);
        let mut inexact = o != Equal;
        let mut i = nlen;
        if n.get_bit(i - 2) {
            inexact |= res.mul_prec_round_assign_ref(&x, wprec, rnd1) != Equal;
        }
        if i > 2 {
            i -= 3;
            loop {
                if res.is_infinite() || res.is_zero() {
                    break;
                }
                inexact |= res.square_prec_round_assign(wprec, Ceiling) != Equal;
                if n.get_bit(i) {
                    inexact |= res.mul_prec_round_assign_ref(&x, wprec, rnd1) != Equal;
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        // Internal overflow (res is infinite) or underflow (res reached the minimum exponent): the
        // approximation error has not been accounted for, so hand off to `pow_integer`, which
        // handles the exponent range precisely.
        if res.is_infinite() || res.is_zero() || res.get_exponent().unwrap() <= Float::MIN_EXPONENT
        {
            if res.is_zero() {
                // Unreachable: squares round up and multiplications round away from zero, so res is
                // a magnitude over-estimate that never rounds to zero; underflow instead surfaces
                // as the minimum binade, handled by the exponent check above.
                fail_on_untested_path("pow_u, res rounded to zero");
            }
            return x.pow_integer_prec_round(Integer::from(n), prec, rm);
        }
        if !inexact || float_can_round(res.significand_ref().unwrap(), err, prec, rm) {
            return Float::from_float_prec_round(res, prec, rm);
        }
        wprec += wprec >> 1;
    }
}

// This is `mpfr_pow_ui` (`POW_U`) from `pow_ui.c`, MPFR 4.3.0: x^n for a `u64` n, by binary
// exponentiation with a Ziv loop, falling back to `pow_integer` (`mpfr_pow_z`) on an internal
// overflow or underflow.
fn pow_u_ref(x: &Float, n: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // x^0 = 1 for any x, even NaN
    if n == 0 {
        return (Float::one_prec(prec), Equal);
    }
    if x.is_nan() {
        return (Float::NAN, Equal);
    }
    if x.is_infinite() {
        // Inf^n = Inf; (-Inf)^n = Inf for n even, -Inf for n odd
        return (
            if x.is_sign_negative() && n.odd() {
                Float::NEGATIVE_INFINITY
            } else {
                Float::INFINITY
            },
            Equal,
        );
    }
    if x.is_zero() {
        // 0^n = 0 for any n; positive unless x is negative and n is odd
        return (
            if x.is_sign_negative() && n.odd() {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            },
            Equal,
        );
    }
    if n <= 2 {
        return if n == 1 {
            // x^1 = x
            Float::from_float_prec_round_ref(x, prec, rm)
        } else {
            // x^2 = sqr(x)
            x.square_prec_round_ref(prec, rm)
        };
    }
    // n >= 3: square-and-multiply. `nlen` is the bit length of n, so 2^(nlen - 1) <= n < 2^nlen.
    let nlen = n.significant_bits();
    // Multiplications round away from zero (squares round up; their results are non-negative), so
    // that an intermediate overflow or underflow is a true exception rather than rounding noise.
    let rnd1 = if x.is_sign_positive() { Ceiling } else { Floor };
    let mut wprec = {
        let p = prec + 67 + prec.ceiling_log_base_2();
        if p <= nlen {
            // Unreachable for a `u64` n: p >= 1 + 3 + 64 = 68 always exceeds nlen, which is at most
            // 64. (In MPFR, where GMP_NUMB_BITS may be 32 and n may be wider, this clamp matters.)
            fail_on_untested_path("pow_u, working precision clamped up to nlen + 1");
            nlen + 1
        } else {
            p
        }
    };
    match pow_near_one_fast_path(x, nlen, false, x.is_sign_negative() && n.odd(), prec, rm) {
        NearOne::Rounded(v, o) => return (v, o),
        NearOne::JumpStart(extra) => wprec += extra,
        NearOne::No => {}
    }
    loop {
        let err = wprec - 1 - nlen;
        let (mut res, o) = x.square_prec_round_ref(wprec, Ceiling);
        let mut inexact = o != Equal;
        let mut i = nlen;
        if n.get_bit(i - 2) {
            inexact |= res.mul_prec_round_assign_ref(x, wprec, rnd1) != Equal;
        }
        if i > 2 {
            i -= 3;
            loop {
                if res.is_infinite() || res.is_zero() {
                    break;
                }
                inexact |= res.square_prec_round_assign(wprec, Ceiling) != Equal;
                if n.get_bit(i) {
                    inexact |= res.mul_prec_round_assign_ref(x, wprec, rnd1) != Equal;
                }
                if i == 0 {
                    break;
                }
                i -= 1;
            }
        }
        // Internal overflow (res is infinite) or underflow (res reached the minimum exponent): the
        // approximation error has not been accounted for, so hand off to `pow_integer`, which
        // handles the exponent range precisely.
        if res.is_infinite() || res.is_zero() || res.get_exponent().unwrap() <= Float::MIN_EXPONENT
        {
            if res.is_zero() {
                // Unreachable: squares round up and multiplications round away from zero, so res is
                // a magnitude over-estimate that never rounds to zero; underflow instead surfaces
                // as the minimum binade, handled by the exponent check above.
                fail_on_untested_path("pow_u, res rounded to zero");
            }
            return x.pow_integer_prec_round_ref_val(Integer::from(n), prec, rm);
        }
        if !inexact || float_can_round(res.significand_ref().unwrap(), err, prec, rm) {
            return Float::from_float_prec_round(res, prec, rm);
        }
        wprec += wprec >> 1;
    }
}

// This is `mpfr_pow_si` (`POW_S`) from `pow_si.c`, MPFR 4.3.0: x^n for an `i64` n. For n >= 0 it is
// `pow_u` (`mpfr_pow_ui`); for n < 0, x^n = (1/x)^|n| is computed by `pow_integer` (`mpfr_pow_z`),
// whose negative-exponent path is exactly what `mpfr_pow_si` inlines.
fn pow_s(x: Float, n: i64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if n >= 0 {
        pow_u(x, n.unsigned_abs(), prec, rm)
    } else {
        x.pow_integer_prec_round(Integer::from(n), prec, rm)
    }
}

fn pow_s_ref(x: &Float, n: i64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if n >= 0 {
        pow_u_ref(x, n.unsigned_abs(), prec, rm)
    } else {
        x.pow_integer_prec_round_ref_val(Integer::from(n), prec, rm)
    }
}

// This is `mpfr_ui_pow_ui` from `ui_pow_ui.c`, MPFR 4.3.0: k^n for `u64` k and n, as a Float, by
// binary exponentiation (all roundings up, so the result is a magnitude over-estimate), falling
// back to `pow_integer` (`mpfr_pow_z`) on overflow. Since k, n >= 0 the result never underflows.
//
// The error budget deliberately deviates from MPFR, whose accounting (one rounding for the initial
// value plus one per squaring, size_n in all) undercounts: the initial rounding of k is amplified
// to the n-th power through the squarings, and the multiplications contribute up to size_n - 1 more
// factors, for at most 2n - 1 < 2^(size_n + 1) Higham factors in all -- a relative error below
// 2^(size_n + 2 - wprec), so size_n + 2 bits are reserved. With MPFR's budget the `float_can_round`
// gate certifies wrongly rounded results at small precisions (upstream mpfr_ui_pow_ui reproduces
// this: 263^15 at precision 1 under `Nearest` returns 2^121 though the true value lies below the
// tie 1.5 * 2^120, and 205^63 at precision 4 under `Down` returns a value above the true one).
fn unsigned_pow_unsigned(k: u64, n: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if n == 0 {
        // k^0 = 1 for any k
        return (Float::one_prec(prec), Equal);
    } else if n == 1 || k <= 1 {
        // k^1 = k; 1^n = 1 and 0^n = 0 for n >= 1; either way the value is k
        return Float::from_unsigned_prec_round(k, prec, rm);
    }
    // k >= 2, n >= 2. `size_n` is the bit length of n, so 2^(size_n - 1) <= n < 2^size_n.
    let size_n = n.significant_bits();
    // k as an exact Float, for the multiplications.
    let kf = Float::from(k);
    let mut wprec = prec + 5 + size_n;
    loop {
        // res starts as k (rounded up), contributing the most significant bit of n.
        let (mut res, o) = Float::from_unsigned_prec_round(k, wprec, Ceiling);
        let mut inexact = o != Equal;
        // err counts the roundings: 1 for the initial value, plus one per squaring.
        for bit in n.bits().rev().skip(1) {
            inexact |= res.square_prec_round_assign(wprec, Ceiling) != Equal;
            if bit {
                inexact |= res.mul_prec_round_assign_ref(&kf, wprec, Ceiling) != Equal;
            }
        }
        if res.is_infinite() {
            // Overflow: the approximation error has not been accounted for, so hand off to
            // `pow_integer`, which handles the exponent range precisely.
            return kf.pow_integer_prec_round(Integer::from(n), prec, rm);
        }
        if !inexact || float_can_round(res.significand_ref().unwrap(), wprec - size_n - 2, prec, rm)
        {
            return Float::from_float_prec_round(res, prec, rm);
        }
        wprec += wprec >> 1;
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

// Resolves |x|^y when the true product y * ln|x| lies at or below the bottom of the Float exponent
// range. In that regime the Ziv loop's Ceiling-rounded product either underflows to -0.0 (making
// exp return exactly 1, whose all-zero error window `float_can_round` can never certify -- an
// infinite loop) or saturates at the minimum positive value (an overestimate whose error the loop's
// budget does not account for, letting it certify a wrongly rounded result near the `Nearest` tie).
// MPFR computes the product in an extended exponent range; malachite has none, so the tiny-product
// case is resolved in exact Rational arithmetic, which has no exponent range at all.
//
// The true result is 1 + delta with 0 < |delta| <= 2^(MIN_EXPONENT + 1). Exact dyadic results --
// including `Nearest` ties, which are dyadic -- are delegated to `pow_is_exact`; the remaining
// values are irrationals strictly between any rounding boundaries, so bracketing exp(t) between the
// exact Rationals 1 + t_lo and 1 + t_hi + t_hi^2 (valid for |t| <= 1/2) and widening the ln|x|
// brackets Ziv-style always terminates. For |x| within a sliver of 1, ln|x| is bracketed by the
// exact atanh-series helper -- a direct `ln` would need working precision on the order of the
// sliver's depth (up to ~2^30 bits) to survive the cancellation.
fn pow_general_tiny_product(
    abs_x: &Float,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // y is never an integer here: the entry's sliver-of-one guard keeps |ln|x|| >= 2^(MIN_EXPONENT
    // + 8), so an integer y (with |y| >= 1) cannot make the product underflow.
    if let Some(result) = pow_is_exact(abs_x, y, prec, rm) {
        return result;
    }
    let yr = Rational::exact_from(y);
    let y_pos = *y > 0u32;
    // Classify |x| as near 1 or not with a cheap low-precision subtraction; near the threshold
    // either branch is correct, so the classification need not be exact.
    let near_one = abs_x
        .sub_prec_ref_val(Float::ONE, 64)
        .0
        .get_exponent()
        .unwrap()
        < -8;
    let e = if near_one {
        Some(Rational::exact_from(abs_x) - Rational::ONE)
    } else {
        None
    };
    let mut wp = 128;
    loop {
        // ln_lo <= ln|x| <= ln_hi, as exact Rationals
        let (ln_lo, ln_hi) = if let Some(e) = &e {
            ln_1_plus_rational_brackets(e, wp)
        } else {
            (
                Rational::exact_from(abs_x.ln_prec_round_ref(wp, Floor).0),
                Rational::exact_from(abs_x.ln_prec_round_ref(wp, Ceiling).0),
            )
        };
        // t_lo <= y ln|x| <= t_hi
        let (t_lo, t_hi) = if y_pos {
            (&yr * ln_lo, &yr * ln_hi)
        } else {
            (&yr * ln_hi, &yr * ln_lo)
        };
        // 1 + t <= exp(t) <= 1 + t + t^2 for |t| <= 1/2
        let lower = Rational::ONE + &t_lo;
        let upper = Rational::ONE + &t_hi + (&t_hi).square();
        let (p_lo, mut o_lo) = Float::from_rational_prec_round(lower, prec, rm);
        let (p_hi, mut o_hi) = Float::from_rational_prec_round(upper, prec, rm);
        // A bracket end landing exactly on a representable value rounds with `Equal`; the true
        // value lies strictly between the ends, so the other end's ordering is the true one.
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        // `lower` and `upper` are positive Rationals near 1 (the result is `1 + tiny`), so
        // `from_rational_prec_round` yields a positive value at precision `prec`, never `NaN` or
        // `-0.0`, and a plain value comparison suffices.
        if o_lo == o_hi && p_lo == p_hi {
            return (p_lo, o_lo);
        }
        wp <<= 1;
    }
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
    // Pre-detect a product y * ln|x| below the exponent range, without first computing ln|x| at
    // working precision: for |x| within a deep sliver of 1 that ln costs on the order of |log2(|x|
    // - 1)| bits of internal precision (up to ~2^30) only for the product to underflow anyway. The
    // exponent estimate errs on the side of not firing; the in-loop detection below is the
    // backstop.
    let ey = i64::from(y.get_exponent().unwrap());
    let d = abs_x.sub_prec_ref_val(Float::ONE, 64).0;
    let d_exp = i64::from(d.get_exponent().unwrap());
    // the exponent of ln|x|, within ~1: for |x| near 1, ln|x| ~ |x| - 1; otherwise |ln|x|| > 2^-9
    // and a 64-bit ln suffices
    let ln_exp = if d_exp < -8 {
        d_exp
    } else {
        i64::from(abs_x.ln_prec_round_ref(64, Floor).0.get_exponent().unwrap())
    };
    // Product exponents add within 1 (exp(a * b) is exp(a) + exp(b) or one less), and ln_exp itself
    // is accurate within ~1, so trigger with a couple of binades of margin. Over-triggering is
    // harmless: the resolver is correct for any small product, and for the borderline
    // (bottom-binade but representable) products the x involved is deep within a near-sliver of 1,
    // where the loop's `ln` would need catastrophic working precision anyway.
    if ey.saturating_add(ln_exp) <= i64::from(Float::MIN_EXPONENT) + 2 {
        let (mut result, mut o) = pow_general_tiny_product(&abs_x, y, prec, rm);
        if neg_result {
            result.neg_assign();
            o = o.reverse();
        }
        return (result, o);
    }
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
        // A product below the exponent range comes back as -0.0 (negative underflow) or saturated
        // at the minimum positive value (positive underflow); both derail the loop, so resolve them
        // exactly. (A genuine product equal to the minimum positive value takes this path too,
        // harmlessly.)
        if k.is_none()
            && (t.is_zero()
                || (t.get_exponent() == Some(Float::MIN_EXPONENT) && raw_power_of_2(&t)))
        {
            (result, o) = pow_general_tiny_product(&abs_x, y, prec, rm);
            break;
        }
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
        t.exp_prec_assign(wprec);
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
            wprec.checked_sub(u64::saturating_from(err)).unwrap_or(1),
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

// Decides exactly whether z * log2|x| >= bound -- equivalently, whether |x|^z >= 2^bound -- for a
// finite nonzero x that is not a power of 2 and a nonzero z. Writing |x| = a * 2^b with a odd (and
// a >= 3, since x is not a power of 2), log2|x| = b + log2(a), and log2(a) is bracketed between
// exact Rationals at widening precision. log2(a) is irrational, so z * (b + log2(a)) never equals
// the integer bound and the comparison always resolves.
fn pow_exponent_at_least(x: &Float, z: &Integer, bound: i64) -> bool {
    let (a, b) = float_to_odd_mantissa_and_exponent_natural(&x.abs());
    debug_assert!(a > 1u32);
    let ar = Rational::from(a);
    let zr = Rational::from(z);
    let br = Rational::from(b);
    let bound_r = Rational::from(bound);
    let z_pos = *z > 0u32;
    let mut wprec = 128;
    loop {
        let (l_lo, l_hi) = log_2_rational_brackets(&ar, wprec);
        let (t_lo, t_hi) = if z_pos {
            (&zr * (&br + l_lo), &zr * (&br + l_hi))
        } else {
            (&zr * (&br + l_hi), &zr * (&br + l_lo))
        };
        if t_lo >= bound_r {
            return true;
        }
        if t_hi < bound_r {
            return false;
        }
        wprec <<= 1;
    }
}

// If `|x|` is a sliver of 1 -- within a couple of binades of the smallest positive `Float`, where
// `ln|x|` falls below the smallest positive `Float` -- returns `x`'s exact `Rational` value, and
// otherwise `None`. Only a `Float` in `(1/2, 2)` with a precision near `2^30` can be a sliver, so
// the exact `Rational` (which occupies ~128 MB) is built only past the cheap exponent and precision
// tests.
fn float_sliver_of_one(x: &Float) -> Option<Rational> {
    let ex = i64::from(x.get_exponent().unwrap());
    if (ex == 0 || ex == 1)
        && x.get_prec().unwrap() >= u64::exact_from(-i64::from(Float::MIN_EXPONENT) - 8)
    {
        let xr = Rational::exact_from(x);
        let d = (&xr).abs() - Rational::ONE;
        if d != 0u32 && d.floor_log_base_2_abs() < i64::from(Float::MIN_EXPONENT) + 8 {
            return Some(xr);
        }
    }
    None
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
                return (Self::one_prec(prec), Equal);
            }
            (float_nan!(), _) => return (Self::NAN, Equal),
            // pow(+1, NaN) = 1
            (_, float_nan!()) => {
                return if *x == 1u32 {
                    (Self::one_prec(prec), Equal)
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
                    Equal => (Self::one_prec(prec), Equal),
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
        // When |x| is a sliver of 1 -- within a couple of binades of the smallest positive Float --
        // ln|x| falls below the smallest positive Float, so every Float-based route below (the
        // early over/underflow bounds, `pow_general`) would underflow it and lose the precision
        // needed for y * ln|x| (which can still be an ordinary, even overflowing, value). Delegate
        // to the exact-Rational power, which brackets log2 with the atanh series over `Rational`s
        // and never materializes a sub-`MIN_EXPONENT` Float logarithm. Only huge-precision Floats
        // in (1/2, 2) can be slivers, so the exact Rational is built only past those cheap tests.
        if let Some(xr) = float_sliver_of_one(x) {
            return Self::rational_pow_prec_round_val_ref(xr, y, prec, rm);
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
                if t >= const { Self::const_from_signed(Self::MAX_EXPONENT as SignedLimb) } {
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
    #[allow(clippy::needless_pass_by_value)]
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
    #[inline]
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
    #[inline]
    pub fn pow_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(other, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
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
    #[inline]
    pub fn pow_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(&other, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
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
    #[inline]
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
    #[inline]
    pub fn pow_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.pow_prec_round_ref_ref(other, prec, Nearest)
    }

    #[allow(clippy::needless_pass_by_value)]
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

    #[allow(clippy::needless_pass_by_value)]
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

    #[allow(clippy::needless_pass_by_value)]
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

    #[allow(clippy::needless_pass_by_value)]
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
    /// assert_eq!(
    ///     x.pow_prec_round_assign(Float::from(2.5), 5, Ceiling),
    ///     Greater
    /// );
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
    /// assert_eq!(
    ///     x.pow_prec_round_assign(Float::from(2.5), 20, Ceiling),
    ///     Greater
    /// );
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
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&Float::from(2.5), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&Float::from(2.5), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "16.0");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&Float::from(2.5), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "15.5");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&Float::from(2.5), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "15.58846");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&Float::from(2.5), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "15.58847");
    ///
    /// let mut x = Float::from(3);
    /// assert_eq!(
    ///     x.pow_prec_round_assign_ref(&Float::from(2.5), 20, Nearest),
    ///     Less
    /// );
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
    /// assert_eq!(
    ///     (&Float::from(10)).pow(&Float::from(-0.5)).to_string(),
    ///     "0.3"
    /// );
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

// Represents an `Integer` exactly as a `Float`, at just enough precision. Routes a `Float ^
// Integer` power through the `Float ^ Float` power, which dispatches to `pow_integer`.
fn integer_to_exact_float(z: Integer) -> Float {
    let prec = z.significant_bits().max(1);
    Float::from_integer_prec_round(z, prec, Exact).0
}

impl Float {
    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and with the specified rounding mode. Both are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec_round(Integer::from(5), 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec_round(Integer::from(-2), 10, Ceiling);
    /// assert_eq!(p.to_string(), "0.1112");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round(
        self,
        other: Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round(integer_to_exact_float(other), prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value and the
    /// [`Integer`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec_round_val_ref(&Integer::from(5), 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec_round_val_ref(&Integer::from(-2), 10, Ceiling);
    /// assert_eq!(p.to_string(), "0.1112");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_val_ref(
        self,
        other: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round(integer_to_exact_float(other.clone()), prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference and the
    /// [`Integer`] by value. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_prec_round_ref_val(Integer::from(5), 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let x = Float::from(3);
    /// let (p, o) = (&x).pow_integer_prec_round_ref_val(Integer::from(-2), 10, Ceiling);
    /// assert_eq!(p.to_string(), "0.1112");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_ref_val(
        &self,
        other: Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_val(integer_to_exact_float(other), prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and with the specified rounding mode. Both are taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,n)=1.0$
    /// - $f(\text{NaN},n)=\text{NaN}$ if $n \neq 0$
    /// - $f(-1.0,n)=1.0$ if $n$ is even, and $-1.0$ if $n$ is odd
    /// - $f(\infty,n)=\infty$ if $n>0$, and $0.0$ if $n<0$
    /// - $f(-\infty,n)=-\infty$ if $n$ is positive and odd, $\infty$ if $n$ is positive and even,
    ///   $-0.0$ if $n$ is negative and odd, and $0.0$ if $n$ is negative and even
    /// - $f(0.0,n)=0.0$ if $n>0$, and $\infty$ if $n<0$
    /// - $f(-0.0,n)=-0.0$ if $n$ is positive and odd, $0.0$ if $n$ is positive and even, $-\infty$
    ///   if $n$ is negative and odd, and $\infty$ if $n$ is negative and even
    ///
    /// Overflow and underflow:
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,n,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd $n$) mirror the bullets above, with the
    ///   rounding directions reflected.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_prec_round_ref_ref(&Integer::from(5), 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let x = Float::from(3);
    /// let (p, o) = (&x).pow_integer_prec_round_ref_ref(&Integer::from(-2), 10, Ceiling);
    /// assert_eq!(p.to_string(), "0.1112");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_ref_ref(
        &self,
        other: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.pow_prec_round_ref_val(integer_to_exact_float(other.clone()), prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and to the nearest value. Both are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_integer_prec_round`] instead.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec(Integer::from(5), 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec(Integer::from(-2), 10);
    /// assert_eq!(p.to_string(), "0.1111");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_integer_prec(self, other: Integer, prec: u64) -> (Self, Ordering) {
        self.pow_integer_prec_round(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and to the nearest value. The [`Float`] is taken by value and the [`Integer`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_integer_prec_round_val_ref`] instead.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec_val_ref(&Integer::from(5), 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_integer_prec_val_ref(&Integer::from(-2), 10);
    /// assert_eq!(p.to_string(), "0.1111");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_integer_prec_val_ref(self, other: &Integer, prec: u64) -> (Self, Ordering) {
        self.pow_integer_prec_round_val_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and to the nearest value. The [`Float`] is taken by reference and the [`Integer`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_integer_prec_round_ref_val`] instead.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_prec_ref_val(Integer::from(5), 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_prec_ref_val(Integer::from(-2), 10);
    /// assert_eq!(p.to_string(), "0.1111");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_integer_prec_ref_val(&self, other: Integer, prec: u64) -> (Self, Ordering) {
        self.pow_integer_prec_round_ref_val(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the specified
    /// precision and to the nearest value. Both are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_integer_prec_round_ref_ref`] instead.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_prec_ref_ref(&Integer::from(5), 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_prec_ref_ref(&Integer::from(-2), 10);
    /// assert_eq!(p.to_string(), "0.1111");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_integer_prec_ref_ref(&self, other: &Integer, prec: u64) -> (Self, Ordering) {
        self.pow_integer_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the precision of
    /// the base and with the specified rounding mode. Both are taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_integer_prec_round`]
    /// instead.
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
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_integer_round(Integer::from(5), Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_integer_round(Integer::from(5), Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_round(self, other: Integer, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_integer_prec_round(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the precision of
    /// the base and with the specified rounding mode. The [`Float`] is taken by value and the
    /// [`Integer`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::pow_integer_prec_round_val_ref`] instead.
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
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_integer_round_val_ref(&Integer::from(5), Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_integer_round_val_ref(&Integer::from(5), Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_round_val_ref(self, other: &Integer, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_integer_prec_round_val_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the precision of
    /// the base and with the specified rounding mode. The [`Float`] is taken by reference and the
    /// [`Integer`] by value. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::pow_integer_prec_round_ref_val`] instead.
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
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_round_ref_val(Integer::from(5), Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_round_ref_val(Integer::from(5), Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_round_ref_val(&self, other: Integer, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_integer_prec_round_ref_val(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the precision of
    /// the base and with the specified rounding mode. Both are taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::pow_integer_prec_round_ref_ref`] instead.
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
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_round_ref_ref(&Integer::from(5), Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_integer_round_ref_ref(&Integer::from(5), Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_integer_round_ref_ref(&self, other: &Integer, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_integer_prec_round_ref_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by value.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_integer_prec_round_assign(Integer::from(5), 20, Floor);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_assign(
        &mut self,
        other: Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.pow_prec_round_assign(integer_to_exact_float(other), prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by
    /// reference.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_integer_prec_round_assign_ref(&Integer::from(5), 20, Floor);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn pow_integer_prec_round_assign_ref(
        &mut self,
        other: &Integer,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.pow_prec_round_assign(integer_to_exact_float(other.clone()), prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by value.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_integer_prec_assign(Integer::from(5), 20);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn pow_integer_prec_assign(&mut self, other: Integer, prec: u64) -> Ordering {
        self.pow_prec_assign(integer_to_exact_float(other), prec)
    }

    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by
    /// reference.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_integer_prec_assign_ref(&Integer::from(5), 20);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn pow_integer_prec_assign_ref(&mut self, other: &Integer, prec: u64) -> Ordering {
        self.pow_prec_assign(integer_to_exact_float(other.clone()), prec)
    }

    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by value.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_integer_round_assign(Integer::from(5), Floor);
    /// assert_eq!(x.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_integer_round_assign(&mut self, other: Integer, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.pow_prec_round_assign(integer_to_exact_float(other), prec, rm)
    }

    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by
    /// reference.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
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
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::integer::Integer;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_integer_round_assign_ref(&Integer::from(5), Floor);
    /// assert_eq!(x.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    /// ```
    pub fn pow_integer_round_assign_ref(&mut self, other: &Integer, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.pow_prec_round_assign(integer_to_exact_float(other.clone()), prec, rm)
    }
}

impl Pow<Integer> for Float {
    type Output = Self;

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the nearest value.
    /// Both are taken by value.
    ///
    /// The output precision is the precision of the base. If the power is equidistant from two
    /// [`Float`]s with that precision, the [`Float`] with fewer 1s in its binary expansion is
    /// chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$, where $p$ is the precision of the base.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_integer_prec`]
    /// instead. If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::pow_integer_prec_round`] instead.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Float::from(2).pow(Integer::from(10)).to_string(), "1.0e3");
    /// assert_eq!(Float::from(2).pow(Integer::from(-3)).to_string(), "0.1");
    /// ```
    #[inline]
    fn pow(self, other: Integer) -> Self {
        let prec = self.significant_bits();
        self.pow_integer_prec(other, prec).0
    }
}

impl Pow<&Integer> for Float {
    type Output = Self;

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the nearest value.
    /// The [`Float`] is taken by value and the [`Integer`] by reference.
    ///
    /// The output precision is the precision of the base. If the power is equidistant from two
    /// [`Float`]s with that precision, the [`Float`] with fewer 1s in its binary expansion is
    /// chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$, where $p$ is the precision of the base.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_integer_prec`]
    /// instead. If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::pow_integer_prec_round`] instead.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Float::from(2).pow(&Integer::from(10)).to_string(), "1.0e3");
    /// assert_eq!(Float::from(2).pow(&Integer::from(-3)).to_string(), "0.1");
    /// ```
    #[inline]
    fn pow(self, other: &Integer) -> Self {
        let prec = self.significant_bits();
        self.pow_integer_prec_val_ref(other, prec).0
    }
}

impl Pow<Integer> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the nearest value.
    /// The [`Float`] is taken by reference and the [`Integer`] by value.
    ///
    /// The output precision is the precision of the base. If the power is equidistant from two
    /// [`Float`]s with that precision, the [`Float`] with fewer 1s in its binary expansion is
    /// chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$, where $p$ is the precision of the base.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_integer_prec`]
    /// instead. If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::pow_integer_prec_round`] instead.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Float::from(2)).pow(Integer::from(10)).to_string(),
    ///     "1.0e3"
    /// );
    /// assert_eq!((&Float::from(2)).pow(Integer::from(-3)).to_string(), "0.1");
    /// ```
    #[inline]
    fn pow(self, other: Integer) -> Float {
        let prec = self.significant_bits();
        self.pow_integer_prec_ref_val(other, prec).0
    }
}

impl Pow<&Integer> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to the power of an [`Integer`], rounding the result to the nearest value.
    /// Both are taken by reference.
    ///
    /// The output precision is the precision of the base. If the power is equidistant from two
    /// [`Float`]s with that precision, the [`Float`] with fewer 1s in its binary expansion is
    /// chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$, where $p$ is the precision of the base.
    ///
    /// See the [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_integer_prec`]
    /// instead. If you want to specify the output precision and the rounding mode, consider using
    /// [`Float::pow_integer_prec_round`] instead.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Float::from(2)).pow(&Integer::from(10)).to_string(),
    ///     "1.0e3"
    /// );
    /// assert_eq!((&Float::from(2)).pow(&Integer::from(-3)).to_string(), "0.1");
    /// ```
    #[inline]
    fn pow(self, other: &Integer) -> Float {
        let prec = self.significant_bits();
        self.pow_integer_prec_ref_ref(other, prec).0
    }
}

impl PowAssign<Integer> for Float {
    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by value,
    /// and rounding the result to the nearest value.
    ///
    /// The output precision is the precision of the base. See the
    /// [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special cases,
    /// overflow, and underflow.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Float::from(2);
    /// x.pow_assign(Integer::from(10));
    /// assert_eq!(x.to_string(), "1.0e3");
    /// ```
    #[inline]
    fn pow_assign(&mut self, other: Integer) {
        let prec = self.significant_bits();
        self.pow_integer_prec_assign(other, prec);
    }
}

impl PowAssign<&Integer> for Float {
    /// Raises a [`Float`] to the power of an [`Integer`] in place, taking the [`Integer`] by
    /// reference, and rounding the result to the nearest value.
    ///
    /// The output precision is the precision of the base. See the
    /// [`Float::pow_integer_prec_round_ref_ref`] documentation for information on special cases,
    /// overflow, and underflow.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Float::from(2);
    /// x.pow_assign(&Integer::from(10));
    /// assert_eq!(x.to_string(), "1.0e3");
    /// ```
    #[inline]
    fn pow_assign(&mut self, other: &Integer) {
        let prec = self.significant_bits();
        self.pow_integer_prec_assign_ref(other, prec);
    }
}

impl Float {
    /// Raises a [`Float`] to the power of a [`u64`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,n)=1.0$
    /// - $f(\text{NaN},n)=\text{NaN}$ if $n \neq 0$
    /// - $f(-1.0,n)=1.0$ if $n$ is even, and $-1.0$ if $n$ is odd
    /// - $f(\infty,n)=\infty$ if $n>0$
    /// - $f(-\infty,n)=\infty$ if $n$ is positive and even, and $-\infty$ if $n$ is odd
    /// - $f(0.0,n)=0.0$ if $n>0$
    /// - $f(-0.0,n)=0.0$ if $n$ is positive and even, and $-0.0$ if $n$ is odd
    ///
    /// Overflow and underflow:
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,n,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd $n$) mirror the bullets above, with the
    ///   rounding directions reflected.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (p, o) = Float::from(3).pow_u_prec_round(5, 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_u_prec_round(5, 2, Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec_round(self, n: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_u(self, n, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`u64`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (p, o) = (&Float::from(3)).pow_u_prec_round_ref(5, 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = (&Float::from(3)).pow_u_prec_round_ref(5, 2, Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec_round_ref(&self, n: u64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_u_ref(self, n, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`u64`], rounding the result to the specified precision
    /// and to the nearest value. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_u_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_u_prec(5, 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_u_prec(5, 2);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec(self, n: u64, prec: u64) -> (Self, Ordering) {
        pow_u(self, n, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`u64`], rounding the result to the specified precision
    /// and to the nearest value. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_u_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_u_prec_ref(5, 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = (&Float::from(3)).pow_u_prec_ref(5, 2);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_prec_ref(&self, n: u64, prec: u64) -> (Self, Ordering) {
        pow_u_ref(self, n, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`u64`], rounding the result to the precision of the
    /// base and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_u_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_u_round(5, Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_u_round(5, Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_round(self, n: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        pow_u(self, n, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`u64`], rounding the result to the precision of the
    /// base and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_u_prec_round_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_u_round_ref(5, Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_u_round_ref(5, Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_u_round_ref(&self, n: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_u_ref(self, n, self.significant_bits(), rm)
    }

    /// Raises a [`Float`] to the power of a [`u64`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let o = x.pow_u_prec_round_assign(5, 20, Floor);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn pow_u_prec_round_assign(&mut self, n: u64, prec: u64, rm: RoundingMode) -> Ordering {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        let (result, o) = pow_u(x, n, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to the power of a [`u64`] in place, rounding the result to the specified
    /// precision and to the nearest value.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_u_prec_assign(5, 20);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn pow_u_prec_assign(&mut self, n: u64, prec: u64) -> Ordering {
        self.pow_u_prec_round_assign(n, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`u64`] in place, rounding the result to the precision
    /// of the base and with the specified rounding mode.
    ///
    /// See the [`Float::pow_u_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_u_round_assign(5, Floor);
    /// assert_eq!(x.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_u_round_assign(&mut self, n: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.pow_u_prec_round_assign(n, prec, rm)
    }
}

impl Pow<u64> for Float {
    type Output = Self;

    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the nearest value at
    /// the precision of the base. The [`Float`] is taken by value.
    ///
    /// If the power is equidistant from two [`Float`]s with that precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$, where $p$ is the precision of the base.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_s_prec`] instead. If
    /// you want to specify the output precision and the rounding mode, consider using
    /// [`Float::pow_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::from(2).pow(10i64).to_string(), "1.0e3");
    /// assert_eq!(Float::from(0.5).pow(-1i64).to_string(), "2.0");
    /// ```
    #[inline]
    fn pow(self, n: u64) -> Self {
        let prec = self.significant_bits();
        pow_u(self, n, prec, Nearest).0
    }
}

impl Pow<u64> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the nearest value at
    /// the precision of the base. The [`Float`] is taken by reference.
    ///
    /// If the power is equidistant from two [`Float`]s with that precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$, where $p$ is the precision of the base.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_s_prec`] instead. If
    /// you want to specify the output precision and the rounding mode, consider using
    /// [`Float::pow_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::from(2)).pow(10i64).to_string(), "1.0e3");
    /// assert_eq!(Float::from(0.5).pow(-1i64).to_string(), "2.0");
    /// ```
    #[inline]
    fn pow(self, n: u64) -> Float {
        pow_u_ref(self, n, self.significant_bits(), Nearest).0
    }
}

impl PowAssign<u64> for Float {
    /// Raises a [`Float`] to the power of a [`i64`] in place, rounding the result to the nearest
    /// value at the precision of the base.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(2);
    /// x.pow_assign(10i64);
    /// assert_eq!(x.to_string(), "1.0e3");
    /// ```
    #[inline]
    fn pow_assign(&mut self, n: u64) {
        let prec = self.significant_bits();
        self.pow_u_prec_assign(n, prec);
    }
}

impl Float {
    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded power is less than, equal to, or greater than
    /// the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0)=1.0$ for any $x$, even `NaN`
    /// - $f(1.0,n)=1.0$
    /// - $f(\text{NaN},n)=\text{NaN}$ if $n \neq 0$
    /// - $f(-1.0,n)=1.0$ if $n$ is even, and $-1.0$ if $n$ is odd
    /// - $f(\infty,n)=\infty$ if $n>0$, and $0.0$ if $n<0$
    /// - $f(-\infty,n)=\infty$ if $n$ is positive and even, $-\infty$ if $n$ is positive and odd,
    ///   $0.0$ if $n$ is negative and even, and $-0.0$ if $n$ is negative and odd
    /// - $f(0.0,n)=0.0$ if $n>0$, and $\infty$ if $n<0$
    /// - $f(-0.0,n)=0.0$ if $n$ is positive and even, $-0.0$ if $n$ is positive and odd, $\infty$
    ///   if $n$ is negative and even, and $-\infty$ if $n$ is negative and odd
    ///
    /// Overflow and underflow:
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,n,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,n,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,n,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - Negative results (from negative $x$ and odd $n$) mirror the bullets above, with the
    ///   rounding directions reflected.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (p, o) = Float::from(3).pow_s_prec_round(5, 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_s_prec_round(-2, 10, Ceiling);
    /// assert_eq!(p.to_string(), "0.1112");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_prec_round(self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_s(self, n, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (p, o) = (&Float::from(3)).pow_s_prec_round_ref(5, 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = (&Float::from(3)).pow_s_prec_round_ref(-2, 10, Ceiling);
    /// assert_eq!(p.to_string(), "0.1112");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_prec_round_ref(&self, n: i64, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        pow_s_ref(self, n, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the specified precision
    /// and to the nearest value. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded power is less than, equal to, or greater than the exact
    /// power. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_s_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_s_prec(5, 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::from(3).pow_s_prec(-2, 10);
    /// assert_eq!(p.to_string(), "0.1111");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_s_prec(self, n: i64, prec: u64) -> (Self, Ordering) {
        pow_s(self, n, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the specified precision
    /// and to the nearest value. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,n,p) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::pow_s_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_s_prec_ref(5, 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = (&Float::from(3)).pow_s_prec_ref(-2, 10);
    /// assert_eq!(p.to_string(), "0.1111");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_s_prec_ref(&self, n: i64, prec: u64) -> (Self, Ordering) {
        pow_s_ref(self, n, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the precision of the
    /// base and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded power is less than, equal to, or greater
    /// than the exact power. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_s_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).pow_s_round(5, Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).pow_s_round(5, Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_round(self, n: i64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        pow_s(self, n, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`i64`], rounding the result to the precision of the
    /// base and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// $$
    /// f(x,n,p,m) = x^n+\varepsilon.
    /// $$
    /// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^n$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p+1}$.
    /// - If $x^n$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^n|\rfloor-p}$.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::pow_s_prec_round_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = (&Float::from(3)).pow_s_round_ref(5, Floor);
    /// assert_eq!(p.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = (&Float::from(3)).pow_s_round_ref(5, Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn pow_s_round_ref(&self, n: i64, rm: RoundingMode) -> (Self, Ordering) {
        pow_s_ref(self, n, self.significant_bits(), rm)
    }

    /// Raises a [`Float`] to the power of a [`i64`] in place, rounding the result to the specified
    /// precision and with the specified rounding mode.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let o = x.pow_s_prec_round_assign(5, 20, Floor);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn pow_s_prec_round_assign(&mut self, n: i64, prec: u64, rm: RoundingMode) -> Ordering {
        let mut x = Self::ZERO;
        swap(self, &mut x);
        let (result, o) = pow_s(x, n, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to the power of a [`i64`] in place, rounding the result to the specified
    /// precision and to the nearest value.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_s_prec_assign(5, 20);
    /// assert_eq!(x.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn pow_s_prec_assign(&mut self, n: i64, prec: u64) -> Ordering {
        self.pow_s_prec_round_assign(n, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`i64`] in place, rounding the result to the precision
    /// of the base and with the specified rounding mode.
    ///
    /// See the [`Float::pow_s_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(3);
    /// let o = x.pow_s_round_assign(5, Floor);
    /// assert_eq!(x.to_string(), "2.0e2");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn pow_s_round_assign(&mut self, n: i64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.pow_s_prec_round_assign(n, prec, rm)
    }
}

impl Pow<i64> for Float {
    type Output = Self;

    /// Raises a [`Float`] to an [`i64`] power, rounding the result to the nearest value at the
    /// precision of the base. The [`Float`] is taken by value.
    #[inline]
    fn pow(self, n: i64) -> Self {
        let prec = self.significant_bits();
        pow_s(self, n, prec, Nearest).0
    }
}

impl Pow<i64> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to an [`i64`] power, rounding the result to the nearest value at the
    /// precision of the base. The [`Float`] is taken by reference.
    #[inline]
    fn pow(self, n: i64) -> Float {
        pow_s_ref(self, n, self.significant_bits(), Nearest).0
    }
}

impl PowAssign<i64> for Float {
    /// Raises a [`Float`] to an [`i64`] power in place, rounding the result to the nearest value at
    /// the precision of the base.
    #[inline]
    fn pow_assign(&mut self, n: i64) {
        let prec = self.significant_bits();
        self.pow_s_prec_assign(n, prec);
    }
}

impl Float {
    /// Raises a [`u64`] to the power of a [`u64`], returning a [`Float`] rounded to the specified
    /// precision and with the specified rounding mode. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   x^y\rfloor-p+1}$.
    /// - If $x^y$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   x^y\rfloor-p}$.
    ///
    /// The result is always nonnegative, so it never underflows.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1.0$ for any $x$
    /// - $f(0,y,p,m)=0.0$ if $y>0$
    /// - $f(1,y,p,m)=1.0$
    ///
    /// Overflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
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
    /// let (p, o) = Float::unsigned_pow_unsigned_prec_round(3, 5, 20, Floor);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::unsigned_pow_unsigned_prec_round(3, 5, 2, Ceiling);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn unsigned_pow_unsigned_prec_round(
        x: u64,
        y: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        unsigned_pow_unsigned(x, y, prec, rm)
    }

    /// Raises a [`u64`] to the power of a [`u64`], returning a [`Float`] rounded to the specified
    /// precision and to the nearest value. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// See the [`Float::unsigned_pow_unsigned_prec_round`] documentation for information on special
    /// cases and overflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::unsigned_pow_unsigned_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::unsigned_pow_unsigned_prec(3, 5, 20);
    /// assert_eq!(p.to_string(), "243.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::unsigned_pow_unsigned_prec(3, 5, 2);
    /// assert_eq!(p.to_string(), "3.0e2");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn unsigned_pow_unsigned_prec(x: u64, y: u64, prec: u64) -> (Self, Ordering) {
        unsigned_pow_unsigned(x, y, prec, Nearest)
    }

    /// Raises a [`u64`] to the power of a [`Float`], returning a [`Float`] rounded to the specified
    /// precision and with the specified rounding mode. The [`Float`] exponent is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded power is less than, equal to,
    /// or greater than the exact power. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 x^y\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0.0,p,m)=1.0$ for any $x$
    /// - $f(1,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ if $x \neq 1$
    /// - $f(x,\infty,p,m)=\infty$ if $x>1$, and $0.0$ if $x=0$
    /// - $f(x,-\infty,p,m)=0.0$ if $x>1$, and $\infty$ if $x=0$
    /// - $f(0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
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
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
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
    /// let (p, o) = Float::unsigned_pow_prec_round(2, Float::from(0.5), 53, Nearest);
    /// assert_eq!(p.to_string(), "1.4142135623730951");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::unsigned_pow_prec_round(3, Float::from(2.5), 53, Floor);
    /// assert_eq!(p.to_string(), "15.588457268119894");
    /// assert_eq!(o, Less);
    /// ```
    ///
    /// This is equivalent to `mpfr_ui_pow` from `ui_pow.c`, MPFR 4.3.0, which likewise converts the
    /// integer exactly and delegates to `mpfr_pow`.
    #[inline]
    pub fn unsigned_pow_prec_round(
        x: u64,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::from(x).pow_prec_round(y, prec, rm)
    }

    /// Raises a [`u64`] to the power of a [`Float`], returning a [`Float`] rounded to the specified
    /// precision and with the specified rounding mode. The [`Float`] exponent is taken by
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
    ///   2^{\lfloor\log_2 x^y\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// See the [`Float::unsigned_pow_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
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
    /// let (p, o) = Float::unsigned_pow_prec_round_ref(2, &Float::from(0.5), 53, Nearest);
    /// assert_eq!(p.to_string(), "1.4142135623730951");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::unsigned_pow_prec_round_ref(3, &Float::from(2.5), 53, Floor);
    /// assert_eq!(p.to_string(), "15.588457268119894");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_prec_round_ref(
        x: u64,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::from(x).pow_prec_round_val_ref(y, prec, rm)
    }

    /// Raises a [`u64`] to the power of a [`Float`], returning a [`Float`] rounded to the specified
    /// precision and to the nearest value. The [`Float`] exponent is taken by value. An
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
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// See the [`Float::unsigned_pow_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::unsigned_pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::unsigned_pow_prec(2, Float::from(0.5), 53);
    /// assert_eq!(p.to_string(), "1.4142135623730951");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::unsigned_pow_prec(3, Float::from(2.5), 53);
    /// assert_eq!(p.to_string(), "15.588457268119896");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn unsigned_pow_prec(x: u64, y: Self, prec: u64) -> (Self, Ordering) {
        Self::unsigned_pow_prec_round(x, y, prec, Nearest)
    }

    /// Raises a [`u64`] to the power of a [`Float`], returning a [`Float`] rounded to the specified
    /// precision and to the nearest value. The [`Float`] exponent is taken by reference. An
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
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// See the [`Float::unsigned_pow_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::unsigned_pow_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::unsigned_pow_prec_ref(2, &Float::from(0.5), 53);
    /// assert_eq!(p.to_string(), "1.4142135623730951");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::unsigned_pow_prec_ref(3, &Float::from(2.5), 53);
    /// assert_eq!(p.to_string(), "15.588457268119896");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn unsigned_pow_prec_ref(x: u64, y: &Self, prec: u64) -> (Self, Ordering) {
        Self::unsigned_pow_prec_round_ref(x, y, prec, Nearest)
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`Float`] rounded to the
    /// specified precision and with the specified rounding mode. The [`Rational`] exponent is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is zero or infinite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 x^y\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=1.0$ for any $x$
    /// - $f(1,y,p,m)=1.0$ for any $y$
    /// - $f(0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
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
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::unsigned_pow_rational_prec_round(8, Rational::from_signeds(1, 3), 20, Floor);
    /// assert_eq!(p.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) =
    ///     Float::unsigned_pow_rational_prec_round(3, Rational::from_signeds(1, 2), 2, Floor);
    /// assert_eq!(p.to_string(), "1.5");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn unsigned_pow_rational_prec_round(
        x: u64,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        unsigned_pow_rational(x, &y, prec, rm)
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`Float`] rounded to the
    /// specified precision and with the specified rounding mode. The [`Rational`] exponent is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is zero or infinite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 x^y\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// See the [`Float::unsigned_pow_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::unsigned_pow_rational_prec_round_ref(
    ///     8,
    ///     &Rational::from_signeds(1, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::unsigned_pow_rational_prec_round_ref(
    ///     3,
    ///     &Rational::from_signeds(1, 2),
    ///     2,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn unsigned_pow_rational_prec_round_ref(
        x: u64,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        unsigned_pow_rational(x, y, prec, rm)
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`Float`] rounded to the
    /// specified precision and to the nearest value. The [`Rational`] exponent is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded power is less than, equal
    /// to, or greater than the exact power.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is zero or infinite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// See the [`Float::unsigned_pow_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::unsigned_pow_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::unsigned_pow_rational_prec(8, Rational::from_signeds(1, 3), 20);
    /// assert_eq!(p.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::unsigned_pow_rational_prec(3, Rational::from_signeds(1, 2), 53);
    /// assert_eq!(p.to_string(), "1.7320508075688772");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn unsigned_pow_rational_prec(x: u64, y: Rational, prec: u64) -> (Self, Ordering) {
        unsigned_pow_rational(x, &y, prec, Nearest)
    }

    /// Raises a [`u64`] to the power of a [`Rational`], returning a [`Float`] rounded to the
    /// specified precision and to the nearest value. The [`Rational`] exponent is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is zero or infinite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 x^y\rfloor-p}$.
    ///
    /// See the [`Float::unsigned_pow_rational_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::unsigned_pow_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::unsigned_pow_rational_prec_ref(27, &Rational::from_signeds(1, 3), 20);
    /// assert_eq!(p.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (p, o) = Float::unsigned_pow_rational_prec_ref(3, &Rational::from_signeds(1, 2), 53);
    /// assert_eq!(p.to_string(), "1.7320508075688772");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn unsigned_pow_rational_prec_ref(x: u64, y: &Rational, prec: u64) -> (Self, Ordering) {
        unsigned_pow_rational(x, y, prec, Nearest)
    }
}

// k^q for a u64 k and Rational q. Since MPFR has no rational-exponent power, this is not a port:
// the value is 2^(q * log2(k)). Exact-rational results (k a perfect b-th power) and a power-of-2
// base are peeled off first (a Ziv-style squeeze never converges on an exactly-representable
// result); the remaining results are irrational and are bracketed by squeezing 2^(q * log2(k))
// between exact Rationals.
fn unsigned_pow_rational(k: u64, q: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    // Exact rounding: compute with Floor and demand exactness.
    if rm == Exact {
        let (result, o) = unsigned_pow_rational(k, q, prec, Floor);
        assert_eq!(o, Equal, "Inexact unsigned_pow_rational");
        return (result, Equal);
    }
    // k^0 = 1 for any k, even 0; 1^q = 1 for any q
    if *q == 0u32 || k == 1 {
        return (Float::one_prec(prec), Equal);
    }
    // 0^q = 0 for q > 0, and +Inf for q < 0
    if k == 0 {
        return if *q > 0u32 {
            (Float::ZERO, Equal)
        } else {
            (Float::INFINITY, Equal)
        };
    }
    // k = 2^s: k^q = 2^(s * q), and `power_of_2_rational_prec_round` handles all exactness,
    // overflow, and underflow.
    if k.is_power_of_2() {
        return Float::power_of_2_rational_prec_round(
            Rational::from(k.trailing_zeros()) * q,
            prec,
            rm,
        );
    }
    // k = j^b (with q = a / b in lowest terms): k^q = j^a is an exact rational, obtained by raising
    // the exact Float j to the integer power a.
    if let Ok(b) = u64::try_from(q.denominator_ref())
        && let Some(j) = k.checked_root(b)
    {
        let a = Integer::from_sign_and_abs_ref(*q >= 0, q.numerator_ref());
        return Float::from(j).pow_integer_prec_round(a, prec, rm);
    }
    // The remaining results are irrational. When `q` is tiny enough that `k ^ q` is within a few
    // ulps of 1, evaluating it as `2 ^ (q * log2(k))` would compute `log2(k)` to nearly `prec` bits
    // needlessly; a dedicated near-1 path handles that case far more cheaply.
    if let Some(result) = unsigned_pow_rational_near_one(k, q, prec, rm) {
        return result;
    }
    // Otherwise squeeze 2^(q * log2(k)) between exact Rationals. Since k >= 2, log2(k) >= 1, so
    // there is no sub-`MIN_EXPONENT` logarithm to contend with.
    pow_squeeze_t(&Rational::from(k), 0, q, prec, rm)
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

/// Raises a [`Rational`] to a primitive float power, returning a primitive float.
///
/// The result is correctly rounded to the nearest value. Unlike a primitive-float base, a
/// [`Rational`] base may lie outside the primitive float's exponent range or so close to 1 that its
/// logarithm is unrepresentable; both are handled exactly, by working with the base as an exact
/// [`Rational`].
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
/// - $f(x,\pm0.0)=1.0$ for any $x$
/// - $f(1,y)=1.0$ for any $y$, even `NaN`
/// - $f(x,\text{NaN})=\text{NaN}$ otherwise
/// - $f(x,\infty)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
/// - $f(x,-\infty)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
/// - $f(\pm1,\pm\infty)=1.0$
/// - $f(-1,y)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
/// - $f(0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the results
///   take positive signs
/// - $f(x,y)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
///
/// If the result overflows, $\pm\infty$ is returned, and if it underflows, $\pm0.0$ is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::pow::primitive_float_rational_pow;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_rational_pow(
///         &Rational::from_unsigneds(3u32, 2u32),
///         2.5
///     )),
///     NiceFloat(2.7556759606310752)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_rational_pow(
///         &Rational::from_unsigneds(9u32, 4u32),
///         0.5
///     )),
///     NiceFloat(1.5)
/// );
/// assert!(
///     primitive_float_rational_pow::<f64>(&-Rational::from_unsigneds(3u32, 2u32), 0.5).is_nan()
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_rational_pow<T: PrimitiveFloat>(x: &Rational, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|y2, prec| Float::rational_pow_prec_ref_val(x, y2, prec), y)
}

/// Raises a primitive float to a [`Rational`] power, returning a primitive float.
///
/// The result is correctly rounded to the nearest value. Unlike a primitive-float exponent, the
/// exact [`Rational`] exponent selects a definite branch of the power, so results that are exactly
/// representable (such as roots of perfect powers) come out exactly.
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
/// - $f(x,0)=1.0$ for any $x$, even `NaN`
/// - $f(1.0,y)=1.0$
/// - $f(\text{NaN},y)=\text{NaN}$ if $y \neq 0$
/// - $f(x,y)=\text{NaN}$ if $x<0$ and $y$ is not an integer
/// - $f(-1.0,y)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
/// - $f(\infty,y)=\infty$ if $y>0$, and $0.0$ if $y<0$
/// - $f(-\infty,y)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive and not
///   an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is negative and not
///   an odd integer
/// - $f(0.0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$
/// - $f(-0.0,y)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an odd
///   integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative and not
///   an odd integer
///
/// If the result overflows, $\pm\infty$ is returned, and if it underflows, $\pm0.0$ is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::pow::primitive_float_pow_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_pow_rational(
///         4.0,
///         &Rational::from_signeds(1, 2)
///     )),
///     NiceFloat(2.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_pow_rational(
///         2.0,
///         &Rational::from_signeds(3, 2)
///     )),
///     NiceFloat(2.8284271247461903)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_pow_rational(
///         4.0,
///         &Rational::from_signeds(-1, 2)
///     )),
///     NiceFloat(0.5)
/// );
/// assert!(primitive_float_pow_rational::<f64>(-8.0, &Rational::from_signeds(1, 3)).is_nan());
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_pow_rational<T: PrimitiveFloat>(x: T, y: &Rational) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::pow_rational_prec_val_ref(x, y, prec), x)
}

/// Raises a primitive float to the power of an [`Integer`], returning a primitive float.
///
/// The result is correctly rounded to the nearest value. Unlike a primitive-float exponent, an
/// arbitrarily large [`Integer`] exponent is handled exactly.
///
/// $$
/// f(x,n) = x^n+\varepsilon.
/// $$
/// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x^n$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^n|\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(x,0)=1.0$ for any $x$, even `NaN`
/// - $f(1,n)=1.0$
/// - $f(\text{NaN},n)=\text{NaN}$ if $n \neq 0$
/// - $f(-1,n)=1.0$ if $n$ is even, and $-1.0$ if $n$ is odd
/// - $f(\infty,n)=\infty$ if $n>0$, and $0.0$ if $n<0$
/// - $f(-\infty,n)=-\infty$ if $n$ is positive and odd, $\infty$ if $n$ is positive and even,
///   $-0.0$ if $n$ is negative and odd, and $0.0$ if $n$ is negative and even
/// - $f(0.0,n)=0.0$ if $n>0$, and $\infty$ if $n<0$
/// - $f(-0.0,n)=-0.0$ if $n$ is positive and odd, $0.0$ if $n$ is positive and even, $-\infty$ if
///   $n$ is negative and odd, and $\infty$ if $n$ is negative and even
///
/// If the result overflows, $\pm\infty$ is returned, and if it underflows, $\pm0.0$ is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::pow::primitive_float_pow_integer;
/// use malachite_nz::integer::Integer;
///
/// assert_eq!(
///     NiceFloat(primitive_float_pow_integer(3.0, &Integer::from(5))),
///     NiceFloat(243.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_pow_integer(2.0, &Integer::from(-3))),
///     NiceFloat(0.125)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_pow_integer(-2.0, &Integer::from(3))),
///     NiceFloat(-8.0)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_pow_integer<T: PrimitiveFloat>(x: T, y: &Integer) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::pow_integer_prec_val_ref(x, y, prec), x)
}

/// Raises a primitive float to the power of a [`u64`], returning a primitive float.
///
/// The result is correctly rounded to the nearest value.
///
/// $$
/// f(x,n) = x^n+\varepsilon.
/// $$
/// - If $x^n$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x^n$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^n|\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(x,0)=1.0$ for any $x$, even `NaN`
/// - $f(1.0,n)=1.0$
/// - $f(\text{NaN},n)=\text{NaN}$ if $n \neq 0$
/// - $f(-1.0,n)=1.0$ if $n$ is even, and $-1.0$ if $n$ is odd
/// - $f(\infty,n)=\infty$ if $n>0$
/// - $f(-\infty,n)=\infty$ if $n$ is positive and even, and $-\infty$ if $n$ is odd
/// - $f(0.0,n)=0.0$ if $n>0$
/// - $f(-0.0,n)=0.0$ if $n$ is positive and even, and $-0.0$ if $n$ is odd
///
/// If the result overflows, $\pm\infty$ is returned, and if it underflows, $\pm0.0$ is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::pow::primitive_float_pow_u;
///
/// assert_eq!(NiceFloat(primitive_float_pow_u(3.0, 5)), NiceFloat(243.0));
/// assert_eq!(NiceFloat(primitive_float_pow_u(2.0, 10)), NiceFloat(1024.0));
/// assert_eq!(NiceFloat(primitive_float_pow_u(-2.0, 3)), NiceFloat(-8.0));
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_pow_u<T: PrimitiveFloat>(x: T, n: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| x.pow_u_prec(n, prec), x)
}

/// Raises a [`u64`] to the power of a primitive float, returning a primitive float.
///
/// The result is correctly rounded to the nearest value.
///
/// $$
/// f(x,y) = x^y+\varepsilon.
/// $$
/// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x^y$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 x^y\rfloor-p}$, where
///   $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(x,0.0)=1.0$ for any $x$
/// - $f(1,y)=1.0$ for any $y$, even `NaN`
/// - $f(x,\text{NaN})=\text{NaN}$ if $x \neq 1$
/// - $f(x,\infty)=\infty$ if $x>1$, and $0.0$ if $x=0$
/// - $f(x,-\infty)=0.0$ if $x>1$, and $\infty$ if $x=0$
/// - $f(0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$
///
/// If the result overflows, $\infty$ is returned, and if it underflows, $0.0$ is returned.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::pow::primitive_float_unsigned_pow;
///
/// assert_eq!(
///     NiceFloat(primitive_float_unsigned_pow(2, 0.5)),
///     NiceFloat(1.4142135623730951)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_unsigned_pow(3, 2.5)),
///     NiceFloat(15.588457268119896)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_unsigned_pow(2, -1.0)),
///     NiceFloat(0.5)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_unsigned_pow<T: PrimitiveFloat>(x: u64, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|y2, prec| Float::unsigned_pow_prec(x, y2, prec), y)
}

// Brackets of ln(1 + e) for an exact nonzero Rational e with |e| < 1/2, as exact Rationals, to a
// relative accuracy of about 2^-wprec. Uses the atanh series ln(1 + e) = 2 atanh(u) with u = e / (2
// + e) and |u| < 1/3: atanh(u) = sum_{k>=0} u^(2k+1)/(2k+1), whose tail after the term in u^(2k+1)
// is bounded in magnitude by that term times u^2 / (1 - u^2) < that term * 9/8. The partial sum and
// the tail both have the sign of e, so the exact value lies between the partial sum and (partial
// sum + tail). Splits a positive Rational x as x' * 2^g with g the nearest integer to log2(x) and
// x' in [1/sqrt(2), sqrt(2)), so that x' is close to 1 (never near 2, where a Float log would
// collapse).
fn rational_mantissa_nearest_power_of_2(x: &Rational) -> (Rational, i64) {
    let fl = x.floor_log_base_2_abs();
    let mant = x >> fl;
    let g = if (&mant).square() < 2u32 { fl } else { fl + 1 };
    (x >> g, g)
}

// Whether x^y is a dyadic rational (and therefore possibly exactly representable), for a positive
// non-power-of-2 Rational x = (a / b) * 2^e with a, b odd and coprime, and a finite nonzero
// non-singular Float y = c * 2^d with c an odd Integer. If so, returns (m, z, pow) such that x^y =
// m^z * 2^pow with m an odd Natural and z a positive Integer; otherwise returns None. Since x is
// not a power of 2, a Ziv-style squeeze on an exact x^y would never terminate, and a nearest-mode
// tie is possible only in the dyadic case, so this decides when the direct route is required.
fn rational_pow_exact_decomposition(
    a: &Natural,
    b: &Natural,
    e: i64,
    y: &Float,
) -> Option<(Natural, Integer, Integer)> {
    let (c, d) = float_to_odd_mantissa_and_exponent(y);
    let (mut a, mut b) = (a.clone(), b.clone());
    let mut e = Integer::from(e);
    // Descend the negative powers of 2 in the exponent: x must be a perfect 2^|d|-th power.
    if d < 0 {
        for _ in 0..-d {
            if a != 1u32 {
                a = a.checked_sqrt()?;
            }
            if b != 1u32 {
                b = b.checked_sqrt()?;
            }
            if e.odd() {
                return None;
            }
            e >>= 1;
        }
    } else {
        e <<= d;
    }
    // Now x^y = (a / b)^(c * 2^max(d, 0)) * 2^(e * c), with the power of 2 in the exponent already
    // scaled into e. Dyadic requires the denominator (after accounting for c's sign) to be 1.
    let pow = e * &c;
    let m = if c > 0u32 {
        if b != 1u32 {
            return None;
        }
        a
    } else {
        if a != 1u32 {
            return None;
        }
        b
    };
    let mut z = Integer::from(c.unsigned_abs());
    if d > 0 {
        z <<= d;
    }
    Some((m, z, pow))
}

// The in-range squeeze: x is positive, not a dyadic rational, and comfortably within the Float
// exponent range, and y is finite, nonzero, and not a small integer. Brackets x between dyadic
// Floats at growing precision and applies `Float::pow` to both ends, tightening until both ends
// round identically. Since x has an odd prime factor in its denominator, x^y is never exactly
// representable and never a nearest-mode tie, so the squeeze terminates.
fn rational_pow_squeeze_x(
    x: &Rational,
    y: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut wprec = prec.saturating_add(Limb::WIDTH << 1);
    let mut increment = Limb::WIDTH;
    loop {
        let x_lo = Float::from_rational_prec_round_ref(x, wprec, Floor).0;
        let x_hi = Float::from_rational_prec_round_ref(x, wprec, Ceiling).0;
        let (p_lo, mut o_lo) = x_lo.pow_prec_round_val_ref(y, prec, rm);
        let (p_hi, mut o_hi) = x_hi.pow_prec_round_val_ref(y, prec, rm);
        // A bracket end that lands exactly on a representable power rounds with `Equal`; the true
        // value lies strictly between the ends, so the other end's ordering is the true one.
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        // `x` is positive, so `Float::pow` yields a positive value at precision `prec` (or `+inf`
        // on overflow, `+0.0` on underflow), never `NaN` or `-0.0`, and a plain value comparison
        // suffices.
        if o_lo == o_hi && p_lo == p_hi {
            return (p_lo, o_lo);
        }
        wprec += increment;
        increment = wprec >> 1;
    }
}

// The shared rational-exponent squeeze: computes (x' * 2^e)^y for an exact Rational x' whose binary
// logarithm `log_2_rational_brackets` can bracket, an integer e, and an exact Rational exponent y
// (finite and nonzero), assuming the true result is irrational. Brackets t = y * (e + log2(x'))
// between exact Rationals -- Rationals have no exponent range, so no underflow or overflow can
// occur here -- and applies `Float::power_of_2_rational_prec_round` to both ends, which itself
// handles results at or beyond the exponent boundaries, growing the working precision until the
// ends agree. `rational_pow` reaches this in its extreme regime with x' in [1/sqrt(2), sqrt(2));
// `unsigned_pow_rational` reaches it with x' = k and e = 0. Growth past the initial precision is
// rare but constructible: 6^(1 + 2^-300) lies within 2^-300 of the rounding boundary 6.0, so the
// first bracket straddles it at any target precision below ~300. Fast path for `k ^ q` when the
// result is extremely close to 1 (`q` so tiny that `k ^ q = exp(q * ln k)` differs from 1 by at
// most a handful of ulps). The general squeeze in `pow_squeeze_t` evaluates `log2(k)` to about
// `prec` bits, which is wasteful here; instead bracket `ln(k)` between two `Rational`s from a
// single modest-precision `ln(k)` and apply `exp_rational_near_one` to the tiny products `q *
// ln(k)`. Returns `None` when the result is not close enough to 1 for this to help (the caller then
// squeezes). Mirrors `power_of_2_rational_near_one`, replacing the constant `ln(2)` with `ln(k)`.
// `k >= 2` and `q` is a nonzero non-integer, so `k ^ q` is irrational.
fn unsigned_pow_rational_near_one(
    k: u64,
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    // `2 ^ ql <= |q| < 2 ^ (ql + 1)` and `log2(k) < kb <= 2 ^ kbb` (kbb the bit length of kb), so
    // `|q * log2(k)| < 2 ^ (ql + 1 + kbb)`. Take this path only when that bound puts `k ^ q` within
    // roughly a machine word's worth of ulps of 1: then `exp_rational_near_one` converges in O(1)
    // terms and `ln(k)` is needed to only about `prec + t_exp_ub` bits. The `t_exp_ub >= 0` guard
    // also keeps `|q * ln(k)| < 1`, which `exp_rational_near_one` requires.
    let ql = q.floor_log_base_2_abs();
    let kbb = i64::exact_from(k.significant_bits().significant_bits());
    let t_exp_ub = ql + 1 + kbb;
    if t_exp_ub >= 0 || t_exp_ub > -i64::exact_from(prec) + i64::exact_from(Limb::WIDTH) {
        return None;
    }
    // `k > 1`, so `k ^ q > 1` exactly when `q > 0`. Because `q * ln(k)` is tiny, `ln(k)` needs only
    // about `prec + t_exp_ub` bits to separate the two exp brackets at the target precision -- far
    // below `prec`. Start a little above that and let the Ziv loop grow it.
    let above = *q > 0u32;
    let mut working_prec = u64::saturating_from(i64::exact_from(prec) + t_exp_ub) + Limb::WIDTH;
    let mut increment = Limb::WIDTH;
    let kf = Float::from(k);
    loop {
        // `ln_k_lo <= ln(k) <= ln_k_hi`, as exact Rationals, from a single `ln(k)` computation.
        let (ln_k_lo, ln_k_hi) = floor_and_ceiling(kf.ln_prec_round_ref(working_prec, Floor));
        let ln_k_lo = Rational::exact_from(&ln_k_lo);
        let ln_k_hi = Rational::exact_from(&ln_k_hi);
        // `q * ln(k)` lies between these two products (which end is smaller depends on the sign of
        // `q`), and exp is increasing, so `k ^ q` lies between the exps of the two products.
        let (p_lo, p_hi) = if above {
            (q * ln_k_lo, q * ln_k_hi)
        } else {
            (q * ln_k_hi, q * ln_k_lo)
        };
        let (lo, o_lo) = exp_rational_near_one(&p_lo, prec, rm);
        let (hi, o_hi) = exp_rational_near_one(&p_hi, prec, rm);
        if o_lo == o_hi && lo == hi {
            return Some((lo, o_lo));
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

fn pow_squeeze_t(
    xp: &Rational,
    e: i64,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let er = Rational::from(e);
    let mut wprec = prec.saturating_add(Limb::WIDTH << 1);
    let mut increment = Limb::WIDTH;
    loop {
        let (l_lo, l_hi) = log_2_rational_brackets(xp, wprec);
        let (t_lo, t_hi) = if *y > 0u32 {
            (y * (&er + l_lo), y * (&er + l_hi))
        } else {
            (y * (&er + l_hi), y * (&er + l_lo))
        };
        let (p_lo, mut o_lo) = Float::power_of_2_rational_prec_round(t_lo, prec, rm);
        let (p_hi, mut o_hi) = Float::power_of_2_rational_prec_round(t_hi, prec, rm);
        // A bracket end landing exactly on a representable power rounds with `Equal`; the true
        // value lies strictly between the ends, so the other end's ordering is the true one.
        if o_lo == Equal {
            fail_on_untested_path(
                "pow_squeeze_t, lo_eq: exact results (t an integer) are caught by each caller's \
                 exact decomposition before the squeeze, so t is never an integer here; a bracket \
                 end equalling an integer is a measure-zero coincidence of the log brackets",
            );
            o_lo = o_hi;
        }
        if o_hi == Equal {
            fail_on_untested_path(
                "pow_squeeze_t, hi_eq: as lo_eq -- t is never an integer in the squeeze, so a \
                 bracket end equalling one is a measure-zero coincidence",
            );
            o_hi = o_lo;
        }
        // `power_of_2_rational_prec_round` yields a positive value at precision `prec` (or `+inf`
        // on overflow, `+0.0` on underflow), never `NaN` or `-0.0`, so a plain value comparison
        // suffices -- no need for `ComparableFloatRef` to force equal precisions or to make `NaN`s
        // compare equal.
        if o_lo == o_hi && p_lo == p_hi {
            return (p_lo, o_lo);
        }
        wprec += increment;
        increment = wprec >> 1;
    }
}

// The exact-dyadic route: x^y = m^z * 2^pow with m odd. If the result's odd part is small enough to
// affect prec-bit rounding (or to be a nearest-mode tie), materialize it; otherwise the value is
// neither representable nor a tie and the caller may squeeze safely.
fn rational_pow_exact(
    m: &Natural,
    z: &Integer,
    pow: &Integer,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let zu = u64::try_from(z).ok()?;
    // The rejection must use a *lower* bound on the significant bits of m^z: returning `None`
    // asserts that the result is neither representable at `prec` nor a `Nearest` tie (both need at
    // most prec + 2 significant bits), and the caller then squeezes -- which never terminates on a
    // representable value or a tie. Since m >= 2^(sb(m) - 1), m^z >= 2^(z * (sb(m) - 1)), so
    // sb(m^z) >= z * (sb(m) - 1) + 1. (An upper bound like z * sb(m) is unsound here: it
    // overestimates sb(m^z) by up to z - 1 bits, letting exactly-representable results and ties
    // leak into the squeeze.) The materialization below stays cheap: the caller has peeled
    // power-of-2 bases, so m is odd and m >= 3, hence sb(m) >= 2 and any admitted z satisfies z <=
    // z * (sb(m) - 1) <= prec + 1, giving sb(m^z) <= z * sb(m) <= 2 * prec + 2.
    debug_assert!(*m > 1u32 && m.odd());
    let bits_lower = (m.significant_bits() - 1).checked_mul(zu)?.checked_add(1)?;
    if bits_lower > prec + 2 {
        return None;
    }
    let value = m.clone().pow(zu);
    let (result, o) = Float::from_natural_prec_round(value, prec, rm);
    // Scale by 2^pow. An exponent beyond i64 with a prec-bit odd part is a definite overflow or
    // underflow.
    let Ok(shift) = i64::try_from(pow) else {
        return Some(if *pow > 0u32 {
            fail_on_untested_path(
                "rational_pow, ex_pow_overflow: reachable only with a base whose 2-adic \
                 valuation exceeds i64::MAX / prec while its odd part fits in prec + 2 bits -- \
                 simultaneously a ~512-MB base and a ~2^31 precision, beyond practical test \
                 sizes",
            );
            exp_overflow(prec, rm)
        } else {
            fail_on_untested_path(
                "rational_pow, ex_pow_underflow: as ex_pow_overflow, in the negative-exponent \
                 direction",
            );
            exp_underflow(prec, if rm == Nearest { Down } else { rm })
        });
    };
    let (shifted, oo) = result.shl_prec_round(shift, prec, rm);
    Some((shifted, if oo == Equal { o } else { oo }))
}

// Whether the Rational y is an odd integer.
fn rational_odd_integer(y: &Rational) -> bool {
    *y.denominator_ref() == 1u32 && y.numerator_ref().odd()
}

// Raises a finite, positive Float x to the power of a finite, nonzero, non-integer Rational y = a /
// b (in lowest terms, so b >= 2), returning the result rounded to `prec` bits with `rm`.
fn positive_float_pow_rational(
    x: &Float,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // x = c * 2^d with c odd (c >= 1).
    let (c, d) = float_to_odd_mantissa_and_exponent_natural(x);
    // x = 2^d: x^y = 2^(d * y), an exact-Rational exponent that `power_of_2_rational_prec_round`
    // handles completely (exactness, overflow, and underflow).
    if c == 1u32 {
        return Float::power_of_2_rational_prec_round(Rational::from(d) * y, prec, rm);
    }
    // x^(a/b) is rational exactly when x is a perfect b-th power of a Float, i.e. b | d and the odd
    // part c is a perfect b-th power j^b. Then x^(1/b) = j * 2^(d/b) is an exact Float `base`, and
    // x^(a/b) = base^a is delegated to `pow_integer`, which correctly rounds the (possibly
    // non-dyadic, for a < 0) result and handles overflow and underflow. Otherwise x^(a/b) is
    // irrational.
    if let Ok(b) = u64::try_from(y.denominator_ref())
        && d.unsigned_abs().divisible_by(b)
        && let Some(j) = (&c).checked_root(b)
    {
        let base = Float::exact_from(j) << (d / i64::exact_from(b));
        let a = Integer::from_sign_and_abs_ref(*y > 0u32, y.numerator_ref());
        return base.pow_integer_prec_round(a, prec, rm);
    }
    // The result is irrational. First a tiny-result shortcut: if |y * log2(x)| is far below 1, then
    // x^y rounds to 1 +/- ulp, sparing the (possibly huge) log2 bracketing. Since |y| < 2^ey and
    // |log2(x)| < 2^expb, one has |y * log2(x)| < 2^(ey + expb).
    let ex = i64::from(x.get_exponent().unwrap());
    let ey = y.floor_log_base_2_abs() + 1;
    let above = (*y > 0u32) == (*x > 1u32);
    let expb = if ex == 0 || ex == 1 {
        // x is in (1/2, 2), close to 1 (and x != 1, since |x| = 1 was handled by the caller): with
        // fld = floor(log2|x - 1|), one has |log2(x)| < 2^(fld + 2).
        (Rational::exact_from(x) - Rational::ONE).floor_log_base_2_abs() + 2
    } else {
        // x is bounded away from 1: |log2(x)| <= expx = max(ex, 1 - ex) < 2^ceil(log2(expx)).
        let expx = if ex > 1 { ex } else { 1 - ex };
        i64::exact_from(u64::exact_from(expx).ceiling_log_base_2())
    };
    if ey + expb < -i64::exact_from(prec) - 1 {
        return float_one_plus_tiny(prec, rm, above);
    }
    // General squeeze: bracket log2(x) = d + log2(c) between exact Rationals and apply 2^(y * (d +
    // log2(c))). Working in the exponent (t-space) stays correct even when x is a sliver of 1,
    // where a Float-based log2(x) would underflow below the smallest positive Float.
    pow_squeeze_t(&Rational::from(c), d, y, prec, rm)
}

// Raises a Float to the power of a Rational, returning a Float rounded to `prec` bits with `rm`.
fn float_rational_pow(x: &Float, y: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    // Exact rounding: compute with Nearest and demand exactness.
    if rm == Exact {
        let (result, o) = float_rational_pow(x, y, prec, Nearest);
        assert_eq!(o, Equal, "Inexact pow");
        return (result, Equal);
    }
    // x^0 = 1 for any x, even NaN.
    if *y == 0u32 {
        return (Float::one_prec(prec), Equal);
    }
    // Singular x; see Section F.9.4.4 of the C standard. y is a finite nonzero Rational, so the
    // singular-y cases (0, NaN, +/-Inf) do not arise.
    match x {
        float_nan!() => return (Float::NAN, Equal),
        Float(Infinity { sign }) => {
            let negative = !*sign && rational_odd_integer(y);
            return (
                match (*y > 0u32, negative) {
                    (true, false) => Float::INFINITY,
                    (true, true) => Float::NEGATIVE_INFINITY,
                    (false, false) => Float::ZERO,
                    (false, true) => Float::NEGATIVE_ZERO,
                },
                Equal,
            );
        }
        Float(Zero { sign }) => {
            let negative = !*sign && rational_odd_integer(y);
            return (
                match (*y < 0u32, negative) {
                    (true, false) => Float::INFINITY,
                    (true, true) => Float::NEGATIVE_INFINITY,
                    (false, false) => Float::ZERO,
                    (false, true) => Float::NEGATIVE_ZERO,
                },
                Equal,
            );
        }
        _ => {}
    }
    // x finite and nonzero.
    let y_is_integer = *y.denominator_ref() == 1u32;
    // x^y for x < 0 and y not an integer is not defined.
    if x.is_sign_negative() && !y_is_integer {
        return (Float::NAN, Equal);
    }
    // |x| = 1: (+/-1)^y = +/-1 (the sign is negative only for x = -1 and odd y).
    if x.partial_cmp_abs(&Float::ONE).unwrap() == Equal {
        let negative = x.is_sign_negative() && rational_odd_integer(y);
        return Float::from_float_prec_round(
            if negative { -Float::ONE } else { Float::ONE },
            prec,
            rm,
        );
    }
    // Integer y: the multiplication-based `pow_integer` handles negative x (via parity), overflow,
    // and underflow.
    if y_is_integer {
        return pow_integer(x, &Integer::rounding_from(y, Exact).0, prec, rm);
    }
    // x > 0 (negative x with non-integer y was rejected above), y = a / b with b >= 2.
    positive_float_pow_rational(x, y, prec, rm)
}

// Whether x^y is a dyadic rational (hence possibly exactly representable), for a positive
// non-power-of-2 Rational x = (a / b) * 2^e (a, b odd and coprime) and a finite nonzero non-integer
// Rational y = a_y / b_y (in lowest terms, b_y >= 2). If so, returns (m, z, pow) such that x^y =
// m^z * 2^pow with m an odd Natural (> 1) and z a positive Integer; otherwise returns None. Since x
// is not a power of 2, a Ziv-style squeeze on an exact x^y would never terminate, and a
// nearest-mode tie is possible only in the dyadic case, so this decides when the direct route is
// required.
fn rational_rational_pow_exact_decomposition(
    a: &Natural,
    b: &Natural,
    e: i64,
    y: &Rational,
) -> Option<(Natural, Integer, Integer)> {
    let b_y = u64::try_from(y.denominator_ref()).ok()?;
    // 2^(e * a_y / b_y) is dyadic exactly when b_y | e (since gcd(a_y, b_y) = 1).
    if !e.unsigned_abs().divisible_by(b_y) {
        return None;
    }
    // (a / b)^(a_y / b_y) is dyadic only if a and b are each perfect b_y-th powers.
    let p = a.checked_root(b_y)?;
    let q = b.checked_root(b_y)?;
    let a_y_abs = y.numerator_ref();
    // pow = e * a_y / b_y = (e / b_y) * a_y, an exact integer.
    let pow = Integer::from(e / i64::exact_from(b_y))
        * Integer::from_sign_and_abs_ref(*y > 0u32, a_y_abs);
    if *y > 0u32 {
        // p^a_y / q^a_y is dyadic (q odd) only when q = 1, i.e. b = 1. Then m = p (> 1, since x is
        // not a power of 2, so a > 1 here).
        if q != 1u32 {
            return None;
        }
        Some((p, Integer::from(a_y_abs), pow))
    } else {
        // q^|a_y| / p^|a_y| is dyadic only when p = 1, i.e. a = 1. Then m = q (> 1).
        if p != 1u32 {
            return None;
        }
        Some((q, Integer::from(a_y_abs), pow))
    }
}

// Raises a Rational to a Rational power, returning a Float rounded to `prec` bits with `rm`.
fn rational_rational_pow(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    // Exact rounding: compute with Nearest and demand exactness.
    if rm == Exact {
        let (result, o) = rational_rational_pow(x, y, prec, Nearest);
        assert_eq!(o, Equal, "Inexact rational_rational_pow");
        return (result, Equal);
    }
    // x^0 = 1 for any x, even 0.
    if *y == 0u32 {
        return (Float::one_prec(prec), Equal);
    }
    // x = 0: a Rational zero is unsigned, so the results take positive signs.
    if *x == 0u32 {
        return if *y > 0u32 {
            (Float::ZERO, Equal)
        } else {
            (Float::INFINITY, Equal)
        };
    }
    let y_is_integer = *y.denominator_ref() == 1u32;
    // Negative x: only an integer y is defined; the sign is that of (-1)^y.
    if *x < 0u32 {
        if !y_is_integer {
            return (Float::NAN, Equal);
        }
        let negative = rational_odd_integer(y);
        let (result, o) = rational_rational_pow(&(-x), y, prec, if negative { -rm } else { rm });
        return if negative {
            (-result, o.reverse())
        } else {
            (result, o)
        };
    }
    if *x == 1u32 {
        return (Float::one_prec(prec), Equal);
    }
    // x = 2^e exactly: x^y = 2^(e * y) with e * y an exact Rational;
    // `power_of_2_rational_prec_round` handles all exactness, overflow, and underflow.
    if let Some(e) = x.checked_log_base_2() {
        let t = Rational::from(e) * y;
        return Float::power_of_2_rational_prec_round(t, prec, rm);
    }
    // Small integer y with a small base: materialize x^y as an exact Rational;
    // `from_rational_prec_round` handles all rounding, including at the range boundaries.
    let nbits = x.significant_bits();
    if y_is_integer
        && let Ok(z) = i64::try_from(y.numerator_ref())
        && z.unsigned_abs().saturating_mul(nbits) <= max(65536, prec << 2)
    {
        let z = if *y > 0u32 { z } else { -z };
        return Float::from_rational_prec_round(x.pow(z), prec, rm);
    }
    let fl = x.floor_log_base_2_abs();
    let in_range =
        fl > i64::from(Float::MIN_EXPONENT) + 2 && fl < i64::from(Float::MAX_EXPONENT) - 2;
    // A base within a few binades of 1 is a sliver whose logarithm is at or below the smallest
    // positive Float; it must go through the exact-Rational t-space squeeze (which brackets log2
    // over Rationals) rather than any Float-based route, which would underflow the logarithm. `x`
    // is a sliver only when it lies in `(1/2, 2)`, i.e. `fl` is 0 or -1.
    let sliver_fld = if fl == 0 || fl == -1 {
        Some((x - Rational::ONE).floor_log_base_2_abs())
    } else {
        None
    };
    let sliver_of_one = sliver_fld.is_some_and(|fld| fld < i64::from(Float::MIN_EXPONENT) + 8);
    // A dyadic in-range non-sliver base is exactly convertible to a Float; `Float::pow_rational`
    // does the rest, exactness and boundary behavior included.
    if in_range && !sliver_of_one && x.denominator_ref().is_power_of_2() {
        let xf = Float::from_rational_prec_round_ref(x, nbits, Floor).0;
        return xf.pow_rational_prec_round_val_ref(y, prec, rm);
    }
    // Possible exact dyadic results must be handled directly: a Ziv squeeze never terminates on an
    // exactly-representable value and can stall on a nearest-mode tie.
    let n = x.numerator_ref();
    let d = x.denominator_ref();
    let alpha = i64::exact_from(n.trailing_zeros().unwrap());
    let beta = i64::exact_from(d.trailing_zeros().unwrap());
    let a = n >> alpha;
    let b = d >> beta;
    if let Some((m, z, pow)) = rational_rational_pow_exact_decomposition(&a, &b, alpha - beta, y)
        && let Some(result) = rational_pow_exact(&m, &z, &pow, prec, rm)
    {
        return result;
    }
    // Tiny-result shortcut for a sliver of 1: if |y * log2(x)| is far below 1, x^y rounds to 1 +/-
    // ulp, avoiding the (up to 128-MB) log2 brackets. With fld = floor_log2|x - 1|, one has
    // |log2(x)| < 2^(fld + 2), so |y * log2(x)| < 2^(ey + fld + 2).
    if let Some(fld) = sliver_fld {
        let ey = y.floor_log_base_2_abs() + 1;
        if ey + fld + 2 < -i64::exact_from(prec) - 1 {
            let above = (*y > 0u32) == (*x > 1u32);
            return float_one_plus_tiny(prec, rm, above);
        }
    }
    // The result is irrational (or a non-dyadic rational): squeeze 2^(y * log2(x)) in the exponent
    // (t-space) over exact Rationals. Splitting off the odd part keeps the log2 bracketing exact
    // for extreme or sliver bases, where a Float logarithm would underflow.
    let xp = Rational::from(a) / Rational::from(b);
    pow_squeeze_t(&xp, alpha - beta, y, prec, rm)
}

impl Float {
    /// Raises a [`Rational`] to a [`Rational`] power, returning the result as a [`Float`] rounded
    /// to the specified precision and with the specified rounding mode. Both [`Rational`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(x,0,p,m)=1.0$ for any $x$, even $0$
    /// - $f(0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(1,y,p,m)=1.0$
    /// - $f(-1,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    ///
    /// Both operands are exact [`Rational`]s, so the exact [`Rational`] exponent selects a definite
    /// branch of the power, and results that are exactly representable (such as roots of perfect
    /// powers) are detected and rounded exactly.
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
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_rational_prec_round(
    ///     Rational::from_signeds(3, 2),
    ///     Rational::from_signeds(5, 2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.755672");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_rational_prec_round(
    ///     Rational::from_signeds(3, 2),
    ///     Rational::from_signeds(5, 2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    ///
    /// // (9/4)^(1/2) = 3/2 is exact.
    /// let (p, o) = Float::rational_pow_rational_prec_round(
    ///     Rational::from_signeds(9, 4),
    ///     Rational::from_signeds(1, 2),
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "1.5");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_pow_rational_prec_round(
        x: Rational,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_rational_pow(&x, &y, prec, rm)
    }

    /// Raises a [`Rational`] to a [`Rational`] power, returning the result as a [`Float`] rounded
    /// to the specified precision and with the specified rounding mode. Both [`Rational`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::rational_pow_rational_prec_round`] for special cases, overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_rational_prec_round_ref(
    ///     &Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(-1, 2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "1.224747");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_rational_prec_round_ref(
        x: &Rational,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        rational_rational_pow(x, y, prec, rm)
    }

    /// Raises a [`Rational`] to a [`Rational`] power, returning the result as a [`Float`] rounded
    /// to the specified precision and to the nearest value. Both [`Rational`]s are taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded power is less than, equal
    /// to, or greater than the exact power. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::rational_pow_rational_prec_round`] for special cases, overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_rational_prec(
    ///     Rational::from_signeds(3, 2),
    ///     Rational::from_signeds(5, 2),
    ///     20,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) =
    ///     Float::rational_pow_rational_prec(Rational::from(8), Rational::from_signeds(1, 3), 10);
    /// assert_eq!(p.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn rational_pow_rational_prec(x: Rational, y: Rational, prec: u64) -> (Self, Ordering) {
        rational_rational_pow(&x, &y, prec, Nearest)
    }

    /// Raises a [`Rational`] to a [`Rational`] power, returning the result as a [`Float`] rounded
    /// to the specified precision and to the nearest value. Both [`Rational`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::rational_pow_rational_prec_round`] for special cases, overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_rational_prec_ref(
    ///     &Rational::from_signeds(3, 2),
    ///     &Rational::from_signeds(5, 2),
    ///     20,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_rational_prec_ref(
        x: &Rational,
        y: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        rational_rational_pow(x, y, prec, Nearest)
    }
}

impl Float {
    // Raises a Rational to a Float power, returning a Float rounded to the specified precision with
    // the specified rounding mode.

    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and with the specified rounding mode. The [`Rational`] and the
    /// [`Float`] are both taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
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
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p,m)=1.0$
    /// - $f(-1,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
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
    /// If you know you'll be using `Nearest`, consider using [`Float::rational_pow_prec_ref_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.755672");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn rational_pow_prec_round_ref_ref(
        x: &Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // Exact rounding: compute with Nearest and demand exactness.
        if rm == Exact {
            let (result, o) = Self::rational_pow_prec_ref_ref(x, y, prec);
            assert_eq!(o, Equal, "Inexact rational_pow");
            return (result, Equal);
        }
        // Singular y; see Section F.9.4.4 of the C standard.
        match y {
            // x^0 = 1 for any x, even 0
            float_either_zero!() => {
                return (Self::one_prec(prec), Equal);
            }
            // 1^y = 1 for any y, even NaN
            float_nan!() => {
                return if *x == 1u32 {
                    (Self::one_prec(prec), Equal)
                } else {
                    (Self::NAN, Equal)
                };
            }
            Self(Infinity { sign }) => {
                let mut cmp = x.cmp_abs(&Rational::ONE);
                if !*sign {
                    cmp = cmp.reverse();
                }
                return match cmp {
                    Greater => (Self::INFINITY, Equal),
                    Less => (Self::ZERO, Equal),
                    Equal => (Self::one_prec(prec), Equal),
                };
            }
            _ => {}
        }
        // x = 0: Rational zero is unsigned, so the results take positive signs.
        if *x == 0u32 {
            return if *y > 0u32 {
                (Self::ZERO, Equal)
            } else {
                (Self::INFINITY, Equal)
            };
        }
        let y_is_integer = y.is_integer();
        // Negative x: only integer y is defined; the sign is that of (-1)^y.
        if *x < 0u32 {
            if !y_is_integer {
                return (Self::NAN, Equal);
            }
            let negative = float_odd_integer(y);
            let (result, o) = Self::rational_pow_prec_round_ref_ref(
                &(-x),
                y,
                prec,
                if negative { -rm } else { rm },
            );
            return if negative {
                (-result, o.reverse())
            } else {
                (result, o)
            };
        }
        if *x == 1u32 {
            return (Self::one_prec(prec), Equal);
        }
        // x = 2^e exactly: x^y = 2^(e * y) with e * y an exact Rational;
        // `power_of_2_rational_prec_round` handles all exactness, overflow, and underflow.
        if let Some(e) = x.checked_log_base_2() {
            let t = Rational::from(e) * Rational::exact_from(y);
            return Self::power_of_2_rational_prec_round(t, prec, rm);
        }
        // Small integer y with a small base: materialize x^y as an exact Rational;
        // `from_rational_prec_round` handles all rounding, including at the range boundaries.
        let nbits = x.significant_bits();
        if y_is_integer && y.get_exponent().unwrap() <= 32 {
            let z = i64::rounding_from(y, Nearest).0;
            if z.unsigned_abs().saturating_mul(nbits) <= max(65536, prec << 2) {
                return Self::from_rational_prec_round(x.pow(z), prec, rm);
            }
        }
        let fl = x.floor_log_base_2_abs();
        let in_range =
            fl > i64::from(Self::MIN_EXPONENT) + 2 && fl < i64::from(Self::MAX_EXPONENT) - 2;
        // A base within a few binades of 1 (from either side) has a logarithm at or below the
        // smallest positive Float, where any Float-based power -- the dyadic shortcut or the
        // x-space squeeze below, both of which call `Float::pow` -- would underflow internally
        // (`ln` cannot represent the sub-`MIN_EXPONENT` result). Such a base goes through the
        // exact-Rational t-space squeeze, which brackets `log2` with the atanh series over
        // `Rational`s and never materializes a sub-`MIN_EXPONENT` Float logarithm. `x` is a sliver
        // of 1 only when it lies in `(1/2, 2)`, i.e. `fl` is 0 or -1; the exact subtraction is
        // skipped otherwise.
        let sliver_fld = if fl == 0 || fl == -1 {
            if *x == 1u32 {
                None
            } else {
                Some((x - Rational::ONE).floor_log_base_2_abs())
            }
        } else {
            None
        };
        let sliver_of_one = sliver_fld.is_some_and(|fld| fld < i64::from(Self::MIN_EXPONENT) + 8);
        // A dyadic in-range non-sliver x is exactly convertible; Float::pow does the rest,
        // exactness and boundary behavior included.
        if in_range && !sliver_of_one && x.denominator_ref().is_power_of_2() {
            let xf = Self::from_rational_prec_round_ref(x, nbits, Floor).0;
            return xf.pow_prec_round_val_ref(y, prec, rm);
        }
        // Possible exact dyadic results must be handled directly: a Ziv squeeze never terminates on
        // an exactly-representable value and can stall on a nearest-mode tie.
        let n = x.numerator_ref();
        let d = x.denominator_ref();
        let alpha = i64::exact_from(n.trailing_zeros().unwrap());
        let beta = i64::exact_from(d.trailing_zeros().unwrap());
        let a = n >> alpha;
        let b = d >> beta;
        if let Some((m, z, pow)) = rational_pow_exact_decomposition(&a, &b, alpha - beta, y)
            && let Some(result) = rational_pow_exact(&m, &z, &pow, prec, rm)
        {
            return result;
        }
        if in_range && !sliver_of_one {
            rational_pow_squeeze_x(x, y, prec, rm)
        } else {
            // Tiny-result shortcut for a sliver of 1: if |y * log2(x)| is far below 1, x^y rounds
            // to 1 +/- ulp, avoiding the (up to 128-MB) log2 brackets. With fld = floor_log2|x -
            // 1|, one has |log2(x)| < 2^(fld + 2), so |y * log2(x)| < 2^(ey + fld + 2); when that
            // is below 2^(-prec - 1) the result is within half an ulp of 1.
            if let Some(fld) = sliver_fld {
                let ey = i64::from(y.get_exponent().unwrap());
                if ey + fld + 2 < -i64::exact_from(prec) - 1 {
                    let above = (*y > 0u32) == (*x > 1u32);
                    return float_one_plus_tiny(prec, rm, above);
                }
            }
            // Extreme x -- beyond the exponent range or a sliver of 1: split off the power of 2
            // (rounded to the nearest, so the mantissa is close to 1) and work with exact Rationals
            // in the exponent.
            let (xp, g) = rational_mantissa_nearest_power_of_2(x);
            pow_squeeze_t(&xp, g, &Rational::exact_from(y), prec, rm)
        }
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and with the specified rounding mode. The [`Rational`] and the
    /// [`Float`] are both taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
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
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p,m)=1.0$
    /// - $f(-1,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
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
    /// If you know you'll be using `Nearest`, consider using [`Float::rational_pow_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_prec_round(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.755672");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_round(
        x: Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::rational_pow_prec_round_ref_ref(&x, &y, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// value and the [`Float`] by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
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
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p,m)=1.0$
    /// - $f(-1,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
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
    /// If you know you'll be using `Nearest`, consider using [`Float::rational_pow_prec_val_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.755672");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_round_val_ref(
        x: Rational,
        y: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::rational_pow_prec_round_ref_ref(&x, y, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and with the specified rounding mode. The [`Rational`] is taken by
    /// reference and the [`Float`] by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
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
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,\pm0.0,p,m)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p,m)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p,m)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p,m)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p,m)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p,m)=1.0$
    /// - $f(-1,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
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
    /// If you know you'll be using `Nearest`, consider using [`Float::rational_pow_prec_ref_val`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.9");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(p.to_string(), "2.755672");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) = Float::rational_pow_prec_round_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_round_ref_val(
        x: &Rational,
        y: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::rational_pow_prec_round_ref_ref(x, &y, prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and to the nearest value. The [`Rational`] and the [`Float`] are
    /// both taken by value. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p)=1.0$
    /// - $f(-1,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_pow_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::rational_pow_prec(Rational::from_unsigneds(3u32, 2u32), Float::from(2.5), 5);
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) =
    ///     Float::rational_pow_prec(Rational::from_unsigneds(3u32, 2u32), Float::from(2.5), 20);
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec(x: Rational, y: Self, prec: u64) -> (Self, Ordering) {
        Self::rational_pow_prec_ref_ref(&x, &y, prec)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and to the nearest value. The [`Rational`] is taken by value and the
    /// [`Float`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p)=1.0$
    /// - $f(-1,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_pow_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_prec_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_val_ref(
    ///     Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_val_ref(x: Rational, y: &Self, prec: u64) -> (Self, Ordering) {
        Self::rational_pow_prec_ref_ref(&x, y, prec)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and to the nearest value. The [`Rational`] is taken by reference and
    /// the [`Float`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p)=1.0$
    /// - $f(-1,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_pow_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_prec_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     5,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_ref_val(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     Float::from(2.5),
    ///     20,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_ref_val(x: &Rational, y: Self, prec: u64) -> (Self, Ordering) {
        Self::rational_pow_prec_ref_ref(x, &y, prec)
    }

    /// Raises a [`Rational`] to a [`Float`] power, returning the result as a [`Float`] rounded to
    /// the specified precision and to the nearest value. The [`Rational`] and the [`Float`] are
    /// both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(x,\pm0.0,p)=1.0$ for any $x$, even $0$
    /// - $f(1,y,p)=1.0$ for any $y$, even `NaN`
    /// - $f(x,\text{NaN},p)=\text{NaN}$ otherwise
    /// - $f(x,\infty,p)=\infty$ if $|x|>1$, and $0.0$ if $|x|<1$
    /// - $f(x,-\infty,p)=0.0$ if $|x|>1$, and $\infty$ if $|x|<1$
    /// - $f(\pm1,\pm\infty,p)=1.0$
    /// - $f(-1,y,p)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(0,y,p)=0.0$ if $y>0$, and $\infty$ if $y<0$; a [`Rational`] zero is unsigned, so the
    ///   results take positive signs
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ and $y$ is finite and not an integer
    ///
    /// Unlike a [`Float`] base, a [`Rational`] base may lie outside the [`Float`] exponent range or
    /// so close to 1 that no [`Float`] can represent its logarithm; both cases are handled exactly,
    /// by working with the base as an exact [`Rational`] throughout.
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - Negative results (from negative $x$ and odd integer $y$) mirror the bullets above.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::rational_pow_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits(),
    /// y.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::rational_pow_prec_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     5,
    /// );
    /// assert_eq!(p.to_string(), "2.8");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::rational_pow_prec_ref_ref(
    ///     &Rational::from_unsigneds(3u32, 2u32),
    ///     &Float::from(2.5),
    ///     20,
    /// );
    /// assert_eq!(p.to_string(), "2.755676");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn rational_pow_prec_ref_ref(x: &Rational, y: &Self, prec: u64) -> (Self, Ordering) {
        Self::rational_pow_prec_round_ref_ref(x, y, prec, Nearest)
    }
}

impl Float {
    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. Both the [`Float`] and the [`Rational`] are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
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
    /// - $f(x,0,p,m)=1.0$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1.0,y,p,m)=1.0$
    /// - $f(-1.0,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is
    ///   negative and not an odd integer
    /// - $f(0.0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    ///
    /// Unlike the exponent of a [`Float`], the exact [`Rational`] exponent selects a definite
    /// branch of the power, so results that are exactly representable (such as roots of perfect
    /// powers) are detected and rounded exactly.
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
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) =
    ///     Float::from(2).pow_rational_prec_round(Rational::from_signeds(3, 2), 20, Floor);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) =
    ///     Float::from(2).pow_rational_prec_round(Rational::from_signeds(3, 2), 20, Ceiling);
    /// assert_eq!(p.to_string(), "2.82843");
    /// assert_eq!(o, Greater);
    ///
    /// let (p, o) =
    ///     Float::from(8).pow_rational_prec_round(Rational::from_signeds(1, 3), 20, Floor);
    /// assert_eq!(p.to_string(), "2.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec_round(
        self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        float_rational_pow(&self, &other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    pub fn pow_rational_prec_round_val_ref(
        self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        float_rational_pow(&self, other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn pow_rational_prec_round_ref_val(
        &self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        float_rational_pow(self, &other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode. Both the [`Float`] and the [`Rational`] are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    pub fn pow_rational_prec_round_ref_ref(
        &self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        float_rational_pow(self, other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and to the nearest value. Both the [`Float`] and the [`Rational`] are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded power is less than,
    /// equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
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
    /// - $f(x,0,p,m)=1.0$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1.0,y,p,m)=1.0$
    /// - $f(-1.0,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is
    ///   negative and not an odd integer
    /// - $f(0.0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    ///
    /// Unlike the exponent of a [`Float`], the exact [`Rational`] exponent selects a definite
    /// branch of the power, so results that are exactly representable (such as roots of perfect
    /// powers) are detected and rounded exactly.
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
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(2).pow_rational_prec(Rational::from_signeds(3, 2), 20);
    /// assert_eq!(p.to_string(), "2.828426");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(27).pow_rational_prec(Rational::from_signeds(2, 3), 20);
    /// assert_eq!(p.to_string(), "9.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn pow_rational_prec(self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.pow_rational_prec_round(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and to the nearest value. The [`Float`] is taken by value and the [`Rational`] by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    pub fn pow_rational_prec_val_ref(self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.pow_rational_prec_round_val_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and to the nearest value. The [`Float`] is taken by reference and the [`Rational`]
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    pub fn pow_rational_prec_ref_val(&self, other: Rational, prec: u64) -> (Self, Ordering) {
        self.pow_rational_prec_round_ref_val(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the specified
    /// precision and to the nearest value. Both the [`Float`] and the [`Rational`] are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    pub fn pow_rational_prec_ref_ref(&self, other: &Rational, prec: u64) -> (Self, Ordering) {
        self.pow_rational_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the precision of
    /// the base and with the specified rounding mode. Both the [`Float`] and the [`Rational`] are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
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
    /// - $f(x,0,p,m)=1.0$ for any $x$, even `NaN`
    /// - $f(\text{NaN},y,p,m)=\text{NaN}$ if $y \neq 0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ and $y$ is not an integer
    /// - $f(1.0,y,p,m)=1.0$
    /// - $f(-1.0,y,p,m)=1.0$ if $y$ is an even integer, and $-1.0$ if $y$ is an odd integer
    /// - $f(\infty,y,p,m)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(-\infty,y,p,m)=-\infty$ if $y$ is a positive odd integer, $\infty$ if $y$ is positive
    ///   and not an odd integer, $-0.0$ if $y$ is a negative odd integer, and $0.0$ if $y$ is
    ///   negative and not an odd integer
    /// - $f(0.0,y,p,m)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(-0.0,y,p,m)=-0.0$ if $y$ is a positive odd integer, $0.0$ if $y$ is positive and not an
    ///   odd integer, $-\infty$ if $y$ is a negative odd integer, and $\infty$ if $y$ is negative
    ///   and not an odd integer
    ///
    /// Unlike the exponent of a [`Float`], the exact [`Rational`] exponent selects a definite
    /// branch of the power, so results that are exactly representable (such as roots of perfect
    /// powers) are detected and rounded exactly.
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
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// // The output precision is the precision of the base, here 3 bits.
    /// let (p, o) = Float::from(5).pow_rational_round(Rational::from_signeds(3, 2), Floor);
    /// assert_eq!(p.to_string(), "10.0");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(5).pow_rational_round(Rational::from_signeds(3, 2), Ceiling);
    /// assert_eq!(p.to_string(), "12.0");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn pow_rational_round(self, other: Rational, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_rational_prec_round(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the precision of
    /// the base and with the specified rounding mode. The [`Float`] is taken by value and the
    /// [`Rational`] by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    pub fn pow_rational_round_val_ref(
        self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_val_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the precision of
    /// the base and with the specified rounding mode. The [`Float`] is taken by reference and the
    /// [`Rational`] by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// power is less than, equal to, or greater than the exact power. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    pub fn pow_rational_round_ref_val(
        &self,
        other: Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_ref_val(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the precision of
    /// the base and with the specified rounding mode. Both the [`Float`] and the [`Rational`] are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The output precision is the precision of `self`. See [`RoundingMode`] for a description of
    /// the possible rounding modes.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    pub fn pow_rational_round_ref_ref(
        &self,
        other: &Rational,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_ref_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// value.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    #[allow(clippy::needless_pass_by_value)]
    pub fn pow_rational_prec_round_assign(
        &mut self,
        other: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = float_rational_pow(self, &other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// reference.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    pub fn pow_rational_prec_round_assign_ref(
        &mut self,
        other: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = float_rational_pow(self, other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// value.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    #[inline]
    pub fn pow_rational_prec_assign(&mut self, other: Rational, prec: u64) -> Ordering {
        self.pow_rational_prec_round_assign(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// reference.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    #[inline]
    pub fn pow_rational_prec_assign_ref(&mut self, other: &Rational, prec: u64) -> Ordering {
        self.pow_rational_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// value.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    pub fn pow_rational_round_assign(&mut self, other: Rational, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_assign(other, prec, rm)
    }

    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// reference.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the base's
    /// precision.
    pub fn pow_rational_round_assign_ref(
        &mut self,
        other: &Rational,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_assign_ref(other, prec, rm)
    }
}

impl Pow<Rational> for Float {
    type Output = Self;

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the nearest value
    /// at the precision of the base. Both the [`Float`] and the [`Rational`] are taken by value.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    fn pow(self, other: Rational) -> Self {
        let prec = self.significant_bits();
        self.pow_rational_prec_round(other, prec, Nearest).0
    }
}

impl Pow<&Rational> for Float {
    type Output = Self;

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the nearest value
    /// at the precision of the base. The [`Float`] is taken by value and the [`Rational`] by
    /// reference.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    fn pow(self, other: &Rational) -> Self {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Pow<Rational> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the nearest value
    /// at the precision of the base. The [`Float`] is taken by reference and the [`Rational`] by
    /// value.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    fn pow(self, other: Rational) -> Float {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Pow<&Rational> for &Float {
    type Output = Float;

    /// Raises a [`Float`] to the power of a [`Rational`], rounding the result to the nearest value
    /// at the precision of the base. Both the [`Float`] and the [`Rational`] are taken by
    /// reference.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    fn pow(self, other: &Rational) -> Float {
        let prec = self.significant_bits();
        self.pow_rational_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl PowAssign<Rational> for Float {
    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// value, and rounding the result to the nearest value at the precision of the base.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    fn pow_assign(&mut self, other: Rational) {
        let prec = self.significant_bits();
        self.pow_rational_prec_assign(other, prec);
    }
}

impl PowAssign<&Rational> for Float {
    /// Raises a [`Float`] to the power of a [`Rational`] in place, taking the [`Rational`] by
    /// reference, and rounding the result to the nearest value at the precision of the base.
    ///
    /// See the [`Float::pow_rational_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    #[inline]
    fn pow_assign(&mut self, other: &Rational) {
        let prec = self.significant_bits();
        self.pow_rational_prec_assign_ref(other, prec);
    }
}

impl Float {
    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and with the specified rounding mode. Both [`Float`]s are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded power is
    /// less than, equal to, or greater than the exact power. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// `powr(x, y)` is $e^{y\ln x}$; unlike [`pow`](Float::pow_prec_round), its base is restricted
    /// to $x\geq 0$ and it never produces a negative result.
    ///
    /// Special cases:
    /// - $f(x,y)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $\pm0$ or $\infty$ and $y=0$, or
    ///   if $x=1$ and $y$ is infinite
    /// - $f(x,0)=1.0$ if $x$ is finite and positive
    /// - $f(1.0,y)=1.0$ if $y$ is finite
    /// - $f(\infty,y)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(\pm0.0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty)=\infty$ if $x>1$, and $0.0$ if $0<x<1$
    /// - $f(x,-\infty)=0.0$ if $x>1$, and $\infty$ if $0<x<1$
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
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).powr_prec_round(Float::from(2.5), 20, Floor);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).powr_prec_round(Float::from(2.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "15.58847");
    /// assert_eq!(o, Greater);
    ///
    /// // A negative base gives NaN (unlike `pow`).
    /// let (p, o) = Float::from(-2).powr_prec_round(Float::from(3), 10, Nearest);
    /// assert_eq!(p.to_string(), "NaN");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn powr_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.powr_prec_round_ref_ref(&other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and with the specified rounding mode. The first [`Float`]
    /// is taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    #[inline]
    pub fn powr_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.powr_prec_round_ref_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and with the specified rounding mode. The first [`Float`]
    /// is taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded power is less than, equal to, or greater than the exact power. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn powr_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.powr_prec_round_ref_ref(&other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and with the specified rounding mode. Both [`Float`]s are
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded power
    /// is less than, equal to, or greater than the exact power. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// `powr(x, y)` is $e^{y\ln x}$; unlike [`pow`](Float::pow_prec_round), its base is restricted
    /// to $x\geq 0$ and it never produces a negative result.
    ///
    /// Special cases:
    /// - $f(x,y)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $\pm0$ or $\infty$ and $y=0$, or
    ///   if $x=1$ and $y$ is infinite
    /// - $f(x,0)=1.0$ if $x$ is finite and positive
    /// - $f(1.0,y)=1.0$ if $y$ is finite
    /// - $f(\infty,y)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(\pm0.0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty)=\infty$ if $x>1$, and $0.0$ if $0<x<1$
    /// - $f(x,-\infty)=0.0$ if $x>1$, and $\infty$ if $0<x<1$
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
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).powr_prec_round(Float::from(2.5), 20, Floor);
    /// assert_eq!(p.to_string(), "15.58846");
    /// assert_eq!(o, Less);
    ///
    /// let (p, o) = Float::from(3).powr_prec_round(Float::from(2.5), 20, Ceiling);
    /// assert_eq!(p.to_string(), "15.58847");
    /// assert_eq!(o, Greater);
    ///
    /// // A negative base gives NaN (unlike `pow`).
    /// let (p, o) = Float::from(-2).powr_prec_round(Float::from(3), 10, Nearest);
    /// assert_eq!(p.to_string(), "NaN");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn powr_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        let x = self;
        let y = other;
        // powr(x, y) = exp(y * ln(x)). This is `mpfr_powr` from `powr.c`, MPFR 4.3.0.
        match (x, y) {
            // A NaN or negative base (finite negative or -Inf) is NaN (pow allows a negative base
            // with an integer exponent); and a singular +0, -0, or +Inf base with a zero exponent
            // is NaN (pow gives 1).
            (Self(NaN | Finite { sign: false, .. } | Infinity { sign: false }), _)
            | (Self(Zero { .. } | Infinity { sign: true }), float_either_zero!()) => {
                (Self::NAN, Equal)
            }
            // powr treats -0 like +0: a finite nonzero exponent gives +0 (y > 0) or +Inf (y < 0),
            // always positive (pow gives a signed result for odd-integer y).
            (float_negative_zero!(), Self(Finite { sign, .. })) => {
                if *sign {
                    (Self::ZERO, Equal)
                } else {
                    (Self::INFINITY, Equal)
                }
            }
            // A base of exactly 1 with an infinite exponent is NaN (pow gives 1).
            (_, float_either_infinity!()) if *x == 1u32 => (Self::NAN, Equal),
            // Everything else defers to pow.
            _ => self.pow_prec_round_ref_ref(y, prec, rm),
        }
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and to the nearest value. Both [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded power is less than,
    /// equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// `powr(x, y)` is $e^{y\ln x}$; unlike [`pow`](Float::pow_prec_round), its base is restricted
    /// to $x\geq 0$ and it never produces a negative result.
    ///
    /// Special cases:
    /// - $f(x,y)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $\pm0$ or $\infty$ and $y=0$, or
    ///   if $x=1$ and $y$ is infinite
    /// - $f(x,0)=1.0$ if $x$ is finite and positive
    /// - $f(1.0,y)=1.0$ if $y$ is finite
    /// - $f(\infty,y)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(\pm0.0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty)=\infty$ if $x>1$, and $0.0$ if $0<x<1$
    /// - $f(x,-\infty)=0.0$ if $x>1$, and $\infty$ if $0<x<1$
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
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(9).powr_prec(Float::from(0.5), 10);
    /// assert_eq!(p.to_string(), "3.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn powr_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.powr_prec_round_ref_ref(&other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and to the nearest value. The first [`Float`] is taken by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    #[inline]
    pub fn powr_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.powr_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and to the nearest value. The first [`Float`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn powr_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.powr_prec_round_ref_ref(&other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the specified precision and to the nearest value. Both [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded power is less
    /// than, equal to, or greater than the exact power. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the power is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    #[inline]
    pub fn powr_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.powr_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the maximum of the precisions of the inputs and with the specified rounding mode.
    /// Both [`Float`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded power is less than, equal to, or greater than the exact power. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y) = x^y+\varepsilon.
    /// $$
    /// - If $x^y$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^y$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p+1}$.
    /// - If $x^y$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |x^y|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// `powr(x, y)` is $e^{y\ln x}$; unlike [`pow`](Float::pow_prec_round), its base is restricted
    /// to $x\geq 0$ and it never produces a negative result.
    ///
    /// Special cases:
    /// - $f(x,y)=\text{NaN}$ if $x$ is `NaN`, if $x<0$, if $x$ is $\pm0$ or $\infty$ and $y=0$, or
    ///   if $x=1$ and $y$ is infinite
    /// - $f(x,0)=1.0$ if $x$ is finite and positive
    /// - $f(1.0,y)=1.0$ if $y$ is finite
    /// - $f(\infty,y)=\infty$ if $y>0$, and $0.0$ if $y<0$
    /// - $f(\pm0.0,y)=0.0$ if $y>0$, and $\infty$ if $y<0$
    /// - $f(x,\infty)=\infty$ if $x>1$, and $0.0$ if $0<x<1$
    /// - $f(x,-\infty)=0.0$ if $x>1$, and $\infty$ if $0<x<1$
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
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the output
    /// precision.
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (p, o) = Float::from(3).powr_round(Float::from(2.5), Floor);
    /// assert_eq!(p.to_string(), "14.0");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn powr_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.powr_prec_round_ref_ref(&other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the maximum of the precisions of the inputs and with the specified rounding mode.
    /// The first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    pub fn powr_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.powr_prec_round_ref_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the maximum of the precisions of the inputs and with the specified rounding mode.
    /// The first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded power is less than, equal to, or greater than the
    /// exact power. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    #[allow(clippy::needless_pass_by_value)]
    pub fn powr_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.powr_prec_round_ref_ref(&other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power using the IEEE 754 `powr` function, rounding the
    /// result to the maximum of the precisions of the inputs and with the specified rounding mode.
    /// Both [`Float`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded power is less than, equal to, or greater than the exact power. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    pub fn powr_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits().max(other.significant_bits());
        self.powr_prec_round_ref_ref(other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power in place using the IEEE 754 `powr` function, taking
    /// the exponent by value.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    #[allow(clippy::needless_pass_by_value)]
    pub fn powr_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = self.powr_prec_round_ref_ref(&other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to a [`Float`] power in place using the IEEE 754 `powr` function, taking
    /// the exponent by reference.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    pub fn powr_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = self.powr_prec_round_ref_ref(other, prec, rm);
        *self = result;
        o
    }

    /// Raises a [`Float`] to a [`Float`] power in place using the IEEE 754 `powr` function, taking
    /// the exponent by value.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn powr_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.powr_prec_round_assign(other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power in place using the IEEE 754 `powr` function, taking
    /// the exponent by reference.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    #[inline]
    pub fn powr_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.powr_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Raises a [`Float`] to a [`Float`] power in place using the IEEE 754 `powr` function, taking
    /// the exponent by value.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the output
    /// precision.
    #[allow(clippy::needless_pass_by_value)]
    pub fn powr_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits().max(other.significant_bits());
        self.powr_prec_round_assign(other, prec, rm)
    }

    /// Raises a [`Float`] to a [`Float`] power in place using the IEEE 754 `powr` function, taking
    /// the exponent by reference.
    ///
    /// See the [`Float::powr_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n (\log n)^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the output
    /// precision.
    pub fn powr_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits().max(other.significant_bits());
        self.powr_prec_round_assign_ref(other, prec, rm)
    }
}
