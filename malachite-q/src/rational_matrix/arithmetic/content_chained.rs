// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::num::arithmetic::traits::{Content, ContentChained, Gcd};

impl ContentChained<Rational> for RationalMatrix {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalMatrix`], taking the matrix by value
    /// and `x` by value.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -3/4; 0, 1/4]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .content_chained(Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     a.content_chained(Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let a = RationalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(
    ///     a.content_chained(Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: Rational) -> Rational {
        self.content().gcd(x)
    }
}

impl ContentChained<&Rational> for RationalMatrix {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalMatrix`], taking the matrix by value
    /// and `x` by reference.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -3/4; 0, 1/4]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .content_chained(&Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     a.content_chained(&Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let a = RationalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(
    ///     a.content_chained(&Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: &Rational) -> Rational {
        self.content().gcd(x)
    }
}

impl ContentChained<Rational> for &RationalMatrix {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalMatrix`], taking the matrix by
    /// reference and `x` by value.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -3/4; 0, 1/4]").unwrap();
    /// assert_eq!(
    ///     (&a).content_chained(Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     (&a).content_chained(Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let a = RationalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(
    ///     (&a).content_chained(Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: Rational) -> Rational {
        self.content().gcd(x)
    }
}

impl ContentChained<&Rational> for &RationalMatrix {
    type Output = Rational;

    /// Computes the GCD of `x` and the elements of a [`RationalMatrix`], taking the matrix by
    /// reference and `x` by reference.
    ///
    /// The GCD of `x` and the elements of a matrix of zeros, or of a matrix with no elements, is
    /// `x`. The computation stops as soon as the GCD reaches 1.
    ///
    /// $$
    /// f(A, x) = \gcd\left(x, \gcd_{i, j} A_{ij}\right).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements and of `x`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ContentChained;
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, -3/4; 0, 1/4]").unwrap();
    /// assert_eq!(
    ///     (&a).content_chained(&Rational::from_str("1/6").unwrap())
    ///         .to_string(),
    ///     "1/12"
    /// );
    /// assert_eq!(
    ///     (&a).content_chained(&Rational::from_str("0").unwrap())
    ///         .to_string(),
    ///     "1/4"
    /// );
    /// let a = RationalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(
    ///     (&a).content_chained(&Rational::from_str("1/3").unwrap())
    ///         .to_string(),
    ///     "1/3"
    /// );
    /// ```
    #[inline]
    fn content_chained(self, x: &Rational) -> Rational {
        self.content().gcd(x)
    }
}
