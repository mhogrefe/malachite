// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::natural_polynomial::NaturalPolynomial;

/// The error returned when a [`RationalPolynomial`] with a coefficient that is negative or not an
/// integer is converted to a [`NaturalPolynomial`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalPolynomialFromRationalPolynomialError;

impl TryFrom<RationalPolynomial> for NaturalPolynomial {
    type Error = NaturalPolynomialFromRationalPolynomialError;

    /// Converts a [`RationalPolynomial`] to a [`NaturalPolynomial`], taking the
    /// [`RationalPolynomial`] by value and returning an error if any coefficient is negative or not
    /// an integer.
    ///
    /// Each coefficient's limbs are reused rather than copied. A successful conversion keeps the
    /// degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// See [here](super::natural_polynomial_from_rational_polynomial#try_from).
    fn try_from(p: RationalPolynomial) -> Result<Self, Self::Error> {
        if p.denominator == 1u32 {
            Self::try_from(p.numerator).map_err(|_| NaturalPolynomialFromRationalPolynomialError)
        } else {
            Err(NaturalPolynomialFromRationalPolynomialError)
        }
    }
}

impl TryFrom<&RationalPolynomial> for NaturalPolynomial {
    type Error = NaturalPolynomialFromRationalPolynomialError;

    /// Converts a [`RationalPolynomial`] to a [`NaturalPolynomial`], taking the
    /// [`RationalPolynomial`] by reference and returning an error if any coefficient is negative or
    /// not an integer.
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
    /// See [here](super::natural_polynomial_from_rational_polynomial#try_from).
    fn try_from(p: &RationalPolynomial) -> Result<Self, Self::Error> {
        if p.denominator == 1u32 {
            Self::try_from(&p.numerator).map_err(|_| NaturalPolynomialFromRationalPolynomialError)
        } else {
            Err(NaturalPolynomialFromRationalPolynomialError)
        }
    }
}

impl ConvertibleFrom<&RationalPolynomial> for NaturalPolynomial {
    /// Determines whether a [`RationalPolynomial`] can be converted to a [`NaturalPolynomial`]
    /// (when all its coefficients are non-negative integers). Takes the [`RationalPolynomial`] by
    /// reference.
    ///
    /// Unlike checking whether [`TryFrom`] succeeds, this allocates nothing.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// See [here](super::natural_polynomial_from_rational_polynomial#convertible_from).
    #[inline]
    fn convertible_from(p: &RationalPolynomial) -> bool {
        p.denominator == 1u32 && Self::convertible_from(&p.numerator)
    }
}
