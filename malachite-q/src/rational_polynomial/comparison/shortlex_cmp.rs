// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::comparison::cmp::cmp_coefficients_same_length;
use crate::rational_polynomial::{ShortlexRationalPolynomial, ShortlexRationalPolynomialRef};
use core::cmp::Ordering::{self, *};

impl Ord for ShortlexRationalPolynomialRef<'_> {
    /// Compares two [`ShortlexRationalPolynomialRef`]s.
    ///
    /// The order is shortlex: the polynomials are compared first by degree, and then, in case of a
    /// tie, by their coefficients from highest to lowest. The zero polynomial, having no degree at
    /// all, comes first. This is a total order, and its equality agrees with
    /// [`RationalPolynomial`](crate::rational_polynomial::RationalPolynomial) equality. It is the
    /// order FLINT gives polynomials, the one `fmpq_poly_cmp` implements.
    ///
    /// Where the degrees differ this parts company with
    /// [`RationalPolynomial`](crate::rational_polynomial::RationalPolynomial)'s own [`Ord`], which
    /// compares polynomials by how they behave for large arguments and so lets a negative leading
    /// coefficient send a high-degree polynomial to the bottom. Here degree decides outright, so
    /// $-x^3 > x^2$ where the asymptotic order has $-x^3 < x^2$. Once the degrees agree the two
    /// orders are the same, and this shares the other's coefficient comparison, which reduces
    /// nothing and never builds a common denominator.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, counting numerator coefficients and denominator. Polynomials of
    /// different degrees are compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::{RationalPolynomial, ShortlexRationalPolynomialRef};
    ///
    /// let zero = RationalPolynomial::from_str("0").unwrap();
    /// let x_squared = RationalPolynomial::from_str("x^2").unwrap();
    /// let negative_x_cubed = RationalPolynomial::from_str("-x^3").unwrap();
    ///
    /// // The zero polynomial comes first.
    /// assert!(ShortlexRationalPolynomialRef(&zero) < ShortlexRationalPolynomialRef(&x_squared));
    /// // Degree decides, whatever the sign of the leading coefficient.
    /// assert!(
    ///     ShortlexRationalPolynomialRef(&x_squared)
    ///         < ShortlexRationalPolynomialRef(&negative_x_cubed)
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let xs = self.0.numerator.coefficients_asc();
        let ys = other.0.numerator.coefficients_asc();
        // The numerator is held with no trailing zero, so its length is the degree plus one, and 0
        // for the zero polynomial; comparing lengths is comparing degrees.
        match xs.len().cmp(&ys.len()) {
            Equal => {
                cmp_coefficients_same_length(xs, &self.0.denominator, ys, &other.0.denominator)
            }
            c => c,
        }
    }
}

impl PartialOrd for ShortlexRationalPolynomialRef<'_> {
    /// Compares two [`ShortlexRationalPolynomialRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortlexRationalPolynomial {
    /// Compares two [`ShortlexRationalPolynomial`]s.
    ///
    /// The order is shortlex: the polynomials are compared first by degree, and then, in case of a
    /// tie, by their coefficients from highest to lowest. See the [`Ord`] implementation for
    /// [`ShortlexRationalPolynomialRef`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, counting numerator coefficients and denominator. Polynomials of
    /// different degrees are compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_polynomial::{RationalPolynomial, ShortlexRationalPolynomial};
    ///
    /// let x_squared = RationalPolynomial::from_str("x^2").unwrap();
    /// let negative_x_cubed = RationalPolynomial::from_str("-x^3").unwrap();
    ///
    /// // Degree decides, whatever the sign of the leading coefficient.
    /// assert!(
    ///     ShortlexRationalPolynomial(x_squared) < ShortlexRationalPolynomial(negative_x_cubed)
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl PartialOrd for ShortlexRationalPolynomial {
    /// Compares two [`ShortlexRationalPolynomial`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
