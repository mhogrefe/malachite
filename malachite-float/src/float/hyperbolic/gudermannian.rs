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
use crate::big_exponent_float::hyperbolic::gudermannian::{
    gudermannian_helper, gudermannian_rational_helper,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::hyperbolic::traits::{Gudermannian, GudermannianAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

fn gudermannian_prec_round_helper(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x {
        Float(NaN) => (float_nan!(), Equal),
        Float(Infinity { sign: true }) => Float::pi_over_2_prec_round(prec, rm),
        Float(Infinity { sign: false }) => {
            let (p, o) = Float::pi_over_2_prec_round(prec, -rm);
            (-p, o.reverse())
        }
        Float(Zero { .. }) => (x.clone(), Equal),
        _ => {
            let (g, o) = gudermannian_helper(&BigExponentFloat::from(x), prec, rm);
            float_from_rounded_big_exponent_float(g, o, prec, rm)
        }
    }
}

fn gudermannian_rational_prec_round_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let (g, o) = gudermannian_rational_helper(x, prec, rm);
    float_from_rounded_big_exponent_float(g, o, prec, rm)
}

impl Float {
    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than, equal to, or greater than the exact value. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// This function never overflows. It only underflows when
    /// $|\operatorname{gd}(x)|<2^{-2^{30}}\leq|x|$ (for an $x$ close to zero,
    /// $\operatorname{gd}(x)$ is slightly closer to zero than $x$ is):
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Floor`, `Up`, or `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gudermannian_prec`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::gudermannian_round`] instead. If both of these things are true, consider using
    /// [`Float::gudermannian`] instead.
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
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.86577034");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gudermannian_prec_round_helper(&self, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], rounding the
    /// result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded value
    /// is less than, equal to, or greater than the exact value. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// This function never overflows. It only underflows when
    /// $|\operatorname{gd}(x)|<2^{-2^{30}}\leq|x|$ (for an $x$ close to zero,
    /// $\operatorname{gd}(x)$ is slightly closer to zero than $x$ is):
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$ and $m$ is `Floor`, `Up`, or `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gudermannian_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::gudermannian_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).gudermannian()` instead.
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
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "0.844");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.86577034");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        gudermannian_prec_round_helper(self, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0,p)=\pm0.0$
    ///
    /// This function never overflows. It only underflows when
    /// $|\operatorname{gd}(x)|<2^{-2^{30}}\leq|x|$ (for an $x$ close to zero,
    /// $\operatorname{gd}(x)$ is slightly closer to zero than $x$ is):
    /// - If $0<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::gudermannian`] instead.
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
    /// let (w, o) = Float::one_prec(100).gudermannian_prec(5);
    /// assert_eq!(w.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec(20);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_prec(self, prec: u64) -> (Self, Ordering) {
        self.gudermannian_prec_round(prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], rounding the
    /// result to the nearest value of the specified precision. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0,p)=\pm0.0$
    ///
    /// This function never overflows. It only underflows when
    /// $|\operatorname{gd}(x)|<2^{-2^{30}}\leq|x|$ (for an $x$ close to zero,
    /// $\operatorname{gd}(x)$ is slightly closer to zero than $x$ is):
    /// - If $0<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using `(&Float).gudermannian()` instead.
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
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_ref(5);
    /// assert_eq!(w.to_string(), "0.875");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_prec_ref(20);
    /// assert_eq!(w.to_string(), "0.86576939");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.gudermannian_prec_round_ref(prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0,m)=\pm0.0$
    ///
    /// See the [`Float::gudermannian_prec_round`] documentation for information on underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::gudermannian_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::gudermannian`] instead.
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
    /// let (w, o) = Float::one_prec(100).gudermannian_round(Floor);
    /// assert_eq!(w.to_string(), "0.86576948323965862428960184619172");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_round(Ceiling);
    /// assert_eq!(w.to_string(), "0.86576948323965862428960184619251");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_round(Nearest);
    /// assert_eq!(w.to_string(), "0.86576948323965862428960184619172");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.gudermannian_prec_round(prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], rounding the
    /// result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0,m)=\pm0.0$
    ///
    /// See the [`Float::gudermannian_prec_round`] documentation for information on underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::gudermannian_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).gudermannian()` instead.
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
    /// let (w, o) = Float::one_prec(100).gudermannian_round_ref(Floor);
    /// assert_eq!(w.to_string(), "0.86576948323965862428960184619172");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_round_ref(Ceiling);
    /// assert_eq!(w.to_string(), "0.86576948323965862428960184619251");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::one_prec(100).gudermannian_round_ref(Nearest);
    /// assert_eq!(w.to_string(), "0.86576948323965862428960184619172");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.gudermannian_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`] in place,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::gudermannian_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::gudermannian_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::gudermannian_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::gudermannian_assign`] instead.
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
    /// assert_eq!(x.gudermannian_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.875");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.875");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.86576939");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.86577034");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.86576939");
    /// ```
    #[inline]
    pub fn gudermannian_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).gudermannian_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`] in place,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::gudermannian_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::gudermannian_assign`] instead.
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
    /// assert_eq!(x.gudermannian_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.875");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.86576939");
    /// ```
    #[inline]
    pub fn gudermannian_prec_assign(&mut self, prec: u64) -> Ordering {
        self.gudermannian_prec_round_assign(prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`] in place,
    /// rounding the result with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$, where $p$ is the precision of the
    ///   input.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::gudermannian_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::gudermannian_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::gudermannian_assign`] instead.
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
    /// assert_eq!(x.gudermannian_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.86576948323965862428960184619172");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.86576948323965862428960184619251");
    ///
    /// let mut x = Float::one_prec(100);
    /// assert_eq!(x.gudermannian_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.86576948323965862428960184619172");
    /// ```
    #[inline]
    pub fn gudermannian_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.gudermannian_prec_round_assign(prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    ///
    /// This function never overflows. Underflow:
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
    /// If you know you'll be using `Nearest`, consider using [`Float::gudermannian_rational_prec`]
    /// instead.
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
    /// let (w, o) =
    ///     Float::gudermannian_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::gudermannian_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::gudermannian_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::gudermannian_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::gudermannian_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(w.to_string(), "0.56693649");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::gudermannian_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn gudermannian_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gudermannian_rational_prec_round_helper(&x, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p+1}$.
    /// - If $\operatorname{gd}(x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    ///
    /// This function never overflows. Underflow:
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
    /// [`Float::gudermannian_rational_prec_ref`] instead.
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
    /// let (w, o) = Float::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.56693649");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::gudermannian_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        gudermannian_rational_prec_round_helper(x, prec, rm)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    ///
    /// This function never overflows. Underflow:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_rational_prec_round`] instead.
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
    /// let (w, o) = Float::gudermannian_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::gudermannian_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::gudermannian_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    ///
    /// This function never overflows. Underflow:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_rational_prec_round_ref`] instead.
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
    /// let (w, o) = Float::gudermannian_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::gudermannian_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.56693554");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn gudermannian_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::gudermannian_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Gudermannian for Float {
    type Output = Self;

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], taking it by
    /// value.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::gudermannian_prec_round`] documentation for information on underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::gudermannian_prec`]. If you want both of these things, consider using
    /// [`Float::gudermannian_prec_round`].
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
    /// use malachite_base::num::basic::traits::{NaN, Zero};
    /// use malachite_base::num::hyperbolic::traits::Gudermannian;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.gudermannian().is_nan());
    /// assert_eq!(Float::ZERO.gudermannian(), Float::ZERO);
    /// assert_eq!(
    ///     Float::one_prec(100).gudermannian().to_string(),
    ///     "0.86576948323965862428960184619172"
    /// );
    /// ```
    #[inline]
    fn gudermannian(self) -> Self {
        let prec = self.significant_bits();
        self.gudermannian_prec(prec).0
    }
}

