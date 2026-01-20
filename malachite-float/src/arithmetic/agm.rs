// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 1999-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::basic::extended::{ExtendedFloat, agm_prec_round_normal_extended};
use crate::{
    Float, emulate_float_float_to_float_fn, emulate_rational_rational_to_float_fn,
    float_either_infinity, float_either_zero, float_infinity, float_nan, float_zero, test_overflow,
    test_underflow,
};
use alloc::borrow::Cow;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    Agm, AgmAssign, CeilingLogBase2, ShrRoundAssign, Sign, Sqrt, SqrtAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero as ZeroTrait;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::natural::arithmetic::float_sub::exponent_shift_compare;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// This is mpfr_cmp2 from cmp2.c, MPFR 4.3.0.
fn cmp2_helper(b: &Float, c: &Float, cancel: &mut u64) -> Ordering {
    match (b, c) {
        (
            Float(Finite {
                exponent: x_exp,
                precision: x_prec,
                significand: x,
                ..
            }),
            Float(Finite {
                exponent: y_exp,
                precision: y_prec,
                significand: y,
                ..
            }),
        ) => {
            let (o, c) = exponent_shift_compare(
                x.as_limbs_asc(),
                i64::from(*x_exp),
                *x_prec,
                y.as_limbs_asc(),
                i64::from(*y_exp),
                *y_prec,
            );
            *cancel = c;
            o
        }
        _ => panic!(),
    }
}

