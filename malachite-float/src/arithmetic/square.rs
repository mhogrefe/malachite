// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{
    float_either_infinity, float_either_zero, float_infinity, float_nan, float_zero, Float,
};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, IsPowerOf2, Square, SquareAssign,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float_square::{
    square_float_significand_in_place, square_float_significand_ref,
};

impl Float {
    /// Squares a [`Float`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded square is less than, equal to, or greater than the exact square.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p+1}$.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\infty$
    /// - $f(\pm0.0,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::square_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::square_round`] instead. If both of these things are true, consider using
    /// [`Float::square`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact squaring.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = Float::from(PI).square_prec_round(5, Floor);
    /// assert_eq!(square.to_string(), "9.5");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round(5, Ceiling);
    /// assert_eq!(square.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round(5, Nearest);
    /// assert_eq!(square.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round(20, Floor);
    /// assert_eq!(square.to_string(), "9.8696");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round(20, Ceiling);
    /// assert_eq!(square.to_string(), "9.86961");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round(20, Nearest);
    /// assert_eq!(square.to_string(), "9.8696");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec_round(mut self, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        let o = self.square_prec_round_assign(prec, rm);
        (self, o)
    }

    /// Squares a [`Float`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded square is less than, equal to, or greater than the exact
    /// square. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p+1}$.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\infty$
    /// - $f(\pm0.0,p,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,y,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::square_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::square_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).square()`instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact squaring.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = Float::from(PI).square_prec_round_ref(5, Floor);
    /// assert_eq!(square.to_string(), "9.5");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round_ref(5, Ceiling);
    /// assert_eq!(square.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round_ref(5, Nearest);
    /// assert_eq!(square.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round_ref(20, Floor);
    /// assert_eq!(square.to_string(), "9.8696");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round_ref(20, Ceiling);
    /// assert_eq!(square.to_string(), "9.86961");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec_round_ref(20, Nearest);
    /// assert_eq!(square.to_string(), "9.8696");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        match self {
            float_nan!() => (float_nan!(), Equal),
            float_either_infinity!() => (float_infinity!(), Equal),
            float_either_zero!() => (float_zero!(), Equal),
            Float(Finite {
                exponent: x_exp,
                precision: x_prec,
                significand: x,
                ..
            }) => {
                let twice_exp = x_exp << 1;
                if twice_exp - 1 > Float::MAX_EXPONENT {
                    assert!(rm != Exact, "Inexact Float squaring");
                    return match rm {
                        Ceiling | Up | Nearest => (float_infinity!(), Greater),
                        _ => (Float::max_finite_value_with_prec(prec), Less),
                    };
                } else if twice_exp < Float::MIN_EXPONENT - 1 {
                    assert!(rm != Exact, "Inexact Float squaring");
                    return match rm {
                        Floor | Down | Nearest => (float_zero!(), Less),
                        _ => (Float::min_positive_value_prec(prec), Greater),
                    };
                }
                let (square, exp_offset, o) = square_float_significand_ref(x, *x_prec, prec, rm);
                let exp = x_exp
                    .arithmetic_checked_shl(1u32)
                    .unwrap()
                    .checked_add(exp_offset)
                    .unwrap();
                if exp > Float::MAX_EXPONENT {
                    assert!(rm != Exact, "Inexact Float squaring");
                    return match rm {
                        Ceiling | Up | Nearest => (float_infinity!(), Greater),
                        _ => (Float::max_finite_value_with_prec(prec), Less),
                    };
                } else if exp < Float::MIN_EXPONENT {
                    return if rm == Nearest
                        && exp == Float::MIN_EXPONENT - 1
                        && (o == Less || !square.is_power_of_2())
                    {
                        (Float::min_positive_value_prec(prec), Greater)
                    } else {
                        match rm {
                            Exact => panic!("Inexact float squaring"),
                            Ceiling | Up => (Float::min_positive_value_prec(prec), Greater),
                            _ => (float_zero!(), Less),
                        }
                    };
                }
                (
                    Float(Finite {
                        sign: true,
                        exponent: exp,
                        precision: prec,
                        significand: square,
                    }),
                    o,
                )
            }
        }
    }

    /// Squares a [`Float`], rounding the result to the nearest value of the specified precision.
    /// The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded square is less than, equal to, or greater than the exact square. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the square is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^2|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\infty$
    /// - $f(\pm0.0,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::square_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::square`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = Float::from(PI).square_prec(5);
    /// assert_eq!(square.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec(20);
    /// assert_eq!(square.to_string(), "9.8696");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec(self, prec: u64) -> (Float, Ordering) {
        self.square_prec_round(prec, Nearest)
    }

    /// Squares a [`Float`], rounding the result to the nearest value of the specified precision.
    /// The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded square is less than, equal to, or greater than the exact square. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the square is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^2|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\infty$
    /// - $f(\pm0.0,p)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::square_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).square()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = Float::from(PI).square_prec_ref(5);
    /// assert_eq!(square.to_string(), "10.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_prec_ref(20);
    /// assert_eq!(square.to_string(), "9.8696");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn square_prec_ref(&self, prec: u64) -> (Float, Ordering) {
        self.square_prec_round_ref(prec, Nearest)
    }

    /// Squares a [`Float`], rounding the result with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded square is
    /// less than, equal to, or greater than the exact square. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\infty$
    /// - $f(\pm0.0,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::square_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::square`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input is not high enough to represent the
    /// output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = Float::from(PI).square_round(Floor);
    /// assert_eq!(square.to_string(), "9.86960440108935");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = Float::from(PI).square_round(Ceiling);
    /// assert_eq!(square.to_string(), "9.86960440108936");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_round(Nearest);
    /// assert_eq!(square.to_string(), "9.86960440108936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn square_round(self, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.square_prec_round(prec, rm)
    }

    /// Squares a [`Float`], rounding the result with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded square
    /// is less than, equal to, or greater than the exact square. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,y,m) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\infty$
    /// - $f(\pm0.0,m)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $f(x,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you want to specify an output precision, consider using [`Float::square_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).square()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input is not high enough to represent the
    /// output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (square, o) = Float::from(PI).square_round_ref(Floor);
    /// assert_eq!(square.to_string(), "9.86960440108935");
    /// assert_eq!(o, Less);
    ///
    /// let (square, o) = Float::from(PI).square_round_ref(Ceiling);
    /// assert_eq!(square.to_string(), "9.86960440108936");
    /// assert_eq!(o, Greater);
    ///
    /// let (square, o) = Float::from(PI).square_round_ref(Nearest);
    /// assert_eq!(square.to_string(), "9.86960440108936");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn square_round_ref(&self, rm: RoundingMode) -> (Float, Ordering) {
        let prec = self.significant_bits();
        self.square_prec_round_ref(prec, rm)
    }

    /// Squares a [`Float`] in place, rounding the result to the specified precision and with the
    /// specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded square
    /// is less than, equal to, or greater than the exact square. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns
    /// `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::square_prec_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::square_prec_assign`] instead.
    /// If you know that your target precision is the precision of the input, consider using
    /// [`Float::square_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::square_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but `prec` is too small for an exact squaring;
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "9.5");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "9.8696");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.86961");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "9.8696");
    /// ```
    #[inline]
    pub fn square_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        assert_ne!(prec, 0);
        match self {
            float_nan!() => Equal,
            Float(Infinity { sign } | Zero { sign }) => {
                *sign = true;
                Equal
            }
            Float(Finite {
                sign: ref mut x_sign,
                exponent: ref mut x_exp,
                precision: ref mut x_prec,
                significand: ref mut x,
            }) => {
                let twice_exp = *x_exp << 1;
                if twice_exp - 1 > Float::MAX_EXPONENT {
                    assert!(rm != Exact, "Inexact Float squaring");
                    return match rm {
                        Ceiling | Up | Nearest => {
                            *self = float_infinity!();
                            Greater
                        }
                        _ => {
                            *self = Float::max_finite_value_with_prec(prec);
                            Less
                        }
                    };
                } else if twice_exp < Float::MIN_EXPONENT - 1 {
                    assert!(rm != Exact, "Inexact Float squaring");
                    return match rm {
                        Floor | Down | Nearest => {
                            *self = float_zero!();
                            Less
                        }
                        _ => {
                            *self = Float::min_positive_value_prec(prec);
                            Greater
                        }
                    };
                }
                let (exp_offset, o) = square_float_significand_in_place(x, *x_prec, prec, rm);
                *x_exp = x_exp
                    .arithmetic_checked_shl(1u32)
                    .unwrap()
                    .checked_add(exp_offset)
                    .unwrap();
                if *x_exp > Float::MAX_EXPONENT {
                    assert!(rm != Exact, "Inexact Float squaring");
                    return match rm {
                        Ceiling | Up | Nearest => {
                            *self = float_infinity!();
                            Greater
                        }
                        _ => {
                            *self = Float::max_finite_value_with_prec(prec);
                            Less
                        }
                    };
                } else if *x_exp < Float::MIN_EXPONENT {
                    return if rm == Nearest
                        && *x_exp == Float::MIN_EXPONENT - 1
                        && (o == Less || !x.is_power_of_2())
                    {
                        {
                            *self = Float::min_positive_value_prec(prec);
                            Greater
                        }
                    } else {
                        match rm {
                            Exact => panic!("Inexact float squaring"),
                            Ceiling | Up => {
                                *self = Float::min_positive_value_prec(prec);
                                Greater
                            }
                            _ => {
                                *self = float_zero!();
                                Less
                            }
                        }
                    };
                }
                *x_sign = true;
                *x_prec = prec;
                o
            }
        }
    }

    /// Squares a [`Float`] in place, rounding the result to the nearest value of the specified
    /// precision. An [`Ordering`] is returned, indicating whether the rounded square is less than,
    /// equal to, or greater than the exact square. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// If the square is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^2|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::square_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::square_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::square`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n \log n \log\log n + m)$
    ///
    /// $M(n, m) = O(n \log n + m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $m$ is
    /// `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "10.0");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "9.8696");
    /// ```
    #[inline]
    pub fn square_prec_assign(&mut self, prec: u64) -> Ordering {
        self.square_prec_round_assign(prec, Nearest)
    }

    /// Squares a [`Float`] in place, rounding the result with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded square is less than, equal to, or
    /// greater than the exact square. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p+1}$, where $p$ is the maximum precision of the inputs.
    /// - If $x^2$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |x^2|\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::square_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::square_prec_round_assign`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::square_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the precision of the input is not high enough to represent the
    /// output.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "9.86960440108935");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "9.86960440108936");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.square_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "9.86960440108936");
    /// ```
    #[inline]
    pub fn square_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.square_prec_round_assign(prec, rm)
    }
}

