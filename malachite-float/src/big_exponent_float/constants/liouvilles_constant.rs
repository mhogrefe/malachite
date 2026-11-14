// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::{BigExponentFloat, Float};
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode;

impl BigExponentFloat {
    /// Returns an approximation of Liouville's constant in a given base, with the given precision
    /// and rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// Liouville's constant in base $b$ has a digit of 1 at every position that is a factorial and
    /// a digit of 0 everywhere else. That is,
    /// $$
    /// L_b = \sum_{n=1}^\infty b^{-n!}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 L_b\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 L_b\rfloor-p}$.
    ///
    /// Base 10 gives the classical constant, $0.110001000000000000000001\ldots$, the first number
    /// proven transcendental. The constant is transcendental in every base, being a Liouville
    /// number: its factorial-spaced digits make it approximable by rationals far too well for an
    /// algebraic number.
    ///
    /// The result is that of [its `Float`
    /// counterpart](crate::Float::liouvilles_constant_base_prec_round), converted to a
    /// [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2, if `prec` is zero, or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::liouvilles_constant_base_prec_round(2, 64, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.c40001");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::liouvilles_constant_base_prec_round(2, 64, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x0.c400010000000001");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn liouvilles_constant_base_prec_round(
        base: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let (x, o) = Float::liouvilles_constant_base_prec_round(base, prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of Liouville's constant in a given base, with the given precision
    /// and rounded to the nearest value of that precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded value is less than or greater than the exact value.
    ///
    /// See
    /// [`liouvilles_constant_base_prec_round`](BigExponentFloat::liouvilles_constant_base_prec_round)
    /// for details.
    ///
    /// The result is that of [its `Float`
    /// counterpart](crate::Float::liouvilles_constant_base_prec), converted to a
    /// [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `base` is less than 2 or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::liouvilles_constant_base_prec(2, 64);
    /// assert_eq!(format!("{x:?}"), "0x0.c40001");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::liouvilles_constant_base_prec(3, 50);
    /// assert_eq!(format!("{x:?}"), "0x0.722102754dfd4");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn liouvilles_constant_base_prec(base: u64, prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::liouvilles_constant_base_prec(base, prec);
        (Self::from(x), o)
    }

    /// Returns an approximation of Liouville's constant in base 10, with the given precision and
    /// rounded using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// Liouville's constant has a decimal digit of 1 at every position that is a factorial and a
    /// digit of 0 everywhere else, so it begins $0.110001000000000000000001\ldots$.
    ///
    /// $$
    /// x = L = \sum_{n=1}^{\infty} 10^{-n!}+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// This is the base-10 specialization of
    /// [`liouvilles_constant_base_prec_round`](BigExponentFloat::liouvilles_constant_base_prec_round).
    ///
    /// The result is that of [its `Float`
    /// counterpart](crate::Float::liouvilles_constant_prec_round), converted to a
    /// [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::liouvilles_constant_prec_round(100, Floor);
    /// assert_eq!(format!("{x:?}"), "0x0.1c29068986fcdee34fc7466d12");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::liouvilles_constant_prec_round(100, Ceiling);
    /// assert_eq!(format!("{x:?}"), "0x0.1c29068986fcdee34fc7466d14");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn liouvilles_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (x, o) = Float::liouvilles_constant_prec_round(prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of Liouville's constant in base 10, with the given precision and
    /// rounded to the nearest value of that precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// Liouville's constant has a decimal digit of 1 at every position that is a factorial and a
    /// digit of 0 everywhere else, so it begins $0.110001000000000000000001\ldots$.
    ///
    /// $$
    /// x = L = \sum_{n=1}^{\infty} 10^{-n!}+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p-1}$.
    ///
    /// The constant is irrational and transcendental.
    ///
    /// This is the base-10 specialization of
    /// [`liouvilles_constant_base_prec`](BigExponentFloat::liouvilles_constant_base_prec).
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::liouvilles_constant_prec),
    /// converted to a [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::liouvilles_constant_prec(100);
    /// assert_eq!(format!("{x:?}"), "0x0.1c29068986fcdee34fc7466d12");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn liouvilles_constant_prec(prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::liouvilles_constant_prec(prec);
        (Self::from(x), o)
    }
}
