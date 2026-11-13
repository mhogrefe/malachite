// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::big_exponent_float::arithmetic::mul::mul_helper;
use alloc::borrow::Cow;
use alloc::vec::Vec;
use core::cmp::Ordering;
use core::iter::Product;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, One, Zero as ZeroTrait,
};
use malachite_base::rounding_modes::RoundingMode;

// The exact product of the factors. The finite factors are multiplied in a balanced tree, so that
// the operands of each multiplication have similar sizes.
pub(crate) fn product_exact<'a, I: Iterator<Item = Cow<'a, BigExponentFloat>>>(
    xs: I,
) -> BigExponentFloat {
    let mut sign = true;
    let mut has_zero = false;
    let mut has_infinity = false;
    let mut factors = Vec::new();
    for x in xs {
        match &*x {
            BigExponentFloat(NaN) => return BigExponentFloat::NAN,
            BigExponentFloat(Zero) => has_zero = true,
            BigExponentFloat(Infinity { sign: s }) => {
                has_infinity = true;
                sign ^= !*s;
            }
            BigExponentFloat(Finite { sign: s, .. }) => {
                sign ^= !*s;
                factors.push(x);
            }
        }
    }
    if has_infinity {
        return if has_zero {
            BigExponentFloat::NAN
        } else if sign {
            BigExponentFloat::INFINITY
        } else {
            BigExponentFloat::NEGATIVE_INFINITY
        };
    }
    if has_zero {
        return BigExponentFloat::ZERO;
    }
    while factors.len() > 1 {
        let mut next = Vec::with_capacity(factors.len().div_ceil(2));
        let mut it = factors.into_iter();
        while let Some(x) = it.next() {
            next.push(match it.next() {
                Some(y) => Cow::Owned(mul_helper(x, y, None).0),
                None => x,
            });
        }
        factors = next;
    }
    factors.pop().map_or(BigExponentFloat::ONE, Cow::into_owned)
}

impl BigExponentFloat {
    /// Multiplies a slice of [`BigExponentFloat`]s, rounding the result to the specified precision
    /// and with the specified rounding mode. An [`Ordering`] is also returned, indicating whether
    /// the rounded product is less than, equal to, or greater than the exact product. Although
    /// `NaN`s are not comparable to any [`BigExponentFloat`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The exact product is formed and then rounded once.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1},p,m) = \prod_{i=0}^{n-1} x_i+\varepsilon.
    /// $$
    /// - If $\prod_{i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\prod_{i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is not `Nearest`, then
    ///   $|\varepsilon| < 2^{\lfloor\log_2 |\prod_{i=0}^{n-1} x_i|\rfloor-p+1}$.
    /// - If $\prod_{i=0}^{n-1} x_i$ is finite and nonzero, and $m$ is `Nearest`, then
    ///   $|\varepsilon| \leq 2^{\lfloor\log_2 |\prod_{i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// Special cases:
    /// - The product of an empty slice is 1.
    /// - If any factor is NaN, or if both an infinity and 0 are factors, the product is NaN.
    /// - Otherwise, if an infinity is a factor, the product is an infinity whose sign is the
    ///   product of the factors' signs.
    /// - Otherwise, if 0 is a factor, the product is 0.
    ///
    /// If you know you'll be using `Nearest`, consider using [`BigExponentFloat::product_prec`]
    /// instead. If you want the exact product, consider using [`Product::product`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m \log^2 m \log\log m + n)$
    ///
    /// $M(n, m) = O(m \log m + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the product cannot be represented
    /// exactly with `prec` bits.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs =
    ///     [BigExponentFloat::from(PI), BigExponentFloat::from(E), BigExponentFloat::from(SQRT_2)];
    ///
    /// let (result, o) = BigExponentFloat::product_prec_round(&xs, 5, Floor);
    /// assert_eq!(format!("{result:?}"), "0xc.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::product_prec_round(&xs, 5, Ceiling);
    /// assert_eq!(format!("{result:?}"), "0xc.8");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::product_prec_round(&xs, 5, Nearest);
    /// assert_eq!(format!("{result:?}"), "0xc.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::product_prec_round(&xs, 20, Floor);
    /// assert_eq!(format!("{result:?}"), "0xc.13b6");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::product_prec_round(&xs, 20, Ceiling);
    /// assert_eq!(format!("{result:?}"), "0xc.13b7");
    /// assert_eq!(o, Greater);
    ///
    /// let (result, o) = BigExponentFloat::product_prec_round(&xs, 20, Nearest);
    /// assert_eq!(format!("{result:?}"), "0xc.13b7");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn product_prec_round(xs: &[Self], prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        product_exact(xs.iter().map(Cow::Borrowed)).round_to_prec_round(prec, rm)
    }

    /// Multiplies a slice of [`BigExponentFloat`]s, rounding the result to the nearest value of the
    /// specified precision. An [`Ordering`] is also returned, indicating whether the rounded
    /// product is less than, equal to, or greater than the exact product. Although `NaN`s are not
    /// comparable to any [`BigExponentFloat`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The exact product is formed and then rounded once.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1},p) = \prod_{i=0}^{n-1} x_i+\varepsilon.
    /// $$
    /// - If $\prod_{i=0}^{n-1} x_i$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or
    ///   assumed to be 0.
    /// - If $\prod_{i=0}^{n-1} x_i$ is finite and nonzero, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\prod_{i=0}^{n-1} x_i|\rfloor-p}$.
    ///
    /// Special cases:
    /// - The product of an empty slice is 1.
    /// - If any factor is NaN, or if both an infinity and 0 are factors, the product is NaN.
    /// - Otherwise, if an infinity is a factor, the product is an infinity whose sign is the
    ///   product of the factors' signs.
    /// - Otherwise, if 0 is a factor, the product is 0.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`BigExponentFloat::product_prec_round`] instead. If you want the exact product, consider
    /// using [`Product::product`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m \log^2 m \log\log m + n)$
    ///
    /// $M(n, m) = O(m \log m + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is the sum of the
    /// significant bits of the elements of `xs`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::{E, PI, SQRT_2};
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let xs =
    ///     [BigExponentFloat::from(PI), BigExponentFloat::from(E), BigExponentFloat::from(SQRT_2)];
    ///
    /// let (result, o) = BigExponentFloat::product_prec(&xs, 5);
    /// assert_eq!(format!("{result:?}"), "0xc.0");
    /// assert_eq!(o, Less);
    ///
    /// let (result, o) = BigExponentFloat::product_prec(&xs, 20);
    /// assert_eq!(format!("{result:?}"), "0xc.13b7");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn product_prec(xs: &[Self], prec: u64) -> (Self, Ordering) {
        product_exact(xs.iter().map(Cow::Borrowed)).round_to_prec(prec)
    }
}

