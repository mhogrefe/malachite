// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

/// The error returned when an [`IntegerPolynomial`] has a coefficient that is negative or too large
/// for the coefficient type of the [`UnsignedPolynomial`] it is being converted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedPolynomialFromIntegerPolynomialError;

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>> TryFrom<&IntegerPolynomial>
    for UnsignedPolynomial<T>
{
    type Error = UnsignedPolynomialFromIntegerPolynomialError;

    /// Converts an [`IntegerPolynomial`] to an [`UnsignedPolynomial`], taking the
    /// [`IntegerPolynomial`] by reference and returning an error if any coefficient is negative or
    /// too large for `T`.
    ///
    /// No coefficient is ever wrapped. A successful conversion keeps the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// See [here](super::unsigned_polynomial_from_integer_polynomial#try_from).
    fn try_from(p: &IntegerPolynomial) -> Result<Self, Self::Error> {
        Ok(Self::from_coefficients_asc(
            p.coefficients
                .iter()
                .map(|c| T::try_from(c).map_err(|_| UnsignedPolynomialFromIntegerPolynomialError))
                .collect::<Result<Vec<T>, _>>()?,
        ))
    }
}

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>> TryFrom<IntegerPolynomial>
    for UnsignedPolynomial<T>
{
    type Error = UnsignedPolynomialFromIntegerPolynomialError;

    /// Converts an [`IntegerPolynomial`] to an [`UnsignedPolynomial`], taking the
    /// [`IntegerPolynomial`] by value and returning an error if any coefficient is negative or too
    /// large for `T`.
    ///
    /// Taking the polynomial by value saves nothing, since the coefficients are copied into new
    /// storage either way; this is here so that a conversion can be written without a borrow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// See [here](super::unsigned_polynomial_from_integer_polynomial#try_from).
    #[inline]
    fn try_from(p: IntegerPolynomial) -> Result<Self, Self::Error> {
        Self::try_from(&p)
    }
}

impl<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer>>
    ConvertibleFrom<&IntegerPolynomial> for UnsignedPolynomial<T>
{
    /// Determines whether an [`IntegerPolynomial`] can be converted to an [`UnsignedPolynomial`]
    /// (when every coefficient is non-negative and representable as a `T`). Takes the
    /// [`IntegerPolynomial`] by reference.
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
    /// See [here](super::unsigned_polynomial_from_integer_polynomial#convertible_from).
    #[inline]
    fn convertible_from(p: &IntegerPolynomial) -> bool {
        p.coefficients.iter().all(|c| T::convertible_from(c))
    }
}
