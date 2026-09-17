// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::emulate_float_to_float_fn;
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{Abs, Asec, AsecAssign, CeilingLogBase2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Exact, Nearest};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;

// Computes asec(x) for a finite nonzero `Float` x, rounded to precision `prec` with rounding mode
// `rm`.
//
// MPFR has no arcsecant. Rather than take acos(1/x), which would round the reciprocal first and pay
// for it -- the arccosine is not Lipschitz at 1, so an x near 1 would lose about half the bits of
// the reciprocal -- the identity is used in the form
//
//     asec(x) = atan(sqrt(x^2 - 1)),
//
// for a positive x, and pi minus that for a negative one. The subtraction x^2 - 1 is where an x
// near 1 loses bits, and it is done at a precision wide enough to be exact: the square of a p-bit
// `Float` needs 2p bits, and their difference no more. So nothing is lost, and unlike the arccosine
// the cost does not grow as x approaches +-1.
fn asec_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let positive = *x > 0u32;
    match x.partial_cmp_abs(&1u32).unwrap() {
        // asec(x) = NaN for |x| < 1, the secant never taking a value there
        Less => (Float::NAN, Equal),
        // asec(1) = +0, exactly, and asec(-1) = pi
        Equal => {
            if positive {
                (Float::ZERO, Equal)
            } else {
                Float::pi_prec_round(prec, rm)
            }
        }
        Greater => {
            assert_ne!(rm, Exact, "Inexact asec");
            let exp_x = i64::from(x.get_exponent().unwrap());
            // the width at which x^2 - 1 is exact
            let exact_w = (x.get_prec().unwrap() << 1) + 2;
            let mut w = prec + prec.ceiling_log_base_2() + 10;
            let mut increment = Limb::WIDTH;
            loop {
                let q = if exp_x << 1 > i64::exact_from(w) + 2 {
                    // x^2 - 1 = x^2(1 - x^-2), and x^-2 is below the working precision here, so the
                    // square root is |x| itself -- which also keeps a huge x from squaring out of
                    // the exponent range
                    x.abs()
                } else {
                    x.square_prec_ref(max(w, exact_w))
                        .0
                        .sub_prec(Float::ONE, max(w, exact_w))
                        .0
                        .sqrt_prec(w)
                        .0
                };
                // The square root is correctly rounded and the arctangent neither amplifies a
                // relative error nor adds more than its own half ulp, so three bits of slack cover
                // a positive x; pi and the subtraction take one more.
                let t = q.atan_prec(w).0;
                let (t, err) = if positive {
                    (t, 3)
                } else {
                    (Float::pi_prec(w).0.sub_prec(t, w).0, 4)
                };
                if float_can_round(t.significand_ref().unwrap(), w - err, prec, rm) {
                    return Float::from_float_prec_round(t, prec, rm);
                }
                w += increment;
                increment = w >> 1;
            }
        }
    }
}

