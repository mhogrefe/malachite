// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use alloc::vec::Vec;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;

/// The error returned when a [`RationalMatrix`] has an element that is negative or too large for
/// the element type of the [`UnsignedMatrix`] it is being converted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedMatrixFromRationalMatrixError;

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Rational>> TryFrom<&RationalMatrix>
    for UnsignedMatrix<T>
{
    type Error = UnsignedMatrixFromRationalMatrixError;

    /// Converts a [`RationalMatrix`] to an [`UnsignedMatrix`], taking the [`RationalMatrix`] by
    /// reference and returning an error if any element is negative, is not an integer, or is too
    /// large for `T`.
    ///
    /// No element is ever rounded or wrapped, and no negative element is ever reinterpreted as
    /// unsigned. A successful conversion keeps the height and width.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.elements.len()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_matrix_from_rational_matrix#try_from).
    fn try_from(m: &RationalMatrix) -> Result<Self, Self::Error> {
        Ok(Self::from_owned_elements(
            m.height(),
            m.width(),
            m.elements
                .iter()
                .map(|x| T::try_from(x).map_err(|_| UnsignedMatrixFromRationalMatrixError))
                .collect::<Result<Vec<T>, _>>()?,
        ))
    }
}

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Rational>> TryFrom<RationalMatrix>
    for UnsignedMatrix<T>
{
    type Error = UnsignedMatrixFromRationalMatrixError;

    /// Converts a [`RationalMatrix`] to an [`UnsignedMatrix`], taking the [`RationalMatrix`] by
    /// value and returning an error if any element is negative, is not an integer, or is too large
    /// for `T`.
    ///
    /// Taking the matrix by value saves nothing, since the elements are copied into new storage
    /// either way; this is here so that a conversion can be written without a borrow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.elements.len()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_matrix_from_rational_matrix#try_from).
    #[inline]
    fn try_from(m: RationalMatrix) -> Result<Self, Self::Error> {
        Self::try_from(&m)
    }
}

impl<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Rational>> ConvertibleFrom<&RationalMatrix>
    for UnsignedMatrix<T>
{
    /// Determines whether a [`RationalMatrix`] can be converted to an [`UnsignedMatrix`] (when
    /// every element is representable as a `T`). Takes the [`RationalMatrix`] by reference.
    ///
    /// Unlike checking whether [`TryFrom`] succeeds, this allocates nothing.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.elements.len()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_matrix_from_rational_matrix#convertible_from).
    #[inline]
    fn convertible_from(m: &RationalMatrix) -> bool {
        m.elements.iter().all(|x| T::convertible_from(x))
    }
}
