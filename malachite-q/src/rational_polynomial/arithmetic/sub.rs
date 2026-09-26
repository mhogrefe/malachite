// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use crate::rational_polynomial::arithmetic::add::add_or_sub_owned_ref;
use core::mem::take;
use core::ops::{Sub, SubAssign};
use core::ptr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Computes $q - p$ reusing the storage of $q$, then negates it to give $p - q$, as FLINT does when
// the output aliases the second operand.
fn sub_ref_owned(p: &RationalPolynomial, q: RationalPolynomial) -> RationalPolynomial {
    -add_or_sub_owned_ref(
        q.numerator,
        q.denominator,
        &p.numerator,
        &p.denominator,
        true,
    )
}

// Subtracts two `RationalPolynomial`s taken by value, reusing the storage of the one with the
// longer numerator.
fn sub_owned_owned(p: RationalPolynomial, q: RationalPolynomial) -> RationalPolynomial {
    if q.numerator.len() > p.numerator.len() {
        sub_ref_owned(&p, q)
    } else {
        add_or_sub_owned_ref(
            p.numerator,
            p.denominator,
            &q.numerator,
            &q.denominator,
            true,
        )
    }
}

impl Sub<Self> for RationalPolynomial {
    type Output = Self;

    /// Subtracts one [`RationalPolynomial`] from another, taking both by value.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// The difference is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$,
    /// the numerators are scaled to the common denominator $ab/g$ and subtracted, and then only a
    /// factor of $g$ can be shared by the new numerator and denominator, so only that is divided
    /// out. When the two polynomials have the same degree, their leading coefficients can cancel,
    /// and then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         - RationalPolynomial::from_str("-1/2*x+1/6").unwrap())
    ///     .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         - RationalPolynomial::from_str("1/2*x^2-1/5*x").unwrap())
    ///     .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub` from `fmpq_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn sub(self, other: Self) -> Self {
        sub_owned_owned(self, other)
    }
}

impl Sub<&Self> for RationalPolynomial {
    type Output = Self;

    /// Subtracts one [`RationalPolynomial`] from another, taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// The difference is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$,
    /// the numerators are scaled to the common denominator $ab/g$ and subtracted, and then only a
    /// factor of $g$ can be shared by the new numerator and denominator, so only that is divided
    /// out. When the two polynomials have the same degree, their leading coefficients can cancel,
    /// and then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         - &RationalPolynomial::from_str("-1/2*x+1/6").unwrap())
    ///         .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         - &RationalPolynomial::from_str("1/2*x^2-1/5*x").unwrap())
    ///         .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub` from `fmpq_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn sub(self, other: &Self) -> Self {
        add_or_sub_owned_ref(
            self.numerator,
            self.denominator,
            &other.numerator,
            &other.denominator,
            true,
        )
    }
}

impl Sub<RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Subtracts one [`RationalPolynomial`] from another, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// The difference is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$,
    /// the numerators are scaled to the common denominator $ab/g$ and subtracted, and then only a
    /// factor of $g$ can be shared by the new numerator and denominator, so only that is divided
    /// out. When the two polynomials have the same degree, their leading coefficients can cancel,
    /// and then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         - RationalPolynomial::from_str("-1/2*x+1/6").unwrap())
    ///     .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         - RationalPolynomial::from_str("1/2*x^2-1/5*x").unwrap())
    ///     .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub` from `fmpq_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn sub(self, other: RationalPolynomial) -> RationalPolynomial {
        sub_ref_owned(self, other)
    }
}

impl Sub<&RationalPolynomial> for &RationalPolynomial {
    type Output = RationalPolynomial;

    /// Subtracts one [`RationalPolynomial`] from another, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p - q.
    /// $$
    ///
    /// The difference is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$,
    /// the numerators are scaled to the common denominator $ab/g$ and subtracted, and then only a
    /// factor of $g$ can be shared by the new numerator and denominator, so only that is divided
    /// out. When the two polynomials have the same degree, their leading coefficients can cancel,
    /// and then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x+1/3").unwrap()
    ///         - &RationalPolynomial::from_str("-1/2*x+1/6").unwrap())
    ///         .to_string(),
    ///     "x+1/6"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&RationalPolynomial::from_str("1/2*x^2+1/3").unwrap()
    ///         - &RationalPolynomial::from_str("1/2*x^2-1/5*x").unwrap())
    ///         .to_string(),
    ///     "1/5*x+1/3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub` from `fmpq_poly/sub.c`, FLINT 3.6.0.
    fn sub(self, other: &RationalPolynomial) -> RationalPolynomial {
        if ptr::eq(self, other) {
            return RationalPolynomial::ZERO;
        }
        add_or_sub_owned_ref(
            self.numerator.clone(),
            self.denominator.clone(),
            &other.numerator,
            &other.denominator,
            true,
        )
    }
}

impl SubAssign<Self> for RationalPolynomial {
    /// Subtracts a [`RationalPolynomial`] from a [`RationalPolynomial`] in place, taking the second
    /// by value.
    ///
    /// $$
    /// p \gets p - q.
    /// $$
    ///
    /// The difference is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$,
    /// the numerators are scaled to the common denominator $ab/g$ and subtracted, and then only a
    /// factor of $g$ can be shared by the new numerator and denominator, so only that is divided
    /// out. When the two polynomials have the same degree, their leading coefficients can cancel,
    /// and then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p -= RationalPolynomial::from_str("-1/2*x+1/6").unwrap();
    /// assert_eq!(p.to_string(), "x+1/6");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// p -= RationalPolynomial::from_str("1/2*x^2-1/5*x").unwrap();
    /// assert_eq!(p.to_string(), "1/5*x+1/3");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub` from `fmpq_poly/sub.c`, FLINT 3.6.0.
    fn sub_assign(&mut self, other: Self) {
        *self = sub_owned_owned(take(self), other);
    }
}

impl SubAssign<&Self> for RationalPolynomial {
    /// Subtracts a [`RationalPolynomial`] from a [`RationalPolynomial`] in place, taking the second
    /// by reference.
    ///
    /// $$
    /// p \gets p - q.
    /// $$
    ///
    /// The difference is kept in lowest terms. With denominators $a$ and $b$ and $g = \gcd(a, b)$,
    /// the numerators are scaled to the common denominator $ab/g$ and subtracted, and then only a
    /// factor of $g$ can be shared by the new numerator and denominator, so only that is divided
    /// out. When the two polynomials have the same degree, their leading coefficients can cancel,
    /// and then the degree of the difference is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// numerators' coefficients and the denominators of both polynomials.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let mut p = RationalPolynomial::from_str("1/2*x+1/3").unwrap();
    /// p -= &RationalPolynomial::from_str("-1/2*x+1/6").unwrap();
    /// assert_eq!(p.to_string(), "x+1/6");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = RationalPolynomial::from_str("1/2*x^2+1/3").unwrap();
    /// p -= &RationalPolynomial::from_str("1/2*x^2-1/5*x").unwrap();
    /// assert_eq!(p.to_string(), "1/5*x+1/3");
    /// ```
    ///
    /// This is equivalent to `fmpq_poly_sub` from `fmpq_poly/sub.c`, FLINT 3.6.0.
    fn sub_assign(&mut self, other: &Self) {
        let p = take(self);
        *self = add_or_sub_owned_ref(
            p.numerator,
            p.denominator,
            &other.numerator,
            &other.denominator,
            true,
        );
    }
}