fn agm_prec_round_normal(
    mut a: Float,
    mut b: Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if a < 0u32 || b < 0u32 {
        return (float_nan!(), Equal);
    }
    let mut working_prec = prec + prec.ceiling_log_base_2() + 15;
    // b (op2) and a (op1) are the 2 operands but we want b >= a
    match a.partial_cmp(&b).unwrap() {
        Equal => return Float::from_float_prec_round(a, prec, rm),
        Greater => swap(&mut a, &mut b),
        _ => {}
    }
    let mut scaleop = 0;
    let mut increment = Limb::WIDTH;
    let mut v;
    let mut scaleit;
    loop {
        let mut err: u64 = 0;
        let mut u;
        loop {
            let u_o;
            let v_o;
            (u, u_o) = a.mul_prec_ref_ref(&b, working_prec);
            (v, v_o) = a.add_prec_ref_ref(&b, working_prec);
            let u_overflow = test_overflow(&u, u_o);
            let v_overflow = test_overflow(&v, v_o);
            if u_overflow || v_overflow || test_underflow(&u, u_o) || test_underflow(&v, v_o) {
                assert_eq!(scaleop, 0);
                let e1 = a.get_exponent().unwrap();
                let e2 = b.get_exponent().unwrap();
                if u_overflow || v_overflow {
                    // Let's recall that emin <= e1 <= e2 <= emax. There has been an overflow. Thus
                    // e2 >= emax/2. If the mpfr_mul overflowed, then e1 + e2 > emax. If the
                    // mpfr_add overflowed, then e2 = emax. We want: (e1 + scale) + (e2 + scale) <=
                    // emax, i.e. scale <= (emax - e1 - e2) / 2. Let's take scale = min(floor((emax
                    // - e1 - e2) / 2), -1). This is OK, as:
                    // ```
                    // - emin <= scale <= -1.
                    // - e1 + scale >= emin. Indeed:
                    //    * If e1 + e2 > emax, then
                    //      e1 + scale >= e1 + (emax - e1 - e2) / 2 - 1
                    //                 >= (emax + e1 - emax) / 2 - 1
                    //                 >= e1 / 2 - 1 >= emin.
                    //    * Otherwise, mpfr_mul didn't overflow, therefore
                    //      mpfr_add overflowed and e2 = emax, so that
                    //      e1 > emin (see restriction below).
                    //      e1 + scale > emin - 1, thus e1 + scale >= emin.
                    // - e2 + scale <= emax, since scale < 0.
                    // ```
                    let e_agm = e1 + e2;
                    if e_agm > Float::MAX_EXPONENT {
                        scaleop = -((e_agm - Float::MAX_EXPONENT + 1) / 2);
                        assert!(scaleop < 0);
                    } else {
                        // The addition necessarily overflowed.
                        assert_eq!(e2, Float::MAX_EXPONENT);
                        // The case where e1 = emin and e2 = emax is not supported here. This would
                        // mean that the precision of e2 would be huge (and possibly not supported
                        // in practice anyway).
                        assert!(e1 > Float::MIN_EXPONENT);
                        // Note: this case is probably impossible to have in practice since we need
                        // e2 = emax, and no overflow in the product. Since the product is >=
                        // 2^(e1+e2-2), it implies e1 + e2 - 2 <= emax, thus e1 <= 2. Now to get an
                        // overflow we need op1 >= 1/2 ulp(op2), which implies that the precision of
                        // op2 should be at least emax-2. On a 64-bit computer this is impossible to
                        // have, and would require a huge amount of memory on a 32-bit computer.
                        scaleop = -1;
                    }
                } else {
                    // underflow only (in the multiplication)
                    //
                    // We have e1 + e2 <= emin (so, e1 <= e2 <= 0). We want: (e1 + scale) + (e2 +
                    // scale) >= emin + 1, i.e. scale >= (emin + 1 - e1 - e2) / 2. let's take scale
                    // = ceil((emin + 1 - e1 - e2) / 2). This is OK, as: 1. 1 <= scale <= emax. 2.
                    // e1 + scale >= emin + 1 >= emin. 3. e2 + scale <= scale <= emax.
                    assert!(e1 <= e2 && e2 <= 0);
                    scaleop = (Float::MIN_EXPONENT + 2 - e1 - e2) / 2;
                    assert!(scaleop > 0);
                }
                a <<= scaleop;
                b <<= scaleop;
            } else {
                break;
            }
        }
        u.sqrt_assign();
        v >>= 1u32;
        scaleit = 0;
        let mut n: u64 = 1;
        let mut eq = 0;
        'mid: while cmp2_helper(&u, &v, &mut eq) != Equal && eq <= working_prec - 2 {
            let mut uf;
            let mut vf;
            loop {
                vf = (&u + &v) >> 1;
                // See proof in algorithms.tex
                if eq > working_prec >> 2 {
                    // vf = V(k)
                    let low_p = (working_prec + 1) >> 1;
                    let (mut w, o) = v.sub_prec_ref_ref(&u, low_p); // e = V(k-1)-U(k-1)
                    let mut underflow = test_underflow(&w, o);
                    let o = w.square_round_assign(Nearest); // e = e^2
                    underflow |= test_underflow(&w, o);
                    let o = w.shr_round_assign(4, Nearest); // e*= (1/2)^2*1/4
                    underflow |= test_underflow(&w, o);
                    let o = w.div_prec_assign_ref(&vf, low_p); // 1/4*e^2/V(k)
                    underflow |= test_underflow(&w, o);
                    let vf_exp = vf.get_exponent().unwrap();
                    if !underflow {
                        v = vf.sub_prec(w, working_prec).0;
                        // 0 or 1
                        err = u64::exact_from(vf_exp - v.get_exponent().unwrap());
                        break 'mid;
                    }
                    // There has been an underflow because of the cancellation between V(k-1) and
                    // U(k-1). Let's use the conventional method.
                }
                // U(k) increases, so that U.V can overflow (but not underflow).
                uf = &u * &v;
                // For multiplication using Nearest, is_infinite is sufficient for overflow checking
                if uf.is_infinite() {
                    let scale2 = -(((u.get_exponent().unwrap() + v.get_exponent().unwrap())
                        - Float::MAX_EXPONENT
                        + 1)
                        / 2);
                    u <<= scale2;
                    v <<= scale2;
                    scaleit += scale2;
                } else {
                    break;
                }
            }
            u = uf.sqrt();
            swap(&mut v, &mut vf);
            n += 1;
        }
        // the error on v is bounded by (18n+51) ulps, or twice if there was an exponent loss in the
        // final subtraction
        //
        // 18n+51 should not overflow since n is about log(p)
        err += (18 * n + 51).ceiling_log_base_2();
        // we should have n+2 <= 2^(p/4) [see algorithms.tex]
        if (n + 2).ceiling_log_base_2() <= working_prec >> 2
            && float_can_round(v.significand_ref().unwrap(), working_prec - err, prec, rm)
        {
            break;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
    v.shr_prec_round(scaleop + scaleit, prec, rm)
}

fn agm_prec_round_ref_ref_normal(
    a: &Float,
    b: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if *a < 0u32 || *b < 0u32 {
        return (float_nan!(), Equal);
    }
    let mut working_prec = prec + prec.ceiling_log_base_2() + 15;
    let mut a = Cow::Borrowed(a);
    let mut b = Cow::Borrowed(b);
    // b (op2) and a (op1) are the 2 operands but we want b >= a
    match a.partial_cmp(&b).unwrap() {
        Equal => return Float::from_float_prec_round_ref(a.as_ref(), prec, rm),
        Greater => swap(&mut a, &mut b),
        _ => {}
    }
    let mut scaleop = 0;
    let mut increment = Limb::WIDTH;
    let mut v;
    let mut scaleit;
    loop {
        let mut err: u64 = 0;
        let mut u;
        loop {
            let u_o;
            let v_o;
            (u, u_o) = a.mul_prec_ref_ref(&b, working_prec);
            (v, v_o) = a.add_prec_ref_ref(&b, working_prec);
            let u_overflow = test_overflow(&u, u_o);
            let v_overflow = test_overflow(&v, v_o);
            if u_overflow || v_overflow || test_underflow(&u, u_o) || test_underflow(&v, v_o) {
                assert_eq!(scaleop, 0);
                let e1 = a.get_exponent().unwrap();
                let e2 = b.get_exponent().unwrap();
                if u_overflow || v_overflow {
                    // Let's recall that emin <= e1 <= e2 <= emax. There has been an overflow. Thus
                    // e2 >= emax/2. If the mpfr_mul overflowed, then e1 + e2 > emax. If the
                    // mpfr_add overflowed, then e2 = emax. We want: (e1 + scale) + (e2 + scale) <=
                    // emax, i.e. scale <= (emax - e1 - e2) / 2. Let's take scale = min(floor((emax
                    // - e1 - e2) / 2), -1). This is OK, as:
                    // ```
                    // - emin <= scale <= -1.
                    // - e1 + scale >= emin. Indeed:
                    //    * If e1 + e2 > emax, then
                    //      e1 + scale >= e1 + (emax - e1 - e2) / 2 - 1
                    //                 >= (emax + e1 - emax) / 2 - 1
                    //                 >= e1 / 2 - 1 >= emin.
                    //    * Otherwise, mpfr_mul didn't overflow, therefore
                    //      mpfr_add overflowed and e2 = emax, so that
                    //      e1 > emin (see restriction below).
                    //      e1 + scale > emin - 1, thus e1 + scale >= emin.
                    // - e2 + scale <= emax, since scale < 0.
                    // ```
                    let e_agm = e1 + e2;
                    if e_agm > Float::MAX_EXPONENT {
                        scaleop = -((e_agm - Float::MAX_EXPONENT + 1) / 2);
                        assert!(scaleop < 0);
                    } else {
                        // The addition necessarily overflowed.
                        assert_eq!(e2, Float::MAX_EXPONENT);
                        // The case where e1 = emin and e2 = emax is not supported here. This would
                        // mean that the precision of e2 would be huge (and possibly not supported
                        // in practice anyway).
                        assert!(e1 > Float::MIN_EXPONENT);
                        // Note: this case is probably impossible to have in practice since we need
                        // e2 = emax, and no overflow in the product. Since the product is >=
                        // 2^(e1+e2-2), it implies e1 + e2 - 2 <= emax, thus e1 <= 2. Now to get an
                        // overflow we need op1 >= 1/2 ulp(op2), which implies that the precision of
                        // op2 should be at least emax-2. On a 64-bit computer this is impossible to
                        // have, and would require a huge amount of memory on a 32-bit computer.
                        scaleop = -1;
                    }
                } else {
                    // underflow only (in the multiplication)
                    //
                    // We have e1 + e2 <= emin (so, e1 <= e2 <= 0). We want: (e1 + scale) + (e2 +
                    // scale) >= emin + 1, i.e. scale >= (emin + 1 - e1 - e2) / 2. let's take scale
                    // = ceil((emin + 1 - e1 - e2) / 2). This is OK, as: 1. 1 <= scale <= emax. 2.
                    // e1 + scale >= emin + 1 >= emin. 3. e2 + scale <= scale <= emax.
                    assert!(e1 <= e2 && e2 <= 0);
                    scaleop = (Float::MIN_EXPONENT + 2 - e1 - e2) / 2;
                    assert!(scaleop > 0);
                }
                *a.to_mut() <<= scaleop;
                *b.to_mut() <<= scaleop;
            } else {
                break;
            }
        }
        u.sqrt_assign();
        v >>= 1u32;
        scaleit = 0;
        let mut n: u64 = 1;
        let mut eq = 0;
        'mid: while cmp2_helper(&u, &v, &mut eq) != Equal && eq <= working_prec - 2 {
            let mut uf;
            let mut vf;
            loop {
                vf = (&u + &v) >> 1;
                // See proof in algorithms.tex
                if eq > working_prec >> 2 {
                    // vf = V(k)
                    let low_p = (working_prec + 1) >> 1;
                    let (mut w, o) = v.sub_prec_ref_ref(&u, low_p); // e = V(k-1)-U(k-1)
                    let mut underflow = test_underflow(&w, o);
                    let o = w.square_round_assign(Nearest); // e = e^2
                    underflow |= test_underflow(&w, o);
                    let o = w.shr_round_assign(4, Nearest); // e*= (1/2)^2*1/4
                    underflow |= test_underflow(&w, o);
                    let o = w.div_prec_assign_ref(&vf, low_p); // 1/4*e^2/V(k)
                    underflow |= test_underflow(&w, o);
                    let vf_exp = vf.get_exponent().unwrap();
                    if !underflow {
                        v = vf.sub_prec(w, working_prec).0;
                        // 0 or 1
                        err = u64::exact_from(vf_exp - v.get_exponent().unwrap());
                        break 'mid;
                    }
                    // There has been an underflow because of the cancellation between V(k-1) and
                    // U(k-1). Let's use the conventional method.
                }
                // U(k) increases, so that U.V can overflow (but not underflow).
                uf = &u * &v;
                // For multiplication using Nearest, is_infinite is sufficient for overflow checking
                if uf.is_infinite() {
                    let scale2 = -(((u.get_exponent().unwrap() + v.get_exponent().unwrap())
                        - Float::MAX_EXPONENT
                        + 1)
                        / 2);
                    u <<= scale2;
                    v <<= scale2;
                    scaleit += scale2;
                } else {
                    break;
                }
            }
            u = uf.sqrt();
            swap(&mut v, &mut vf);
            n += 1;
        }
        // the error on v is bounded by (18n+51) ulps, or twice if there was an exponent loss in the
        // final subtraction
        //
        // 18n+51 should not overflow since n is about log(p)
        err += (18 * n + 51).ceiling_log_base_2();
        // we should have n+2 <= 2^(p/4) [see algorithms.tex]
        if (n + 2).ceiling_log_base_2() <= working_prec >> 2
            && float_can_round(v.significand_ref().unwrap(), working_prec - err, prec, rm)
        {
            break;
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
    v.shr_prec_round(scaleop + scaleit, prec, rm)
}

fn agm_rational_helper(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        let (y_lo, y_o) = Float::from_rational_prec_round_ref(y, working_prec, Floor);
        if x_o == Equal && y_o == Equal {
            return agm_prec_round_normal(x_lo, y_lo, prec, rm);
        }
        let mut x_hi = x_lo.clone();
        if x_o != Equal {
            x_hi.increment();
        }
        let mut y_hi = y_lo.clone();
        if y_o != Equal {
            y_hi.increment();
        }
        let (agm_lo, mut o_lo) = agm_prec_round_normal(x_lo, y_lo, prec, rm);
        let (agm_hi, mut o_hi) = agm_prec_round_normal(x_hi, y_hi, prec, rm);
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && agm_lo == agm_hi {
            return (agm_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

fn agm_rational_helper_extended(
    x: &Rational,
    y: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = ExtendedFloat::from_rational_prec_round_ref(x, working_prec, Floor);
        let (y_lo, y_o) = ExtendedFloat::from_rational_prec_round_ref(y, working_prec, Floor);
        if x_o == Equal && y_o == Equal {
            let (agm, o) = agm_prec_round_normal_extended(x_lo, y_lo, prec, rm);
            return agm.into_float_helper(prec, rm, o);
        }
        let mut x_hi = x_lo.clone();
        if x_o != Equal {
            x_hi.increment();
        }
        let mut y_hi = y_lo.clone();
        if y_o != Equal {
            y_hi.increment();
        }
        let (agm_lo, mut o_lo) = agm_prec_round_normal_extended(x_lo, y_lo, prec, rm);
        let (agm_hi, mut o_hi) = agm_prec_round_normal_extended(x_hi, y_hi, prec, rm);
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && agm_lo == agm_hi {
            return agm_lo.into_float_helper(prec, rm, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`Float`]s are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded AGM is less than,
    /// equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(-\infty,x,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p,m)=\infty$
    /// - $f(\pm0.0,x,p,m)=f(x,\pm0.0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_prec`] instead. If you
    /// know that your target precision is the maximum of the precisions of the two inputs, consider
    /// using [`Float::agm_round`] instead. If both of these things are true, consider using
    /// [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round(Float::from(6), 5, Floor);
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round(Float::from(6), 5, Ceiling);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round(Float::from(6), 5, Nearest);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round(Float::from(6), 20, Floor);
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round(Float::from(6), 20, Ceiling);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round(Float::from(6), 20, Nearest);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (&self, &other) {
            (float_nan!(), _) | (_, float_nan!()) => (float_nan!(), Equal),
            (float_infinity!(), x) | (x, float_infinity!()) if *x > 0.0 => {
                (float_infinity!(), Equal)
            }
            (float_either_infinity!(), _) | (_, float_either_infinity!()) => (float_nan!(), Equal),
            (float_either_zero!(), _) | (_, float_either_zero!()) => (float_zero!(), Equal),
            _ => agm_prec_round_normal(self, other, prec, rm),
        }
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// value and the second by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded AGM is less than, equal to, or greater than the exact AGM. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(-\infty,x,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p,m)=\infty$
    /// - $f(\pm0.0,x,p,m)=f(x,\pm0.0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_prec_val_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::agm_round_val_ref`] instead. If both of these things are true,
    /// consider using [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_val_ref(&Float::from(6), 5, Floor);
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_val_ref(&Float::from(6), 5, Ceiling);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_val_ref(&Float::from(6), 5, Nearest);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_val_ref(&Float::from(6), 20, Floor);
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_val_ref(&Float::from(6), 20, Ceiling);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_val_ref(&Float::from(6), 20, Nearest);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_round_val_ref(
        mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = self.agm_prec_round_assign_ref(other, prec, rm);
        (self, o)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. The first [`Float`] is taken by
    /// reference and the second by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded AGM is less than, equal to, or greater than the exact AGM. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(-\infty,x,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p,m)=\infty$
    /// - $f(\pm0.0,x,p,m)=f(x,\pm0.0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_prec_ref_val`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::agm_round_ref_val`] instead. If both of these things are true,
    /// consider using [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_val_ref(&Float::from(6), 5, Floor);
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_val(Float::from(6), 5, Ceiling);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_val(Float::from(6), 5, Nearest);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_val(Float::from(6), 20, Floor);
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_val(Float::from(6), 20, Ceiling);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_val(Float::from(6), 20, Nearest);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_round_ref_val(
        &self,
        mut other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o = other.agm_prec_round_assign_ref(self, prec, rm);
        (other, o)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// specified precision and with the specified rounding mode. Both [`Float`]s are taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded AGM is less
    /// than, equal to, or greater than the exact AGM. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(x,\text{NaN},p,m)=f(-\infty,x,p,m)=f(x,-\infty,p,m)=\text{NaN}$
    /// - $f(\infty,x,p,m)=f(x,\infty,p,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p,m)=\infty$
    /// - $f(\pm0.0,x,p,m)=f(x,\pm0.0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_prec_ref_ref`] instead.
    /// If you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::agm_round_ref_ref`] instead. If both of these things are true,
    /// consider using [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_ref(&Float::from(6), 5, Floor);
    /// assert_eq!(agm.to_string(), "13.0");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_ref(&Float::from(6), 5, Ceiling);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_ref(&Float::from(6), 5, Nearest);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_ref(&Float::from(6), 20, Floor);
    /// assert_eq!(agm.to_string(), "13.45816");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_ref(&Float::from(6), 20, Ceiling);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_round_ref_ref(&Float::from(6), 20, Nearest);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    ///
    /// This is mpfr_agm from agm.c, MPFR 4.3.0.
    #[inline]
    pub fn agm_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (self, other) {
            (float_nan!(), _) | (_, float_nan!()) => (float_nan!(), Equal),
            (float_infinity!(), x) | (x, float_infinity!()) if *x > 0.0 => {
                (float_infinity!(), Equal)
            }
            (float_either_infinity!(), _) | (_, float_either_infinity!()) => (float_nan!(), Equal),
            (float_either_zero!(), _) | (_, float_either_zero!()) => (float_zero!(), Equal),
            _ => agm_prec_round_ref_ref_normal(self, other, prec, rm),
        }
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// nearest value of the specified precision. Both [`Float`]s are taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the agm is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(-\infty,x,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p)=\infty$
    /// - $f(\pm0.0,x,p)=f(x,\pm0.0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_round`] instead. If you know that your target precision is the maximum of
    /// the precisions of the two inputs, consider using [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (agm, o) = Float::from(24).agm_prec(Float::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec(Float::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.agm_prec_round(other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] is taken by value and the
    /// second by reference. An [`Ordering`] is also returned, indicating whether the rounded AGM is
    /// less than, equal to, or greater than the exact AGM. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the agm is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(-\infty,x,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p)=\infty$
    /// - $f(\pm0.0,x,p)=f(x,\pm0.0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_round_val_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (agm, o) = Float::from(24).agm_prec_val_ref(&Float::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from(24).agm_prec_val_ref(&Float::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.agm_prec_round_val_ref(other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// nearest value of the specified precision. The first [`Float`] is taken by reference and the
    /// second by value. An [`Ordering`] is also returned, indicating whether the rounded AGM is
    /// less than, equal to, or greater than the exact AGM. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the agm is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(-\infty,x,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p)=\infty$
    /// - $f(\pm0.0,x,p)=f(x,\pm0.0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_round_ref_val`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (agm, o) = (&Float::from(24)).agm_prec_ref_val(Float::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&Float::from(24)).agm_prec_ref_val(Float::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.agm_prec_round_ref_val(other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result to the
    /// nearest value of the specified precision. Both [`Float`]s are taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the agm is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(x,\text{NaN},p)=f(-\infty,x,p)=f(x,-\infty,p)=\text{NaN}$
    /// - $f(\infty,x,p)=f(x,\infty,p)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,p)=\infty$
    /// - $f(\pm0.0,x,p)=f(x,\pm0.0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_round_ref_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (agm, o) = (&Float::from(24)).agm_prec_ref_ref(&Float::from(6), 5);
    /// assert_eq!(agm.to_string(), "13.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = (&Float::from(24)).agm_prec_ref_ref(&Float::from(6), 20);
    /// assert_eq!(agm.to_string(), "13.45818");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.agm_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result with the
    /// specified rounding mode. Both [`Float`]s are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(-\infty,x,m)=f(x,-\infty,m)=\text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,m)=\infty$
    /// - $f(\pm0.0,x,m)=f(x,\pm0.0,m)=0.0$
    /// - $f(x,y,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::agm_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round(Float::from(6), Floor);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315696");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round(Float::from(6), Ceiling);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round(Float::from(6), Nearest);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round(other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by value and the second by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to,
    /// or greater than the exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(-\infty,x,m)=f(x,-\infty,m)=\text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,m)=\infty$
    /// - $f(\pm0.0,x,m)=f(x,\pm0.0,m)=0.0$
    /// - $f(x,y,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::agm_prec_round_val_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round_val_ref(&Float::from(6), Floor);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315696");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round_val_ref(&Float::from(6), Ceiling);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round_val_ref(&Float::from(6), Nearest);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_val_ref(other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result with the
    /// specified rounding mode. The first [`Float`] is taken by reference and the second by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to,
    /// or greater than the exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(-\infty,x,m)=f(x,-\infty,m)=\text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,m)=\infty$
    /// - $f(\pm0.0,x,m)=f(x,\pm0.0,m)=0.0$
    /// - $f(x,y,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::agm_prec_round_ref_val`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) =
    ///     (&Float::from_unsigned_prec(24u8, 100).0).agm_round_ref_val(Float::from(6), Floor);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315696");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) =
    ///     (&Float::from_unsigned_prec(24u8, 100).0).agm_round_ref_val(Float::from(6), Ceiling);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) =
    ///     (&Float::from_unsigned_prec(24u8, 100).0).agm_round_ref_val(Float::from(6), Nearest);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_ref_val(other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, rounding the result with the
    /// specified rounding mode. Both [`Float`]s are taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,m)=f(x,\text{NaN},m)=f(-\infty,x,m)=f(x,-\infty,m)=\text{NaN}$
    /// - $f(\infty,x,m)=f(x,\infty,m)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty,m)=\infty$
    /// - $f(\pm0.0,x,m)=f(x,\pm0.0,m)=0.0$
    /// - $f(x,y,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using [`Float::agm_prec_round_ref_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::agm`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round_ref_ref(&Float::from(6), Floor);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315696");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round_ref_ref(&Float::from(6), Ceiling);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::from_unsigned_prec(24u8, 100)
    ///     .0
    ///     .agm_round_ref_ref(&Float::from(6), Nearest);
    /// assert_eq!(agm.to_string(), "13.45817148172561542076681315698");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded AGM is less than, equal to, or greater than the exact AGM.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::agm_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_prec_assign`] instead. If
    /// you know that your target precision is the maximum of the precisions of the two inputs,
    /// consider using [`Float::agm_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::agm_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_round_assign(Float::from(6), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "13.0");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_round_assign(Float::from(6), 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_round_assign(Float::from(6), 5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_round_assign(Float::from(6), 20, Floor), Less);
    /// assert_eq!(x.to_string(), "13.45816");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(Float::from(6), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.45818");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign(Float::from(6), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.45818");
    /// ```
    #[inline]
    pub fn agm_prec_round_assign(&mut self, other: Self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        let mut x = Self::ZERO;
        swap(&mut x, self);
        (*self, o) = x.agm_prec_round(other, prec, rm);
        o
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and rounding the result to the specified precision and with the specified rounding
    /// mode. The [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::agm_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_prec_assign_ref`]
    /// instead. If you know that your target precision is the maximum of the precisions of the two
    /// inputs, consider using [`Float::agm_round_assign_ref`] instead. If both of these things are
    /// true, consider using [`Float::agm_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_round_assign_ref(&Float::from(6), 5, Floor), Less);
    /// assert_eq!(x.to_string(), "13.0");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&Float::from(6), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&Float::from(6), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&Float::from(6), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(x.to_string(), "13.45816");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&Float::from(6), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.45818");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(
    ///     x.agm_prec_round_assign_ref(&Float::from(6), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "13.45818");
    /// ```
    #[inline]
    pub fn agm_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let o;
        (*self, o) = self.agm_prec_round_ref_ref(other, prec, rm);
        o
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and rounding the result to the nearest value of the specified precision. The
    /// [`Float`] on the right-hand side is taken by value. An [`Ordering`] is returned, indicating
    /// whether the rounded AGM is less than, equal to, or greater than the exact AGM. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets the [`Float`] to
    /// `NaN` it also returns `Equal`.
    ///
    /// If the agm is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::agm_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_round_assign`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::agm_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_assign(Float::from(6), 5), Greater);
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_assign(Float::from(6), 20), Greater);
    /// assert_eq!(x.to_string(), "13.45818");
    /// ```
    #[inline]
    pub fn agm_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.agm_prec_round_assign(other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and rounding the result to the nearest value of the specified precision. The
    /// [`Float`] on the right-hand side is taken by reference. An [`Ordering`] is returned,
    /// indicating whether the rounded AGM is less than, equal to, or greater than the exact AGM.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the agm is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::agm_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_round_assign_ref`] instead. If you know that your target precision is the
    /// maximum of the precisions of the two inputs, consider using [`Float::agm_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_assign_ref(&Float::from(6), 5), Greater);
    /// assert_eq!(x.to_string(), "13.5");
    ///
    /// let mut x = Float::from(24);
    /// assert_eq!(x.agm_prec_assign_ref(&Float::from(6), 20), Greater);
    /// assert_eq!(x.to_string(), "13.45818");
    /// ```
    #[inline]
    pub fn agm_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.agm_prec_round_assign_ref(other, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and rounding the result with the specified rounding mode. The [`Float`] on the
    /// right-hand side is taken by value. An [`Ordering`] is returned, indicating whether the
    /// rounded AGM is less than, equal to, or greater than the exact AGM. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::agm_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::agm_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::agm_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// assert_eq!(x.agm_round_assign(Float::from(6), Floor), Less);
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315696");
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// assert_eq!(x.agm_round_assign(Float::from(6), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315698");
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// assert_eq!(x.agm_round_assign(Float::from(6), Nearest), Greater);
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315698");
    /// ```
    #[inline]
    pub fn agm_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_assign(other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and rounding the result with the specified rounding mode. The [`Float`] on the
    /// right-hand side is taken by reference. An [`Ordering`] is returned, indicating whether the
    /// rounded AGM is less than, equal to, or greater than the exact AGM. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precision of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the
    ///   inputs.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs.
    ///
    /// See the [`Float::agm_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::agm_prec_round_assign_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::agm_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Float`] arguments are positive and distinct (and the
    /// exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// assert_eq!(x.agm_round_assign_ref(&Float::from(6), Floor), Less);
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315696");
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// assert_eq!(x.agm_round_assign_ref(&Float::from(6), Ceiling), Greater);
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315698");
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// assert_eq!(x.agm_round_assign_ref(&Float::from(6), Nearest), Greater);
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315698");
    /// ```
    #[inline]
    pub fn agm_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_assign_ref(other, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the specified precision and with the specified rounding mode, and returning the result as a
    /// [`Float`]. Both [`Rational`]s are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded AGM is less than, equal to, or greater than the exact AGM.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=f(x,0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,t,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec_round(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_rational_prec_round(
        x: Rational,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::agm_rational_prec_round_val_ref(x, &y, prec, rm)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the specified precision and with the specified rounding mode, and returning the result as a
    /// [`Float`]. The first [`Rational`]s is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=f(x,0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,t,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_rational_prec_val_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn agm_rational_prec_round_val_ref(
        x: Rational,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (x.sign(), y.sign()) {
            (Equal, _) | (_, Equal) => return (float_zero!(), Equal),
            (Less, _) | (_, Less) => return (float_nan!(), Equal),
            _ => {}
        }
        if x == *y {
            return Self::from_rational_prec_round(x, prec, rm);
        }
        assert_ne!(rm, Exact, "Inexact AGM");
        let x_exp = i32::saturating_from(x.floor_log_base_2_abs()).saturating_add(1);
        let y_exp = i32::saturating_from(y.floor_log_base_2_abs()).saturating_add(1);
        let x_overflow = x_exp > Self::MAX_EXPONENT;
        let y_overflow = y_exp > Self::MAX_EXPONENT;
        let x_underflow = x_exp < Self::MIN_EXPONENT;
        let y_underflow = y_exp < Self::MIN_EXPONENT;
        match (x_overflow, y_overflow, x_underflow, y_underflow) {
            (true, true, _, _) => Self::from_rational_prec_round(x, prec, rm),
            (_, _, true, true)
                if rm != Nearest
                    || x_exp < Self::MIN_EXPONENT - 1 && y_exp < Self::MIN_EXPONENT - 1 =>
            {
                Self::from_rational_prec_round(x, prec, rm)
            }
            (false, false, false, false)
                if x_exp < Self::MAX_EXPONENT && y_exp < Self::MAX_EXPONENT =>
            {
                agm_rational_helper(&x, y, prec, rm)
            }
            _ => agm_rational_helper_extended(&x, y, prec, rm),
        }
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the specified precision and with the specified rounding mode, and returning the result as a
    /// [`Float`]. The first [`Rational`]s is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded AGM is less than, equal to, or
    /// greater than the exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=f(x,0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,t,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_rational_prec_ref_val`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn agm_rational_prec_round_ref_val(
        x: &Rational,
        y: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (x.sign(), y.sign()) {
            (Equal, _) | (_, Equal) => return (float_zero!(), Equal),
            (Less, _) | (_, Less) => return (float_nan!(), Equal),
            _ => {}
        }
        if *x == y {
            return Self::from_rational_prec_round(y, prec, rm);
        }
        assert_ne!(rm, Exact, "Inexact AGM");
        let x_exp = i32::saturating_from(x.floor_log_base_2_abs()).saturating_add(1);
        let y_exp = i32::saturating_from(y.floor_log_base_2_abs()).saturating_add(1);
        let x_overflow = x_exp > Self::MAX_EXPONENT;
        let y_overflow = y_exp > Self::MAX_EXPONENT;
        let x_underflow = x_exp < Self::MIN_EXPONENT;
        let y_underflow = y_exp < Self::MIN_EXPONENT;
        match (x_overflow, y_overflow, x_underflow, y_underflow) {
            (true, true, _, _) => Self::from_rational_prec_round(y, prec, rm),
            (_, _, true, true)
                if rm != Nearest
                    || x_exp < Self::MIN_EXPONENT - 1 && y_exp < Self::MIN_EXPONENT - 1 =>
            {
                Self::from_rational_prec_round(y, prec, rm)
            }
            (false, false, false, false)
                if x_exp < Self::MAX_EXPONENT && y_exp < Self::MAX_EXPONENT =>
            {
                agm_rational_helper(x, &y, prec, rm)
            }
            _ => agm_rational_helper_extended(x, &y, prec, rm),
        }
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the specified precision and with the specified rounding mode, and returning the result as a
    /// [`Float`]. Both [`Rational`]s are taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded AGM is less than, equal to, or greater than the exact AGM.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p,m) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=f(x,0,p,m)=0.0$
    /// - $f(x,y,p,m)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,y,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`,
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,t,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,y,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::agm_rational_prec_ref_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985109");
    /// assert_eq!(o, Less);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    ///
    /// let (agm, o) = Float::agm_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn agm_rational_prec_round_ref_ref(
        x: &Rational,
        y: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match (x.sign(), y.sign()) {
            (Equal, _) | (_, Equal) => return (float_zero!(), Equal),
            (Less, _) | (_, Less) => return (float_nan!(), Equal),
            _ => {}
        }
        if x == y {
            return Self::from_rational_prec_round_ref(x, prec, rm);
        }
        assert_ne!(rm, Exact, "Inexact AGM");
        let x_exp = i32::saturating_from(x.floor_log_base_2_abs()).saturating_add(1);
        let y_exp = i32::saturating_from(y.floor_log_base_2_abs()).saturating_add(1);
        let x_overflow = x_exp > Self::MAX_EXPONENT;
        let y_overflow = y_exp > Self::MAX_EXPONENT;
        let x_underflow = x_exp < Self::MIN_EXPONENT;
        let y_underflow = y_exp < Self::MIN_EXPONENT;
        match (x_overflow, y_overflow, x_underflow, y_underflow) {
            (true, true, _, _) => Self::from_rational_prec_round_ref(x, prec, rm),
            (_, _, true, true)
                if rm != Nearest
                    || x_exp < Self::MIN_EXPONENT - 1 && y_exp < Self::MIN_EXPONENT - 1 =>
            {
                Self::from_rational_prec_round_ref(x, prec, rm)
            }
            (false, false, false, false)
                if x_exp < Self::MAX_EXPONENT && y_exp < Self::MAX_EXPONENT =>
            {
                agm_rational_helper(x, y, prec, rm)
            }
            _ => agm_rational_helper_extended(x, y, prec, rm),
        }
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the nearest value of the specified precision, and returning the result as a [`Float`]. Both
    /// [`Rational`]s are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded AGM is less than, equal to, or greater than the exact AGM. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p)=f(x,0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Up`, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn agm_rational_prec(x: Rational, y: Rational, prec: u64) -> (Self, Ordering) {
        Self::agm_rational_prec_round_val_ref(x, &y, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the nearest value of the specified precision, and returning the result as a [`Float`]. The
    /// first [`Rational`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p)=f(x,0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Up`, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_rational_prec_round_val_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec_val_ref(
    ///     Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_rational_prec_val_ref(x: Rational, y: &Rational, prec: u64) -> (Self, Ordering) {
        Self::agm_rational_prec_round_val_ref(x, y, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the nearest value of the specified precision, and returning the result as a [`Float`]. The
    /// first [`Rational`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded AGM is less than, equal to, or greater than the
    /// exact AGM. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p)=f(x,0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Up`, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_rational_prec_round_ref_val`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_rational_prec_ref_val(x: &Rational, y: Rational, prec: u64) -> (Self, Ordering) {
        Self::agm_rational_prec_round_ref_val(x, y, prec, Nearest)
    }

    /// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, rounding the result to
    /// the nearest value of the specified precision, and returning the result as a [`Float`]. Both
    /// [`Rational`]s are taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded AGM is less than, equal to, or greater than the exact AGM. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,p) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon|
    ///   < 2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p+1}$.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 \text{AGM}(x,y)\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p)=f(x,0,p)=0.0$
    /// - $f(x,y,p)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,y,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Up`, $-\infty$ is returned instead.
    /// - If $0<f(x,y,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,y,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,y,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_rational_prec_round_ref_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the two [`Rational`] arguments are positive and distinct (and
    /// the exact result is therefore irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (agm, o) = Float::agm_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(2u8, 3),
    ///     &Rational::from_unsigneds(1u8, 5),
    ///     20,
    /// );
    /// assert_eq!(agm.to_string(), "0.3985114");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn agm_rational_prec_ref_ref(x: &Rational, y: &Rational, prec: u64) -> (Self, Ordering) {
        Self::agm_rational_prec_round_ref_ref(x, y, prec, Nearest)
    }
}

impl Agm<Self> for Float {
    type Output = Self;

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, taking both by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the agm
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(-\infty,x)=f(x,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty)=\infty$
    /// - $f(\pm0.0,x)=f(x,\pm0.0)=0.0$
    /// - $f(x,y)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::agm_prec`]
    /// instead. If you want to specify the output precision, consider using [`Float::agm_round`].
    /// If you want both of these things, consider using [`Float::agm_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Agm;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from_unsigned_prec(24u8, 100)
    ///         .0
    ///         .agm(Float::from(6))
    ///         .to_string(),
    ///     "13.45817148172561542076681315698"
    /// );
    /// ```
    #[inline]
    fn agm(self, other: Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round(other, prec, Nearest).0
    }
}

impl Agm<&Self> for Float {
    type Output = Self;

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, taking the first by value
    /// and the second by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the agm
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(-\infty,x)=f(x,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty)=\infty$
    /// - $f(\pm0.0,x)=f(x,\pm0.0)=0.0$
    /// - $f(x,y)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_val_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::agm_round_val_ref`]. If you want both of these things, consider using
    /// [`Float::agm_prec_round_val_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Agm;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from_unsigned_prec(24u8, 100)
    ///         .0
    ///         .agm(&Float::from(6))
    ///         .to_string(),
    ///     "13.45817148172561542076681315698"
    /// );
    /// ```
    #[inline]
    fn agm(self, other: &Self) -> Self {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_val_ref(other, prec, Nearest).0
    }
}

impl Agm<Float> for &Float {
    type Output = Float;

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, taking the first by
    /// reference and the second by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the agm
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(-\infty,x)=f(x,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty)=\infty$
    /// - $f(\pm0.0,x)=f(x,\pm0.0)=0.0$
    /// - $f(x,y)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_ref_val`] instead. If you want to specify the output precision, consider
    /// using [`Float::agm_round_ref_val`]. If you want both of these things, consider using
    /// [`Float::agm_prec_round_ref_val`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Agm;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(24u8, 100).0)
    ///         .agm(Float::from(6))
    ///         .to_string(),
    ///     "13.45817148172561542076681315698"
    /// );
    /// ```
    #[inline]
    fn agm(self, other: Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_ref_val(other, prec, Nearest).0
    }
}

impl Agm<&Float> for &Float {
    type Output = Float;

    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, taking both by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the agm
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x)=f(x,\text{NaN})=f(-\infty,x)=f(x,-\infty)=\text{NaN}$
    /// - $f(\infty,x)=f(x,\infty)=\text{NaN}$ if $x\neq\infty$
    /// - $f(\infty,\infty)=\infty$
    /// - $f(\pm0.0,x)=f(x,\pm0.0)=0.0$
    /// - $f(x,y)=\text{NaN}$ if $x<0$ or $y<0$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_ref_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::agm_round_ref_ref`]. If you want both of these things, consider using
    /// [`Float::agm_prec_round_ref_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Agm;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(24u8, 100).0)
    ///         .agm(&Float::from(6))
    ///         .to_string(),
    ///     "13.45817148172561542076681315698"
    /// );
    /// ```
    #[inline]
    fn agm(self, other: &Float) -> Float {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_ref_ref(other, prec, Nearest).0
    }
}

impl AgmAssign<Self> for Float {
    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and taking the [`Float`] on the right-hand side by value.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the agm
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::agm`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::agm_round_assign`]. If you want both of these things, consider using
    /// [`Float::agm_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AgmAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// x.agm_assign(Float::from(6));
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315698");
    /// ```
    #[inline]
    fn agm_assign(&mut self, other: Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_assign(other, prec, Nearest);
    }
}

impl AgmAssign<&Self> for Float {
    /// Computes the arithmetic-geometric mean (AGM) of two [`Float`]s, mutating the first one in
    /// place, and taking the [`Float`] on the right-hand side by reference.
    ///
    /// If the output has a precision, it is the maximum of the precisions of the inputs. If the agm
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = \text{AGM}(x,y)+\varepsilon
    /// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
    /// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
    /// $$
    /// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::agm`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::agm_prec_assign_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::agm_round_assign_ref`]. If you want both of these things, consider
    /// using [`Float::agm_prec_round_assign_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`, and $m$ is `other.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AgmAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(24u8, 100).0;
    /// x.agm_assign(&Float::from(6));
    /// assert_eq!(x.to_string(), "13.45817148172561542076681315698");
    /// ```
    #[inline]
    fn agm_assign(&mut self, other: &Self) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.agm_prec_round_assign_ref(other, prec, Nearest);
    }
}

/// Computes the arithmetic-geometric mean (AGM) of two primitive floats.
///
/// $$
/// f(x,y) = \text{AGM}(x,y)+\varepsilon
/// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
/// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
/// $$
/// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
///   be 0.
/// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN},x)=f(x,\text{NaN})=f(-\infty,x)=f(x,-\infty)=\text{NaN}$
/// - $f(\infty,x)=f(x,\infty)=\text{NaN}$ if $x\neq\infty$
/// - $f(\infty,\infty)=\infty$
/// - $f(\pm0.0,x)=f(x,\pm0.0)=0.0$
/// - $f(x,y)=\text{NaN}$ if $x<0$ or $y<0$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::agm::primitive_float_agm;
///
/// assert_eq!(
///     NiceFloat(primitive_float_agm(24.0, 6.0)),
///     NiceFloat(13.458171481725616)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_agm<T: PrimitiveFloat>(x: T, y: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(Float::agm_prec, x, y)
}

/// Computes the arithmetic-geometric mean (AGM) of two [`Rational`]s, returning the result as a
/// primitive float.
///
/// $$
/// f(x,y) = \text{AGM}(x,y)+\varepsilon
/// =\frac{\pi}{2}\left(\int_0^{\frac{\pi}{2}}\frac{\mathrm{d}\theta}
/// {\sqrt{x^2\cos^2\theta+y^2\sin^2\theta}}\right)^{-1}+\varepsilon.
/// $$
/// - If $\text{AGM}(x,y)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
///   be 0.
/// - If $\text{AGM}(x,y)$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   \text{AGM}(x,y)\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0,x)=f(x,0)=0.0$
/// - $f(x,y)=\text{NaN}$ if $x<0$ or $y<0$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::agm::primitive_float_agm;
/// use malachite_float::arithmetic::agm::primitive_float_agm_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_agm_rational::<f64>(
///         &Rational::from_unsigneds(2u8, 3),
///         &Rational::from_unsigneds(1u8, 5)
///     )),
///     NiceFloat(0.3985113702200345)
/// );
/// ```
#[allow(clippy::type_repetition_in_bounds)]
#[inline]
pub fn primitive_float_agm_rational<T: PrimitiveFloat>(x: &Rational, y: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_rational_to_float_fn(Float::agm_rational_prec_ref_ref, x, y)
}
