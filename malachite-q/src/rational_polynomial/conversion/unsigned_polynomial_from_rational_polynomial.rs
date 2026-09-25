// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;

/// The error returned when a [`RationalPolynomial`] has a coefficient that is negative, not an
/// integer, or too large for the coefficient type of the [`UnsignedPolynomial`] it is being
/// converted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedPolynomialFromRationalPolynomialError;

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>> TryFrom<&RationalPolynomial>
    for UnsignedPolynomial<T>
{
    type Error = UnsignedPolynomialFromRationalPolynomialError;

    /// Converts a [`RationalPolynomial`] to an [`UnsignedPolynomial`], taking the
    /// [`RationalPolynomial`] by reference and returning an error if any coefficient is negative,
    /// not an integer, or too large for `T`.
    ///
    /// No coefficient is ever wrapped or rounded. A successful conversion keeps the degree.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// See [here](super::unsigned_polynomial_from_rational_polynomial#try_from).
    fn try_from(p: &RationalPolynomial) -> Result<Self, Self::Error> {
        if p.denominator == 1u32 {
            Self::try_from(&p.numerator).map_err(|_| UnsignedPolynomialFromRationalPolynomialError)
        } else {
            Err(UnsignedPolynomialFromRationalPolynomialError)
        }
    }
}

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>> TryFrom<RationalPolynomial>
    for UnsignedPolynomial<T>
{
    type Error = UnsignedPolynomialFromRationalPolynomialError;

    /// Converts a [`RationalPolynomial`] to an [`UnsignedPolynomial`], taking the
    /// [`RationalPolynomial`] by value and returning an error if any coefficient is negative, not
    /// an integer, or too large for `T`.
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
    /// See [here](super::unsigned_polynomial_from_rational_polynomial#try_from).
    #[inline]
    fn try_from(p: RationalPolynomial) -> Result<Self, Self::Error> {
        Self::try_from(&p)
    }
}

impl<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer>>
    ConvertibleFrom<&RationalPolynomial> for UnsignedPolynomial<T>
{
    /// Determines whether a [`RationalPolynomial`] can be converted to an [`UnsignedPolynomial`]
    /// (when all its coefficients are integers representable as a `T`). Takes the
    /// [`RationalPolynomial`] by reference.
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
    /// See [here](super::unsigned_polynomial_from_rational_polynomial#convertible_from).
    #[inline]
    fn convertible_from(p: &RationalPolynomial) -> bool {
        p.denominator == 1u32 && Self::convertible_from(&p.numerator)
    }
}
