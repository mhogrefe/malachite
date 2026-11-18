// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::conversion::float_from_big_exponent_float::*;
use crate::big_exponent_float::exponential::exp::power_of_2_integer;
use crate::big_exponent_float::exponential::wright_omega::{
    wright_omega_helper, wright_omega_rational_helper,
};
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::exponential::traits::{WrightOmega, WrightOmegaAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_q::Rational;

// The result when omega(x) is positive and so far below the least positive `Float` that it rounds
// as any such value does.
fn far_underflow(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    float_from_rounded_big_exponent_float(
        power_of_2_integer(-Integer::power_of_2(40)),
        if rm == Ceiling || rm == Up {
            Greater
        } else {
            Less
        },
        prec,
        rm,
    )
}

fn wright_omega_prec_round_helper(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    match x {
        Float(NaN) => (float_nan!(), Equal),
        Float(Infinity { sign: true }) => (float_infinity!(), Equal),
        Float(Infinity { sign: false }) => (float_zero!(), Equal),
        // omega(x) < e^x, which is below half of the least positive `Float` here; computing it
        // would take as many bits of ln 2 as x has before its binary point.
        Float(Finite {
            sign: false,
            exponent,
            ..
        }) if *exponent > 31 => {
            assert_ne!(rm, Exact, "Inexact Wright omega function");
            far_underflow(prec, rm)
        }
        _ => {
            let (g, o) = wright_omega_helper(&BigExponentFloat::from(x), prec, rm);
            float_from_rounded_big_exponent_float(g, o, prec, rm)
        }
    }
}

fn wright_omega_rational_prec_round_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // See `wright_omega_prec_round_helper`.
    if *x <= const { Rational::const_from_signed(-0x80000000) } {
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact, "Inexact Wright omega function");
        return far_underflow(prec, rm);
    }
    let (g, o) = wright_omega_rational_helper(x, prec, rm);
    float_from_rounded_big_exponent_float(g, o, prec, rm)
}

