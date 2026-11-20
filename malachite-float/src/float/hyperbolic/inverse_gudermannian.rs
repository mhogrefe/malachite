// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::conversion::float_from_big_exponent_float::*;
use crate::big_exponent_float::hyperbolic::inverse_gudermannian::{
    inverse_gudermannian_helper, inverse_gudermannian_rational_helper,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::hyperbolic::traits::{InverseGudermannian, InverseGudermannianAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

fn inverse_gudermannian_prec_round_helper(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x {
        Float(NaN | Infinity { .. }) => (float_nan!(), Equal),
        Float(Zero { .. }) => (x.clone(), Equal),
        _ => {
            let (g, o) = inverse_gudermannian_helper(&BigExponentFloat::from(x), prec, rm);
            float_from_rounded_big_exponent_float(g, o, prec, rm)
        }
    }
}

fn inverse_gudermannian_rational_prec_round_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let (g, o) = inverse_gudermannian_rational_helper(x, prec, rm);
    float_from_rounded_big_exponent_float(g, o, prec, rm)
}

impl Float {
    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// This function never underflows, since $|\operatorname{gd}^{-1}(x)|\geq|x|$. It does not
    /// overflow either: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as $|x|$
    /// approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$ to
    /// fit in memory.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::inverse_gudermannian_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::inverse_gudermannian_round`] instead. If both of these things are true,
    /// consider using [`Float::inverse_gudermannian`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.2261925");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_gudermannian_prec_round_helper(&self, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// This function never underflows, since $|\operatorname{gd}^{-1}(x)|\geq|x|$. It does not
    /// overflow either: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as $|x|$
    /// approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$ to
    /// fit in memory.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::inverse_gudermannian_prec_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::inverse_gudermannian_round_ref`]
    /// instead. If both of these things are true, consider using `(&Float).inverse_gudermannian()`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.2261925");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_gudermannian_prec_round_helper(self, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// This function never underflows, since $|\operatorname{gd}^{-1}(x)|\geq|x|$. It does not
    /// overflow either: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as $|x|$
    /// approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$ to
    /// fit in memory.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_prec_round`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::inverse_gudermannian`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec(5);
    /// assert_eq!(w.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec(20);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec(self, prec: u64) -> (Self, Ordering) {
        self.inverse_gudermannian_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// This function never underflows, since $|\operatorname{gd}^{-1}(x)|\geq|x|$. It does not
    /// overflow either: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as $|x|$
    /// approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$ to
    /// fit in memory.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_prec_round_ref`] instead. If you know that your target
    /// precision is the precision of the input, consider using `(&Float).inverse_gudermannian()`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_ref(5);
    /// assert_eq!(w.to_string(), "1.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.2261906");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.inverse_gudermannian_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$, where $p$ is the precision of
    ///   the input.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(x,m)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::inverse_gudermannian_prec_round`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::inverse_gudermannian`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_round(Floor);
    /// assert_eq!(w.to_string(), "1.2261911708835170708130609674712");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_round(Ceiling);
    /// assert_eq!(w.to_string(), "1.2261911708835170708130609674728");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_round(Nearest);
    /// assert_eq!(w.to_string(), "1.2261911708835170708130609674712");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.inverse_gudermannian_prec_round(prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$, where $p$ is the precision of
    ///   the input.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(x,m)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::inverse_gudermannian_prec_round_ref`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using `(&Float).inverse_gudermannian()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_round_ref(Floor);
    /// assert_eq!(w.to_string(), "1.2261911708835170708130609674712");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_round_ref(Ceiling);
    /// assert_eq!(w.to_string(), "1.2261911708835170708130609674728");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).inverse_gudermannian_round_ref(Nearest);
    /// assert_eq!(w.to_string(), "1.2261911708835170708130609674712");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.inverse_gudermannian_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`] in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::inverse_gudermannian_prec_round`] documentation for information on special
    /// cases.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::inverse_gudermannian_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::inverse_gudermannian_round_assign`]
    /// instead. If both of these things are true, consider using
    /// [`Float::inverse_gudermannian_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(
    ///     x.inverse_gudermannian_prec_round_assign(5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.25");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(
    ///     x.inverse_gudermannian_prec_round_assign(5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.25");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.2261906");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(
    ///     x.inverse_gudermannian_prec_round_assign(20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.2261925");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.2261906");
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).inverse_gudermannian_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`] in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::inverse_gudermannian_prec_round`] documentation for information on special
    /// cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_prec_round_assign`] instead. If you know that your target
    /// precision is the precision of the input, consider using
    /// [`Float::inverse_gudermannian_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec,
    /// self.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.25");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.2261906");
    /// ```
    #[inline]
    pub fn inverse_gudermannian_prec_assign(&mut self, prec: u64) -> Ordering {
        self.inverse_gudermannian_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`] in
    /// place, rounding the result with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$, where $p$ is the precision of
    ///   the input.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::inverse_gudermannian_prec_round`] documentation for information on special
    /// cases.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::inverse_gudermannian_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::inverse_gudermannian_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.2261911708835170708130609674712");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.2261911708835170708130609674728");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.inverse_gudermannian_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "1.2261911708835170708130609674712");
    /// ```
    #[inline]
    pub fn inverse_gudermannian_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.inverse_gudermannian_prec_round_assign(prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// This function never overflows: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as
    /// $|x|$ approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$
    /// to fit in memory. Underflow:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$ and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$ and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::inverse_gudermannian_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.63962173");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn inverse_gudermannian_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_gudermannian_rational_prec_round_helper(&x, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded value is less than, equal to, or greater
    /// than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// This function never overflows: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as
    /// $|x|$ approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$
    /// to fit in memory. Underflow:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$ and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$ and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::inverse_gudermannian_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case whenever the result is neither zero nor `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.656");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.63962173");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::inverse_gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_gudermannian_rational_prec_round_helper(x, prec, rm)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// This function never overflows: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as
    /// $|x|$ approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$
    /// to fit in memory. Underflow:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (w, o) = Float::inverse_gudermannian_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::inverse_gudermannian_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::inverse_gudermannian_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $|x|>\pi/2$
    ///
    /// This function never overflows: $|\operatorname{gd}^{-1}(x)|$ grows like $-\ln(\pi/2-|x|)$ as
    /// $|x|$ approaches $\pi/2$, so an overflow would take an argument that is too close to $\pi/2$
    /// to fit in memory. Underflow:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(prec, x.significant_bits())`.
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
    /// let (w, o) =
    ///     Float::inverse_gudermannian_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.625");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::inverse_gudermannian_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.63962269");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn inverse_gudermannian_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::inverse_gudermannian_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl InverseGudermannian for Float {
    type Output = Self;

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// taking it by value.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(x)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_round`] instead. If you want to specify the output precision,
    /// consider using [`Float::inverse_gudermannian_prec`]. If you want both of these things,
    /// consider using [`Float::inverse_gudermannian_prec_round`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::hyperbolic::traits::InverseGudermannian;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.inverse_gudermannian().is_nan());
    /// assert!(Float::INFINITY.inverse_gudermannian().is_nan());
    /// assert_eq!(Float::ZERO.inverse_gudermannian(), Float::ZERO);
    /// assert_eq!(
    ///     Float::one_prec(100).inverse_gudermannian().to_string(),
    ///     "1.2261911708835170708130609674712"
    /// );
    /// ```
    #[inline]
    fn inverse_gudermannian(self) -> Self {
        let prec = self.significant_bits();
        self.inverse_gudermannian_prec(prec).0
    }
}

