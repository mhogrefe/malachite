// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::{ShortlexIntegerPolynomial, ShortlexIntegerPolynomialRef};
use core::cmp::Ordering;

impl Ord for ShortlexIntegerPolynomialRef<'_> {
    /// Compares two [`ShortlexIntegerPolynomialRef`]s.
    ///
    /// The order is shortlex: the polynomials are compared first by degree, and then, in case of a
    /// tie, by their coefficients from highest to lowest. The zero polynomial, having no degree at
    /// all, comes first. This is a total order, and its equality agrees with
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial) equality. It is the
    /// order FLINT uses for polynomials, the one `fmpq_poly_cmp` implements.
    ///
    /// Where the degrees differ this parts company with
    /// [`IntegerPolynomial`](crate::integer_polynomial::IntegerPolynomial)'s own [`Ord`], which
    /// compares polynomials by how they behave for large arguments and so lets a negative leading
    /// coefficient send a high-degree polynomial to the bottom. Here degree decides outright, so
    /// $-x^3 > x^2$ where the asymptotic order has $-x^3 < x^2$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, summed over their coefficients. Polynomials of different degrees are
    /// compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::{IntegerPolynomial, ShortlexIntegerPolynomialRef};
    ///
    /// let zero = IntegerPolynomial::from_str("0").unwrap();
    /// let x_squared = IntegerPolynomial::from_str("x^2").unwrap();
    /// let negative_x_cubed = IntegerPolynomial::from_str("-x^3").unwrap();
    ///
    /// // The zero polynomial comes first.
    /// assert!(ShortlexIntegerPolynomialRef(&zero) < ShortlexIntegerPolynomialRef(&x_squared));
    /// // Degree decides, whatever the sign of the leading coefficient.
    /// assert!(
    ///     ShortlexIntegerPolynomialRef(&x_squared)
    ///         < ShortlexIntegerPolynomialRef(&negative_x_cubed)
    /// );
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        // The coefficients are held with no trailing zero, so the length is the degree plus one,
        // and 0 for the zero polynomial; comparing lengths is comparing degrees. Once the lengths
        // agree the iterators have the same length, so comparing them from the leading coefficient
        // down is the comparison at the highest coefficient where the two differ.
        self.0
            .coefficients
            .len()
            .cmp(&other.0.coefficients.len())
            .then_with(|| {
                self.0
                    .coefficients
                    .iter()
                    .rev()
                    .cmp(other.0.coefficients.iter().rev())
            })
    }
}

impl PartialOrd for ShortlexIntegerPolynomialRef<'_> {
    /// Compares two [`ShortlexIntegerPolynomialRef`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &ShortlexIntegerPolynomialRef) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortlexIntegerPolynomial {
    /// Compares two [`ShortlexIntegerPolynomial`]s.
    ///
    /// The order is shortlex: the polynomials are compared first by degree, and then, in case of a
    /// tie, by their coefficients from highest to lowest. See the [`Ord`] implementation for
    /// [`ShortlexIntegerPolynomialRef`] for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the smaller of the two polynomials'
    /// total number of bits, summed over their coefficients. Polynomials of different degrees are
    /// compared in constant time.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::{IntegerPolynomial, ShortlexIntegerPolynomial};
    ///
    /// let x_squared = IntegerPolynomial::from_str("x^2").unwrap();
    /// let negative_x_cubed = IntegerPolynomial::from_str("-x^3").unwrap();
    ///
    /// // Degree decides, whatever the sign of the leading coefficient.
    /// assert!(ShortlexIntegerPolynomial(x_squared) < ShortlexIntegerPolynomial(negative_x_cubed));
    /// ```
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(&other.as_ref())
    }
}

impl PartialOrd for ShortlexIntegerPolynomial {
    /// Compares two [`ShortlexIntegerPolynomial`]s.
    ///
    /// See the documentation for the [`Ord`] implementation.
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