impl Square for Float {
    type Output = Float;

    /// Squares a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the square is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^2|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\infty$
    /// - $f(\pm0.0)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::square_prec`] instead. If you want to specify the output precision, consider using
    /// [`Float::square_round`]. If you want both of these things, consider using
    /// [`Float::square_prec_round`].
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
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.square().is_nan());
    /// assert_eq!(Float::INFINITY.square(), Float::INFINITY);
    /// assert_eq!(Float::NEGATIVE_INFINITY.square(), Float::INFINITY);
    /// assert_eq!(Float::from(1.5).square(), 2.0);
    /// assert_eq!(Float::from(-1.5).square(), 2.0);
    /// ```
    #[inline]
    fn square(self) -> Float {
        let prec = self.significant_bits();
        self.square_prec_round(prec, Nearest).0
    }
}

impl Square for &Float {
    type Output = Float;

    /// Squares a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the square is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x,y) = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^2|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\infty$
    /// - $f(\pm0.0)=0.0$
    ///
    /// Overflow and underflow:
    /// - If $f(x)\geq 2^{2^{30}-1}$, $\infty$ is returned instead.
    /// - If $f(x)\geq 2^{2^{30}-1}$, $-\infty$ is returned instead.
    /// - If $0<f(x)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::square_prec_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::square_round_ref`]. If you want both of these things, consider using
    /// [`Float::square_prec_round_ref`].
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
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).square().is_nan());
    /// assert_eq!((&Float::INFINITY).square(), Float::INFINITY);
    /// assert_eq!((&Float::NEGATIVE_INFINITY).square(), Float::INFINITY);
    /// assert_eq!((&Float::from(1.5)).square(), 2.0);
    /// assert_eq!((&Float::from(-1.5)).square(), 2.0);
    /// ```
    #[inline]
    fn square(self) -> Float {
        let prec = self.significant_bits();
        self.square_prec_round_ref(prec, Nearest).0
    }
}

impl SquareAssign for Float {
    /// Squares a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the square is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x\gets = x^2+\varepsilon.
    /// $$
    /// - If $x^2$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x^2$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |x^2|\rfloor-p}$,
    ///   where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::square`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::square_prec_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::square_round_assign`]. If you want both of these things, consider using
    /// [`Float::square_prec_round_assign`].
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
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.square_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.square_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.square_assign();
    /// assert_eq!(x, Float::INFINITY);
    ///
    /// let mut x = Float::from(1.5);
    /// x.square_assign();
    /// assert_eq!(x, 2.0);
    ///
    /// let mut x = Float::from(-1.5);
    /// x.square_assign();
    /// assert_eq!(x, 2.0);
    /// ```
    #[inline]
    fn square_assign(&mut self) {
        let prec = self.significant_bits();
        self.square_prec_round_assign(prec, Nearest);
    }
}