impl InverseGudermannian for &Float {
    type Output = Float;

    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`],
    /// taking it by reference.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(x)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_round_ref`] instead. If you want to specify the output
    /// precision, consider using [`Float::inverse_gudermannian_prec_ref`]. If you want both of
    /// these things, consider using [`Float::inverse_gudermannian_prec_round_ref`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::hyperbolic::traits::InverseGudermannian;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).inverse_gudermannian().is_nan());
    /// assert!((&Float::INFINITY).inverse_gudermannian().is_nan());
    /// assert_eq!((&Float::ZERO).inverse_gudermannian(), Float::ZERO);
    /// assert_eq!(
    ///     (&Float::one_prec(100)).inverse_gudermannian().to_string(),
    ///     "1.2261911708835170708130609674712"
    /// );
    /// ```
    #[inline]
    fn inverse_gudermannian(self) -> Float {
        self.inverse_gudermannian_prec_ref(self.significant_bits())
            .0
    }
}

impl InverseGudermannianAssign for Float {
    /// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Float`] in
    /// place.
    ///
    /// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| <
    /// \pi/2$. The function is odd and increasing, and it takes every real value.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{gd}^{-1}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed
    ///   to be 0.
    /// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::inverse_gudermannian`] documentation for information on special cases.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::inverse_gudermannian_round_assign`] instead. If you want to specify the output
    /// precision, consider using [`Float::inverse_gudermannian_prec_assign`]. If you want both of
    /// these things, consider using [`Float::inverse_gudermannian_prec_round_assign`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, Zero};
    /// use malachite_base::num::hyperbolic::traits::InverseGudermannianAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.inverse_gudermannian_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.inverse_gudermannian_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.inverse_gudermannian_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::one_prec(100);
    /// x.inverse_gudermannian_assign();
    /// assert_eq!(x.to_string(), "1.2261911708835170708130609674712");
    /// ```
    #[inline]
    fn inverse_gudermannian_assign(&mut self) {
        let prec = self.significant_bits();
        self.inverse_gudermannian_prec_assign(prec);
    }
}

/// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a primitive float.
///
/// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| < \pi/2$.
/// The function is odd and increasing, and it takes every real value.
///
/// $$
/// f(x) = \operatorname{gd}^{-1}(x)+\varepsilon.
/// $$
/// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24
///   if `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
/// - $f(x)=\text{NaN}$ if $|x|>\pi/2$, including $\pm\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::hyperbolic::inverse_gudermannian::*;
///
/// assert!(primitive_float_inverse_gudermannian(f32::NAN).is_nan());
/// assert!(primitive_float_inverse_gudermannian(f32::INFINITY).is_nan());
/// assert!(primitive_float_inverse_gudermannian(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_inverse_gudermannian(0.0f32)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_inverse_gudermannian(1.0f32)),
///     NiceFloat(1.2261912)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_inverse_gudermannian(-1.0f32)),
///     NiceFloat(-1.2261912)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_inverse_gudermannian<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::inverse_gudermannian_prec, x)
}

/// Computes $\operatorname{gd}^{-1}(x)$, the inverse Gudermannian function, of a [`Rational`],
/// returning the result as a primitive float.
///
/// $\operatorname{gd}^{-1}(x) = \int_0^x \sec t\,dt = \ln((1+\sin x)/\cos x)$, for $|x| < \pi/2$.
/// The function is odd and increasing, and it takes every real value.
///
/// $$
/// f(x) = \operatorname{gd}^{-1}(x)+\varepsilon.
/// $$
/// - If $\operatorname{gd}^{-1}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
///   0.
/// - If $\operatorname{gd}^{-1}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\operatorname{gd}^{-1}(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24
///   if `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
/// - $f(x)=\text{NaN}$ if $|x|>\pi/2$
///
/// Overflow is not possible, but underflow is: an `x` close enough to zero gives `0.0` or `-0.0`.
///
/// # Worst-case complexity
/// $T(n) = O(n^{3/2} \log n \log\log n)$
///
/// $M(n) = O(n \log n)$
///
/// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::hyperbolic::inverse_gudermannian::*;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_inverse_gudermannian_rational::<f64>(
///         &Rational::ZERO
///     )),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_inverse_gudermannian_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.6396225158077645)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_inverse_gudermannian_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(-1.2261911708835171)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_inverse_gudermannian_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::inverse_gudermannian_rational_prec_ref, x)
}
