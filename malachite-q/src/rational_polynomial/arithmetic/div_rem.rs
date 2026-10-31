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

use crate::rational_polynomial::{RationalPolynomial, reduce};
use alloc::borrow::Cow;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{DivAssignRem, DivRem, Pow, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{CheckedDivRem, Polynomial, PseudoDivRem};
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::integer_vector::arithmetic::scalar_mul::integers_mul_scalar_assign;

// The polynomial `numerator / denominator`, in lowest terms, where `denominator` is nonzero and may
// be negative.
pub(crate) fn over_integer(
    numerator: IntegerPolynomial,
    denominator: Integer,
) -> RationalPolynomial {
    if denominator < 0u32 {
        reduce(-numerator, denominator.unsigned_abs())
    } else {
        reduce(numerator, denominator.unsigned_abs())
    }
}

// `p` times the integer `c`.
pub(crate) fn mul_integer(p: IntegerPolynomial, c: &Integer) -> IntegerPolynomial {
    let mut xs = p.into_coefficients_asc();
    integers_mul_scalar_assign(&mut xs, c);
    IntegerPolynomial::from_coefficients_asc(xs)
}

// The denominator that the numerators of the pseudo-quotient and pseudo-remainder of the numerators
// of `p` and `q`, with exponent `d`, are over: $a \ell^d$, where $a$ is the denominator of `p` and
// $\ell$ the leading coefficient of the numerator of `q`. With $\ell^d A = QB + R$ over the
// integers, $A/a = (bQ/(a \ell^d)) (B/b) + R/(a \ell^d)$.
pub(crate) fn pseudo_denominator(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    d: u64,
) -> Integer {
    Integer::from(&p.denominator) * q.numerator.leading_coefficient().pow(d)
}

// The quotient and remainder of `p` by `q`, or `None` if `q` is 0. An owned `p` is consumed where
// the result reuses it.
//
// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0, except that
// `None` is returned where FLINT throws.
pub(crate) fn checked_div_rem_helper(
    p: Cow<'_, RationalPolynomial>,
    q: &RationalPolynomial,
) -> Option<(RationalPolynomial, RationalPolynomial)> {
    if *q == RationalPolynomial::ZERO {
        return None;
    }
    let (x_len, y_len) = (p.numerator.len(), q.numerator.len());
    if x_len < y_len {
        return Some((RationalPolynomial::ZERO, p.into_owned()));
    }
    let b = Integer::from(&q.denominator);
    if y_len == 1 {
        let denominator = Integer::from(&p.denominator) * q.numerator.leading_coefficient();
        let numerator = p.into_owned().numerator;
        return Some((
            over_integer(mul_integer(numerator, &b), denominator),
            RationalPolynomial::ZERO,
        ));
    }
    let (quotient, remainder, d) = (&p.numerator).pseudo_div_rem(&q.numerator);
    let denominator = pseudo_denominator(&p, q, d);
    Some((
        over_integer(mul_integer(quotient, &b), denominator.clone()),
        over_integer(remainder, denominator),
    ))
}

pub(crate) fn div_rem_helper(
    p: Cow<'_, RationalPolynomial>,
    q: &RationalPolynomial,
) -> (RationalPolynomial, RationalPolynomial) {
    checked_div_rem_helper(p, q).expect("division by zero")
}

impl DivRem<Self> for RationalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, taking both by
    /// value.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The quotient and remainder are found from the pseudo-division of the numerators, $\ell^d a =
    /// bq + r$ over the integers, where $\ell$ is the leading coefficient of the divisor's
    /// numerator, by rescaling both by the denominators and $\ell^d$.
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .div_rem(RationalPolynomial::from_str("2*x+1").unwrap());
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0.
    #[inline]
    fn div_rem(self, other: Self) -> (Self, Self) {
        div_rem_helper(Cow::Owned(self), &other)
    }
}

impl DivRem<&Self> for RationalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, taking the first
    /// by value and the second by reference.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The quotient and remainder are found from the pseudo-division of the numerators, $\ell^d a =
    /// bq + r$ over the integers, where $\ell$ is the leading coefficient of the divisor's
    /// numerator, by rescaling both by the denominators and $\ell^d$.
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .div_rem(&RationalPolynomial::from_str("2*x+1").unwrap());
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0.
    #[inline]
    fn div_rem(self, other: &Self) -> (Self, Self) {
        div_rem_helper(Cow::Owned(self), other)
    }
}

impl DivRem<RationalPolynomial> for &RationalPolynomial {
    type DivOutput = RationalPolynomial;
    type RemOutput = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, taking the first
    /// by reference and the second by value.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The quotient and remainder are found from the pseudo-division of the numerators, $\ell^d a =
    /// bq + r$ over the integers, where $\ell$ is the leading coefficient of the divisor's
    /// numerator, by rescaling both by the denominators and $\ell^d$.
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .div_rem(RationalPolynomial::from_str("2*x+1").unwrap());
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0.
    #[inline]
    fn div_rem(self, other: RationalPolynomial) -> (RationalPolynomial, RationalPolynomial) {
        div_rem_helper(Cow::Borrowed(self), &other)
    }
}