impl Product<Self> for BigExponentFloat {
    /// Multiplies the [`BigExponentFloat`]s produced by an iterator, taken by value. The product is
    /// exact.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1}) = \prod_{i=0}^{n-1} x_i.
    /// $$
    ///
    /// Special cases:
    /// - The product of an empty slice is 1.
    /// - If any factor is NaN, or if both an infinity and 0 are factors, the product is NaN.
    /// - Otherwise, if an infinity is a factor, the product is an infinity whose sign is the
    ///   product of the factors' signs.
    /// - Otherwise, if 0 is a factor, the product is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m \log^2 m \log\log m + n)$
    ///
    /// $M(n, m) = O(m \log m + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of factors, and $m$ is the
    /// sum of their significant bits.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let xs = [
    ///     BigExponentFloat::from(1.5),
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    /// ];
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::product(xs.into_iter())),
    ///     "0x1.2"
    /// );
    /// ```
    #[inline]
    fn product<I: Iterator<Item = Self>>(xs: I) -> Self {
        product_exact(xs.map(Cow::Owned))
    }
}

impl<'a> Product<&'a Self> for BigExponentFloat {
    /// Multiplies the [`BigExponentFloat`]s produced by an iterator, taken by reference. The
    /// product is exact.
    ///
    /// $$
    /// f((x_i)_{i=0}^{n-1}) = \prod_{i=0}^{n-1} x_i.
    /// $$
    ///
    /// Special cases:
    /// - The product of an empty slice is 1.
    /// - If any factor is NaN, or if both an infinity and 0 are factors, the product is NaN.
    /// - Otherwise, if an infinity is a factor, the product is an infinity whose sign is the
    ///   product of the factors' signs.
    /// - Otherwise, if 0 is a factor, the product is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m \log^2 m \log\log m + n)$
    ///
    /// $M(n, m) = O(m \log m + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of factors, and $m$ is the
    /// sum of their significant bits.
    ///
    /// # Examples
    /// ```
    /// use core::iter::Product;
    /// use malachite_float::BigExponentFloat;
    ///
    /// let xs = [
    ///     BigExponentFloat::from(1.5),
    ///     BigExponentFloat::from(0.25),
    ///     BigExponentFloat::from(3.0),
    /// ];
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::product(xs.iter())),
    ///     "0x1.2"
    /// );
    /// ```
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(xs: I) -> Self {
        product_exact(xs.map(Cow::Borrowed))
    }
}
