// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::integer_polynomial::IntegerPolynomial;

/// The error returned when a [`RationalPolynomial`] with a non-integer coefficient is converted to
/// an [`IntegerPolynomial`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegerPolynomialFromRationalPolynomialError;

impl TryFrom<RationalPolynomial> for IntegerPolynomial {
    type Error = IntegerPolynomialFromRationalPolynomialError;

    /// Converts a [`RationalPolynomial`] to an [`IntegerPolynomial`], taking the
    /// [`RationalPolynomial`] by value and returning an error if any coefficient is not an integer.
    ///
    /// Since the representation is in lowest terms, every coefficient is an integer exactly when
    /// the denominator is 1, and then the numerator is the result; it is moved out, not copied.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::integer_polynomial_from_rational_polynomial#try_from).
    fn try_from(p: RationalPolynomial) -> Result<Self, Self::Error> {
        if p.denominator == 1u32 {
            Ok(p.numerator)
        } else {
            Err(IntegerPolynomialFromRationalPolynomialError)
        }
    }
}

impl TryFrom<&RationalPolynomial> for IntegerPolynomial {
    type Error = IntegerPolynomialFromRationalPolynomialError;

    /// Converts a [`RationalPolynomial`] to an [`IntegerPolynomial`], taking the
    /// [`RationalPolynomial`] by reference and returning an error if any coefficient is not an
    /// integer.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients.
    ///
    /// # Examples
    /// See [here](super::integer_polynomial_from_rational_polynomial#try_from).
    fn try_from(p: &RationalPolynomial) -> Result<Self, Self::Error> {
        if p.denominator == 1u32 {
            Ok(p.numerator.clone())
        } else {
            Err(IntegerPolynomialFromRationalPolynomialError)
        }
    }
}

impl ConvertibleFrom<&RationalPolynomial> for IntegerPolynomial {
    /// Determines whether a [`RationalPolynomial`] can be converted to an [`IntegerPolynomial`]
    /// (when all its coefficients are integers). Takes the [`RationalPolynomial`] by reference.
    ///
    /// Unlike checking whether [`TryFrom`] succeeds, this allocates nothing.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// See [here](super::integer_polynomial_from_rational_polynomial#convertible_from).
    #[inline]
    fn convertible_from(p: &RationalPolynomial) -> bool {
        p.denominator == 1u32
    }
}
