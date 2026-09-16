// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::emulate_float_to_float_fn;
use crate::float::arithmetic::asin::{asin_at_prec, asin_cancellation};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use malachite_base::num::arithmetic::traits::{Acos, AcosAssign, CeilingLogBase2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// Computes acos(x) for a finite nonzero `Float` x, rounded to precision `prec` with rounding mode
// `rm`.
//
// This is mpfr_acos from acos.c, MPFR 4.2.2, for a finite nonzero input.
fn acos_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let positive = *x > 0u32;
    match x.partial_cmp_abs(&1u32).unwrap() {
        // acos(x) = NaN for |x| > 1
        Greater => (Float::NAN, Equal),
        // acos(1) = +0, exactly, and acos(-1) = pi
        Equal => {
            if positive {
                (Float::ZERO, Equal)
            } else {
                Float::pi_prec_round(prec, rm)
            }
        }
        Less => {
            assert_ne!(rm, Exact, "Inexact acos");
            // The quotient x/sqrt(1 - x^2) loses the bits that 1 - x^2 does, and for a positive x
            // the subtraction pi/2 - asin(x) loses about as many again, since acos(x) is small
            // there; a negative x keeps acos(x) near pi, so nothing cancels in the subtraction and
            // only the quotient's loss is charged for.
            let cancel = asin_cancellation(x, positive);
            let supplement = if positive { (cancel << 1) - 2 } else { cancel };
            let mut w = prec + prec.ceiling_log_base_2() + 10 + supplement;
            let mut increment = Limb::WIDTH;
            loop {
                // acos(x) = pi/2 - asin(x) = pi/2 - atan(x/sqrt(1 - x^2))
                let t = asin_at_prec(x, w);
                // exact
                let half_pi = Float::pi_prec(w).0 >> 1u32;
                let t = half_pi.sub_prec(t, w).0;
                if float_can_round(t.significand_ref().unwrap(), w - supplement, prec, rm) {
                    return Float::from_float_prec_round(t, prec, rm);
                }
                w += increment;
                increment = w >> 1;
            }
        }
    }
}

impl Float {
    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0.0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case; unlike the arcsine, a zero input is not one, since
    /// $\pi/2$ is never exactly representable.
    ///
    /// Overflow is not possible, since the result lies in $[0,\pi]$. The result is zero only at
    /// $x=1$: an input just below 1 gives about $\sqrt{2(1-x)}$, which stays representable unless
    /// the input's precision exceeds $2^{31}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::acos_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::acos_round`] instead. If both of these things are true, consider using
    /// [`Float::acos`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O((n+m) (\log (n+m))^3 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a
    /// working precision of about $n$ plus the bits that cancel there, which an input within
    /// $2^{-m}$ of 1 pushes to $2m$; a negative input loses nothing in the subtraction, but its
    /// quotient still costs $m$. The arctangent at that width dominates, and the magnitude of the
    /// input does not otherwise drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is NaN, $|x|>1$, or $x$ is 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.5).acos_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.5).acos_prec_round(10, Ceiling);
    /// assert_eq!(c.to_string(), "1.0488");
    /// assert_eq!(o, Greater);
    ///
    /// // acos(1) is zero, exactly
    /// let (c, o) = Float::ONE.acos_prec_round(10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn acos_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_prec_round_ref(prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the specified
    /// precision and with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from(0.5)).acos_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from(0.5)).acos_prec_round_ref(10, Ceiling);
    /// assert_eq!(c.to_string(), "1.0488");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn acos_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arccosine is NaN outside [-1, 1], and both infinities are outside it
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // acos(±0.0) = pi/2
            Zero { .. } => {
                let (pi, o) = Self::pi_prec_round(prec, rm);
                // exact
                (pi >> 1u32, o)
            }
            Finite { .. } => acos_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arccosine is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.5).acos_prec(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.5).acos_prec(53);
    /// assert_eq!(c.to_string(), "1.0471975511965979");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_prec(self, prec: u64) -> (Self, Ordering) {
        self.acos_prec_round(prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result to the nearest value
    /// of the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_prec`] and [`Float::acos_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from(0.5)).acos_prec_ref(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from(0.5)).acos_prec_ref(53);
    /// assert_eq!(c.to_string(), "1.0471975511965979");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.acos_prec_round_ref(prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds and the special cases; this function
    /// behaves the same way, with $p$ the precision of the input.
    ///
    /// If you want to specify an output precision, consider using [`Float::acos_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// let (c, o) = x.clone().acos_round(Floor);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = x.acos_round(Ceiling);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.acos_prec_round(prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_round`] and [`Float::acos_prec_round`]; this function behaves the same
    /// way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// let (c, o) = (&x).acos_round_ref(Floor);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.acos_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], in place, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.5);
    /// let o = x.acos_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (c, o) = self.acos_prec_round_ref(prec, rm);
        *self = c;
        o
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], in place, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function assigns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::acos_prec`] and [`Float::acos_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.5);
    /// let o = x.acos_prec_assign(10);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec_assign(&mut self, prec: u64) -> Ordering {
        self.acos_prec_round_assign(prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Float`], in place, rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. An
    /// [`Ordering`] is returned, indicating whether the rounded arccosine is less than, equal to,
    /// or greater than the exact arccosine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::acos_round`] and [`Float::acos_prec_round`]; this function behaves the same
    /// way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the input.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// let o = x.acos_round_assign(Floor);
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.acos_prec_round_assign(prec, rm)
    }
}

