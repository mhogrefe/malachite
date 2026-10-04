// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2005-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::round_near_x::round_near_reciprocal;
use crate::float::arithmetic::sech::reciprocal_hyperbolic_large;
use crate::float::arithmetic::tan::{MAX_SETTLED_EXPONENT, reciprocal_of_down_bracket};
use crate::{Float, emulate_float_to_float_fn};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{CeilingLogBase2, Csch, CschAssign, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// This is mpfr_csch from csch.c (an instantiation of gen_inverse.h), MPFR 4.2.2, where the input is
// finite and nonzero, with the scaled path for large inputs and the bracket path for results near
// the top of the exponent range.
fn csch_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact csch");
    let exp_x = i64::from(x.get_exponent().unwrap());
    // ACTION_TINY from csch.c: EXP(x) <= -2 max(PREC(x), PREC(y)). There csch x = 1/x - x/6 + ...,
    // and |csch x - 1/x| <= |x|/6 for |x| <= 1, with the correction opposing the sign of 1/x, so
    // that the hyperbolic cosecant lies just short of 1/x.
    let n = i64::exact_from(max(x.get_prec().unwrap(), prec));
    if exp_x <= -(n << 1) {
        return round_near_reciprocal(x, false, prec, rm);
    }
    if let Some(result) = reciprocal_hyperbolic_large(x, false, prec, rm) {
        return result;
    }
    // |x| < 2^29, so sinh(x) < exp(2^29) < 2^(2^30 - 1) cannot overflow, and |csch(x)| > 2^(-2^30)
    // is well above the bottom of the exponent range.
    let mut working_prec = prec + prec.ceiling_log_base_2() + 3;
    let mut increment = Limb::WIDTH;
    loop {
        // the sinh has an error below 1 ulp, and rounding toward zero fixes its sign
        let s = x.sinh_prec_round_ref(working_prec, Down).0;
        // the error is less than c_w + 2*c_u*k_u (see algorithms.tex), where c_w = 1/2, c_u = 1
        // since the sinh was rounded toward zero, thus 1/2 + 2 < 4
        let r = (&s).reciprocal();
        // A reciprocal whose exponent is below MAX_SETTLED_EXPONENT can be rounded to any precision
        // without leaving the exponent range, so the `Float` reciprocal settles it; the rest go to
        // the bracket. Since |sinh(x)| >= |x|, which is at least the smallest positive `Float`,
        // these are only reached for an x near the bottom of the exponent range that is not tiny,
        // which requires a precision of about 2^29 bits.
        match r.get_exponent().map(i64::from) {
            Some(e) if e < MAX_SETTLED_EXPONENT => {
                if float_can_round(r.significand_ref().unwrap(), working_prec - 2, prec, rm) {
                    return Float::from_float_prec_round(r, prec, rm);
                }
            }
            _ => {
                fail_on_untested_path(
                    "csch_prec_round_normal_ref, result near the top of the range",
                );
                if let Some(result) = reciprocal_of_down_bracket(&s, working_prec, prec, rm) {
                    return result;
                }
            }
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded hyperbolic
    /// cosecant is less than, equal to, or greater than the exact hyperbolic cosecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p+1}$.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow happens only for inputs of magnitude at most about $2^{-2^{30}+1}$, and underflow
    /// for inputs of magnitude above about $7.4\times10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csch_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::csch_round`] instead. If both of these things are true, consider using
    /// [`Float::csch`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosecant of
    /// a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.85091782");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.85091877");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.85091782");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csch_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.csch_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// hyperbolic cosecant is less than, equal to, or greater than the exact hyperbolic cosecant.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p+1}$.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=-0.0$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=-\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\leq -2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Overflow happens only for inputs of magnitude at most about $2^{-2^{30}+1}$, and underflow
    /// for inputs of magnitude above about $7.4\times10^8$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csch_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::csch_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).csch()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosecant of
    /// a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.85091782");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.85091877");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.85091782");
    /// assert_eq!(o, Less);
    /// ```
    pub fn csch_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN => (Self::NAN, Equal),
            // csch(+Inf) = +0, csch(-Inf) = -0
            Infinity { sign } => (
                if *sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                },
                Equal,
            ),
            // csch(+0) = +Inf, csch(-0) = -Inf
            Zero { sign } => (
                if *sign {
                    Self::INFINITY
                } else {
                    Self::NEGATIVE_INFINITY
                },
                Equal,
            ),
            Finite { .. } => csch_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic cosecant is less
    /// than, equal to, or greater than the exact hyperbolic cosecant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cosecant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{csch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csch_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::csch`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_prec(5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_prec(20);
    /// assert_eq!(c.to_string(), "0.85091782");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csch_prec(self, prec: u64) -> (Self, Ordering) {
        self.csch_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded hyperbolic cosecant is less
    /// than, equal to, or greater than the exact hyperbolic cosecant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cosecant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{csch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=-0.0$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=-\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\leq -2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csch_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).csch()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.85091782");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn csch_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.csch_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded hyperbolic cosecant is less than, equal to, or
    /// greater than the exact hyperbolic cosecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// See the [`Float::csch_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::csch_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::csch`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosecant of
    /// a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_round(Floor);
    /// assert_eq!(c.to_string(), "0.85091812823932154513384276328642");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.85091812823932154513384276328721");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_round(Nearest);
    /// assert_eq!(c.to_string(), "0.85091812823932154513384276328721");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csch_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.csch_prec_round(prec, rm)
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded hyperbolic cosecant is less
    /// than, equal to, or greater than the exact hyperbolic cosecant. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=-0.0$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=-\infty$
    ///
    /// See the [`Float::csch_prec_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::csch_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).csch()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosecant of
    /// a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.csch_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.85091812823932154513384276328642");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.85091812823932154513384276328721");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .csch_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.85091812823932154513384276328721");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn csch_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.csch_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], in place, rounding
    /// the result to the specified precision and with the specified rounding mode. An [`Ordering`]
    /// is returned, indicating whether the rounded hyperbolic cosecant is less than, equal to, or
    /// greater than the exact hyperbolic cosecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p+1}$.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::csch_prec_round`] documentation for information on special cases and
    /// overflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::csch_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::csch_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::csch_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosecant of
    /// a finite nonzero [`Float`] is never exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.875");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.85091782");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.85091877");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.85091782");
    /// ```
    #[inline]
    pub fn csch_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.csch_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], in place, rounding
    /// the result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded hyperbolic cosecant is less than, equal to, or greater than
    /// the exact hyperbolic cosecant. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the hyperbolic cosecant is equidistant from two [`Float`]s with the specified precision,
    /// the [`Float`] with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a
    /// description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{csch} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::csch_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csch_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::csch_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.85091782");
    /// ```
    #[inline]
    pub fn csch_prec_assign(&mut self, prec: u64) -> Ordering {
        self.csch_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], in place, rounding
    /// the result with the specified rounding mode. An [`Ordering`] is returned, indicating whether
    /// the rounded hyperbolic cosecant is less than, equal to, or greater than the exact hyperbolic
    /// cosecant. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{csch} x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{csch} x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::csch_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::csch_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::csch_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and `self` is finite and nonzero, since the hyperbolic cosecant of
    /// a finite nonzero [`Float`] is never exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.85091812823932154513384276328642");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.85091812823932154513384276328721");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.csch_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.85091812823932154513384276328721");
    /// ```
    #[inline]
    pub fn csch_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.csch_prec_round_assign(prec, rm)
    }
}

