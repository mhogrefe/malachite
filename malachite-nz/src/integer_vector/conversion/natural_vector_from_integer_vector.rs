// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;
use malachite_base::num::conversion::traits::ConvertibleFrom;

/// The error returned when an [`IntegerVector`] with a negative element is converted to a
/// [`NaturalVector`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalVectorFromIntegerVectorError;

impl TryFrom<IntegerVector> for NaturalVector {
    type Error = NaturalVectorFromIntegerVectorError;

    /// Converts an [`IntegerVector`] to a [`NaturalVector`], taking the [`IntegerVector`] by value
    /// and returning an error if any element is negative.
    ///
    /// Each element's limbs are reused rather than copied. A successful conversion keeps the
    /// dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// See [here](super::natural_vector_from_integer_vector#try_from).
    fn try_from(v: IntegerVector) -> Result<Self, Self::Error> {
        Ok(Self {
            elements: v
                .elements
                .into_iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalVectorFromIntegerVectorError))
                .collect::<Result<Vec<Natural>, _>>()?,
        })
    }
}

impl TryFrom<&IntegerVector> for NaturalVector {
    type Error = NaturalVectorFromIntegerVectorError;

    /// Converts an [`IntegerVector`] to a [`NaturalVector`], taking the [`IntegerVector`] by
    /// reference and returning an error if any element is negative.
    ///
    /// A successful conversion keeps the dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of limbs of the
    /// elements.
    ///
    /// # Examples
    /// See [here](super::natural_vector_from_integer_vector#try_from).
    fn try_from(v: &IntegerVector) -> Result<Self, Self::Error> {
        Ok(Self {
            elements: v
                .elements
                .iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalVectorFromIntegerVectorError))
                .collect::<Result<Vec<Natural>, _>>()?,
        })
    }
}

impl ConvertibleFrom<&IntegerVector> for NaturalVector {
    /// Determines whether an [`IntegerVector`] can be converted to a [`NaturalVector`] (when none
    /// of its elements is negative). Takes the [`IntegerVector`] by reference.
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
    /// See [here](super::natural_vector_from_integer_vector#convertible_from).
    #[inline]
    fn convertible_from(v: &IntegerVector) -> bool {
        v.elements.iter().all(Natural::convertible_from)
    }
}
