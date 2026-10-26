// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;

impl CharacteristicPolynomial for NaturalMatrix {
    type Output = IntegerPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`], taking it by value.
    ///
    /// The characteristic polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic
    /// polynomial of degree $n$; that of the $0 \times 0$ matrix is 1. Its coefficients may be
    /// negative, so it is an [`IntegerPolynomial`]. It is computed as for an [`IntegerMatrix`] with
    /// the same elements; see [`characteristic_polynomial`](
    /// malachite_base::num::arithmetic::traits::CharacteristicPolynomial).
    ///
    /// $$
    /// f(A) = \det(xI - A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `characteristic_polynomial` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!(a.clone().characteristic_polynomial().to_string(), "1");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^2-5*x-2"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^2-5*x+6"
    /// );
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().characteristic_polynomial().to_string(),
    ///     "x^3-16*x^2-12*x+3"
    /// );
    /// ```
    #[inline]
    fn characteristic_polynomial(self) -> IntegerPolynomial {
        IntegerMatrix::from(self).characteristic_polynomial()
    }
}

impl CharacteristicPolynomial for &NaturalMatrix {
    type Output = IntegerPolynomial;

    /// Computes the characteristic polynomial of a square [`NaturalMatrix`], taking it by
    /// reference.
    ///
    /// The characteristic polynomial of an $n \times n$ matrix $A$ is $\det(xI - A)$, a monic
    /// polynomial of degree $n$; that of the $0 \times 0$ matrix is 1. Its coefficients may be
    /// negative, so it is an [`IntegerPolynomial`]. It is computed as for an [`IntegerMatrix`] with
    /// the same elements; see [`characteristic_polynomial`](
    /// malachite_base::num::arithmetic::traits::CharacteristicPolynomial).
    ///
    /// $$
    /// f(A) = \det(xI - A).
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `characteristic_polynomial` for an [`IntegerMatrix`] with the same
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CharacteristicPolynomial;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[]_{0,0}").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "1");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "x^2-5*x-2");
    ///
    /// let a = NaturalMatrix::from_str("[2, 0; 0, 3]").unwrap();
    /// assert_eq!((&a).characteristic_polynomial().to_string(), "x^2-5*x+6");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).characteristic_polynomial().to_string(),
    ///     "x^3-16*x^2-12*x+3"
    /// );
    /// ```
    #[inline]
    fn characteristic_polynomial(self) -> IntegerPolynomial {
        IntegerMatrix::from(self).characteristic_polynomial()
    }
}