impl Float {
    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded arcsecant is less than,
    /// equal to, or greater than the exact arcsecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The arcsecant is the inverse of the secant on $[0,\pi]$, so it is the arccosine of the
    /// reciprocal, and it is defined only outside $(-1,1)$. See [`RoundingMode`] for a description
    /// of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{asec} x|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{asec} x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
    /// - $f(\pm\infty,p,m)=\pi/2$, rounded, the value the secant grows toward
    /// - $f(1,p,m)=0.0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case.
    ///
    /// Overflow is not possible, since the result lies in $[0,\pi]$. The result is zero only at
    /// $x=1$: an input just above 1 gives about $\sqrt{2(x-1)}$, which stays representable unless
    /// the input's precision exceeds $2^{31}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::asec_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::asec_round`] instead. If both of these things are true, consider using
    /// [`Float::asec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`: the arcsecant is taken as $\arctan(\sqrt{x^2-1})$, and the
    /// arctangent runs at a working precision of about $n$, which costs the first term; the second
    /// is the square, taken at twice the input's precision, where $x^2-1$ is exact. Unlike the
    /// arccosine, whose working precision grows as its input approaches $\pm1$, the arcsecant's
    /// does not: all of the cancellation is confined to that exact subtraction.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x$ is NaN, $|x|<1$, or $x$ is 1).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::TWO.asec_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::TWO.asec_prec_round(10, Ceiling);
    /// assert_eq!(c.to_string(), "1.0488");
    /// assert_eq!(o, Greater);
    ///
    /// // asec(1) is zero, exactly
    /// let (c, o) = Float::ONE.asec_prec_round(10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn asec_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.asec_prec_round_ref(prec, rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded arcsecant is
    /// less than, equal to, or greater than the exact arcsecant. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::TWO).asec_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// // an input of -1 gives pi
    /// let (c, o) = (&Float::NEGATIVE_ONE).asec_prec_round_ref(10, Nearest);
    /// assert_eq!(c.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    pub fn asec_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // the arcsecant is NaN inside (-1, 1), and both zeros are inside it
            NaN | Zero { .. } => (Self::NAN, Equal),
            // the secant grows without bound toward pi/2, so an infinite input gives pi/2
            Infinity { .. } => {
                let (pi, o) = Self::pi_prec_round(prec, rm);
                // exact
                (pi >> 1u32, o)
            }
            Finite { .. } => asec_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded arcsecant is less than, equal to, or
    /// greater than the exact arcsecant. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the arcsecant is equidistant from two [`Float`]s with the specified precision, the
    /// [`Float`] with fewer 1s in its binary expansion is chosen.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::TWO.asec_prec(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(2u32, 100).0.asec_prec(100);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_prec(self, prec: u64) -> (Self, Ordering) {
        self.asec_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arcsecant is less than, equal
    /// to, or greater than the exact arcsecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec`] and [`Float::asec_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::TWO).asec_prec_ref(10);
    /// assert_eq!(c.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(2u32, 100).0).asec_prec_ref(100);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.asec_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds and the special cases; this function
    /// behaves the same way, with $p$ the precision of the input.
    ///
    /// If you want to specify an output precision, consider using [`Float::asec_prec_round`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
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
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// let (c, o) = x.asec_round(Floor);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.asec_prec_round(prec, rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], rounding the result with the
    /// specified rounding mode. The precision of the output is the precision of the input. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsecant is less than, equal to, or greater than the exact arcsecant. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`Float::asec_round`] and [`Float::asec_prec_round`]; this function behaves the same
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
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// let (c, o) = (&x).asec_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "1.0471975511965977461542144610936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asec_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.asec_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded arcsecant is less than, equal to, or greater than
    /// the exact arcsecant. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec_round`] for the error bounds, the special cases, and the complexity;
    /// this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::TWO;
    /// let o = x.asec_prec_round_assign(10, Floor);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (s, o) = self.asec_prec_round_ref(prec, rm);
        *self = s;
        o
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded arcsecant is less than, equal to, or greater than the exact
    /// arcsecant. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_prec`] and [`Float::asec_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::TWO;
    /// let o = x.asec_prec_assign(10);
    /// assert_eq!(x.to_string(), "1.0469");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_prec_assign(&mut self, prec: u64) -> Ordering {
        self.asec_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place, rounding the
    /// result with the specified rounding mode. The precision of the output is the precision of the
    /// input. An [`Ordering`] is returned, indicating whether the rounded arcsecant is less than,
    /// equal to, or greater than the exact arcsecant. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function assigns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::asec_round`] and [`Float::asec_prec_round`]; this function behaves the same
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
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// let o = x.asec_round_assign(Floor);
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610921");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asec_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.asec_prec_round_assign(prec, rm)
    }
}