impl Csch for Float {
    type Output = Self;

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], taking it by
    /// value.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cosecant
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{csch} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// See the [`Float::csch_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csch_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::csch_prec`]. If you want both of these things, consider using
    /// [`Float::csch_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Csch;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.csch().is_nan());
    /// assert_eq!(Float::INFINITY.csch(), 0);
    /// assert_eq!(Float::NEGATIVE_INFINITY.csch(), 0);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.csch().to_string(),
    ///     "0.85091812823932154513384276328721"
    /// );
    /// ```
    #[inline]
    fn csch(self) -> Self {
        let prec = self.significant_bits();
        self.csch_prec_round(prec, Nearest).0
    }
}

impl Csch for &Float {
    type Output = Float;

    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], taking it by
    /// reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cosecant
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{csch} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=-0.0$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=-\infty$
    ///
    /// See the [`Float::csch_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csch_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::csch_prec_ref`]. If you want both of these things, consider using
    /// [`Float::csch_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Csch;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).csch().is_nan());
    /// assert_eq!((&Float::INFINITY).csch(), 0);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).csch(), 0);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).csch().to_string(),
    ///     "0.85091812823932154513384276328721"
    /// );
    /// ```
    #[inline]
    fn csch(self) -> Float {
        self.csch_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl CschAssign for Float {
    /// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the hyperbolic cosecant
    /// is equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s
    /// in its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{csch} x+\varepsilon.
    /// $$
    /// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{csch} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{csch} x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::csch`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::csch_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::csch_prec_assign`]. If you want both of these things, consider using
    /// [`Float::csch_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CschAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.csch_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.csch_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.csch_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.csch_assign();
    /// assert_eq!(x.to_string(), "0.85091812823932154513384276328721");
    /// ```
    #[inline]
    fn csch_assign(&mut self) {
        let prec = self.significant_bits();
        self.csch_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\operatorname{csch} x$, the hyperbolic cosecant of a primitive float. The result is
/// correctly rounded.
///
/// $$
/// f(x) = \operatorname{csch} x+\varepsilon.
/// $$
/// - If $\operatorname{csch} x$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\operatorname{csch} x$ is nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   |\operatorname{csch} x|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$
/// - $f(-\infty)=-0.0$
/// - $f(0.0)=\infty$
/// - $f(-0.0)=-\infty$
///
/// An `x` of magnitude below the reciprocal of the largest finite value, such as a subnormal, gives
/// a result that overflows to $\pm\infty$. An `x` of large magnitude gives a subnormal result, or
/// underflows to $\pm0.0$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::csch::primitive_float_csch;
///
/// assert!(primitive_float_csch(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_csch(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csch(-0.0f32)),
///     NiceFloat(f32::NEGATIVE_INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csch(1.0f32)),
///     NiceFloat(0.8509181)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csch(-1.0f64)),
///     NiceFloat(-0.8509181282393216)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_csch(720.0f64)),
///     NiceFloat(4.06446160484e-313)
/// );
/// assert_eq!(NiceFloat(primitive_float_csch(746.0f64)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_csch(5.0e-309f64)),
///     NiceFloat(f64::INFINITY)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_csch<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::csch_prec, x)
}
