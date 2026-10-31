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
use crate::rational_polynomial::arithmetic::div_rem::{mul_integer, over_integer};
use alloc::borrow::Cow;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{Content, DivExact, DivExactAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::CheckedDivExact;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;
use malachite_nz::natural::Natural;

// The content and primitive part of the numerator of `p`, which is nonzero, consuming an owned `p`.
pub(crate) fn content_and_primitive_part(
    p: Cow<'_, RationalPolynomial>,
) -> (Natural, IntegerPolynomial) {
    let c = (&p.numerator).content();
    let numerator = match p {
        Cow::Owned(p) => p.numerator,
        Cow::Borrowed(p) if c == 1u32 => p.numerator.clone(),
        Cow::Borrowed(p) => return (c.clone(), (&p.numerator).div_exact(Integer::from(&c))),
    };
    if c == 1u32 {
        (c, numerator)
    } else {
        let pp = numerator.div_exact(Integer::from(&c));
        (c, pp)
    }
}

// The polynomial $(c_1 b / (a c_2)) r$, where $r$ is the quotient of the primitive parts of the
// numerators of `p` and `q`, $c_1$ and $c_2$ their contents, and $a$ and $b$ the denominators of
// `p` and `q`: $p/q$, when $r$ is.
fn scale_quotient(
    a: &Natural,
    q: &RationalPolynomial,
    r: IntegerPolynomial,
    c_1: Natural,
    c_2: Natural,
) -> RationalPolynomial {
    over_integer(
        mul_integer(r, &Integer::from(c_1 * &q.denominator)),
        Integer::from(a * c_2),
    )
}

// The quotient of `p` by `q`, or `None` if `q` is 0 or does not divide `p`. The quotient of two
// rational polynomials is the quotient of their primitive parts, scaled.
//
// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, except that
// `None` is returned for a zero divisor, where FLINT returns the quotient 0 for a zero dividend.
pub(crate) fn checked_div_exact_helper(
    p: Cow<'_, RationalPolynomial>,
    q: &RationalPolynomial,
) -> Option<RationalPolynomial> {
    if *q == RationalPolynomial::ZERO {
        return None;
    }
    if *p == RationalPolynomial::ZERO {
        return Some(RationalPolynomial::ZERO);
    }
    let a = p.denominator.clone();
    let (c_1, pp_1) = content_and_primitive_part(p);
    let (c_2, pp_2) = content_and_primitive_part(Cow::Borrowed(q));
    let r = pp_1.checked_div_exact(pp_2)?;
    Some(scale_quotient(&a, q, r, c_1, c_2))
}

// The quotient of `p` by `q`, which must divide it; if it does not, the result is unspecified.
fn div_exact_helper(p: Cow<'_, RationalPolynomial>, q: &RationalPolynomial) -> RationalPolynomial {
    assert_ne!(*q, RationalPolynomial::ZERO, "division by zero");
    if *p == RationalPolynomial::ZERO {
        return RationalPolynomial::ZERO;
    }
    let a = p.denominator.clone();
    let (c_1, pp_1) = content_and_primitive_part(p);
    let (c_2, pp_2) = content_and_primitive_part(Cow::Borrowed(q));
    let r = pp_1.div_exact(pp_2);
    scale_quotient(&a, q, r, c_1, c_2)
}

impl DivExact<Self> for RationalPolynomial {
    type Output = Self;

    /// Divides a [`RationalPolynomial`] by another that divides it, taking both by value. If
    /// `other` does not divide `self`, the result is unspecified; use
    /// [`checked_div_exact`](malachite_base::polynomial::CheckedDivExact::checked_div_exact) when
    /// it may not.
    ///
    /// $$
    /// f(a, b) = a/b.
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .div_exact(RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, without
    /// the check that the division is exact.
    #[inline]
    fn div_exact(self, other: Self) -> Self {
        div_exact_helper(Cow::Owned(self), &other)
    }
}

impl DivExact<&Self> for RationalPolynomial {
    type Output = Self;

    /// Divides a [`RationalPolynomial`] by another that divides it, taking the first by value and
    /// the second by reference. If `other` does not divide `self`, the result is unspecified; use
    /// [`checked_div_exact`](malachite_base::polynomial::CheckedDivExact::checked_div_exact) when
    /// it may not.
    ///
    /// $$
    /// f(a, b) = a/b.
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .div_exact(&RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, without
    /// the check that the division is exact.
    #[inline]
    fn div_exact(self, other: &Self) -> Self {
        div_exact_helper(Cow::Owned(self), other)
    }
}

impl DivExact<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides a [`RationalPolynomial`] by another that divides it, taking the first by reference
    /// and the second by value. If `other` does not divide `self`, the result is unspecified; use
    /// [`checked_div_exact`](malachite_base::polynomial::CheckedDivExact::checked_div_exact) when
    /// it may not.
    ///
    /// $$
    /// f(a, b) = a/b.
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .div_exact(RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, without
    /// the check that the division is exact.
    #[inline]
    fn div_exact(self, other: RationalPolynomial) -> RationalPolynomial {
        div_exact_helper(Cow::Borrowed(self), &other)
    }
}

impl DivExact<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides a [`RationalPolynomial`] by another that divides it, taking both by reference. If
    /// `other` does not divide `self`, the result is unspecified; use
    /// [`checked_div_exact`](malachite_base::polynomial::CheckedDivExact::checked_div_exact) when
    /// it may not.
    ///
    /// $$
    /// f(a, b) = a/b.
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::num::arithmetic::traits::DivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .div_exact(&RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, without
    /// the check that the division is exact.
    #[inline]
    fn div_exact(self, other: &RationalPolynomial) -> RationalPolynomial {
        div_exact_helper(Cow::Borrowed(self), other)
    }
}

impl DivExactAssign<Self> for RationalPolynomial {
    /// Divides a [`RationalPolynomial`] in place by another that divides it, taking the right-hand
    /// side by value. If `other` does not divide `self`, the result is unspecified.
    ///
    /// $$
    /// a \gets a/b.
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
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2-1/4").unwrap();
    /// p.div_exact_assign(RationalPolynomial::from_str("x-1/2").unwrap());
    /// assert_eq!(p.to_string(), "x+1/2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, without
    /// the check that the division is exact.
    #[inline]
    fn div_exact_assign(&mut self, other: Self) {
        *self = div_exact_helper(Cow::Owned(take(self)), &other);
    }
}

impl DivExactAssign<&Self> for RationalPolynomial {
    /// Divides a [`RationalPolynomial`] in place by another that divides it, taking the right-hand
    /// side by reference. If `other` does not divide `self`, the result is unspecified.
    ///
    /// $$
    /// a \gets a/b.
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
    /// use malachite_base::num::arithmetic::traits::DivExactAssign;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("x^2-1/4").unwrap();
    /// p.div_exact_assign(&RationalPolynomial::from_str("x-1/2").unwrap());
    /// assert_eq!(p.to_string(), "x+1/2");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, without
    /// the check that the division is exact.
    #[inline]
    fn div_exact_assign(&mut self, other: &Self) {
        *self = div_exact_helper(Cow::Owned(take(self)), other);
    }
}

impl CheckedDivExact<Self> for RationalPolynomial {
    type Output = Self;

    /// Divides a [`RationalPolynomial`] by another, returning `None` if the divisor is 0 or does
    /// not divide the dividend, taking both by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(a/b) & \text{if } b \neq 0 \text{ and } b \mid a, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .checked_div_exact(RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// assert!(
    ///     (RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div_exact(RationalPolynomial::from_str("x+1").unwrap())
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a zero divisor, where FLINT returns the quotient 0 for a zero
    /// dividend.
    #[inline]
    fn checked_div_exact(self, other: Self) -> Option<Self> {
        checked_div_exact_helper(Cow::Owned(self), &other)
    }
}

impl CheckedDivExact<&Self> for RationalPolynomial {
    type Output = Self;

    /// Divides a [`RationalPolynomial`] by another, returning `None` if the divisor is 0 or does
    /// not divide the dividend, taking the first by value and the second by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(a/b) & \text{if } b \neq 0 \text{ and } b \mid a, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .checked_div_exact(&RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// assert!(
    ///     (RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div_exact(&RationalPolynomial::from_str("x+1").unwrap())
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a zero divisor, where FLINT returns the quotient 0 for a zero
    /// dividend.
    #[inline]
    fn checked_div_exact(self, other: &Self) -> Option<Self> {
        checked_div_exact_helper(Cow::Owned(self), other)
    }
}

impl CheckedDivExact<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides a [`RationalPolynomial`] by another, returning `None` if the divisor is 0 or does
    /// not divide the dividend, taking the first by reference and the second by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(a/b) & \text{if } b \neq 0 \text{ and } b \mid a, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .checked_div_exact(RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// assert!(
    ///     (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div_exact(RationalPolynomial::from_str("x+1").unwrap())
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a zero divisor, where FLINT returns the quotient 0 for a zero
    /// dividend.
    #[inline]
    fn checked_div_exact(self, other: RationalPolynomial) -> Option<RationalPolynomial> {
        checked_div_exact_helper(Cow::Borrowed(self), &other)
    }
}

impl CheckedDivExact<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Divides a [`RationalPolynomial`] by another, returning `None` if the divisor is 0 or does
    /// not divide the dividend, taking both by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}(a/b) & \text{if } b \neq 0 \text{ and } b \mid a, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// The quotient is that of the primitive parts of the numerators over the integers, scaled by
    /// the contents and denominators.
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
    /// use malachite_base::polynomial::CheckedDivExact;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("x^2-1/4").unwrap())
    ///         .checked_div_exact(&RationalPolynomial::from_str("x-1/2").unwrap())
    ///         .unwrap()
    ///         .to_string(),
    ///     "x+1/2"
    /// );
    /// assert!(
    ///     (&RationalPolynomial::from_str("x^2+1").unwrap())
    ///         .checked_div_exact(&RationalPolynomial::from_str("x+1").unwrap())
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_divides` from `fmpq_poly/divides.c`, FLINT 3.6.0, except
    /// that `None` is returned for a zero divisor, where FLINT returns the quotient 0 for a zero
    /// dividend.
    #[inline]
    fn checked_div_exact(self, other: &RationalPolynomial) -> Option<RationalPolynomial> {
        checked_div_exact_helper(Cow::Borrowed(self), other)
    }
}
