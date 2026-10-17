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
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;

/// The error returned when a [`RationalMatrix`] with an element that is not an integer is converted
/// to an [`IntegerMatrix`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntegerMatrixFromRationalMatrixError;

impl TryFrom<RationalMatrix> for IntegerMatrix {
    type Error = IntegerMatrixFromRationalMatrixError;

    /// Converts a [`RationalMatrix`] to an [`IntegerMatrix`], taking the [`RationalMatrix`] by
    /// value and returning an error if any element is not an integer.
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
    /// See [here](super::integer_matrix_from_rational_matrix#try_from).
    fn try_from(m: RationalMatrix) -> Result<Self, Self::Error> {
        let (height, width) = (m.height(), m.width());
        Ok(Self::from_owned_elements(
            height,
            width,
            m.elements
                .into_iter()
                .map(|x| Integer::try_from(x).map_err(|_| IntegerMatrixFromRationalMatrixError))
                .collect::<Result<Vec<Integer>, _>>()?,
        ))
    }
}

impl TryFrom<&RationalMatrix> for IntegerMatrix {
    type Error = IntegerMatrixFromRationalMatrixError;

    /// Converts a [`RationalMatrix`] to an [`IntegerMatrix`], taking the [`RationalMatrix`] by
    /// reference and returning an error if any element is not an integer.
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
    /// See [here](super::integer_matrix_from_rational_matrix#try_from).
    fn try_from(m: &RationalMatrix) -> Result<Self, Self::Error> {
        Ok(Self::from_owned_elements(
            m.height(),
            m.width(),
            m.elements
                .iter()
                .map(|x| Integer::try_from(x).map_err(|_| IntegerMatrixFromRationalMatrixError))
                .collect::<Result<Vec<Integer>, _>>()?,
        ))
    }
}

impl ConvertibleFrom<&RationalMatrix> for IntegerMatrix {
    /// Determines whether a [`RationalMatrix`] can be converted to an [`IntegerMatrix`] (when every
    /// element is an integer). Takes the [`RationalMatrix`] by reference.
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
    /// See [here](super::integer_matrix_from_rational_matrix#convertible_from).
    #[inline]
    fn convertible_from(m: &RationalMatrix) -> bool {
        m.elements.iter().all(Integer::convertible_from)
    }
}
