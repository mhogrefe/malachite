// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec::Vec;

impl NaturalVector {
    /// Converts a slice of [`Natural`]s to a [`NaturalVector`], cloning them.
    ///
    /// The vector's dimension is the length of the slice. Every slice is a valid vector, so this
    /// cannot fail; the empty slice gives the 0-dimensional vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_elements(&[Natural::ONE, Natural::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(NaturalVector::from_elements(&[]).to_string(), "()");
    /// ```
    #[inline]
    pub fn from_elements(xs: &[Natural]) -> Self {
        Self {
            elements: xs.to_vec(),
        }
    }

    /// Converts a [`Vec`] of [`Natural`]s to a [`NaturalVector`], taking ownership of it.
    ///
    /// The vector's dimension is the length of the [`Vec`]. Every [`Vec`] is a valid vector, so
    /// this cannot fail, and nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_owned_elements(vec![Natural::ONE, Natural::TWO]).to_string(),
    ///     "(1, 2)"
    /// );
    /// assert_eq!(
    ///     NaturalVector::from_owned_elements(Vec::new()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    pub const fn from_owned_elements(xs: Vec<Natural>) -> Self {
        Self { elements: xs }
    }
}
