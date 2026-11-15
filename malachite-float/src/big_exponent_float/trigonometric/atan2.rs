// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::trigonometric::atan::{atan2_helper, atan2_rational_helper};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_q::Rational;

impl BigExponentFloat {
    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the
    /// specified precision and with the specified rounding mode. Both arguments are taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal
    /// to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm\pi/4$ and $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$,
    ///   for finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::atan2_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_prec_round(BigExponentFloat::from(-2), 5, Floor);
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_prec_round(BigExponentFloat::from(-2), 5, Ceiling);
    /// assert_eq!(y.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_prec_round(BigExponentFloat::from(-2), 5, Nearest);
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_prec_round(BigExponentFloat::from(-2), 20, Floor);
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_prec_round(BigExponentFloat::from(-2), 20, Ceiling);
    /// assert_eq!(y.to_string(), "2.1588020");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_prec_round(BigExponentFloat::from(-2), 20, Nearest);
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        atan2_helper(&self, &other, None, prec, rm, true)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the
    /// specified precision and with the specified rounding mode. $y$ is taken by value and $x$ by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm\pi/4$ and $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$,
    ///   for finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.1588020");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(&self, other, None, prec, rm, true)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the
    /// specified precision and with the specified rounding mode. $y$ is taken by reference and $x$
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm\pi/4$ and $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$,
    ///   for finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.1588020");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(self, &other, None, prec, rm, true)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the
    /// specified precision and with the specified rounding mode. $y$ is taken by reference and $x$
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm\pi/4$ and $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$,
    ///   for finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.25");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.1588020");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(self, other, None, prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)$, the angle of the point
    /// $(x,y)$ for another [`BigExponentFloat`] $x$, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`BigExponentFloat`] $x$ is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm\pi/4$ and $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$,
    ///   for finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign(BigExponentFloat::from(-2), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign(BigExponentFloat::from(-2), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.25");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign(BigExponentFloat::from(-2), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign(BigExponentFloat::from(-2), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign(BigExponentFloat::from(-2), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.1588020");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign(BigExponentFloat::from(-2), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = atan2_helper(self, &other, None, prec, rm, true);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)$, the angle of the point
    /// $(x,y)$ for another [`BigExponentFloat`] $x$, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`BigExponentFloat`] $x$ is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm\pi/4$ and $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$,
    ///   for finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign_ref(&BigExponentFloat::from(-2), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign_ref(&BigExponentFloat::from(-2), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.25");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign_ref(&BigExponentFloat::from(-2), 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign_ref(&BigExponentFloat::from(-2), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign_ref(&BigExponentFloat::from(-2), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.1588020");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_round_assign_ref(&BigExponentFloat::from(-2), 20, Nearest),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// ```
    #[inline]
    pub fn atan2_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = atan2_helper(self, other, None, prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the nearest
    /// value of the specified precision. Both arguments are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm\pi/4$ and $f(\pm\infty,-\infty,p)=\pm3\pi/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm\pi$, with the sign of $y$, for
    ///   finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan2_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec(BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec(BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(&self, &other, None, prec, Nearest, true)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the nearest
    /// value of the specified precision. $y$ is taken by value and $x$ by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm\pi/4$ and $f(\pm\infty,-\infty,p)=\pm3\pi/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm\pi$, with the sign of $y$, for
    ///   finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_val_ref(&BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_prec_val_ref(&BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(&self, other, None, prec, Nearest, true)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the nearest
    /// value of the specified precision. $y$ is taken by reference and $x$ by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm\pi/4$ and $f(\pm\infty,-\infty,p)=\pm3\pi/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm\pi$, with the sign of $y$, for
    ///   finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_prec_ref_val(BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(3)).atan2_prec_ref_val(BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(self, &other, None, prec, Nearest, true)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`BigExponentFloat`]s $y$ and $x$, rounding the result to the nearest
    /// value of the specified precision. $y$ is taken by reference and $x$ by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm\pi/4$ and $f(\pm\infty,-\infty,p)=\pm3\pi/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm\pi$, with the sign of $y$, for
    ///   finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(3)).atan2_prec_ref_ref(&BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "2.12");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(3)).atan2_prec_ref_ref(&BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "2.158798");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(self, other, None, prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)$, the angle of the point
    /// $(x,y)$ for another [`BigExponentFloat`] $x$, rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] $x$ is taken by value. An [`Ordering`] is
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm\pi/4$ and $f(\pm\infty,-\infty,p)=\pm3\pi/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm\pi$, with the sign of $y$, for
    ///   finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(y.atan2_prec_assign(BigExponentFloat::from(-2), 5), Less);
    /// assert_eq!(y.to_string(), "2.12");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(y.atan2_prec_assign(BigExponentFloat::from(-2), 20), Less);
    /// assert_eq!(y.to_string(), "2.158798");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let (result, o) = atan2_helper(self, &other, None, prec, Nearest, true);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)$, the angle of the point
    /// $(x,y)$ for another [`BigExponentFloat`] $x$, rounding the result to the nearest value of
    /// the specified precision. The [`BigExponentFloat`] $x$ is taken by reference. An [`Ordering`]
    /// is returned, indicating whether the rounded angle is less than, equal to, or greater than
    /// the exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function sets the [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm\pi/4$ and $f(\pm\infty,-\infty,p)=\pm3\pi/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm\pi$, with the sign of $y$, for
    ///   finite nonzero $y$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_assign_ref(&BigExponentFloat::from(-2), 5),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.12");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_prec_assign_ref(&BigExponentFloat::from(-2), 20),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.158798");
    /// ```
    #[inline]
    pub fn atan2_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let (result, o) = atan2_helper(self, other, None, prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. Both arguments
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan2_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4468575");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_rational_prec_round(
        y: Rational,
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, &x, None, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. $y$ is taken by
    /// value and $x$ by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4468575");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_rational_prec_round_val_ref(
        y: Rational,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, x, None, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. $y$ is taken by
    /// reference and $x$ by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4468575");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_rational_prec_round_ref_val(
        y: &Rational,
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, &x, None, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the specified precision and with the specified rounding mode. $y$ is taken by
    /// reference and $x$ by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4468575");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_rational_prec_round_ref_ref(
        y: &Rational,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, x, None, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. Both arguments are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan2_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_rational_prec(y: Rational, x: Rational, prec: u64) -> (Self, Ordering) {
        atan2_rational_helper(&y, &x, None, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. $y$ is taken by value and $x$ by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_rational_prec_val_ref(y: Rational, x: &Rational, prec: u64) -> (Self, Ordering) {
        atan2_rational_helper(&y, x, None, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. $y$ is taken by reference and
    /// $x$ by value. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_rational_prec_ref_val(y: &Rational, x: Rational, prec: u64) -> (Self, Ordering) {
        atan2_rational_helper(y, &x, None, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, for [`Rational`]s $y$ and $x$, returning a [`BigExponentFloat`], rounding
    /// the result to the nearest value of the specified precision. $y$ is taken by reference and
    /// $x$ by reference. An [`Ordering`] is also returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=\pi$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm\pi/2$, with the sign of $y$, for $y \neq 0$
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.446854");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_rational_prec_ref_ref(y: &Rational, x: &Rational, prec: u64) -> (Self, Ordering) {
        atan2_rational_helper(y, x, None, prec, Nearest)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the specified precision and with the specified rounding mode. Both
    /// arguments are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p,m)=f(y,\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p,m)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p,m)=\pmu/8$ and $f(\pm\infty,-\infty,u,p,m)=\pm3u/8$
    /// - $f(y,\infty,u,p,m)=0$ for finite $y$, and $f(y,-\infty,u,p,m)=\pmu/2$, with the sign of
    ///   $y$, for finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan2_with_period_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.405083");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_prec_round(
        self,
        other: Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(&self, &other, Some(u), prec, rm, true)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the specified precision and with the specified rounding mode. $y$ is
    /// taken by value and $x$ by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p,m)=f(y,\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p,m)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p,m)=\pmu/8$ and $f(\pm\infty,-\infty,u,p,m)=\pm3u/8$
    /// - $f(y,\infty,u,p,m)=0$ for finite $y$, and $f(y,-\infty,u,p,m)=\pmu/2$, with the sign of
    ///   $y$, for finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.405083");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_round_val_ref(
        self,
        other: &Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(&self, other, Some(u), prec, rm, true)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the specified precision and with the specified rounding mode. $y$ is
    /// taken by reference and $x$ by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it
    /// also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p,m)=f(y,\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p,m)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p,m)=\pmu/8$ and $f(\pm\infty,-\infty,u,p,m)=\pm3u/8$
    /// - $f(y,\infty,u,p,m)=0$ for finite $y$, and $f(y,-\infty,u,p,m)=\pmu/2$, with the sign of
    ///   $y$, for finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.405083");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_prec_round_ref_val(
        &self,
        other: Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(self, &other, Some(u), prec, rm, true)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the specified precision and with the specified rounding mode. $y$ is
    /// taken by reference and $x$ by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p,m)=f(y,\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p,m)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p,m)=\pmu/8$ and $f(\pm\infty,-\infty,u,p,m)=\pm3u/8$
    /// - $f(y,\infty,u,p,m)=0$ for finite $y$, and $f(y,-\infty,u,p,m)=\pmu/2$, with the sign of
    ///   $y$, for finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.405083");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_round_ref_ref(
        &self,
        other: &Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(self, other, Some(u), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of
    /// the point $(x,y)$ measured in $u$ths of a turn for another [`BigExponentFloat`] $x$,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] $x$ is taken by value. An [`Ordering`] is returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p,m)=f(y,\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p,m)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p,m)=\pmu/8$ and $f(\pm\infty,-\infty,u,p,m)=\pm3u/8$
    /// - $f(y,\infty,u,p,m)=0$ for finite $y$, and $f(y,-\infty,u,p,m)=\pmu/2$, with the sign of
    ///   $y$, for finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign(BigExponentFloat::from(-2), 7, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign(BigExponentFloat::from(-2), 7, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign(BigExponentFloat::from(-2), 7, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign(BigExponentFloat::from(-2), 7, 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.405083");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign(BigExponentFloat::from(-2), 7, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign(BigExponentFloat::from(-2), 7, 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_prec_round_assign(
        &mut self,
        other: Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = atan2_helper(self, &other, Some(u), prec, rm, true);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of
    /// the point $(x,y)$ measured in $u$ths of a turn for another [`BigExponentFloat`] $x$,
    /// rounding the result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] $x$ is taken by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p,m)=f(y,\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p,m)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p,m)=\pmu/8$ and $f(\pm\infty,-\infty,u,p,m)=\pm3u/8$
    /// - $f(y,\infty,u,p,m)=0$ for finite $y$, and $f(y,-\infty,u,p,m)=\pmu/2$, with the sign of
    ///   $y$, for finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign_ref(&BigExponentFloat::from(-2), 7, 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign_ref(&BigExponentFloat::from(-2), 7, 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.5");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign_ref(&BigExponentFloat::from(-2), 7, 5, Nearest),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign_ref(&BigExponentFloat::from(-2), 7, 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.405083");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign_ref(&BigExponentFloat::from(-2), 7, 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign_ref(&BigExponentFloat::from(-2), 7, 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_round_assign_ref(
        &mut self,
        other: &Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = atan2_helper(self, other, Some(u), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the nearest value of the specified precision. Both arguments are
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p)=f(y,\text{NaN},u,p)=\text{NaN}$
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p)=\pmu/8$ and $f(\pm\infty,-\infty,u,p)=\pm3u/8$
    /// - $f(y,\infty,u,p)=0$ for finite $y$, and $f(y,-\infty,u,p)=\pmu/2$, with the sign of $y$,
    ///   for finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan2_with_period_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_with_period_prec(BigExponentFloat::from(-2), 7, 5);
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_with_period_prec(BigExponentFloat::from(-2), 7, 20);
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_prec(self, other: Self, u: u64, prec: u64) -> (Self, Ordering) {
        atan2_helper(&self, &other, Some(u), prec, Nearest, true)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the nearest value of the specified precision. $y$ is taken by value
    /// and $x$ by reference. An [`Ordering`] is also returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p)=f(y,\text{NaN},u,p)=\text{NaN}$
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p)=\pmu/8$ and $f(\pm\infty,-\infty,u,p)=\pm3u/8$
    /// - $f(y,\infty,u,p)=0$ for finite $y$, and $f(y,-\infty,u,p)=\pmu/2$, with the sign of $y$,
    ///   for finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_with_period_prec_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_val_ref(
        self,
        other: &Self,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_helper(&self, other, Some(u), prec, Nearest, true)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the nearest value of the specified precision. $y$ is taken by
    /// reference and $x$ by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p)=f(y,\text{NaN},u,p)=\text{NaN}$
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p)=\pmu/8$ and $f(\pm\infty,-\infty,u,p)=\pm3u/8$
    /// - $f(y,\infty,u,p)=0$ for finite $y$, and $f(y,-\infty,u,p)=\pmu/2$, with the sign of $y$,
    ///   for finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_prec_ref_val(
        &self,
        other: Self,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_helper(self, &other, Some(u), prec, Nearest, true)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`BigExponentFloat`]s $y$ and $x$,
    /// rounding the result to the nearest value of the specified precision. $y$ is taken by
    /// reference and $x$ by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p)=f(y,\text{NaN},u,p)=\text{NaN}$
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p)=\pmu/8$ and $f(\pm\infty,-\infty,u,p)=\pm3u/8$
    /// - $f(y,\infty,u,p)=0$ for finite $y$, and $f(y,-\infty,u,p)=\pmu/2$, with the sign of $y$,
    ///   for finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_with_period_prec_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_ref_ref(
        &self,
        other: &Self,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_helper(self, other, Some(u), prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of
    /// the point $(x,y)$ measured in $u$ths of a turn for another [`BigExponentFloat`] $x$,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] $x$ is taken by value. An [`Ordering`] is returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p)=f(y,\text{NaN},u,p)=\text{NaN}$
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p)=\pmu/8$ and $f(\pm\infty,-\infty,u,p)=\pm3u/8$
    /// - $f(y,\infty,u,p)=0$ for finite $y$, and $f(y,-\infty,u,p)=\pmu/2$, with the sign of $y$,
    ///   for finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_assign(BigExponentFloat::from(-2), 7, 5),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_assign(BigExponentFloat::from(-2), 7, 20),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_prec_assign(&mut self, other: Self, u: u64, prec: u64) -> Ordering {
        let (result, o) = atan2_helper(self, &other, Some(u), prec, Nearest, true);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of
    /// the point $(x,y)$ measured in $u$ths of a turn for another [`BigExponentFloat`] $x$,
    /// rounding the result to the nearest value of the specified precision. The
    /// [`BigExponentFloat`] $x$ is taken by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,u,p)=f(y,\text{NaN},u,p)=\text{NaN}$
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,u,p)=\pmu/4$ for finite $x$
    /// - $f(\pm\infty,\infty,u,p)=\pmu/8$ and $f(\pm\infty,-\infty,u,p)=\pm3u/8$
    /// - $f(y,\infty,u,p)=0$ for finite $y$, and $f(y,-\infty,u,p)=\pmu/2$, with the sign of $y$,
    ///   for finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_prec_round), this function
    /// accepts arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_assign_ref(&BigExponentFloat::from(-2), 7, 5),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "2.38");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_with_period_prec_assign_ref(&BigExponentFloat::from(-2), 7, 20),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "2.4050865");
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_assign_ref(
        &mut self,
        other: &Self,
        u: u64,
        prec: u64,
    ) -> Ordering {
        let (result, o) = atan2_helper(self, other, Some(u), prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. Both arguments are taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan2_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.726006");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_rational_prec_round(
        y: Rational,
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, &x, Some(u), prec, rm)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. $y$ is taken by value and $x$ by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.726006");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_rational_prec_round_val_ref(
        y: Rational,
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, x, Some(u), prec, rm)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. $y$ is taken by reference and $x$ by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.726006");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_rational_prec_round_ref_val(
        y: &Rational,
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, &x, Some(u), prec, rm)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the specified precision and with the
    /// specified rounding mode. $y$ is taken by reference and $x$ by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded angle is less than, equal to, or greater than
    /// the exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p+1}$.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p,m)=0$ if $x \geq 0$, and $f(0,x,u,p,m)=u/2$ if $x<0$: the zero is taken as
    ///   $+0$, as FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p,m)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.62");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "2.726006");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_with_period_rational_prec_round_ref_ref(
        y: &Rational,
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, x, Some(u), prec, rm)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// Both arguments are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan2_with_period_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_rational_prec(
        y: Rational,
        x: Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, &x, Some(u), prec, Nearest)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// $y$ is taken by value and $x$ by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_rational_prec_val_ref(
        y: Rational,
        x: &Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, x, Some(u), prec, Nearest)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// $y$ is taken by reference and $x$ by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_with_period_rational_prec_ref_val(
        y: &Rational,
        x: Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, &x, Some(u), prec, Nearest)
    }

    /// Computes $u\operatorname{atan2}(y,x)/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis measured in $u$ths of a turn, for [`Rational`]s $y$ and $x$, returning
    /// a [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// $y$ is taken by reference and $x$ by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,u,p) = u\operatorname{atan2}(y,x)/(2\pi)+\varepsilon.
    /// $$
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is zero, or the result is `NaN`, $\varepsilon$ may
    ///   be ignored or assumed to be 0.
    /// - If $u\operatorname{atan2}(y,x)/(2\pi)$ is nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |u\operatorname{atan2}(y,x)/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p)=0$ if $x \geq 0$, and $f(0,x,u,p)=u/2$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,u,p)=\pmu/4$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pmu/8$ and $\pm3u/8$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_with_period_rational_prec_round), this
    /// function never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "2.75");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_with_period_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     7,
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "2.7260017");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_with_period_rational_prec_ref_ref(
        y: &Rational,
        x: &Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, x, Some(u), prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the specified precision and with the specified rounding mode. Both arguments
    /// are taken by value. An [`Ordering`] is also returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable
    /// to any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm1/4$ and $f(\pm\infty,-\infty,p,m)=\pm3/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::atan2_pi_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_round(BigExponentFloat::from(-2), 5, Floor);
    /// assert_eq!(y.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_round(BigExponentFloat::from(-2), 5, Ceiling);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_round(BigExponentFloat::from(-2), 5, Nearest);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_round(BigExponentFloat::from(-2), 20, Floor);
    /// assert_eq!(y.to_string(), "0.6871662");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_round(BigExponentFloat::from(-2), 20, Ceiling);
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_round(BigExponentFloat::from(-2), 20, Nearest);
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        atan2_helper(&self, &other, Some(2), prec, rm, true)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the specified precision and with the specified rounding mode. $y$ is taken by
    /// value and $x$ by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm1/4$ and $f(\pm\infty,-\infty,p,m)=\pm3/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.6871662");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec_round_val_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(&self, other, Some(2), prec, rm, true)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the specified precision and with the specified rounding mode. $y$ is taken by
    /// reference and $x$ by value. An [`Ordering`] is also returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm1/4$ and $f(\pm\infty,-\infty,p,m)=\pm3/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.6871662");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_val(
    ///     BigExponentFloat::from(-2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(self, &other, Some(2), prec, rm, true)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the specified precision and with the specified rounding mode. $y$ is taken by
    /// reference and $x$ by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm1/4$ and $f(\pm\infty,-\infty,p,m)=\pm3/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.656");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.6871662");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = (&BigExponentFloat::from(3)).atan2_pi_prec_round_ref_ref(
    ///     &BigExponentFloat::from(-2),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_helper(self, other, Some(2), prec, rm, true)
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)/\pi$, the angle of the
    /// point $(x,y)$ measured in half-turns for another [`BigExponentFloat`] $x$, rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] $x$ is taken by value. An [`Ordering`] is returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm1/4$ and $f(\pm\infty,-\infty,p,m)=\pm3/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign(BigExponentFloat::from(-2), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "0.656");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign(BigExponentFloat::from(-2), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign(BigExponentFloat::from(-2), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign(BigExponentFloat::from(-2), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "0.6871662");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign(BigExponentFloat::from(-2), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign(BigExponentFloat::from(-2), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = atan2_helper(self, &other, Some(2), prec, rm, true);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)/\pi$, the angle of the
    /// point $(x,y)$ measured in half-turns for another [`BigExponentFloat`] $x$, rounding the
    /// result to the specified precision and with the specified rounding mode. The
    /// [`BigExponentFloat`] $x$ is taken by reference. An [`Ordering`] is returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function sets the
    /// [`BigExponentFloat`] to `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p,m)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p,m)=\pm1/4$ and $f(\pm\infty,-\infty,p,m)=\pm3/4$
    /// - $f(y,\infty,p,m)=0$ for finite $y$, and $f(y,-\infty,p,m)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
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
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign_ref(&BigExponentFloat::from(-2), 5, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "0.656");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign_ref(&BigExponentFloat::from(-2), 5, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign_ref(&BigExponentFloat::from(-2), 5, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign_ref(&BigExponentFloat::from(-2), 20, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "0.6871662");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign_ref(&BigExponentFloat::from(-2), 20, Ceiling),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign_ref(&BigExponentFloat::from(-2), 20, Nearest),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// ```
    #[inline]
    pub fn atan2_pi_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = atan2_helper(self, other, Some(2), prec, rm, true);
        *self = result;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the nearest value of the specified precision. Both arguments are taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm1/4$ and $f(\pm\infty,-\infty,p)=\pm3/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan2_pi_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec(BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::from(3).atan2_pi_prec(BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(&self, &other, Some(2), prec, Nearest, true)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the nearest value of the specified precision. $y$ is taken by value and $x$ by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm1/4$ and $f(\pm\infty,-\infty,p)=\pm3/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_val_ref(&BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     BigExponentFloat::from(3).atan2_pi_prec_val_ref(&BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(&self, other, Some(2), prec, Nearest, true)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the nearest value of the specified precision. $y$ is taken by reference and
    /// $x$ by value. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm1/4$ and $f(\pm\infty,-\infty,p)=\pm3/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(3)).atan2_pi_prec_ref_val(BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(3)).atan2_pi_prec_ref_val(BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(self, &other, Some(2), prec, Nearest, true)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`BigExponentFloat`]s $y$ and $x$, rounding
    /// the result to the nearest value of the specified precision. $y$ is taken by reference and
    /// $x$ by reference. An [`Ordering`] is also returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm1/4$ and $f(\pm\infty,-\infty,p)=\pm3/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(3)).atan2_pi_prec_ref_ref(&BigExponentFloat::from(-2), 5);
    /// assert_eq!(y.to_string(), "0.688");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) =
    ///     (&BigExponentFloat::from(3)).atan2_pi_prec_ref_ref(&BigExponentFloat::from(-2), 20);
    /// assert_eq!(y.to_string(), "0.68716717");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        atan2_helper(self, other, Some(2), prec, Nearest, true)
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)/\pi$, the angle of the
    /// point $(x,y)$ measured in half-turns for another [`BigExponentFloat`] $x$, rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] $x$ is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm1/4$ and $f(\pm\infty,-\infty,p)=\pm3/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_assign(BigExponentFloat::from(-2), 5),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_assign(BigExponentFloat::from(-2), 20),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let (result, o) = atan2_helper(self, &other, Some(2), prec, Nearest, true);
        *self = result;
        o
    }

    /// Replaces a [`BigExponentFloat`] $y$ with $\operatorname{atan2}(y,x)/\pi$, the angle of the
    /// point $(x,y)$ measured in half-turns for another [`BigExponentFloat`] $x$, rounding the
    /// result to the nearest value of the specified precision. The [`BigExponentFloat`] $x$ is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to
    /// any [`BigExponentFloat`], whenever this function sets the [`BigExponentFloat`] to `NaN` it
    /// also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(\text{NaN},x,p)=f(y,\text{NaN},p)=\text{NaN}$
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(\pm\infty,x,p)=\pm1/2$ for finite $x$
    /// - $f(\pm\infty,\infty,p)=\pm1/4$ and $f(\pm\infty,-\infty,p)=\pm3/4$
    /// - $f(y,\infty,p)=0$ for finite $y$, and $f(y,-\infty,p)=\pm1$, with the sign of $y$, for
    ///   finite nonzero $y$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_prec_round), this function accepts
    /// arguments with any exponent, and its result never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_assign_ref(&BigExponentFloat::from(-2), 5),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.688");
    ///
    /// let mut y = BigExponentFloat::from(3);
    /// assert_eq!(
    ///     y.atan2_pi_prec_assign_ref(&BigExponentFloat::from(-2), 20),
    ///     Greater
    /// );
    /// assert_eq!(y.to_string(), "0.68716717");
    /// ```
    #[inline]
    pub fn atan2_pi_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let (result, o) = atan2_helper(self, other, Some(2), prec, Nearest, true);
        *self = result;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. Both arguments are taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`BigExponentFloat::atan2_pi_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.77885723");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_rational_prec_round(
        y: Rational,
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, &x, Some(2), prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. $y$ is taken by value and $x$ by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.77885723");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_rational_prec_round_val_ref(
        y: Rational,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, x, Some(2), prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. $y$ is taken by reference and $x$ by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.77885723");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_rational_prec_round_ref_val(
        y: &Rational,
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, &x, Some(2), prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the specified precision and with the specified
    /// rounding mode. $y$ is taken by reference and $x$ by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p+1}$.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, and $m$ is `Nearest`, then $|\varepsilon|
    ///   \leq 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0$ if $x \geq 0$, and $f(0,x,p,m)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p,m)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p,m)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.75");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(y.to_string(), "0.77885723");
    /// assert_eq!(o, Less);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_round_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_rational_prec_round_ref_ref(
        y: &Rational,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, x, Some(2), prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// Both arguments are taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::atan2_pi_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_rational_prec(y: Rational, x: Rational, prec: u64) -> (Self, Ordering) {
        atan2_rational_helper(&y, &x, Some(2), prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// $y$ is taken by value and $x$ by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_val_ref(
    ///     Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_rational_prec_val_ref(
        y: Rational,
        x: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_rational_helper(&y, x, Some(2), prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// $y$ is taken by reference and $x$ by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_ref_val(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    #[inline]
    pub fn atan2_pi_rational_prec_ref_val(
        y: &Rational,
        x: Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, &x, Some(2), prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis measured in half-turns, for [`Rational`]s $y$ and $x$, returning a
    /// [`BigExponentFloat`], rounding the result to the nearest value of the specified precision.
    /// $y$ is taken by reference and $x$ by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// $$
    /// f(y,x,p) = \operatorname{atan2}(y,x)/\pi+\varepsilon.
    /// $$
    /// - If $\operatorname{atan2}(y,x)/\pi$ is zero, or the result is `NaN`, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - If $\operatorname{atan2}(y,x)/\pi$ is nonzero, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)/\pi|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,p)=0$ if $x \geq 0$, and $f(0,x,p)=1$ if $x<0$: the zero is taken as $+0$, as
    ///   FLINT's `arb_atan2` takes it (so that $f(0,0)=0$)
    /// - $f(y,0,p)=\pm1/2$, with the sign of $y$, for $y \neq 0$
    /// - $f(y,x,0,p)=0$ unless an argument is `NaN`
    /// - if $|y|=|x|$, the result is one of $\pm1/4$ and $\pm3/4$, exact when representable with
    ///   `prec` bits
    ///
    /// Unlike [its `Float` counterpart](crate::Float::atan2_pi_rational_prec_round), this function
    /// never underflows.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`.
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
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     5,
    /// );
    /// assert_eq!(y.to_string(), "0.781");
    /// assert_eq!(o, Greater);
    ///
    /// let (y, o) = BigExponentFloat::atan2_pi_rational_prec_ref_ref(
    ///     &Rational::from_unsigneds(1u8, 3),
    ///     &Rational::from_signeds(-2i8, 5),
    ///     20,
    /// );
    /// assert_eq!(y.to_string(), "0.7788582");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_rational_prec_ref_ref(
        y: &Rational,
        x: &Rational,
        prec: u64,
    ) -> (Self, Ordering) {
        atan2_rational_helper(y, x, Some(2), prec, Nearest)
    }
}
