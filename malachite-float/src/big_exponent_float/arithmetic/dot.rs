// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::arithmetic::sum::sum_helper;
use alloc::vec::Vec;
use core::cmp::Ordering;
use malachite_base::rounding_modes::RoundingMode::{self, *};

// The dot product of `xs` and `ys`, rounded to `prec` bits with `rm`: the exact products, summed
// with a single rounding.
fn dot_helper(
    xs: &[BigExponentFloat],
    ys: &[BigExponentFloat],
    prec: u64,
    rm: RoundingMode,
) -> (BigExponentFloat, Ordering) {
    assert_eq!(xs.len(), ys.len());
    let products: Vec<BigExponentFloat> = xs.iter().zip(ys).map(|(x, y)| x * y).collect();
    sum_helper(products.iter(), prec, rm)
}

impl BigExponentFloat {
    /// Computes the dot product of two equal-length slices of [`BigExponentFloat`]s, rounding the
    /// result to the specified precision and with the specified rounding mode. An [`Ordering`] is
    /// also returned, indicating whether the rounded dot product is less than, equal to, or greater
    /// than the exact dot product. Although `NaN`s are not comparable to any [`BigExponentFloat`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly and summed as by [`BigExponentFloat::sum_prec_round`], so
    /// only one rounding occurs.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1},(y_i)_{i=0}^{n-1},p,m) = \sum_{i=0}^{n-1} x_iy_i+\varepsilon.
    /// $$
    /// - If $\sum_{i=0}^{n-1} x_iy_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_{i=0}^{n-1} x_iy_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\sum_{i=0}^{n-1} x_iy_i|\rfloor-p+1}$.
    /// - If $\sum_{i=0}^{n-1} x_iy_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\sum_{i=0}^{n-1} x_iy_i|\rfloor-p}$.
    ///
    /// Special cases:
    /// - The dot product of empty slices is 0.
    /// - If any term $x_iy_i$ is NaN (because an input is NaN, or because an infinity is paired
    ///   with 0), or if two terms are infinities of opposite signs, the dot product is NaN.
    /// - Otherwise, if any term is infinite, the dot product is that infinity.
    /// - If the exact dot product of finite terms is zero, the dot product is 0.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::dot_prec`]
    /// instead. If you want the exact dot product, consider using [`BigExponentFloat::dot`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m \log m \log\log m + n \log n)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs` and `ys` plus $n$ times `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `xs` and `ys` have different lengths, or if `rm` is `Exact` but
    /// the dot product cannot be represented exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [BigExponentFloat::from(PI), BigExponentFloat::from(E)];
    /// let ys = [BigExponentFloat::from(SQRT_2), BigExponentFloat::from(LN_2)];
    ///
    /// let (result, o) = BigExponentFloat::dot_prec_round(&xs, &ys, 5, Floor);
    /// assert_eq!(format!("{result:?}"), "0x6.4");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::dot_prec_round(&xs, &ys, 5, Ceiling);
    /// assert_eq!(format!("{result:?}"), "0x6.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::dot_prec_round(&xs, &ys, 5, Nearest);
    /// assert_eq!(format!("{result:?}"), "0x6.4");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::dot_prec_round(&xs, &ys, 20, Floor);
    /// assert_eq!(format!("{result:?}"), "0x6.53b98");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::dot_prec_round(&xs, &ys, 20, Ceiling);
    /// assert_eq!(format!("{result:?}"), "0x6.53ba");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::dot_prec_round(&xs, &ys, 20, Nearest);
    /// assert_eq!(format!("{result:?}"), "0x6.53b98");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn dot_prec_round(
        xs: &[Self],
        ys: &[Self],
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        dot_helper(xs, ys, prec, rm)
    }

    /// Computes the dot product of two equal-length slices of [`BigExponentFloat`]s, rounding the
    /// result to the nearest value of the specified precision. An [`Ordering`] is also returned,
    /// indicating whether the rounded dot product is less than, equal to, or greater than the exact
    /// dot product. Although `NaN`s are not comparable to any [`BigExponentFloat`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The products are computed exactly and summed as by [`BigExponentFloat::sum_prec_round`], so
    /// only one rounding occurs.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1},(y_i)_{i=0}^{n-1},p) = \sum_{i=0}^{n-1} x_iy_i+\varepsilon.
    /// $$
    /// - If $\sum_{i=0}^{n-1} x_iy_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\sum_{i=0}^{n-1} x_iy_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sum_{i=0}^{n-1} x_iy_i|\rfloor-p}$.
    ///
    /// Special cases:
    /// - The dot product of empty slices is 0.
    /// - If any term $x_iy_i$ is NaN (because an input is NaN, or because an infinity is paired
    ///   with 0), or if two terms are infinities of opposite signs, the dot product is NaN.
    /// - Otherwise, if any term is infinite, the dot product is that infinity.
    /// - If the exact dot product of finite terms is zero, the dot product is 0.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::dot_prec_round`] instead. If you want the exact dot product, consider
    /// using [`BigExponentFloat::dot`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m \log m \log\log m + n \log n)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs` and `ys` plus $n$ times `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, if `xs` and `ys` have different lengths.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, LN_2, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs = [BigExponentFloat::from(PI), BigExponentFloat::from(E)];
    /// let ys = [BigExponentFloat::from(SQRT_2), BigExponentFloat::from(LN_2)];
    ///
    /// let (result, o) = BigExponentFloat::dot_prec(&xs, &ys, 5);
    /// assert_eq!(format!("{result:?}"), "0x6.4");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::dot_prec(&xs, &ys, 20);
    /// assert_eq!(format!("{result:?}"), "0x6.53b98");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn dot_prec(xs: &[Self], ys: &[Self], prec: u64) -> (Self, Ordering) {
        dot_helper(xs, ys, prec, Nearest)
    }

    /// Computes the dot product of two equal-length slices of [`BigExponentFloat`]s. The dot
    /// product is exact.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1},(y_i)_{i=0}^{n-1}) = \sum_{i=0}^{n-1} x_iy_i.
    /// $$
    ///
    /// Special cases:
    /// - The dot product of empty slices is 0.
    /// - If any term $x_iy_i$ is NaN (because an input is NaN, or because an infinity is paired
    ///   with 0), or if two terms are infinities of opposite signs, the dot product is NaN.
    /// - Otherwise, if any term is infinite, the dot product is that infinity.
    /// - If the exact dot product of finite terms is zero, the dot product is 0.
    ///
    /// When the terms are finite and nonzero, the significand of the dot product spans the gaps
    /// between their exponents. To bound its size, use [`BigExponentFloat::dot_prec_round`] or
    /// [`BigExponentFloat::dot_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the significant bits of
    /// the inputs plus the difference between the largest exponent of a term and the smallest.
    ///
    /// # Panics
    /// Panics if `xs` and `ys` have different lengths, or if two terms' exponents differ by
    /// $2^{64}$ or more and both are finite and nonzero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    ///
    /// let xs = [BigExponentFloat::from(1.5), BigExponentFloat::from(0.25)];
    /// let ys = [BigExponentFloat::from(3.0), BigExponentFloat::from(-0.5)];
    /// assert_eq!(format!("{:?}", BigExponentFloat::dot(&xs, &ys)), "0x4.6");
    /// ```
    pub fn dot(xs: &[Self], ys: &[Self]) -> Self {
        assert_eq!(xs.len(), ys.len());
        xs.iter().zip(ys).map(|(x, y)| x * y).sum()
    }
}
