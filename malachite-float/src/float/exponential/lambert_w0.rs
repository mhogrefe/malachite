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
use crate::big_exponent_float::exponential::lambert_w0::{
    lambert_w0_helper, lambert_w0_rational_helper,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::exponential::traits::{LambertW0, LambertW0Assign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

fn lambert_w0_prec_round_helper(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x {
        Float(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
        Float(Infinity { sign: true }) => (float_infinity!(), Equal),
        Float(Zero { .. }) => (x.clone(), Equal),
        _ => {
            let (w, o) = lambert_w0_helper(&BigExponentFloat::from(x), prec, rm);
            float_from_rounded_big_exponent_float(w, o, prec, rm)
        }
    }
}

fn lambert_w0_rational_prec_round_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let (w, o) = lambert_w0_rational_helper(x, prec, rm);
    float_from_rounded_big_exponent_float(w, o, prec, rm)
}

impl Float {
    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// This function never overflows. It only underflows when $x$ is positive and
    /// $W_0(x)<2^{-2^{30}}\leq x$ (for a tiny positive $x$, $W_0(x)$ is slightly less than $x$):
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::lambert_w0_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::lambert_w0_round`] instead. If both of these things are true, consider using
    /// [`Float::lambert_w0`] instead.
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
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "0.56714249");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        lambert_w0_prec_round_helper(&self, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// This function never overflows. It only underflows when $x$ is positive and
    /// $W_0(x)<2^{-2^{30}}\leq x$ (for a tiny positive $x$, $W_0(x)$ is slightly less than $x$):
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::lambert_w0_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::lambert_w0_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).lambert_w0()` instead.
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
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "0.56714249");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        lambert_w0_prec_round_helper(self, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`],
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// This function never overflows. It only underflows when $x$ is positive and
    /// $W_0(x)<2^{-2^{30}}\leq x$ (for a tiny positive $x$, $W_0(x)$ is slightly less than $x$):
    /// - If $0<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::lambert_w0`] instead.
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
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100).0.lambert_w0_prec(5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100).0.lambert_w0_prec(20);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec(self, prec: u64) -> (Self, Ordering) {
        self.lambert_w0_prec_round(prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`],
    /// rounding the result to the nearest value of the specified precision. The [`Float`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=\pm0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// This function never overflows. It only underflows when $x$ is positive and
    /// $W_0(x)<2^{-2^{30}}\leq x$ (for a tiny positive $x$, $W_0(x)$ is slightly less than $x$):
    /// - If $0<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).lambert_w0()` instead.
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
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_ref(5);
    /// assert_eq!(w.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_prec_ref(20);
    /// assert_eq!(w.to_string(), "0.56714344");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.lambert_w0_prec_round_ref(prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`],
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(x,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// See the [`Float::lambert_w0_prec_round`] documentation for information on underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::lambert_w0_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::lambert_w0`] instead.
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
    /// precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_round(Floor);
    /// assert_eq!(w.to_string(), "0.56714329040978387299996866221002");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_round(Ceiling);
    /// assert_eq!(w.to_string(), "0.56714329040978387299996866221081");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_round(Nearest);
    /// assert_eq!(w.to_string(), "0.56714329040978387299996866221002");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn lambert_w0_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.lambert_w0_prec_round(prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`],
    /// rounding the result with the specified rounding mode. The [`Float`] is taken by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=\pm0.0$
    /// - $f(x,m)=\text{NaN}$ if $x<-1/e$
    ///
    /// See the [`Float::lambert_w0_prec_round`] documentation for information on underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::lambert_w0_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).lambert_w0()` instead.
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
    /// precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_round_ref(Floor);
    /// assert_eq!(w.to_string(), "0.56714329040978387299996866221002");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_round_ref(Ceiling);
    /// assert_eq!(w.to_string(), "0.56714329040978387299996866221081");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .lambert_w0_round_ref(Nearest);
    /// assert_eq!(w.to_string(), "0.56714329040978387299996866221002");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn lambert_w0_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.lambert_w0_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`] in
    /// place, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded value is less than, equal to, or
    /// greater than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::lambert_w0_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::lambert_w0_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::lambert_w0_round_assign`] instead. If both of these things are true, consider
    /// using [`Float::lambert_w0_assign`] instead.
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
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.594");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.56714249");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.56714344");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.56714344");
    /// ```
    #[inline]
    pub fn lambert_w0_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).lambert_w0_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`] in
    /// place, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::lambert_w0_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::lambert_w0_assign`] instead.
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
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.56714344");
    /// ```
    #[inline]
    pub fn lambert_w0_prec_assign(&mut self, prec: u64) -> Ordering {
        self.lambert_w0_prec_round_assign(prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`] in
    /// place, rounding the result with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::lambert_w0_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::lambert_w0_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::lambert_w0_assign`] instead.
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
    /// precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.56714329040978387299996866221002");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.56714329040978387299996866221081");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.lambert_w0_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.56714329040978387299996866221002");
    /// ```
    #[inline]
    pub fn lambert_w0_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.lambert_w0_prec_round_assign(prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded value is less than, equal to, or greater than
    /// the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
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
    /// If you know you'll be using `Nearest`, consider using [`Float::lambert_w0_rational_prec`]
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
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(w.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(w.to_string(), "0.40156317");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(w.to_string(), "0.40156364");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(w.to_string(), "0.40156364");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn lambert_w0_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        lambert_w0_rational_prec_round_helper(&x, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the specified precision and with the specified rounding mode and
    /// returning the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded value is less than, equal to, or greater
    /// than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p+1}$.
    /// - If $W_0(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0.0$
    /// - $f(x,p,m)=\text{NaN}$ if $x<-1/e$
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
    /// [`Float::lambert_w0_rational_prec_ref`] instead.
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
    /// with the given precision (which is the case whenever the result is finite and nonzero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(w.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::lambert_w0_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(w.to_string(), "0.40156317");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.40156364");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::lambert_w0_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.40156364");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        lambert_w0_rational_prec_round_helper(x, prec, rm)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// This function never overflows. Underflow:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_rational_prec_round`] instead.
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
    /// let (w, o) = Float::lambert_w0_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::lambert_w0_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.40156364");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::lambert_w0_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
    /// rounding the result to the nearest value of the specified precision and returning the result
    /// as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0.0$
    /// - $f(x,p)=\text{NaN}$ if $x<-1/e$
    ///
    /// This function never overflows. Underflow:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_rational_prec_round_ref`] instead.
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
    /// let (w, o) = Float::lambert_w0_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::lambert_w0_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.40156364");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn lambert_w0_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::lambert_w0_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl LambertW0 for Float {
    type Output = Self;

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`], taking
    /// it by value.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(x)=\text{NaN}$ if $x<-1/e$
    ///
    /// See the [`Float::lambert_w0_prec_round`] documentation for information on underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::lambert_w0_prec`]. If you want both of these things, consider using
    /// [`Float::lambert_w0_prec_round`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::exponential::traits::LambertW0;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.lambert_w0().is_nan());
    /// assert_eq!(Float::INFINITY.lambert_w0(), Float::INFINITY);
    /// assert!(Float::NEGATIVE_INFINITY.lambert_w0().is_nan());
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100)
    ///         .0
    ///         .lambert_w0()
    ///         .to_string(),
    ///     "0.56714329040978387299996866221002"
    /// );
    /// ```
    #[inline]
    fn lambert_w0(self) -> Self {
        let prec = self.significant_bits();
        self.lambert_w0_prec(prec).0
    }
}

impl LambertW0 for &Float {
    type Output = Float;

    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`], taking
    /// it by reference.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    /// - $f(x)=\text{NaN}$ if $x<-1/e$
    ///
    /// See the [`Float::lambert_w0_prec_round`] documentation for information on underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::lambert_w0_prec_ref`]. If you want both of these things, consider
    /// using [`Float::lambert_w0_prec_round_ref`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::exponential::traits::LambertW0;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).lambert_w0().is_nan());
    /// assert_eq!((&Float::INFINITY).lambert_w0(), Float::INFINITY);
    /// assert!((&Float::NEGATIVE_INFINITY).lambert_w0().is_nan());
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0)
    ///         .lambert_w0()
    ///         .to_string(),
    ///     "0.56714329040978387299996866221002"
    /// );
    /// ```
    #[inline]
    fn lambert_w0(self) -> Float {
        self.lambert_w0_prec_ref(self.significant_bits()).0
    }
}

