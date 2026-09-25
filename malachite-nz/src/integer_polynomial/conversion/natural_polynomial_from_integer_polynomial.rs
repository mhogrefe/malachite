// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::polynomial::Polynomial;

/// The error returned when an [`IntegerPolynomial`] with a negative coefficient is converted to a
/// [`NaturalPolynomial`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalPolynomialFromIntegerPolynomialError;

impl TryFrom<IntegerPolynomial> for NaturalPolynomial {
    type Error = NaturalPolynomialFromIntegerPolynomialError;

    /// Converts an [`IntegerPolynomial`] to a [`NaturalPolynomial`], taking the
    /// [`IntegerPolynomial`] by value and returning an error if any coefficient is negative.
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
    /// See [here](super::natural_polynomial_from_integer_polynomial#try_from).
    fn try_from(p: IntegerPolynomial) -> Result<Self, Self::Error> {
        Ok(Self::from_coefficients_asc(
            p.coefficients
                .into_iter()
                .map(|c| {
                    Natural::try_from(c).map_err(|_| NaturalPolynomialFromIntegerPolynomialError)
                })
                .collect::<Result<Vec<Natural>, _>>()?,
        ))
    }
}

impl TryFrom<&IntegerPolynomial> for NaturalPolynomial {
    type Error = NaturalPolynomialFromIntegerPolynomialError;

    /// Converts an [`IntegerPolynomial`] to a [`NaturalPolynomial`], taking the
    /// [`IntegerPolynomial`] by reference and returning an error if any coefficient is negative.
    ///
    /// A successful conversion keeps the degree.
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
    /// See [here](super::natural_polynomial_from_integer_polynomial#try_from).
    fn try_from(p: &IntegerPolynomial) -> Result<Self, Self::Error> {
        Ok(Self::from_coefficients_asc(
            p.coefficients
                .iter()
                .map(|c| {
                    Natural::try_from(c).map_err(|_| NaturalPolynomialFromIntegerPolynomialError)
                })
                .collect::<Result<Vec<Natural>, _>>()?,
        ))
    }
}

impl ConvertibleFrom<&IntegerPolynomial> for NaturalPolynomial {
    /// Determines whether an [`IntegerPolynomial`] can be converted to a [`NaturalPolynomial`]
    /// (when none of its coefficients is negative). Takes the [`IntegerPolynomial`] by reference.
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
    /// See [here](super::natural_polynomial_from_integer_polynomial#convertible_from).
    #[inline]
    fn convertible_from(p: &IntegerPolynomial) -> bool {
        p.coefficients.iter().all(Natural::convertible_from)
    }
}
