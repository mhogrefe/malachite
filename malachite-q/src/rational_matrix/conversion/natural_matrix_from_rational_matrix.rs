// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;

/// The error returned when a [`RationalMatrix`] with an element that is negative or not an integer
/// is converted to a [`NaturalMatrix`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NaturalMatrixFromRationalMatrixError;

impl TryFrom<RationalMatrix> for NaturalMatrix {
    type Error = NaturalMatrixFromRationalMatrixError;

    /// Converts a [`RationalMatrix`] to a [`NaturalMatrix`], taking the [`RationalMatrix`] by value
    /// and returning an error if any element is negative or is not an integer.
    ///
    /// Each element's numerator limbs are reused rather than copied. A successful conversion keeps
    /// the height and width.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.elements.len()`.
    ///
    /// # Examples
    /// See [here](super::natural_matrix_from_rational_matrix#try_from).
    fn try_from(m: RationalMatrix) -> Result<Self, Self::Error> {
        let (height, width) = (m.height(), m.width());
        Ok(Self::from_owned_elements(
            height,
            width,
            m.elements
                .into_iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalMatrixFromRationalMatrixError))
                .collect::<Result<Vec<Natural>, _>>()?,
        ))
    }
}

impl TryFrom<&RationalMatrix> for NaturalMatrix {
    type Error = NaturalMatrixFromRationalMatrixError;

    /// Converts a [`RationalMatrix`] to a [`NaturalMatrix`], taking the [`RationalMatrix`] by
    /// reference and returning an error if any element is negative or is not an integer.
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
    /// See [here](super::natural_matrix_from_rational_matrix#try_from).
    fn try_from(m: &RationalMatrix) -> Result<Self, Self::Error> {
        Ok(Self::from_owned_elements(
            m.height(),
            m.width(),
            m.elements
                .iter()
                .map(|x| Natural::try_from(x).map_err(|_| NaturalMatrixFromRationalMatrixError))
                .collect::<Result<Vec<Natural>, _>>()?,
        ))
    }
}

impl ConvertibleFrom<&RationalMatrix> for NaturalMatrix {
    /// Determines whether a [`RationalMatrix`] can be converted to a [`NaturalMatrix`] (when every
    /// element is a nonnegative integer). Takes the [`RationalMatrix`] by reference.
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
    /// See [here](super::natural_matrix_from_rational_matrix#convertible_from).
    #[inline]
    fn convertible_from(m: &RationalMatrix) -> bool {
        m.elements.iter().all(Natural::convertible_from)
    }
}
