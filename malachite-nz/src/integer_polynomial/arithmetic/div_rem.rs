// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::pseudo_div_rem::{div_rem_divide_and_conquer, trimmed};
use malachite_base::num::arithmetic::traits::{DivAssignRem, DivRem};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::CheckedDivRem;

// The quotient and remainder of `p` by `q`, or `None` if `q` is 0 or the quotient over the
// rationals is not integral: the divisor's leading coefficient must divide the leading coefficient
// of every remainder along the way.
//
// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with `exact`
// set, except that `None` is returned where FLINT returns 0.
pub(crate) fn checked_div_rem_helper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> Option<(IntegerPolynomial, IntegerPolynomial)> {
    if *q == IntegerPolynomial::ZERO {
        return None;
    }
    if p.coefficients.len() < q.coefficients.len() {
        return Some((IntegerPolynomial::ZERO, p.clone()));
    }
    let (qs, rs) = div_rem_divide_and_conquer(&p.coefficients, &q.coefficients)?;
    Some((trimmed(qs), trimmed(rs)))
}

pub(crate) fn div_rem_helper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (IntegerPolynomial, IntegerPolynomial) {
    assert_ne!(*q, IntegerPolynomial::ZERO, "division by zero");
    checked_div_rem_helper(p, q).expect("the quotient must have integer coefficients")
}

impl DivRem<Self> for IntegerPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, taking the
    /// dividend by value and the divisor by value.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Use [`CheckedDivRem`] or
    /// [`PseudoDivRem`](malachite_base::polynomial::PseudoDivRem) when it may not be.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = p.div_rem(q);
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_rem(self, other: Self) -> (Self, Self) {
        div_rem_helper(&self, &other)
    }
}

impl DivRem<&Self> for IntegerPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, taking the
    /// dividend by value and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Use [`CheckedDivRem`] or
    /// [`PseudoDivRem`](malachite_base::polynomial::PseudoDivRem) when it may not be.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = p.div_rem(&q);
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_rem(self, other: &Self) -> (Self, Self) {
        div_rem_helper(&self, other)
    }
}

impl DivRem<IntegerPolynomial> for &IntegerPolynomial {
    type DivOutput = IntegerPolynomial;
    type RemOutput = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, taking the
    /// dividend by reference and the divisor by value.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Use [`CheckedDivRem`] or
    /// [`PseudoDivRem`](malachite_base::polynomial::PseudoDivRem) when it may not be.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = (&p).div_rem(q);
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_rem(self, other: IntegerPolynomial) -> (IntegerPolynomial, IntegerPolynomial) {
        div_rem_helper(self, &other)
    }
}

impl DivRem<&IntegerPolynomial> for &IntegerPolynomial {
    type DivOutput = IntegerPolynomial;
    type RemOutput = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, taking the
    /// dividend by reference and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = (q, r), \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Use [`CheckedDivRem`] or
    /// [`PseudoDivRem`](malachite_base::polynomial::PseudoDivRem) when it may not be.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = (&p).div_rem(&q);
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_rem(self, other: &IntegerPolynomial) -> (IntegerPolynomial, IntegerPolynomial) {
        div_rem_helper(self, other)
    }
}

impl DivAssignRem<Self> for IntegerPolynomial {
    type RemOutput = Self;

    /// Divides an [`IntegerPolynomial`] by another in place, replacing it with the quotient and
    /// returning the remainder, taking the divisor by value.
    ///
    /// $$
    /// a \gets q, \quad a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!(p.div_assign_rem(q).to_string(), "4");
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_assign_rem(&mut self, other: Self) -> Self {
        let (q, r) = div_rem_helper(self, &other);
        *self = q;
        r
    }
}

impl DivAssignRem<&Self> for IntegerPolynomial {
    type RemOutput = Self;

    /// Divides an [`IntegerPolynomial`] by another in place, replacing it with the quotient and
    /// returning the remainder, taking the divisor by reference.
    ///
    /// $$
    /// a \gets q, \quad a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::DivAssignRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!(p.div_assign_rem(&q).to_string(), "4");
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_assign_rem(&mut self, other: &Self) -> Self {
        let (q, r) = div_rem_helper(self, other);
        *self = q;
        r
    }
}

impl CheckedDivRem<Self> for IntegerPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// division is impossible, taking the dividend by value and the divisor by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } a = bq + r \text{ with } \deg r < \deg b, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = p.checked_div_rem(q).unwrap();
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!(p.checked_div_rem(q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div_rem(self, other: Self) -> Option<(Self, Self)> {
        checked_div_rem_helper(&self, &other)
    }
}

impl CheckedDivRem<&Self> for IntegerPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// division is impossible, taking the dividend by value and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } a = bq + r \text{ with } \deg r < \deg b, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = p.checked_div_rem(&q).unwrap();
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!(p.checked_div_rem(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div_rem(self, other: &Self) -> Option<(Self, Self)> {
        checked_div_rem_helper(&self, other)
    }
}

impl CheckedDivRem<IntegerPolynomial> for &IntegerPolynomial {
    type DivOutput = IntegerPolynomial;
    type RemOutput = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// division is impossible, taking the dividend by reference and the divisor by value.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } a = bq + r \text{ with } \deg r < \deg b, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = (&p).checked_div_rem(q).unwrap();
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!((&p).checked_div_rem(q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div_rem(
        self,
        other: IntegerPolynomial,
    ) -> Option<(IntegerPolynomial, IntegerPolynomial)> {
        checked_div_rem_helper(self, &other)
    }
}

impl CheckedDivRem<&IntegerPolynomial> for &IntegerPolynomial {
    type DivOutput = IntegerPolynomial;
    type RemOutput = IntegerPolynomial;

    /// Divides two [`IntegerPolynomial`]s, returning the quotient and remainder, or `None` if the
    /// division is impossible, taking the dividend by reference and the divisor by reference.
    ///
    /// $$
    /// f(a, b) = \\begin{cases}
    ///     \operatorname{Some}((q, r)) & \text{if } a = bq + r \text{ with } \deg r < \deg b, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// let (s, r) = (&p).checked_div_rem(&q).unwrap();
    /// assert_eq!(s.to_string(), "2*x+1");
    /// assert_eq!(r.to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!((&p).checked_div_rem(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div_rem(
        self,
        other: &IntegerPolynomial,
    ) -> Option<(IntegerPolynomial, IntegerPolynomial)> {
        checked_div_rem_helper(self, other)
    }
}
