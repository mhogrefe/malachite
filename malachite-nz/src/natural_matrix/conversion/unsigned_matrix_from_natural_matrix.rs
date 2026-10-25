// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;

/// The error returned when a [`NaturalMatrix`] has an element too large for the element type of the
/// [`UnsignedMatrix`] it is being converted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedMatrixFromNaturalMatrixError;

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>> TryFrom<&NaturalMatrix>
    for UnsignedMatrix<T>
{
    type Error = UnsignedMatrixFromNaturalMatrixError;

    /// Converts a [`NaturalMatrix`] to an [`UnsignedMatrix`], taking the [`NaturalMatrix`] by
    /// reference and returning an error if any element is too large for `T`.
    ///
    /// No element is ever wrapped. A successful conversion keeps the height and width.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.height() * m.width()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_matrix_from_natural_matrix#try_from).
    fn try_from(m: &NaturalMatrix) -> Result<Self, Self::Error> {
        Ok(Self::from_owned_elements(
            m.height(),
            m.width(),
            m.elements
                .iter()
                .map(|x| T::try_from(x).map_err(|_| UnsignedMatrixFromNaturalMatrixError))
                .collect::<Result<Vec<T>, _>>()?,
        ))
    }
}

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Natural>> TryFrom<NaturalMatrix>
    for UnsignedMatrix<T>
{
    type Error = UnsignedMatrixFromNaturalMatrixError;

    /// Converts a [`NaturalMatrix`] to an [`UnsignedMatrix`], taking the [`NaturalMatrix`] by value
    /// and returning an error if any element is too large for `T`.
    ///
    /// Taking the matrix by value saves nothing, since the elements are copied into new storage
    /// either way; this is here so that a conversion can be written without a borrow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.height() * m.width()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_matrix_from_natural_matrix#try_from).
    #[inline]
    fn try_from(m: NaturalMatrix) -> Result<Self, Self::Error> {
        Self::try_from(&m)
    }
}

impl<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Natural>> ConvertibleFrom<&NaturalMatrix>
    for UnsignedMatrix<T>
{
    /// Determines whether a [`NaturalMatrix`] can be converted to an [`UnsignedMatrix`] (when every
    /// element is representable as a `T`). Takes the [`NaturalMatrix`] by reference.
    ///
    /// Unlike checking whether [`TryFrom`] succeeds, this allocates nothing.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.height() * m.width()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_matrix_from_natural_matrix#convertible_from).
    #[inline]
    fn convertible_from(m: &NaturalMatrix) -> bool {
        m.elements.iter().all(|x| T::convertible_from(x))
    }
}
