// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::named::Named;
use malachite_base::num::conversion::traits::ExactFrom;

/// Functions for converting a [`NaturalVector`] to and from other types.
pub mod conversion;
/// Iterators that generate [`NaturalVector`]s without repetition.
pub mod exhaustive;
/// Iterators that generate [`NaturalVector`]s randomly.
#[cfg(feature = "random")]
pub mod random;

/// A vector whose elements are [`Natural`]s.
///
/// Its dimension is the number of elements, and the 0-dimensional vector has none. The field is
/// public, since every [`Vec`] of [`Natural`]s is a valid vector: unlike a polynomial, a vector has
/// no normal form to maintain, and two vectors are equal exactly when their elements are.
///
/// A `NaturalVector` is serialized as the list of its elements, each serialized the way a
/// [`Natural`] is.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{One, Two};
/// use malachite_nz::natural::Natural;
/// use malachite_nz::natural_vector::NaturalVector;
///
/// let v = NaturalVector {
///     elements: vec![Natural::ONE, Natural::TWO, Natural::from(3u32)],
/// };
/// assert_eq!(v.dimension(), 3);
/// ```
#[derive(Clone, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct NaturalVector {
    pub elements: Vec<Natural>,
}

impl NaturalVector {
    /// Returns the dimension of a [`NaturalVector`]: the number of its elements.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector {
    ///     elements: vec![Natural::from(4u32), Natural::from(5u32)],
    /// };
    /// assert_eq!(v.dimension(), 2);
    /// assert_eq!(
    ///     NaturalVector {
    ///         elements: Vec::new()
    ///     }
    ///     .dimension(),
    ///     0
    /// );
    /// ```
    #[inline]
    pub fn dimension(&self) -> u64 {
        u64::exact_from(self.elements.len())
    }
}

// Implements `Named` for `NaturalVector`.
impl_named!(NaturalVector);
