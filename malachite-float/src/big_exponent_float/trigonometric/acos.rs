// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::trigonometric::atan::{
    InverseTrigFunction, inverse_trig_helper, inverse_trig_rational_helper,
};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes $\arccos x$, the arccosine of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded arccosine is less
    /// than, equal to, or greater than the exact arccosine. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos x|\rfloor-p+1}$.
    /// - If $\arccos x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::acos_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "1.1863995");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.18640");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "1.1863995");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, &self, None, prec, rm, true)
    }

    /// Computes $\arccos x$, the arccosine of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded arccosine is
    /// less than, equal to, or greater than the exact arccosine. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos x|\rfloor-p+1}$.
    /// - If $\arccos x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "1.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "1.1863995");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "1.18640");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "1.1863995");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, self, None, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arccos x$, its arccosine, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos x|\rfloor-p+1}$.
    /// - If $\arccos x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.12");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.1863995");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.18640");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.1863995");
    /// ```
    #[inline]
    pub fn acos_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acos, self, None, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\arccos x$, the arccosine of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=\pi/2$, rounded
    /// - $f(1,p)=0$
    /// - $f(-1,p)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acos_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec(5);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_prec(20);
    /// assert_eq!(y.to_string(), "1.1863995");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, &self, None, prec, Nearest, true)
    }

    /// Computes $\arccos x$, the arccosine of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arccosine is less than, equal
    /// to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=\pi/2$, rounded
    /// - $f(1,p)=0$
    /// - $f(-1,p)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_ref(5);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_prec_ref(20);
    /// assert_eq!(y.to_string(), "1.1863995");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, self, None, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arccos x$, its arccosine, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=\pi/2$, rounded
    /// - $f(1,p)=0$
    /// - $f(-1,p)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "1.19");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "1.1863995");
    /// ```
    #[inline]
    pub fn acos_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acos, self, None, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos x|\rfloor-p+1}$.
    /// - If $\arccos x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acos_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acos_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(y.to_string(), "1.2309589");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.230961");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.2309589");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acos_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, &x, None, prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos x|\rfloor-p+1}$.
    /// - If $\arccos x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=\pi/2$, rounded
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` and the result is nonzero (every nonzero
    /// result is irrational).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.19");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.2309589");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.230961");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.2309589");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, x, None, prec, rm)
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded arccosine
    /// is less than, equal to, or greater than the exact arccosine. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=\pi/2$, rounded
    /// - $f(1,p)=0$
    /// - $f(-1,p)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acos_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "1.2309589");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acos_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, &x, None, prec, Nearest)
    }

    /// Computes $\arccos x$, the arccosine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// arccosine is less than, equal to, or greater than the exact arccosine. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos x+\varepsilon.
    /// $$
    /// - If $\arccos x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arccos x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arccos
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=\pi/2$, rounded
    /// - $f(1,p)=0$
    /// - $f(-1,p)=\pi$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "1.2");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acos_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "1.2309589");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, x, None, prec, Nearest)
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(1,u,p,m)=0$
    /// - $f(-1,u,p,m)=u/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acos_with_period_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 5, Floor);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 20, Floor);
    /// assert_eq!(y.to_string(), "1.3217487");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "1.321751");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec_round(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "1.3217487");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, &self, Some(u), prec, rm, true)
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(1,u,p,m)=0$
    /// - $f(-1,u,p,m)=u/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_with_period_prec_round_ref(7, 5, Floor);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acos_with_period_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acos_with_period_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_with_period_prec_round_ref(7, 20, Floor);
    /// assert_eq!(y.to_string(), "1.3217487");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acos_with_period_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "1.321751");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).acos_with_period_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "1.3217487");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, self, Some(u), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $u\arccos x/(2\pi)$, its arccosine measured in $u$ths
    /// of a turn, rounding the result to the specified precision and with the specified rounding
    /// mode. An [`Ordering`] is returned, indicating whether the rounded arccosine is less than,
    /// equal to, or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(1,u,p,m)=0$
    /// - $f(-1,u,p,m)=u/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_with_period_prec_round_assign(7, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_with_period_prec_round_assign(7, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "1.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_with_period_prec_round_assign(7, 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_with_period_prec_round_assign(7, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "1.3217487");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(
    ///     x.acos_with_period_prec_round_assign(7, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "1.321751");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_with_period_prec_round_assign(7, 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "1.3217487");
    /// ```
    #[inline]
    pub fn acos_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acos, self, Some(u), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arccos
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=u/4$
    /// - $f(1,u,p)=0$
    /// - $f(-1,u,p)=u/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acos_with_period_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec(7, 5);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_with_period_prec(7, 20);
    /// assert_eq!(y.to_string(), "1.3217487");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acos,
            &self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`BigExponentFloat`] measured in $u$ths of
    /// a turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arccos
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=u/4$
    /// - $f(1,u,p)=0$
    /// - $f(-1,u,p)=u/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_with_period_prec_ref(7, 5);
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_with_period_prec_ref(7, 20);
    /// assert_eq!(y.to_string(), "1.3217487");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acos,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $u\arccos x/(2\pi)$, its arccosine measured in $u$ths
    /// of a turn, rounding the result to the nearest value of the specified precision. An
    /// [`Ordering`] is returned, indicating whether the rounded arccosine is less than, equal to,
    /// or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arccos
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=u/4$
    /// - $f(1,u,p)=0$
    /// - $f(-1,u,p)=u/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_with_period_prec_assign(7, 5), Less);
    /// assert_eq!(x.to_string(), "1.31");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_with_period_prec_assign(7, 20), Less);
    /// assert_eq!(x.to_string(), "1.3217487");
    /// ```
    #[inline]
    pub fn acos_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Acos,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(1,u,p,m)=0$
    /// - $f(-1,u,p,m)=u/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acos_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.371391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.3713932");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.3713932");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acos_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, &x, Some(u), prec, rm)
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arccos x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arccos x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=u/4$
    /// - $f(1,u,p,m)=0$
    /// - $f(-1,u,p,m)=u/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.31");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "1.371391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "1.3713932");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "1.3713932");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, x, Some(u), prec, rm)
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arccos
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=u/4$
    /// - $f(1,u,p)=0$
    /// - $f(-1,u,p)=u/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acos_with_period_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "1.3713932");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acos_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, &x, Some(u), prec, Nearest)
    }

    /// Computes $u\arccos x/(2\pi)$, the arccosine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arccosine is less than, equal to, or greater than the exact
    /// arccosine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arccos x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arccos x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arccos x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arccos
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=u/4$
    /// - $f(1,u,p)=0$
    /// - $f(-1,u,p)=u/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=u/6$, $f(-1/2,\ldots)=2u/6$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "1.38");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "1.3713932");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, x, Some(u), prec, Nearest)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p+1}$.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=1/2$
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=1$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::acos_pi_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.3776426");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.37764311");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.3776426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, &self, Some(2), prec, rm, true)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p+1}$.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=1/2$
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=1$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.3776426");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.37764311");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.3776426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Acos, self, Some(2), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arccos(x)/\pi$, its arccosine measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded arccosine is less than, equal to,
    /// or greater than the exact arccosine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p+1}$.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=1/2$
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=1$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.3776426");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.37764311");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.3776426");
    /// ```
    #[inline]
    pub fn acos_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Acos, self, Some(2), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=1/2$
    /// - $f(1,p)=0$
    /// - $f(-1,p)=1$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acos_pi_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec(5);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).acos_pi_prec(20);
    /// assert_eq!(y.to_string(), "0.3776426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acos,
            &self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arccosine is less than, equal to, or greater than the exact arccosine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=1/2$
    /// - $f(1,p)=0$
    /// - $f(-1,p)=1$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).acos_pi_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.3776426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn acos_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Acos,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $\arccos(x)/\pi$, its arccosine measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=1/2$
    /// - $f(1,p)=0$
    /// - $f(-1,p)=1$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.acos_pi_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.3776426");
    /// ```
    #[inline]
    pub fn acos_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Acos,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, returning
    /// a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p+1}$.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=1/2$
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=1$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::acos_pi_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.39182615");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3918266");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3918266");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acos_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, &x, Some(2), prec, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, returning
    /// a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arccosine is less than, equal to, or greater than
    /// the exact arccosine. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p+1}$.
    /// - If $\arccos(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=1/2$
    /// - $f(1,p,m)=0$
    /// - $f(-1,p,m)=1$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.406");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.39182615");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3918266");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3918266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, x, Some(2), prec, rm)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, returning
    /// a [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=1/2$
    /// - $f(1,p)=0$
    /// - $f(-1,p)=1$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::acos_pi_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::acos_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.3918266");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn acos_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, &x, Some(2), prec, Nearest)
    }

    /// Computes $\arccos(x)/\pi$, the arccosine of a [`Rational`] measured in half-turns, returning
    /// a [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded arccosine is less than, equal to, or greater than the exact arccosine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arccos(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arccos(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arccos(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arccos(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=1/2$
    /// - $f(1,p)=0$
    /// - $f(-1,p)=1$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(1/2,\ldots)=1/3$, $f(-1/2,\ldots)=2/3$ are exact when representable with
    ///   `prec` bits, and rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::acos_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acos_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::acos_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.3918266");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn acos_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Acos, x, Some(2), prec, Nearest)
    }
}
