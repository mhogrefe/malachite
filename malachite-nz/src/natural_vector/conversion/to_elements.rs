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
    /// Returns a [`NaturalVector`]'s elements as a [`Vec`], cloning them.
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
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.to_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .to_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn to_elements(&self) -> Vec<Natural> {
        self.elements.clone()
    }

    /// Returns a [`NaturalVector`]'s elements as a [`Vec`], taking ownership of the
    /// [`NaturalVector`].
    ///
    /// Nothing is copied or allocated.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.into_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .into_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn into_elements(self) -> Vec<Natural> {
        self.elements
    }

    /// Returns a reference to a [`NaturalVector`]'s elements, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.elements_ref().to_debug_string(), "[1, 2, 3]");
    ///
    /// // A slice of the elements can be taken directly.
    /// let tail = &v.elements_ref()[1..];
    /// assert_eq!(tail.to_debug_string(), "[2, 3]");
    /// assert_eq!(
    ///     NaturalVector::from_str("()")
    ///         .unwrap()
    ///         .elements_ref()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn elements_ref(&self) -> &[Natural] {
        &self.elements
    }
}