impl LambertW0Assign for Float {
    /// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Float`] in
    /// place.
    ///
    /// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets W_0(x)+\varepsilon.
    /// $$
    /// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |W_0(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::lambert_w0`] documentation for information on special cases and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::lambert_w0_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::lambert_w0_prec_assign`]. If you want both of these things, consider
    /// using [`Float::lambert_w0_prec_round_assign`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_base::num::exponential::traits::LambertW0Assign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.lambert_w0_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.lambert_w0_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.lambert_w0_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.lambert_w0_assign();
    /// assert_eq!(x.to_string(), "0.56714329040978387299996866221002");
    /// ```
    #[inline]
    fn lambert_w0_assign(&mut self) {
        let prec = self.significant_bits();
        self.lambert_w0_prec_assign(prec);
    }
}

/// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a primitive float.
///
/// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
///
/// $$
/// f(x) = W_0(x)+\varepsilon.
/// $$
/// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |W_0(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
/// - $f(x)=\text{NaN}$ if $x<-1/e$
///
/// Overflow is not possible, and a nonzero `x` never gives a zero result.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::exponential::lambert_w0::primitive_float_lambert_w0;
///
/// assert!(primitive_float_lambert_w0(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_lambert_w0(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_lambert_w0(0.0f32)),
///     NiceFloat(0.0)
/// );
/// assert!(primitive_float_lambert_w0(-1.0f32).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_lambert_w0(1.0f32)),
///     NiceFloat(0.56714326)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_lambert_w0<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::lambert_w0_prec, x)
}

/// Computes $W_0(x)$, the principal branch of the Lambert $W$ function, of a [`Rational`],
/// returning the result as a primitive float.
///
/// $W_0(x)$ is the solution $w \geq -1$ of $we^w = x$, which exists when $x \geq -1/e$.
///
/// $$
/// f(x) = W_0(x)+\varepsilon.
/// $$
/// - If $W_0(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $W_0(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |W_0(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0.0$
/// - $f(x)=\text{NaN}$ if $x<-1/e$
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
/// use malachite_float::float::exponential::lambert_w0::primitive_float_lambert_w0_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_lambert_w0_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_lambert_w0_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.4015636367870726)
/// );
/// assert!(primitive_float_lambert_w0_rational::<f64>(&Rational::NEGATIVE_ONE).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_lambert_w0_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::lambert_w0_rational_prec_ref, x)
}
