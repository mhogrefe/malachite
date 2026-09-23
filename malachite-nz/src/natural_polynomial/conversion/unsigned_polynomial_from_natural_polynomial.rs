// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;

/// The error returned when a [`NaturalPolynomial`] has a coefficient too large for the coefficient
/// type of the [`UnsignedPolynomial`] it is being converted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedPolynomialFromNaturalPolynomialError;

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>> TryFrom<&NaturalPolynomial>
    for UnsignedPolynomial<T>
{
    type Error = UnsignedPolynomialFromNaturalPolynomialError;

    /// Converts a [`NaturalPolynomial`] to an [`UnsignedPolynomial`], taking the
    /// [`NaturalPolynomial`] by reference and returning an error if any coefficient is too large
    /// for `T`.
    ///
    /// No coefficient is ever wrapped. The leading coefficient stays nonzero, so a successful
    /// conversion keeps the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// See [here](super::unsigned_polynomial_from_natural_polynomial#try_from).
    fn try_from(p: &NaturalPolynomial) -> Result<Self, Self::Error> {
        Ok(Self::from_coefficients_asc(
            p.coefficients
                .iter()
                .map(|c| T::try_from(c).map_err(|_| UnsignedPolynomialFromNaturalPolynomialError))
                .collect::<Result<Vec<T>, _>>()?,
        ))
    }
}

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>> TryFrom<NaturalPolynomial>
    for UnsignedPolynomial<T>
{
    type Error = UnsignedPolynomialFromNaturalPolynomialError;

    /// Converts a [`NaturalPolynomial`] to an [`UnsignedPolynomial`], taking the
    /// [`NaturalPolynomial`] by value and returning an error if any coefficient is too large for
    /// `T`.
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
    /// See [here](super::unsigned_polynomial_from_natural_polynomial#try_from).
    #[inline]
    fn try_from(p: NaturalPolynomial) -> Result<Self, Self::Error> {
        Self::try_from(&p)
    }
}
