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
    /// Returns an approximation of Catalan's constant, $G=\sum_{k=0}^\infty
    /// \frac{(-1)^k}{(2k+1)^2}$, with the given precision and rounded using the given
    /// [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the rounded value is
    /// less than or greater than the exact value of the constant. (The rounded value is never equal
    /// to the exact value of the constant. Catalan's constant is irrational; a proof, formalized in
    /// Lean, is in [OpenAI's math
    /// repository](https://github.com/openai/math/blob/main/lean/OAI/NumberTheory/Catalan/Results/Conclusions.lean).)
    ///
    /// $$
    /// x = G+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| < 2^{-p}$.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::catalans_constant_prec_round),
    /// converted to a [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (x, o) = BigExponentFloat::catalans_constant_prec_round(100, Floor);
    /// assert_eq!(x.to_string(), "0.91596559417721901505460351493173");
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = BigExponentFloat::catalans_constant_prec_round(100, Ceiling);
    /// assert_eq!(x.to_string(), "0.9159655941772190150546035149325");
    /// assert_eq!(o, Greater);
    /// ```
    #[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
    #[inline]
    pub fn catalans_constant_prec_round(prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        let (x, o) = Float::catalans_constant_prec_round(prec, rm);
        (Self::from(x), o)
    }

    /// Returns an approximation of Catalan's constant, $G=\sum_{k=0}^\infty
    /// \frac{(-1)^k}{(2k+1)^2}$, with the given precision and rounded to the nearest value of that
    /// precision. An [`Ordering`] is also returned, indicating whether the rounded value is less
    /// than or greater than the exact value of the constant. (The rounded value is never equal to
    /// the exact value of the constant. Catalan's constant is irrational; a proof, formalized in
    /// Lean, is in [OpenAI's math
    /// repository](https://github.com/openai/math/blob/main/lean/OAI/NumberTheory/Catalan/Results/Conclusions.lean).)
    ///
    /// $$
    /// x = G+\varepsilon.
    /// $$
    /// - $|\varepsilon| < 2^{-p}$.
    ///
    /// The result is that of [its `Float` counterpart](crate::Float::catalans_constant_prec),
    /// converted to a [`BigExponentFloat`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
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
    /// let (x, o) = BigExponentFloat::catalans_constant_prec(1);
    /// assert_eq!(x.to_string(), "1.0");
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = BigExponentFloat::catalans_constant_prec(10);
    /// assert_eq!(x.to_string(), "0.9160");
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = BigExponentFloat::catalans_constant_prec(100);
    /// assert_eq!(x.to_string(), "0.9159655941772190150546035149325");
    /// assert_eq!(o, Greater);
    /// ```
    #[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
    #[inline]
    pub fn catalans_constant_prec(prec: u64) -> (Self, Ordering) {
        let (x, o) = Float::catalans_constant_prec(prec);
        (Self::from(x), o)
    }
}