impl Acos for Float {
    type Output = Self;

    /// Computes $\arccos x$, the arccosine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0)=\pi/2$, rounded
    /// - $f(1)=0.0$
    /// - $f(-1)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case. Overflow is not possible, since the result lies in
    /// $[0,\pi]$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::acos_prec`]. If you want both of these things, consider using
    /// [`Float::acos_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acos;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.acos().is_nan());
    /// // the arccosine is NaN outside [-1, 1], and both infinities are outside it
    /// assert!(Float::INFINITY.acos().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.acos().is_nan());
    /// assert_eq!(Float::ONE.acos().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!(x.acos().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn acos(self) -> Self {
        let prec = self.significant_bits();
        self.acos_prec(prec).0
    }
}

impl Acos for &Float {
    type Output = Float;

    /// Computes $\arccos x$, the arccosine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|>1$
    /// - $f(\pm0.0)=\pi/2$, rounded
    /// - $f(1)=0.0$
    /// - $f(-1)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case. Overflow is not possible, since the result lies in
    /// $[0,\pi]$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::acos_prec_ref`]. If you want both of these things, consider using
    /// [`Float::acos_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Acos;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).acos().is_nan());
    /// assert_eq!((&Float::ONE).acos().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// assert_eq!((&x).acos().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn acos(self) -> Float {
        self.acos_prec_ref(self.significant_bits()).0
    }
}

impl AcosAssign for Float {
    /// Computes $\arccos x$, the arccosine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the arccosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \arccos x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|>1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$, where $p$ is the
    ///   precision of the input.
    ///
    /// See the [`Float::acos`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::acos_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::acos_prec_assign`]. If you want both of these things, consider using
    /// [`Float::acos_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arccosine is taken as $\pi/2-\arctan(x/\sqrt{1-x^2})$ at a working precision of about $n$
    /// plus the bits that cancel there, which an input within $2^{-n}$ of 1 pushes to another $2n$;
    /// the arctangent at that width dominates. The magnitude of the input does not otherwise drive
    /// the cost.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AcosAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.acos_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.acos_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0 >> 1u32;
    /// x.acos_assign();
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn acos_assign(&mut self) {
        let prec = self.significant_bits();
        self.acos_prec_assign(prec);
    }
}

/// Computes $\arccos x$, the arccosine of a primitive float, returning the result as a primitive
/// float.
///
/// $$
/// f(x) = \arccos x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\arccos x|\rfloor-p}$ and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases below are exact.
///
/// Special cases:
/// - $f(\text{NaN})=f(\pm\infty)=\text{NaN}$
/// - $f(x)=\text{NaN}$ for $|x|>1$
/// - $f(\pm0.0)=\pi/2$, rounded
/// - $f(1)=0.0$
/// - $f(-1)=\pi$, rounded
///
/// Overflow is not possible, since the result lies in $[0,\pi]$, and neither is underflow: the only
/// input whose arccosine is zero is 1, where the result is exact.
///
/// # Worst-case complexity
/// $T(m) = O(m \log m \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::acos::primitive_float_acos;
///
/// assert!(primitive_float_acos(f32::NAN).is_nan());
/// // the arccosine is NaN outside [-1, 1]
/// assert!(primitive_float_acos(2.0f32).is_nan());
/// assert_eq!(NiceFloat(primitive_float_acos(1.0f32)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_acos(0.5f32)),
///     NiceFloat(1.0471976)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos(0.5f64)),
///     NiceFloat(1.0471975511965979)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_acos(-1.0f64)),
///     NiceFloat(3.141592653589793)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_acos<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::acos_prec, x)
}
