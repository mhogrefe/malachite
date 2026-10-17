// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ConvertibleFrom;

/// The error returned when an [`IntegerMatrix`] with a negative element is converted to a
/// [`NaturalMatrix`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalMatrixFromIntegerMatrixError;

impl TryFrom<IntegerMatrix> for NaturalMatrix {
    type Error = NaturalMatrixFromIntegerMatrixError;

    /// Converts an [`IntegerMatrix`] to a [`NaturalMatrix`], taking the [`IntegerMatrix`] by value
    /// and returning an error if any element is negative.
    ///
    /// Each element's limbs are reused rather than copied. A successful conversion keeps the height
    /// and width.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.elements.len()`.
    ///
    /// # Examples
    /// See [here](super::natural_matrix_from_integer_matrix#try_from).
    fn try_from(m: IntegerMatrix) -> Result<Self, Self::Error> {
        let (height, width) = (m.height(), m.width());
        Ok(Self::from_owned_elements(
            height,
            width,
            m.elements
                .into_iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalMatrixFromIntegerMatrixError))
                .collect::<Result<Vec<Natural>, _>>()?,
        ))
    }
}

impl TryFrom<&IntegerMatrix> for NaturalMatrix {
    type Error = NaturalMatrixFromIntegerMatrixError;

    /// Converts an [`IntegerMatrix`] to a [`NaturalMatrix`], taking the [`IntegerMatrix`] by
    /// reference and returning an error if any element is negative.
    ///
    /// A successful conversion keeps the height and width.
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
    /// See [here](super::natural_matrix_from_integer_matrix#try_from).
    fn try_from(m: &IntegerMatrix) -> Result<Self, Self::Error> {
        Ok(Self::from_owned_elements(
            m.height(),
            m.width(),
            m.elements
                .iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalMatrixFromIntegerMatrixError))
                .collect::<Result<Vec<Natural>, _>>()?,
        ))
    }
}

impl ConvertibleFrom<&IntegerMatrix> for NaturalMatrix {
    /// Determines whether an [`IntegerMatrix`] can be converted to a [`NaturalMatrix`] (when none
    /// of its elements is negative). Takes the [`IntegerMatrix`] by reference.
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
    /// See [here](super::natural_matrix_from_integer_matrix#convertible_from).
    #[inline]
    fn convertible_from(m: &IntegerMatrix) -> bool {
        m.elements.iter().all(Natural::convertible_from)
    }
}