impl Float {
    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded value is less than, equal
    /// to, or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(1,p,m)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::wright_omega_prec`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::wright_omega_round`] instead. If both of these things are true, consider using
    /// [`Float::wright_omega`] instead.
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
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round(5, Floor);
    /// assert_eq!(w.to_string(), "1.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round(5, Nearest);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round(20, Floor);
    /// assert_eq!(w.to_string(), "1.5571442");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round(20, Nearest);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        wright_omega_prec_round_helper(&self, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than, equal to, or greater than the exact value. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=\infty$
    /// - $f(-\infty,p,m)=0.0$
    /// - $f(1,p,m)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::wright_omega_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::wright_omega_round_ref`] instead. If both of these things are true, consider
    /// using `(&Float).wright_omega()` instead.
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
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round_ref(5, Floor);
    /// assert_eq!(w.to_string(), "1.50");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round_ref(5, Ceiling);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round_ref(5, Nearest);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round_ref(20, Floor);
    /// assert_eq!(w.to_string(), "1.5571442");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round_ref(20, Ceiling);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_round_ref(20, Nearest);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        wright_omega_prec_round_helper(self, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded value is less than, equal to, or greater
    /// than the exact value. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(1,p)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::wright_omega`] instead.
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
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100).0.wright_omega_prec(5);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100).0.wright_omega_prec(20);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec(self, prec: u64) -> (Self, Ordering) {
        self.wright_omega_prec_round(prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded value is less than, equal to,
    /// or greater than the exact value. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=\infty$
    /// - $f(-\infty,p)=0.0$
    /// - $f(1,p)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using `(&Float).wright_omega()` instead.
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
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_ref(5);
    /// assert_eq!(w.to_string(), "1.56");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_prec_ref(20);
    /// assert_eq!(w.to_string(), "1.5571461");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.wright_omega_prec_round_ref(prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], rounding the result with
    /// the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(1,m)=1.0$
    ///
    /// See the [`Float::wright_omega_prec_round`] documentation for information on underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::wright_omega_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::wright_omega`] instead.
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
    /// precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_round(Floor);
    /// assert_eq!(w.to_string(), "1.5571455989976114168586719999993");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_round(Ceiling);
    /// assert_eq!(w.to_string(), "1.5571455989976114168586720000009");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_round(Nearest);
    /// assert_eq!(w.to_string(), "1.5571455989976114168586720000009");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.wright_omega_prec_round(prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], rounding the result with
    /// the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=\infty$
    /// - $f(-\infty,m)=0.0$
    /// - $f(1,m)=1.0$
    ///
    /// See the [`Float::wright_omega_prec_round`] documentation for information on underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::wright_omega_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).wright_omega()` instead.
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
    /// precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_round_ref(Floor);
    /// assert_eq!(w.to_string(), "1.5571455989976114168586719999993");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_round_ref(Ceiling);
    /// assert_eq!(w.to_string(), "1.5571455989976114168586720000009");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::from_unsigned_prec(2u32, 100)
    ///     .0
    ///     .wright_omega_round_ref(Nearest);
    /// assert_eq!(w.to_string(), "1.5571455989976114168586720000009");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.wright_omega_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`] in place, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// returned, indicating whether the rounded value is less than, equal to, or greater than the
    /// exact value. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::wright_omega_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::wright_omega_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::wright_omega_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::wright_omega_assign`] instead.
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
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.50");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.5571442");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5571461");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.5571461");
    /// ```
    #[inline]
    pub fn wright_omega_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) = core::mem::take(self).wright_omega_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`] in place, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::wright_omega_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_prec_round_assign`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::wright_omega_assign`] instead.
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
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.56");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "1.5571461");
    /// ```
    #[inline]
    pub fn wright_omega_prec_assign(&mut self, prec: u64) -> Ordering {
        self.wright_omega_prec_round_assign(prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`] in place, rounding the
    /// result with the specified rounding mode. An [`Ordering`] is returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::wright_omega_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::wright_omega_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::wright_omega_assign`] instead.
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
    /// precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "1.5571455989976114168586719999993");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.5571455989976114168586720000009");
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// assert_eq!(x.wright_omega_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.5571455989976114168586720000009");
    /// ```
    #[inline]
    pub fn wright_omega_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.wright_omega_prec_round_assign(prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than, equal to, or greater than the exact value. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p,m)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::wright_omega_rational_prec`]
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
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) =
    ///     Float::wright_omega_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(w.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::wright_omega_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::wright_omega_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Nearest);
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::wright_omega_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(w.to_string(), "0.81032181");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) =
    ///     Float::wright_omega_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) =
    ///     Float::wright_omega_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Nearest);
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn wright_omega_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        wright_omega_rational_prec_round_helper(&x, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than, equal to, or greater than the exact
    /// value. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p+1}$.
    /// - If $\omega(x)$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p,m)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$ and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$ and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::wright_omega_rational_prec_ref`] instead.
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
    /// with the given precision (which is the case unless $x$ is 1, infinite, or `NaN`).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (w, o) = Float::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.781");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(w.to_string(), "0.81032181");
    /// assert_eq!(o, Less);
    ///
    /// let (w, o) = Float::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::wright_omega_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        wright_omega_rational_prec_round_helper(x, prec, rm)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_rational_prec_round`] instead.
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
    /// let (w, o) = Float::wright_omega_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::wright_omega_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::wright_omega_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], rounding the result to
    /// the nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than, equal to, or greater than the exact value. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the value is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(1,p)=1.0$
    ///
    /// This function never overflows, since $\omega(x)<x$ for $x>1$. It underflows when $x$ is so
    /// negative that $\omega(x)$, which is then about $e^x$, is less than $2^{-2^{30}}$:
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_rational_prec_round_ref`] instead.
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
    /// let (w, o) = Float::wright_omega_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(w.to_string(), "0.812");
    /// assert_eq!(o, Greater);
    ///
    /// let (w, o) = Float::wright_omega_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(w.to_string(), "0.81032276");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn wright_omega_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::wright_omega_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl WrightOmega for Float {
    type Output = Self;

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], taking it by value.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(1)=1.0$
    ///
    /// See the [`Float::wright_omega_prec_round`] documentation for information on underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_round`] instead. If you want to specify the output precision, consider
    /// using [`Float::wright_omega_prec`]. If you want both of these things, consider using
    /// [`Float::wright_omega_prec_round`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_base::num::exponential::traits::WrightOmega;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.wright_omega().is_nan());
    /// assert_eq!(Float::INFINITY.wright_omega(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.wright_omega(), Float::ZERO);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(2u32, 100)
    ///         .0
    ///         .wright_omega()
    ///         .to_string(),
    ///     "1.5571455989976114168586720000009"
    /// );
    /// ```
    #[inline]
    fn wright_omega(self) -> Self {
        let prec = self.significant_bits();
        self.wright_omega_prec(prec).0
    }
}