impl Gudermannian for &Float {
    type Output = Float;

    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`], taking it by
    /// reference.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\pm\pi/2$, correctly rounded
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::gudermannian_prec_round`] documentation for information on underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::gudermannian_prec_ref`]. If you want both of these things, consider
    /// using [`Float::gudermannian_prec_round_ref`].
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
    /// use malachite_base::num::basic::traits::{NaN, Zero};
    /// use malachite_base::num::hyperbolic::traits::Gudermannian;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).gudermannian().is_nan());
    /// assert_eq!((&Float::ZERO).gudermannian(), Float::ZERO);
    /// assert_eq!(
    ///     (&Float::one_prec(100)).gudermannian().to_string(),
    ///     "0.86576948323965862428960184619172"
    /// );
    /// ```
    #[inline]
    fn gudermannian(self) -> Float {
        self.gudermannian_prec_ref(self.significant_bits()).0
    }
}

impl GudermannianAssign for Float {
    /// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Float`] in place.
    ///
    /// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The
    /// function is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \operatorname{gd}(x)+\varepsilon.
    /// $$
    /// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::gudermannian`] documentation for information on special cases and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::gudermannian_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::gudermannian_prec_assign`]. If you want both of these things,
    /// consider using [`Float::gudermannian_prec_round_assign`].
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
    /// use malachite_base::num::basic::traits::{NaN, Zero};
    /// use malachite_base::num::hyperbolic::traits::GudermannianAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.gudermannian_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.gudermannian_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::one_prec(100);
    /// x.gudermannian_assign();
    /// assert_eq!(x.to_string(), "0.86576948323965862428960184619172");
    /// ```
    #[inline]
    fn gudermannian_assign(&mut self) {
        let prec = self.significant_bits();
        self.gudermannian_prec_assign(prec);
    }
}

/// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a primitive float.
///
/// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The function
/// is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
///
/// $$
/// f(x) = \operatorname{gd}(x)+\varepsilon.
/// $$
/// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\pm\pi/2$, correctly rounded
/// - $f(\pm0.0)=\pm0.0$
///
/// Overflow is not possible, and a nonzero `x` never gives a zero result.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::hyperbolic::gudermannian::primitive_float_gudermannian;
///
/// assert!(primitive_float_gudermannian(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian(f32::INFINITY)),
///     NiceFloat(core::f32::consts::FRAC_PI_2)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian(f32::NEGATIVE_INFINITY)),
///     NiceFloat(-core::f32::consts::FRAC_PI_2)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian(0.0f32)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian(1.0f32)),
///     NiceFloat(0.8657695)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian(-1.0f32)),
///     NiceFloat(-0.8657695)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_gudermannian<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::gudermannian_prec, x)
}

/// Computes $\operatorname{gd}(x)$, the Gudermannian function, of a [`Rational`], returning the
/// result as a primitive float.
///
/// $\operatorname{gd}(x) = \int_0^x \operatorname{sech} t\,dt = 2\arctan(\tanh(x/2))$. The function
/// is odd and increasing, and its values lie in $(-\pi/2, \pi/2)$.
///
/// $$
/// f(x) = \operatorname{gd}(x)+\varepsilon.
/// $$
/// - If $\operatorname{gd}(x)$ is zero or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\operatorname{gd}(x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\operatorname{gd}(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if
///   `T` is a [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
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
/// use malachite_float::float::hyperbolic::gudermannian::primitive_float_gudermannian_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian_rational::<f64>(
///         &Rational::ZERO
///     )),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.5669355635975818)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_gudermannian_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(-0.8657694832396586)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_gudermannian_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::gudermannian_rational_prec_ref, x)
}
