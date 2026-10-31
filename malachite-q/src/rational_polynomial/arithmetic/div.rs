// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::div_rem::{
    mul_integer, over_integer, pseudo_denominator,
};
use alloc::borrow::Cow;
use core::mem::take;
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::CheckedDiv;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Polynomial, PseudoDiv};
use malachite_nz::integer::Integer;

// The quotient of `p` by `q`, or `None` if `q` is 0.
//
// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0, except that `None` is
// returned where FLINT throws.
pub(crate) fn checked_div_helper(
    p: Cow<'_, RationalPolynomial>,
    q: &RationalPolynomial,
) -> Option<RationalPolynomial> {
    if *q == RationalPolynomial::ZERO {
        return None;
    }
    let (x_len, y_len) = (p.numerator.len(), q.numerator.len());
    if x_len < y_len {
        return Some(RationalPolynomial::ZERO);
    }
    let b = Integer::from(&q.denominator);
    if y_len == 1 {
        let denominator = Integer::from(&p.denominator) * q.numerator.leading_coefficient();
        return Some(over_integer(
            mul_integer(p.into_owned().numerator, &b),
            denominator,
        ));
    }
    let (quotient, d) = (&p.numerator).pseudo_div(&q.numerator);
    Some(over_integer(
        mul_integer(quotient, &b),
        pseudo_denominator(&p, q, d),
    ))
}

fn div_helper(p: Cow<'_, RationalPolynomial>, q: &RationalPolynomial) -> RationalPolynomial {
    checked_div_helper(p, q).expect("division by zero")
}

impl Div<Self> for RationalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`RationalPolynomial`]s, taking both by value.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2+1").unwrap()
    ///         / RationalPolynomial::from_str("2*x+1").unwrap())
    ///     .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0.
    #[inline]
    fn div(self, other: Self) -> Self {
        div_helper(Cow::Owned(self), &other)
    }
}

impl Div<&Self> for RationalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`RationalPolynomial`]s, taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2+1").unwrap()
    ///         / &RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0.
    #[inline]
    fn div(self, other: &Self) -> Self {
        div_helper(Cow::Owned(self), other)
    }
}

impl Div<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Finds the quotient of two [`RationalPolynomial`]s, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2+1").unwrap()
    ///         / RationalPolynomial::from_str("2*x+1").unwrap())
    ///     .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0.
    #[inline]
    fn div(self, other: RationalPolynomial) -> RationalPolynomial {
        div_helper(Cow::Borrowed(self), &other)
    }
}

impl Div<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Finds the quotient of two [`RationalPolynomial`]s, taking both by reference.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2+1").unwrap()
    ///         / &RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0.
    #[inline]
    fn div(self, other: &RationalPolynomial) -> RationalPolynomial {
        div_helper(Cow::Borrowed(self), other)
    }
}

impl DivAssign<Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] by its quotient by another, taking the right-hand side by
    /// value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2+1").unwrap();
    /// p /= RationalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!(p.to_string(), "1/2*x-1/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0.
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = div_helper(Cow::Owned(take(self)), &other);
    }
}

impl DivAssign<&Self> for RationalPolynomial {
    /// Replaces a [`RationalPolynomial`] by its quotient by another, taking the right-hand side by
    /// reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2+1").unwrap();
    /// p /= &RationalPolynomial::from_str("2*x+1").unwrap();
    /// assert_eq!(p.to_string(), "1/2*x-1/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0.
    #[inline]
    fn div_assign(&mut self, other: &Self) {
        *self = div_helper(Cow::Owned(take(self)), other);
    }
}

impl CheckedDiv<Self> for RationalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`RationalPolynomial`]s, or `None` if the divisor is 0, taking
    /// both by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div(RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// assert!(
    ///     (RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div(RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div(self, other: Self) -> Option<Self> {
        checked_div_helper(Cow::Owned(self), &other)
    }
}

impl CheckedDiv<&Self> for RationalPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`RationalPolynomial`]s, or `None` if the divisor is 0, taking the
    /// first by value and the second by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div(&RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// assert!(
    ///     (RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div(&RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div(self, other: &Self) -> Option<Self> {
        checked_div_helper(Cow::Owned(self), other)
    }
}

impl CheckedDiv<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Finds the quotient of two [`RationalPolynomial`]s, or `None` if the divisor is 0, taking the
    /// first by reference and the second by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div(RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// assert!(
    ///     (&RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div(RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div(self, other: RationalPolynomial) -> Option<RationalPolynomial> {
        checked_div_helper(Cow::Borrowed(self), &other)
    }
}

impl CheckedDiv<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Finds the quotient of two [`RationalPolynomial`]s, or `None` if the divisor is 0, taking
    /// both by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// numerators and denominators of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div(&RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "1/2*x-1/4"
    /// );
    /// assert!(
    ///     (&RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div(&RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_div` from `fmpq_poly/div.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div(self, other: &RationalPolynomial) -> Option<RationalPolynomial> {
        checked_div_helper(Cow::Borrowed(self), other)
    }
}
