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
    /// Computes $\arcsin x$, the arcsine of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded arcsine is less
    /// than, equal to, or greater than the exact arcsine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin x|\rfloor-p+1}$.
    /// - If $\arcsin x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::asin_prec`]
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
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.3843966");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.38439703");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.3843966");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, &self, None, prec, rm, true)
    }

    /// Computes $\arcsin x$, the arcsine of a [`BigExponentFloat`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`BigExponentFloat`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded arcsine is
    /// less than, equal to, or greater than the exact arcsine. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin x|\rfloor-p+1}$.
    /// - If $\arcsin x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_prec_round), this function accepts
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.3843966");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.38439703");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.3843966");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, self, None, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arcsin x$, its arcsine, rounding the result to the
    /// specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin x|\rfloor-p+1}$.
    /// - If $\arcsin x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_prec_round), this function accepts
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
    /// assert_eq!(x.asin_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.38");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.3843966");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.38439703");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.3843966");
    /// ```
    #[inline]
    pub fn asin_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Asin, self, None, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\arcsin x$, the arcsine of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded arcsine is less than, equal
    /// to, or greater than the exact arcsine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asin_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec(5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_prec(20);
    /// assert_eq!(y.to_string(), "0.3843966");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, &self, None, prec, Nearest, true)
    }

    /// Computes $\arcsin x$, the arcsine of a [`BigExponentFloat`], rounding the result to the
    /// nearest value of the specified precision. The [`BigExponentFloat`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded arcsine is less than, equal
    /// to, or greater than the exact arcsine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_prec_round), this function accepts
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.3843966");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, self, None, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arcsin x$, its arcsine, rounding the result to the
    /// nearest value of the specified precision. An [`Ordering`] is returned, indicating whether
    /// the rounded arcsine is less than, equal to, or greater than the exact arcsine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_prec_round), this function accepts
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
    /// assert_eq!(x.asin_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.391");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.3843966");
    /// ```
    #[inline]
    pub fn asin_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Asin, self, None, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsine is less than, equal to, or greater than the exact arcsine. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin x|\rfloor-p+1}$.
    /// - If $\arcsin x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::asin_rational_prec`] instead.
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
    ///     BigExponentFloat::asin_rational_prec_round(Rational::from_unsigneds(1u8, 3), 5, Floor);
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::asin_rational_prec_round(Rational::from_unsigneds(1u8, 3), 20, Floor);
    /// assert_eq!(y.to_string(), "0.33983660");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3398371");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3398371");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asin_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, &x, None, prec, rm)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsine is less than, equal to, or greater than the exact arcsine. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin x|\rfloor-p+1}$.
    /// - If $\arcsin x$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_rational_prec_round), this function
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
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.328");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.33983660");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.3398371");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.3398371");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, x, None, prec, rm)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded arcsine is
    /// less than, equal to, or greater than the exact arcsine. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asin_rational_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::asin_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.3398371");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asin_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, &x, None, prec, Nearest)
    }

    /// Computes $\arcsin x$, the arcsine of a [`Rational`], returning a [`BigExponentFloat`],
    /// rounding the result to the nearest value of the specified precision. The [`Rational`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded arcsine
    /// is less than, equal to, or greater than the exact arcsine. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin x+\varepsilon.
    /// $$
    /// - If $\arcsin x$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - If $\arcsin x$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\arcsin
    ///   x|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm\pi/2$, rounded
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_rational_prec_round), this function
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
    /// let (y, o) = BigExponentFloat::asin_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.344");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::asin_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.3398371");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, x, None, prec, Nearest)
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsine is less than, equal to, or greater than the exact arcsine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=0$
    /// - $f(\pm1,u,p,m)=\pmu/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::asin_with_period_prec`] instead.
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
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 5, Floor);
    /// assert_eq!(y.to_string(), "0.422");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.422");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 20, Floor);
    /// assert_eq!(y.to_string(), "0.4282503");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.42825079");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec_round(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.4282503");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, &self, Some(u), prec, rm, true)
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the specified precision and with the specified rounding mode.
    /// The [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsine is less than, equal to, or greater than the exact arcsine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=0$
    /// - $f(\pm1,u,p,m)=\pmu/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_prec_round), this function
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_with_period_prec_round_ref(7, 5, Floor);
    /// assert_eq!(y.to_string(), "0.422");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).asin_with_period_prec_round_ref(7, 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.44");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).asin_with_period_prec_round_ref(7, 5, Nearest);
    /// assert_eq!(y.to_string(), "0.422");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_with_period_prec_round_ref(7, 20, Floor);
    /// assert_eq!(y.to_string(), "0.4282503");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).asin_with_period_prec_round_ref(7, 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.42825079");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(0.375)).asin_with_period_prec_round_ref(7, 20, Nearest);
    /// assert_eq!(y.to_string(), "0.4282503");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, self, Some(u), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $u\arcsin x/(2\pi)$, its arcsine measured in $u$ths of
    /// a turn, rounding the result to the specified precision and with the specified rounding mode.
    /// An [`Ordering`] is returned, indicating whether the rounded arcsine is less than, equal to,
    /// or greater than the exact arcsine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=0$
    /// - $f(\pm1,u,p,m)=\pmu/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_prec_round), this function
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
    /// assert_eq!(x.asin_with_period_prec_round_assign(7, 5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.422");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_with_period_prec_round_assign(7, 5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.44");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_with_period_prec_round_assign(7, 5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.422");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_with_period_prec_round_assign(7, 20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.4282503");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(
    ///     x.asin_with_period_prec_round_assign(7, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(x.to_string(), "0.42825079");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_with_period_prec_round_assign(7, 20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.4282503");
    /// ```
    #[inline]
    pub fn asin_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Asin, self, Some(u), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arcsine is less than, equal to, or greater than the exact arcsine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arcsin
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=0$
    /// - $f(\pm1,u,p)=\pmu/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asin_with_period_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec(7, 5);
    /// assert_eq!(y.to_string(), "0.422");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_with_period_prec(7, 20);
    /// assert_eq!(y.to_string(), "0.4282503");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Asin,
            &self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`BigExponentFloat`] measured in $u$ths of a
    /// turn, rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsine is less than, equal to, or greater than the exact arcsine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arcsin
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=0$
    /// - $f(\pm1,u,p)=\pmu/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_prec_round), this function
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_with_period_prec_ref(7, 5);
    /// assert_eq!(y.to_string(), "0.422");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_with_period_prec_ref(7, 20);
    /// assert_eq!(y.to_string(), "0.4282503");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Asin,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $u\arcsin x/(2\pi)$, its arcsine measured in $u$ths of
    /// a turn, rounding the result to the nearest value of the specified precision. An [`Ordering`]
    /// is returned, indicating whether the rounded arcsine is less than, equal to, or greater than
    /// the exact arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arcsin
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=0$
    /// - $f(\pm1,u,p)=\pmu/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_prec_round), this function
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
    /// assert_eq!(x.asin_with_period_prec_assign(7, 5), Less);
    /// assert_eq!(x.to_string(), "0.422");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_with_period_prec_assign(7, 20), Less);
    /// assert_eq!(x.to_string(), "0.4282503");
    /// ```
    #[inline]
    pub fn asin_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Asin,
            self,
            Some(u),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded arcsine is less than, equal to, or greater than the
    /// exact arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=0$
    /// - $f(\pm1,u,p,m)=\pmu/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::asin_with_period_rational_prec`] instead.
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
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.3786068");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.37860727");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.37860727");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asin_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, &x, Some(u), prec, rm)
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the specified precision and with
    /// the specified rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded arcsine is less than, equal to, or greater than the
    /// exact arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p+1}$.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\arcsin x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p,m)=0$
    /// - $f(\pm1,u,p,m)=\pmu/4$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_rational_prec_round), this
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
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.391");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.3786068");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.37860727");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.37860727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, x, Some(u), prec, rm)
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsine is less than, equal to, or greater than the exact arcsine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arcsin
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=0$
    /// - $f(\pm1,u,p)=\pmu/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asin_with_period_rational_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.37860727");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asin_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, &x, Some(u), prec, Nearest)
    }

    /// Computes $u\arcsin x/(2\pi)$, the arcsine of a [`Rational`] measured in $u$ths of a turn,
    /// returning a [`BigExponentFloat`], rounding the result to the nearest value of the specified
    /// precision. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,u,p) = u\arcsin x/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\arcsin x/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $u\arcsin x/(2\pi)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2 |u\arcsin
    ///   x/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,u,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,u,p)=0$
    /// - $f(\pm1,u,p)=\pmu/4$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pmu/12$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_with_period_rational_prec_round), this
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
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_with_period_rational_prec_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.37860727");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, x, Some(u), prec, Nearest)
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arcsine is less than, equal to, or greater than the exact arcsine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p+1}$.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::asin_pi_prec`]
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
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec_round(5, Floor);
    /// assert_eq!(y.to_string(), "0.121");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec_round(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec_round(5, Nearest);
    /// assert_eq!(y.to_string(), "0.121");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec_round(20, Floor);
    /// assert_eq!(y.to_string(), "0.12235725");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec_round(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.1223574");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec_round(20, Nearest);
    /// assert_eq!(y.to_string(), "0.12235725");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, &self, Some(2), prec, rm, true)
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsine is less than, equal to, or greater than the exact arcsine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p+1}$.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_prec_round), this function accepts
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_round_ref(5, Floor);
    /// assert_eq!(y.to_string(), "0.121");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_round_ref(5, Ceiling);
    /// assert_eq!(y.to_string(), "0.12");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_round_ref(5, Nearest);
    /// assert_eq!(y.to_string(), "0.121");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_round_ref(20, Floor);
    /// assert_eq!(y.to_string(), "0.12235725");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_round_ref(20, Ceiling);
    /// assert_eq!(y.to_string(), "0.1223574");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_round_ref(20, Nearest);
    /// assert_eq!(y.to_string(), "0.12235725");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        inverse_trig_helper(InverseTrigFunction::Asin, self, Some(2), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] with $\arcsin(x)/\pi$, its arcsine measured in half-turns,
    /// rounding the result to the specified precision and with the specified rounding mode. An
    /// [`Ordering`] is returned, indicating whether the rounded arcsine is less than, equal to, or
    /// greater than the exact arcsine. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p+1}$.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_prec_round), this function accepts
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
    /// assert_eq!(x.asin_pi_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.121");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_pi_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.12");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_pi_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.121");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_pi_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.12235725");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_pi_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.1223574");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_pi_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.12235725");
    /// ```
    #[inline]
    pub fn asin_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (result, o) =
            inverse_trig_helper(InverseTrigFunction::Asin, self, Some(2), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded arcsine is less than, equal to, or greater than the exact arcsine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asin_pi_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec(5);
    /// assert_eq!(y.to_string(), "0.121");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(0.375).asin_pi_prec(20);
    /// assert_eq!(y.to_string(), "0.12235725");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_pi_prec(self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Asin,
            &self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`BigExponentFloat`] measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded arcsine is less than, equal to, or greater than the exact arcsine.
    /// Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_prec_round), this function accepts
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
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_ref(5);
    /// assert_eq!(y.to_string(), "0.121");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(0.375)).asin_pi_prec_ref(20);
    /// assert_eq!(y.to_string(), "0.12235725");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn asin_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        inverse_trig_helper(
            InverseTrigFunction::Asin,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        )
    }

    /// Replaces a [`BigExponentFloat`] with $\arcsin(x)/\pi$, its arcsine measured in half-turns,
    /// rounding the result to the nearest value of the specified precision. An [`Ordering`] is
    /// returned, indicating whether the rounded arcsine is less than, equal to, or greater than the
    /// exact arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=f(\pm\infty,p)=\text{NaN}$
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_prec_round), this function accepts
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
    /// assert_eq!(x.asin_pi_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.121");
    ///
    /// let mut x = BigExponentFloat::from(0.375);
    /// assert_eq!(x.asin_pi_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.12235725");
    /// ```
    #[inline]
    pub fn asin_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        let (result, o) = inverse_trig_helper(
            InverseTrigFunction::Asin,
            self,
            Some(2),
            prec,
            Nearest,
            true,
        );
        *self = result;
        o
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p+1}$.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::asin_pi_rational_prec`] instead.
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
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.105");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.11");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.11");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.108173");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.10817349");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.10817349");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asin_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, &x, Some(2), prec, rm)
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded arcsine is less than, equal to, or greater than the exact
    /// arcsine. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p+1}$.
    /// - If $\arcsin(x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p,m)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p,m)=0$
    /// - $f(\pm1,p,m)=\pm1/2$
    /// - $f(x,0,p,m)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_rational_prec_round), this function
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
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.105");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.11");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.11");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.108173");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.10817349");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.10817349");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, x, Some(2), prec, rm)
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded arcsine is less than, equal to, or greater than the exact arcsine. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::asin_pi_rational_prec_round`] instead.
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
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.11");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::asin_pi_rational_prec(Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.10817349");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn asin_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, &x, Some(2), prec, Nearest)
    }

    /// Computes $\arcsin(x)/\pi$, the arcsine of a [`Rational`] measured in half-turns, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// The [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded arcsine is less than, equal to, or greater than the exact arcsine. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(x,p) = \arcsin(x)/\pi+\varepsilon.
    /// $$
    /// - If $\arcsin(x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\arcsin(x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\arcsin(x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(x,p)=\text{NaN}$ for $|x|>1$
    /// - $f(0,p)=0$
    /// - $f(\pm1,p)=\pm1/2$
    /// - $f(x,0,p)=0$ for $x$ in the domain
    /// - the results $f(\pm1/2,\ldots)=\pm1/6$ are exact when representable with `prec` bits, and
    ///   rounded otherwise
    ///
    /// Unlike [its `Float` counterpart](crate::Float::asin_pi_rational_prec_round), this function
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
    ///     BigExponentFloat::asin_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 5);
    /// assert_eq!(y.to_string(), "0.11");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::asin_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 3), 20);
    /// assert_eq!(y.to_string(), "0.10817349");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn asin_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        inverse_trig_rational_helper(InverseTrigFunction::Asin, x, Some(2), prec, Nearest)
    }
}