impl WrightOmega for &Float {
    type Output = Float;

    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`], taking it by reference.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=\infty$
    /// - $f(-\infty)=0.0$
    /// - $f(1)=1.0$
    ///
    /// See the [`Float::wright_omega_prec_round`] documentation for information on underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_round_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::wright_omega_prec_ref`]. If you want both of these things, consider
    /// using [`Float::wright_omega_prec_round_ref`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_base::num::exponential::traits::WrightOmega;
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).wright_omega().is_nan());
    /// assert_eq!((&Float::INFINITY).wright_omega(), Float::INFINITY);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).wright_omega(), Float::ZERO);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(2u32, 100).0)
    ///         .wright_omega()
    ///         .to_string(),
    ///     "1.5571455989976114168586720000009"
    /// );
    /// ```
    #[inline]
    fn wright_omega(self) -> Float {
        self.wright_omega_prec_ref(self.significant_bits()).0
    }
}

impl WrightOmegaAssign for Float {
    /// Computes $\omega(x)$, the Wright omega function, of a [`Float`] in place.
    ///
    /// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is
    /// the positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is
    /// positive and increasing.
    ///
    /// If the output has a precision, it is the precision of the input. If the value is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \omega(x)+\varepsilon.
    /// $$
    /// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\omega(x)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// See the [`Float::wright_omega`] documentation for information on special cases and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::wright_omega_round_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::wright_omega_prec_assign`]. If you want both of these things,
    /// consider using [`Float::wright_omega_prec_round_assign`].
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
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_base::num::exponential::traits::WrightOmegaAssign;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.wright_omega_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.wright_omega_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.wright_omega_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::from_unsigned_prec(2u32, 100).0;
    /// x.wright_omega_assign();
    /// assert_eq!(x.to_string(), "1.5571455989976114168586720000009");
    /// ```
    #[inline]
    fn wright_omega_assign(&mut self) {
        let prec = self.significant_bits();
        self.wright_omega_prec_assign(prec);
    }
}

/// Computes $\omega(x)$, the Wright omega function, of a primitive float.
///
/// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is the
/// positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is positive
/// and increasing.
///
/// $$
/// f(x) = \omega(x)+\varepsilon.
/// $$
/// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\omega(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=0.0$
/// - $f(1)=1.0$
///
/// Overflow is not possible, but underflow is: a negative enough `x` gives `0.0`.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::exponential::wright_omega::primitive_float_wright_omega;
///
/// assert!(primitive_float_wright_omega(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega(f32::NEGATIVE_INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega(0.0f32)),
///     NiceFloat(0.56714326)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega(1.0f32)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega(-1.0f32)),
///     NiceFloat(0.27846456)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_wright_omega<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::wright_omega_prec, x)
}

/// Computes $\omega(x)$, the Wright omega function, of a [`Rational`], returning the result as a
/// primitive float.
///
/// $\omega(x) = W_0(e^x)$, where $W_0$ is the principal branch of the Lambert $W$ function, is the
/// positive solution $y$ of $y + \ln y = x$. It is defined for every real $x$, and it is positive
/// and increasing.
///
/// $$
/// f(x) = \omega(x)+\varepsilon.
/// $$
/// - If $\omega(x)$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $\omega(x)$ is finite and nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
///   |\omega(x)|\rfloor-p}$, where $p$ is the precision of the output (typically 24 if `T` is a
///   [`f32`] and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(1)=1.0$
///
/// Overflow is not possible, but underflow is: a negative enough `x` gives `0.0`.
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
/// use malachite_float::float::exponential::wright_omega::primitive_float_wright_omega_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega_rational::<f64>(
///         &Rational::ZERO
///     )),
///     NiceFloat(0.5671432904097838)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega_rational::<f64>(
///         &Rational::from_unsigneds(3u8, 5)
///     )),
///     NiceFloat(0.810322706979245)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_wright_omega_rational::<f64>(
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(0.2784645427610738)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_wright_omega_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::wright_omega_rational_prec_ref, x)
}
