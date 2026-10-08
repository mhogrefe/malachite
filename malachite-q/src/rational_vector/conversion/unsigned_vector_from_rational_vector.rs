// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::Rational;
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::unsigned_vector::UnsignedVector;

/// The error returned when a [`RationalVector`] has an element too large for the element type of
/// the [`UnsignedVector`] it is being converted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedVectorFromRationalVectorError;

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Rational>> TryFrom<&RationalVector>
    for UnsignedVector<T>
{
    type Error = UnsignedVectorFromRationalVectorError;

    /// Converts a [`RationalVector`] to an [`UnsignedVector`], taking the [`RationalVector`] by
    /// reference and returning an error if any element is negative, not an integer, or too large
    /// for `T`.
    ///
    /// No element is ever rounded or wrapped, and no negative element is ever reinterpreted as
    /// unsigned. A successful conversion keeps the dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_vector_from_rational_vector#try_from).
    fn try_from(v: &RationalVector) -> Result<Self, Self::Error> {
        Ok(Self {
            elements: v
                .elements
                .iter()
                .map(|x| T::try_from(x).map_err(|_| UnsignedVectorFromRationalVectorError))
                .collect::<Result<Vec<T>, _>>()?,
        })
    }
}

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Rational>> TryFrom<RationalVector>
    for UnsignedVector<T>
{
    type Error = UnsignedVectorFromRationalVectorError;

    /// Converts a [`RationalVector`] to an [`UnsignedVector`], taking the [`RationalVector`] by
    /// value and returning an error if any element is negative, not an integer, or too large for
    /// `T`.
    ///
    /// Taking the vector by value saves nothing, since the elements are copied into new storage
    /// either way; this is here so that a conversion can be written without a borrow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_vector_from_rational_vector#try_from).
    #[inline]
    fn try_from(v: RationalVector) -> Result<Self, Self::Error> {
        Self::try_from(&v)
    }
}

impl<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Rational>> ConvertibleFrom<&RationalVector>
    for UnsignedVector<T>
{
    /// Determines whether a [`RationalVector`] can be converted to an [`UnsignedVector`] (when
    /// every element is representable as a `T`). Takes the [`RationalVector`] by reference.
    ///
    /// Unlike checking whether [`TryFrom`] succeeds, this allocates nothing.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_vector_from_rational_vector#convertible_from).
    #[inline]
    fn convertible_from(v: &RationalVector) -> bool {
        v.elements.iter().all(|x| T::convertible_from(x))
    }
}