impl DivRem<&RationalPolynomial> for &RationalPolynomial {
    type DivOutput = RationalPolynomial;
    type RemOutput = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, taking both by
    /// reference.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The quotient and remainder are found from the pseudo-division of the numerators, $\ell^d a =
    /// bq + r$ over the integers, where $\ell$ is the leading coefficient of the divisor's
    /// numerator, by rescaling both by the denominators and $\ell^d$.
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
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .div_rem(&RationalPolynomial::from_str("2*x+1").unwrap());
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0.
    #[inline]
    fn div_rem(self, other: &RationalPolynomial) -> (RationalPolynomial, RationalPolynomial) {
        div_rem_helper(Cow::Borrowed(self), other)
    }
}

impl DivAssignRem<Self> for RationalPolynomial {
    type RemOutput = Self;

    /// Divides a [`RationalPolynomial`] by another in place, returning the remainder, taking the
    /// right-hand side by value.
    ///
    /// $$
    /// a \gets q, \quad f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2+1").unwrap();
    /// assert_eq!(
    ///     p.div_assign_rem(RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .to_string(),
    ///     "5/4"
    /// );
    /// assert_eq!(p.to_string(), "1/2*x-1/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0.
    #[inline]
    fn div_assign_rem(&mut self, other: Self) -> Self {
        let (q, r) = div_rem_helper(Cow::Owned(take(self)), &other);
        *self = q;
        r
    }
}

impl DivAssignRem<&Self> for RationalPolynomial {
    type RemOutput = Self;

    /// Divides a [`RationalPolynomial`] by another in place, returning the remainder, taking the
    /// right-hand side by reference.
    ///
    /// $$
    /// a \gets q, \quad f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
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
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2+1").unwrap();
    /// assert_eq!(
    ///     p.div_assign_rem(&RationalPolynomial::from_str("2*x+1").unwrap())
    ///         .to_string(),
    ///     "5/4"
    /// );
    /// assert_eq!(p.to_string(), "1/2*x-1/4");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0.
    #[inline]
    fn div_assign_rem(&mut self, other: &Self) -> Self {
        let (q, r) = div_rem_helper(Cow::Owned(take(self)), other);
        *self = q;
        r
    }
}

impl CheckedDivRem<Self> for RationalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// divisor is 0, taking both by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } b \neq 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a = bq + r$ and $\deg r < \deg b$.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .checked_div_rem(RationalPolynomial::from_str("2*x+1").unwrap())
    ///     .unwrap();
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// assert!(
    ///     (RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div_rem(RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div_rem(self, other: Self) -> Option<(Self, Self)> {
        checked_div_rem_helper(Cow::Owned(self), &other)
    }
}

impl CheckedDivRem<&Self> for RationalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// divisor is 0, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } b \neq 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a = bq + r$ and $\deg r < \deg b$.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .checked_div_rem(&RationalPolynomial::from_str("2*x+1").unwrap())
    ///     .unwrap();
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// assert!(
    ///     (RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div_rem(&RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div_rem(self, other: &Self) -> Option<(Self, Self)> {
        checked_div_rem_helper(Cow::Owned(self), other)
    }
}

impl CheckedDivRem<RationalPolynomial> for &RationalPolynomial {
    type DivOutput = RationalPolynomial;
    type RemOutput = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// divisor is 0, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } b \neq 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a = bq + r$ and $\deg r < \deg b$.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .checked_div_rem(RationalPolynomial::from_str("2*x+1").unwrap())
    ///     .unwrap();
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// assert!(
    ///     (&RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div_rem(RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div_rem(
        self,
        other: RationalPolynomial,
    ) -> Option<(RationalPolynomial, RationalPolynomial)> {
        checked_div_rem_helper(Cow::Borrowed(self), &other)
    }
}

impl CheckedDivRem<&RationalPolynomial> for &RationalPolynomial {
    type DivOutput = RationalPolynomial;
    type RemOutput = RationalPolynomial;

    /// Divides two [`RationalPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// divisor is 0, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } b \neq 0, \\\\
    ///     \operatorname{None} & \text{otherwise},
    /// \\end{cases}
    /// $$
    /// where $a = bq + r$ and $\deg r < \deg b$.
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
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let (q, r) = (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///     .checked_div_rem(&RationalPolynomial::from_str("2*x+1").unwrap())
    ///     .unwrap();
    /// assert_eq!(q.to_string(), "1/2*x-1/4");
    /// assert_eq!(r.to_string(), "5/4");
    /// assert!(
    ///     (&RationalPolynomial::from_str("x").unwrap())
    ///         .checked_div_rem(&RationalPolynomial::ZERO)
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divrem` from `fmpq_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT throws.
    #[inline]
    fn checked_div_rem(
        self,
        other: &RationalPolynomial,
    ) -> Option<(RationalPolynomial, RationalPolynomial)> {
        checked_div_rem_helper(Cow::Borrowed(self), other)
    }
}
