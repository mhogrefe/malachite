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
use crate::integer_polynomial::arithmetic::div_rem::{checked_div_rem_helper, div_rem_helper};
use core::ops::{Rem, RemAssign};
use malachite_base::polynomial::CheckedRem;

impl Rem<Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`IntegerPolynomial`]s, taking the dividend by value and the
    /// divisor by value.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Unlike `%` with an [`Integer`](crate::integer::Integer),
    /// which reduces each coefficient, this is the remainder of polynomial division.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((p % q).to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem` from `fmpz_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn rem(self, other: Self) -> Self {
        div_rem_helper(&self, &other).1
    }
}

impl Rem<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`IntegerPolynomial`]s, taking the dividend by value and the
    /// divisor by reference.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Unlike `%` with an [`Integer`](crate::integer::Integer),
    /// which reduces each coefficient, this is the remainder of polynomial division.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((p % &q).to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem` from `fmpz_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn rem(self, other: &Self) -> Self {
        div_rem_helper(&self, other).1
    }
}

impl Rem<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the remainder of two [`IntegerPolynomial`]s, taking the dividend by reference and the
    /// divisor by value.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Unlike `%` with an [`Integer`](crate::integer::Integer),
    /// which reduces each coefficient, this is the remainder of polynomial division.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p % q).to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem` from `fmpz_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn rem(self, other: IntegerPolynomial) -> IntegerPolynomial {
        div_rem_helper(self, &other).1
    }
}

impl Rem<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the remainder of two [`IntegerPolynomial`]s, taking the dividend by reference and the
    /// divisor by reference.
    ///
    /// $$
    /// f(a, b) = r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way. Unlike `%` with an [`Integer`](crate::integer::Integer),
    /// which reduces each coefficient, this is the remainder of polynomial division.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p % &q).to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem` from `fmpz_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn rem(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        div_rem_helper(self, other).1
    }
}

impl RemAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its remainder by another, taking the divisor by
    /// value.
    ///
    /// $$
    /// a \gets r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// p %= q;
    /// assert_eq!(p.to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem` from `fmpz_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn rem_assign(&mut self, other: Self) {
        *self = div_rem_helper(self, &other).1;
    }
}

impl RemAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its remainder by another, taking the divisor by
    /// reference.
    ///
    /// $$
    /// a \gets r, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// p %= &q;
    /// assert_eq!(p.to_string(), "4");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_rem` from `fmpz_poly/rem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn rem_assign(&mut self, other: &Self) {
        *self = div_rem_helper(self, other).1;
    }
}

impl CheckedRem<Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by value and the divisor by value.
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
    /// use malachite_base::polynomial::CheckedRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!(p.checked_rem(q).unwrap().to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!(p.checked_rem(q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_rem(self, other: Self) -> Option<Self> {
        checked_div_rem_helper(&self, &other).map(|(_, r)| r)
    }
}

impl CheckedRem<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the remainder of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by value and the divisor by reference.
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
    /// use malachite_base::polynomial::CheckedRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!(p.checked_rem(&q).unwrap().to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!(p.checked_rem(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_rem(self, other: &Self) -> Option<Self> {
        checked_div_rem_helper(&self, other).map(|(_, r)| r)
    }
}

impl CheckedRem<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the remainder of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by reference and the divisor by value.
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
    /// use malachite_base::polynomial::CheckedRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p).checked_rem(q).unwrap().to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!((&p).checked_rem(q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_rem(self, other: IntegerPolynomial) -> Option<IntegerPolynomial> {
        checked_div_rem_helper(self, &other).map(|(_, r)| r)
    }
}

impl CheckedRem<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the remainder of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by reference and the divisor by reference.
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
    /// use malachite_base::polynomial::CheckedRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p).checked_rem(&q).unwrap().to_string(), "4");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!((&p).checked_rem(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_rem(self, other: &IntegerPolynomial) -> Option<IntegerPolynomial> {
        checked_div_rem_helper(self, other).map(|(_, r)| r)
    }
}
