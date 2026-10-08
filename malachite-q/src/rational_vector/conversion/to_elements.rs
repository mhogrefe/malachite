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

impl RationalVector {
    /// Returns a [`RationalVector`]'s elements as a [`Vec`], cloning them.
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.to_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .to_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn to_elements(&self) -> Vec<Rational> {
        self.elements.clone()
    }

    /// Returns a [`RationalVector`]'s elements as a [`Vec`], taking ownership of the
    /// [`RationalVector`].
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
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.into_elements().to_debug_string(), "[1, 2, 3]");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .into_elements()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn into_elements(self) -> Vec<Rational> {
        self.elements
    }

    /// Returns a reference to a [`RationalVector`]'s elements, as a slice.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(v.elements_ref().to_debug_string(), "[1, 2, 3]");
    ///
    /// // A slice of the elements can be taken directly.
    /// let tail = &v.elements_ref()[1..];
    /// assert_eq!(tail.to_debug_string(), "[2, 3]");
    /// assert_eq!(
    ///     RationalVector::from_str("()")
    ///         .unwrap()
    ///         .elements_ref()
    ///         .to_debug_string(),
    ///     "[]"
    /// );
    /// ```
    #[inline]
    pub fn elements_ref(&self) -> &[Rational] {
        &self.elements
    }
}
