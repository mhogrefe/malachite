// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use alloc::vec::Vec;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::unsigned_vector::UnsignedVector;

/// The error returned when a [`IntegerVector`] has an element too large for the element type of the
/// [`UnsignedVector`] it is being converted to.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnsignedVectorFromIntegerVectorError;

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>> TryFrom<&IntegerVector>
    for UnsignedVector<T>
{
    type Error = UnsignedVectorFromIntegerVectorError;

    /// Converts an [`IntegerVector`] to an [`UnsignedVector`], taking the [`IntegerVector`] by
    /// reference and returning an error if any element is negative or too large for `T`.
    ///
    /// No element is ever wrapped, and no negative element is ever reinterpreted as unsigned. A
    /// successful conversion keeps the dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// See [here](super::unsigned_vector_from_integer_vector#try_from).
    fn try_from(v: &IntegerVector) -> Result<Self, Self::Error> {
        Ok(Self {
            elements: v
                .elements
                .iter()
                .map(|x| T::try_from(x).map_err(|_| UnsignedVectorFromIntegerVectorError))
                .collect::<Result<Vec<T>, _>>()?,
        })
    }
}

impl<T: PrimitiveUnsigned + for<'a> TryFrom<&'a Integer>> TryFrom<IntegerVector>
    for UnsignedVector<T>
{
    type Error = UnsignedVectorFromIntegerVectorError;

    /// Converts an [`IntegerVector`] to an [`UnsignedVector`], taking the [`IntegerVector`] by
    /// value and returning an error if any element is negative or too large for `T`.
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
    /// See [here](super::unsigned_vector_from_integer_vector#try_from).
    #[inline]
    fn try_from(v: IntegerVector) -> Result<Self, Self::Error> {
        Self::try_from(&v)
    }
}

impl<T: PrimitiveUnsigned + for<'a> ConvertibleFrom<&'a Integer>> ConvertibleFrom<&IntegerVector>
    for UnsignedVector<T>
{
    /// Determines whether an [`IntegerVector`] can be converted to an [`UnsignedVector`] (when
    /// every element is representable as a `T`). Takes the [`IntegerVector`] by reference.
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
    /// See [here](super::unsigned_vector_from_integer_vector#convertible_from).
    #[inline]
    fn convertible_from(v: &IntegerVector) -> bool {
        v.elements.iter().all(|x| T::convertible_from(x))
    }
}
