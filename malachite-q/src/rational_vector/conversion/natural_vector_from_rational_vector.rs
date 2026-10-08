// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::rational_vector::RationalVector;
use alloc::vec::Vec;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;

/// The error returned when a [`RationalVector`] with an element that is negative or not an integer
/// is converted to a [`NaturalVector`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalVectorFromRationalVectorError;

impl TryFrom<RationalVector> for NaturalVector {
    type Error = NaturalVectorFromRationalVectorError;

    /// Converts a [`RationalVector`] to a [`NaturalVector`], taking the [`RationalVector`] by value
    /// and returning an error if any element is negative or not an integer.
    ///
    /// Each element's numerator limbs are reused rather than copied. A successful conversion keeps
    /// the dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// See [here](super::natural_vector_from_rational_vector#try_from).
    fn try_from(v: RationalVector) -> Result<Self, Self::Error> {
        Ok(Self {
            elements: v
                .elements
                .into_iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalVectorFromRationalVectorError))
                .collect::<Result<Vec<Natural>, _>>()?,
        })
    }
}

impl TryFrom<&RationalVector> for NaturalVector {
    type Error = NaturalVectorFromRationalVectorError;

    /// Converts a [`RationalVector`] to a [`NaturalVector`], taking the [`RationalVector`] by
    /// reference and returning an error if any element is negative or not an integer.
    ///
    /// A successful conversion keeps the dimension.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of limbs of the
    /// elements' numerators.
    ///
    /// # Examples
    /// See [here](super::natural_vector_from_rational_vector#try_from).
    fn try_from(v: &RationalVector) -> Result<Self, Self::Error> {
        Ok(Self {
            elements: v
                .elements
                .iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalVectorFromRationalVectorError))
                .collect::<Result<Vec<Natural>, _>>()?,
        })
    }
}

impl ConvertibleFrom<&RationalVector> for NaturalVector {
    /// Determines whether a [`RationalVector`] can be converted to a [`NaturalVector`] (when every
    /// element is a nonnegative integer). Takes the [`RationalVector`] by reference.
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
    /// See [here](super::natural_vector_from_rational_vector#convertible_from).
    #[inline]
    fn convertible_from(v: &RationalVector) -> bool {
        v.elements.iter().all(Natural::convertible_from)
    }
}