impl Asec for Float {
    type Output = Self;

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$, where $p$
    ///   is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(x)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
    /// - $f(\pm\infty)=\pi/2$, rounded
    /// - $f(1)=0.0$
    /// - $f(-1)=\pi$, rounded
    ///
    /// The zero at $x=1$ is the only exact case. Overflow is not possible, since the result lies in
    /// $[0,\pi]$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_round`] instead. If you want to specify the output precision, consider using
    /// [`Float::asec_prec`]. If you want both of these things, consider using
    /// [`Float::asec_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.asec().is_nan());
    /// // the arcsecant is NaN inside (-1, 1)
    /// assert!(Float::ZERO.asec().is_nan());
    /// assert!(Float::ONE_HALF.asec().is_nan());
    /// assert_eq!(Float::ONE.asec().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.asec().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn asec(self) -> Self {
        let prec = self.significant_bits();
        self.asec_prec(prec).0
    }
}

impl Asec for &Float {
    type Output = Float;

    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$, where $p$
    ///   is the precision of the input.
    ///
    /// See the [`Float::asec`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::asec_prec_ref`]. If you want both of these things, consider using
    /// [`Float::asec_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Asec;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).asec().is_nan());
    /// assert_eq!((&Float::ONE).asec().to_string(), "0.0");
    ///
    /// let x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!((&x).asec().to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn asec(self) -> Float {
        self.asec_prec_ref(self.significant_bits()).0
    }
}

impl AsecAssign for Float {
    /// Computes $\operatorname{asec} x$, the arcsecant of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the arcsecant is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{asec} x+\varepsilon.
    /// $$
    /// - If $x$ is NaN, if $|x|<1$, or if $x$ is 1, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - Otherwise, $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$, where $p$
    ///   is the precision of the input.
    ///
    /// See the [`Float::asec`] documentation for information on the special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::asec_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::asec_prec_assign`]. If you want both of these things, consider using
    /// [`Float::asec_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^3 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`: the
    /// arcsecant is taken as $\arctan(\sqrt{x^2-1})$, with the square at twice the input's
    /// precision, where $x^2-1$ is exact, and the arctangent at about $n$ bits, which dominates.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AsecAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.asec_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ONE;
    /// x.asec_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// x.asec_assign();
    /// assert_eq!(x.to_string(), "1.0471975511965977461542144610936");
    /// ```
    #[inline]
    fn asec_assign(&mut self) {
        let prec = self.significant_bits();
        self.asec_prec_assign(prec);
    }
}

/// Computes $\operatorname{asec} x$, the arcsecant of a primitive float, returning the result as a
/// primitive float.
///
/// $$
/// f(x) = \operatorname{asec} x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{asec} x|\rfloor-p}$ and $p$ is the
/// precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases
/// below are exact.
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(x)=\text{NaN}$ for $|x|<1$, including $\pm0.0$
/// - $f(\pm\infty)=\pi/2$, rounded
/// - $f(1)=0.0$
/// - $f(-1)=\pi$, rounded
///
/// Overflow is not possible, since the result lies in $[0,\pi]$, and neither is underflow: the only
/// input whose arcsecant is zero is 1, where the result is exact.
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
/// use malachite_float::float::arithmetic::asec::primitive_float_asec;
///
/// assert!(primitive_float_asec(f32::NAN).is_nan());
/// // the arcsecant is NaN inside (-1, 1)
/// assert!(primitive_float_asec(0.5f32).is_nan());
/// assert_eq!(NiceFloat(primitive_float_asec(1.0f32)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_asec(2.0f32)),
///     NiceFloat(1.0471976)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec(2.0f64)),
///     NiceFloat(1.0471975511965979)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_asec(-1.0f64)),
///     NiceFloat(3.141592653589793)
/// );
/// // a huge input is a quarter turn
/// assert_eq!(
///     NiceFloat(primitive_float_asec(1.0e300f64)),
///     NiceFloat(1.5707963267948966)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_asec<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::asec_prec, x)
}
